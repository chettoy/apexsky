use std::{fmt, hash, marker, mem, ops};

#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[repr(transparent)]
pub struct IntPtr64<T: ?Sized = ()> {
    address: u64,
    phantom_data: marker::PhantomData<fn(T) -> T>,
}

#[inline]
const fn nibbles(word: u64) -> [u8; 16] {
    let b = word.to_le_bytes();
    [
        b[7] >> 4,
        b[7] & 0xf,
        b[6] >> 4,
        b[6] & 0xf,
        b[5] >> 4,
        b[5] & 0xf,
        b[4] >> 4,
        b[4] & 0xf,
        b[3] >> 4,
        b[3] & 0xf,
        b[2] >> 4,
        b[2] & 0xf,
        b[1] >> 4,
        b[1] & 0xf,
        b[0] >> 4,
        b[0] & 0xf,
    ]
}
#[inline]
const fn digit(nibble: u8) -> u8 {
    if nibble < 10 {
        b'0' + nibble
    } else {
        b'a' + (nibble - 10)
    }
}

impl<T: ?Sized> ArchivedIntPtr64<T> {
    pub fn to_raw(&self) -> u64 {
        self.address.into()
    }
}

impl<T: ?Sized> IntPtr64<T> {
    // Work around unstable const fn features
    const PHANTOM_DATA: marker::PhantomData<fn(T) -> T> = marker::PhantomData;
    /// Null pointer constant.
    pub const NULL: IntPtr64<T> = IntPtr64 {
        address: 0,
        phantom_data: marker::PhantomData,
    };
    /// Creates a null pointer.
    #[inline]
    pub const fn new() -> IntPtr64<T> {
        IntPtr64::NULL
    }
    /// Creates a pointer from its address.
    #[inline]
    pub const fn from_raw(address: u64) -> IntPtr64<T> {
        IntPtr64 {
            address,
            phantom_data: Self::PHANTOM_DATA,
        }
    }
    /// Constructs a pointer with an offset.
    #[inline]
    pub const fn member(address: u64, offset: u32) -> IntPtr64<T> {
        let address = address + offset as u64;
        IntPtr64 {
            address,
            phantom_data: Self::PHANTOM_DATA,
        }
    }
    /// Returns true if the pointer is null.
    #[inline]
    pub const fn is_null(&self) -> bool {
        self.address == 0
    }
    /// Casts the pointer to a different type keeping the pointer address fixed.
    #[inline]
    pub const fn cast<U: ?Sized>(self) -> IntPtr64<U> {
        IntPtr64 {
            address: self.address,
            phantom_data: IntPtr64::<U>::PHANTOM_DATA,
        }
    }
    /// Offsets the pointer to a field.
    #[inline]
    pub const fn field<U: ?Sized>(self, offset: u32) -> IntPtr64<U> {
        let address = self.address + offset as u64;
        IntPtr64 {
            address,
            phantom_data: IntPtr64::<U>::PHANTOM_DATA,
        }
    }
    /// Offsets the pointer and cast.
    #[inline]
    pub const fn offset<U: ?Sized>(self, offset: i64) -> IntPtr64<U> {
        let address = self.address.wrapping_add(offset as u64);
        IntPtr64 {
            address,
            phantom_data: IntPtr64::<U>::PHANTOM_DATA,
        }
    }
    /// Returns the raw integer, type ascription helper.
    #[inline]
    pub const fn into_raw(self) -> u64 {
        self.address
    }
    /// Formats the pointer.
    #[inline]
    pub const fn fmt(self) -> [u8; 18] {
        let n = nibbles(self.address);
        [
            b'0',
            b'x',
            digit(n[0]),
            digit(n[1]),
            digit(n[2]),
            digit(n[3]),
            digit(n[4]),
            digit(n[5]),
            digit(n[6]),
            digit(n[7]),
            digit(n[8]),
            digit(n[9]),
            digit(n[10]),
            digit(n[11]),
            digit(n[12]),
            digit(n[13]),
            digit(n[14]),
            digit(n[15]),
        ]
    }
}
#[cfg(target_pointer_width = "64")]
impl<T: ?Sized> IntPtr64<T> {
    /// Creates a pointer from `usize`.
    #[inline]
    pub const fn from_usize(address: usize) -> IntPtr64<T> {
        Self::from_raw(address as u64)
    }
    /// Returns the address as `usize`.
    #[inline]
    pub const fn into_usize(self) -> usize {
        self.address as usize
    }
}

