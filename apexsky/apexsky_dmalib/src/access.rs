use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;
use std::thread::{sleep, sleep_until};
use std::time::{Duration, Instant};

use obfstr::obfstr as s;
use regex::bytes::Regex;
use tokio::sync::{oneshot, watch};
use tracing::instrument;

use crate::mem::{
    memflow_impl::MemflowOs, memprocfs_impl::MemProcFsOs, MemOs, MemProc, MemProcImpl,
    ProcessStatus,
};
use crate::mem::{MemConnector, MemOsImpl};
use crate::AccessError;

pub type MemApi = crossbeam_channel::Sender<PriorityAccess>;

pub fn create_api() -> (MemApi, crossbeam_channel::Receiver<PriorityAccess>) {
    crossbeam_channel::bounded::<PriorityAccess>(0x2000)
}

#[derive(Debug)]
pub struct PriorityAccess {
    priority: i32,
    req: AccessType,
}

impl PriorityAccess {
    fn new(req: AccessType, priority: i32) -> Self {
        Self { priority, req }
    }
}

#[derive(Debug)]
pub enum AccessType {
    FlushRequests(FlushRequestsRequest),
    MemBaseaddr(MemBaseaddrRequest),
    MemRead(MemReadRequest),
    MemWrite(MemWriteRequest),
    MemDump(MemDumpRequest),
    MemFindSignature(MemFindSignatureRequest),
}

#[derive(Debug)]
pub struct FlushRequestsRequest {
    priority_threshold: i32,
    future_tx: oneshot::Sender<usize>,
}

#[derive(Debug)]
pub struct MemBaseaddrRequest {
    future_tx: oneshot::Sender<Option<u64>>,
}

#[derive(Debug)]
pub struct MemReadRequest {
    address: u64,
    data_size: usize,
    id: usize,
    future_tx: oneshot::Sender<anyhow::Result<Vec<u8>>>,
}

#[derive(Debug)]
pub struct MemWriteRequest {
    address: u64,
    write: Vec<u8>,
    id: usize,
    future_tx: oneshot::Sender<anyhow::Result<()>>,
}

#[derive(Debug)]
pub struct MemDumpRequest {
    future_tx: oneshot::Sender<anyhow::Result<Vec<u8>>>,
}

#[derive(Debug, Default)]
struct DumpMemory {
    base: u64,
    time_date_stamp: u32,
    checksum: u32,
    size_of_image: u32,
}

impl DumpMemory {
    fn check_and_update(&mut self, mem: &mut MemProcImpl) -> anyhow::Result<()> {
        use pelite::pe64::*;
        let base = mem.get_proc_baseaddr();
        let Ok(dos_header) = mem.read::<image::IMAGE_DOS_HEADER>(base) else {
            anyhow::bail!("{}", s!("[-] Failed to read PE Header"));
        };
        let Ok(nt_headers) = mem.read::<image::IMAGE_NT_HEADERS>(base + dos_header.e_lfanew as u64)
        else {
            anyhow::bail!("{}", s!("[-] Failed to read NT Header"));
        };
        if nt_headers.Signature != image::IMAGE_NT_HEADERS_SIGNATURE
            || nt_headers.OptionalHeader.Magic != image::IMAGE_NT_OPTIONAL_HDR_MAGIC
        {
            anyhow::bail!("{}", s!("[-] Failed signature check"));
        }
        let section_headers = dos_header.e_lfanew
            + size_of::<u32>() as u32
            + size_of::<image::IMAGE_FILE_HEADER>() as u32
            + nt_headers.FileHeader.SizeOfOptionalHeader as u32;
        let Ok(_text_section) =
            mem.read::<image::IMAGE_SECTION_HEADER>(base + section_headers as u64)
        else {
            anyhow::bail!("{}", s!("[-] Failed to read Section Header"));
        };
        self.base = base;
        self.time_date_stamp = nt_headers.FileHeader.TimeDateStamp;
        self.checksum = nt_headers.OptionalHeader.CheckSum;
        self.size_of_image = nt_headers.OptionalHeader.SizeOfImage;
        Ok(())
    }

