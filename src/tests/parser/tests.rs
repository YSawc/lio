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
    let mut t = Token::tokenize("fn -> int { 12+3 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = NodeArr::w_parser(&mut it, vec![]).unwrap().0.ret_node_st;
    let e = {
        NodeSt {
            c: Node::plus(Loc::new(18, 19)),
            lhs: Some(n.to_owned().lhs.unwrap()),
            rhs: Some(n.to_owned().rhs.unwrap()),
            ..Default::default()
        }
    };
    assert_eq!(e, n)
}

#[test]
fn evaluation_final_value_test() {
    let mut t = Token::tokenize("fn -> int { 12+3 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = NodeArr::w_parser(&mut it, vec![]).unwrap().0.ret_node_st;
    let e = {
        NodeSt {
            c: Node::plus(Loc::new(18, 19)),
            lhs: Some(n.to_owned().lhs.unwrap()),
            rhs: Some(n.to_owned().rhs.unwrap()),
            ..Default::default()
        }
    };
    assert_eq!(e, n)
}

#[test]
fn parser_assign_test() {
    let mut t = Token::tokenize("fn { int a = 3; a; _ }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(n)
}

#[test]
fn return_with_unclosed_test() {
    let mut t = Token::tokenize("fn -> int { return 3 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotClosedStmt(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn operater_after_return_test() {
    let mut t = Token::tokenize("fn -> int { return 3; 4 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::OperatorAfterRetrun(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn not_exit_when_failed_parser_test() {
    let mut t = Token::tokenize("fn { +3 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(_) => true,
    };
    assert!(n)
}

#[test]
fn not_number_test() {
    let mut t = Token::tokenize("fn { 5+ }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotNumber(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn unclosed_paren_test() {
    let mut t = Token::tokenize("fn { 5+(3+2*2 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotClosedParen(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn operator_out_of_function_test() {
    let mut t = Token::tokenize("0; fn { 12+3; }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::OperatorOutOfFnction(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn not_match_return_type_test() {
    let mut t = Token::tokenize("fn { 12+3; 0 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotMatchReturnType(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn match_void_type_test() {
    let mut t = Token::tokenize("fn { _ }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => true,
        _ => false,
    };
    assert!(n)
}

#[test]
fn not_match_void_type_test() {
    let mut t = Token::tokenize("fn -> int { _ }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotMatchReturnType(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn unexpected_underscore_operator_test() {
    let mut t = Token::tokenize("fn -> int { _; 1 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::UnexpectedUnderScoreOperator(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn not_opened_stmt_test() {
    let mut t = Token::tokenize("fn -> int { if (2 == 3)  5; } else { 10; } }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotOpenedStmt(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn parse_not_number_in_if_statement_test() {
    let mut t = Token::tokenize("fn -> int { if (2 == 3) { 5; else { 10; } }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotNumber(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn not_opened_else_stmt_test() {
    let mut t = Token::tokenize("fn -> int { if (2 == 3) { 5; } else 10; } }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotOpenedStmt(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

// #[test]
// fn not_closed_else_stmt_test() {
//     let mut t = Token::tokenize("fn -> int { if (2 == 3) { 5; } else { 10;  0}").unwrap();
//     let mut it = TokenIter::new(&mut t);
//     let n = match NodeArr::w_parser(&mut it, vec![]) {
//         Ok(_) => false,
//         Err(e) => match e {
//             ParseError::NotClosedStmt(_) => true,
//             _ => false,
//         },
//     };
//     assert!(n)
// }

// #[test]
// fn eof_around_closed_else_stmt_test() {
//     let mut t = Token::tokenize("fn -> int { if (2 == 3) { 5; } else { 10; }").unwrap();
//     let mut it = TokenIter::new(&mut t);
//     let n = match NodeArr::w_parser(&mut it, vec![]) {
//         Ok(_) => false,
//         Err(e) => match e {
//             ParseError::Eof => true,
//             _ => false,
//         },
//     };
//     assert!(n)
// }

#[test]
fn unexpected_under_score_operator_test() {
    let mut t = Token::tokenize("fn { _; }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::UnexpectedUnderScoreOperator(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn type_match_another_one_of_statement_1_test() {
    let mut t = Token::tokenize("fn { int i = 9; if (i) { i; _ } else { 3*4; _ } _ }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(n)
}

#[test]
fn type_match_another_one_of_statement_2_test() {
    let mut t = Token::tokenize("fn { int i = 9; if (i) { 2; 0; } else { 3*4; _ } _ }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(n)
}

#[test]
fn type_match_another_one_of_statement_3_test() {
    let mut t = Token::tokenize("fn { int i = 9; if (i) { i; 0 } else { 3*4; 3 } _ }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(n)
}

#[test]
fn type_not_match_another_one_of_statement_1_test() {
    let mut t = Token::tokenize("fn { int i = 9; if (i) { i; 0 } else { 3*4; 2; } _ }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotMatchTypeAnotherOneOfStatement(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn checker_for_not_definition_variable_in_single_evaluation_value_test() {
    let mut t = Token::tokenize("fn -> int { int aa = 9; b }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotDefinitionVar(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn checker_for_not_definition_variable_in_multiple_evaluation_values_test() {
    let mut t = Token::tokenize("fn -> int { int aa = 9; b*3 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotDefinitionVar(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn check_closed_immediate_in_new_assign_test() {
    let mut t = Token::tokenize("fn { int aa = return 9; _ }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotClosedImmediate(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn check_closed_immediate_in_assign_test() {
    let mut t = Token::tokenize("fn { int aa = 9; aa; aa = return 3; _ }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotClosedImmediate(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn check_opened_immediate_in_assign_fail_test1() {
    let mut t =
        Token::tokenize("fn -> int { int i = 0; i = while (i < 30) { i = i + 1; | { i } } i }")
            .unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotOpenedImmediate(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn check_opened_immediate_in_assign_fail_test2() {
    let mut t =
        Token::tokenize("fn -> int { int i = 0; i = while (i < 30) { i = i + 1; | i = i; i } i }")
            .unwrap();
    let mut it = TokenIter::new(&mut t);

    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotOpenedImmediate(_) => true,
            _ => false,
        },
    };
    assert!(n)
}

#[test]
fn check_opened_immediate_in_assign_pass_test() {
    let mut t = Token::tokenize("fn -> int { int i = 0; i = while (i < 30) { i = i + 1; | i } i }")
        .unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(n)
}

#[test]
fn expect_type_test() {
    let mut t = Token::tokenize("fn -> (0) { 0 }").unwrap();
    let mut it = TokenIter::new(&mut t);

    let n = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotType(_) => true,
            _ => false,
        },
    };
    assert!(n)
}
