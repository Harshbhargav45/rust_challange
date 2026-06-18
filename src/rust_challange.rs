mod error;
mod model;
mod serializer;
mod storage;

pub use error::{ConvertError, StorageError};
pub use model::Person;
pub use serializer::{Borsh, Json, Serializer, Wincode};
pub use storage::Storage;

#[cfg(test)]
mod tests;
