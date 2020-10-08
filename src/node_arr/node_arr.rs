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
            node_st_vec: v.to_owned(),
            ret_node_st: v.to_owned().last().unwrap().to_owned(),
        }
    }

    pub fn w_parser(vt: Vec<Token>) -> Result<Self, ParseError> {
        let mut vti = vt.iter().peekable();
        let mut n = vec![];

        while vti.peek() != None {
            n.push(match NodeSt::parser(&mut vti) {
                Ok(n) => {
                    // println!("n: {:?}", n);
                    if n.c.value == NodeKind::Return && vti.peek() != None {
                        return Err(ParseError::OperatorAfterRetrun(
                            vti.next().unwrap().to_owned(),
                        ));
                    }
                    n
                }
                Err(e) => return Err(e),
            });
        }

        let a = Self::new(n);
        Ok(a)
    }
}
