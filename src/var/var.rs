use super::super::node::node::*;
// use super::super::node_arr::node_arr::*;
use super::super::program::program::*;

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

pub fn vex(ns: &mut NodeSt, ev: Vec<Vec<Var>>, uv: &mut Vec<String>) -> NodeSt {
    if ns.lhs != None {
        let l = Some(Box::new(vex(
            &mut ns.lhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
            ev.to_owned(),
            uv,
        )));
        if ns.lhs != l {
            ns.lhs = l;
        };
    }

    if ns.rhs != None {
        let r = Some(Box::new(vex(
            &mut ns.rhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
            ev.to_owned(),
            uv,
        )));
        if ns.rhs != r {
            ns.rhs = r;
        };
    }

    match ns.c.value.to_owned() {
        NodeKind::Ident(s) => {
            let n = Program::find_v(s, ev.to_owned()).unwrap();
            ns.c = n.to_owned().n.c;
            if uv.contains(&n.to_owned().s.to_owned()) {
            } else {
                uv.push(n.s);
            }
            if n.n.lhs != None {
                ns.lhs = Some(Box::new(vex(
                    &mut n.n.lhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
                    ev.to_owned(),
                    uv,
                )));
            }
            if n.n.rhs != None {
                ns.rhs = Some(Box::new(vex(
                    &mut n.n.rhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
                    ev.to_owned(),
                    uv,
                )));
            }
            return ns.to_owned();
        }
        _ => return ns.to_owned(),
    };
}
