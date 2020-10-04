pub type Result<T> = std::result::Result<T, DeBencodingError>;

pub enum DeBencodingError {
    /// Trailing characters after deserializing
    TrailingCharacters,
    /// A unrecognized type indicator, the supported
    /// being 'i' for integers, 'l' for lists, and
    /// any number for strings, the number indicating
    /// how long the string's data should be/
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
    EoF,
    /// An integer was expected, but not provided
    ExpectedInt,
}

impl std::fmt::Debug for DeBencodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DeBencodingError::*;
        match self {
            TrailingCharacters => write!(f, "Unexpected trailing characters"),
            DeBencodingError::UnexpectedCharType(c) => {
                write!(f, "Unexpected character type while parsing '{}'", c)
            }
            NoFoundColon => write!(f, "No colon found whilst parsing string"),
            ParseIntError => write!(f, "Error occurred whilst parsing integer"),
            WrongLengthOfString => write!(
                f,
                "The length of the string does not match the given length"
            ),
            NoFoundOpeningDeliminator => write!(f, "No found opening deliminator while parsing"),
            NoFoundClosingDeliminator => write!(f, "No found closing deliminator while parsing"),
            EoF => write!(f, "Unexpected end of file"),
            ExpectedInt => write!(f, "An integer was expected, but not provided"),
        }
    }
}

impl std::fmt::Display for DeBencodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
