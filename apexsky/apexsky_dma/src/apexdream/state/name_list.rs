use super::*;

const SIZE: usize = sdk::MAX_PLAYERS * 2;

pub struct NameList {
    pointers1: Box<[sdk::Ptr<[u8]>]>,
    pointers2: Box<[sdk::Ptr<[u8]>]>,
    names: Box<[String]>,
}
impl Default for NameList {
    fn default() -> NameList {
        NameList {
            pointers1: vec![sdk::Ptr::new(); SIZE].into_boxed_slice(),
            pointers2: vec![sdk::Ptr::new(); SIZE].into_boxed_slice(),
            names: vec![String::new(); SIZE].into_boxed_slice(),
        }
    }
}
impl NameList {
    #[instrument(skip_all)]
    pub async fn update(&mut self, api: &Api, ctx: &UpdateContext) {
        let base_addr = api.apex_base;
        let data = &ctx.data;

        if !ctx.ticked(25, 19) {
            return;
        }

        // Read the name list and check for new names
        let _ = api
            .vm_read_into(base_addr.field(data.name_list), &mut *self.pointers2)
            .await;
        std::mem::swap(&mut self.pointers1, &mut self.pointers2);

        let pointers1 = unsafe { self.pointers1.get_unchecked(..SIZE) };
        let pointers2 = unsafe { self.pointers2.get_unchecked(..SIZE) };
        let names = unsafe { self.names.get_unchecked_mut(..SIZE) };

        let mut futs_read = vec![];
        for i in 0..SIZE {
            if pointers1[i] != pointers2[i] {
                names[i].clear();
                futs_read.push((
                    i,
                    tokio::spawn({
                        let api = api.clone();
                        let s_ptr = pointers1[i];
                        async move {
                            let mut name_buf = [0u8; 128];
                            api.vm_read_cstr(s_ptr, &mut name_buf)
                                .await
                                .map(|s| s.to_string())
                        }
                    }),
                ));
            }
        }
        for (i, fut_read) in futs_read {
            if let Ok(name) = fut_read.await.unwrap() {
                names[i] = name;
            }
        }
    }
}
impl GameState {
    pub fn get_player_name(&self, handle: sdk::EHandle) -> Option<&str> {
        let index = handle.index()?.wrapping_sub(1).wrapping_mul(3);
        let name = self.name_list.names.get(index)?;
        Some(name.as_str())
    }
    pub fn get_name1(&self, index: usize) -> &str {
        match self.name_list.names.get(index * 2) {
            Some(name) => &**name,
            None => "",
        }
    }
    pub fn get_name2(&self, index: usize) -> &str {
        match self.name_list.names.get(index * 2 + 1) {
            Some(name) => &**name,
            None => "",
        }
    }
}
