#[cfg(test)]
use super::super::location::location::*;
#[cfg(test)]
use super::super::node::node::*;
#[cfg(test)]
use super::super::token::token::*;

#[test]
fn parser_test() {
    let t = Token::tokenize("12+3").unwrap();
    let l = NodeSt::parser(t);
    let e = {
        NodeSt {
            c: Annot {
                value: NodeKind::Add,
                loc: Loc { f: 2, e: 3 },
            },
            lhs: Some(Box::new(NodeSt {
                c: Annot {
                    value: NodeKind::Num(12),
                    loc: Loc { f: 0, e: 2 },
                },
                lhs: None,
                rhs: None,
            })),
            rhs: Some(Box::new(NodeSt {
                c: Annot {
                    value: NodeKind::Num(3),
                    loc: Loc { f: 3, e: 4 },
                },
                lhs: None,
                rhs: None,
            })),
        }
    };

    assert_eq!(e, l)
}
