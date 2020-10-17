use lio::code_gen::gen_x86_64::*;
use lio::error::error::*;
use lio::program::program::*;
// use lio::fmt::fmt::*;
// use lio::parser::error::*;
use lio::simplified::simplified::*;
// use lio::token::error::*;
use lio::map::map::*;
// use lio::node_arr::node_arr::*;
use lio::token::token::*;
use std::env;
use std::io::Write;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];
    let mut fsimplified = false;
    if args.len() > 2 {
        if args[2] == "simplified" {
            fsimplified = true;
        }
    }

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
            let t = match map(t) {
                Ok(t) => t,
                Err(e) => {
                    // e.show_diagnostic(arg1); // FIXME
                    show_trace(e);
                    continue;
                }
            };
            let mut _nas = match Program::w_parser(t.to_owned()) {
                Ok(nas) => nas,
                Err(e) => {
                    // e.show_diagnostic(arg1); // FIXME
                    show_trace(e);
                    continue;
                }
            };
            println!("{:?}", _nas);

            let mut _na = vec![];
            if fsimplified {
                for n in _nas.to_owned().na {
                    _na.push(simplified(n))
                }
                println!("after simplified: {:?}", _na);
            }

            let mut min = _na.iter();
            let _nst = gen(min.next().unwrap().to_owned());
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

        let t = match map(t) {
            Ok(t) => t,
            Err(e) => {
                // e.show_diagnostic(arg1); // FIXME
                show_trace(e);
                std::process::exit(1);
            }
        };
        let mut _nas = match Program::w_parser(t.to_owned()) {
            Ok(nas) => nas,
            Err(e) => {
                // e.show_diagnostic(arg1); // FIXME
                show_trace(e);
                std::process::exit(1);
            }
        };

        let mut _na = vec![];
        if fsimplified {
            for n in _nas.to_owned().na {
                _na.push(simplified(n))
            }
            println!("after simplified: {:?}", _na);
        } else {
            for n in _nas.to_owned().na {
                _na.push(n)
            }
        }

        // println!("_na {:?}", _na);
        let mut min = _na.iter();
        // println!("min {:?}", min);
        let _nst = gen(min.next().unwrap().to_owned());
    }
}
