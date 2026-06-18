# Rust Serializer Challenge

A versatile, abstract serialization and generic storage library in Rust that standardizes encoding and decoding across multiple formats. 

This project provides a unified interface allowing you to seamlessly encode, decode, and store Rust structs in-memory using different serialization formats, all through a single standardized interface.

## ✨ Features

- **Unified Serializer Interface**: A standard trait that effortlessly handles multiple byte-conversion schemas.
- **Generic Storage Wrapper**: An in-memory container (`Storage<T, S>`) that safely encapsulates your data alongside its intended serializer.
- **Multiple Supported Formats**:
  - `Borsh`: Highly efficient, compact binary serialization.
  - `Wincode`: Fast, zero-copy native Rust serialization.
  - `Json`: Standard, human-readable text-based serialization using `serde_json`.
- **Dynamic Conversion**: Allows zero-friction migration of data formats. You can convert a stored JSON object into Borsh bytes on-the-fly via the `.convert()` method.
- **Robust Type Safety**: Strongly typed generic bounds ensure that all data structures implement the necessary encoding and decoding schemas at compile-time.

## 🏗️ Architecture & Modules

### 1. `serializer.rs` (The Core Trait)
Defines the `Serializer` trait which mandates `to_bytes` and `from_bytes` behavior. It provides concrete, drop-in implementations for `Borsh`, `Wincode`, and `Json`. 
The generics heavily enforce trait bounds: any struct passed to these serializers must implement the encode/decode traits for *all* supported formats simultaneously.

### 2. `storage.rs` (The Container)
Defines `Storage<T, S>`, representing an in-memory byte buffer (`Option<Vec<u8>>`).
- **`save()` & `load()`**: Serializes native Rust structs into raw bytes, and decodes them back.
- **`convert()`**: A powerful method that effortlessly migrates a `Storage` instance from one serialization format to another. It decodes the internal bytes using the current format, re-encodes them with the new format, and yields a fresh `Storage` container.

### 3. `model.rs` (The Data)
Contains a sample `Person` struct. It acts as an example of how models must be annotated with all required macros (`#[derive(Serialize, Deserialize, BorshSerialize, SchemaWrite...)]`) to achieve full compatibility across the diverse serializer backends.

### 4. `error.rs` (Error Handling)
Provides strictly-typed custom error enums:
- `StorageError`: Used for failures during data saving, loading, or missing data.
- `ConvertError`: Safely wraps failures during the deserialization/re-serialization cycle of format conversion.

## 🧪 Testing

The library includes a robust test suite in `tests.rs` verifying:
1. Independent saving and loading of the `Person` struct across Borsh, Wincode, and JSON.
2. The `convert()` capability, executing a live conversion from a `Json` storage container to a `Borsh` container without data loss.

To execute the test suite, run:
```bash
cargo test
```

## 📜 License

This project is open-source. Feel free to use and expand the serialization formats!
