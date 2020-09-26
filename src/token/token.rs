use super::super::location::location::*;
use super::error::*;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Num(u8),
    Plus,
    Minus,
    Asterisk,
    Slash,
    NULL,
}

pub type Token = Annot<TokenKind>;

impl Token {
    fn number(n: u8, loc: Loc) -> Self {
        Self::new(TokenKind::Num(n), loc)
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
    pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
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
                b'\n' => {
                    return Err(TokenError::invalid_token(
                        input.to_string().chars().nth(i).unwrap(),
                        {
                            Loc {
                                f: i as u8,
                                e: i as u8 + 1,
                            }
                        },
                    ))
                }
                b' ' => (),
                _ => unimplemented!(),
            }
            i += 1
        }
        Ok(p_data)
    }
}

impl Token {
    pub fn get_val(&mut self) -> u8 {
        match self.value {
            TokenKind::Num(n) => n,
            _ => unimplemented!(),
        }
    }
}
