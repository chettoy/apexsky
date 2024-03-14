use std::{sync::Arc, time::Duration};

use obfstr::obfstr as s;
use parking_lot::RwLock;
use tokio::{sync::watch, time::sleep};
use tracing::instrument;

use crate::SharedState;

#[instrument(skip_all)]
pub async fn esp_loop(
    mut active: watch::Receiver<bool>,
    shared_state: Arc<RwLock<SharedState>>,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));
    while *active.borrow_and_update() {
        sleep(Duration::from_millis(15)).await;

        let (spec, spec_all) = {
            let state = shared_state.read();
            (
                state.spectator_name.len(),
                state.allied_spectator_name.len(),
            )
        };
        //tracing::trace!(spec, spec_all);
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}
