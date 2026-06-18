use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use wincode_derive::{SchemaRead, SchemaWrite};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize, SchemaWrite, SchemaRead)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub active: bool,
}
