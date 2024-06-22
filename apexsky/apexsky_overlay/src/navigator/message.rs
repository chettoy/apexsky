use rand::Rng;

use super::resource::ContentId;

pub trait AsSonicMessage {
    fn cut_in(&self) -> bool;
    fn is_bgm(&self) -> bool;
    fn src_id(&self) -> String;
    fn position(&self) -> [f32; 3];
    fn velocity(&self) -> [f32; 3];
}

#[derive(Debug, Clone)]
pub enum SonicMessage {
    Voice(VoicePrompt),
}

impl AsSonicMessage for SonicMessage {
    fn cut_in(&self) -> bool {
        match self {
            SonicMessage::Voice(inner) => inner.cut_in(),
        }
    }

    fn is_bgm(&self) -> bool {
        match self {
            SonicMessage::Voice(inner) => inner.is_bgm(),
        }
    }

    fn src_id(&self) -> String {
        match self {
            SonicMessage::Voice(inner) => inner.src_id(),
        }
    }

    fn position(&self) -> [f32; 3] {
        match self {
            SonicMessage::Voice(inner) => inner.position(),
        }
    }

    fn velocity(&self) -> [f32; 3] {
        match self {
            SonicMessage::Voice(inner) => inner.velocity(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VoicePrompt {
    src_id: ContentId,
    alter_id: usize,
    position: [f32; 3],
}

impl VoicePrompt {
    pub fn new(src_id: ContentId, position: [f32; 3]) -> Self {
        let alter_id = match src_id {
            ContentId::Connected => rand::thread_rng().gen_range(1..=3),
            ContentId::DualTeamsNearby => 1,
            ContentId::EnemyInTheRear => rand::thread_rng().gen_range(1..=5),
            ContentId::UnderObservation => rand::thread_rng().gen_range(1..=4),
        };
        Self {
            src_id,
            alter_id,
            position,
        }
    }
}

impl AsSonicMessage for VoicePrompt {
    fn cut_in(&self) -> bool {
        false
    }

    fn is_bgm(&self) -> bool {
        false
    }

    fn src_id(&self) -> String {
        format!("default-{}-{}", self.src_id as i32, self.alter_id)
    }

    fn position(&self) -> [f32; 3] {
        self.position
    }

    fn velocity(&self) -> [f32; 3] {
        [0.0, 0.0, 0.0]
    }
}
