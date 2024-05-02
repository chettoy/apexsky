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

pub type MemApi = mpsc::Sender<PriorityAccess>;

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
            AccessType::MemBaseaddr(r) => {
                let _ = r.future_tx.send(match mem.get_proc_baseaddr() {
                    0 => None,
                    a => Some(a),
                });
                flush_report.done_count += 1;
            }
            AccessType::MemRead(r) => {
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
            AccessType::MemWrite(r) => {
                let _ = r.future_tx.send(mem.write_raw(r.address, &r.write));
                flush_report.done_count += 1;
            }
            AccessType::FlushRequests(r) => {
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

pub trait AccessRequest {
    fn with_priority(self, priority: i32) -> PriorityAccess;
    fn dispatch(self, api: &MemApi) -> anyhow::Result<()>;
}

impl AccessRequest for PriorityAccess {
    fn with_priority(mut self, priority: i32) -> PriorityAccess {
        self.priority = priority;
        self
    }

    fn dispatch(self, api: &MemApi) -> anyhow::Result<()> {
        api.blocking_send(self).map_err(|e| {
            tracing::error!(%e, ?e);
            e.into()
        })
    }
}

impl AccessRequest for AccessType {
    fn with_priority(self, priority: i32) -> PriorityAccess {
        PriorityAccess {
            priority,
            req: self,
        }
    }

    fn dispatch(self, api: &MemApi) -> anyhow::Result<()> {
        self.with_priority(0).dispatch(api)
    }
}

pub trait AsyncAccessRequest<T> {
    fn with_priority(self, priority: i32) -> (PriorityAccess, oneshot::Receiver<T>);
    fn dispatch(self, api: &MemApi) -> anyhow::Result<oneshot::Receiver<T>>;
}

impl<T, R> AsyncAccessRequest<T> for (R, oneshot::Receiver<T>)
where
    R: AccessRequest,
{
    fn with_priority(self, priority: i32) -> (PriorityAccess, oneshot::Receiver<T>) {
        (self.0.with_priority(priority), self.1)
    }

    fn dispatch(self, api: &MemApi) -> anyhow::Result<oneshot::Receiver<T>> {
        self.0.dispatch(api).map(|_| self.1)
    }
}

pub trait AsyncMemReadResponse {
    async fn recv_for<T>(self) -> anyhow::Result<T>
    where
        T: dataview::Pod + Default;
    fn recv_then<T>(self, callback: fn(anyhow::Result<T>))
    where
        T: dataview::Pod + Default;
}

impl AsyncMemReadResponse for oneshot::Receiver<anyhow::Result<Vec<u8>>> {
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
