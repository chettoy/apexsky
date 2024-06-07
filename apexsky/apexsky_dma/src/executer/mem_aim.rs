use std::mem::size_of;

use apexsky::{aimbot::normalize_angles, offsets::G_OFFSETS};
use obfstr::obfstr as s;

use crate::{
    apexdream::base::math,
    workers::access::{AccessType, MemApi, PendingAccessRequest, PendingMemRead},
};

use super::{AimExecuter, AimbotAction};

#[derive(Debug)]
pub struct MemAimHelper {
    pub mem: MemApi,
    pub apex_base: u64,
    pub lplayer_ptr: u64,
}

#[derive(Debug)]
pub struct MemAimExecuter<'a> {
    father: &'a MemAimHelper,
    view_angles: [f32; 3],
    updated_viewangles: Option<[f32; 3]>,
}

impl MemAimHelper {
    pub async fn self_update(&mut self) {
        let Some((apex_base, lplayer_ptr)) = Self::read_ptr(&self.mem).await else {
            self.apex_base = 0;
            self.lplayer_ptr = 0;
            return;
        };
        self.apex_base = apex_base;
        self.lplayer_ptr = lplayer_ptr;
    }

    pub fn ready(&self) -> bool {
        self.apex_base != 0 && self.lplayer_ptr != 0
    }

    pub fn get_executer(&self, view_angles: [f32; 3]) -> MemAimExecuter {
        MemAimExecuter {
            father: self,
            view_angles,
            updated_viewangles: None,
        }
    }

    async fn read_ptr(mem: &MemApi) -> Option<(u64, u64)> {
        let apex_base = AccessType::mem_baseaddr()
            .with_priority(1)
            .dispatch(&mem)
            .await
            .ok()?
            .await
            .ok()??;
        let lplayer_ptr =
            AccessType::mem_read(apex_base + G_OFFSETS.local_ent, size_of::<u64>(), 0)
                .with_priority(1)
                .dispatch(&mem)
                .await
                .ok()?
                .recv_for::<u64>()
                .await
                .ok()?;
        Some((apex_base, lplayer_ptr))
    }

    pub async fn read_viewangles(mem: &MemApi, ptr: u64) -> anyhow::Result<[f32; 3]> {
        AccessType::mem_read(ptr + G_OFFSETS.player_viewangles, size_of::<[f32; 3]>(), 0)
            .with_priority(50)
            .dispatch(mem)
            .await?
            .recv_for::<[f32; 3]>()
            .await
    }

    pub async fn write_viewangles(mem: &MemApi, ptr: u64, data: &[f32; 3]) -> anyhow::Result<()> {
        AccessType::mem_write_typed::<[f32; 3]>(ptr + G_OFFSETS.player_viewangles, data, 0)
            .with_priority(50)
            .dispatch(mem)
            .await?
            .await?
    }

    pub async fn write_attack_button(
        mem: &MemApi,
        apex_base: u64,
        force_attack_state: i32,
    ) -> anyhow::Result<()> {
        AccessType::mem_write_typed::<i32>(
            apex_base + G_OFFSETS.in_attack + 0x8,
            &force_attack_state,
            0,
        )
        .with_priority(50)
        .dispatch(mem)
        .await?
        .await?
    }
}

impl MemAimExecuter<'_> {
    pub fn get_updated_viewangles(&self) -> Option<[f32; 3]> {
        self.updated_viewangles
    }
}

impl AimExecuter for MemAimExecuter<'_> {
    async fn perform(&mut self, action: AimbotAction) -> anyhow::Result<()> {
        if let Some(delta) = action.shift_angles {
            // calc and check target view angles
            let mut update_angles = math::add(self.view_angles, delta);
            if update_angles[0].abs() > 360.0
                || update_angles[1].abs() > 360.0
                || update_angles[2].abs() > 360.0
            {
                tracing::warn!(?update_angles, "{}", s!("got invalid target view_angles"));
                anyhow::bail!("{}", s!("got invalid target view_angles"))
            }
            normalize_angles(&mut update_angles);

            // write target view angles
            if let Err(e) = MemAimHelper::write_viewangles(
                &self.father.mem,
                self.father.lplayer_ptr,
                &update_angles,
            )
            .await
            {
                tracing::warn!(%e, "{}", s!("err write viewangles"));
                anyhow::bail!("{}", s!("err write viewangles"));
            }
            self.updated_viewangles = Some(update_angles);
        }

        if let Some(trigger) = action.force_attack {
            let force_attack = if trigger { 5 } else { 4 };
            if let Err(e) = MemAimHelper::write_attack_button(
                &self.father.mem,
                self.father.apex_base,
                force_attack,
            )
            .await
            {
                tracing::warn!(%e, "{}", s!("err write force_attack"));
                anyhow::bail!("{}", s!("err write force_attack"));
            }
        }

        Ok(())
    }
}
