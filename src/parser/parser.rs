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
        let mut lhs = Self::new_num(it.next().unwrap().to_owned())?;

        while it.peek() != None {
            lhs = Self::expr(&mut it, &mut lhs)?;
        }
        Ok(lhs)
    }

    pub fn expr(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
        lhs: &mut NodeSt,
    ) -> Result<NodeSt, ParseError> {
        *lhs = match it.peek().map(|vt| vt.value.to_owned()) {
            Some(TokenKind::Plus) => Self::new_nds(
                Annot::new(NodeKind::Add, it.next().unwrap().loc.to_owned()),
                lhs.to_owned(),
                Self::primary(&mut it)?,
            ),
            Some(TokenKind::Minus) => Self::new_nds(
                Annot::new(NodeKind::Sub, it.next().unwrap().loc.to_owned()),
                lhs.to_owned(),
                Self::primary(&mut it)?,
            ),
            _ => return Err(ParseError::NotOperator(it.next().unwrap().to_owned())),
        };

        Ok(lhs.to_owned())
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
