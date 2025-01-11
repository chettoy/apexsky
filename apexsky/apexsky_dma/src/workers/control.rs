use std::time::Duration;

use obfstr::obfstr as s;
use tokio::{sync::watch, time::sleep};
use tracing::instrument;

use crate::global_state::G_CONTEXT;
use crate::lock_config;
use crate::SharedStateType;

#[instrument(skip_all)]
pub async fn control_loop(
    mut active: watch::Receiver<bool>,
    shared_state: SharedStateType,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));
    while *active.borrow_and_update() {
        sleep(Duration::from_millis(100)).await;
        let spectator_count = { shared_state.spectator_list.lock().1.len() };
        let spectator_count: i32 = spectator_count.try_into()?;
        if spectator_count > 0 {
            tokio::task::spawn_blocking(move || kbd_backlight_blink(spectator_count)).await?;
            sleep(Duration::from_secs(10) - Duration::from_millis(100)).await;
        }
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}

pub fn kbd_backlight_blink(count: i32) -> bool {
    if !(1..=10).contains(&count)
        || !lock_config!()
            .settings
            .feature_settings
            .kbd_backlight_control
    {
        return false;
    }
    (|| -> anyhow::Result<()> {
        G_CONTEXT.lock().unwrap().kbd_blink(count.try_into()?)?;
        Ok(())
    })()
    .is_ok()
}
