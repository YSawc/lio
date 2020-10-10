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
    let t = Token::tokenize("int a = 2; int b = 8*a; int c = 2*b+a; 0;").unwrap();
    let l = NodeArr::w_parser(t).unwrap().l;
    let mut e: Vec<Var> = Vec::new();
    e.push(Var {
        s: "a".to_string(),
        n: NodeSt {
            c: Node::number(2, Loc::new(10, 11)),
            lhs: None,
            rhs: None,
        },
    });
    e.push(Var {
        s: "b".to_string(),
        n: NodeSt {
            c: Node::number(16, Loc::new(21, 23)),
            lhs: None,
            rhs: None,
        },
    });
    e.push(Var {
        s: "c".to_string(),
        n: NodeSt {
            c: Node::number(34, Loc::new(34, 38)),
            lhs: None,
            rhs: None,
        },
    });
    assert_eq!(e, l);
}
