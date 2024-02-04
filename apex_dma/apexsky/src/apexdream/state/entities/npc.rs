use crate::offsets::G_OFFSETS;

use super::*;

#[derive(Default, Debug, Clone)]
pub struct BaseNPCEntity {
    pub entity_ptr: sdk::Ptr,
    pub entity_size: u32,
    pub index: u32,

    pub origin: [f32; 3],
    pub angles: [f32; 3],
    pub velocity: [f32; 3],
    pub accel: [f32; 3],
    pub derivative_origin: EstimateDerivative,
    pub derivative_velocity: EstimateDerivative,

    pub health: i32,
    pub max_health: i32,
    pub shields: i32,
    pub max_shields: i32,
    pub health_history: Vec<ValueChanged<i32>>,

    pub team_num: i32,
    pub team_color: [u8; 3],

    pub model_name: ModelName,
    pub studio: StudioModel,
    pub bones: super::BoneArray,

    pub skin: i32,
    pub skin_mod: i32,
    pub body: i32,
    pub camo_index: i32,

    pub flags: u32,
    pub life_state: u8,
    pub last_visible_time: f32,
    pub tmp_last_lastviz: f32,
    pub is_visible: bool,
    pub visible_time: f64,
}
impl BaseNPCEntity {
    pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
        let entity_size = cc.ClassSize;
        Box::new(BaseNPCEntity {
            entity_ptr,
            entity_size,
            index,
            ..BaseNPCEntity::default()
        }) as Box<dyn Entity>
    }
    pub fn is_alive(&self) -> bool {
        self.life_state == 0
    }
    pub fn is_onground(&self) -> bool {
        self.flags & 0x1 != 0
    }
    pub fn is_ducking(&self) -> bool {
        self.flags & 0x2 != 0
    }
    pub fn height(&self) -> f32 {
        if self.is_ducking() {
            36.0
        } else {
            60.0
        }
    }
    pub fn get_bone_pos(&self, bone: usize) -> [f32; 3] {
        sdk::add(self.origin, self.bones.get_pos(bone))
    }
}
impl Entity for BaseNPCEntity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_ref(&self) -> EntityRef<'_> {
        EntityRef::BaseNPC(self)
    }
    fn is_serialized(&self) -> bool {
        false
    }
    fn get_info(&self) -> EntityInfo {
        EntityInfo {
            entity_ptr: self.entity_ptr,
            index: self.index as usize,
            handle: sdk::EHandle::from(self.index),
            rate: 1,
        }
    }
    #[instrument(skip_all)]
    fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
        #[derive(sdk::Pod)]
        #[repr(C)]
        struct Indices {
            origin: [u32; 6],
            health: [u32; 4],
            team: [u32; 1],
            model_name: [u32; 2],
            bone_array: [u32; 2],
            studio: [u32; 2],
            skin: [u32; 4],
            state: [u32; 3],
        }

        let data = &ctx.data;
        let mut indices = Indices {
            origin: [
                data.entity_origin + 0,
                data.entity_origin + 4,
                data.entity_origin + 8,
                data.entity_origin + 24,
                data.entity_origin + 28,
                data.entity_origin + 32,
            ],
            health: [
                data.entity_health,
                data.entity_max_health,
                data.entity_shield_health,
                data.entity_shield_health + 4,
            ],
            team: [data.entity_team_num],
            model_name: [data.entity_model_name + 0, data.entity_model_name + 4],
            bone_array: [data.animating_bone_array + 0, data.animating_bone_array + 4],
            studio: [data.animating_studiohdr + 0, data.animating_studiohdr + 4],
            skin: [
                data.animating_skin + 0,
                data.animating_skin + 4,
                data.animating_skin + 8,
                data.animating_skin + 12,
            ],
            state: [
                data.entity_flags,
                data.entity_life_state,
                //data.bcc_last_visible_time,
                G_OFFSETS.player_last_visible_time.try_into().unwrap(),
            ],
        };

        if let Ok(fields) = api.vm_gatherd(self.entity_ptr, self.entity_size, &mut indices) {
            self.origin = [
                f32::from_bits(fields.origin[0]),
                f32::from_bits(fields.origin[1]),
                f32::from_bits(fields.origin[2]),
            ];
            self.angles = [
                f32::from_bits(fields.origin[3]),
                f32::from_bits(fields.origin[4]),
                f32::from_bits(fields.origin[5]),
            ];
            let estvel = self.derivative_origin.update(ctx.time, self.origin, 0.1);
            self.velocity = estvel;

            self.accel = self
                .derivative_velocity
                .update(ctx.time, self.velocity, 0.1);

            let health = fields.health[0] as i32;
            let shields = fields.health[2] as i32;

            if health > 1000 || shields > 1000 || health < 0 || shields < 0 {
                tracing::debug!(health, shields);
                self.health = 0;
                self.shields = 0;
            } else {
                if health + shields != self.health + self.shields {
                    self.health_history.push(ValueChanged::new(
                        ctx.time,
                        self.health + self.shields,
                        health + shields,
                    ));
                }
                self.health = health;
                self.shields = shields;
            }

            self.max_health = fields.health[1] as i32;
            self.max_shields = fields.health[3] as i32;

            self.team_num = fields.team[0] as i32;

            let model_name_ptr = fields.model_name[0] as u64 | (fields.model_name[1] as u64) << 32;
            self.model_name.update(api, model_name_ptr.into());
            let studio_ptr = fields.studio[0] as u64 | (fields.studio[1] as u64) << 32;
            self.studio.update(api, sdk::Ptr::from_raw(studio_ptr));
            let bones_ptr = fields.bone_array[0] as u64 | (fields.bone_array[1] as u64) << 32;
            self.bones
                .update(api, ctx, &self.studio, sdk::Ptr::from_raw(bones_ptr));

            self.skin = fields.skin[0] as i32;
            self.skin_mod = fields.skin[1] as i32;
            self.body = fields.skin[2] as i32;
            self.camo_index = fields.skin[3] as i32;

            self.flags = fields.state[0];
            self.life_state = fields.state[1] as u8;
            self.last_visible_time = f32::from_bits(fields.state[2]);
        }
    }
    #[instrument(skip_all)]
    fn post(&mut self, _api: &mut Api, ctx: &UpdateContext, state: &GameState) {
        // Check if npc is visible
        // let is_visible = self.last_visible_time > 0.0
        //     && (self.last_visible_time - state.client.curtime).abs() < f32::EPSILON;
        let is_visible = self.last_visible_time > self.tmp_last_lastviz
            || (self.last_visible_time < 0.0 && self.tmp_last_lastviz > 0.0);
        //tracing::trace!(is_visible, self.last_visible_time, state.client.curtime);
        // Take note when the npc became visible
        if !self.is_visible && is_visible {
            self.visible_time = ctx.time;
        }
        self.is_visible = is_visible;
        self.tmp_last_lastviz = self.last_visible_time;
    }
}
