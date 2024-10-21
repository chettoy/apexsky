use std::{marker::PhantomData, time::Duration};

use super::{api::bindings::ohosky::main::rpc, ConvertWasmResultToAnyhow};
use crate::common::{
    rpc::{ISharedRpcClient, ISharedRpcService},
    share::ISharableValue,
};

#[derive(Debug, Clone)]
pub struct SharedRpcService<T: ISharableValue, R: ISharableValue> {
    id: u64,
    _arg_type: PhantomData<T>,
    _ret_type: PhantomData<R>,
}

#[derive(Debug, Clone)]
pub struct SharedRpcClient<T: ISharableValue, R: ISharableValue> {
    id: u64,
    _arg_type: PhantomData<T>,
    _ret_type: PhantomData<R>,
}

impl<T: ISharableValue, R: ISharableValue> ISharedRpcService<T, R> for SharedRpcService<T, R> {
    fn register(name: &str) -> Self {
        Self {
            id: rpc::reg_rpc(name),
            _arg_type: PhantomData,
            _ret_type: PhantomData,
        }
    }

    async fn recv(&self) -> anyhow::Result<T> {
        loop {
            if let Some(data) = self.try_recv()? {
                break Ok(data);
            }
            tokio::time::sleep(Duration::from_micros(200)).await;
        }
    }

    fn recv_blocking(&self) -> anyhow::Result<T> {
        let data = rpc::recv_blocking(self.id).to_anyhow()?;
        Ok(ISharableValue::from_raw(data))
    }

    fn try_recv(&self) -> anyhow::Result<Option<T>> {
        let data = rpc::try_recv(self.id).to_anyhow()?;
        Ok(data.map(ISharableValue::from_raw))
    }

    fn reply(&self, ret: R) -> anyhow::Result<()> {
        rpc::reply(self.id, &ret.into_raw()).to_anyhow()
    }
}

impl<T: ISharableValue, R: ISharableValue> ISharedRpcClient<T, R> for SharedRpcClient<T, R> {
    fn new(name: &str) -> Self {
        Self {
            id: rpc::use_rpc(name),
            _arg_type: PhantomData,
            _ret_type: PhantomData,
        }
    }

    fn is_online(&self) -> anyhow::Result<bool> {
        rpc::is_online(self.id).to_anyhow()
    }

    async fn wait_online(&self) -> anyhow::Result<()> {
        while !self.is_online()? {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok(())
    }

    async fn call(&self, _arg: T) -> anyhow::Result<R> {
        anyhow::bail!("not implemented");
    }

    fn call_blocking(&self, arg: T) -> anyhow::Result<R> {
        let ok_ret = rpc::call_blocking(self.id, &arg.into_raw()).to_anyhow()?;
        Ok(ISharableValue::from_raw(ok_ret))
    }
}
