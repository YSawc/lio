#[cfg(test)]
use super::super::super::super::location::location::*;
#[cfg(test)]
use super::super::super::super::node::node::*;
#[cfg(test)]
use super::super::super::super::node_arr::node_arr::*;
#[cfg(test)]
use super::super::super::super::token::token::*;

#[test]
fn simplified_test() {
    let mut t = Token::tokenize("fn -> int { 2*(2-1)+5 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = NodeArr::w_parser(&mut it, vec![]).unwrap().0;
    let n = n.simplified().ret_nodes.first().unwrap().to_owned();
    let l = NodeSt::num(7, Loc::new(16, 17));
    assert_eq!(l, n);
}

#[test]
fn simplified_with_minus_test() {
    let mut t = Token::tokenize("fn -> int { 2*(2-4)+5 }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = NodeArr::w_parser(&mut it, vec![]).unwrap().0;
    let n = n.simplified().ret_nodes.first().unwrap().to_owned();
    let l = NodeSt::num(1, Loc::new(16, 17));
    assert_eq!(l, n);
}

#[test]
fn simplified_with_variable_test() {
    let mut t = Token::tokenize("fn -> int { int a = 1; 2*3-a }").unwrap();
    let mut it = TokenIter::new(&mut t);
    let n = NodeArr::w_parser(&mut it, vec![]).unwrap().0;
    let n = n.simplified().ret_nodes.first().unwrap().to_owned();
    let l = NodeSt {
        c: Annot {
            value: NodeKind::Sub,
            loc: Loc { f: 29, e: 30 },
        },
        lhs: Some(Box::new(NodeSt {
            c: Annot {
                value: NodeKind::Num(6),
                loc: Loc { f: 26, e: 27 },
            },
            ..Default::default()
        })),
        rhs: Some(Box::new(NodeSt {
            c: Annot {
                value: NodeKind::LVar(1),
                loc: Loc { f: 27, e: 29 },
            },
            ..Default::default()
        })),
        ..Default::default()
    };
    assert_eq!(l, n);
}
