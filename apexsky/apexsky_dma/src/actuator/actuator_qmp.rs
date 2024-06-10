use qapi::futures::{QapiService, QmpStreamTokio};

use super::{delta_to_mouse_move, AimActuator};

pub struct QmpAimActuator {
    #[cfg(unix)]
    qmp: QapiService<QmpStreamTokio<tokio::io::WriteHalf<tokio::net::UnixStream>>>,
    #[cfg(not(unix))]
    qmp: QapiService<QmpStreamTokio<tokio::io::WriteHalf<tokio::net::TcpStream>>>,
    _handle: tokio::task::JoinHandle<()>,
}

impl QmpAimActuator {
    pub async fn connect(socket_addr: &str) -> anyhow::Result<Self> {
        #[cfg(unix)]
        let stream = QmpStreamTokio::open_uds(socket_addr).await?;
        #[cfg(not(unix))]
        let stream = QmpStreamTokio::open_tcp(socket_addr).await?;
        tracing::trace!(?stream.capabilities);
        let stream = stream.negotiate().await?;
        let (qmp, handle) = stream.spawn_tokio();

        Ok(Self {
            qmp,
            _handle: handle,
        })
    }

    async fn send_mouse_event(&mut self, events: Vec<qapi::qmp::InputEvent>) -> anyhow::Result<()> {
        self.qmp
            .execute(qapi::qmp::input_send_event {
                device: None,
                head: None,
                events,
            })
            .await?;
        Ok(())
    }
}

fn event_mouse_left(down: bool) -> Vec<qapi::qmp::InputEvent> {
    use qapi::qmp::{InputBtnEvent, InputBtnEventWrapper, InputButton, InputEvent};
    vec![InputEvent::btn(InputBtnEventWrapper {
        data: InputBtnEvent {
            button: InputButton::left,
            down,
        },
    })]
}

fn event_mouse_move(x: i16, y: i16) -> Vec<qapi::qmp::InputEvent> {
    use qapi::qmp::{InputAxis, InputEvent, InputMoveEvent, InputMoveEventWrapper};
    vec![
        InputEvent::rel(InputMoveEventWrapper {
            data: InputMoveEvent {
                axis: InputAxis::x,
                value: x.into(),
            },
        }),
        InputEvent::rel(InputMoveEventWrapper {
            data: InputMoveEvent {
                axis: InputAxis::y,
                value: y.into(),
            },
        }),
    ]
}

impl AimActuator for QmpAimActuator {
    async fn perform(&mut self, action: super::AimbotAction) -> anyhow::Result<()> {
        match (action.shift_angles, action.force_attack) {
            (None, None) => Ok(()),
            (None, Some(down)) => {
                self.send_mouse_event(event_mouse_left(down)).await?;
                Ok(())
            }
            (Some(delta), None) => {
                let update = delta_to_mouse_move(delta);
                self.send_mouse_event(event_mouse_move(update.0, update.1))
                    .await?;
                Ok(())
            }
            (Some(delta), Some(down)) => {
                let update = delta_to_mouse_move(delta);
                self.send_mouse_event(
                    [event_mouse_left(down), event_mouse_move(update.0, update.1)].concat(),
                )
                .await?;
                Ok(())
            }
        }
    }
}
