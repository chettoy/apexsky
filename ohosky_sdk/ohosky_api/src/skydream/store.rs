use super::sky;
use crate::common::store::ISharedStore;

pub struct SharedStoreApi;

impl ISharedStore for SharedStoreApi {
    #[inline]
    fn set(id: u64, value: Vec<u8>) {
        sky!(.store.set)(id, value.into())
    }

    #[inline]
    fn get(id: u64) -> Option<Vec<u8>> {
        sky!(.store.get)(id).into_rust().map(Into::into)
    }

    #[inline]
    fn has(id: u64) -> bool {
        sky!(.store.has)(id)
    }

    #[inline]
    fn del(id: u64) -> bool {
        sky!(.store.del)(id)
    }

    #[inline]
    fn set_child(id: u64, child_id: u64, value: Vec<u8>) {
        sky!(.store.child_set)(id, child_id, value.into())
    }

    #[inline]
    fn get_child(id: u64, child_id: u64) -> Option<Vec<u8>> {
        sky!(.store.child_get)(id, child_id)
            .into_rust()
            .map(Into::into)
    }

    #[inline]
    fn has_child(id: u64, child_id: u64) -> bool {
        sky!(.store.child_has)(id, child_id)
    }

    #[inline]
    fn del_child(id: u64, child_id: u64) -> bool {
        sky!(.store.child_del)(id, child_id)
    }

    #[inline]
    fn insert_children(id: u64, entries: Vec<(u64, Vec<u8>)>) {
        let entries = entries
            .into_iter()
            .map(|(_0, _1)| safer_ffi::Tuple2 { _0, _1: _1.into() })
            .collect::<Vec<_>>()
            .into();
        sky!(.store.insert_children)(id, entries)
    }

    #[inline]
    fn swap_children(id: u64, entries: Vec<(u64, Vec<u8>)>) {
        let entries = entries
            .into_iter()
            .map(|(_0, _1)| safer_ffi::Tuple2 { _0, _1: _1.into() })
            .collect::<Vec<_>>()
            .into();
        sky!(.store.swap_children)(id, entries)
    }

    #[inline]
    fn get_children(id: u64) -> Vec<(u64, Vec<u8>)> {
        Into::<Vec<_>>::into(sky!(.store.get_children)(id))
            .into_iter()
            .map(|safer_ffi::Tuple2 { _0, _1 }| (_0, _1.into()))
            .collect::<Vec<_>>()
    }

    #[inline]
    fn list_children(id: u64) -> Vec<u64> {
        sky!(.store.list_children)(id).into()
    }

    #[inline]
    fn clear_children(id: u64) -> usize {
        sky!(.store.clear_children)(id)
    }
}
