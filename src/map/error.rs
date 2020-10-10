use super::super::location::location::*;
use super::super::token::token::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapErrorKind {
    InvalidMap(Token),
    InvalidStruct(Token),
}

pub type MapError = Annot<MapErrorKind>;

impl MapError {
    pub fn invalid_map(tok: Token, loc: Loc) -> Self {
        Self::new(MapErrorKind::InvalidMap(tok), loc)
    }
    pub fn invalid_struct(tok: Token, loc: Loc) -> Self {
        Self::new(MapErrorKind::InvalidStruct(tok), loc)
    }
}
