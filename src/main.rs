use rr::code_gen::gen_x86::*;
use rr::node::node::*;
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
    let _nst = gen(_nst);
}
