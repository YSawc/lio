#[cfg(test)]
use super::super::super::location::location::*;
#[cfg(test)]
use super::super::super::map::map::*;
#[cfg(test)]
use super::super::super::token::token::*;

#[test]
fn map_test() {
    let t = Token::tokenize("map 3 5 +").unwrap().to_owned();
    let l = map(t).unwrap();
    let e = vec![(Annot::new(TokenKind::Num(12), Loc::new(0, 3)))];
    assert_eq!(e, l);
}
