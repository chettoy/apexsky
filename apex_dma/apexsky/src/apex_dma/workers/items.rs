use std::{sync::Arc, time::Duration};

use obfstr::obfstr as s;
use tokio::{
    sync::{watch, Mutex},
    time::sleep,
};
use tracing::instrument;

use crate::SharedState;

#[instrument]
pub async fn items_loop(
    mut active: watch::Receiver<bool>,
    shared_state: Arc<Mutex<SharedState>>,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));
    while *active.borrow_and_update() {
        sleep(Duration::from_millis(15)).await;
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}
