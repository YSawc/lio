use rr::location::location::*;
use rr::node::node::*;
use rr::parser::parser::*;
use rr::token::token::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];

    println!("INPUT: {}", "12+1*2");
    let t = Token::tokenize("12+1*2");
    // println!("{:?}", t);
    let nst = NodeSt::parser(t);
    // println!("{:?}", nst);

    // println!("  .intel_syntax noprefix");
    // println!("  .global _start");
    // println!("_start:");
    // println!("    mov rax, 60");
    // println!("    mov rdi, {}", arg1);
    // println!("    syscall");
}

#[test]
fn tokenize_test() {
    let t = Token::tokenize("12+1*2");
    let e = vec![
        (Annot {
            value: TokenKind::Num(12),
            loc: Loc { f: 0, e: 2 },
        }),
        (Annot {
            value: TokenKind::Plus,
            loc: Loc { f: 2, e: 3 },
        }),
        (Annot {
            value: TokenKind::Num(1),
            loc: Loc { f: 3, e: 4 },
        }),
        (Annot {
            value: TokenKind::Asterisk,
            loc: Loc { f: 4, e: 5 },
        }),
        (Annot {
            value: TokenKind::Num(2),
            loc: Loc { f: 5, e: 6 },
        }),
    ];
    let nst = NodeSt::parser(t);
    println!("{:?}", nst);
}
