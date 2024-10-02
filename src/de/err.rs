use thiserror::Error;

#[derive(Error, Debug)]
pub enum BencodingDeserializeError {
    #[error("{0}")]
    SerdeDeserializeError(String),
    #[error("a character was tried to be read with none remaining")]
    OutOfCharacters,
    #[error("after parsing there are unexpected characters remaining")]
    TrailingCharacters,
    #[error("opening symbol '{0}' was expected but not found")]
    NoFoundOpeningDeliminator(char),
    #[error("symbol ':' was expected but not found")]
    NoFoundColon,
    #[error("closing symbol '{0}' was expected but not found")]
    NoFoundClosingDeliminator(char),
    #[error("a string containing invalid UTF-8 was tried to be read")]
    InputNotUtf8,
    #[error("an error occurred while parsing an int")]
    ParseIntError,
    #[error("type 'bool' not supported")]
    InvalidTypeBool,
    #[error("type 'float' not supported")]
    InvalidTypeFloat,
    #[error("type '{0}' not supported")]
    InvalidTypeOther(char),
    #[error("expected null")]
    ExpectedNull,
}

impl serde::de::Error for BencodingDeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::SerdeDeserializeError(msg.to_string())
    }
}
