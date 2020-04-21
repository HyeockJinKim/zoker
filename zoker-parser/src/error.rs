use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub error: ParseErrorType,
    // pub location: Location,
}

#[derive(Debug, PartialEq)]
pub struct LexicalError {
    pub error: LexicalErrorType,
    // pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum ParseErrorType {
    InvalidToken,
    UnrecognizedToken,
}

#[derive(Debug, PartialEq)]
pub enum LexicalErrorType {
    UnrecognizedToken,
}

impl From<std::num::ParseIntError> for LexicalError {
    fn from(_: std::num::ParseIntError) -> Self {
        LexicalError {
            error: LexicalErrorType::UnrecognizedToken,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseErrorType::InvalidToken => write!(f, "Got invalid token"),
            ParseErrorType::UnrecognizedToken => write!(f, "Got unexpected token"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