impl<T> IntPtr64<[T]> {
    /// Decays the pointee from `[T]` to `T`.
    #[inline]
    pub const fn decay(self) -> IntPtr64<T> {
        IntPtr64 {
            address: self.address,
            phantom_data: IntPtr64::<T>::PHANTOM_DATA,
        }
    }
    /// Pointer arithmetic, gets the pointer of an element at the specified index.
    #[inline]
    pub const fn at(self, i: usize) -> IntPtr64<T> {
        let address = self.address + (i * mem::size_of::<T>()) as u64;
        IntPtr64 {
            address,
            phantom_data: IntPtr64::<T>::PHANTOM_DATA,
        }
    }
}
impl<T, const N: usize> IntPtr64<[T; N]> {
    /// Decays the pointee from `[T; N]` to `T`.
    #[inline]
    pub const fn decay(self) -> IntPtr64<T> {
        IntPtr64 {
            address: self.address,
            phantom_data: IntPtr64::<T>::PHANTOM_DATA,
        }
    }
    /// Pointer arithmetic, gets the pointer of an element at the specified index.
    #[inline]
    pub const fn at(self, i: usize) -> IntPtr64<T> {
        let address = self.address + (i * mem::size_of::<T>()) as u64;
        IntPtr64 {
            address,
            phantom_data: IntPtr64::<T>::PHANTOM_DATA,
        }
    }
}

impl<T: ?Sized> Copy for IntPtr64<T> {}
impl<T: ?Sized> Clone for IntPtr64<T> {
    #[inline]
    fn clone(&self) -> IntPtr64<T> {
        *self
    }
}
impl<T: ?Sized> Default for IntPtr64<T> {
    #[inline]
    fn default() -> IntPtr64<T> {
        IntPtr64::NULL
    }
}
impl<T: ?Sized> Eq for IntPtr64<T> {}
impl<T: ?Sized> PartialEq for IntPtr64<T> {
    #[inline]
    fn eq(&self, rhs: &IntPtr64<T>) -> bool {
        self.address == rhs.address
    }
}
impl<T: ?Sized> hash::Hash for IntPtr64<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.address.hash(state)
    }
}
impl<T: ?Sized> AsRef<u64> for IntPtr64<T> {
    #[inline]
    fn as_ref(&self) -> &u64 {
        &self.address
    }
}
impl<T: ?Sized> AsMut<u64> for IntPtr64<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut u64 {
        &mut self.address
    }
}

impl<T: ?Sized> From<u64> for IntPtr64<T> {
    #[inline]
    fn from(address: u64) -> IntPtr64<T> {
        IntPtr64 {
            address,
            phantom_data: marker::PhantomData,
        }
    }
}
impl<T: ?Sized> From<IntPtr64<T>> for u64 {
    #[inline]
    fn from(ptr: IntPtr64<T>) -> u64 {
        ptr.address
    }
}

impl<T> ops::Add<usize> for IntPtr64<T> {
    type Output = IntPtr64<T>;
    #[inline]
    fn add(self, other: usize) -> IntPtr64<T> {
        let address = self.address + (other * mem::size_of::<T>()) as u64;
        IntPtr64 {
            address,
            phantom_data: self.phantom_data,
        }
    }
}
impl<T> ops::Sub<usize> for IntPtr64<T> {
    type Output = IntPtr64<T>;
    #[inline]
    fn sub(self, other: usize) -> IntPtr64<T> {
        let address = self.address - (other * mem::size_of::<T>()) as u64;
        IntPtr64 {
            address,
            phantom_data: self.phantom_data,
        }
    }
}

impl<T: ?Sized> fmt::Debug for ArchivedIntPtr64<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.address == 0 {
            f.pad("0x0")
        } else {
            let buf = IntPtr64::<()>::fmt(self.address.to_native().into());
            f.pad(unsafe { std::str::from_utf8_unchecked(&buf) })
        }
    }
}
impl<T: ?Sized> fmt::Debug for IntPtr64<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_null() {
            f.pad("0x0")
        } else {
            let buf = IntPtr64::fmt(*self);
            f.pad(unsafe { std::str::from_utf8_unchecked(&buf) })
        }
    }
}
impl<T: ?Sized> fmt::UpperHex for IntPtr64<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.address.fmt(f)
    }
}
impl<T: ?Sized> fmt::LowerHex for IntPtr64<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.address.fmt(f)
    }
}
impl<T: ?Sized> fmt::Display for IntPtr64<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_null() {
            f.pad("0x0")
        } else {
            let buf = IntPtr64::fmt(*self);
            f.pad(unsafe { std::str::from_utf8_unchecked(&buf) })
        }
    }
}

unsafe impl<T: ?Sized + 'static> dataview::Pod for IntPtr64<T> {}

#[test]
fn units() {
    let a = IntPtr64::<f64>::from(0x2000);
    let b = a + 0x40;
    let c = a - 0x40;
    assert_eq!(mem::size_of_val(&a), 8);
    assert_eq!(b.into_raw(), 0x2200);
    assert_eq!(format!("{}", a), "0x0000000000002000");
    assert_eq!(format!("{}", IntPtr64::<()>::NULL), "0x0");
    assert_eq!(c.into_raw(), 0x1E00);
    assert_eq!(
        IntPtr64::<[u32]>::from_raw(0x1000).at(1),
        IntPtr64::<u32>::from_raw(0x1004)
    );
    assert_eq!(
        IntPtr64::<[u32; 2]>::from_raw(0x1000).at(1),
        IntPtr64::<u32>::from_raw(0x1004)
    );
}
