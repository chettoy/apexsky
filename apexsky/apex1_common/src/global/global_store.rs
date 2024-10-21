use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

pub use ohosky_api::common::store::ISharedStore;
pub trait StoreBackend: ISharedStore {}

#[cfg(all(feature = "ohosky", not(feature = "skydream")))]
use ohosky_api::ohosky::SharedStoreApi as GlobalStore;
#[cfg(feature = "skydream")]
use ohosky_api::skydream::SharedStoreApi as GlobalStore;

#[cfg(any(feature = "ohosky", feature = "skydream"))]
impl StoreBackend for GlobalStore {}

#[cfg(any(feature = "ohosky", feature = "skydream"))]
impl<K, T> Store<GlobalStore, K, T> for K
where
    K: Record<T>,
    T: StoreValue,
{
}

pub trait StoreValue: Sized {
    fn into_store_data(val: Self) -> anyhow::Result<Vec<u8>>;
    fn from_store_data(bin: Vec<u8>) -> anyhow::Result<Self>;
}

pub trait Record<T: StoreValue> {
    fn name() -> String;

    fn name_id() -> u64 {
        let mut hasher = DefaultHasher::new();
        Self::name().hash(&mut hasher);
        hasher.finish()
    }

    #[inline]
    fn set<S: StoreBackend>(val: T) -> anyhow::Result<()> {
        S::set(
            Self::name_id(),
            T::into_store_data(val).inspect_err(|e| tracing::error!(?e))?,
        );
        Ok(())
    }

    #[inline]
    fn get<S: StoreBackend>() -> anyhow::Result<Option<T>> {
        match S::get(Self::name_id()) {
            Some(bin) => T::from_store_data(bin)
                .inspect_err(|e| tracing::error!(?e))
                .map(Some),
            None => Ok(None),
        }
    }

    #[inline]
    fn has<S: StoreBackend>() -> bool {
        S::has(Self::name_id())
    }

    #[inline]
    fn del<S: StoreBackend>() -> bool {
        S::del(Self::name_id())
    }

    #[inline]
    fn set_child<S: StoreBackend>(sub_idx: u64, val: T) -> anyhow::Result<()> {
        S::set_child(
            Self::name_id(),
            sub_idx,
            T::into_store_data(val).inspect_err(|e| tracing::error!(?e))?,
        );
        Ok(())
    }

    #[inline]
    fn get_child<S: StoreBackend>(sub_idx: u64) -> anyhow::Result<Option<T>> {
        match S::get_child(Self::name_id(), sub_idx) {
            Some(bin) => T::from_store_data(bin)
                .inspect_err(|e| tracing::error!(?e))
                .map(Some),
            None => Ok(None),
        }
    }

    #[inline]
    fn has_child<S: StoreBackend>(sub_idx: u64) -> bool {
        S::has_child(Self::name_id(), sub_idx)
    }

    #[inline]
    fn del_child<S: StoreBackend>(sub_idx: u64) -> bool {
        S::del_child(Self::name_id(), sub_idx)
    }

    #[inline]
    fn get_children<S: StoreBackend>() -> anyhow::Result<HashMap<u64, T>> {
        S::get_children(Self::name_id())
            .into_iter()
            .map(|(k, bin)| {
                T::from_store_data(bin)
                    .inspect_err(|e| tracing::error!(?e))
                    .map(|v| (k, v))
            })
            .try_collect()
    }

    #[inline]
    fn list_children<S: StoreBackend>() -> Vec<u64> {
        S::list_children(Self::name_id())
    }

    #[inline]
    fn reset_children<S: StoreBackend>(children: Vec<(u64, T)>) -> anyhow::Result<()> {
        let children: Vec<_> = children
            .into_iter()
            .map(|(k, v)| T::into_store_data(v).map(|v| (k, v)))
            .try_collect()
            .inspect_err(|e| tracing::error!(?e))?;
        S::swap_children(Self::name_id(), children);
        Ok(())
    }

    #[inline]
    fn clear_children<S: StoreBackend>() -> usize {
        S::clear_children(Self::name_id())
    }
}

pub trait Store<S: StoreBackend, K: Record<T>, T: StoreValue> {
    #[inline]
    fn set(val: T) -> anyhow::Result<()> {
        K::set::<S>(val)
    }

    #[inline]
    fn get() -> anyhow::Result<Option<T>> {
        K::get::<S>()
    }

    #[inline]
    fn has() -> bool {
        K::has::<S>()
    }

    #[inline]
    fn del() -> bool {
        K::del::<S>()
    }

    #[inline]
    fn set_child(sub_idx: u64, val: T) -> anyhow::Result<()> {
        K::set_child::<S>(sub_idx, val)
    }

    #[inline]
    fn get_child(sub_idx: u64) -> anyhow::Result<Option<T>> {
        K::get_child::<S>(sub_idx)
    }

    #[inline]
    fn has_child(sub_idx: u64) -> bool {
        K::has_child::<S>(sub_idx)
    }

    #[inline]
    fn del_child(sub_idx: u64) -> bool {
        K::del_child::<S>(sub_idx)
    }

    #[inline]
    fn get_children() -> anyhow::Result<HashMap<u64, T>> {
        K::get_children::<S>()
    }

    #[inline]
    fn list_children() -> Vec<u64> {
        K::list_children::<S>()
    }

    #[inline]
    fn reset_children(children: Vec<(u64, T)>) -> anyhow::Result<()> {
        K::reset_children::<S>(children)
    }

    #[inline]
    fn clear_children() -> usize {
        K::clear_children::<S>()
    }
}
