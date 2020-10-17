use super::super::node::node::*;
use super::super::var::var::*;
use rustc_hash::FxHashMap;

pub fn beta(ns: &mut NodeSt, ev: Vec<Vec<Var>>, uv: &mut Vec<String>) -> NodeSt {
    let mut map: FxHashMap<String, Var> = FxHashMap::default();
    vex(ns, ev, uv, &mut map)
}

pub fn find_map(s: String, m: &mut FxHashMap<String, Var>) -> Option<Var> {
    match m.get(&s.to_string()) {
        Some(v) => Some(v.to_owned()),
        None => None,
    }
}
