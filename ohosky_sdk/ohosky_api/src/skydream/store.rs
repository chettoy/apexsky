use bytes::Bytes;

use super::sky;
use crate::common::store::ISharedStore;

pub struct SharedStoreApi;

impl ISharedStore for SharedStoreApi {
    #[inline]
    fn set(id: u64, value: Bytes) {
        sky!(.store.set)(id, Vec::from(value).into())
    }

    #[inline]
    fn get(id: u64) -> Option<Bytes> {
        sky!(.store.get)(id)
            .into_rust()
            .map(|data| data.to_vec().into())
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
    fn set_child(id: u64, child_id: u64, value: Bytes) {
        sky!(.store.child_set)(id, child_id, Vec::from(value).into())
    }

    #[inline]
    fn get_child(id: u64, child_id: u64) -> Option<Bytes> {
        sky!(.store.child_get)(id, child_id)
            .into_rust()
            .map(|data| data.to_vec().into())
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
    fn insert_children(id: u64, entries: Vec<(u64, Bytes)>) {
        let entries = entries
            .into_iter()
            .map(|(_0, _1)| safer_ffi::Tuple2 {
                _0,
                _1: Vec::from(_1).into(),
            })
            .collect::<Vec<_>>();
        let entries = safer_ffi::slice::Ref::from(entries.as_slice());
        sky!(.store.insert_children)(id, entries)
    }

    #[inline]
    fn swap_children(id: u64, entries: Vec<(u64, Bytes)>) {
        let entries = entries
            .into_iter()
            .map(|(_0, _1)| safer_ffi::Tuple2 {
                _0,
                _1: Vec::from(_1).into(),
            })
            .collect::<Vec<_>>();
        let entries = safer_ffi::slice::Ref::from(entries.as_slice());
        sky!(.store.swap_children)(id, entries)
    }

    #[inline]
    fn count_children(id: u64) -> usize {
        sky!(.store.count_children)(id)
    }

    #[inline]
    fn get_children(id: u64) -> Vec<(u64, Bytes)> {
        let count = Self::count_children(id);
        let buf: safer_ffi::Vec<_> = Vec::with_capacity(count).into();
        let buf = sky!(.store.get_children)(id, buf);
        Vec::from(buf)
            .into_iter()
            .map(|safer_ffi::Tuple2 { _0, _1 }| (_0, _1.to_vec().into()))
            .collect::<Vec<_>>()
    }

    #[inline]
    fn list_children(id: u64) -> Vec<u64> {
        let count = Self::count_children(id);
        let buf: safer_ffi::Vec<u64> = Vec::with_capacity(count).into();
        let buf = sky!(.store.list_children)(id, buf);
        buf.into()
    }

    #[inline]
    fn clear_children(id: u64) -> usize {
        sky!(.store.clear_children)(id)
    }
}