    fn dump(&mut self, mem: &mut MemProcImpl) -> anyhow::Result<Vec<u8>> {
        self.check_and_update(mem)?;
        let mut image_data = vec![0u8; self.size_of_image as usize];
        mem.read_raw_into(self.base, &mut image_data)?;

        // Fixup PE headers
        let (_, _, data_directory, section_headers) =
            unsafe { pelite::pe64::headers_mut(&mut image_data) };
        for section in section_headers {
            section.PointerToRawData = section.VirtualAddress;
            section.SizeOfRawData = section.VirtualSize;
            if &section.Name == b".reloc\0\0" {
                if let Some(reloc_dir) =
                    data_directory.get_mut(pelite::image::IMAGE_DIRECTORY_ENTRY_BASERELOC)
                {
                    reloc_dir.VirtualAddress = section.VirtualAddress;
                    reloc_dir.Size = section.VirtualSize;
                }
            }
        }

        Ok(image_data)
    }
}

#[derive(Debug)]
pub struct MemFindSignatureRequest {
    signature: MemSignature,
    range: Range<usize>,
    future_tx: oneshot::Sender<anyhow::Result<Option<(usize, Vec<u8>)>>>,
}

#[derive(Debug)]
pub struct MemSignature(pub Vec<Option<u8>>);

impl MemSignature {
    pub fn scan(&self, buffer: &[u8]) -> Option<(usize, Vec<u8>)> {
        let re = self
            .0
            .iter()
            .map(|&b| match b {
                Some(byte) => format!("{}{:02X}{}", r"(?-u:\x", byte, ")"),
                None => s!(r"(?-u:[\x00-\xFF])").to_string(),
            })
            .collect::<Vec<_>>()
            .join("");
        //tracing::info!(?re, "{:X?}", buffer);
        let re = Regex::new(&re).unwrap();
        re.find(buffer)
            .map(|mat| (mat.start(), mat.as_bytes().to_vec()))
    }
}

impl FromStr for MemSignature {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .split(' ')
            .map(|s| match s {
                "?" => Ok(None),
                "??" => Ok(None),
                b => u8::from_str_radix(b, 16).map(|byte| Some(byte)),
            })
            .try_collect::<Vec<Option<u8>>>()
            .map(|signature| Self(signature))
    }
}

impl AccessType {
    pub fn flush(priority_threshold: i32) -> (Self, oneshot::Receiver<usize>) {
        let (future_tx, rx) = oneshot::channel();
        (
            AccessType::FlushRequests(FlushRequestsRequest {
                priority_threshold,
                future_tx,
            }),
            rx,
        )
    }

    pub fn mem_baseaddr() -> (Self, oneshot::Receiver<Option<u64>>) {
        let (future_tx, rx) = oneshot::channel();
        (
            AccessType::MemBaseaddr(MemBaseaddrRequest { future_tx }),
            rx,
        )
    }

    /// # Examples
    ///
    /// ```
    /// struct AimingInfo {
    ///     pub local_origin: [f32; 3],
    ///     pub view_angles: [f32; 3],
    ///     pub target_origin: [f32; 3],
    ///     pub target_vel: [f32; 3],
    /// }
    ///
    /// async fn read_aiming_info(
    ///     mem_aim_helper: &MemAimHelper,
    ///     target: &dyn AimEntity,
    /// ) -> anyhow::Result<AimingInfo> {
    ///     use apexsky::offsets::G_OFFSETS;
    ///     use apexsky_dmalib::access::{AccessType, PendingAccessRequest, PendingMemRead};
    ///     use std::mem::size_of;
    ///
    ///     let lplayer_ptr = mem_aim_helper.lplayer_ptr;
    ///     let target_ptr = target.get_entity_ptr();
    ///     let mem = &mem_aim_helper.mem;
    ///
    ///     let reqs = (
    ///         AccessType::mem_read(
    ///             lplayer_ptr + G_OFFSETS.centity_origin,
    ///             size_of::<[f32; 3]>(),
    ///             0,
    ///         ),
    ///         AccessType::mem_read(
    ///             lplayer_ptr + G_OFFSETS.player_viewangles,
    ///             size_of::<[f32; 3]>(),
    ///             0,
    ///         ),
    ///         AccessType::mem_read(
    ///             target_ptr + G_OFFSETS.centity_origin,
    ///             size_of::<[f32; 3]>(),
    ///             0,
    ///         ),
    ///         AccessType::mem_read(
    ///             target_ptr + G_OFFSETS.centity_velocity,
    ///             size_of::<[f32; 3]>(),
    ///             0,
    ///         ),
    ///     );
    ///     let futs = tokio::try_join!(
    ///         reqs.0.with_priority(10).dispatch(mem),
    ///         reqs.1.with_priority(10).dispatch(mem),
    ///         reqs.2.with_priority(10).dispatch(mem),
    ///         reqs.3.with_priority(10).dispatch(mem),
    ///     )?;
    ///     let vals = tokio::try_join!(
    ///         futs.0.recv_for::<[f32; 3]>(),
    ///         futs.1.recv_for::<[f32; 3]>(),
    ///         futs.2.recv_for::<[f32; 3]>(),
    ///         futs.3.recv_for::<[f32; 3]>(),
    ///     )?;
    ///     Ok(AimingInfo {
    ///         local_origin: vals.0,
    ///         view_angles: vals.1,
    ///         target_origin: vals.2,
    ///         target_vel: if target.is_player() {
    ///             vals.3
    ///         } else {
    ///             target.get_abs_velocity()
    ///         },
    ///     })
    /// }
    /// ```
    pub fn mem_read(
        addr: u64,
        len: usize,
        req_id: usize,
    ) -> (Self, oneshot::Receiver<anyhow::Result<Vec<u8>>>) {
        let (future_tx, rx) = oneshot::channel();
        (
            AccessType::MemRead(MemReadRequest {
                address: addr,
                data_size: len,
                id: req_id,
                future_tx,
            }),
            rx,
        )
    }

