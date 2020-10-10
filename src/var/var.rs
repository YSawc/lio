use super::super::node::node::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var {
    pub s: String,
    pub n: NodeSt,
}

impl Var {
    pub fn new(s: String, n: NodeSt) -> Self {
        Self { s, n }
    }

    pub fn default() -> Self {
        Self {
            s: String::new(),
            n: NodeSt::default(),
        }
    }
}
