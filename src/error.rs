use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum StorageError<E: Error + Send + Sync + 'static> {
    NoData,
    Serialization(E),
}

impl<E: Error + Send + Sync + 'static> Display for StorageError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::NoData => write!(f, "no data stored"),
            StorageError::Serialization(err) => write!(f, "serialization error: {}", err),
        }
    }
}

impl<E: Error + Send + Sync + 'static> Error for StorageError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            StorageError::NoData => None,
            StorageError::Serialization(err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum ConvertError<DE: Error + Send + Sync + 'static, SE: Error + Send + Sync + 'static> {
    NoData,
    Deserialize(DE),
    Serialize(SE),
}

impl<DE: Error + Send + Sync + 'static, SE: Error + Send + Sync + 'static> Display
    for ConvertError<DE, SE>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvertError::NoData => write!(f, "no data stored"),
            ConvertError::Deserialize(err) => write!(f, "deserialize error: {}", err),
            ConvertError::Serialize(err) => write!(f, "serialize error: {}", err),
        }
    }
}

impl<DE: Error + Send + Sync + 'static, SE: Error + Send + Sync + 'static> Error
    for ConvertError<DE, SE>
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConvertError::NoData => None,
            ConvertError::Deserialize(err) => Some(err),
            ConvertError::Serialize(err) => Some(err),
        }
    }
}
