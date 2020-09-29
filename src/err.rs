pub enum DeBencodingError {
    EndOfString,
    UnexpectedCharType(char),
    NoFoundColon,
    ParseIntError,
    WrongLengthOfString,
    NoFoundClosingDeliminator,
}

impl std::fmt::Debug for DeBencodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DeBencodingError::*;
        match self {
            EndOfString => write!(f, "Unexpected end of string"),
            DeBencodingError::UnexpectedCharType(c) => {
                write!(f, "Unexpected character type while parsing '{}'", c)
            }
            NoFoundColon => write!(f, "No colon found whilst parsing string"),
            ParseIntError => write!(f, "Error occurred whilst parsing integer"),
            WrongLengthOfString => write!(
                f,
                "The length of the string does not match the given length"
            ),
            NoFoundClosingDeliminator => write!(f, "No found closing deliminator while parsing"),
        }
    }
}
