use zerocopy::{FromBytes, Immutable, KnownLayout};

use super::Ptr;

#[derive(Debug, Default, FromBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct CUtlMemory<T> {
    pub pMemory: Ptr<[T]>,
    pub nAllocationCount: i32,
    pub nGrowSize: i32,
}

#[derive(Debug, Default, FromBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct CUtlVector<T> {
    pub Memory: CUtlMemory<T>,
    pub Size: i32,
    pub pElements: Ptr<[T]>,
}
