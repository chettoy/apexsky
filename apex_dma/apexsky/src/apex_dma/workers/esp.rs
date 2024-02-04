use std::{sync::Arc, time::Duration};

use obfstr::obfstr as s;
use tokio::{
    sync::{watch, Mutex},
    time::sleep,
};
use tracing::instrument;

use crate::SharedState;

#[instrument(skip_all)]
pub async fn esp_loop(
    mut active: watch::Receiver<bool>,
    shared_state: Arc<Mutex<SharedState>>,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));
    while *active.borrow_and_update() {
        sleep(Duration::from_millis(15)).await;

        let (spec, spec_all) = {
            let state = shared_state.lock().await;
            (state.spectator_count, state.allied_spectator_count)
        };
        //tracing::trace!(spec, spec_all);
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}
