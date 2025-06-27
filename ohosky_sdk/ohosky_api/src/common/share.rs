pub trait ISharableValue: Sized {
    type ValueType;
    fn from_value(value: Self::ValueType) -> anyhow::Result<Self>;
    fn from_raw(data: Vec<u8>) -> Self;
    fn into_raw(self) -> Vec<u8>;
    fn from_bytes(data: bytes::Bytes) -> Self {
        Self::from_raw(data.into())
    }
    fn into_bytes(self) -> bytes::Bytes {
        bytes::Bytes::from(self.into_raw())
    }
    fn into_value(self) -> anyhow::Result<Self::ValueType>;
}

impl ISharableValue for Vec<u8> {
    type ValueType = Vec<u8>;

    fn from_value(value: Self::ValueType) -> anyhow::Result<Self> {
        Ok(value)
    }

    fn from_raw(data: Vec<u8>) -> Self {
        data
    }

    fn into_raw(self) -> Vec<u8> {
        self
    }

    fn into_value(self) -> anyhow::Result<Self::ValueType> {
        Ok(self)
    }
}

impl ISharableValue for () {
    type ValueType = ();

    fn from_value(_value: Self::ValueType) -> anyhow::Result<Self> {
        Ok(())
    }

    fn from_raw(_data: Vec<u8>) -> Self {}

    fn into_raw(self) -> Vec<u8> {
        RkyvValue::from_value(()).unwrap().into_raw()
    }

    fn into_value(self) -> anyhow::Result<Self::ValueType> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RkyvValue<T>
where
    T: for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                rkyv::rancor::Error,
            >,
        > + rkyv::Archive,
    T::Archived: rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>
        + rkyv::Portable
        + for<'a> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'a, rkyv::rancor::Error>>,
{
    data: Vec<u8>,
    _value_type: std::marker::PhantomData<T>,
}

impl<T> ISharableValue for RkyvValue<T>
where
    T: for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                rkyv::rancor::Error,
            >,
        > + rkyv::Archive,
    T::Archived: rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>
        + rkyv::Portable
        + for<'a> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'a, rkyv::rancor::Error>>,
{
    type ValueType = T;

    fn from_value(value: T) -> anyhow::Result<Self> {
        Ok(Self {
            data: rkyv::to_bytes(&value)?.into_vec(),
            _value_type: std::marker::PhantomData,
        })
    }

    fn from_raw(data: Vec<u8>) -> Self {
        Self {
            data,
            _value_type: std::marker::PhantomData,
        }
    }

    fn into_raw(self) -> Vec<u8> {
        self.data
    }

    fn into_value(self) -> anyhow::Result<T> {
        Ok(rkyv::from_bytes(&self.data)?)
    }
}

impl<T> RkyvValue<T>
where
    T: for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                rkyv::rancor::Error,
            >,
        > + rkyv::Archive,
    T::Archived: rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>
        + rkyv::Portable
        + for<'a> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'a, rkyv::rancor::Error>>,
{
    pub fn access(&self) -> anyhow::Result<&T::Archived> {
        Ok(rkyv::access(&self.data)?)
    }

    /// # Safety
    ///
    /// See `rkyv::access_unchecked` for more information.
    pub unsafe fn access_unchecked(&self) -> &T::Archived {
        unsafe { rkyv::access_unchecked(&self.data) }
    }

    pub fn access_mut(&'_ mut self) -> anyhow::Result<rkyv::seal::Seal<'_, T::Archived>> {
        Ok(rkyv::access_mut(&mut self.data)?)
    }

    pub fn serialize_from(value: &T) -> anyhow::Result<Self> {
        Ok(Self {
            data: rkyv::to_bytes(value)?.into_vec(),
            _value_type: std::marker::PhantomData,
        })
    }
}

#[test]
fn test_share_value() {
    let str = obfstr::obfstring!("test");

    #[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
    struct TestValue0(String);

    let value = TestValue0(str.clone());
    let value = RkyvValue::from_value(value).unwrap();
    assert_eq!(value.access().unwrap().0, str);

    let value = RkyvValue::from_value(str.clone()).unwrap();
    assert_eq!(value.access().unwrap(), &str);
}
