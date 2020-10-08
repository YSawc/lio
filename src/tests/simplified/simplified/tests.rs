#[cfg(test)]
use super::super::super::super::location::location::*;
#[cfg(test)]
use super::super::super::super::node::node::*;
#[cfg(test)]
use super::super::super::super::simplified::simplified::*;
#[cfg(test)]
use super::super::super::super::token::token::*;

#[test]
fn simplified_test() {
    let t = Token::tokenize("2*(2-1)+5;").unwrap();
    let ti = t.iter().peekable();
    let n = NodeSt::parser(ti).unwrap();
    let n = simplified(n);
    let l = NodeSt {
        c: Node::number(7, Loc::new(0, 1)),
        lhs: None,
        rhs: None,
    };
    assert_eq!(l, n);
}

#[test]
fn simplified_with_minus_test() {
    let t = Token::tokenize("2*(2-4)+5;").unwrap();
    let ti = t.iter().peekable();
    let n = NodeSt::parser(ti).unwrap();
    let n = simplified(n);
    let l = NodeSt {
        c: Node::number(1, Loc::new(0, 1)),
        lhs: None,
        rhs: None,
    };
    assert_eq!(l, n);
}