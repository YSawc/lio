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
    LParen,
    RParen,
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn number(n: u8, loc: Loc) -> Self {
        Self::new(TokenKind::Num(n), loc)
    }

    pub fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }

    pub fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }

    pub fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }

    pub fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }
    pub fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    pub fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

impl Token {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
        let mut p_data = Vec::new();
        let l = input.len();
        let mut b = 0;
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
                            f: t as u8 + b,
                            e: (i + 1) as u8 + b,
                        },
                    ));
                }
                b'+' => {
                    p_data.push(Self::plus(Loc {
                        f: i as u8 + b,
                        e: (i + 1) as u8 + b,
                    }));
                }
                b'-' => {
                    p_data.push(Self::minus(Loc {
                        f: i as u8 + b,
                        e: (i + 1) as u8 + b,
                    }));
                }
                b'*' => {
                    p_data.push(Self::asterisk(Loc {
                        f: i as u8 + b,
                        e: (i + 1) as u8 + b,
                    }));
                }
                b'/' => {
                    p_data.push(Self::slash(Loc {
                        f: i as u8 + b,
                        e: (i + 1) as u8 + b,
                    }));
                }
                b'(' => {
                    p_data.push(Self::lparen(Loc {
                        f: i as u8 + b,
                        e: (i + 1) as u8 + b,
                    }));
                }
                b')' => {
                    p_data.push(Self::rparen(Loc {
                        f: i as u8 + b,
                        e: (i + 1) as u8 + b,
                    }));
                }
                b' ' => b += 1,
                _ => {
                    b = 0;
                    return Err(TokenError::invalid_token(
                        input.to_string().chars().nth(i).unwrap(),
                        {
                            Loc {
                                f: i as u8 + b,
                                e: i as u8 + 1 + b,
                            }
                        },
                    ));
                }
            }
            i += 1
        }
        Ok(p_data)
    }
}

impl Token {
    pub fn get_val(&mut self) -> Result<u8, TokenErrorKind> {
        match self.value {
            TokenKind::Num(n) => Ok(n),
            _ => Err(TokenErrorKind::InvalidNumber(self.to_owned())),
        }
    }
}
