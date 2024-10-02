use thiserror::Error;

#[derive(Error, Debug)]

pub enum BencodingSerializeError {
    #[error("{0}")]
    SerdeSerializeError(String),
}

impl serde::ser::Error for BencodingSerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::SerdeSerializeError(msg.to_string())
    }
}
