use super::super::location::location::*;
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

    pub fn new_num(mut t: Token) -> Self {
        Self {
            c: Annot {
                value: NodeKind::Num(Token::get_val(&mut t)),
                loc: t.loc,
            },
            ..Default::default()
        }
    }

    pub fn parser(vt: Vec<Token>) -> Self {
        let mut ps = 0;
        let mut lhs = Self::new_num(vt[ps].to_owned());
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
                _ => unimplemented!(),
            };
            ps += 1;

            let n = Self::new_num(vt[ps].to_owned());
            ps += 1;

            lhs = Self::new_nds(_c, Box::new(lhs), Box::new(n));
        }
        lhs
    }
}
