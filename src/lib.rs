#![warn(clippy::pedantic, clippy::nursery)]

//! This crate is used for encoding and decoding of bencoded variables.
//! ```
//! use bencoding::{to_string, from_str};
//!
//! let example_string = "spam";
//!
//! assert_eq!("4:spam", to_string(&example_string).unwrap()); // "spam" is encoded as "4:spam"
//!
//! let example_int = 5;
//!
//! assert_eq!("i5e", to_string(&example_int).unwrap()); // 5 is encoded as "i5e"
//! ```

pub mod de;
pub mod ser;

pub use de::{from_bytes, from_str};
pub use ser::to_string;
