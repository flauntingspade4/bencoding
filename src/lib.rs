//! This crate is used for encoding and decoding of bencoded variables.
//! ```
//! /*use bencoding::BenCodeAble;
//!
//! let example_string = "spam";
//!
//! assert_eq!("4:spam", example_string.bencode()); // "spam" is encoded as "4:spam"
//!
//! let example_int = 5;
//!
//! assert_eq!("i5e", example_int.bencode()); // 5 is encoded as "i5e" */
//! ```

mod dict;
pub mod err;


mod de;
mod ser;

pub use de::from_str;
pub use ser::to_string;

/*
/// An enum for the different kinds of bencoding, can be safely ignored
pub enum BenCodeType<T: BenCodeAble> {
    BenCodedString(String),
    BenCodedInt(i64),
    BenCodedList(Vec<T>),
}
*/
