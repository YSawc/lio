use super::super::location::location::*;
use super::super::node::error::*;
use super::super::node::node::*;
use super::super::token::token::*;

impl NodeSt {
    pub fn new_nds(c: Node, lhs: Box<NodeSt>, rhs: Box<NodeSt>) -> Self {
        Self {
            c,
            lhs: Some(lhs),
            rhs: Some(rhs),
            ..Default::default()
        }
    }

    pub fn new_num(mut t: Token) -> Result<Self, NodeError> {
        let n = match Token::get_val(&mut t) {
            Ok(n) => n,
            Err(_) => return Err(NodeError::not_number(t.clone(), t.loc)),
        };

        Ok(Self {
            c: Annot {
                value: NodeKind::Num(n),
                loc: t.loc,
            },
            ..Default::default()
        })
    }

    pub fn parser(vt: Vec<Token>) -> Result<Self, NodeError> {
        let mut ps = 0;
        let mut rhs = match Self::new_num(vt[ps].to_owned()) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        let mut _c = Node::default();
        ps += 1;

        loop {
            if vt.len() <= ps {
                break;
            }

            let t = vt[ps].to_owned();
            println!("t: {:?}", t);
            _c = match t.to_owned().value {
                TokenKind::Plus => Annot {
                    value: NodeKind::Add,
                    loc: t.to_owned().loc,
                },
                TokenKind::Minus => Annot {
                    value: NodeKind::Sub,
                    loc: t.to_owned().loc,
                },
                _ => panic!(""),
            };
            ps += 1;

            let n = Self::new_num(vt[ps].to_owned()).unwrap();
            ps += 1;

            rhs = Self::new_nds(_c, Box::new(rhs), Box::new(n));
        }
        Ok(rhs)
    }
}
