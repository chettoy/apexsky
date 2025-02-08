pub const PRIO_PREEMPT: i32 = 0x10;
pub const PRIO_HIGH: i32 = 1;
pub const PRIO_LOW: i32 = 0;
pub const PRIO_PASSIVE: i32 = -1;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DmalibAccessTarget {
    pub target_process_name: String,
    pub override_module_base: Option<u64>,
    pub check_time_date_stamp: Option<u32>,
    pub speed_test: bool,
    pub cache_phys_addr: bool,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FindSigResult {
    pub address: u64,
    pub data: bytes::Bytes,
}

#[allow(async_fn_in_trait)]
pub trait IMemAccess: Sized {
    fn open(target: &DmalibAccessTarget) -> anyhow::Result<Self>;

    async fn get_baseaddr(&self, priority: i32) -> anyhow::Result<Option<u64>>;

    fn get_baseaddr_blocking(&self, priority: i32) -> anyhow::Result<Option<u64>>;

    async fn read_raw(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<bytes::Bytes>;

    fn read_raw_blocking(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<bytes::Bytes>;

    async fn read_raw_into(
        &self,
        addr: u64,
        out: &mut [u8],
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()>;

    fn read_raw_into_blocking(
        &self,
        addr: u64,
        out: &mut [u8],
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()>;

    async fn read_raw_list(
        &self,
        list: &mut Vec<(u64, usize, Option<bytes::Bytes>)>,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()>;

    fn read_raw_list_blocking(
        &self,
        list: &mut Vec<(u64, usize, Option<bytes::Bytes>)>,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()>;

    #[inline]
    async fn read<T: zerocopy::FromBytes>(
        &self,
        addr: u64,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<T> {
        self.read_raw(addr, size_of::<T>(), priority, req_id)
            .await
            .map(|bytes| T::read_from_bytes(&bytes).unwrap())
    }

    async fn write_raw(
        &self,
        addr: u64,
        data: &[u8],
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()>;

    fn write_raw_blocking(
        &self,
        addr: u64,
        data: &[u8],
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()>;

    #[inline]
    async fn write<T: zerocopy::IntoBytes + zerocopy::Immutable>(
        &self,
        addr: u64,
        data: &T,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        self.write_raw(addr, data.as_bytes(), priority, req_id)
            .await
    }

    async fn find_sig(
        &self,
        sig: &str,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Option<FindSigResult>>;

    fn find_sig_blocking(
        &self,
        sig: &str,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Option<FindSigResult>>;

    async fn dump(&self) -> anyhow::Result<bytes::Bytes>;

    fn dump_blocking(&self) -> anyhow::Result<bytes::Bytes>;
}
