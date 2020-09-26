#[cfg(test)]
use super::super::location::location::*;
#[cfg(test)]
use super::super::node::node::*;
#[cfg(test)]
use super::super::token::error::*;
#[cfg(test)]
use super::super::token::token::*;

#[test]
fn parser_test() {
    let t = Token::tokenize("12+1*2").unwrap();
    let l = NodeSt::parser(t);
    let e = NodeSt {
        c: Annot {
            value: NodeKind::Default,
            loc: Loc { f: 0, e: 0 },
        },
        lhs: Some(Box::new(NodeSt {
            c: Annot {
                value: NodeKind::Num(2),
                loc: Loc { f: 5, e: 6 },
            },
            lhs: None,
            rhs: None,
        })),
        rhs: Some(Box::new(NodeSt {
            c: Annot {
                value: NodeKind::Default,
                loc: Loc { f: 0, e: 0 },
            },
            lhs: None,
            rhs: None,
        })),
    };
    assert_eq!(e, l)
}

#[test]
fn unimplemented_new_line_test() {
    let l = match Token::tokenize("1+1\n") {
        Err(e) => match e.value {
            TokenErrorKind::InvalidToken('\n') => true,
            _ => false,
        },
        _ => false,
    };
    assert!(l);
}
