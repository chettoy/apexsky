use super::api::CDmalibAccessConfig;
use super::sky;
use crate::common::dmalib::{DmalibAccessTarget, FindSigResult, IMemAccess};

pub use crate::common::dmalib::{PRIO_HIGH, PRIO_LOW, PRIO_PASSIVE, PRIO_PREEMPT};

#[derive(Debug, Clone)]
pub struct MemAccess(u64);

impl MemAccess {
    pub const fn into_handle(self) -> u64 {
        self.0
    }

    pub const fn from_handle(handle: u64) -> Self {
        Self(handle)
    }
}

impl<'a> From<&'a DmalibAccessTarget> for CDmalibAccessConfig<'a> {
    fn from(val: &'a DmalibAccessTarget) -> Self {
        CDmalibAccessConfig {
            target_process_name: val.target_process_name.as_str().into(),
            override_module_base: val.override_module_base.unwrap_or(0),
            check_time_date_stamp: val.check_time_date_stamp.unwrap_or(0),
            speed_test: val.speed_test,
            cache_phys_addr: val.cache_phys_addr,
        }
    }
}

impl IMemAccess for MemAccess {
    fn open(target: &DmalibAccessTarget) -> anyhow::Result<Self> {
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
    ) -> anyhow::Result<bytes::Bytes> {
        let ret = sky!(.dmalib.mem_read_async)(self.0, addr, len, priority, req_id).await;
        if ret.is_empty() {
            anyhow::bail!("fail");
        }
        Ok(convert_ffi_bytes(ret))
    }

    fn read_raw_blocking(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<bytes::Bytes> {
        let ret = sky!(.dmalib.mem_read_blocking)(self.0, addr, len, priority, req_id);
        if ret.is_empty() {
            anyhow::bail!("fail");
        }
        Ok(convert_ffi_bytes(ret))
    }

    async fn read_raw_list(
        &self,
        list: &mut Vec<(u64, usize, Option<bytes::Bytes>)>,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let read_list = list
            .iter()
            .map(|(addr, len, _out)| safer_ffi::Tuple3 {
                _0: *addr,
                _1: *len,
                _2: safer_ffi::bytes::Bytes::empty(),
            })
            .collect::<Vec<_>>();

        let ret =
            sky!(.dmalib.mem_read_list_async)(self.0, read_list.into(), priority, req_id).await;

        assert_eq!(ret.len(), list.len());

        for ((_addr, _len, out), safer_ffi::Tuple3 { _0, _1, _2: data }) in
            list.iter_mut().zip(Vec::from(ret))
        {
            *out = if data.is_empty() {
                None
            } else {
                Some(convert_ffi_bytes(data))
            };
        }

        Ok(())
    }

    fn read_raw_list_blocking(
        &self,
        list: &mut Vec<(u64, usize, Option<bytes::Bytes>)>,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let read_list = list
            .iter()
            .map(|(addr, len, _out)| safer_ffi::Tuple3 {
                _0: *addr,
                _1: *len,
                _2: safer_ffi::bytes::Bytes::empty(),
            })
            .collect::<Vec<_>>();

        let ret = sky!(.dmalib.mem_read_list_blocking)(self.0, read_list.into(), priority, req_id);

        for ((_addr, _len, out), safer_ffi::Tuple3 { _0, _1, _2: data }) in
            list.iter_mut().zip(Vec::from(ret))
        {
            *out = if data.is_empty() {
                None
            } else {
                Some(convert_ffi_bytes(data))
            };
        }

        Ok(())
    }

    async fn write_raw(
        &self,
        addr: u64,
        data: &[u8],
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let ret = sky!(.dmalib.mem_write_async)(
            self.0,
            addr,
            safer_ffi::bytes::Bytes::copied_from_slice(data),
            priority,
            req_id,
        )
        .await;
        if !ret {
            anyhow::bail!("fail");
        } else {
            Ok(())
        }
    }

    fn write_raw_blocking(
        &self,
        addr: u64,
        data: &[u8],
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
        sig: &str,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Option<FindSigResult>> {
        let sig = safer_ffi::bytes::Bytes::copied_from_slice(sig.as_bytes());
        let ret = sky!(.dmalib.mem_find_sig_async)(self.0, sig, start, end)
            .await
            .into_anyhow()?;
        Ok(ret
            .into_rust()
            .map(|safer_ffi::Tuple2 { _0, _1 }| FindSigResult {
                address: _0,
                data: convert_ffi_bytes(_1),
            }))
    }

    fn find_sig_blocking(
        &self,
        sig: &str,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Option<FindSigResult>> {
        let ret =
            sky!(.dmalib.mem_find_sig_blocking)(self.0, sig.into(), start, end).into_anyhow()?;
        Ok(ret
            .into_rust()
            .map(|safer_ffi::Tuple2 { _0, _1 }| FindSigResult {
                address: _0,
                data: convert_ffi_bytes(_1),
            }))
    }

    async fn dump(&self) -> anyhow::Result<bytes::Bytes> {
        let ret = sky!(.dmalib.mem_dump_async)(self.0).await.into_anyhow()?;
        Ok(convert_ffi_bytes(ret))
    }

    fn dump_blocking(&self) -> anyhow::Result<bytes::Bytes> {
        let ret = sky!(.dmalib.mem_dump_blocking)(self.0).into_anyhow()?;
        Ok(convert_ffi_bytes(ret))
    }
}

fn convert_ffi_bytes(ffi_bytes: safer_ffi::bytes::Bytes<'static>) -> bytes::Bytes {
    ffi_bytes.to_vec().into()
}
