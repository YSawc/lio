#[cfg(test)]
use super::super::location::location::*;
#[cfg(test)]
use super::super::token::token::*;

#[test]
fn tokenize_test() {
    let t = Token::tokenize("12+1*2").to_owned();
    let e = vec![
        (Annot {
            value: TokenKind::Num(12),
            loc: Loc { f: 0, e: 2 },
        }),
        (Annot {
            value: TokenKind::Plus,
            loc: Loc { f: 2, e: 3 },
        }),
        (Annot {
            value: TokenKind::Num(1),
            loc: Loc { f: 3, e: 4 },
        }),
        (Annot {
            value: TokenKind::Asterisk,
            loc: Loc { f: 4, e: 5 },
        }),
        (Annot {
            value: TokenKind::Num(2),
            loc: Loc { f: 5, e: 6 },
        }),
    ];
    assert_eq!(e, t);
}
