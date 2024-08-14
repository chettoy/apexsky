use apexsky::offsets::G_OFFSETS;

use self::sdk::EHandle;

use super::*;

#[derive(Debug, Default, Clone, sdk::Pod)]
#[repr(C)]
pub struct CObserverMode {
    pub observer_mode: i32,
    pub observer_target: EHandle,
}
const _: [(); 0x8] = [(); std::mem::size_of::<CObserverMode>()];

const SIZE: usize = sdk::MAX_PLAYERS;

pub struct ObserverList {
    observer_modes: Box<[CObserverMode]>,
}
impl Default for ObserverList {
    fn default() -> Self {
        Self {
            observer_modes: vec![CObserverMode::default(); SIZE].into_boxed_slice(),
        }
    }
}
impl ObserverList {
    #[instrument(skip_all)]
    pub async fn update(&mut self, api: &Api, ctx: &UpdateContext) {
        let base_addr = api.apex_base;

        if !ctx.ticked(25, 19) {
            return;
        }

        let mut ptr: intptr::IntPtr = sdk::Ptr::NULL;
        let _ = api
            .vm_read_into(
                base_addr.field(G_OFFSETS.spectator_list.try_into().unwrap()),
                &mut ptr,
            )
            .await;
        if ptr.is_null() {
            return;
        }
        let _ = api
            .vm_read_into(
                ptr.field::<[CObserverMode]>(G_OFFSETS.global_observer_mode.try_into().unwrap()),
                &mut *self.observer_modes,
            )
            .await;
    }
}
impl GameState {
    pub fn get_observer_target(&self, handle: sdk::EHandle) -> Option<&PlayerEntity> {
        let index = handle.index()?.wrapping_sub(1);
        let target_handle = self.observer_list.observer_modes[index].observer_target;
        self.entity_as::<PlayerEntity>(target_handle)
    }
}
