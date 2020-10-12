use super::super::token::token::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    NotNumber(Token),
    NotOperator(Token),
    NotImplementedOperator(Token),
    NotClosedParen(Token),
    NotClosedStmt(Token),
    OperatorAfterRetrun(Token),
    NotIdent(Token),
    NotAssign(Token),
    NotDefinitionVar(Token),
    NotLBrace(Token),
    NotRBrace(Token),
    OperatorOutOfFnction(Token),
    // UnexpectedToken(Token),
    // NotExpression(Token),
    // UnclosedOpenParen(Token),
    // RedundantExpression(Token),
    Eof,
}
