#[cfg(test)]
use super::super::location::location::*;
#[cfg(test)]
use super::super::node::node::*;
#[cfg(test)]
use super::super::token::token::*;

#[test]
fn parser_test() {
    let t = Token::tokenize("12+1*2");
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
