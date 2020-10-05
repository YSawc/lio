use lio::code_gen::gen_x86::*;
use lio::node::node::*;
use lio::simplified::beta::*;
use lio::token::token::*;
use std::env;
use std::io::Write;
use std::process::Command;

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

            if s == "." {
                let _ = Command::new("cc")
                    .arg("-o")
                    .arg("workspace/tmp")
                    .arg("workspace/tmp.s")
                    .spawn()
                    .expect("failed to execute process")
                    .wait();

                let _ = Command::new("cat")
                    .arg("workspace/tmp.s")
                    .spawn()
                    .expect("failed to execute process")
                    .wait();

                let o = Command::new("workspace/tmp")
                    .status()
                    .expect("failed to execute process");
                println!("output: {:?}", o.code().unwrap());
                continue;
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
        println!("after tokenized: {:?}", t);
        let mut _nst = NodeSt::parser(t).unwrap();
        println!("after parsed: {:?}", _nst);

        if args.len() > 2 {
            if args[2] == "simplified" {
                _nst = beta(_nst);
                println!("after beta: {:?}", _nst);
            }
        }
        // let _nst = beta(_nst);
        // println!("after beta: {:?}", _nst);

        let _nst = gen(_nst);
    }
}
