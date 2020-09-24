use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Number,
    Plus,
    Minus,
    Asterisk,
    Slash,
    NULL,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub s: String,
    pub t: TokenKind,
}

pub struct Loc {
    pub loc_x: u8,
    pub loc_y: u8,
    pub t: Token,
}

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
                p_data.push(Token {
                    s: input[t..i + 1].to_string(),
                    t: TokenKind::Number,
                });
            }
            b'+' => {
                p_data.push(Token {
                    s: "+".to_string(),
                    t: TokenKind::Plus,
                });
            }
            b'-' => {
                p_data.push(Token {
                    s: "-".to_string(),
                    t: TokenKind::Minus,
                });
            }
            b'*' => {
                p_data.push(Token {
                    s: "*".to_string(),
                    t: TokenKind::Asterisk,
                });
            }
            b'/' => {
                p_data.push(Token {
                    s: "/".to_string(),
                    t: TokenKind::Slash,
                });
            }
            b' ' => (),
            _ => unimplemented!(),
        }
        i += 1
    }
    p_data
}

#[test]
fn parser_test() {
    let l = parse("12+1*2");
    let e = vec![
        (Token {
            s: "12".to_string(),
            t: TokenKind::Number,
        }),
        (Token {
            s: "+".to_string(),
            t: TokenKind::Plus,
        }),
        (Token {
            s: "1".to_string(),
            t: TokenKind::Number,
        }),
        (Token {
            s: "*".to_string(),
            t: TokenKind::Asterisk,
        }),
        (Token {
            s: "2".to_string(),
            t: TokenKind::Number,
        }),
    ];
    assert_eq!(e, l);
}
