use super::*;

const REQUEST_ID_READ_ITEMS: usize = 0; //obfstr::random!(usize);

const SIZE: usize = sdk::MAX_PLAYERS * 2;

pub struct NameList {
    pointers1: Box<[sdk::Ptr<[u8]>]>,
    pointers2: Box<[sdk::Ptr<[u8]>]>,
    names: Box<[String]>,
    retry_rate: Box<[u8]>,
}
impl Default for NameList {
    fn default() -> NameList {
        NameList {
            pointers1: vec![sdk::Ptr::new(); SIZE].into_boxed_slice(),
            pointers2: vec![sdk::Ptr::new(); SIZE].into_boxed_slice(),
            names: vec![String::new(); SIZE].into_boxed_slice(),
            retry_rate: vec![1; SIZE].into_boxed_slice(),
        }
    }
}
impl NameList {
    #[instrument(skip_all)]
    pub async fn update(&mut self, api: &Api, ctx: &UpdateContext) {
        let base_addr = api.apex_base;
        let data = &ctx.data;

        const TICK_RATE: u32 = 25;
        const TICK_OFFSET: u32 = 19;
        const MAX_RETRY_RATE: u8 = 32;

        if !ctx.ticked(TICK_RATE, TICK_OFFSET) {
            return;
        }

        // Read the name list and check for new names
        let Ok(()) = api
            .vm_read_into(base_addr.field(data.name_list), &mut *self.pointers2)
            .await
        else {
            return;
        };
        std::mem::swap(&mut self.pointers1, &mut self.pointers2);

        let pointers1 = unsafe { self.pointers1.get_unchecked(..SIZE) };
        let pointers2 = unsafe { self.pointers2.get_unchecked(..SIZE) };
        let names = unsafe { self.names.get_unchecked_mut(..SIZE) };
        let retry_rate = unsafe { self.retry_rate.get_unchecked_mut(..SIZE) };

        let mut futs_read = Vec::with_capacity(SIZE);
        for i in 0..SIZE {
            if pointers1[i] != pointers2[i]
                || (names[i].is_empty()
                    && ctx.ticked(TICK_RATE * retry_rate[i] as u32, TICK_OFFSET))
            {
                names[i].clear();
                let s_ptr = pointers1[i];
                if s_ptr.is_null() {
                    continue;
                }
                futs_read.push((
                    i,
                    tokio::spawn({
                        let api = api.clone_new(REQUEST_ID_READ_ITEMS);
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
            match fut_read.await.unwrap() {
                Ok(name) => {
                    names[i] = name;
                    retry_rate[i] = 1;
                }
                Err(e) => {
                    tracing::debug!(?e);
                    names[i].clear();
                    if retry_rate[i] > MAX_RETRY_RATE {
                        retry_rate[i] = MAX_RETRY_RATE;
                    } else {
                        retry_rate[i] *= 2;
                    }
                }
            }
        }
    }
}

impl GameState {
    pub fn get_player_name(&self, handle: sdk::EHandle) -> Option<&str> {
        let index = handle
            .index()?
            .wrapping_sub(1)
            .wrapping_mul(3)
            .wrapping_add(2);
        let name = self.name_list.names.get(index)?;
        Some(name.as_str())
    }
    pub fn get_name1(&self, index: usize) -> Option<&str> {
        let index = index.wrapping_sub(1).wrapping_mul(3).wrapping_add(1);
        let name = self.name_list.names.get(index)?;
        Some(name.as_str())
    }
    pub fn get_name2(&self, index: usize) -> Option<&str> {
        let index = index.wrapping_sub(1).wrapping_mul(3).wrapping_add(2);
        let name = self.name_list.names.get(index)?;
        Some(name.as_str())
    }
}
