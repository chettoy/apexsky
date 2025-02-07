use dataview::Pod;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Default, Pod, FromBytes, Immutable, IntoBytes, KnownLayout,
)]
#[repr(C)]
pub struct kbutton_t {
    pub down: [i32; 2],
    pub state: u32,
}
