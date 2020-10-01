#[cfg(test)]
use super::super::location::location::*;
#[cfg(test)]
use super::super::node::node::*;
#[cfg(test)]
use super::super::token::token::*;
#[cfg(test)]
use super::error::*;

#[test]
fn parser_test() {
    let t = Token::tokenize("12+3").unwrap();
    let l = NodeSt::parser(t).unwrap();
    let e = {
        NodeSt {
            c: Node::plus(Loc::new(2, 3)),
            lhs: Some(Box::new(NodeSt {
                c: Node::number(12, Loc::new(0, 2)),
                lhs: None,
                rhs: None,
            })),
            rhs: Some(Box::new(NodeSt {
                c: Node::number(3, Loc::new(3, 4)),
                lhs: None,
                rhs: None,
            })),
        }
    };
    assert_eq!(e, l)
}

#[test]
fn not_exit_when_failed_parser_test() {
    let t = Token::tokenize("+3").unwrap();
    let l = match NodeSt::parser(t) {
        Ok(_) => false,
        Err(_) => true,
    };
    assert!(l)
}

#[test]
fn reached_at_eof_test() {
    let t = Token::tokenize("5+").unwrap();
    let l = match NodeSt::parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::Eof => true,
            _ => false,
        },
    };
    assert!(l)
}
