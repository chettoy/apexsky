use super::*;

#[derive(Default, Debug, Clone)]
pub struct WaypointEntity {
    pub entity_ptr: sdk::Ptr,
    pub entity_size: u32,
    pub index: u32,
    pub origin: [f32; 3],
    pub network_flags: u32,
    pub visibility_flags: u32,
    pub team_num: i32,
    pub team_member_index: i32,
    pub owner_entity: sdk::EHandle,
    // type = 9 bloodhound trail
    // type = 7 player ping
    pub waypoint_type: i32,
    pub waypoint_bitfield: u32,
}
impl WaypointEntity {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
        let entity_size = cc.ClassSize;
        Box::new(WaypointEntity {
            entity_ptr,
            entity_size,
            index,
            ..WaypointEntity::default()
        })
    }
}
#[async_trait]
impl Entity for WaypointEntity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_ref(&self) -> EntityRef<'_> {
        EntityRef::Waypoint(self)
    }
    fn is_serialized(&self) -> bool {
        true
    }
    fn get_info(&self) -> EntityInfo {
        EntityInfo {
            entity_ptr: self.entity_ptr,
            index: self.index as usize,
            handle: sdk::EHandle::from(self.index),
            rate: 64,
        }
    }
    #[instrument(skip_all)]
    async fn update(&mut self, api: &Api, ctx: &UpdateContext) {
        #[derive(sdk::Pod)]
        #[repr(C)]
        struct Indices {
            origin: [u32; 3],
            network_flags: u32,
            visibility_flags: u32,
            team_num: [u32; 2],
            owner_entity: u32,
            waypoint_type: [u32; 2],
        }

        let data = &ctx.data;
        let mut indices = Indices {
            origin: [
                data.entity_origin,
                data.entity_origin + 4,
                data.entity_origin + 8,
            ],
            network_flags: 0x02d8, //0x03d4,
            visibility_flags: data.entity_team_num - 8,
            team_num: [data.entity_team_num, data.entity_team_num + 4],
            owner_entity: data.entity_owner_entity,
            waypoint_type: [data.waypoint_type, data.waypoint_type + 4],
        };

        if let Ok(fields) = api
            .vm_gatherd(self.entity_ptr, self.entity_size, true, &mut indices)
            .await
        {
            self.origin[0] = f32::from_bits(fields.origin[0]);
            self.origin[1] = f32::from_bits(fields.origin[1]);
            self.origin[2] = f32::from_bits(fields.origin[2]);
            self.network_flags = fields.network_flags;
            self.visibility_flags = fields.visibility_flags;
            self.team_num = fields.team_num[0] as i32;
            self.team_member_index = fields.team_num[1] as i32;
            self.owner_entity = sdk::EHandle::from(fields.owner_entity);
            self.waypoint_type = fields.waypoint_type[0] as i32;
            self.waypoint_bitfield = fields.waypoint_type[1];
        }
    }
}
