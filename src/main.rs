// use rr::location::location::*;
use rr::node::node::*;
// use rr::parser::parser::*;
use rr::token::token::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];

    println!("INPUT: {}", arg1);
    let t = Token::tokenize(arg1).unwrap();
    println!("{:?}", t);
    let _nst = NodeSt::parser(t);
    println!("{:?}", _nst);

    println!("  .intel_syntax noprefix");
    println!("  .global _start");
    println!("_start:");
    println!("    mov rax, 60");
    println!("    mov rdi, {}", arg1);
    println!("    syscall");
}
