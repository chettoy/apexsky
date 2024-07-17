use std::{collections::HashMap, sync::Arc};

use obfstr::obfstr as s;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

use crate::{ExtensionMessage, ExtensionRuntime, GameApi, UserMod};

struct NewUserModTask {
    usermod: UserMod,
    create_result_tx: oneshot::Sender<CreateTaskResult>,
    join_handle_tx: oneshot::Sender<JoinHandle<anyhow::Result<()>>>,
}

pub struct CreateTaskResult {
    pub return_value: Option<serde_json::Value>,
    pub msg_tx: async_channel::Sender<ExtensionMessage>,
}

pub struct TaskContext {
    pub return_value: Option<serde_json::Value>,
    pub msg_tx: async_channel::Sender<ExtensionMessage>,
    pub join_handle: JoinHandle<anyhow::Result<()>>,
}

pub struct RunningManager {
    run_tx: mpsc::UnboundedSender<NewUserModTask>,
    running: HashMap<String, TaskContext>,
}

impl RunningManager {
    pub fn new(game_api: Arc<dyn GameApi>) -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let (run_tx, mut run_rx) = mpsc::unbounded_channel::<NewUserModTask>();

        tokio::task::spawn_blocking(move || {
            let local = tokio::task::LocalSet::new();
            local.spawn_local(async move {
                while let Some(new_task) = run_rx.recv().await {
                    let handle = tokio::task::spawn_local(usermod_thread(
                        new_task.usermod,
                        new_task.create_result_tx,
                        Arc::clone(&game_api),
                    ));
                    let _ = new_task.join_handle_tx.send(handle);
                }
            });
            rt.block_on(local);
        });

        Self {
            run_tx,
            running: HashMap::new(),
        }
    }

    #[tracing::instrument(skip_all, fields(package_name = installed.package_name))]
    pub async fn start(&mut self, installed: UserMod) -> anyhow::Result<()> {
        // Read package info
        let package_name = installed.package_name.to_owned();

        // Create task
        let (create_result_tx, create_result_rx) = oneshot::channel();
        let (join_handle_tx, join_handle_rx) = oneshot::channel();
        self.run_tx.send(NewUserModTask {
            usermod: installed,
            create_result_tx,
            join_handle_tx,
        })?;
        let create_result = create_result_rx.await?;
        let join_handle = join_handle_rx.await?;
        tracing::info!(
            "{}{}{}{:?}",
            s!("<package:"),
            package_name,
            s!("> created: "),
            create_result.return_value
        );

        // Save context
        self.running.insert(
            package_name,
            TaskContext {
                return_value: create_result.return_value,
                msg_tx: create_result.msg_tx,
                join_handle,
            },
        );

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub async fn stop(&mut self, package_name: &str) -> anyhow::Result<()> {
        let Some(ctx) = self.running.remove(package_name) else {
            return Ok(());
        };

        if ctx.join_handle.is_finished() {
            return Ok(());
        }

        ctx.join_handle.abort();

        Ok(())
    }

    pub fn get_running<'a>(&'a self) -> &HashMap<String, TaskContext> {
        &self.running
    }
}

#[tracing::instrument(skip_all, fields(package_name = usermod.package_name))]
pub(crate) async fn usermod_thread(
    usermod: UserMod,
    create_result_tx: oneshot::Sender<CreateTaskResult>,
    game_api: Arc<dyn GameApi>,
) -> anyhow::Result<()> {
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
