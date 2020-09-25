use super::super::location::location::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Num,
    Add,
    Sub,
    Mul,
    Div,
    Nul,
}

pub type Node = Annot<NodeKind>;

pub enum NodeSt {
    Nodes {
        c: Node,
        rhs: Box<NodeSt>,
        lhs: Box<NodeSt>,
    },
    Val {
        val: u8,
    }
}