    pub fn mem_write(
        addr: u64,
        data: Vec<u8>,
        req_id: usize,
    ) -> (Self, oneshot::Receiver<anyhow::Result<()>>) {
        let (future_tx, rx) = oneshot::channel();
        (
            AccessType::MemWrite(MemWriteRequest {
                address: addr,
                write: data,
                id: req_id,
                future_tx,
            }),
            rx,
        )
    }

    pub fn mem_write_typed<T: dataview::Pod + ?Sized>(
        addr: u64,
        data: &T,
        req_id: usize,
    ) -> (Self, oneshot::Receiver<anyhow::Result<()>>) {
        let data = dataview::bytes(data);
        Self::mem_write(addr, data.to_vec(), req_id)
    }

    pub fn mem_dump() -> (Self, oneshot::Receiver<anyhow::Result<Vec<u8>>>) {
        let (future_tx, rx) = oneshot::channel();
        (AccessType::MemDump(MemDumpRequest { future_tx }), rx)
    }

    pub fn mem_find_signature(
        signature: MemSignature,
        range: Range<u64>,
    ) -> (
        Self,
        oneshot::Receiver<anyhow::Result<Option<(usize, Vec<u8>)>>>,
    ) {
        let (future_tx, rx) = oneshot::channel();
        (
            AccessType::MemFindSignature(MemFindSignatureRequest {
                signature,
                range: Range {
                    start: range.start.try_into().unwrap(),
                    end: range.end.try_into().unwrap(),
                },
                future_tx,
            }),
            rx,
        )
    }
}

#[derive(Debug, Default)]
struct ScatterRequestMap {
    read: HashMap<usize, Vec<MemReadRequest>>,
    scan: Vec<MemFindSignatureRequest>,
    write: HashMap<usize, Vec<MemWriteRequest>>,
}

#[derive(Debug, Default)]
struct ContextData {
    priority_queue: BinaryHeap<PriorityAccess>,
    scatter_map: ScatterRequestMap,
    dumper: DumpMemory,
}

#[derive(Debug, Clone)]
pub struct ConnectConfig {
    pub mem_connector: crate::mem::MemConnector,
    pub target_proc_name: String,
    pub check_time_date_stamp: Option<u32>,
    pub speed_test: bool,
}

