use super::super::node::node::*;
use super::super::node_arr::node_arr::*;
use super::super::parser::error::*;
use super::super::var::var::*;
use rustc_hash::FxHashMap;

pub fn beta(ns: &mut NodeSt, a: &mut NodeArr) -> Result<NodeSt, ParseError> {
    let mut map: FxHashMap<String, Var> = FxHashMap::default();
    Ok(vex(ns, a, &mut map)?)
}

pub fn find_map(s: String, m: &mut FxHashMap<String, Var>) -> Option<Var> {
    match m.get(&s.to_string()) {
        Some(v) => Some(v.to_owned()),
        None => None,
    }
}
