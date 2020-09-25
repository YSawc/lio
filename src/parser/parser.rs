use super::super::location::location::*;
use super::super::node::node::*;
use super::super::token::token::*;

fn mul(t: &Token) -> Box<NodeSt> {
    let n: Box<NodeSt> = unary(t.to_owned());

    match t.to_owned().value {
        TokenKind::Asterisk => new_nds(
            {
                Annot {
                    value: NodeKind::Mul,
                    loc: t.to_owned().loc,
                }
            },
            n,
            unary(t.to_owned()),
        ),
        TokenKind::Slash => new_nds(
            {
                Annot {
                    value: NodeKind::Div,
                    loc: t.to_owned().loc,
                }
            },
            n,
            unary(t.to_owned()),
        ),
        _ => unimplemented!(),
    }
}

fn unary(t: Token) -> Box<NodeSt> {
    let n: Box<NodeSt> = idx(t.to_owned());
    match t.to_owned().value {
        TokenKind::Plus => new_nds(
            {
                Annot {
                    value: NodeKind::Add,
                    loc: t.to_owned().loc,
                }
            },
            n,
            unary(t.to_owned()),
        ),
        TokenKind::Minus => new_nds(
            {
                Annot {
                    value: NodeKind::Sub,
                    loc: t.to_owned().loc,
                }
            },
            n,
            unary(t.to_owned()),
        ),

        _ => unimplemented!(),
    }
}

fn idx(t: Token) -> Box<NodeSt> {
    match t.value {
        TokenKind::Num(_) => new_num(0),
        _ => unimplemented!(),
    }
}

fn new_nds(c: Node, lhs: Box<NodeSt>, rhs: Box<NodeSt>) -> Box<NodeSt> {
    Box::new(NodeSt::Nodes { c, lhs, rhs })
}

fn new_num(val: u8) -> Box<NodeSt> {
    Box::new(NodeSt::Val { val })
}
