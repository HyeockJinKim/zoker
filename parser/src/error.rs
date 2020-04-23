use lalrpop_util::ParseError as LalrpopError;

use crate::location::Location;
use crate::token::Tok;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub error: ParseErrorType,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub struct LexicalError {
    pub error: LexicalErrorType,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum ParseErrorType {
    /// Parser encountered an unexpected end of input
    EOF,
    /// Parser encountered an extra token
    ExtraToken(Tok),
    /// Parser encountered an invalid token
    InvalidToken,
    /// Parser encountered an unexpected token
    UnrecognizedToken(Tok, Option<String>),
    /// Maps to `User` type from `lalrpop-util`
    Lexical(LexicalErrorType),
}

#[derive(Debug, PartialEq)]
pub enum LexicalErrorType {
    UnrecognizedToken { tok: char },
    OtherError(String),
}

impl From<LalrpopError<Location, Tok, LexicalError>> for ParseError {
    fn from(err: LalrpopError<Location, Tok, LexicalError>) -> Self {
        match err {
            LalrpopError::InvalidToken { location } => ParseError {
                error: ParseErrorType::EOF,
                location,
            },
            LalrpopError::ExtraToken { token } => ParseError {
                error: ParseErrorType::ExtraToken(token.1),
                location: token.0,
            },
            LalrpopError::User { error } => ParseError {
                error: ParseErrorType::Lexical(error.error),
                location: error.location,
            },
            LalrpopError::UnrecognizedToken { token, expected } => {
                // Hacky, but it's how CPython does it. See PyParser_AddToken,
                // in particular "Only one possible expected token" comment.
                let expected = if expected.len() == 1 {
                    Some(expected[0].clone())
                } else {
                    None
                };
                ParseError {
                    error: ParseErrorType::UnrecognizedToken(token.1, expected),
                    location: token.0,
                }
            }
            LalrpopError::UnrecognizedEOF { location, .. } => ParseError {
                error: ParseErrorType::EOF,
                location,
            },
        }
    }
}

impl From<num_bigint::ParseBigIntError> for LexicalError {
    fn from(_err: num_bigint::ParseBigIntError) -> Self {
        LexicalError {
            error: LexicalErrorType::UnrecognizedToken { tok: 'c' },
            location: Default::default(),
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
        match self {
            ParseErrorType::InvalidToken => write!(f, "Got invalid token"),
            ParseErrorType::UnrecognizedToken(_tok, _opts) => write!(f, "Got unexpected token"),
            _ => write!(f, "Got parser Error"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
