use std::marker::PhantomData;
use std::time::Duration;

use crate::common::msg::ISharedMessageChannel;
use crate::common::share::ISharableValue;

use super::api::bindings::ohosky::main::msg;
use super::ConvertWasmResultToAnyhow;

pub struct SharedMessageChannel<T> {
    id: u64,
    _value_type: PhantomData<T>,
}

impl<T: ISharableValue> ISharedMessageChannel<T> for SharedMessageChannel<T> {
    fn new(name: &str) -> Self {
        Self {
            id: msg::reg_msg(name),
            _value_type: PhantomData,
        }
    }

    async fn send(&self, value: T) -> anyhow::Result<()> {
        self.send_blocking(value)
    }

    fn send_blocking(&self, value: T) -> anyhow::Result<()> {
        msg::send_blocking(self.id, &value.into_raw()).to_anyhow()
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
        let data = msg::recv_blocking(self.id).to_anyhow()?;
        Ok(ISharableValue::from_raw(data))
    }

    fn try_recv(&self) -> anyhow::Result<Option<T>> {
        let data = msg::try_recv(self.id).to_anyhow()?;
        Ok(data.map(ISharableValue::from_raw))
    }
}
