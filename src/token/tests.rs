#[cfg(test)]
use super::super::location::location::*;
#[cfg(test)]
use super::super::token::error::*;
#[cfg(test)]
use super::super::token::token::*;

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
fn invalid_token_test() {
    let l = match Token::tokenize("1+1\n") {
        Err(e) => match e.value {
            TokenErrorKind::InvalidToken('\n') => true,
            _ => false,
        },
        _ => false,
    };
    assert!(l);
}
