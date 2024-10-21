use async_trait::async_trait;

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
    pub data: Vec<u8>,
}

#[async_trait]
pub trait IMemAccess: Sized {
    fn open(target: DmalibAccessTarget) -> anyhow::Result<Self>;

    async fn get_baseaddr(&self, priority: i32) -> anyhow::Result<Option<u64>>;

    fn get_baseaddr_blocking(&self, priority: i32) -> anyhow::Result<Option<u64>>;

    async fn read_raw(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<Vec<u8>>;

    fn read_raw_blocking(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<Vec<u8>>;

    async fn read<T: dataview::Pod + Default>(
        &self,
        addr: u64,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<T> {
        self.read_raw(addr, size_of::<T>(), priority, req_id)
            .await
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

    async fn write_raw(
        &self,
        addr: u64,
        data: Vec<u8>,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()>;

    fn write_raw_blocking(
        &self,
        addr: u64,
        data: Vec<u8>,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()>;

    fn write<T: dataview::Pod>(
        &self,
        addr: u64,
        data: &T,
        priority: i32,
        req_id: usize,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> {
        let data = dataview::bytes(data);
        self.write_raw(addr, data.to_vec(), priority, req_id)
    }

    async fn find_sig(
        &self,
        sig: String,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Option<FindSigResult>>;

    fn find_sig_blocking(
        &self,
        sig: String,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Option<FindSigResult>>;

    async fn dump(&self) -> anyhow::Result<Vec<u8>>;

    fn dump_blocking(&self) -> anyhow::Result<Vec<u8>>;
}
