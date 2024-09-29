use std::fmt::Display;

pub type Result<T> = std::result::Result<T, DeBencodingError>;

#[derive(Debug, Clone)]
pub enum DeBencodingError {
    /// Trailing characters after deserializing
    TrailingCharacters,
    /// A unrecognized type indicator, the supported
    /// being 'i' for integers, 'l' for lists, 'd' for
    /// dictionaries and an integer for strings, indicating
    /// it's length
    UnexpectedCharType(char),
    /// Strings are of the format `length:value`. This error
    /// indicates the colon between length and value hasn't been
    /// found
    NoFoundColon,
    /// An error related to parsing an integer
    ParseIntError,
    /// An error indicating that the string after
    /// the colon is the wrong length
    WrongLengthOfString,
    /// An error for when no opening deliminator
    /// has been found
    NoFoundOpeningDeliminator,
    /// An error for when no closing deliminator
    /// has been found
    NoFoundClosingDeliminator,
    /// An unexpected end of file
    Eof,
    /// An integer was expected, but not provided
    ExpectedInt,
    /// Expected a null value
    ExpectedNull,
    /// Expected a char
    ExpectedChar,
    /// A generic error type for serde
    SerdeError(String),
    /// Only integer, list, string and dictionaries are supported
    UnsupportedValueType,
    /// A deserializer was attempted to be constructed using bytes
    /// that were not valid utf8
    InputNotUtf8,
}

impl std::fmt::Display for DeBencodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TrailingCharacters => write!(f, "Unexpected trailing characters"),
            Self::UnexpectedCharType(c) => {
                write!(f, "Unexpected character type while parsing '{}'", c)
            }
            Self::NoFoundColon => write!(f, "No colon found whilst parsing string"),
            Self::ParseIntError => write!(f, "Error occurred whilst parsing integer"),
            Self::WrongLengthOfString => write!(
                f,
                "The length of the string does not match the given length"
            ),
            Self::NoFoundOpeningDeliminator => {
                write!(f, "No found opening deliminator while parsing")
            }
            Self::NoFoundClosingDeliminator => {
                write!(f, "No found closing deliminator while parsing")
            }
            Self::Eof => write!(f, "Unexpected end of file"),
            Self::ExpectedInt => write!(f, "An integer was expected, but not provided"),
            Self::ExpectedNull => write!(f, "Expected a null value"),
            Self::ExpectedChar => write!(f, "Expected a char"),
            Self::SerdeError(s) => write!(f, "Serde error {}", s),
            Self::UnsupportedValueType => write!(
                f,
                "Only integer, list, string and dictionaries are supported"
            ),
            Self::InputNotUtf8 => write!(f, "A deserializer was attempted to be constructed using bytes that were not valid utf8"),
        }
    }
}

impl std::error::Error for DeBencodingError {}

impl serde::ser::Error for DeBencodingError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::SerdeError(msg.to_string())
    }
}

impl serde::de::Error for DeBencodingError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::SerdeError(msg.to_string())
    }
}