#[instrument(skip_all)]
pub fn io_thread(
    active: watch::Receiver<bool>,
    access_rx: crossbeam_channel::Receiver<PriorityAccess>,
    config: ConnectConfig,
) -> Result<(), AccessError> {
    tracing::debug!("{}", s!("task start"));

    let mut speed_test_done = false;
    let mut accessible: bool = false;
    let mut ctx: ContextData = ContextData::default();
    let mut start_instant;
    let mut next_flush_instant = Instant::now();

    let mut mem_os = create_os_instance(config.mem_connector.clone())
        .map_err(|e| AccessError::Connector(config.mem_connector, e))?;

    while *active.borrow() {
        start_instant = Instant::now();

        // Fallback response
        if !accessible {
            fn notify_of_unavaliable<T>(sender: oneshot::Sender<anyhow::Result<T>>) {
                sender
                    .send(Err(anyhow::anyhow!("{}", s!("!accessible"))))
                    .ok();
            }
            loop {
                match access_rx.try_recv() {
                    Ok(req) => match req.req {
                        AccessType::FlushRequests(r) => {
                            r.future_tx.send(0).ok();
                        }
                        AccessType::MemBaseaddr(r) => {
                            r.future_tx.send(None).ok();
                        }
                        AccessType::MemRead(r) => {
                            notify_of_unavaliable(r.future_tx);
                        }
                        AccessType::MemWrite(r) => {
                            notify_of_unavaliable(r.future_tx);
                        }
                        AccessType::MemDump(r) => {
                            notify_of_unavaliable(r.future_tx);
                        }
                        AccessType::MemFindSignature(r) => {
                            notify_of_unavaliable(r.future_tx);
                        }
                    },
                    Err(e) => {
                        match e {
                            crossbeam_channel::TryRecvError::Empty => (),
                            crossbeam_channel::TryRecvError::Disconnected => {
                                tracing::error!(%e, ?e);
                            }
                        }
                        break;
                    }
                }
            }
        }

        // Find process
        let Some(mut mem) = find_target_process(&mut mem_os, config.target_proc_name.to_owned())
        else {
            accessible = false;
            sleep_until(start_instant + Duration::from_secs(2));
            continue;
        };

        // Check
        if mem.check_proc_status() != ProcessStatus::FoundReady
            || ctx
                .dumper
                .check_and_update(&mut mem)
                .inspect_err(|e| {
                    tracing::warn!("{}", e);
                })
                .is_err()
        {
            accessible = false;
            sleep_until(start_instant + Duration::from_secs(2));
            continue;
        }

        // Found and ready
        if !accessible {
            tracing::info!("{}", s!("Process found"));
            tracing::info!(dumper=?ctx.dumper, "{}{:x}", s!("Base: 0x"), mem.get_proc_baseaddr());

            if config.speed_test && !speed_test_done {
                tracing::debug!("{}", s!("speed_test"));
                mem.speed_test();
                println!("{}", s!("Press enter to continue.."));
                tracing::debug!("{}", s!("press to continue"));
                let _ = std::io::stdin().read_line(&mut String::new());
                speed_test_done = true;
            }

            if let Some(check) = config.check_time_date_stamp {
                if check != ctx.dumper.time_date_stamp {
                    return Err(AccessError::InvalidTimeDateStamp(
                        ctx.dumper.time_date_stamp,
                        check,
                    ));
                }
            }

            accessible = true;
        }

        while *active.borrow() {
            let loop_duration = start_instant.elapsed().as_millis();
            start_instant = Instant::now();
            tracing::trace!(?loop_duration);

            if mem.check_proc_status() != ProcessStatus::FoundReady {
                accessible = false;
                sleep(Duration::from_secs(2));
                break;
            }

            #[inline(always)]
            fn push_req(req: PriorityAccess, ctx: &mut ContextData, mem: &mut MemProcImpl) {
                let preempt = req.priority > 0xf;
                ctx.priority_queue.push(req);
                if preempt {
                    execute_requests(0x10, ctx, mem);
                }
            }

            loop {
                // match access_rx.recv_deadline(next_flush_instant) {
                //     Ok(req) => push_req(req, &mut priority_queue, &mut scatter_map, &mut mem),
                //     Err(e) => match e {
                //         crossbeam_channel::RecvTimeoutError::Timeout => {
                //             execute_requests(0, &mut priority_queue, &mut scatter_map, &mut mem);
                //             next_flush_instant = start_instant + Duration::from_millis(1);
                //             break;
                //         }
                //         crossbeam_channel::RecvTimeoutError::Disconnected => {
                //             tracing::error!(%e, ?e);
                //             execute_requests(0, &mut priority_queue, &mut scatter_map, &mut mem);
                //             break;
                //         }
                //     },
                // }

                match access_rx.try_recv() {
                    Ok(req) => push_req(req, &mut ctx, &mut mem),
                    Err(e) => {
                        match e {
                            crossbeam_channel::TryRecvError::Empty => {
                                if Instant::now() < next_flush_instant {
                                    // Execute higher priority requests first
                                    if ctx
                                        .priority_queue
                                        .peek()
                                        .is_some_and(|req| req.priority >= 1)
                                    {
                                        execute_requests(1, &mut ctx, &mut mem);
                                    }
                                    // Wait until next flush instant
                                    if let Ok(req) = access_rx.recv_deadline(next_flush_instant) {
                                        push_req(req, &mut ctx, &mut mem);
                                    }
                                    continue;
                                }

                                let now = Instant::now();
                                assert!(now >= next_flush_instant);
                                next_flush_instant = start_instant + Duration::from_millis(1);
                                if next_flush_instant < now {
                                    next_flush_instant = now + Duration::from_millis(1);
                                }
                                assert!(now <= next_flush_instant);
                                // Flush requests
                            }
                            crossbeam_channel::TryRecvError::Disconnected => {
                                tracing::error!(%e, ?e);
                            }
                        }
                        execute_requests(0, &mut ctx, &mut mem);
                        break;
                    }
                }
            }
        }
    }

    tracing::debug!("{}", s!("task end"));
    Ok(())
}

