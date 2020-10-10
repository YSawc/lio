use super::super::location::location::*;
use super::super::token::token::*;
use super::error::*;

pub fn map(vt: Vec<Token>) -> Result<Vec<Token>, MapError> {
    let mut rv: Vec<Token> = vec![];
    let mut vti = vt.iter().peekable();
    let mut _f = 0i8;
    let mut _e = 0i8;
    while vti.peek() != None {
        let mut me = vti.to_owned();
        let ma = vti.peek().unwrap().to_owned().to_owned();
        match vti.peek().unwrap().value {
            TokenKind::Map => {
                let loc = vti.peek().unwrap().loc.to_owned();
                vti.next();
                if vti.peek() == None {
                    return Err(MapError::invalid_struct(
                        me.next().unwrap().to_owned(),
                        ma.loc.to_owned(),
                    ));
                }
                me = vti.to_owned();
                match vti.next().unwrap().value {
                    TokenKind::Num(n) => {
                        _f = n.to_owned();
                    }
                    _ => {
                        return Err(MapError::invalid_struct(
                            me.next().unwrap().to_owned(),
                            ma.loc.to_owned(),
                        ));
                    }
                }
                me = vti.to_owned();
                match vti.next().unwrap().value {
                    TokenKind::Num(n) => {
                        _e = n.to_owned();
                    }
                    _ => {
                        return Err(MapError::invalid_struct(
                            me.next().unwrap().to_owned(),
                            ma.loc.to_owned(),
                        ));
                    }
                }
                me = vti.to_owned();
                let mut n = 0;
                // println!("(_f, _e) : ({}, {})", _f, _e);
                match vti.peek().unwrap().value {
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Asterisk | TokenKind::Slash => {
                        match vti.next().unwrap().value {
                            TokenKind::Plus => {
                                for i in _f..=_e {
                                    n += i
                                }
                            }
                            TokenKind::Minus => {
                                for i in _f..=_e {
                                    n -= i
                                }
                            }
                            TokenKind::Asterisk => {
                                n = 1;
                                for i in _f..=_e {
                                    n *= i
                                }
                            }
                            TokenKind::Slash => {
                                n = 1;
                                for i in _f..=_e {
                                    n /= i
                                }
                            }
                            _ => unreachable!(),
                        }
                        println!("n: {:?}", n);
                        rv.push(Token::number(
                            n,
                            Loc::new(loc.f.to_owned(), loc.f.to_owned() + ((n % 10) + 1) as u8),
                        ));
                    }
                    _ => {
                        // m.next();
                        return Err(MapError::invalid_struct(
                            me.next().unwrap().to_owned(),
                            ma.loc.to_owned(),
                        ));
                    }
                }
            }
            _ => rv.push(vti.next().unwrap().to_owned()),
        }
    }
    return Ok(rv);
}
