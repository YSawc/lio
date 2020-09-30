use super::super::parser::error::*;
use super::super::token::error::*;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Error {
    Token(TokenError),
    Parse(ParseError),
}

impl From<TokenError> for Error {
    fn from(e: TokenError) -> Self {
        Error::Token(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parse(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error")
    }
}

impl StdError for TokenError {}

impl StdError for ParseError {}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use self::Error::*;
        match self {
            Token(tok) => Some(tok),
            Parse(parse) => Some(parse),
        }
    }
}
