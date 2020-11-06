#[cfg(test)]
use super::super::super::node_arr::node_arr::*;
#[cfg(test)]
use super::super::super::parser::error::*;
#[cfg(test)]
use super::super::super::program::program::*;
#[cfg(test)]
use super::super::super::token::token::*;

#[test]
fn unused_variable_check_for_used_variable_test() {
    let mut t = Token::tokenize("fn int { int a = 3; a; int a = 0; int b = 5; a*b }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let l = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(l)
}

#[test]
fn unused_variable_check_when_parsed_whole_function_test() {
    let mut t = Token::tokenize("fn { int a = 3; int b = a; int c = 5; b }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let l = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::UnusedVariable(_) => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn unused_variable_check_just_before_overwrite_variable_test() {
    let mut t = Token::tokenize("fn { int a = 3; int a = 0; a }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let l = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::UnusedVariable(_) => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn unused_variable_check_for_update_variable_test() {
    let mut t =
        Token::tokenize("fn int { int a = 3; int b = a; int c = 5; a = 1; 0+c+b }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let l = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(l)
}

#[test]
fn undefined_variable_check_for_local_variable_test() {
    let mut t = Token::tokenize("fn { int a = 3; d = 1; a }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let l = match NodeArr::w_parser(&mut it, vec![]) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::UndefinedVariable(_) => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn unused_variable_check_for_under_score_local_variable_test() {
    let mut t = Token::tokenize("fn { int _g = 10; _ }").unwrap();
    let n = match Program::w_parser(&mut t) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(n)
}

#[test]
fn unused_variable_check_for_global_variable_test() {
    let mut t = Token::tokenize("int g = 10; fn int { int g = 3; g }").unwrap();
    let l = match Program::w_parser(&mut t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::UnusedVariable(_) => true,
            _ => false,
        },
    };
    assert!(l)
}

#[test]
fn unused_variable_check_for_under_score_global_variable_test() {
    let mut t = Token::tokenize("int _g = 10; fn { _ }").unwrap();
    let n = match Program::w_parser(&mut t) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(n)
}

#[test]
fn not_a_compile_time_constant_test() {
    let mut t = Token::tokenize("int _g = 3; int e = _g+1; fn { _ }").unwrap();
    let n = match Program::w_parser(&mut t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::NotACompileTimeConstant(_) => true,
            _ => false,
        },
    };
    assert!(n)
}
