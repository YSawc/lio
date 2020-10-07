use super::super::location::location::*;
use super::super::node::node::*;
use super::super::token::token::*;
use super::error::*;

impl NodeSt {
    pub fn new_nds(c: Node, lhs: NodeSt, rhs: NodeSt) -> Self {
        Self {
            c,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
            ..Default::default()
        }
    }

    pub fn new_num(mut t: Token) -> Result<Self, ParseError> {
        let n = match Token::get_val(&mut t) {
            Ok(n) => n,
            Err(_) => return Err(ParseError::NotNumber(t)),
        };

        Ok(Self {
            c: Annot::new(NodeKind::Num(n), t.loc),
            ..Default::default()
        })
    }
}

impl NodeSt {
    pub fn parser(vt: Vec<Token>) -> Result<Self, ParseError> {
        let mut it = vt.iter().peekable();
        let lhs = Self::cmp(&mut it)?;
        Ok(lhs)
    }

    pub fn cmp(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        let mut lhs = Self::expr(it)?;

        loop {
            match it.peek().map(|vt| vt.value.to_owned()) {
                Some(TokenKind::E) | Some(TokenKind::NE) | Some(TokenKind::L)
                | Some(TokenKind::LE) | Some(TokenKind::G) | Some(TokenKind::GE) => {
                    let op = match it.next().unwrap() {
                        Token {
                            value: TokenKind::E,
                            loc,
                        } => Node::eq(loc.to_owned()),
                        Token {
                            value: TokenKind::NE,
                            loc,
                        } => Node::neq(loc.to_owned()),
                        Token {
                            value: TokenKind::L,
                            loc,
                        } => Node::l(loc.to_owned()),
                        Token {
                            value: TokenKind::LE,
                            loc,
                        } => Node::le(loc.to_owned()),
                        Token {
                            value: TokenKind::G,
                            loc,
                        } => Node::g(loc.to_owned()),
                        Token {
                            value: TokenKind::GE,
                            loc,
                        } => Node::ge(loc.to_owned()),

                        _ => unreachable!(),
                    };
                    let rhs = Self::expr(&mut it)?;

                    lhs = Self::new_nds(op, lhs, rhs);
                }
                _ => return Ok(lhs),
            }
        }
    }

    pub fn expr(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        let mut lhs = Self::mul(it)?;

        loop {
            match it.peek().map(|vt| vt.value.to_owned()) {
                Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
                    let op = match it.next().unwrap() {
                        Token {
                            value: TokenKind::Plus,
                            loc,
                        } => Node::plus(loc.to_owned()),
                        Token {
                            value: TokenKind::Minus,
                            loc,
                        } => Node::minus(loc.to_owned()),
                        _ => unreachable!(),
                    };
                    let rhs = Self::mul(&mut it)?;

                    lhs = Self::new_nds(op, lhs, rhs);
                }
                _ => return Ok(lhs),
            }
        }
    }

    pub fn mul(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        let mut lhs = Self::unary(it)?;

        loop {
            match it.peek().map(|vt| vt.value.to_owned()) {
                Some(TokenKind::Asterisk) | Some(TokenKind::Slash) | Some(TokenKind::Percent) => {
                    let op = match it.next().unwrap() {
                        Token {
                            value: TokenKind::Asterisk,
                            loc,
                        } => Node::mul(loc.to_owned()),
                        Token {
                            value: TokenKind::Slash,
                            loc,
                        } => Node::div(loc.to_owned()),
                        Token {
                            value: TokenKind::Percent,
                            loc,
                        } => Node::surplus(loc.to_owned()),

                        _ => unreachable!(),
                    };
                    let rhs = Self::unary(&mut it)?;

                    lhs = Self::new_nds(op, lhs, rhs);
                }
                _ => return Ok(lhs),
            }
        }
    }

    pub fn unary(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        match it.peek().map(|vt| vt.value.to_owned()) {
            Some(TokenKind::LParen) => {
                let rhs = match it.next().unwrap() {
                    Token {
                        value: TokenKind::LParen,
                        loc: _,
                    } => Self::cmp(&mut it)?,
                    _ => unreachable!(),
                };
                if it.peek() == None {
                    return Err(ParseError::Eof);
                }
                match it.next().unwrap() {
                    Token {
                        value: TokenKind::RParen,
                        loc: _,
                    } => return Ok(rhs),
                    _ => return Err(ParseError::NotClosedParen(it.next().unwrap().to_owned())),
                }
            }
            _ => return Self::primary(it),
        }
    }

    pub fn primary(
        it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        if it.peek() == None {
            return Err(ParseError::Eof);
        }

        Ok(Self::new_num(it.next().unwrap().to_owned())?)
    }
}
