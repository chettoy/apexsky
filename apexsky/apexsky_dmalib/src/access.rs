use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::thread::{sleep, sleep_until};
use std::time::{Duration, Instant};

use obfstr::obfstr as s;
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

type ScatterRequestMap = (
    HashMap<usize, Vec<MemReadRequest>>,
    HashMap<usize, Vec<MemWriteRequest>>,
);

#[instrument(skip_all)]
pub fn io_thread(
    active: watch::Receiver<bool>,
    access_rx: crossbeam_channel::Receiver<PriorityAccess>,
    mem_connector: crate::mem::MemConnector,
    target_proc_name: String,
) -> Result<(), AccessError> {
    tracing::debug!("{}", s!("task start"));

    let mut speed_test_done = false;
    let mut accessible: bool = false;
    let mut priority_queue: BinaryHeap<PriorityAccess> = BinaryHeap::new();
    let mut scatter_map: ScatterRequestMap = ScatterRequestMap::default();
    let mut start_instant;
    let mut next_flush_instant = Instant::now();

    let mut mem_os = create_os_instance(mem_connector.clone())
        .map_err(|e| AccessError::Connector(mem_connector, e))?;

    while *active.borrow() {
        start_instant = Instant::now();

        // Fallback response
        if !accessible {
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
                            r.future_tx
                                .send(Err(anyhow::anyhow!("{}", s!("!accessible"))))
                                .ok();
                        }
                        AccessType::MemWrite(r) => {
                            r.future_tx
                                .send(Err(anyhow::anyhow!("{}", s!("!accessible"))))
                                .ok();
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
        let Some(mut mem) = find_target_process(&mut mem_os, target_proc_name.to_owned()) else {
            accessible = false;
            sleep_until(start_instant + Duration::from_secs(2));
            continue;
        };
        // Check
        if mem.check_proc_status() != ProcessStatus::FoundReady {
            accessible = false;
            sleep_until(start_instant + Duration::from_secs(2));
            continue;
        }

        // Found and ready
        if !accessible {
            tracing::info!("{}", s!("Apex process found"));
            println!("{}{:x}", s!("Base: 0x"), mem.get_proc_baseaddr());

            if !speed_test_done {
                tracing::debug!("{}", s!("speed_test"));
                mem.speed_test();
                println!("{}", s!("Press enter to continue.."));
                tracing::debug!("{}", s!("press to continue"));
                let _ = std::io::stdin().read_line(&mut String::new());
                speed_test_done = true;
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
            fn push_req(
                req: PriorityAccess,
                priority_queue: &mut BinaryHeap<PriorityAccess>,
                scatter_map: &mut ScatterRequestMap,
                mem: &mut MemProcImpl,
            ) {
                let preempt = req.priority > 0xf;
                priority_queue.push(req);
                if preempt {
                    execute_requests(0x10, priority_queue, scatter_map, mem);
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
                    Ok(req) => push_req(req, &mut priority_queue, &mut scatter_map, &mut mem),
                    Err(e) => {
                        match e {
                            crossbeam_channel::TryRecvError::Empty => {
                                if Instant::now() < next_flush_instant {
                                    // Execute higher priority requests first
                                    if priority_queue.peek().is_some_and(|req| req.priority >= 1) {
                                        execute_requests(
                                            1,
                                            &mut priority_queue,
                                            &mut scatter_map,
                                            &mut mem,
                                        );
                                    }
                                    // Wait until next flush instant
                                    if let Ok(req) = access_rx.recv_deadline(next_flush_instant) {
                                        push_req(
                                            req,
                                            &mut priority_queue,
                                            &mut scatter_map,
                                            &mut mem,
                                        );
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
                        execute_requests(0, &mut priority_queue, &mut scatter_map, &mut mem);
                        break;
                    }
                }
            }
        }
    }

    tracing::debug!("{}", s!("task end"));
    Ok(())
}

fn execute_requests(
    priority_threshold: i32,
    priority_queue: &mut BinaryHeap<PriorityAccess>,
    scatter_map: &mut ScatterRequestMap,
    mem: &mut MemProcImpl,
) {
    // flush requests
    let flush_report = consume_requests(None, priority_threshold, priority_queue, scatter_map, mem);

    // try scatter read
    if !scatter_map.0.is_empty() {
        if let Some(mem_scatter) = mem.get_vmm_scatter() {
            let mut pending_read =
                Vec::with_capacity(scatter_map.0.get(&0).map(|arr0| arr0.len()).unwrap_or(0));
            scatter_map.0.drain().for_each(|(_id, req_arr)| {
                req_arr.into_iter().for_each(|r| {
                    if let Err(e) = mem_scatter.prepare(r.address, r.data_size) {
                        tracing::debug!(%e, ?e);
                        let _ = r.future_tx.send(Err(e));
                    } else {
                        pending_read.push(r);
                    }
                });
            });
            match mem_scatter.execute() {
                Ok(_) => {
                    pending_read.into_iter().for_each(|r| {
                        let result = mem_scatter.read(r.address, r.data_size);
                        let _ = r.future_tx.send(result);
                    });
                }
                Err(e) => {
                    pending_read.into_iter().for_each(|r| {
                        let err = Err(anyhow::anyhow!("{}{e}", s!("scatter read failed: ")));
                        let _ = r.future_tx.send(err);
                    });
                }
            }
        }
    }

    // try normal read
    scatter_map.0.drain().for_each(|(_id, req_arr)| {
        req_arr.into_iter().for_each(|r| {
            let mut buf = vec![0; r.data_size];
            let _ = r
                .future_tx
                .send(mem.read_raw_into(r.address, &mut buf).map(|_| buf));
        });
    });

    // try scatter write
    if !scatter_map.1.is_empty() {
        if let Some(mem_scatter) = mem.get_vmm_scatter() {
            let mut pending_tx =
                Vec::with_capacity(scatter_map.1.get(&0).map(|arr0| arr0.len()).unwrap_or(0));
            scatter_map.1.drain().for_each(|(_id, req_arr)| {
                req_arr.into_iter().for_each(|r| {
                    if let Err(e) = mem_scatter.prepare_write(r.address, &r.write) {
                        tracing::warn!(%e, ?e);
                        let _ = r.future_tx.send(Err(e));
                    } else {
                        pending_tx.push(r.future_tx);
                    }
                });
            });
            match mem_scatter.execute() {
                Ok(_) => {
                    pending_tx.into_iter().for_each(|tx| {
                        let _ = tx.send(Ok(()));
                    });
                }
                Err(e) => {
                    pending_tx.into_iter().for_each(|tx| {
                        let err = Err(anyhow::anyhow!("{}{e}", s!("scatter write failed: ")));
                        let _ = tx.send(err);
                    });
                }
            }
        }
    }

    // try normal write
    scatter_map.1.drain().for_each(|(_id, req_arr)| {
        req_arr.into_iter().for_each(|r| {
            let _ = r.future_tx.send(mem.write_raw(r.address, &r.write));
        });
    });

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
    priority_queue: &mut BinaryHeap<PriorityAccess>,
    scatter_map: &mut ScatterRequestMap,
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
                match scatter_map.0.get_mut(&r.id) {
                    Some(arr) => {
                        arr.push(r);
                    }
                    None => {
                        scatter_map.0.insert(r.id, vec![r]);
                    }
                }
                flush_report.pending_count += 1;
            }
            AccessType::MemWrite(r) => {
                match scatter_map.1.get_mut(&r.id) {
                    Some(arr) => {
                        arr.push(r);
                    }
                    None => {
                        scatter_map.1.insert(r.id, vec![r]);
                    }
                }
                flush_report.pending_count += 1;
            }
            AccessType::FlushRequests(r) => {
                flush_report.children.push(consume_requests(
                    Some(r),
                    default_priority_threshold,
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
}

impl AccessRequest for PriorityAccess {
    fn with_priority(mut self, priority: i32) -> PriorityAccess {
        self.priority = priority;
        self
    }

    async fn dispatch(self, api: &MemApi) -> anyhow::Result<()> {
        api.send(self).map_err(|e| {
            let e: anyhow::Error = e.into();
            e.context(s!("Failed to dispatch access request").to_string())
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

    async fn dispatch(self, api: &MemApi) -> anyhow::Result<()> {
        self.with_priority(0x1).dispatch(api).await
    }
}

pub trait PendingAccessRequest<T> {
    fn with_priority(self, priority: i32) -> (PriorityAccess, oneshot::Receiver<T>);
    fn dispatch(
        self,
        api: &MemApi,
    ) -> impl std::future::Future<Output = anyhow::Result<oneshot::Receiver<T>>> + Send;
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
