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
    L,
    LE,
    G,
    GE,
    Return,
    Int,
    Ident(String),
    Assign,
    NewAssign,
    If,
    Else,
    Fn,
    LBrace,
    RBrace,
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
    pub cond: Option<Box<NodeSt>>,
    pub stmt: Option<Box<NodeSt>>,
    pub melse_stmt: Option<Box<NodeSt>>,
}

impl Node {
    pub fn num(n: i8, loc: Loc) -> Self {
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
    pub fn l(loc: Loc) -> Self {
        Self::new(NodeKind::L, loc)
    }
    pub fn le(loc: Loc) -> Self {
        Self::new(NodeKind::LE, loc)
    }
    pub fn g(loc: Loc) -> Self {
        Self::new(NodeKind::G, loc)
    }
    pub fn ge(loc: Loc) -> Self {
        Self::new(NodeKind::GE, loc)
    }
    pub fn ret(loc: Loc) -> Self {
        Self::new(NodeKind::Return, loc)
    }
    pub fn int(loc: Loc) -> Self {
        Self::new(NodeKind::Int, loc)
    }
    pub fn ident(s: String, loc: Loc) -> Self {
        Self::new(NodeKind::Ident(s), loc)
    }
    pub fn assign(loc: Loc) -> Self {
        Self::new(NodeKind::Assign, loc)
    }
    pub fn new_assign(loc: Loc) -> Self {
        Self::new(NodeKind::NewAssign, loc)
    }
    pub fn mif(loc: Loc) -> Self {
        Self::new(NodeKind::If, loc)
    }
    pub fn melse(loc: Loc) -> Self {
        Self::new(NodeKind::Else, loc)
    }
    pub fn mfn(loc: Loc) -> Self {
        Self::new(NodeKind::Fn, loc)
    }
}

impl NodeSt {
    pub fn num(n: i8, loc: Loc) -> Self {
        NodeSt {
            c: Node::num(n, loc),
            ..Default::default()
        }
    }
}
