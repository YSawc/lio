use super::super::location::location::*;
use super::super::node_arr::node_arr::*;

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
    Nill,
    Ident(String),
    Assign,
    NewAssign,
    NewAssignG,
    NewAssignL,
    If,
    Else,
    While,
    Fn,
    LBrace,
    RBrace,
    Pipe,
    UnderScore,
    LTouple,
    NewVar(i32),
    ReAssignVar(i32),
    GVar(String),
    LVar(i32),
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
    pub ret_set: Option<Box<ReturnSet>>,
    pub lhs: Option<Box<NodeSt>>,
    pub rhs: Option<Box<NodeSt>>,
    pub cond: Option<Box<NodeSt>>,
    pub if_stmts: Option<Box<NodeArr>>,
    pub else_if_stmts: Option<Box<NodeArr>>,
    pub stmts: Option<Box<NodeArr>>,
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
    pub fn nill(loc: Loc) -> Self {
        Self::new(NodeKind::Nill, loc)
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
    pub fn new_assign_l(loc: Loc) -> Self {
        Self::new(NodeKind::NewAssignL, loc)
    }
    pub fn if_(loc: Loc) -> Self {
        Self::new(NodeKind::If, loc)
    }
    pub fn else_(loc: Loc) -> Self {
        Self::new(NodeKind::Else, loc)
    }
    pub fn while_(loc: Loc) -> Self {
        Self::new(NodeKind::While, loc)
    }
    pub fn st(loc: Loc) -> Self {
        Self::new(NodeKind::LBrace, loc)
    }
    pub fn pipe(loc: Loc) -> Self {
        Self::new(NodeKind::Pipe, loc)
    }
    pub fn fn_(loc: Loc) -> Self {
        Self::new(NodeKind::Fn, loc)
    }
    pub fn under_score(loc: Loc) -> Self {
        Self::new(NodeKind::UnderScore, loc)
    }
    pub fn l_touple(loc: Loc) -> Self {
        Self::new(NodeKind::LTouple, loc)
    }
    pub fn new_var(i: i32, loc: Loc) -> Self {
        Self::new(NodeKind::NewVar(i), loc)
    }
    pub fn re_assign_var(i: i32, loc: Loc) -> Self {
        Self::new(NodeKind::ReAssignVar(i), loc)
    }
    pub fn g_var(s: String, loc: Loc) -> Self {
        Self::new(NodeKind::GVar(s), loc)
    }
    pub fn l_var(i: i32, loc: Loc) -> Self {
        Self::new(NodeKind::LVar(i), loc)
    }
}

impl NodeSt {
    pub fn num(n: i8, loc: Loc) -> Self {
        NodeSt {
            c: Node::num(n, loc),
            ..Default::default()
        }
    }
    pub fn under_score(loc: Loc) -> Self {
        NodeSt {
            c: Node::under_score(loc),
            ..Default::default()
        }
    }
    pub fn ass_var(i: i32, nst: NodeSt, loc: Loc) -> Self {
        NodeSt {
            c: Node::new_var(i, loc),
            rhs: Some(Box::new(nst)),
            ..Default::default()
        }
    }
    pub fn r_var(i: i32, nst: NodeSt, loc: Loc) -> Self {
        NodeSt {
            c: Node::re_assign_var(i, loc),
            rhs: Some(Box::new(nst)),
            ..Default::default()
        }
    }
    pub fn g_var(s: String, loc: Loc) -> Self {
        NodeSt {
            c: Node::g_var(s, loc),
            ..Default::default()
        }
    }
    pub fn l_var(i: i32, loc: Loc) -> Self {
        NodeSt {
            c: Node::l_var(i, loc),
            ..Default::default()
        }
    }
}

impl NodeSt {
    pub fn get_num(&self) -> i8 {
        match self.c.value {
            NodeKind::Num(i) => i,
            _ => unimplemented!(),
        }
    }

    pub fn isi(&mut self) -> bool {
        // println!("self.c: {:?}", self.c);
        match self.c.value {
            NodeKind::Num(_)
            | NodeKind::Add
            | NodeKind::Sub
            | NodeKind::Mul
            | NodeKind::Div
            | NodeKind::E
            | NodeKind::NE
            | NodeKind::L
            | NodeKind::LE
            | NodeKind::G
            | NodeKind::GE
            | NodeKind::Ident(_)
            | NodeKind::GVar(_)
            | NodeKind::LVar(_) => true,
            NodeKind::If => self
                .if_stmts
                .as_ref()
                .unwrap()
                .ret_nodes
                .first()
                .unwrap()
                .to_owned()
                .isi(),
            _ => false,
        }
    }
}

pub fn isi_all(nds: Vec<NodeSt>) -> bool {
    for mut i in nds {
        if !NodeSt::isi(&mut i) {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReturnSetKind {
    Single,
    Touple,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnSet {
    pub ty: ReturnSetKind,
    pub contents: Vec<String>,
}

impl ReturnSet {
    pub fn new_single(s: String) -> Self {
        let contents = vec![s];

        Self {
            ty: ReturnSetKind::Single,
            contents,
        }
    }

    pub fn new_touple(contents: Vec<String>) -> Self {
        Self {
            ty: ReturnSetKind::Single,
            contents,
        }
    }
}
