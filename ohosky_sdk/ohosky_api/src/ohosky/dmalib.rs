use std::time::Duration;

use async_trait::async_trait;

use super::api::bindings::ohosky::main::{
    access::{self, TargetConfig},
    access_async_result,
};
use super::ConvertWasmResultToAnyhow;
use crate::common::dmalib::{DmalibAccessTarget, FindSigResult, IMemAccess};

pub use crate::common::dmalib::{PRIO_HIGH, PRIO_LOW, PRIO_PASSIVE, PRIO_PREEMPT};

#[derive(Debug, Clone)]
pub struct MemAccess(u32);

impl From<MemAccess> for u32 {
    fn from(val: MemAccess) -> Self {
        val.0
    }
}

impl From<u32> for MemAccess {
    fn from(val: u32) -> Self {
        MemAccess(val)
    }
}

impl From<DmalibAccessTarget> for TargetConfig {
    fn from(val: DmalibAccessTarget) -> Self {
        TargetConfig {
            target_process_name: val.target_process_name,
            override_module_base: val.override_module_base,
            check_time_date_stamp: val.check_time_date_stamp,
            speed_test: val.speed_test,
            cache_phys_addr: val.cache_phys_addr,
        }
    }
}

#[async_trait]
impl IMemAccess for MemAccess {
    fn open(target: DmalibAccessTarget) -> anyhow::Result<Self> {
        let handle = access::mem_open(&target.into()).to_anyhow()?;
        Ok(Self(handle))
    }

    async fn get_baseaddr(&self, priority: i32) -> anyhow::Result<Option<u64>> {
        let id = access::mem_baseaddr_async_call(self.0, priority).to_anyhow()?;
        loop {
            if let Some(ret) = access_async_result::mem_baseaddr_async_result(id) {
                break ret.to_anyhow();
            }
            tokio::time::sleep(Duration::from_micros(100)).await;
        }
    }

    fn get_baseaddr_blocking(&self, priority: i32) -> anyhow::Result<Option<u64>> {
        access::mem_baseaddr(self.0, priority).to_anyhow()
    }

    async fn read_raw(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        _req_id: usize,
    ) -> anyhow::Result<Vec<u8>> {
        let rx = access::mem_read_async_call(self.0, addr, len as u64, priority).to_anyhow()?;
        loop {
            if let Some(ret) = rx.try_recv() {
                break ret.to_anyhow();
            }
            tokio::time::sleep(Duration::from_micros(100)).await;
        }
    }

    fn read_raw_blocking(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        _req_id: usize,
    ) -> anyhow::Result<Vec<u8>> {
        access::mem_read(self.0, addr, len as u64, priority).to_anyhow()
    }

    async fn write_raw(
        &self,
        addr: u64,
        data: Vec<u8>,
        priority: i32,
        _req_id: usize,
    ) -> anyhow::Result<()> {
        let id = access::mem_write_async_call(self.0, addr, &data, priority).to_anyhow()?;
        loop {
            if let Some(ret) = access_async_result::mem_write_async_result(id) {
                break ret.to_anyhow();
            }
            tokio::time::sleep(Duration::from_micros(100)).await;
        }
    }

    fn write_raw_blocking(
        &self,
        addr: u64,
        data: Vec<u8>,
        priority: i32,
        _req_id: usize,
    ) -> anyhow::Result<()> {
        access::mem_write(self.0, addr, &data, priority).to_anyhow()
    }

    async fn find_sig(
        &self,
        _sig: String,
        _start: u64,
        _end: u64,
    ) -> anyhow::Result<Option<FindSigResult>> {
        tracing::error!("{}", obfstr::obfstr!("not-implemented"));
        anyhow::bail!(obfstr::obfstr!("not-implemented").to_string());
    }

    fn find_sig_blocking(
        &self,
        _sig: String,
        _start: u64,
        _end: u64,
    ) -> anyhow::Result<Option<FindSigResult>> {
        tracing::error!("{}", obfstr::obfstr!("not-implemented"));
        anyhow::bail!(obfstr::obfstr!("not-implemented").to_string());
    }

    async fn dump(&self) -> anyhow::Result<Vec<u8>> {
        tracing::error!("{}", obfstr::obfstr!("not-implemented"));
        anyhow::bail!(obfstr::obfstr!("not-implemented").to_string());
    }

    fn dump_blocking(&self) -> anyhow::Result<Vec<u8>> {
        tracing::error!("{}", obfstr::obfstr!("not-implemented"));
        anyhow::bail!(obfstr::obfstr!("not-implemented").to_string());
    }
}
