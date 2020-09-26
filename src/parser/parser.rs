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
        let ps = 0;
        let lhs = Self::new_num(vt[ps].to_owned());
        lhs
    }
}



// pub fn parser(vt: Vec<Token>) -> Vec<Box<NodeSt>> {
//     let mut nv: Vec<Box<NodeSt>> = Vec::new();
//     // let mut lhs: Option<Box<NodeSt>> = None;
//     let mut _lhs: Option<Box<NodeSt>>;
//     let mut rhs: Option<NodeSt> = None;

//     for t in vt {
//         match t.to_owned().value {
//             TokenKind::Plus => unimplemented!(),
//             TokenKind::Minus => unimplemented!(),
//             TokenKind::Num(_) => { _lhs = Some(new_num(1)); ()},
//             _ => unimplemented!(),
//         }
//     }

//     println!("{:?}", _lhs);

//     nv
// }

// fn mul(t: Token) -> Box<NodeSt> {
//     let n: Box<NodeSt> = unary(t.to_owned());

//     match t.to_owned().value {
//         TokenKind::Asterisk => new_nds(
//             {
//                 Annot {
//                     value: NodeKind::Mul,
//                     loc: t.to_owned().loc,
//                 }
//             },
//             n,
//             unary(t.to_owned()),
//         ),
//         TokenKind::Slash => new_nds(
//             {
//                 Annot {
//                     value: NodeKind::Div,
//                     loc: t.to_owned().loc,
//                 }
//             },
//             n,
//             unary(t.to_owned()),
//         ),
//         _ => unimplemented!(),
//     }
// }

// fn unary(t: Token) -> Box<NodeSt> {
//     let n: Box<NodeSt> = idx(t.to_owned());
//     match t.to_owned().value {
//         TokenKind::Plus => new_nds(
//             {
//                 Annot {
//                     value: NodeKind::Add,
//                     loc: t.to_owned().loc,
//                 }
//             },
//             n,
//             unary(t.to_owned()),
//         ),
//         TokenKind::Minus => new_nds(
//             {
//                 Annot {
//                     value: NodeKind::Sub,
//                     loc: t.to_owned().loc,
//                 }
//             },
//             n,
//             unary(t.to_owned()),
//         ),

//         _ => unimplemented!(),
//     }
// }

// fn idx(t: Token) -> Box<NodeSt> {
//     match t.value {
//         TokenKind::Num(_) => new_num(0),
//         _ => unimplemented!(),
//     }
// }

// fn new_nds(c: Node, lhs: Box<NodeSt>, rhs: Box<NodeSt>) -> Box<NodeSt> {
//     Box::new(NodeSt::Nodes { c, lhs, rhs })
// }

// fn new_num(val: u8) -> Box<NodeSt> {
//     Box::new(NodeSt::Val { val })
// }
