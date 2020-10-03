use super::super::node::node::*;
use std::fs;
use std::io::Write;

const REGS: [&str; 8] = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"];
static mut CC: u8 = 0;

pub fn gen(ns: NodeSt) {
    const DIR: &str = "workspace/tmp.s";
    fs::File::create(DIR).unwrap();
    fs::remove_file(DIR).unwrap();

    let mut f = fs::File::create(DIR).unwrap();
    write!(f, " .global main\n").unwrap();
    write!(f, "main:\n").unwrap();

    let r = gen_x86(&mut f, ns);

    write!(f, "  mov %{}, %rax\n", r).unwrap();
    write!(f, "  ret\n").unwrap();
}

fn gen_x86(f: &mut fs::File, ns: NodeSt) -> String {
    match ns.c.value {
        NodeKind::Num(n) => {
            unsafe { write!(f, "  mov ${}, %{}\n", n, REGS[CC as usize]).unwrap() };
            unsafe { CC += 1 };
            unsafe { return REGS[CC as usize - 1].to_string() };
        }
        _ => (),
    }

    let l = gen_x86(f, ns.lhs.unwrap().as_ref().to_owned());
    let r = gen_x86(f, ns.rhs.unwrap().as_ref().to_owned());

    match ns.c.value {
        NodeKind::Add => {
            write!(f, "  add %{}, %{}\n", r, l).unwrap();
            return l;
        }
        NodeKind::Sub => {
            write!(f, "  sub %{}, %{}\n", r, l).unwrap();
            return l;
        }
        NodeKind::Mul => {
            write!(f, "  imul %{}, %{}\n", r, l).unwrap();
            return l;
        }
        _ => unimplemented!(),
    }
}
