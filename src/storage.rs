use crate::error::{ConvertError, StorageError};
use crate::serializer::Serializer;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

pub struct Storage<T, S> {
    bytes: Option<Vec<u8>>,
    serializer: S,
    _phantom: PhantomData<T>,
}

impl<T, S> Storage<T, S> {
    pub fn new(serializer: S) -> Self {
        Self {
            bytes: None,
            serializer,
            _phantom: PhantomData,
        }
    }
}

impl<T, S> Storage<T, S>
where
    S: Serializer,
{
    pub fn save(&mut self, value: &T) -> Result<(), StorageError<S::Error>>
    where
        T: Serialize + BorshSerialize + wincode::SchemaWrite<Src = T>,
    {
        let bytes = self.serializer.to_bytes(value).map_err(StorageError::Serialization)?;
        self.bytes = Some(bytes);
        Ok(())
    }

    pub fn load(&self) -> Result<T, StorageError<S::Error>>
    where
        T: Sized
            + DeserializeOwned
            + BorshDeserialize
            + for<'de> wincode::SchemaRead<'de, Dst = T>,
    {
        let bytes = self.bytes.as_deref().ok_or(StorageError::NoData)?;
        let value = self.serializer.from_bytes(bytes).map_err(StorageError::Serialization)?;
        Ok(value)
    }

    pub fn convert<U>(
        &self,
        serializer: U,
    ) -> Result<Storage<T, U>, ConvertError<S::Error, U::Error>>
    where
        U: Serializer,
        T: Sized
            + Serialize
            + DeserializeOwned
            + BorshSerialize
            + BorshDeserialize
            + for<'de> wincode::SchemaRead<'de, Dst = T>
            + wincode::SchemaWrite<Src = T>,
    {
        let value = self.load().map_err(|err| match err {
            StorageError::NoData => ConvertError::NoData,
            StorageError::Serialization(source_err) => ConvertError::Deserialize(source_err),
        })?;

        let mut new_storage = Storage::new(serializer);
        new_storage
            .save(&value)
            .map_err(|err| match err {
                StorageError::NoData => ConvertError::NoData,
                StorageError::Serialization(write_err) => ConvertError::Serialize(write_err),
            })?;

        Ok(new_storage)
    }

    pub fn has_data(&self) -> bool {
        self.bytes.is_some()
    }
}
