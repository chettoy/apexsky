use super::*;
use std::str;

#[derive(Default, Debug, Clone)]
pub struct BaseEntity {
    pub entity_ptr: sdk::Ptr,
    pub index: u32,
    pub client_class_rva: u32,
    pub network_name: [u8; 32],
    pub origin: [f32; 3],
    // pub signifier_name: [u8; 32],
    pub model_name: ModelName,
}
impl BaseEntity {
    pub fn new(entity_ptr: sdk::Ptr, index: u32, client_class_rva: u32) -> Box<dyn Entity> {
        Box::new(BaseEntity {
            entity_ptr,
            index,
            client_class_rva,
            ..BaseEntity::default()
        }) as Box<dyn Entity>
    }
    // pub fn signifier_name(&self) -> &str {
    // 	from_utf8_buf(&self.signifier_name).unwrap_or("")
    // }
    pub fn network_name(&self) -> &str {
        crate::apexdream::base::from_utf8_buf(&self.network_name).unwrap_or("")
    }
}
#[async_trait]
impl Entity for BaseEntity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_ref(&self) -> EntityRef<'_> {
        EntityRef::BaseEntity(self)
    }
    fn is_serialized(&self) -> bool {
        false
    }
    fn get_info(&self) -> EntityInfo {
        EntityInfo {
            entity_ptr: self.entity_ptr,
            index: self.index as usize,
            handle: sdk::EHandle::from(self.index),
            rate: 128,
        }
    }
    #[instrument(skip_all)]
    async fn update(&mut self, api: &Api, ctx: &UpdateContext) {
        let data = &ctx.data;
        if self.network_name[0] == 0 {
            let client_class_ptr = api
                .apex_base
                .field::<sdk::ClientClass>(self.client_class_rva);
            if let Ok(client_class) = api.vm_read(client_class_ptr).await {
                let _ = api
                    .vm_read_into(client_class.pNetworkName, &mut self.network_name)
                    .await;
            }
        }
        let entity_ptr = self.entity_ptr;
        // let _ = process.vm_read_into(entity_ptr.field(data.entity_signifier_name + 9), &mut self.signifier_name).await;
        let _ = api
            .vm_read_into(entity_ptr.field(data.entity_origin), &mut self.origin)
            .await;
        if let Ok(model_name_ptr) = api
            .vm_read(entity_ptr.field::<sdk::Ptr<[u8]>>(data.entity_model_name))
            .await
        {
            self.model_name.update(api, model_name_ptr).await;
        }
    }
}
