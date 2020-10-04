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

mod bencode;
mod dict;
pub mod err;

pub use bencode::{from_str, BenCodeAble};

pub use dict::Dict;

/// An enum for the different kinds of bencoding, can be safely ignored
pub enum BenCodeType<T: BenCodeAble> {
    BenCodedString(String),
    BenCodedInt(i64),
    BenCodedList(Vec<T>),
}
