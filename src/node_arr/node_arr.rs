use super::super::node::node::*;
use super::super::parser::error::*;
use super::super::simplified::*;
use super::super::token::token::*;
use super::super::var::var::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeArr {
    pub node_st_vec: Vec<NodeSt>,
    pub ret_node_st: NodeSt,
    pub l: Vec<Var>,
}

impl NodeArr {
    pub fn find_l(s: String, vv: Vec<Var>) -> Option<Var> {
        for v in vv {
            if v.s == s {
                return Some(v);
            }
        }
        None
    }
}

impl NodeArr {
    pub fn new(v: Vec<NodeSt>) -> Self {
        Self {
            node_st_vec: v.to_owned(),
            ret_node_st: v.to_owned().last().unwrap().to_owned(),
            l: vec![],
        }
    }

    pub fn w_parser(vt: Vec<Token>) -> Result<Self, ParseError> {
        let mut vti = vt.iter().peekable();
        let mut nv = vec![];
        let mut l: Vec<Var> = vec![];

        while vti.peek() != None {
            nv.push(match NodeSt::parser(&mut vti) {
                Ok(n) => match n.c.value {
                    NodeKind::Return => {
                        if vti.peek() != None {
                            return Err(ParseError::OperatorAfterRetrun(
                                vti.next().unwrap().to_owned(),
                            ));
                        }
                        n
                    }
                    NodeKind::Assign => {
                        let s = match n.to_owned().lhs.unwrap().lhs.unwrap().c.value {
                            NodeKind::Ident(s) => s,
                            _ => unreachable!(),
                        };
                        let mut n = vex(&mut n.to_owned().rhs.unwrap().to_owned(), l.to_owned());
                        n = simplified::exec(n);
                        let v = Var::new(s, n);
                        l.push(v);
                        continue;
                    }
                    NodeKind::Ident(s) => match Self::find_l(s, l.to_owned()) {
                        Some(v) => v.n,
                        None => {
                            return Err(ParseError::NotDefinitionVar(
                                vti.next().unwrap().to_owned(),
                            ))
                        }
                    },
                    _ => {
                        let n = vex(&mut n.to_owned(), l.to_owned());
                        // println!("_n : {:?}", _n);
                        n
                    }
                },
                Err(e) => return Err(e),
            });
        }

        let mut a = Self::new(nv);
        a.l = l;
        Ok(a)
    }
}
