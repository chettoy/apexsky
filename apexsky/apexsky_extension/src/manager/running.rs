use std::{collections::HashMap, sync::Arc};

use obfstr::obfstr as s;
use tokio::sync::{mpsc, oneshot};

use crate::{ExtensionMessage, ExtensionRuntime, GameApi, UserMod};

struct NewUserModTask {
    usermod: UserMod,
    create_result_tx: oneshot::Sender<CreateTaskResult>,
}

pub struct CreateTaskResult {
    pub return_value: Option<serde_json::Value>,
    pub msg_tx: async_channel::Sender<ExtensionMessage>,
}

pub struct RunningManager {
    run_tx: mpsc::UnboundedSender<NewUserModTask>,
    running: HashMap<String, CreateTaskResult>,
}

impl RunningManager {
    pub fn new(game_api: Arc<dyn GameApi>) -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let (run_tx, mut run_rx) = mpsc::unbounded_channel();

        tokio::task::spawn_blocking(move || {
            let local = tokio::task::LocalSet::new();
            local.spawn_local(async move {
                while let Some(new_task) = run_rx.recv().await {
                    tokio::task::spawn_local(usermod_thread(new_task, Arc::clone(&game_api)));
                }
            });
            rt.block_on(local);
        });

        Self {
            run_tx,
            running: HashMap::new(),
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn start(&mut self, installed: UserMod) -> anyhow::Result<()> {
        // Read package info
        let package_name = installed.package_name.to_owned();

        // Create task
        let (tx, rx) = oneshot::channel();
        self.run_tx.send(NewUserModTask {
            usermod: installed,
            create_result_tx: tx,
        })?;
        let create_result = rx.await?;
        tracing::info!(
            "{}{}{}{:?}",
            s!("<package:"),
            package_name,
            s!("> created: "),
            create_result.return_value
        );

        // Save context
        self.running.insert(package_name, create_result);

        Ok(())
    }

    pub fn get_running<'a>(&'a self) -> &HashMap<String, CreateTaskResult> {
        &self.running
    }
}

#[tracing::instrument(skip_all, fields(package_name = new_task.usermod.package_name))]
pub(crate) async fn usermod_thread(
    new_task: NewUserModTask,
    game_api: Arc<dyn GameApi>,
) -> anyhow::Result<()> {
    let NewUserModTask {
        usermod,
        create_result_tx,
    } = new_task;

    let (msg_tx, msg_rx) = async_channel::bounded(1024);

    // Create instance
    let mut usermod_runtime = ExtensionRuntime::new(
        usermod.manifest,
        usermod.source,
        (msg_tx.clone(), msg_rx),
        Some(game_api),
    )?;

    // Execute code
    match usermod_runtime.execute().await {
        Ok(r) => {
            let _ = create_result_tx.send(CreateTaskResult {
                return_value: r,
                msg_tx,
            });
        }
        Err(e) => {
            tracing::error!(%e, ?e);
            return Err(e.into());
        }
    }

    // Run main loop
    match usermod_runtime.run_loop().await {
        Ok(_) => {
            tracing::info!(%usermod.package_name, "{}", s!("exit"));
        }
        Err(e) => {
            tracing::error!(%e, ?e);
        }
    }

    tracing::warn!(
        "{}{}{}",
        s!("usermod_thr<"),
        usermod.package_name,
        s!("> finished")
    );

    Ok(())
}
