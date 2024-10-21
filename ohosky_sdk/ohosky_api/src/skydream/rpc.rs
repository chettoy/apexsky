use std::marker::PhantomData;

use super::sky;
use crate::common::{
    rpc::{ISharedRpcClient, ISharedRpcService},
    share::ISharableValue,
};

pub fn reg_rpc(name: String) -> u64 {
    sky!(.rpc.reg_rpc)(name.into())
}

pub fn use_rpc(name: String) -> u64 {
    sky!(.rpc.use_rpc)(name.into())
}

pub async fn recv(id: u64) -> anyhow::Result<Vec<u8>> {
    sky!(.rpc.recv_async)(id)
        .await
        .into_anyhow()
        .map(Into::into)
}

pub fn recv_blocking(id: u64) -> anyhow::Result<Vec<u8>> {
    sky!(.rpc.recv_blocking)(id).into_anyhow().map(Into::into)
}

pub fn try_recv(id: u64) -> anyhow::Result<Option<Vec<u8>>> {
    sky!(.rpc.try_recv)(id)
        .into_anyhow()
        .map(|ret| ret.into_rust().map(Into::into))
}

pub fn reply(id: u64, ret_value: Vec<u8>) -> anyhow::Result<()> {
    if sky!(.rpc.reply)(id, ret_value.into()) {
        Ok(())
    } else {
        anyhow::bail!("fail");
    }
}

pub fn is_online(id: u64) -> anyhow::Result<bool> {
    sky!(.rpc.is_online)(id).into_anyhow()
}

pub async fn wait_online(id: u64) -> anyhow::Result<bool> {
    sky!(.rpc.wait_online_async)(id).await.into_anyhow()
}

pub async fn call(id: u64, arg_value: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    sky!(.rpc.call_async)(id, arg_value.into())
        .await
        .into_anyhow()
        .map(Into::into)
}

pub fn call_blocking(id: u64, arg_value: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    sky!(.rpc.call_blocking)(id, arg_value.into())
        .into_anyhow()
        .map(Into::into)
}

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
            id: reg_rpc(name.to_string()),
            _arg_type: PhantomData,
            _ret_type: PhantomData,
        }
    }

    async fn recv(&self) -> anyhow::Result<T> {
        let data = recv(self.id).await?;
        Ok(ISharableValue::from_raw(data))
    }

    fn recv_blocking(&self) -> anyhow::Result<T> {
        let data = recv_blocking(self.id)?;
        Ok(ISharableValue::from_raw(data))
    }

    fn try_recv(&self) -> anyhow::Result<Option<T>> {
        let data = try_recv(self.id)?;
        Ok(data.map(|data| ISharableValue::from_raw(data)))
    }

    fn reply(&self, ret: R) -> anyhow::Result<()> {
        reply(self.id, ret.into_raw())
    }
}

impl<T: ISharableValue, R: ISharableValue> ISharedRpcClient<T, R> for SharedRpcClient<T, R> {
    fn new(name: &str) -> Self {
        Self {
            id: use_rpc(name.to_string()),
            _arg_type: PhantomData,
            _ret_type: PhantomData,
        }
    }

    fn is_online(&self) -> anyhow::Result<bool> {
        is_online(self.id)
    }

    async fn wait_online(&self) -> anyhow::Result<()> {
        while !wait_online(self.id).await? {}
        Ok(())
    }

    async fn call(&self, arg: T) -> anyhow::Result<R> {
        let ok_ret = call(self.id, arg.into_raw()).await?;
        Ok(ISharableValue::from_raw(ok_ret))
    }

    fn call_blocking(&self, arg: T) -> anyhow::Result<R> {
        let ok_ret = call_blocking(self.id, arg.into_raw())?;
        Ok(ISharableValue::from_raw(ok_ret))
    }
}
