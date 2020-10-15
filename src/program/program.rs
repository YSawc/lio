// use super::super::location::location::*;
// use super::super::node::node::*;
use super::super::node_arr::node_arr::*;
use super::super::parser::error::*;
// use super::super::parser::parser::*;
// use super::super::simplified::*;
use super::super::token::token::*;
use super::super::var::var::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    pub g: Vec<Var>,
    pub na: Vec<NodeArr>,
}

impl Program {
    pub fn new(g: Vec<Var>, na: Vec<NodeArr>) -> Self {
        Self { g, na }
    }
}

impl Program {
    pub fn find_v(s: String, g: Vec<Var>, l: Vec<Var>) -> Option<Var> {
        for v in g {
            if v.s == s {
                return Some(v);
            }
        }

        for v in l {
            if v.s == s {
                return Some(v);
            }
        }

        None
    }

    pub fn w_parser(vt: Vec<Token>) -> Result<Self, ParseError> {
        let g: Vec<Var> = vec![];
        let mut na: Vec<NodeArr> = vec![];
        let mut it = vt.iter().peekable();
        na.push(NodeArr::w_parser(&mut it)?);
        Ok(Self::new(g, na))
    }
}
