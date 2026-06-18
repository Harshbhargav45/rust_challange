# Rust Serializer Challenge

A versatile, abstract serialization library in Rust that standardizes encoding and decoding across multiple formats. 

This project defines a unified `Serializer` trait, making it effortless to switch between different serialization mechanisms without changing your core application logic.

## ✨ Features

- **Unified Interface**: Implements a standard `Serializer` trait for converting to and from byte vectors (`Vec<u8>`).
- **Multiple Supported Formats**:
  - `Borsh`: Highly efficient, compact binary serialization.
  - `Wincode`: Fast, zero-copy native Rust serialization.
  - `Json`: Standard, human-readable text-based serialization using `serde_json`.
- **Trait Bounds**: Strongly typed generic methods that ensure the data structs implement the required `Serialize`, `DeserializeOwned`, `BorshSerialize`, `BorshDeserialize`, and `SchemaRead`/`SchemaWrite` traits simultaneously.

## 🏗️ Architecture

### The `Serializer` Trait

The core of the project is located in `src/serializer.rs`:

```rust
pub trait Serializer {
    type Error: Error + Send + Sync + 'static;

    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>, Self::Error>
    where
        T: ?Sized + Serialize + BorshSerialize + wincode::SchemaWrite<Src = T>;

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T, Self::Error>
    where
        T: Sized + DeserializeOwned + BorshDeserialize + for<'de> wincode::SchemaRead<'de, Dst = T>;
}
```

This enforces a uniform API across all implementations: `Borsh`, `Wincode`, and `Json`.

## 🧪 Testing

The library includes a robust suite of tests guaranteeing that all serializers accurately save, load, and convert data formats.

```bash
cargo test
```
*Current status: All tests passing!*

## 📜 License

This project is open-source. Feel free to use and expand the serialization formats!
