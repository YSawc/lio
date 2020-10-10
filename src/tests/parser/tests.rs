#[cfg(test)]
use super::super::super::location::location::*;
#[cfg(test)]
use super::super::super::node::node::*;
#[cfg(test)]
use super::super::super::node_arr::node_arr::*;
#[cfg(test)]
use super::super::super::parser::error::*;
#[cfg(test)]
use super::super::super::token::token::*;

#[test]
fn parser_test() {
    let t = Token::tokenize("12+3;").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
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
fn evaluation_final_value_test() {
    let t = Token::tokenize("12+3").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
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
fn parser_assign_test() {
    let t = Token::tokenize("int a = 3; 1").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
    let e = {
        NodeSt {
            c: Node::number(1, Loc::new(14, 15)),
            lhs: None,
            rhs: None,
        }
    };
    assert_eq!(e, l)
}

#[test]
fn return_with_unclosed_test() {
    let t = Token::tokenize("return 3").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotClosedStmt(_) => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn operater_after_return_test() {
    let t = Token::tokenize("return 3; 4").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::OperatorAfterRetrun(_) => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn not_exit_when_failed_parser_test() {
    let t = Token::tokenize("+3").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(_) => true,
    };
    assert!(l)
}

#[test]
fn reached_at_eof_test() {
    let t = Token::tokenize("5+").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::Eof => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn unclosed_eof_test() {
    let t = Token::tokenize("5+(3+2*2").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::Eof => true,
            _ => false,
        },
    };
    assert!(l)
}
