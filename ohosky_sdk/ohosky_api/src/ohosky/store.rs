use bytes::Bytes;

use crate::common::store::ISharedStore;

use super::api::bindings::ohosky::main::store as shared_storage;

pub struct SharedStoreApi;

impl ISharedStore for SharedStoreApi {
    #[inline]
    fn set(id: u64, data: Bytes) {
        tracing::debug!(?id, ?data);
        shared_storage::set(id, &data);
    }

    #[inline]
    fn get(id: u64) -> Option<Bytes> {
        shared_storage::get(id).map(Bytes::from)
    }

    #[inline]
    fn has(id: u64) -> bool {
        shared_storage::has(id)
    }

    #[inline]
    fn del(id: u64) -> bool {
        shared_storage::del(id)
    }

    #[inline]
    fn set_child(id: u64, child_id: u64, data: Bytes) {
        shared_storage::child_set(id, child_id, &data);
    }

    #[inline]
    fn get_child(id: u64, child_id: u64) -> Option<Bytes> {
        shared_storage::child_get(id, child_id).map(Bytes::from)
    }

    #[inline]
    fn has_child(id: u64, child_id: u64) -> bool {
        shared_storage::child_has(id, child_id)
    }

    #[inline]
    fn del_child(id: u64, child_id: u64) -> bool {
        shared_storage::child_del(id, child_id)
    }

    #[inline]
    fn insert_children(id: u64, entries: Vec<(u64, Bytes)>) {
        let entries = entries
            .into_iter()
            .map(|(id, data)| (id, Vec::from(data)))
            .collect::<Vec<_>>();
        shared_storage::insert_children(id, &entries)
    }

    #[inline]
    fn swap_children(id: u64, entries: Vec<(u64, Bytes)>) {
        let entries = entries
            .into_iter()
            .map(|(id, data)| (id, Vec::from(data)))
            .collect::<Vec<_>>();
        shared_storage::swap_children(id, &entries)
    }

    #[inline]
    fn count_children(id: u64) -> usize {
        shared_storage::count_children(id) as usize
    }

    #[inline]
    fn get_children(id: u64) -> Vec<(u64, Bytes)> {
        shared_storage::get_children(id)
            .into_iter()
            .map(|(id, data)| (id, Bytes::from(data)))
            .collect()
    }

    #[inline]
    fn list_children(id: u64) -> Vec<u64> {
        shared_storage::list_children(id)
    }

    #[inline]
    fn clear_children(id: u64) -> usize {
        shared_storage::clear_children(id) as usize
    }
}
