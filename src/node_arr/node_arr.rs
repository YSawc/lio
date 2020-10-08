use super::super::node::node::*;
use super::super::parser::error::*;
use super::super::token::token::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeArr {
    pub node_st_vec: Vec<NodeSt>,
    pub ret_node_st: NodeSt,
}

impl NodeArr {
    pub fn new(v: Vec<NodeSt>) -> Self {
        Self {
            node_st_vec: vec![],
            ret_node_st: v.last().unwrap().to_owned(),
        }
    }

    pub fn w_parser(vt: Vec<Token>) -> Result<Self, ParseError> {
        let mut vti = vt.iter().peekable();
        let mut n = vec![];

        while vti.peek() == None {
            n.push(match NodeSt::parser(vti.to_owned()) {
                Ok(n) => n,
                Err(e) => return Err(e),
            });
        }

        let a = Self::new(n);
        Ok(a)
    }
}
