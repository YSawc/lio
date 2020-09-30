use super::super::location::location::*;
use super::super::token::token::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenErrorKind {
    InvalidToken(char),
    InvalidNumber(Token),
}

pub type TokenError = Annot<TokenErrorKind>;

impl TokenError {
    pub fn invalid_token(c: char, loc: Loc) -> Self {
        Self::new(TokenErrorKind::InvalidToken(c), loc)
    }

    pub fn invalid_number(tok: Token, loc: Loc) -> Self {
        Self::new(TokenErrorKind::InvalidNumber(tok), loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Error {
    Token(TokenError),
}

impl From<TokenError> for Error {
    fn from(e: TokenError) -> Self {
        Error::Token(e)
    }
}
