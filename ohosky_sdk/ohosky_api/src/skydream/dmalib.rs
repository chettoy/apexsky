use async_trait::async_trait;

use super::api::CDmalibAccessTarget;
use super::sky;
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

impl From<DmalibAccessTarget> for CDmalibAccessTarget {
    fn from(val: DmalibAccessTarget) -> Self {
        CDmalibAccessTarget {
            target_process_name: val.target_process_name.into(),
            override_module_base: val.override_module_base.unwrap_or(0),
            check_time_date_stamp: val.check_time_date_stamp.unwrap_or(0),
            speed_test: val.speed_test,
            cache_phys_addr: val.cache_phys_addr,
        }
    }
}

#[async_trait]
impl IMemAccess for MemAccess {
    fn open(target: DmalibAccessTarget) -> anyhow::Result<Self> {
        let handle = sky!(.dmalib.mem_open)(target.into());
        if handle == 0 {
            anyhow::bail!("fail");
        }
        Ok(Self(handle))
    }

    async fn get_baseaddr(&self, priority: i32) -> anyhow::Result<Option<u64>> {
        match sky!(.dmalib.mem_baseaddr_async)(self.0, priority)
            .await
            .into_rust()
        {
            None => anyhow::bail!("fail"),
            Some(0) => Ok(None),
            Some(a) => Ok(Some(a)),
        }
    }

    fn get_baseaddr_blocking(&self, priority: i32) -> anyhow::Result<Option<u64>> {
        match sky!(.dmalib.mem_baseaddr_blocking)(self.0, priority).into_rust() {
            None => anyhow::bail!("fail"),
            Some(0) => Ok(None),
            Some(a) => Ok(Some(a)),
        }
    }

    async fn read_raw(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<Vec<u8>> {
        let ret = sky!(.dmalib.mem_read_async)(self.0, addr, len, priority, req_id).await;
        if ret.is_empty() {
            anyhow::bail!("fail");
        }
        Ok(ret.into())
    }

    fn read_raw_blocking(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<Vec<u8>> {
        let ret = sky!(.dmalib.mem_read_blocking)(self.0, addr, len, priority, req_id);
        if ret.is_empty() {
            anyhow::bail!("fail");
        }
        Ok(ret.into())
    }

    async fn write_raw(
        &self,
        addr: u64,
        data: Vec<u8>,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let ret = sky!(.dmalib.mem_write_async)(self.0, addr, data.into(), priority, req_id).await;
        if !ret {
            anyhow::bail!("fail");
        } else {
            Ok(())
        }
    }

    fn write_raw_blocking(
        &self,
        addr: u64,
        data: Vec<u8>,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let ret = sky!(.dmalib.mem_write_blocking)(self.0, addr, data.into(), priority, req_id);
        if !ret {
            anyhow::bail!("fail");
        } else {
            Ok(())
        }
    }

    async fn find_sig(
        &self,
        sig: String,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Option<FindSigResult>> {
        let ret = sky!(.dmalib.mem_find_sig_async)(self.0, sig.into(), start, end)
            .await
            .into_anyhow()?;
        Ok(ret
            .into_rust()
            .map(|safer_ffi::Tuple2 { _0, _1 }| FindSigResult {
                address: _0,
                data: _1.into(),
            }))
    }

    fn find_sig_blocking(
        &self,
        sig: String,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Option<FindSigResult>> {
        let ret =
            sky!(.dmalib.mem_find_sig_blocking)(self.0, sig.into(), start, end).into_anyhow()?;
        Ok(ret
            .into_rust()
            .map(|safer_ffi::Tuple2 { _0, _1 }| FindSigResult {
                address: _0,
                data: _1.into(),
            }))
    }

    async fn dump(&self) -> anyhow::Result<Vec<u8>> {
        let ret = sky!(.dmalib.mem_dump_async)(self.0).await.into_anyhow()?;
        Ok(ret.into())
    }

    fn dump_blocking(&self) -> anyhow::Result<Vec<u8>> {
        let ret = sky!(.dmalib.mem_dump_blocking)(self.0).into_anyhow()?;
        Ok(ret.into())
    }
}

impl MemAccess {
    pub(crate) fn _mem_read_batch_blocking(
        &self,
        batch: Vec<(u64, &mut [u8])>,
        priority: i32,
        req_id: usize,
    ) -> usize {
        let batch = batch
            .into_iter()
            .map(|(addr, out)| safer_ffi::Tuple2 {
                _0: addr,
                _1: safer_ffi::slice::Mut::from(out),
            })
            .collect::<Vec<_>>()
            .into();
        sky!(.dmalib.mem_read_batch_blocking)(self.0, batch, priority, req_id)
    }
}
