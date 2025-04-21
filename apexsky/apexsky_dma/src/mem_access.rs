use obfstr::obfstr as s;
use ohosky_api::common::dmalib::{DmalibAccessTarget, FindSigResult, IMemAccess};
use ohosky_dmalib::access::{AccessType, MemApi, MemSignature, PendingAccessRequest};

#[derive(Debug, Clone)]
pub struct SimpleMemAccess(pub MemApi);

impl IMemAccess for SimpleMemAccess {
    fn open(_target: &DmalibAccessTarget) -> anyhow::Result<Self> {
        unimplemented!();
    }

    async fn get_baseaddr(&self, priority: i32) -> anyhow::Result<Option<u64>> {
        Ok(AccessType::mem_baseaddr()
            .with_priority(priority)
            .dispatch(&self.0)
            .await?
            .await?)
    }

    fn get_baseaddr_blocking(&self, priority: i32) -> anyhow::Result<Option<u64>> {
        Ok(AccessType::mem_baseaddr()
            .with_priority(priority)
            .blocking_dispatch(&self.0)?
            .blocking_recv()?)
    }

    async fn read_raw(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<bytes::Bytes> {
        AccessType::mem_read(addr, len, req_id)
            .with_priority(priority)
            .dispatch(&self.0)
            .await?
            .await?
    }

    fn read_raw_blocking(
        &self,
        addr: u64,
        len: usize,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<bytes::Bytes> {
        AccessType::mem_read(addr, len, req_id)
            .with_priority(priority)
            .blocking_dispatch(&self.0)?
            .blocking_recv()?
    }

    async fn read_raw_into(
        &self,
        addr: u64,
        out: &mut [u8],
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let ret = self.read_raw(addr, out.len(), priority, req_id).await?;
        out.copy_from_slice(&ret);
        Ok(())
    }

    fn read_raw_into_blocking(
        &self,
        addr: u64,
        out: &mut [u8],
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let ret = self.read_raw_blocking(addr, out.len(), priority, req_id)?;
        out.copy_from_slice(&ret);
        Ok(())
    }

    async fn read_raw_list(
        &self,
        list: &mut Vec<(u64, usize, Option<bytes::Bytes>)>,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let tasks: anyhow::Result<Vec<_>> = list
            .iter()
            .map(|(addr, len, _out)| {
                let (req, rx) = AccessType::mem_read(*addr, *len, req_id).with_priority(priority);
                self.0.send(req)?;
                anyhow::Ok(rx)
            })
            .collect();

        let tasks = tasks.inspect_err(|e| {
            tracing::warn!(
                ?e,
                priority,
                req_id,
                "{}",
                s!("batch read: failed to send requests")
            )
        })?;

        for (rx, (addr, len, out)) in tasks.into_iter().zip(list.iter_mut()) {
            match rx.await {
                Ok(result) => match result {
                    Ok(data) => {
                        *out = Some(data);
                    }
                    Err(e) => {
                        tracing::warn!(?e, addr, len, req_id, "{}", s!("batch read failed"));
                    }
                },
                Err(e) => {
                    tracing::warn!(
                        ?e,
                        addr,
                        len,
                        req_id,
                        "{}",
                        s!("batch read: failed to recv result")
                    );
                }
            }
        }

        Ok(())
    }

    fn read_raw_list_blocking(
        &self,
        list: &mut Vec<(u64, usize, Option<bytes::Bytes>)>,
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let tasks: anyhow::Result<Vec<_>> = list
            .iter()
            .map(|(addr, len, _out)| {
                AccessType::mem_read(*addr, *len, req_id)
                    .with_priority(priority)
                    .blocking_dispatch(&self.0)
            })
            .collect();

        let tasks = tasks.inspect_err(|e| {
            tracing::warn!(
                ?e,
                priority,
                req_id,
                "{}",
                s!("batch read: failed to send requests")
            )
        })?;

        for (rx, (addr, len, out)) in tasks.into_iter().zip(list.iter_mut()) {
            match rx.blocking_recv() {
                Ok(result) => match result {
                    Ok(data) => {
                        *out = Some(data);
                    }
                    Err(e) => {
                        tracing::warn!(?e, addr, len, req_id, "{}", s!("batch read failed"));
                    }
                },
                Err(e) => {
                    tracing::warn!(
                        ?e,
                        addr,
                        len,
                        req_id,
                        "{}",
                        s!("batch read: failed to recv result")
                    );
                }
            }
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
        let data = bytes::Bytes::copy_from_slice(data);
        AccessType::mem_write_bytes(addr, data, req_id)
            .with_priority(priority)
            .dispatch(&self.0)
            .await?
            .await?
    }

    fn write_raw_blocking(
        &self,
        addr: u64,
        data: &[u8],
        priority: i32,
        req_id: usize,
    ) -> anyhow::Result<()> {
        let data = bytes::Bytes::copy_from_slice(data);
        AccessType::mem_write_bytes(addr, data, req_id)
            .with_priority(priority)
            .blocking_dispatch(&self.0)?
            .blocking_recv()?
    }

    async fn find_sig(
        &self,
        sig: &str,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Option<FindSigResult>> {
        let sig = sig.parse::<MemSignature>()?;
        let ret = AccessType::mem_find_signature(sig, start..end)
            .with_priority(ohosky_api::common::dmalib::PRIO_LOW)
            .dispatch(&self.0)
            .await?
            .await??;
        let ret = ret.map(|(addr, data)| FindSigResult {
            address: addr.try_into().unwrap(),
            data: bytes::Bytes::from(data),
        });
        Ok(ret)
    }

    fn find_sig_blocking(
        &self,
        sig: &str,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Option<FindSigResult>> {
        let sig = sig.parse::<MemSignature>()?;
        let ret = AccessType::mem_find_signature(sig, start..end)
            .with_priority(ohosky_api::common::dmalib::PRIO_LOW)
            .blocking_dispatch(&self.0)?
            .blocking_recv()??;
        let ret = ret.map(|(addr, data)| FindSigResult {
            address: addr.try_into().unwrap(),
            data: bytes::Bytes::from(data),
        });
        Ok(ret)
    }

    async fn dump(&self) -> anyhow::Result<bytes::Bytes> {
        let ret = AccessType::mem_dump()
            .with_priority(ohosky_api::common::dmalib::PRIO_LOW)
            .dispatch(&self.0)
            .await?
            .await??;
        Ok(ret.into())
    }

    fn dump_blocking(&self) -> anyhow::Result<bytes::Bytes> {
        let ret = AccessType::mem_dump()
            .with_priority(ohosky_api::common::dmalib::PRIO_LOW)
            .blocking_dispatch(&self.0)?
            .blocking_recv()??;
        Ok(ret.into())
    }
}
