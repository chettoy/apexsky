use std::marker::PhantomData;

use super::sky;
use crate::common::msg::{ISharedMessageChannel, ISharedValueWatcher, ISharedWatchValue};
use crate::common::share::ISharableValue;

pub fn reg_msg(name: &str) -> u64 {
    sky!(.msg.reg_msg)(name.into())
}

pub async fn send(id: u64, value: Vec<u8>) -> anyhow::Result<()> {
    if sky!(.msg.send_async)(id, value.into()).await {
        Ok(())
    } else {
        anyhow::bail!("fail");
    }
}

pub fn send_blocking(id: u64, value: Vec<u8>) -> anyhow::Result<()> {
    if sky!(.msg.send_blocking)(id, value.into()) {
        Ok(())
    } else {
        anyhow::bail!("fail");
    }
}

pub async fn recv(id: u64) -> anyhow::Result<Vec<u8>> {
    let ret = sky!(.msg.recv_async)(id).await;
    match ret.into_rust() {
        Some(data) => Ok(data.to_vec()),
        None => anyhow::bail!("fail"),
    }
}

pub fn recv_blocking(id: u64) -> anyhow::Result<Vec<u8>> {
    let ret = sky!(.msg.recv_blocking)(id);
    match ret.into_rust() {
        Some(data) => Ok(data.to_vec()),
        None => anyhow::bail!("fail"),
    }
}

pub fn try_recv(id: u64) -> anyhow::Result<Option<Vec<u8>>> {
    let ret = sky!(.msg.try_recv)(id);
    match ret.into_rust() {
        Some(result) => Ok(result.into_rust().map(|data| data.to_vec())),
        None => anyhow::bail!("fail"),
    }
}

pub fn subscribe(name: &str, watcher_unique: &str) -> (u64, u64) {
    let safer_ffi::Tuple2 { _0, _1 } = sky!(.msg.subscribe)(name.into(), watcher_unique.into());
    (_0, _1)
}

pub fn unsubscribe(watcher_id: u64) -> bool {
    sky!(.msg.unsubscribe)(watcher_id)
}

pub fn update_value(sender_id: u64, value: Vec<u8>) -> bool {
    sky!(.msg.update_value)(sender_id, value.into())
}

pub fn fetch_value(watcher_id: u64) -> Vec<u8> {
    sky!(.msg.fetch_value)(watcher_id).to_vec()
}

pub async fn next_value_async(watcher_id: u64) -> Vec<u8> {
    sky!(.msg.next_value_async)(watcher_id).await.to_vec()
}

pub fn try_next_value(watcher_id: u64) -> Option<Vec<u8>> {
    sky!(.msg.try_next_value)(watcher_id)
        .into_rust()
        .map(|data| data.to_vec())
}

#[derive(Debug, Clone)]
pub struct SharedMessageChannel<T: ISharableValue> {
    id: u64,
    _value_type: PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct SharedWatchValue<T: ISharableValue> {
    id: u64,
    _value_type: PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct SharedValueWatcher<T: ISharableValue> {
    id: u64,
    _value_type: PhantomData<T>,
}

impl<T: ISharableValue> ISharedMessageChannel<T> for SharedMessageChannel<T> {
    fn new(name: &str) -> Self {
        Self {
            id: reg_msg(name),
            _value_type: PhantomData,
        }
    }

    async fn send(&self, value: T) -> anyhow::Result<()> {
        send(self.id, value.into_raw()).await
    }

    fn send_blocking(&self, value: T) -> anyhow::Result<()> {
        send_blocking(self.id, value.into_raw())
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
        Ok(data.map(ISharableValue::from_raw))
    }
}

impl<T: ISharableValue> ISharedWatchValue<T> for SharedWatchValue<T> {
    fn new(name: &str, watcher_unique: &str) -> (Self, impl ISharedValueWatcher<T>) {
        let (name_id, watcher_id) = subscribe(name, watcher_unique);
        (
            Self {
                id: name_id,
                _value_type: PhantomData,
            },
            SharedValueWatcher {
                id: watcher_id,
                _value_type: PhantomData,
            },
        )
    }

    fn update(&self, value: T) -> anyhow::Result<()> {
        if !update_value(self.id, value.into_raw()) {
            anyhow::bail!("fail");
        }
        Ok(())
    }
}

impl<T: ISharableValue> Drop for SharedValueWatcher<T> {
    fn drop(&mut self) {
        unsubscribe(self.id);
    }
}

impl<T: ISharableValue> ISharedValueWatcher<T> for SharedValueWatcher<T> {
    fn fetch(&self) -> T {
        let data = fetch_value(self.id);
        ISharableValue::from_raw(data)
    }

    async fn next_value(&self) -> T {
        let data = next_value_async(self.id).await;
        ISharableValue::from_raw(data)
    }

    fn try_next_value(&self) -> Option<T> {
        let data = try_next_value(self.id);
        data.map(ISharableValue::from_raw)
    }
}
