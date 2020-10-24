use lio::code_gen::gen_llvm::*;
use lio::code_gen::gen_x86_64::*;
use lio::error::error::*;
use lio::map::map::*;
use lio::program::program::*;
use lio::token::token::*;
use std::env;
use std::io::Write;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];
    let mut fsimplified = false;
    let mut ll = false;
    if args.len() > 2 {
        if args[2] == "simplified" {
            fsimplified = true;
        } else if args[2] == "ll" {
            ll = true;
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

            let t = match Token::tokenize(&s) {
                Ok(t) => t,
                Err(e) => {
                    for e in e.to_owned() {
                        // e.show_diagnostic(arg1); // FIXME
                        show_trace(e);
                    }
                    std::process::exit(1);
                }
            };

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
                    _na.push(n.simplified())
                }
                println!("after simplified: {:?}", _na);
            }

            let mut min = _na.iter();
            let _nst = gen_x86_64(min.next().unwrap().to_owned());
        }
    } else {
        println!("INPUT: {}", arg1);
        let t = match Token::tokenize(arg1) {
            Ok(n) => n,
            Err(e) => {
                for e in e.to_owned() {
                    // e.show_diagnostic(arg1); // FIXME
                    show_trace(e);
                }
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
                _na.push(n.simplified())
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
        if !ll {
            let _nst = gen_x86_64(min.next().unwrap().to_owned());
        } else {
            let _nst = gen_llvm_ir(min.next().unwrap().to_owned());
        }
    }
}
