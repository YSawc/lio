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
    let t = Token::tokenize("fn int { 12+3; }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
    let e = {
        NodeSt {
            c: Node::plus(Loc::new(14, 15)),
            lhs: Some(l.to_owned().lhs.unwrap()),
            rhs: Some(l.to_owned().rhs.unwrap()),
            ..Default::default()
        }
    };
    assert_eq!(e, l)
}

#[test]
fn evaluation_final_value_test() {
    let t = Token::tokenize("fn int { 12+3 }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
    let e = {
        NodeSt {
            c: Node::plus(Loc::new(14, 15)),
            lhs: Some(l.to_owned().lhs.unwrap()),
            rhs: Some(l.to_owned().rhs.unwrap()),
            ..Default::default()
        }
    };
    assert_eq!(e, l)
}

#[test]
fn parser_assign_test() {
    let t = Token::tokenize("fn int { int a = 3; a }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
    let e = { NodeSt::num(3, l.to_owned().c.loc) };
    assert_eq!(e, l)
}

#[test]
fn variable_expansion_test() {
    let t = Token::tokenize("fn int { int a = 3; int b = 5; b*a; }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().ret_node_st;
    let e = NodeSt {
        c: Node::mul(Loc::new(32, 33)),
        lhs: Some(l.to_owned().lhs.unwrap()),
        rhs: Some(l.to_owned().rhs.unwrap()),
        ..Default::default()
    };

    assert_eq!(e, l)
}

#[test]
fn return_with_unclosed_test() {
    let t = Token::tokenize("fn int { return 3 }").unwrap();
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
    let t = Token::tokenize("fn int { return 3; 4 }").unwrap();
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

#[test]
fn not_match_return_type_test() {
    let t = Token::tokenize("fn { 12+3; }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotMatchReturnType(_) => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn match_void_type_test() {
    let t = Token::tokenize("fn { _ }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => true,
        _ => false,
    };
    assert!(l)
}

#[test]
fn not_match_void_type_test() {
    let t = Token::tokenize("fn int { _ }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotMatchReturnType(_) => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn unexpected_underscore_operator_test() {
    let t = Token::tokenize("fn int { _; 1 }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::UnexpectedUnderScoreOperator(_) => true,
            _ => false,
        },
    };
    assert!(l)
}
