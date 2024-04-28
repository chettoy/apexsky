use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::thread::sleep;
use std::time::{Duration, Instant};

use obfstr::obfstr as s;
use tokio::sync::{mpsc, oneshot, watch};
use tracing::instrument;

use crate::mem::{
    memflow_impl::MemflowOs, memprocfs_impl::MemProcFsOs, MemOs, MemProc, MemProcImpl,
    ProcessStatus,
};
use crate::press_to_exit;

pub trait MemAccess {
    async fn mem_baseaddr(&mut self) -> Option<u64>;
    async fn mem_read<T: dataview::Pod + ?Sized>(
        &mut self,
        addr: u64,
        dest: &mut T,
    ) -> anyhow::Result<()>;
    async fn mem_write<T: dataview::Pod + ?Sized>(
        &mut self,
        addr: u64,
        data: &T,
    ) -> anyhow::Result<()>;
}

pub trait MemAccessPro {
    async fn req_mem_read<T: dataview::Pod + ?Sized>(
        &mut self,
        addr: u64,
        dest: &mut T,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()>;
    async fn req_mem_write<T: dataview::Pod + ?Sized>(
        &mut self,
        addr: u64,
        data: &T,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()>;
    async fn req_flush(&mut self, priority_threshold: i32) -> anyhow::Result<usize>;
}

type MemApi = mpsc::Sender<PriorityAccess>;

#[derive(Debug)]
pub struct PriorityAccess {
    priority: i32,
    req: AccessRequest,
}

#[derive(Debug)]
pub enum AccessRequest {
    FlushRequests(FlushRequestsRequest),
    MemBaseaddr(MemBaseaddrRequest),
    MemRead(MemReadRequest),
    MemWrite(MemWriteRequest),
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

impl MemAccess for MemApi {
    #[instrument(skip_all)]
    async fn mem_baseaddr(&mut self) -> Option<u64> {
        let (future_tx, rx) = oneshot::channel();
        if let Err(e) = self
            .send(PriorityAccess {
                priority: 0,
                req: AccessRequest::MemBaseaddr(MemBaseaddrRequest { future_tx }),
            })
            .await
        {
            tracing::error!(%e, ?e);
            return None;
        }
        rx.await.unwrap_or_else(|e| {
            tracing::error!(%e, ?e);
            None
        })
    }

    #[instrument(skip_all)]
    async fn mem_read<T: dataview::Pod + ?Sized>(
        &mut self,
        addr: u64,
        dest: &mut T,
    ) -> anyhow::Result<()> {
        self.req_mem_read(addr, dest, 0, 0).await
    }

    #[instrument(skip_all)]
    async fn mem_write<T: dataview::Pod + ?Sized>(
        &mut self,
        addr: u64,
        data: &T,
    ) -> anyhow::Result<()> {
        self.req_mem_write(addr, data, 0, 0).await
    }
}

impl MemAccessPro for MemApi {
    async fn req_mem_read<T: dataview::Pod + ?Sized>(
        &mut self,
        addr: u64,
        dest: &mut T,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let dest = dataview::bytes_mut(dest);
        let (future_tx, rx) = oneshot::channel();
        self.send(PriorityAccess {
            priority,
            req: AccessRequest::MemRead(MemReadRequest {
                address: addr,
                data_size: dest.len(),
                id: req_id,
                future_tx,
            }),
        })
        .await
        .map_err(|e| {
            tracing::error!(%e, ?e);
            e
        })?;

        let data = rx.await.map_err(|e| {
            tracing::error!(%e, ?e);
            e
        })??;
        dest.copy_from_slice(&data);

        Ok(())
    }

    async fn req_mem_write<T: dataview::Pod + ?Sized>(
        &mut self,
        addr: u64,
        data: &T,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let data = dataview::bytes(data);
        let (future_tx, rx) = oneshot::channel();
        self.send(PriorityAccess {
            priority,
            req: AccessRequest::MemWrite(MemWriteRequest {
                address: addr,
                write: data.to_vec(),
                id: req_id,
                future_tx,
            }),
        })
        .await
        .map_err(|e| {
            tracing::error!(%e, ?e);
            e
        })?;

        rx.await.map_err(|e| {
            tracing::error!(%e, ?e);
            e
        })?
    }

    async fn req_flush(&mut self, priority_threshold: i32) -> anyhow::Result<usize> {
        let (future_tx, rx) = oneshot::channel();
        self.send(PriorityAccess {
            priority: 0,
            req: AccessRequest::FlushRequests(FlushRequestsRequest {
                priority_threshold,
                future_tx,
            }),
        })
        .await
        .map_err(|e| {
            tracing::error!(%e, ?e);
            e
        })?;

        let count = rx.await.map_err(|e| {
            tracing::error!(%e, ?e);
            e
        })?;

        Ok(count)
    }
}

#[instrument(skip_all)]
pub fn io_thread(
    active: watch::Receiver<bool>,
    mut access_rx: mpsc::Receiver<PriorityAccess>,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));

    let mut accessible: bool = false;
    let mut priority_queue: BinaryHeap<PriorityAccess> = BinaryHeap::new();
    let mut scatter_map: HashMap<usize, Vec<MemReadRequest>> = HashMap::new();
    let mut start_instant = Instant::now();

    let connector: String = choose_connector();
    let Some(mut mem_os) = create_os_instance(connector) else {
        press_to_exit();
        return Ok(());
    };

    while *active.borrow() {
        let Some(mut mem) = find_game_process(&mut mem_os) else {
            accessible = false;
            sleep(Duration::from_secs(2));
            continue;
        };
        if mem.check_proc_status() != ProcessStatus::FoundReady {
            accessible = false;
            sleep(Duration::from_secs(2));
            continue;
        }
        if !accessible {
            println!("{}", s!("Apex process found"));
            println!("{}{:x}", s!("Base: 0x"), mem.get_proc_baseaddr());

            tracing::debug!("{}", s!("speed_test"));
            mem.speed_test();
            println!("{}", s!("Press enter to continue.."));
            tracing::debug!("{}", s!("press to continue"));
            let _ = std::io::stdin().read_line(&mut String::new());

            accessible = true;
        }

        while *active.borrow() {
            sleep(Duration::from_millis(2));

            let loop_duration = start_instant.elapsed().as_millis();
            start_instant = Instant::now();

            if mem.check_proc_status() != ProcessStatus::FoundReady {
                accessible = false;
                sleep(Duration::from_secs(2));
                break;
            }

            // Receive all requests and store to queue
            loop {
                match access_rx.try_recv() {
                    Ok(req) => priority_queue.push(req),
                    Err(e) => {
                        match e {
                            mpsc::error::TryRecvError::Empty => (),
                            mpsc::error::TryRecvError::Disconnected => {
                                tracing::error!(%e, ?e);
                            }
                        }
                        break;
                    }
                }
            }

            // Execution of the requests

            // flush requests
            let flush_report =
                consume_requests(None, &mut priority_queue, &mut scatter_map, &mut mem);

            // try scatter read
            if !scatter_map.is_empty() {
                // memory ranges to read are tuples:
                // .0 = the virtual address to read.
                // .1 = vector of u8 which memory should be read into.
                // .2 = u32 receiving the bytes successfully read data.
                let mut memory_range_list: Vec<(
                    (u64, Vec<u8>, u32),
                    MemReadRequest,
                    Option<anyhow::Error>,
                )> = Vec::new();
                // the prepare_ex function will populate the prepared data regions automatically when the
                // VmmScatterMemory is dropped.
                if let Some(mut mem_scatter) = mem.get_vmm_scatter() {
                    scatter_map.drain().for_each(|(_id, req_arr)| {
                        req_arr.into_iter().for_each(|r| {
                            memory_range_list.push((
                                (r.address, vec![0u8; r.data_size], 0u32),
                                r,
                                None,
                            ));
                        });
                    });
                    memory_range_list
                        .iter_mut()
                        .for_each(|(memory_range_i, _req, err)| {
                            if let Err(e) = mem_scatter.prepare_ex(memory_range_i) {
                                err.replace(e);
                            }
                        });
                }
                memory_range_list
                    .into_iter()
                    .for_each(|(memory_range_i, req, err)| {
                        let result = match err {
                            Some(e) => Err(e),
                            None => {
                                if memory_range_i.2 as usize == req.data_size {
                                    Ok(memory_range_i.1)
                                } else {
                                    Err(anyhow::anyhow!(
                                        "{}{}{}{}",
                                        s!("partial read "),
                                        memory_range_i.2,
                                        s!("/"),
                                        req.data_size
                                    ))
                                }
                            }
                        };
                        let _ = req.future_tx.send(result);
                    });
            }

            // try normal read
            scatter_map.drain().for_each(|(_id, req_arr)| {
                req_arr.into_iter().for_each(|r| {
                    let mut buf = vec![0; r.data_size];
                    let _ = r
                        .future_tx
                        .send(mem.read_raw_into(r.address, &mut buf).map(|_| buf));
                });
            });

            // callback
            flush_respond(flush_report);
        }
    }

    tracing::debug!("{}", s!("task end"));
    Ok(())
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
    priority_queue: &mut BinaryHeap<PriorityAccess>,
    scatter_map: &mut HashMap<usize, Vec<MemReadRequest>>,
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
        .unwrap_or(0);

    loop {
        let Some(access) = priority_queue.peek() else {
            break;
        };
        if access.priority < priority_threshold {
            break;
        }
        match priority_queue.pop().unwrap().req {
            AccessRequest::MemBaseaddr(r) => {
                let _ = r.future_tx.send(match mem.get_proc_baseaddr() {
                    0 => None,
                    a => Some(a),
                });
                flush_report.done_count += 1;
            }
            AccessRequest::MemRead(r) => {
                match scatter_map.get_mut(&r.id) {
                    Some(arr) => {
                        arr.push(r);
                    }
                    None => {
                        scatter_map.insert(r.id, vec![r]);
                    }
                }
                flush_report.pending_count += 1;
            }
            AccessRequest::MemWrite(r) => {
                let _ = r.future_tx.send(mem.write_raw(r.address, &r.write));
                flush_report.done_count += 1;
            }
            AccessRequest::FlushRequests(r) => {
                flush_report.children.push(consume_requests(
                    Some(r),
                    priority_queue,
                    scatter_map,
                    mem,
                ));
                flush_report.pending_count += 1;
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

fn choose_connector() -> String {
    let mut connector = s!("dma").to_string();
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        if args[1] == s!("kvm") {
            connector = s!("kvm").to_string();
        } else if args[1] == s!("no-kvm")
            || args[1] == s!("nokvm")
            || args[1] == s!("nodma")
            || args[1] == s!("linux")
            || args[1] == s!("native")
        {
            connector = s!("native").to_string();
        }
    }
    connector
}

fn create_os_instance(connector: String) -> Option<Box<dyn MemOs>> {
    if connector == s!("dma") {
        match MemProcFsOs::new(&connector) {
            Ok(os) => Some(Box::new(os)),
            Err(e) => {
                tracing::error!(?e, "{}", s!("open_os"));
                None
            }
        }
    } else {
        match MemflowOs::new(&connector) {
            Ok(os) => Some(Box::new(os)),
            Err(e) => {
                tracing::error!(?e, "{}", s!("open_os"));
                None
            }
        }
    }
}

fn find_game_process(mem_os: &mut Box<dyn MemOs>) -> Option<MemProcImpl<'_>> {
    tracing::warn!(parent: None, "{}", s!("Searching for apex process..."));
    mem_os
        .open_proc(s!("r5apex.exe").to_string())
        .map(Some)
        .unwrap_or_else(|e| {
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
