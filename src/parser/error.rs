// use super::parser::*;
use super::super::token::token::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    // UnexpectedToken(Token),
    // NotExpression(Token),
    NotOperator(Token),
    // UnclosedOpenParen(Token),
    // RedundantExpression(Token),
    // Eof,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotOperator(tok) => write!(f, "{}: '{}' is not an operator", tok.loc, tok.loc),
        }
    }
}
