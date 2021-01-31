use std::fmt::Display;

pub type Result<T> = std::result::Result<T, DeBencodingError>;

pub enum DeBencodingError {
    /// Trailing characters after deserializing
    TrailingCharacters,
    /// A unrecognized type indicator, the supported
    /// being 'i' for integers, 'l' for lists, and
    /// any number for strings, the number indicating
    /// how long the string's data should be
    UnexpectedCharType(char),
    /// The colon hasn't been found while parsing a
    /// string
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
    Message(String),
}

impl std::fmt::Debug for DeBencodingError {
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
            Self::Message(s) => write!(f, "Serde error {}", s),
        }
    }
}

impl std::fmt::Display for DeBencodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DeBencodingError {}

impl serde::ser::Error for DeBencodingError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl serde::de::Error for DeBencodingError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}
