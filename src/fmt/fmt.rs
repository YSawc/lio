use super::super::location::location::*;
use super::super::token::error::*;
use super::super::token::token::*;
use std::fmt;

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Num(n) => n.fmt(f),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
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
