use super::super::location::location::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenErrorKind {
    InvalidToken(char),
    InvalidNumber,
}

pub type TokenError = Annot<TokenErrorKind>;

impl TokenError {
    pub fn invalid_token(c: char, loc: Loc) -> Self {
        Self::new(TokenErrorKind::InvalidToken(c), loc)
    }

    pub fn invalid_number(loc: Loc) -> Self {
        Self::new(TokenErrorKind::InvalidNumber, loc)
    }
}
