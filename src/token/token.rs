use super::super::location::location::*;
use super::error::*;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Num(i8),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    LParen,
    RParen,
    E,
    NE,
    RT,
    RE,
    GT,
    GE,
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn number(n: i8, loc: Loc) -> Self {
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
    pub fn percent(loc: Loc) -> Self {
        Self::new(TokenKind::Percent, loc)
    }
    pub fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    pub fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
    pub fn eq(loc: Loc) -> Self {
        Self::new(TokenKind::E, loc)
    }
    pub fn neq(loc: Loc) -> Self {
        Self::new(TokenKind::NE, loc)
    }
    pub fn rt(loc: Loc) -> Self {
        Self::new(TokenKind::RT, loc)
    }
    pub fn re(loc: Loc) -> Self {
        Self::new(TokenKind::RE, loc)
    }
    pub fn gt(loc: Loc) -> Self {
        Self::new(TokenKind::GT, loc)
    }
    pub fn ge(loc: Loc) -> Self {
        Self::new(TokenKind::GE, loc)
    }
}

impl Token {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
        let mut p_data = Vec::new();
        let l = input.len();
        let mut b = 0;
        let mut i = 0;

        fn keyword_map() -> HashMap<String, TokenKind> {
            let mut map = HashMap::new();
            map.insert("==".into(), TokenKind::E);
            map.insert("!=".into(), TokenKind::NE);
            map.insert("<".into(), TokenKind::RT);
            map.insert("<=".into(), TokenKind::RE);
            map.insert(">".into(), TokenKind::GT);
            map.insert(">=".into(), TokenKind::GE);
            map
        }

        let ks = keyword_map();

        while i < l {
            match input.as_bytes()[i] {
                b'0'..=b'9' => {
                    let t = i;
                    while i < input.len() && input.as_bytes()[i].is_ascii_digit() {
                        i += 1;
                    }
                    i -= 1;
                    let n: i8 = input[t..i + 1].to_string().parse().unwrap();
                    p_data.push(Self::number(n, Loc::new(t as u8 + b, (i + 1) as u8 + b)));
                }
                b'+' => p_data.push(Self::plus(Loc::new(i as u8 + b, (i + 1) as u8 + b))),
                b'-' => p_data.push(Self::minus(Loc::new(i as u8 + b, (i + 1) as u8 + b))),
                b'*' => p_data.push(Self::asterisk(Loc::new(i as u8 + b, (i + 1) as u8 + b))),
                b'/' => p_data.push(Self::slash(Loc::new(i as u8 + b, (i + 1) as u8 + b))),
                b'%' => p_data.push(Self::percent(Loc::new(i as u8 + b, (i + 1) as u8 + b))),
                b'(' => p_data.push(Self::lparen(Loc::new(i as u8 + b, (i + 1) as u8 + b))),
                b')' => p_data.push(Self::rparen(Loc::new(i as u8 + b, (i + 1) as u8 + b))),
                b' ' => b += 1,
                _ => {
                    let mut _m = false;
                    for k in ks.to_owned() {
                        if input[i..].contains(&k.0) {
                            p_data.push(Self::new(
                                k.1,
                                Loc::new(i as u8, (i as u8 + k.0.len() as u8) + b),
                            ));
                            i += k.0.len();
                            _m = true;
                            break;
                        }
                    }
                    if _m == true {
                        _m = false;
                        break;
                    }

                    b = 0;
                    return Err(TokenError::invalid_token(
                        input.to_string().chars().nth(i).unwrap(),
                        Loc::new(i as u8 + b, i as u8 + 1 + b),
                    ));
                }
            }
            i += 1
        }
        Ok(p_data)
    }
}

impl Token {
    pub fn get_val(&mut self) -> Result<i8, TokenErrorKind> {
        match self.value {
            TokenKind::Num(n) => Ok(n),
            _ => Err(TokenErrorKind::InvalidNumber(self.to_owned())),
        }
    }
}
