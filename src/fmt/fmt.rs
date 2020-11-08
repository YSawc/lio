use super::super::error::error::*;
use super::super::location::location::*;
use super::super::map::error::*;
use super::super::parser::error::*;
use super::super::token::error::*;
use super::super::token::token::*;
use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error")
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Num(n) => n.fmt(f),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::E => write!(f, "=="),
            TokenKind::NE => write!(f, "!="),
            TokenKind::L => write!(f, "<"),
            TokenKind::LE => write!(f, "<="),
            TokenKind::G => write!(f, ">"),
            TokenKind::GE => write!(f, ">="),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Int => write!(f, "int"),
            TokenKind::Ident(s) => s.fmt(f),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Map => write!(f, "map"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Fn => write!(f, "fn"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::UnderScore => write!(f, "_"),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::NotNumber(tok) => write!(f, "{}: {} is not number", tok.loc, tok.value),
            ParseError::NotOperator(tok) => write!(f, "{}: {} is not operator", tok.loc, tok.value),
            ParseError::NotImplementedOperator(tok) => {
                write!(f, "{}: {} is not implemented operator", tok.loc, tok.value)
            }
            ParseError::NotOpenedParen(tok) => write!(
                f,
                "{}: Expected opened paren but got {} ",
                tok.loc, tok.value
            ),
            ParseError::NotClosedParen(tok) => write!(
                f,
                "{}: Expected closeed paren but got {} ",
                tok.loc, tok.value
            ),
            ParseError::NotOpenedStmt(tok) => write!(
                f,
                "{}: Expected opened statement but final token got {}.",
                tok.loc, tok.value
            ),
            ParseError::NotClosedStmt(tok) => write!(
                f,
                "{}: Expected closeed statement but final token got {}.",
                tok.loc, tok.value
            ),
            ParseError::OperatorAfterRetrun(tok) => write!(
                f,
                "{}: {} Operator affter return not expected.",
                tok.loc, tok.value
            ),
            ParseError::NotIdent(tok) => {
                write!(f, "{}: Expected ident but got {}.", tok.loc, tok.value)
            }
            ParseError::NotAssign(tok) => {
                write!(f, "{}: Expected assign but got {}.", tok.loc, tok.value)
            }
            ParseError::NotDefinitionVar(tok) => write!(
                f,
                "{}: Expected definition var but not detected. Failed ident is {}",
                tok.loc, tok.value
            ),
            ParseError::NotLBrace(tok) => {
                write!(f, "{}: Expected LBrace but god {}", tok.loc, tok.value)
            }
            ParseError::NotRBrace(tok) => {
                write!(f, "{}: Expected RBrace but god {}", tok.loc, tok.value)
            }
            ParseError::OperatorOutOfFnction(tok) => write!(
                f,
                "{}: Expected operator inner function but god {}",
                tok.loc, tok.value
            ),
            ParseError::NotACompileTimeConstant(loc) => {
                write!(f, "{}: Not a compile time constant!", loc)
            }
            ParseError::UndefinedVariable(loc) => write!(f, "{} Undefined variable detected!", loc),
            ParseError::UnusedVariable(loc) => write!(f, "{} Unused variable detected!", loc),
            ParseError::NotMatchReturnType(loc) => write!(f, "{}: Return type is not match.", loc),
            ParseError::UnexpectedUnderScoreOperator(loc) => write!(
                f,
                "{}: Expected under socre as void return, but used as operator.",
                loc
            ),
            ParseError::NotMatchTypeAnotherOneOfStatement(loc) => write!(
                f,
                "{}: Expected match return type of each statement but not match.",
                loc
            ),

            ParseError::Eof => write!(f, "Expected token, but not detected."),
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.f, self.e)
    }
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let loc = &self.loc;
        match &self.value {
            TokenErrorKind::InvalidToken(t) => write!(f, "{}: invalid token '{:?}'", loc, t),
            TokenErrorKind::InvalidNumber(t) => write!(f, "{}: invalid number '{:?}'", loc, t),
        }
    }
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let loc = &self.loc;
        match &self.value {
            MapErrorKind::InvalidMap(t) => write!(
                f,
                "{}: invalid map found. Invalid token is : '{:?}'",
                loc, t
            ),
            MapErrorKind::InvalidStruct(t) => write!(
                f,
                "{}: invalid structue found. Final token found here : '{:?}'",
                loc, t
            ),
        }
    }
}
