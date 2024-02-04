use std::{sync::Arc, time::Duration};

use apexsky::kbd_backlight_blink;
use obfstr::obfstr as s;
use tokio::{
    sync::{watch, Mutex},
    time::sleep,
};
use tracing::instrument;

use crate::SharedState;

#[instrument(skip_all)]
pub async fn control_loop(
    mut active: watch::Receiver<bool>,
    shared_state: Arc<Mutex<SharedState>>,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));
    while *active.borrow_and_update() {
        sleep(Duration::from_millis(100)).await;
        let spectators = { shared_state.lock().await.spectator_count };
        if spectators > 0 {
            kbd_backlight_blink(spectators.try_into().unwrap());
            sleep(Duration::from_secs(10) - Duration::from_millis(100)).await;
        }
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}
