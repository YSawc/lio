use super::super::location::location::*;
use super::super::token::token::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    NotNumber(Token),
    NotOperator(Token),
    NotImplementedOperator(Token),
    NotOpenedParen(Token),
    NotClosedParen(Token),
    NotOpenedStmt(Token),
    NotClosedStmt(Token),
    OperatorAfterRetrun(Token),
    NotIdent(Token),
    NotAssign(Token),
    NotLBrace(Token),
    NotRBrace(Token),
    OperatorOutOfFnction(Token),
    NotType(Token),
    NotDefinitionVar(Loc),
    NotACompileTimeConstant(Loc),
    UndefinedVariable(Loc),
    UnusedVariable(Loc),
    NotMatchReturnType(Loc),
    UnexpectedUnderScoreOperator(Loc),
    NotMatchTypeAnotherOneOfStatement(Loc),
    NotClosedImmediate(Loc),
    NotOpenedImmediate(Loc),
    Eof,
}
