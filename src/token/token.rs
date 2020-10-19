use super::super::location::location::*;
use super::error::*;
use rustc_hash::FxHashMap;
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
    L,
    LE,
    G,
    GE,
    SemiColon,
    Return,
    Int,
    Ident(String),
    Assign,
    Map,
    If,
    Else,
    Fn,
    LBrace,
    RBrace,
    UnderScore,
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
    pub fn l(loc: Loc) -> Self {
        Self::new(TokenKind::L, loc)
    }
    pub fn le(loc: Loc) -> Self {
        Self::new(TokenKind::LE, loc)
    }
    pub fn g(loc: Loc) -> Self {
        Self::new(TokenKind::G, loc)
    }
    pub fn ge(loc: Loc) -> Self {
        Self::new(TokenKind::GE, loc)
    }
    pub fn semicolon(loc: Loc) -> Self {
        Self::new(TokenKind::SemiColon, loc)
    }
    pub fn ret(loc: Loc) -> Self {
        Self::new(TokenKind::Return, loc)
    }
    pub fn int(loc: Loc) -> Self {
        Self::new(TokenKind::Int, loc)
    }
    pub fn ident(s: String, loc: Loc) -> Self {
        Self::new(TokenKind::Ident(s), loc)
    }
    pub fn assign(loc: Loc) -> Self {
        Self::new(TokenKind::Assign, loc)
    }
    pub fn map(loc: Loc) -> Self {
        Self::new(TokenKind::Map, loc)
    }
    pub fn mif(loc: Loc) -> Self {
        Self::new(TokenKind::If, loc)
    }
    pub fn melse(loc: Loc) -> Self {
        Self::new(TokenKind::Else, loc)
    }
    pub fn mfn(loc: Loc) -> Self {
        Self::new(TokenKind::Fn, loc)
    }
    pub fn lbrace(loc: Loc) -> Self {
        Self::new(TokenKind::LBrace, loc)
    }
    pub fn rbrace(loc: Loc) -> Self {
        Self::new(TokenKind::RBrace, loc)
    }
}

impl Token {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, Vec<TokenError>> {
        let mut p_data = Vec::new();
        let l = input.len();
        let mut b = 0;
        let mut i = 0;
        let mut e: bool = false;
        let mut ev: Vec<TokenError> = vec![];

        fn multiple_symbol_map_map() -> FxHashMap<String, TokenKind> {
            let mut map = FxHashMap::default();
            map.insert("==".into(), TokenKind::E);
            map.insert("!=".into(), TokenKind::NE);
            map.insert("<=".into(), TokenKind::LE);
            map.insert(">=".into(), TokenKind::GE);
            map.insert("return".into(), TokenKind::Return);
            map.insert("int".into(), TokenKind::Int);
            map.insert("map".into(), TokenKind::Map);
            map.insert("if".into(), TokenKind::If);
            map.insert("else".into(), TokenKind::Else);
            map.insert("fn".into(), TokenKind::Fn);
            map
        }

        fn single_symbol_map() -> FxHashMap<char, TokenKind> {
            let mut map = FxHashMap::default();
            map.insert('+'.into(), TokenKind::Plus);
            map.insert('-'.into(), TokenKind::Minus);
            map.insert('/'.into(), TokenKind::Slash);
            map.insert('*'.into(), TokenKind::Asterisk);
            map.insert('%'.into(), TokenKind::Percent);
            map.insert('('.into(), TokenKind::LParen);
            map.insert(')'.into(), TokenKind::RParen);
            map.insert('<'.into(), TokenKind::L);
            map.insert('>'.into(), TokenKind::G);
            map.insert(';'.into(), TokenKind::SemiColon);
            map.insert('='.into(), TokenKind::Assign);
            map.insert('{'.into(), TokenKind::LBrace);
            map.insert('}'.into(), TokenKind::RBrace);
            map
        }

        let ms = multiple_symbol_map_map();
        let ss = single_symbol_map();

        while i < l {
            if input.as_bytes()[i].is_ascii_digit() {
                let t = i;
                while i < input.len() && input.as_bytes()[i].is_ascii_digit() {
                    i += 1;
                }
                i -= 1;
                let n: i8 = input[t..i + 1].to_string().parse().unwrap();
                p_data.push(Self::number(n, Loc::new(t as u8 + b, (i + 1) as u8 + b)));
            } else if input.as_bytes()[i] == b' ' {
                b += 1;
            } else {
                let mut _m = false;
                for k in ms.to_owned() {
                    if input[i..].starts_with(&k.0) {
                        p_data.push(Self::new(
                            k.1,
                            Loc::new(i as u8 + b, (i as u8 + k.0.len() as u8) + b),
                        ));
                        i += k.0.len();
                        _m = true;
                        break;
                    }
                }

                if _m == true {
                    _m = false;
                    continue;
                }

                for k in ss.to_owned() {
                    if input.chars().nth(i).unwrap() == k.0 {
                        p_data.push(Self::new(k.1, Loc::new(i as u8 + b, (i as u8 + 1) + b)));
                        i += 1;
                        _m = true;
                        break;
                    }
                }

                if _m == true {
                    _m = false;
                    continue;
                }

                b = 0;

                if input.as_bytes()[i].is_ascii_alphabetic() || input.as_bytes()[i] == b'_' {
                    let t = i;
                    let mut s = String::new();
                    if input.as_bytes()[i] == b'_' {
                        s.push(input.chars().nth(i).unwrap());
                        i += 1;
                    }
                    while i < l && input.as_bytes()[i].is_ascii_alphabetic() {
                        s.push(input.chars().nth(i).unwrap());
                        i += 1;
                    }

                    if s == "_" {
                        p_data.push(Self::new(
                            TokenKind::UnderScore,
                            Loc::new(t as u8 + b, (i as u8 + 1) + b),
                        ));
                        continue;
                    }

                    p_data.push(Self::ident(s, Loc::new(t as u8 + b, (i as u8 + 1) + b)));
                    continue;
                }
                ev.push(TokenError::invalid_token(
                    input.to_string().chars().nth(i).unwrap(),
                    Loc::new(i as u8 + b, i as u8 + 1 + b),
                ));
                if !e {
                    e = true;
                }
            }
            i += 1
        }
        if e {
            return Err(ev);
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
