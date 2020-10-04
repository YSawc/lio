use super::super::location::location::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Num(u8),
    Add,
    Sub,
    Mul,
    Div,
    Nul,
    Sur,
    Default,
}

impl Default for NodeKind {
    fn default() -> Self {
        NodeKind::Default
    }
}

pub type Node = Annot<NodeKind>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NodeSt {
    pub c: Node,
    pub lhs: Option<Box<NodeSt>>,
    pub rhs: Option<Box<NodeSt>>,
}

impl Node {
    pub fn number(n: u8, loc: Loc) -> Self {
        Self::new(NodeKind::Num(n), loc)
    }

    pub fn plus(loc: Loc) -> Self {
        Self::new(NodeKind::Add, loc)
    }

    pub fn minus(loc: Loc) -> Self {
        Self::new(NodeKind::Sub, loc)
    }

    pub fn mul(loc: Loc) -> Self {
        Self::new(NodeKind::Mul, loc)
    }

    pub fn div(loc: Loc) -> Self {
        Self::new(NodeKind::Div, loc)
    }

    pub fn surplus(loc: Loc) -> Self {
        Self::new(NodeKind::Sur, loc)
    }
}

impl NodeSt {
    pub fn number(n: u8, loc: Loc) -> Self {
        NodeSt {
            c: Node::number(n, loc),
                lhs: None,
                rhs: None,
        }
    }
}
