use dataview::Pod;
use enum_dispatch::enum_dispatch;
use memflow_impl::{MemflowOs, MemflowProc};
use memprocfs_impl::{MemProcFSProc, MemProcFsOs};

pub mod dma_helper;
pub mod memflow_impl;
pub mod memprocfs_impl;

pub trait MemOs: Send + Sync {
    fn open_proc(&mut self, name: String) -> anyhow::Result<MemProcImpl>;
}

#[enum_dispatch]
pub trait MemProc: Send + Sync {
    fn get_proc_baseaddr(&self) -> u64;
    fn check_proc_status(&mut self) -> ProcessStatus;
    fn speed_test(&mut self);
    fn read_raw_into(&mut self, addr: u64, out: &mut [u8]) -> anyhow::Result<()>;
    fn write_raw(&mut self, addr: u64, data: &[u8]) -> anyhow::Result<()>;
}

pub enum MemOsImpl {
    Memflow(MemflowOs),
    Vmm(MemProcFsOs),
}

impl MemOs for MemOsImpl {
    fn open_proc(&mut self, name: String) -> anyhow::Result<MemProcImpl> {
        match self {
            MemOsImpl::Memflow(inner) => inner.open_proc(name),
            MemOsImpl::Vmm(inner) => inner.open_proc(name),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MemConnector {
    MemflowKvm,
    MemflowNative,
    MemflowPCILeech,
    PCILeech(String),
}

#[derive(Debug)]
#[enum_dispatch(MemProc)]
pub enum MemProcImpl<'a> {
    Memflow(MemflowProc<'a>),
    Vmm(MemProcFSProc<'a>),
}

impl<'a> MemProcImpl<'a> {
    #[inline]
    pub fn read<T: Pod>(&mut self, addr: u64) -> anyhow::Result<T> {
        let mut dest: T = dataview::zeroed();
        self.read_into(addr, &mut dest).map(|()| dest)
    }

    #[inline]
    pub fn read_into<T: Pod + ?Sized>(&mut self, addr: u64, out: &mut T) -> anyhow::Result<()> {
        self.read_raw_into(addr, dataview::bytes_mut(out))
    }

    #[allow(dead_code)]
    #[inline]
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
