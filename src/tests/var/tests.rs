#[cfg(test)]
use super::super::super::location::location::*;
#[cfg(test)]
use super::super::super::node::node::*;
#[cfg(test)]
use super::super::super::node_arr::node_arr::*;
// #[cfg(test)]
// use super::super::super::parser::error::*;
#[cfg(test)]
use super::super::super::token::token::*;
#[cfg(test)]
use super::super::super::var::var::*;

#[test]
fn simplified_variable_under_initialize_test() {
    let t = Token::tokenize("fn { int a = 2; int b = 8*a; int c = 2*b+a; 0; }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().l;
    let mut e: Vec<Var> = Vec::new();
    e.push(Var {
        s: "a".to_string(),
        n: NodeSt::num(2, Loc::new(15, 16)),
    });
    e.push(Var {
        s: "b".to_string(),
        n: NodeSt::num(16, Loc::new(26, 28)),
    });
    e.push(Var {
        s: "c".to_string(),
        n: NodeSt::num(34, Loc::new(39, 43)),
    });
    assert_eq!(e, l);
}
