use super::super::error::error::*;
use super::super::location::location::*;
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
            TokenKind::E => write!(f, "="),
            TokenKind::NE => write!(f, "!="),
            TokenKind::L => write!(f, "<"),
            TokenKind::LE => write!(f, "<="),
            TokenKind::G => write!(f, ">"),
            TokenKind::GE => write!(f, ">="),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Return => write!(f, "return"),
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
            ParseError::NotClosedParen(tok) => write!(
                f,
                "{}: Expected close token but got {} ",
                tok.loc, tok.value
            ),
            ParseError::NotClosedStmt(tok) => write!(
                f,
                "{}: Expected close statement but got {}.",
                tok.loc, tok.value
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
