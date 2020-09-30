use super::super::parser::error::*;
use super::super::token::error::*;

// use super::super::

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
