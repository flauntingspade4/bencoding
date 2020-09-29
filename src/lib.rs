//! This crate is used for encoding and decoding of bencoded variables.
//! ```
//! use bencoding::BenCodeAble;
//!
//! let example_string = "spam";
//!
//! assert_eq!("4:spam", example_string.bencode()); // "spam" is encoded as "4:spam"
//!
//! let example_int = 5;
//!
//! assert_eq!("i5e", example_int.bencode()); // 5 is encoded as "i5e"
//! ```
//!
//! Enable the serde feature to use this library with serde

mod bencode;
pub mod err;

pub use bencode::{from_str, BenCodeAble};

#[cfg(serde)]
mod serde;

pub enum BenCodeType<T: BenCodeAble> {
    BenCodedString(String),
    BenCodedInt(i64),
    BenCodedList(Vec<T>),
}