fn execute_requests(priority_threshold: i32, ctx: &mut ContextData, mem: &mut MemProcImpl) {
    // flush requests
    let flush_report = consume_requests(None, priority_threshold, ctx, mem);

    let scatter_map = &mut ctx.scatter_map;

    let read = !scatter_map.read.is_empty() || !scatter_map.scan.is_empty();
    let write = !scatter_map.write.is_empty();

    if read || write {
        // try scatter read/write
        if let Some(mem_scatter) = mem.get_vmm_scatter() {
            // prepare scatter read
            let mut pending_read =
                Vec::with_capacity(scatter_map.read.get(&0).map(|arr0| arr0.len()).unwrap_or(0));
            if read {
                scatter_map.read.drain().for_each(|(_id, req_arr)| {
                    req_arr.into_iter().for_each(|r| {
                        if let Err(e) = mem_scatter.prepare(r.address, r.data_size) {
                            tracing::debug!(%e, ?e);
                            let _ = r.future_tx.send(Err(e));
                        } else {
                            pending_read.push(r);
                        }
                    });
                });
            }

            // prepare scatter read for scan request
            let pending_scan = scatter_map.scan.drain(..).filter_map(|r| {
                if let Err(e) = mem_scatter.prepare(r.range.start as u64, r.range.len()) {
                    tracing::debug!(%e, ?e);
                    let _ = r.future_tx.send(Err(e));
                    None
                } else {
                    Some(r)
                }
            });

            // prepare scatter write
            let mut pending_write = Vec::with_capacity(
                scatter_map
                    .write
                    .get(&0)
                    .map(|arr0| arr0.len())
                    .unwrap_or(0),
            );
            if write {
                scatter_map.write.drain().for_each(|(_id, req_arr)| {
                    req_arr.into_iter().for_each(|r| {
                        if let Err(e) = mem_scatter.prepare_write(r.address, &r.write) {
                            tracing::warn!(%e, ?e);
                            let _ = r.future_tx.send(Err(e));
                        } else {
                            pending_write.push(r.future_tx);
                        }
                    });
                });
            }

            // execute scatter read/write
            match mem_scatter.execute() {
                Ok(()) => {
                    pending_read.into_iter().for_each(|r| {
                        let result = mem_scatter.read(r.address, r.data_size);
                        let _ = r.future_tx.send(result);
                    });
                    pending_scan.for_each(|r| {
                        let result =
                            mem_scatter
                                .read(r.range.start as u64, r.range.len())
                                .map(|buffer| {
                                    r.signature
                                        .scan(&buffer)
                                        .map(|(offset, data)| (r.range.start + offset, data))
                                });
                        let _ = r.future_tx.send(result);
                    });
                    pending_write.into_iter().for_each(|tx| {
                        let _ = tx.send(Ok(()));
                    });
                }
                Err(e) => {
                    pending_read.into_iter().for_each(|r| {
                        let _ = r
                            .future_tx
                            .send(Err(anyhow::anyhow!("{}{e}", s!("scatter read failed: "))));
                    });
                    pending_scan.for_each(|r| {
                        let _ = r
                            .future_tx
                            .send(Err(anyhow::anyhow!("{}{e}", s!("scatter read failed: "))));
                    });
                    pending_write.into_iter().for_each(|tx| {
                        let _ =
                            tx.send(Err(anyhow::anyhow!("{}{e}", s!("scatter write failed: "))));
                    });
                }
            }
        }

        // try normal read
        if read {
            scatter_map.read.drain().for_each(|(_id, req_arr)| {
                req_arr.into_iter().for_each(|r| {
                    let mut buffer = vec![0; r.data_size];
                    let result = mem.read_raw_into(r.address, &mut buffer).map(|()| buffer);
                    let _ = r.future_tx.send(result);
                });
            });
            scatter_map.scan.drain(..).for_each(|r| {
                let mut buffer = vec![0; r.range.len()];
                let result = mem
                    .read_raw_into(r.range.start as u64, &mut buffer)
                    .map(|()| {
                        r.signature
                            .scan(&buffer)
                            .map(|(offset, data)| (r.range.start + offset, data))
                    });
                let _ = r.future_tx.send(result);
            });
        }

        // try normal write
        if write {
            scatter_map.write.drain().for_each(|(_id, req_arr)| {
                req_arr.into_iter().for_each(|r| {
                    let _ = r.future_tx.send(mem.write_raw(r.address, &r.write));
                });
            });
        }
    }

    // callback
    flush_respond(flush_report);
}

