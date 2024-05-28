use dataview::Pod;

use self::{memflow_impl::MemflowProc, memprocfs_impl::MemProcFSProc};

pub mod dma_helper;
pub mod memflow_impl;
pub mod memprocfs_impl;

pub trait MemOs: Send + Sync {
    fn new(choose_connector: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn open_proc<'a>(&'a mut self, name: String) -> anyhow::Result<MemProcImpl>;
}

pub trait MemProc: Send + Sync {
    fn get_proc_baseaddr(&self) -> u64;
    fn check_proc_status(&mut self) -> ProcessStatus;
    fn speed_test(&mut self);
    fn read_raw_into(&mut self, addr: u64, out: &mut [u8]) -> anyhow::Result<()>;
    fn write_raw(&mut self, addr: u64, data: &[u8]) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub enum MemProcImpl<'a> {
    Memflow(MemflowProc<'a>),
    Vmm(MemProcFSProc<'a>),
}

impl<'a> MemProc for MemProcImpl<'a> {
    fn get_proc_baseaddr(&self) -> u64 {
        match self {
            MemProcImpl::Memflow(m) => m.get_proc_baseaddr(),
            MemProcImpl::Vmm(m) => m.get_proc_baseaddr(),
        }
    }

    fn check_proc_status(&mut self) -> ProcessStatus {
        match self {
            MemProcImpl::Memflow(m) => m.check_proc_status(),
            MemProcImpl::Vmm(m) => m.check_proc_status(),
        }
    }

    fn speed_test(&mut self) {
        match self {
            MemProcImpl::Memflow(m) => m.speed_test(),
            MemProcImpl::Vmm(m) => m.speed_test(),
        }
    }

    fn read_raw_into(&mut self, addr: u64, out: &mut [u8]) -> anyhow::Result<()> {
        match self {
            MemProcImpl::Memflow(m) => m.read_raw_into(addr, out),
            MemProcImpl::Vmm(m) => m.read_raw_into(addr, out),
        }
    }

    fn write_raw(&mut self, addr: u64, data: &[u8]) -> anyhow::Result<()> {
        match self {
            MemProcImpl::Memflow(m) => m.write_raw(addr, data),
            MemProcImpl::Vmm(m) => m.write_raw(addr, data),
        }
    }
}

impl<'a> MemProcImpl<'a> {
    pub fn read_into<T: Pod + ?Sized>(&mut self, addr: u64, out: &mut T) -> anyhow::Result<()> {
        self.read_raw_into(addr, dataview::bytes_mut(out))
    }

    pub fn write<T: Pod + ?Sized>(&mut self, addr: u64, data: &T) -> anyhow::Result<()> {
        self.write_raw(addr, dataview::bytes(data))
    }

    #[tracing::instrument(skip_all)]
    pub fn get_vmm_scatter(&mut self) -> Option<memprocfs::VmmScatterMemory> {
        match self {
            MemProcImpl::Memflow(_) => None,
            MemProcImpl::Vmm(m) => m
                .mem_scatter()
                .map_err(|e| {
                    tracing::error!(%e, ?e);
                })
                .ok(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ProcessStatus {
    NotFound,
    FoundNoAccess,
    FoundReady,
}

impl Default for ProcessStatus {
    fn default() -> Self {
        Self::NotFound
    }
}
