use std::{collections::HashMap, thread::sleep, time::Duration};

use ambisonic::{rodio::Source, AmbisonicBuilder, SoundController};
use tokio::sync::{mpsc, oneshot};

use crate::message::{AsSonicMessage, SonicMessage};

#[derive(Debug)]
pub enum SonicCtrl {
    LoadAudio((String, Vec<u8>)),
    Play(SonicMessage),
    _Shutdown,
}

pub fn sonic_thread(mut access_rx: mpsc::Receiver<SonicCtrl>) -> anyhow::Result<()> {
    tracing::debug!("task start");

    let sonic = AmbisonicBuilder::default().build();
    let mut src_list: HashMap<String, Box<[u8]>> = HashMap::new();
    let mut play_state: Vec<SoundController> = Vec::new();

    loop {
        match access_rx.try_recv() {
            Ok(req) => {
                if let SonicCtrl::_Shutdown = req {
                    break;
                }
                if let Err(e) = handler(req, &sonic, &mut src_list, &mut play_state) {
                    tracing::error!(%e, ?e);
                }
            }
            Err(e) => match e {
                mpsc::error::TryRecvError::Empty => {
                    sleep(Duration::from_millis(10));
                }
                mpsc::error::TryRecvError::Disconnected => {
                    tracing::error!(%e, ?e);
                    break;
                }
            },
        }
    }

    tracing::debug!("task end");
    Ok(())
}

fn handler(
    req: SonicCtrl,
    sonic: &ambisonic::Ambisonic,
    src_list: &mut HashMap<String, Box<[u8]>>,
    play_state: &mut Vec<SoundController>,
) -> anyhow::Result<()> {
    match req {
        SonicCtrl::LoadAudio((name, data)) => {
            src_list.insert(name, data.into_boxed_slice());
        }
        SonicCtrl::Play(msg) => play_state.push({
            let data = src_list.get(&msg.src_id()).unwrap().clone();
            let src = ambisonic::rodio::Decoder::new(std::io::Cursor::new(data))?;
            sonic.play_at(src.convert_samples(), msg.position())
        }),
        SonicCtrl::_Shutdown => (),
    }
    Ok(())
}
