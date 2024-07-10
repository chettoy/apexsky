use std::sync::Arc;

use apexsky::extension::{ExtensionMessage, UserModManager};
use obfstr::obfstr as s;
use tokio::{
    sync::{mpsc, oneshot},
    time::Instant,
};

use crate::api_impl::GameApiHandle;

pub(crate) enum UserModEvent {
    GameAttached,
    GameUnattached,
    ActionTick,
}

pub(crate) async fn usermod_thread(
    game_api: GameApiHandle,
    mut event_rx: mpsc::UnboundedReceiver<UserModEvent>,
) -> anyhow::Result<()> {
    let mut mgr = UserModManager::default();

    let package_name = match mgr.install(s!("./auto_sg.spk").into()).await {
        Ok(name) => name,
        Err(e) => {
            tracing::warn!(%e, "{}", s!("Userscript not installed"));
            return Ok(());
        }
    };

    if let Err(e) = mgr.load(&package_name, Arc::new(game_api)) {
        tracing::error!(%e, ?e, "{}", s!("Failed to init usermod"));
        return Err(e);
    };

    let Some(usermod) = mgr.get_instance_mut(&package_name) else {
        tracing::warn!("{}", s!("Failed to get instance of usermod"));
        return Ok(());
    };

    match usermod.execute().await {
        Ok(r) => {
            println!("{}{}{}{:?}", s!("<"), package_name, s!("> created: "), r);
        }
        Err(e) => {
            tracing::error!(%e, ?e);
            return Err(e.into());
        }
    }

    let msg_tx = usermod.get_msg_tx();

    tokio::spawn(async move {
        loop {
            let Some(event) = event_rx.recv().await else {
                break;
            };
            let mut callback_rx = None;
            let msg = match event {
                UserModEvent::GameAttached => {
                    ExtensionMessage::new(String::from("game_attached"), None)
                }
                UserModEvent::GameUnattached => {
                    ExtensionMessage::new(String::from("game_unattached"), None)
                }
                UserModEvent::ActionTick => {
                    let (tx, rx) = oneshot::channel();
                    callback_rx = Some(rx);
                    ExtensionMessage::new(String::from("action_tick"), Some(tx))
                }
            };
            msg_tx.send(msg).await?;
            if let Some(rx) = callback_rx {
                let wait_start = Instant::now();
                let result = rx.await?;
                let wait_time = wait_start.elapsed();
                tracing::trace!("wait={}ms, return={:?}", wait_time.as_millis(), result);
            }
        }
        anyhow::Ok(())
    });

    if let Err(e) = usermod.run_loop().await {
        tracing::error!(%e, ?e);
    }

    tracing::warn!("{}", s!("usermod_thr finished"));

    Ok(())
}
