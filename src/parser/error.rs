// use super::parser::*;
use super::super::token::token::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    NotNumber(Token),
    NotOperator(Token),
    NotImplementedOperator(Token),
    // UnexpectedToken(Token),
    // NotExpression(Token),
    // UnclosedOpenParen(Token),
    // RedundantExpression(Token),
    Eof,
}
