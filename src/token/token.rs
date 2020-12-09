use super::super::location::location::*;
use super::error::*;
use rustc_hash::FxHashMap;
use std::fmt::Debug;
use std::iter;

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
    While,
    Fn,
    To,
    Nill,
    LBrace,
    RBrace,
    Pipe,
    Comma,
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
    pub fn else_(loc: Loc) -> Self {
        Self::new(TokenKind::Else, loc)
    }
    pub fn while_(loc: Loc) -> Self {
        Self::new(TokenKind::While, loc)
    }
    pub fn fn_(loc: Loc) -> Self {
        Self::new(TokenKind::Fn, loc)
    }
    pub fn to(loc: Loc) -> Self {
        Self::new(TokenKind::To, loc)
    }
    pub fn nill(loc: Loc) -> Self {
        Self::new(TokenKind::Nill, loc)
    }
    pub fn lbrace(loc: Loc) -> Self {
        Self::new(TokenKind::LBrace, loc)
    }
    pub fn rbrace(loc: Loc) -> Self {
        Self::new(TokenKind::RBrace, loc)
    }
    pub fn comma(loc: Loc) -> Self {
        Self::new(TokenKind::Comma, loc)
    }
}

struct Lexer<'a> {
    pub peek: iter::Peekable<std::str::Chars<'a>>,
    pub p_data: Vec<Annot<TokenKind>>,
    pub e: bool,
    pub ev: Vec<TokenError>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            peek: input.chars().peekable(),
            p_data: vec![],
            e: false,
            ev: vec![],
        }
    }
}

impl Token {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, Vec<TokenError>> {
        let mut lexer = Lexer::new(input);
        let l = input.len();
        let mut b = 0;
        let mut i = 0;

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
            map.insert("while".into(), TokenKind::While);
            map.insert("fn".into(), TokenKind::Fn);
            map.insert("->".into(), TokenKind::To);
            map.insert("()".into(), TokenKind::Nill);
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
            map.insert('|'.into(), TokenKind::Pipe);
            map.insert(','.into(), TokenKind::Comma);
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
                lexer
                    .p_data
                    .push(Self::number(n, Loc::new(t as u8 + b, (i + 1) as u8 + b)));
            } else if input.as_bytes()[i] == b' ' {
                b += 1;
            } else {
                let mut _m = false;
                for k in ms.to_owned() {
                    if input[i..].starts_with(&k.0) {
                        lexer.p_data.push(Self::new(
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
                        lexer
                            .p_data
                            .push(Self::new(k.1, Loc::new(i as u8 + b, (i as u8 + 1) + b)));
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
                        lexer.p_data.push(Self::new(
                            TokenKind::UnderScore,
                            Loc::new(t as u8 + b, (i as u8 + 1) + b),
                        ));
                        continue;
                    }

                    lexer
                        .p_data
                        .push(Self::ident(s, Loc::new(t as u8 + b, (i as u8 + 1) + b)));
                    continue;
                }
                lexer.ev.push(TokenError::invalid_token(
                    input.to_string().chars().nth(i).unwrap(),
                    Loc::new(i as u8 + b, i as u8 + 1 + b),
                ));
                if !lexer.e {
                    lexer.e = true;
                }
            }
            i += 1
        }
        if lexer.e {
            return Err(lexer.ev);
        }

        Ok(lexer.p_data)
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

pub struct TokenIter<'a> {
    pub p: std::iter::Peekable<std::slice::Iter<'a, Annot<TokenKind>>>,
    pub shadow_p: std::iter::Peekable<std::slice::Iter<'a, Annot<TokenKind>>>,
}

impl<'a> TokenIter<'a> {
    pub fn new(vt: &'a mut Vec<Annot<TokenKind>>) -> Self {
        let it = vt.iter().peekable();
        Self {
            p: it.to_owned(),
            shadow_p: it,
        }
    }

    pub fn next(&mut self) -> Token {
        self.p.next().unwrap().to_owned()
    }

    pub fn next_with_shadow(&mut self) {
        self.shadow_p = self.p.to_owned();
        self.p.next().unwrap();
    }

    pub fn copy_iter(&mut self) {
        self.shadow_p = self.p.to_owned();
    }

    pub fn peek_token(&mut self) -> Token {
        self.p.peek().unwrap().to_owned().to_owned()
    }

    pub fn peek_value(&mut self) -> TokenKind {
        self.p.peek().unwrap().value.to_owned()
    }

    pub fn peek_shadow(&mut self) -> Token {
        self.shadow_p.peek().unwrap().to_owned().to_owned()
    }

    pub fn back_to_shadow(&mut self) {
        self.p = self.shadow_p.to_owned();
    }

    pub fn peek_expression_or(&mut self) -> bool {
        match self.peek_value() {
            TokenKind::Ident(_)
            | TokenKind::Num(_)
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Asterisk
            | TokenKind::Slash => true,
            _ => false,
        }
    }

    pub fn peek_type_or(&mut self) -> bool {
        match self.peek_value() {
            TokenKind::Int | TokenKind::Nill => true,
            _ => false,
        }
    }
}
