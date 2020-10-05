use super::super::location::location::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Num(i8),
    Add,
    Sub,
    Mul,
    Div,
    Nul,
    Sur,
    E,
    NE,
    RE,
    RT,
    GE,
    GT,
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
    pub fn number(n: i8, loc: Loc) -> Self {
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

    pub fn eq(loc: Loc) -> Self {
        Self::new(NodeKind::E, loc)
    }
    pub fn neq(loc: Loc) -> Self {
        Self::new(NodeKind::NE, loc)
    }
    pub fn rt(loc: Loc) -> Self {
        Self::new(NodeKind::RT, loc)
    }
    pub fn re(loc: Loc) -> Self {
        Self::new(NodeKind::RE, loc)
    }
    pub fn gt(loc: Loc) -> Self {
        Self::new(NodeKind::GT, loc)
    }
    pub fn ge(loc: Loc) -> Self {
        Self::new(NodeKind::GE, loc)
    }
}

impl NodeSt {
    pub fn number(n: i8, loc: Loc) -> Self {
        NodeSt {
            c: Node::number(n, loc),
            lhs: None,
            rhs: None,
        }
    }
}
