use rr::code_gen::gen_x86::*;
use rr::node::node::*;
use rr::token::token::*;
use std::env;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];

    if arg1 == "repl" {
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let s = s.trim();

            if s == "q" {
                break;
            }

            let t = Token::tokenize(&s).unwrap();
            let _nst = match NodeSt::parser(t) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };
            println!("{:?}", _nst);
            let _nst = gen(_nst);
        }
    } else {
        println!("INPUT: {}", arg1);
        let t = Token::tokenize(arg1).unwrap();
        println!("{:?}", t);
        let _nst = NodeSt::parser(t).unwrap();
        // println!("{:?}", _nst.c);
        println!("{:?}", _nst);
        let _nst = gen(_nst);
    }
}
