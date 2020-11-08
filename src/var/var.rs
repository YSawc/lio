use super::super::node::node::*;
use super::super::node_arr::node_arr::*;
use super::super::program::program::*;
use super::super::simplified::beta::*;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var {
    pub s: String,
    pub n: NodeSt,
    pub m: bool,
    pub aln: i32,
    pub gf: bool,
}

impl Var {
    pub fn new_l(s: String, n: NodeSt, aln: i32) -> Self {
        Self {
            s,
            n,
            m: false,
            aln,
            gf: false,
        }
    }

    pub fn new_g(s: String, n: NodeSt, aln: i32) -> Self {
        Self {
            s,
            n,
            m: false,
            aln,
            gf: true,
        }
    }

    pub fn default() -> Self {
        Self {
            s: String::new(),
            n: NodeSt::default(),
            m: false,
            aln: 0,
            gf: false,
        }
    }

    pub fn gmnew(s: String, n: NodeSt, aln: i32) -> Self {
        Self {
            s,
            n,
            m: true,
            aln,
            gf: true,
        }
    }

    pub fn mnew(s: String, n: NodeSt, aln: i32) -> Self {
        Self {
            s,
            n,
            m: true,
            aln,
            gf: false,
        }
    }
}

pub fn vex(ns: &mut NodeSt, a: &mut NodeArr, map: &mut FxHashMap<String, Var>) -> NodeSt {
    if ns.lhs != None {
        let l = Some(Box::new(vex(
            &mut ns.lhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
            a,
            map,
        )));
        if ns.lhs != l {
            ns.lhs = l;
        };
    }

    if ns.rhs != None {
        let r = Some(Box::new(vex(
            &mut ns.rhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
            a,
            map,
        )));
        if ns.rhs != r {
            ns.rhs = r;
        };
    }

    match ns.c.value.to_owned() {
        NodeKind::Ident(s) => {
            let n = match find_map(s.to_owned(), map) {
                Some(v) => v.to_owned(),
                None => {
                    let v = Program::find_v(s.to_owned(), a.imm_env_v.to_owned()).unwrap();
                    map.insert(s.to_owned(), v.to_owned());
                    v
                }
            };
            let mut vn = match n.gf {
                true => {
                    ns.c = Node::g_var(s.to_owned(), ns.to_owned().c.loc);
                    NodeSt::g_var(s, ns.to_owned().c.loc)
                }
                false => {
                    ns.c = Node::l_var(n.aln, ns.to_owned().c.loc);
                    NodeSt::l_var(n.aln, ns.to_owned().c.loc)
                }
            };

            if !a.used_variable.contains(&n.to_owned().s.to_owned()) {
                a.used_variable.push(n.s)
            };

            vn.lhs = ns.lhs.to_owned();
            vn.rhs = ns.rhs.to_owned();
            return ns.to_owned();
        }
        _ => return ns.to_owned(),
    };
}
