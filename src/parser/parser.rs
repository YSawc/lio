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
        let mut _c = Self::default();
        ps += 1;

        loop {
            if vt.len() < ps {
                break;
            }

            let mut t = vt[ps-1].to_owned();
            println!("{:?}", t);
            _c = match t.to_owned().value {
                TokenKind::Asterisk => Self {
                    c: Annot {
                        value: NodeKind::Mul,
                        loc: t.to_owned().loc,
                    },
                    ..Default::default()
                },
                TokenKind::Slash => Self {
                    c: Annot {
                        value: NodeKind::Div,
                        loc: t.to_owned().loc,
                    },
                    ..Default::default()
                },
                TokenKind::Plus => Self {
                    c: Annot {
                        value: NodeKind::Add,
                        loc: t.to_owned().loc,
                    },
                    ..Default::default()
                },
                TokenKind::Minus => Self {
                    c: Annot {
                        value: NodeKind::Sub,
                        loc: t.to_owned().loc,
                    },
                    ..Default::default()
                },
                TokenKind::Num(_) => Self {
                    c: Annot {
                        value: NodeKind::Num(Token::get_val(&mut t)),
                        loc: t.to_owned().loc,
                    },
                    ..Default::default()
                },
                _ => unimplemented!(),
            };

            ps += 1;
            lhs = Self::new_nds( Annot { value: NodeKind::default(), loc: Loc::default() }, Box::new(_c), Box::new(NodeSt::default())  );
        }
        lhs
    }
}
