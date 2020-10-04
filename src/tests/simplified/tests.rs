#[cfg(test)]
use super::super::super::location::location::*;
#[cfg(test)]
use super::super::super::node::node::*;
#[cfg(test)]
use super::super::super::simplified::beta::*;
#[cfg(test)]
use super::super::super::token::token::*;

#[test]
fn fixme_simplified_beta_test() {
    let t = Token::tokenize("5+3").unwrap();
    let mut n = NodeSt::parser(t).unwrap();
    beta(&mut n);
    let l = NodeSt {
        c: Node::number(8, Loc::new(1, 2)), // FIXME:
        lhs: None,
        rhs: None,
    };
    assert_eq!(l, n);
}
