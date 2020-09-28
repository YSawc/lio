use super::super::location::location::*;
use super::super::token::token::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeErrorKind {
    InvalidOp(Token),
    NotNumber(Token)
}

pub type NodeError = Annot<NodeErrorKind>;

impl NodeError {
    pub fn invalid_op(tok: Token, loc: Loc) -> Self {
        Self::new(NodeErrorKind::InvalidOp(tok), loc)
    }
    pub fn not_number(tok: Token, loc: Loc) -> Self {
        Self::new(NodeErrorKind::NotNumber(tok), loc)
    }
}
