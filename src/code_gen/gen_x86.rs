use super::super::node::node::*;
use std::fs;
use std::io::Write;

pub fn gen_x86(ns: NodeSt) {
    let mut f = fs::File::create("tmp.s").unwrap();
    f.write(b"  .global main\n").unwrap();
    f.write(b"main:\n").unwrap();
    match ns.c.value {
        NodeKind::Num(n) => write!(f, "  mov ${}, %rax\n", n).unwrap(),
        _ => unimplemented!(),
    }
    f.write(b"  ret\n").unwrap();
    f.flush().unwrap();
}
