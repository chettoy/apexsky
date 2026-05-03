use std::time::Duration;

use axum::Router;
use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use futures_util::FutureExt;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::{sync::watch, time::sleep};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::instrument;

use crate::global_state::G_STATE;
use crate::obfstr as s;

#[instrument(skip_all)]
pub async fn web_loop(mut active: watch::Receiver<bool>) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));

    let mut server_task: Option<(JoinHandle<Result<(), _>>, oneshot::Sender<()>)> = None;

    while *active.borrow_and_update() {
        sleep(Duration::from_secs(1)).await;
        let run_server = G_STATE.lock().unwrap().config.settings.esp_web_server;
        if !run_server {
            if let Some((task, shutdown_tx)) = server_task {
                // Stop server
                if !task.is_finished() {
                    match shutdown_tx.send(()) {
                        Ok(_) => {
                            tracing::info!("{}", s!("Stopping web server..."));
                        }
                        Err(_) => {
                            task.abort();
                            tracing::info!("{}", s!("Aborting web server..."));
                        }
                    }
                    match task.await {
                        Ok(_) => {
                            tracing::info!("{}", s!("Web server exit."));
                        }
                        Err(e) => {
                            if e.is_cancelled() {
                                tracing::info!("{}", s!("Web server cancelled."));
                            } else {
                                tracing::error!(%e, ?e, "{}", s!("error stop web server"));
                            }
                        }
                    }
                }
                server_task = None;
            }
            G_STATE.lock().unwrap().config.esp_service.web_serving = None;
        } else {
            #[allow(clippy::collapsible_else_if)]
            if let Some((task, _)) = &server_task {
                // Check task
                if task.is_finished()
                    && let Some((task, _)) = server_task.take()
                {
                    match task.await {
                        Ok(r) => {
                            if let Err(e) = r {
                                tracing::error!(%e, ?e);
                            } else {
                                tracing::warn!("{}", s!("web server finished"));
                            }
                        }
                        Err(e) => {
                            tracing::error!(%e, ?e);
                            if let Ok(reason) = e.try_into_panic() {
                                tracing::error!(?reason);
                            }
                        }
                    }
                }
            } else {
                // Start server
                let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

                let task = tokio::spawn(async move {
                    let app: Router = app();

                    let config = G_STATE.lock().unwrap().config.esp_service.clone();
                    let listener = tokio::net::TcpListener::bind(config.web_server_listen).await?;
                    let listen_addr = listener.local_addr()?;
                    tracing::info!("web server listening on {}", listen_addr);
                    G_STATE.lock().unwrap().config.esp_service.web_serving = Some(listen_addr);

                    axum::serve(listener, app)
                        .with_graceful_shutdown(shutdown_rx.map(drop))
                        .await
                });

                server_task = Some((task, shutdown_tx));
            }
        }
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}

fn app() -> Router {
    let serve_dir =
        ServeDir::new(s!("apexsky_overlay-web")).not_found_service(handle_404.into_service());
    Router::new()
        .route("/check", get(check))
        .fallback_service(serve_dir)
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new())
                .layer(TraceLayer::new_for_http())
                .layer(
                    // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
                    // requests don't hang forever.
                    TimeoutLayer::with_status_code(
                        StatusCode::REQUEST_TIMEOUT,
                        Duration::from_secs(10),
                    ),
                ),
        )
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

async fn check() -> Html<&'static str> {
    Html("OK")
}
