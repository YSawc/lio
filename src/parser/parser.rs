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
        let lhs = Self::expr(&mut it)?;
        Ok(lhs)
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
        let mut lhs = Self::primary(it)?;

        loop {
            match it.peek().map(|vt| vt.value.to_owned()) {
                Some(TokenKind::Asterisk) | Some(TokenKind::Slash) => {
                    let op = match it.next().unwrap() {
                        Token {
                            value: TokenKind::Asterisk,
                            loc,
                        } => Node::mul(loc.to_owned()),
                        Token {
                            value: TokenKind::Slash,
                            loc,
                        } => Node::div(loc.to_owned()),
                        _ => unreachable!(),
                    };
                    let rhs = Self::primary(&mut it)?;

                    lhs = Self::new_nds(op, lhs, rhs);
                }
                _ => return Ok(lhs),
            }
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
