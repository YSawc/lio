use super::super::node::node::*;
use super::super::program::program::*;
use super::super::simplified::beta::*;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var {
    pub s: String,
    pub n: NodeSt,
    pub m: bool,
    pub aln: i32,
}

impl Var {
    pub fn new(s: String, n: NodeSt, aln: i32) -> Self {
        Self {
            s,
            n,
            m: false,
            aln,
        }
    }

    pub fn default() -> Self {
        Self {
            s: String::new(),
            n: NodeSt::default(),
            m: false,
            aln: 0,
        }
    }

    pub fn mnew(s: String, n: NodeSt, aln: i32) -> Self {
        Self { s, n, m: true, aln }
    }
}

pub fn vex(
    ns: &mut NodeSt,
    ev: Vec<Vec<Var>>,
    uv: &mut Vec<String>,
    map: &mut FxHashMap<String, Var>,
) -> NodeSt {
    if ns.lhs != None {
        let l = Some(Box::new(vex(
            &mut ns.lhs.as_ref().unwrap().to_owned().as_ref().to_owned(),
            ev.to_owned(),
            uv,
            map,
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
                    let v = Program::find_v(s.to_owned(), ev.to_owned()).unwrap();
                    map.insert(s, v.to_owned());
                    v
                }
            };
            ns.c = Node::var(n.aln, ns.to_owned().c.loc);
            if uv.contains(&n.to_owned().s.to_owned()) {
            } else {
                uv.push(n.s);
            }
            let mut vn = NodeSt::var(n.aln, ns.to_owned().c.loc);
            vn.lhs = ns.lhs.to_owned();
            vn.rhs = ns.rhs.to_owned();
            return ns.to_owned();
        }
        _ => return ns.to_owned(),
    };
}
