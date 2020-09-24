use super::super::location::location::*;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Number(u8),
    Plus,
    Minus,
    Asterisk,
    Slash,
    NULL,
}

type Token = Annot<TokenKind>;

impl Token {
    fn number(n: u8, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }

    fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }

    fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }

    fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }
}

impl Token {
    pub fn parse(input: &str) -> Vec<Token> {
        let mut p_data = Vec::new();
        let l = input.len();
        let mut i = 0;
        while i < l {
            match input.as_bytes()[i] {
                b'0'..=b'9' => {
                    let t = i;
                    while i < input.len() && input.as_bytes()[i].is_ascii_digit() {
                        i += 1;
                    }
                    i -= 1;
                    let n: u8 = input[t..i + 1].to_string().parse().unwrap();
                    p_data.push(Self::number(
                        n,
                        Loc {
                            f: t as u8,
                            e: (i + 1) as u8,
                        },
                    ));
                }
                b'+' => {
                    p_data.push(Self::plus(Loc {
                        f: i as u8,
                        e: (i + 1) as u8,
                    }));
                }
                b'-' => {
                    p_data.push(Self::minus(Loc {
                        f: i as u8,
                        e: (i + 1) as u8,
                    }));
                }
                b'*' => {
                    p_data.push(Self::asterisk(Loc {
                        f: i as u8,
                        e: (i + 1) as u8,
                    }));
                }
                b'/' => {
                    p_data.push(Self::slash(Loc {
                        f: i as u8,
                        e: (i + 1) as u8,
                    }));
                }
                b' ' => (),
                _ => unimplemented!(),
            }
            i += 1
        }
        p_data
    }
}

#[test]
fn parser_test() {
    let l = Token::parse("12+1*2");
    // let l = Token::parse("12");
    let e = vec![
        (Annot {
            value: TokenKind::Number(12),
            loc: Loc { f: 0, e: 2 },
        }),
        (Annot {
            value: TokenKind::Plus,
            loc: Loc { f: 2, e: 3 },
        }),
        (Annot {
            value: TokenKind::Number(1),
            loc: Loc { f: 3, e: 4 },
        }),
        (Annot {
            value: TokenKind::Asterisk,
            loc: Loc { f: 4, e: 5 },
        }),
        (Annot {
            value: TokenKind::Number(2),
            loc: Loc { f: 5, e: 6 },
        }),
    ];
    assert_eq!(e, l);
}
