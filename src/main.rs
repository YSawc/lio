use lio::code_gen::gen_x86::*;
use lio::error::error::*;
// use lio::fmt::fmt::*;
// use lio::parser::error::*;
use lio::simplified::simplified::*;
// use lio::token::error::*;
use lio::node_arr::node_arr::*;
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
            let _na = match NodeArr::w_parser(t.to_owned()) {
                Ok(na) => na,
                Err(e) => {
                    // e.show_diagnostic(arg1); // FIXME
                    show_trace(e);
                    continue;
                }
            };
            println!("{:?}", _na);
            let _nst = gen(_na);
        }
    } else {
        println!("INPUT: {}", arg1);
        let t = match Token::tokenize(arg1) {
            Ok(n) => n,
            Err(e) => {
                // e.show_diagnostic(arg1); // FIXME
                show_trace(e);
                std::process::exit(1);
            }
        };
        println!("after tokenized: {:?}", t);

        let mut _na = match NodeArr::w_parser(t.to_owned()) {
            Ok(na) => na,
            Err(e) => {
                // e.show_diagnostic(arg1); // FIXME
                show_trace(e);
                std::process::exit(1);
            }
        };

        println!("after parsed: {:?}", _na);

        if args.len() > 2 {
            if args[2] == "simplified" {
                _na = simplified(_na);
                println!("after beta: {:?}", _na);
            }
        }

        let _nst = gen(_na);
    }
}
