use dataview::Pod;

use self::{memflow_impl::MemflowProc, memprocfs_impl::MemProcFSProc};

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
    fn read_into<T: Pod + ?Sized>(&mut self, addr: u64, out: &mut T) -> anyhow::Result<()>;
    fn write<T: Pod + ?Sized>(&mut self, addr: u64, data: &T) -> anyhow::Result<()>;
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

    fn read_into<T: Pod + ?Sized>(&mut self, addr: u64, out: &mut T) -> anyhow::Result<()> {
        match self {
            MemProcImpl::Memflow(m) => m.read_into(addr, out),
            MemProcImpl::Vmm(m) => m.read_into(addr, out),
        }
    }

    fn write<T: Pod + ?Sized>(&mut self, addr: u64, data: &T) -> anyhow::Result<()> {
        match self {
            MemProcImpl::Memflow(m) => m.write(addr, data),
            MemProcImpl::Vmm(m) => m.write(addr, data),
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

// pub static G_MEM: Lazy<Mutex<Option<MemProcImpl>>> = Lazy::new(|| Mutex::new(None));

// macro_rules! mem_access {
//     () => {
//         G_MEM.lock().unwrap().unwrap()
//     };
// }

// #[instrument]
// #[no_mangle]
// pub extern "C" fn apex_mem_open_os() -> i32 {
//     match mem_access!().open_os(s!("dma")) {
//         Ok(_) => 0,
//         Err(e) => {
//             error!(error = %e);
//             1
//         }
//     }
// }

// #[instrument]
// #[no_mangle]
// pub extern "C" fn apex_mem_open_proc() -> i32 {
//     let proc_name = String::from(s!("r5apex.exe"));
//     match mem_access!().open_proc(proc_name) {
//         Ok(_) => 0,
//         Err(e) => {
//             error!(error = %e);
//             1
//         }
//     }
// }

// #[no_mangle]
// pub extern "C" fn apex_mem_check_proc() -> ProcessStatus {
//     mem_access!().check_proc_status()
// }

// #[no_mangle]
// pub extern "C" fn apex_mem_baseaddr() -> u64 {
//     mem_access!().get_proc_baseaddr()
// }

// #[no_mangle]
// pub extern "C" fn apex_mem_speed_test() {
//     mem_access!().speed_test()
// }

// #[allow(dead_code)]
// #[inline]
// pub fn apex_mem_read<T: dataview::Pod + Sized + Default>(offset: u64) -> T {
//     let mut apex_mem = mem_access!();
//     let mut v: T = T::default();
//     apex_mem.read_into(offset.into(), &mut v).unwrap();
//     v
// }

// #[allow(dead_code)]
// #[inline]
// pub fn apex_mem_write<T: dataview::Pod + ?Sized>(offset: u64, data: &T) {
//     let mut apex_mem = mem_access!();
//     apex_mem.write(offset.into(), data).unwrap();
// }

#[derive(Debug)]
pub struct ApexMem<'a, 'b> {
    pub base: intptr::IntPtr64,
    pub mem: &'a mut MemProcImpl<'b>,
}

impl<'a, 'b> ApexMem<'a, 'b> {
    pub fn new(mem: &'a mut MemProcImpl<'b>) -> Self {
        Self {
            base: mem.get_proc_baseaddr().into(),
            mem,
        }
    }

    pub fn read_memory<T: dataview::Pod + ?Sized>(&mut self, addr: u64, dest: &mut T) -> i32 {
        match self.mem.read_into(addr.into(), dest) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    pub fn write_memory<T: dataview::Pod + ?Sized>(&mut self, addr: u64, data: &T) -> i32 {
        match self.mem.write(addr.into(), data) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}