#[derive(Debug)]
struct FlushReport {
    done_count: usize,
    pending_count: usize,
    flush_request: Option<FlushRequestsRequest>,
    children: Vec<FlushReport>,
}

fn consume_requests(
    flush_request: Option<FlushRequestsRequest>,
    default_priority_threshold: i32,
    ctx: &mut ContextData,
    mem: &mut MemProcImpl,
) -> FlushReport {
    let mut flush_report = FlushReport {
        done_count: 0,
        pending_count: 0,
        flush_request,
        children: Vec::new(),
    };

    let priority_threshold = flush_report
        .flush_request
        .as_ref()
        .map(|r| r.priority_threshold)
        .unwrap_or(default_priority_threshold);

    loop {
        let Some(access) = ctx.priority_queue.peek() else {
            break;
        };
        if access.priority < priority_threshold {
            break;
        }
        match ctx.priority_queue.pop().unwrap().req {
            AccessType::MemBaseaddr(r) => {
                let _ = r.future_tx.send(match mem.get_proc_baseaddr() {
                    0 => None,
                    a => Some(a),
                });
                flush_report.done_count += 1;
            }
            AccessType::MemRead(r) => {
                match ctx.scatter_map.read.get_mut(&r.id) {
                    Some(arr) => {
                        arr.push(r);
                    }
                    None => {
                        ctx.scatter_map.read.insert(r.id, vec![r]);
                    }
                }
                flush_report.pending_count += 1;
            }
            AccessType::MemWrite(r) => {
                match ctx.scatter_map.write.get_mut(&r.id) {
                    Some(arr) => {
                        arr.push(r);
                    }
                    None => {
                        ctx.scatter_map.write.insert(r.id, vec![r]);
                    }
                }
                flush_report.pending_count += 1;
            }
            AccessType::FlushRequests(r) => {
                flush_report.children.push(consume_requests(
                    Some(r),
                    default_priority_threshold,
                    ctx,
                    mem,
                ));
                flush_report.pending_count += 1;
            }
            AccessType::MemDump(r) => {
                let _ = r.future_tx.send(ctx.dumper.dump(mem));
                flush_report.done_count += 1;
            }
            AccessType::MemFindSignature(r) => {
                if r.range.is_empty() {
                    let _ = r
                        .future_tx
                        .send(Err(anyhow::anyhow!(s!("invalid range").to_string())));
                    flush_report.done_count += 1;
                } else {
                    ctx.scatter_map.scan.push(r);
                    flush_report.pending_count += 1;
                }
            }
        }
    }
    flush_report
}

fn flush_respond(flush_report: FlushReport) {
    flush_report.children.into_iter().for_each(flush_respond);
    if let Some(r) = flush_report.flush_request {
        let _ = r
            .future_tx
            .send(flush_report.pending_count + flush_report.done_count);
    }
}

#[instrument]
fn create_os_instance(connector: MemConnector) -> anyhow::Result<MemOsImpl> {
    match connector {
        MemConnector::PCILeech(_) => Ok(MemOsImpl::Vmm(MemProcFsOs::new(connector)?)),
        _ => Ok(MemOsImpl::Memflow(MemflowOs::new(connector)?)),
    }
}

