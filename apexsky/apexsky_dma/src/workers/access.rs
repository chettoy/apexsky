use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

type MemApi = mpsc::Sender<PriorityAccess>;

pub struct PriorityAccess {
    priority: u32,
    req: AccessRequest,
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

pub enum AccessRequest {
    MemBaseaddr(MemBaseaddrRequest),
    MemRead(MemReadRequest),
    MemWrite(MemWriteRequest),
}

pub struct MemBaseaddrRequest {
    future_tx: oneshot::Sender<Option<u64>>,
}

pub struct MemReadRequest {
    address: u64,
    data_size: usize,
    id: usize,
    future_tx: oneshot::Sender<anyhow::Result<Vec<u8>>>,
}

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
        let dest = dataview::bytes_mut(dest);
        let (future_tx, rx) = oneshot::channel();
        self.send(PriorityAccess {
            priority: 0,
            req: AccessRequest::MemRead(MemReadRequest {
                address: addr,
                data_size: dest.len(),
                id: 0,
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

    #[instrument(skip_all)]
    async fn mem_write<T: dataview::Pod + ?Sized>(
        &mut self,
        addr: u64,
        data: &T,
    ) -> anyhow::Result<()> {
        let data = dataview::bytes(data);
        let (future_tx, rx) = oneshot::channel();
        self.send(PriorityAccess {
            priority: 0,
            req: AccessRequest::MemWrite(MemWriteRequest {
                address: addr,
                write: data.to_vec(),
                id: 0,
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
}

#[instrument(skip_all)]
pub fn io_thread(
    active: watch::Receiver<bool>,
    mut access_rx: mpsc::Receiver<PriorityAccess>,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));

    let mut accessible = false;
    let mut priority_queue = BinaryHeap::new();
    let mut start_instant = Instant::now();

    let connector = choose_connector();
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
                break;
            }

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

            todo!();
            // loop {
            //     let req = priority_queue.pop();
            // }
        }
    }

    tracing::debug!("{}", s!("task end"));
    Ok(())
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
