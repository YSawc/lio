use super::super::location::location::*;
use super::super::node::node::*;
use super::super::token::token::*;
use super::error::*;

impl NodeSt {
    pub fn new_nds(c: Node, lhs: Box<NodeSt>, rhs: Box<NodeSt>) -> Self {
        Self {
            c,
            lhs: Some(lhs),
            rhs: Some(rhs),
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
        let mut lhs = Self::new_num(it.next().unwrap().to_owned())?;

        while it.peek() != None {
            lhs = Self::expr(&mut it, &mut lhs)?;
        }
        Ok(lhs)
    }

    pub fn expr(
        mut it: &mut std::iter::Peekable<std::slice::Iter<'_, Annot<TokenKind>>>,
        lhs: &mut NodeSt,
    ) -> Result<NodeSt, ParseError> {
        let t = it.next().unwrap().to_owned();
        let c = match t.value {
            TokenKind::Plus => Annot::new(NodeKind::Add, t.loc),
            TokenKind::Minus => Annot::new(NodeKind::Sub, t.loc),
            _ => return Err(ParseError::NotOperator(t.to_owned())),
        };

        if it.peek() == None {
            return Err(ParseError::Eof);
        }

        let n = Self::primary(&mut it).unwrap();

        *lhs = Self::new_nds(c, Box::new(lhs.to_owned()), Box::new(n));

        Ok(lhs.to_owned())
    }

    pub fn primary(
        it: &mut std::iter::Peekable<std::slice::Iter<'_, Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        Ok(Self::new_num(it.next().unwrap().to_owned())?)
    }
}
