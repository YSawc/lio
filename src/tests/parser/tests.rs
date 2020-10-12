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
#[cfg(test)]
use super::super::super::var::var::*;

#[test]
fn parser_test() {
    let t = Token::tokenize("fn { 12+3; }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
    let e = {
        NodeSt {
            c: Node::plus(Loc::new(9, 10)),
            lhs: Some(Box::new(NodeSt::num(12, Loc::new(7, 9)))),
            rhs: Some(Box::new(NodeSt::num(3, Loc::new(10, 11)))),
            ..Default::default()
        }
    };
    assert_eq!(e, l)
}

#[test]
fn evaluation_final_value_test() {
    let t = Token::tokenize("fn { 12+3 }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
    let e = {
        NodeSt {
            c: Node::plus(Loc::new(9, 10)),
            lhs: Some(Box::new(NodeSt::num(12, Loc::new(7, 9)))),
            rhs: Some(Box::new(NodeSt::num(3, Loc::new(10, 11)))),
            ..Default::default()
        }
    };
    assert_eq!(e, l)
}

#[test]
fn parser_assign_test() {
    let t = Token::tokenize("fn { int a = 3; 1 }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
    let e = { NodeSt::num(1, Loc::new(19, 20)) };
    assert_eq!(e, l)
}

#[test]
fn variable_expansion_test() {
    let t = Token::tokenize("fn { int a = 3; int b = 5; b*a; }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
    let e = NodeSt {
        c: Node::mul(Loc::new(28, 29)),
        lhs: Some(Box::new(NodeSt::num(5, Loc::new(26, 27)))),
        rhs: Some(Box::new(NodeSt::num(3, Loc::new(15, 16)))),
        ..Default::default()
    };

    assert_eq!(e, l)
}

#[test]
fn return_with_unclosed_test() {
    let t = Token::tokenize("fn { return 3 }").unwrap();
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
    let t = Token::tokenize("fn { return 3; 4 }").unwrap();
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
    let t = Token::tokenize("fn { +3 }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(_) => true,
    };
    assert!(l)
}

#[test]
fn not_number_test() {
    let t = Token::tokenize("fn { 5+ }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotNumber(_) => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn unclosed_paren_test() {
    let t = Token::tokenize("fn { 5+(3+2*2 }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotClosedParen(_) => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn update_variable_test() {
    let t = Token::tokenize("fn { int a = 3; int b = a; int c = 5; a = 1; 0 }").unwrap();
    let l = NodeArr::w_parser(t).unwrap();
    l.to_owned().l;
    let mut e: Vec<Var> = vec![];
    e.push(Var::new("b".to_string(), NodeSt::num(3, Loc::new(15, 16))));
    e.push(Var::new("c".to_string(), NodeSt::num(5, Loc::new(37, 38))));
    e.push(Var::new("a".to_string(), NodeSt::num(1, Loc::new(44, 45))));
    assert_eq!(e, l.to_owned().l)
}

#[test]
fn operator_out_of_function_test() {
    let t = Token::tokenize("0; fn { 12+3; }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::OperatorOutOfFnction(_) => true,
            _ => false,
        },
    };
    assert!(l)
}
