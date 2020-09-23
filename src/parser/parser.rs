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

impl Token {
    pub fn parse(input: &str) -> Vec<Token> {
        let mut p_data = Vec::new();
        let l = input.len();
        let mut i = 0;
        while i < l {
            if input.chars().nth(i).unwrap().is_numeric() {
                p_data.push(Token {
                    s: input.chars().nth(i).unwrap().to_string(),
                    t: TokenKind::Number,
                });
                i += 1;
            } else {
                i += 1;
            }
        }
        p_data
    }
}

#[test]
fn parser_test() {
    let l = Token::parse("1");
    let e = vec![
        (Token {
            s: "1".to_string(),
            t: TokenKind::Number,
        }),
    ];
    assert_eq!(e, l);
}
