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
            c: Annot {
                value: NodeKind::Num(n),
                loc: t.loc,
            },
            ..Default::default()
        })
    }
}

impl NodeSt {
    pub fn parser(vt: Vec<Token>) -> Result<Self, ParseError> {
        let mut it = vt.iter().peekable();
        let mut lhs = Self::new_num(it.next().unwrap().to_owned())?;
        let mut _c = Node::default();

        while it.peek() != None {
            let t = it.next().unwrap().to_owned();

            println!("t: {:?}", t);
            _c = match t.value {
                TokenKind::Plus => Annot {
                    value: NodeKind::Add,
                    loc: t.loc,
                },
                TokenKind::Minus => Annot {
                    value: NodeKind::Sub,
                    loc: t.loc,
                },
                _ => return Err(ParseError::NotOperator(t)),
            };

            if it.peek() == None {
                return Err(ParseError::Eof);
            }

            let n = Self::primary(&mut it).unwrap();

            lhs = Self::new_nds(_c, Box::new(lhs), Box::new(n));
        }
        Ok(lhs)
    }

    pub fn primary(
        it: &mut std::iter::Peekable<std::slice::Iter<'_, Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        Ok(Self::new_num(it.next().unwrap().to_owned())?)
    }
}
