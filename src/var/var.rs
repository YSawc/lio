use super::super::node::node::*;
use super::super::node_arr::node_arr::*;

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

pub fn vex(ns: &mut NodeSt, vv: Vec<Var>) -> NodeSt {
    if ns.lhs != None {
        let l = Some(Box::new(vex(
            &mut ns.lhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
            vv.to_owned(),
        )));
        if ns.lhs != l {
            ns.lhs = l;
        };
    }

    if ns.rhs != None {
        let r = Some(Box::new(vex(
            &mut ns.rhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
            vv.to_owned(),
        )));
        if ns.rhs != r {
            ns.rhs = r;
        };
    }

    match ns.c.value.to_owned() {
        NodeKind::Ident(s) => {
            let n = NodeArr::find_l(s, vv.to_owned()).unwrap();
            // println!("n.n.c : {:?}", n.n.c);
            ns.c = n.n.c;
            // println!("ns.c: {:?}", ns.c);
            if n.n.lhs != None {
                ns.lhs = Some(Box::new(vex(
                    &mut n.n.lhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
                    vv.to_owned(),
                )));
            }
            if n.n.rhs != None {
                ns.rhs = Some(Box::new(vex(
                    &mut n.n.rhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
                    vv.to_owned(),
                )));
            }
            return ns.to_owned();
        }
        _ => return ns.to_owned(),
    };
}
