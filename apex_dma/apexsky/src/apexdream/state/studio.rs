use super::*;

#[derive(Default, Debug, Clone)]
pub struct StudioModel {
	pub ptr: sdk::Ptr<sdk::CStudioHdr>,
	pub studiohdr_ptr: sdk::Ptr<sdk::studiohdr_t>,
	pub studiohdr: sdk::studiohdr_t,

	pub bones: Vec<sdk::mstudiobone_t>,

	pub hitboxset: sdk::mstudiohitboxset_t,

	pub hitboxes: Vec<sdk::mstudiobbox_t>,

	pub hb_lookup: Vec<i32>,

	pub bone_start: i32,
	pub bone_end1: i32,
	pub bone_end2: i32,

	pub bone_head: i32,
	pub bone_body: i32,
}

impl StudioModel {
	pub fn update(&mut self, api: &mut Api, ptr: sdk::Ptr<sdk::CStudioHdr>) -> bool {
		self.ptr = ptr;

		if self.ptr.is_null() {
			self.hitboxes.clear();
			return false;
		}
		// Sometimes this pointer is garbage...
		// Figure out why, this may cause triggerbot to fail!
		if self.ptr.into_raw() % 8 != 0 {
			return false;
		}

		let Ok(cstudio) = api.vm_read(self.ptr) else { return false };
		if self.studiohdr_ptr == cstudio.m_pStudioHdr {
			return true;
		}
		self.studiohdr_ptr = cstudio.m_pStudioHdr;
		let Ok(()) = api.vm_read_into(self.studiohdr_ptr, &mut self.studiohdr) else { return false };

		self.bone_head = -1;
		self.bone_body = -1;

		// Read bones
		let numbones = self.studiohdr.numbones as usize;
		if numbones > 256 {
			return false;
		}
		if self.bones.len() != numbones {
			self.bones.resize_with(numbones, Default::default);
			self.hb_lookup.clear();
			self.hb_lookup.resize(numbones, -1);
		}
		let Ok(()) = api.vm_read_into(self.studiohdr_ptr.field(self.studiohdr.boneoffset()), &mut self.bones[..]) else { return false };

		// Read first hitboxset
		// if self.studiohdr.numhitboxsets == 0 {
		// 	return false;
		// }
		let Ok(()) = api.vm_read_into(self.studiohdr_ptr.field(self.studiohdr.hitboxsetoffset()), &mut self.hitboxset) else { return false };

		// Read hitboxes
		let numhitboxes = self.hitboxset.numhitboxes as usize;
		if numhitboxes > 512 {
			self.hitboxes.clear();
			return false;
		}
		if self.hitboxes.len() != numhitboxes {
			self.hitboxes.resize_with(numhitboxes, Default::default);
		}
		if self.hitboxset.numhitboxes > 0 {
			let Ok(()) = api.vm_read_into(self.studiohdr_ptr.field(self.studiohdr.hitboxsetoffset() + self.hitboxset.hitboxoffset()), &mut self.hitboxes[..]) else { return false };
		}

		// Process hitboxes:
		// * Find the head hitbox bone
		// * Create lookup table bone -> hitbox
		let mut bone_end2 = 0;
		for (i, hb) in self.hitboxes.iter().enumerate() {
			bone_end2 = i32::max(bone_end2, hb.bone as i32 + 1);
			if self.bone_head == -1 {
				if hb.group == sdk::HITGROUP_HEAD {
					self.bone_head = hb.bone as i32;
				}
			}
			if let Some(lookup) = self.hb_lookup.get_mut(hb.bone as usize) {
				*lookup = i as i32;
			}
		}
		self.bone_end2 = bone_end2;

		// Find the range of bones needed for the spine for optimization
		let mut bone_start = i32::MAX;
		let mut bone_end1 = 0;
		let mut bone_body = 0;
		for bbox in self.spine() {
			bone_start = i32::min(bone_start, bbox.bone as i32);
			bone_end1 = i32::max(bone_end1, bbox.bone as i32 + 1);
			bone_body = bbox.bone as i32;
		}
		self.bone_start = bone_start;
		self.bone_end1 = bone_end1;
		self.bone_body = bone_body;

		return true;
	}
	/// Given a hitbox returns its parent hitbox.
	pub fn parent_hitbox(&self, bbox: &sdk::mstudiobbox_t) -> Option<&sdk::mstudiobbox_t> {
		let mut bone = bbox.bone;
		let mut count = 0;
		loop {
			count += 1;
			if count >= self.bones.len() {
				return None;
			}
			let parent = self.bones.get(bone as usize)?.parent as u16;
			// if bone == parent {
			// 	return None;
			// }
			let parent_idx = parent as i32 - 1;
			if parent_idx <= 0 {
				return None;
			}
			if let Some(bbox) = self.hb_lookup.get(parent_idx as usize).and_then(|&index| self.hitboxes.get(index as usize)) {
				return Some(bbox);
			}
			bone = parent;
		}
	}
	/// Starting from the head hitbox, iterate over parent bones returning the hitbox until the origin.
	pub fn spine<'a>(&'a self) -> impl 'a + Clone + Iterator<Item = &'a sdk::mstudiobbox_t> {
		self.hitboxes.iter().take_while(|hb| matches!(hb.group, sdk::HITGROUP_GENERIC | sdk::HITGROUP_HEAD | sdk::HITGROUP_UPPER_BODY | sdk::HITGROUP_LOWER_BODY))
	}
}