fn find_target_process(mem_os: &mut MemOsImpl, proc_name: String) -> Option<MemProcImpl<'_>> {
    tracing::info!(parent: None, "{}", s!("Searching for process..."));
    mem_os.open_proc(proc_name).map(Some).unwrap_or_else(|e| {
        tracing::trace!(?e, "{}", s!("open_proc"));
        None
    })
}

impl PartialEq for PriorityAccess {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl PartialOrd for PriorityAccess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl Eq for PriorityAccess {}

impl Ord for PriorityAccess {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

pub trait AccessRequest {
    fn with_priority(self, priority: i32) -> PriorityAccess;
    fn dispatch(self, api: &MemApi)
        -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    fn blocking_dispatch(self, api: &MemApi) -> anyhow::Result<()>;
}

impl AccessRequest for PriorityAccess {
    fn with_priority(mut self, priority: i32) -> PriorityAccess {
        self.priority = priority;
        self
    }

    async fn dispatch(self, api: &MemApi) -> anyhow::Result<()> {
        self.blocking_dispatch(api)
    }

    #[inline]
    fn blocking_dispatch(self, api: &MemApi) -> anyhow::Result<()> {
        api.send(self).map_err(|e| {
            let e: anyhow::Error = e.into();
            e.context(s!("Failed to dispatch access request").to_string())
        })
    }
}

impl AccessRequest for AccessType {
    fn with_priority(self, priority: i32) -> PriorityAccess {
        PriorityAccess::new(self, priority)
    }

    async fn dispatch(self, api: &MemApi) -> anyhow::Result<()> {
        self.with_priority(0x1).dispatch(api).await
    }

    fn blocking_dispatch(self, api: &MemApi) -> anyhow::Result<()> {
        self.with_priority(0x1).blocking_dispatch(api)
    }
}

pub trait PendingAccessRequest<T> {
    fn with_priority(self, priority: i32) -> (PriorityAccess, oneshot::Receiver<T>);
    fn dispatch(
        self,
        api: &MemApi,
    ) -> impl std::future::Future<Output = anyhow::Result<oneshot::Receiver<T>>> + Send;
    fn blocking_dispatch(self, api: &MemApi) -> anyhow::Result<oneshot::Receiver<T>>;
}

impl<T, R> PendingAccessRequest<T> for (R, oneshot::Receiver<T>)
where
    R: AccessRequest + Send,
    T: Send,
{
    fn with_priority(self, priority: i32) -> (PriorityAccess, oneshot::Receiver<T>) {
        (self.0.with_priority(priority), self.1)
    }

    async fn dispatch(self, api: &MemApi) -> anyhow::Result<oneshot::Receiver<T>> {
        self.0.dispatch(api).await.map(|_| self.1)
    }

    fn blocking_dispatch(self, api: &MemApi) -> anyhow::Result<oneshot::Receiver<T>> {
        self.0.blocking_dispatch(api).map(|_| self.1)
    }
}

pub trait PendingMemRead {
    fn recv_for<T>(self) -> impl std::future::Future<Output = anyhow::Result<T>> + Send
    where
        T: dataview::Pod + Default;
    fn recv_then<T>(self, callback: fn(anyhow::Result<T>))
    where
        T: dataview::Pod + Default;
}

impl PendingMemRead for oneshot::Receiver<anyhow::Result<Vec<u8>>> {
    async fn recv_for<T>(self) -> anyhow::Result<T>
    where
        T: dataview::Pod + Default,
    {
        self.await?
            .map(|data| {
                let mut out: T = T::default();
                dataview::bytes_mut(&mut out).copy_from_slice(&data);
                out
            })
            .map_err(|e| {
                tracing::error!(%e, ?e);
                e
            })
    }

    fn recv_then<T>(self, callback: fn(anyhow::Result<T>))
    where
        T: dataview::Pod + Default,
    {
        tokio::spawn(async move {
            let result = self.recv_for::<T>().await;
            callback(result)
        });
    }
}

pub trait PendingMemWrite {
    fn spawn_err_handler(self);
}

impl PendingMemWrite for oneshot::Receiver<anyhow::Result<()>> {
    fn spawn_err_handler(self) {
        tokio::spawn(async move {
            match self.await {
                Ok(r) => {
                    if let Err(e) = r {
                        tracing::error!(%e, ?e);
                    }
                }
                Err(e) => tracing::error!(%e, ?e),
            }
        });
    }
}
