use super::share::ISharableValue;

pub trait ISharedStore {
    fn set(id: u64, value: bytes::Bytes);
    fn get(id: u64) -> Option<bytes::Bytes>;
    fn has(id: u64) -> bool;
    fn del(id: u64) -> bool;
    fn set_child(id: u64, child_id: u64, value: bytes::Bytes);
    fn get_child(id: u64, child_id: u64) -> Option<bytes::Bytes>;
    fn has_child(id: u64, child_id: u64) -> bool;
    fn del_child(id: u64, child_id: u64) -> bool;
    fn insert_children(id: u64, entries: Vec<(u64, bytes::Bytes)>);
    fn swap_children(id: u64, entries: Vec<(u64, bytes::Bytes)>);
    fn count_children(id: u64) -> usize;
    fn get_children(id: u64) -> Vec<(u64, bytes::Bytes)>;
    fn list_children(id: u64) -> Vec<u64>;
    fn clear_children(id: u64) -> usize;
}

pub trait ISharedValueStore: ISharedStore {
    fn set<T: ISharableValue>(id: u64, value: T) {
        <Self as ISharedStore>::set(id, value.into_bytes());
    }
    fn get<T: ISharableValue>(id: u64) -> Option<T> {
        <Self as ISharedStore>::get(id).map(ISharableValue::from_bytes)
    }
    fn has(id: u64) -> bool {
        <Self as ISharedStore>::has(id)
    }
    fn del(id: u64) -> bool {
        <Self as ISharedStore>::del(id)
    }
    fn set_child<T: ISharableValue>(id: u64, child_id: u64, value: T) {
        <Self as ISharedStore>::set_child(id, child_id, value.into_bytes())
    }
    fn get_child<T: ISharableValue>(id: u64, child_id: u64) -> Option<T> {
        <Self as ISharedStore>::get_child(id, child_id).map(ISharableValue::from_bytes)
    }
    fn has_child(id: u64, child_id: u64) -> bool {
        <Self as ISharedStore>::has_child(id, child_id)
    }
    fn del_child(id: u64, child_id: u64) -> bool {
        <Self as ISharedStore>::del_child(id, child_id)
    }
    fn insert_children<T: ISharableValue>(id: u64, entries: Vec<(u64, T)>) {
        let entries = entries
            .into_iter()
            .map(|(k, v)| (k, v.into_bytes()))
            .collect();
        <Self as ISharedStore>::insert_children(id, entries)
    }
    fn swap_children<T: ISharableValue>(id: u64, entries: Vec<(u64, T)>) {
        let entries = entries
            .into_iter()
            .map(|(k, v)| (k, v.into_bytes()))
            .collect();
        <Self as ISharedStore>::swap_children(id, entries)
    }
    fn get_children<T: ISharableValue>(id: u64) -> Vec<(u64, T)> {
        <Self as ISharedStore>::get_children(id)
            .into_iter()
            .map(|(k, v)| (k, ISharableValue::from_bytes(v)))
            .collect()
    }
    fn list_children(id: u64) -> Vec<u64> {
        <Self as ISharedStore>::list_children(id)
    }
    fn clear_children(id: u64) -> usize {
        <Self as ISharedStore>::clear_children(id)
    }
}
