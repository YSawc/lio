#[cfg(test)]
use super::super::super::super::location::location::*;
#[cfg(test)]
use super::super::super::super::node::node::*;
#[cfg(test)]
use super::super::super::super::node_arr::node_arr::*;
#[cfg(test)]
use super::super::super::super::simplified::simplified::*;
#[cfg(test)]
use super::super::super::super::token::token::*;

#[test]
fn simplified_test() {
    let t = Token::tokenize("fn int { 2*(2-1)+5; }").unwrap();
    let mut t = t.iter().peekable();
    let n = NodeArr::w_parser(&mut t, vec![]).unwrap().0;
    let n = simplified(n).ret_node_st;
    let l = NodeSt::num(7, Loc::new(12, 13));
    assert_eq!(l, n);
}

#[test]
fn simplified_with_minus_test() {
    let t = Token::tokenize("fn int { 2*(2-4)+5; }").unwrap();
    let mut t = t.iter().peekable();
    let n = NodeArr::w_parser(&mut t, vec![]).unwrap().0;
    let n = simplified(n).ret_node_st;
    let l = NodeSt::num(1, Loc::new(12, 13));
    assert_eq!(l, n);
}
