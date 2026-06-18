use borsh::{BorshDeserialize, BorshSerialize};
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

pub trait Serializer {
    type Error: Error + Send + Sync + 'static;

    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: ?Sized + Serialize + BorshSerialize + wincode::SchemaWrite<Src = T>;

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, Self::Error>
    where
        T: Sized
            + DeserializeOwned
            + BorshDeserialize
            + for<'de> wincode::SchemaRead<'de, Dst = T>;
}

pub struct Borsh;
pub struct Wincode;
pub struct Json;

impl Serializer for Borsh {
    type Error = borsh::maybestd::io::Error;

    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: ?Sized + Serialize + BorshSerialize + wincode::SchemaWrite<Src = T>,
    {
        let mut bytes = Vec::new();
        BorshSerialize::serialize(&value, &mut bytes)?;
        Ok(bytes)
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, Self::Error>
    where
        T: Sized + DeserializeOwned + BorshDeserialize + for<'de> wincode::SchemaRead<'de, Dst = T>,
    {
        T::try_from_slice(bytes)
    }
}

impl Serializer for Wincode {
    type Error = wincode::Error;

    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: ?Sized + Serialize + BorshSerialize + wincode::SchemaWrite<Src = T>,
    {
        Ok(wincode::serialize(value)?)
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, Self::Error>
    where
        T: Sized + DeserializeOwned + BorshDeserialize + for<'de> wincode::SchemaRead<'de, Dst = T>,
    {
        Ok(wincode::deserialize(bytes)?)
    }
}

impl Serializer for Json {
    type Error = serde_json::Error;

    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: ?Sized + Serialize + BorshSerialize + wincode::SchemaWrite<Src = T>,
    {
        serde_json::to_vec(value)
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, Self::Error>
    where
        T: Sized + DeserializeOwned + BorshDeserialize + for<'de> wincode::SchemaRead<'de, Dst = T>,
    {
        serde_json::from_slice(bytes)
    }
}
