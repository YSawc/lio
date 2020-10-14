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
fn simplified_variable_under_initialize_test() {
    let t = Token::tokenize("fn int { int a = 2; int b = 8*a; int c = 2*b+a; c; }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().l;
    let loc = l.to_owned();
    let mut il = loc.iter();
    let mut e: Vec<Var> = Vec::new();
    e.push(Var {
        s: "a".to_string(),
        n: NodeSt::num(2, il.next().unwrap().n.c.loc.to_owned()),
    });
    e.push(Var {
        s: "b".to_string(),
        n: NodeSt::num(16, il.next().unwrap().n.c.loc.to_owned()),
    });
    e.push(Var {
        s: "c".to_string(),
        n: NodeSt::num(34, il.next().unwrap().n.c.loc.to_owned()),
    });
    assert_eq!(e, l);
}

#[test]
fn update_variable_test() {
    let t = Token::tokenize("fn int { int a = 3; int b = a; int c = 5; a = 1; a+c+b }").unwrap();
    let l = NodeArr::w_parser(t).unwrap().l;
    let loc = l.to_owned();
    let mut il = loc.iter();
    let mut e: Vec<Var> = vec![];
    e.push(Var::new(
        "b".to_string(),
        NodeSt::num(3, il.next().unwrap().n.c.loc.to_owned()),
    ));
    e.push(Var::new(
        "c".to_string(),
        NodeSt::num(5, il.next().unwrap().n.c.loc.to_owned()),
    ));
    e.push(Var::new(
        "a".to_string(),
        NodeSt::num(1, il.next().unwrap().n.c.loc.to_owned()),
    ));
    assert_eq!(e, l.to_owned())
}

#[test]
fn unused_variable_check_for_used_variable_test() {
    let t = Token::tokenize("fn int { int a = 3; a; int a = 0; int b = 5; a*b }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(l)
}

#[test]
fn unused_variable_check_when_parsed_whole_function_test() {
    let t = Token::tokenize("fn { int a = 3; int b = a; int c = 5; b }").unwrap();
    let l = match NodeArr::w_parser(t) {
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
    let t = Token::tokenize("fn { int a = 3; int a = 0; a }").unwrap();
    let l = match NodeArr::w_parser(t) {
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
    let t = Token::tokenize("fn int { int a = 3; int b = a; int c = 5; a = 1; 0+c+b }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(l)
}

#[test]
fn undefined_variable_test() {
    let t = Token::tokenize("fn { int a = 3; d = 1; a }").unwrap();
    let l = match NodeArr::w_parser(t) {
        Ok(_) => false,
        Err(e) => match e {
            ParseError::UndefinedVariable(_) => true,
            _ => false,
        },
    };
    assert!(l)
}
