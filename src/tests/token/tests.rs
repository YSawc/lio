#[cfg(test)]
use super::super::super::location::location::*;
#[cfg(test)]
use super::super::super::token::token::*;

#[test]
fn tokenize_test() {
    let t = Token::tokenize("12+1*2").unwrap().to_owned();
    let e = vec![
        (Annot::new(TokenKind::Num(12), Loc::new(0, 2))),
        (Annot::new(TokenKind::Plus, Loc::new(2, 3))),
        (Annot::new(TokenKind::Num(1), Loc::new(3, 4))),
        (Annot::new(TokenKind::Asterisk, Loc::new(4, 5))),
        (Annot::new(TokenKind::Num(2), Loc::new(5, 6))),
    ];
    assert_eq!(e, t);
}

#[test]
fn pass_comparison_tokenize_test() {
    let i = "<<===>>=";
    let _t = Token::tokenize(i).unwrap().to_owned();

    assert!(true);
}
