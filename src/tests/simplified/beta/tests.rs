#[cfg(test)]
use super::super::super::super::location::location::*;
#[cfg(test)]
use super::super::super::super::node::node::*;
#[cfg(test)]
use super::super::super::super::simplified::beta::*;
#[cfg(test)]
use super::super::super::super::token::token::*;

#[test]
fn fixme_simplified_beta_test() {
    let t = Token::tokenize("2*(2-1)+5").unwrap();
    let n = NodeSt::parser(t).unwrap();
    let n = beta(n);
    let l = NodeSt {
        c: Node::number(7, Loc::new(0, 0)), // FIXME:
        lhs: None,
        rhs: None,
    };
    assert_eq!(l, n);
}
