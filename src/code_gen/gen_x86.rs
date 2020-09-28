use super::super::node::node::*;
use std::fs;
use std::io::Write;

const REGS: [&str; 8] = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"];
static mut CC: u8 = 0;

pub fn gen(ns: NodeSt) {
    const DIR: &str = "workspace/tmp.s";
    fs::remove_file(DIR).unwrap();

    let mut f = fs::File::create(DIR).unwrap();
    write!(f, " .global main\n").unwrap();
    write!(f, "main:\n").unwrap();

    gen_x86(&mut f, ns);

    unsafe { write!(f, "  mov %{} , %rax\n", REGS[CC as usize - 1 ]).unwrap() } ;
    write!(f, "  ret\n").unwrap();
}

fn gen_x86(f: &mut fs::File, ns: NodeSt) {
    if ns.c.value == NodeKind::Default {
        return;
    }

    if ns.rhs != None {
        gen_x86(f, ns.rhs.unwrap().as_ref().to_owned());
    }
    if ns.lhs != None {
        gen_x86(f, ns.lhs.unwrap().as_ref().to_owned());
    }

    match ns.c.value {
        NodeKind::Num(n) => {
            unsafe { write!(f, "  mov ${}, %{}\n", n, REGS[CC as usize]).unwrap() };
            unsafe { CC += 1 };
        }
        NodeKind::Add => {
            unsafe {
                write!(
                    f,
                    "  add %{}, %{}\n",
                    REGS[CC as usize - 1],
                    REGS[CC as usize - 2]
                )
                .unwrap()
            };
            unsafe { CC -= 1 };
        }
        NodeKind::Sub => {
            unsafe {
                write!(
                    f,
                    "  sub %{}, %{}\n",
                    REGS[CC as usize - 2],
                    REGS[CC as usize - 1],
                )
                .unwrap()
            };
        }

        _ => unimplemented!(),
    }
}
