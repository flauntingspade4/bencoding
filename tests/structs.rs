// Structs used in testing

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Person {
    name: String,
    // Bencoding supports neither enums, or bools
    gender: String,
    age: u16,
}

impl Person {
    pub fn new(name: String, gender: String, age: u16) -> Self {
        Self { name, gender, age }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Publisher {
    name: String,
    publisher_webpage: String,
    publisher_location: String,
}

impl Publisher {
    pub fn new(name: String, publisher_webpage: String, publisher_location: String) -> Self {
        Self {
            name,
            publisher_webpage,
            publisher_location,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct StructContainingVec {
    pub vec: Vec<i64>,
}

impl From<Vec<i64>> for StructContainingVec {
    fn from(vec: Vec<i64>) -> Self {
        Self { vec }
    }
}
