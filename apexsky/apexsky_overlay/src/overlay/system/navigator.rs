use std::collections::HashMap;
use std::mem;

use bevy::prelude::*;
use obfstr::obfstr as s;

use super::game_esp::EspSystem;
use super::sound::{SoundSrcHandle, SoundSystem};
use crate::navigator::{SonicMessage, VoiceNavigator};
use crate::overlay::model::MyOverlayState;

#[derive(Resource, Default)]
pub struct NavigatorSystem {
    source_map: HashMap<String, SoundSrcHandle>,
    instance: VoiceNavigator,
    messages: Vec<SonicMessage>,
}

pub fn setup_voice_navigator(
    sound_system: Res<SoundSystem>,
    mut navigator_system: ResMut<NavigatorSystem>,
) {
    use fyrox_sound::buffer::DataSource;
    use fyrox_sound::buffer::SoundBufferResource;
    use fyrox_sound::buffer::SoundBufferResourceExtension;
    use fyrox_sound::source::SoundSourceBuilder;

    for (voice_id, voice_data) in VoiceNavigator::voice_resource() {
        if let Some(handle) = navigator_system.source_map.get(voice_id) {
            if sound_system.context.state().is_valid_handle(*handle) {
                continue;
            }
        }
        let sound_buf_res =
            SoundBufferResource::new_generic(DataSource::from_memory(voice_data)).unwrap();
        let sound_source = SoundSourceBuilder::new()
            .with_name(voice_id)
            .with_buffer(sound_buf_res)
            .with_status(fyrox_sound::source::Status::Stopped)
            .with_play_once(false)
            .build()
            .unwrap();
        let source_handle = sound_system.context.state().add_source(sound_source);
        navigator_system
            .source_map
            .insert(voice_id.to_owned(), source_handle);
    }
}

pub fn update_voice_navigator(
    overlay_state: Res<MyOverlayState>,
    sound_system: Res<SoundSystem>,
    esp_system: Option<Res<EspSystem>>,
    mut navigator_system: ResMut<NavigatorSystem>,
) {
    use crate::navigator::message::AsSonicMessage;
    use fyrox_sound::algebra::UnitQuaternion;

    if let Some(ref esp_system) = esp_system {
        let mut new_msg = navigator_system.instance.tick(&esp_system.esp_data);
        navigator_system.messages.append(&mut new_msg);
    }

    if !overlay_state.user_gesture {
        return;
    }

    let messages = mem::replace(&mut navigator_system.messages, vec![]);

    if messages.is_empty() {
        return;
    }

    let yaw = esp_system
        .as_ref()
        .and_then(|v| v.esp_data.view_player.as_ref())
        .map(|pl| pl.view_angles.as_ref().map(|v| v.y).unwrap_or(pl.yaw))
        .unwrap_or(0.0);

    for msg in messages.into_iter() {
        let voice_msg = match msg {
            SonicMessage::Voice(inner) => inner,
        };

        let Some(&source_handle) = navigator_system.source_map.get(&voice_msg.src_id()) else {
            tracing::warn!(?voice_msg, "{}", s!("Voice resource do not exists"));
            continue;
        };

        sound_system
            .context
            .state()
            .source_mut(source_handle)
            .set_position(voice_msg.position().into())
            .play();
    }
}
