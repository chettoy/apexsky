use std::time::Duration;

use apexsky::kbd_backlight_blink;
use obfstr::obfstr as s;
use tokio::{sync::watch, time::sleep};
use tracing::instrument;

use crate::SharedStateWrapper;

#[instrument(skip_all)]
pub async fn control_loop(
    mut active: watch::Receiver<bool>,
    shared_state: SharedStateWrapper,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));
    while *active.borrow_and_update() {
        sleep(Duration::from_millis(100)).await;
        let spectator_count = { shared_state.spectator_list.lock().1.len() };
        if spectator_count > 0 {
            kbd_backlight_blink(spectator_count.try_into().unwrap());
            sleep(Duration::from_secs(10) - Duration::from_millis(100)).await;
        }
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}
