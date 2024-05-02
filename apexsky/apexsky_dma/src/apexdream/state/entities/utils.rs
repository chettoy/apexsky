use super::*;

#[derive(Default, Debug, Clone)]
pub struct BoneArray {
    pub v: Vec<[f32; 12]>,
}
impl BoneArray {
    pub async fn update(
        &mut self,
        api: &Api,
        ctx: &UpdateContext,
        studio: &StudioModel,
        ptr: sdk::Ptr<[[f32; 12]]>,
    ) {
        if ptr.is_null() {
            self.v.clear();
            return;
        }
        let numbones = if ctx.full_bones {
            studio.bone_end2
        } else {
            studio.bone_end1
        } as usize;
        if self.v.len() != numbones {
            self.v.resize_with(numbones, Default::default);
        }
        let bones = &mut self.v[..];
        let _ = api.vm_read_into(ptr, bones).await;
    }
    pub fn get_pos(&self, bone: usize) -> [f32; 3] {
        if let Some(matrix) = self.v.get(bone) {
            [matrix[3], matrix[7], matrix[11]]
        } else {
            //tracing::debug!(bone, ?self.v, "{}", s!("711aac39-e83c-4788 invalid bone"));
            [0.0; 3]
        }
    }
}

pub struct PathHistory {
    pub time: f64,
    pub origin: [f32; 3],
    pub velocity: [f32; 3],
}
