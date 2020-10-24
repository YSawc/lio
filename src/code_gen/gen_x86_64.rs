use super::super::node::node::*;
use super::super::node_arr::node_arr::*;
use std::fs;
use std::io::Write;

const REGS: [&str; 8] = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"];
static mut CC: u8 = 0;

pub fn gen_x86_64(na: NodeArr) {
    const DIR: &str = "workspace/tmp.s";
    fs::File::create(DIR).unwrap();
    fs::remove_file(DIR).unwrap();

    let mut f = fs::File::create(DIR).unwrap();
    write!(f, " .global main\n").unwrap();
    write!(f, "main:\n").unwrap();

    let mut r = String::new();
    let mut nai = na.node_st_vec.iter().peekable();
    while nai.peek() != None {
        r = gen_x86_64_code(&mut f, nai.next().unwrap().to_owned());
    }

    write!(f, ".L.return:\n").unwrap();
    write!(f, "  mov %{}, %rax\n", r).unwrap();
    write!(f, "  ret\n").unwrap();
}

fn gen_x86_64_code(f: &mut fs::File, ns: NodeSt) -> String {
    match ns.c.value {
        NodeKind::Num(n) => {
            unsafe { write!(f, "  mov ${}, %{}\n", n, REGS[CC as usize]).unwrap() };
            unsafe { CC += 1 };
            unsafe { return REGS[CC as usize - 1].to_string() };
        }
        NodeKind::Return => {
            let l = gen_x86_64_code(f, ns.lhs.unwrap().as_ref().to_owned());
            write!(f, "  jmp .L.return\n").unwrap();
            return l;
        }
        NodeKind::UnderScore => {
            unsafe { write!(f, "  mov $0, %{}\n", REGS[CC as usize]).unwrap() };
            unsafe { return REGS[CC as usize].to_string() };
        }
        _ => (),
    }

    let l = gen_x86_64_code(f, ns.lhs.unwrap().as_ref().to_owned());
    let r = gen_x86_64_code(f, ns.rhs.unwrap().as_ref().to_owned());

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
        NodeKind::Div => {
            write!(f, "  mov %{}, %rax\n", l).unwrap();
            write!(f, "  cqo\n").unwrap();
            write!(f, "  idiv %{}\n", r).unwrap();
            write!(f, "  mov %rax, %{}\n", l).unwrap();
            return l;
        }
        NodeKind::Sur => {
            write!(f, "  mov %{}, %rax\n", l).unwrap();
            write!(f, "  cqo\n").unwrap();
            write!(f, "  idiv %{}\n", r).unwrap();
            write!(f, "  mov %rdx, %{}\n", l).unwrap();
            return l;
        }
        NodeKind::E => {
            write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
            write!(f, "  sete %al\n").unwrap();
            write!(f, "  movzb %al, %{}\n", l).unwrap();
            return l;
        }
        NodeKind::NE => {
            write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
            write!(f, "  setne %al\n").unwrap();
            write!(f, "  movzb %al, %{}\n", l).unwrap();
            return l;
        }
        NodeKind::L => {
            write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
            write!(f, "  setl %al\n").unwrap();
            write!(f, "  movzb %al, %{}\n", l).unwrap();
            return l;
        }
        NodeKind::LE => {
            write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
            write!(f, "  setle %al\n").unwrap();
            write!(f, "  movzb %al, %{}\n", l).unwrap();
            return l;
        }
        NodeKind::G => {
            write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
            write!(f, "  setg %al\n").unwrap();
            write!(f, "  movzb %al, %{}\n", l).unwrap();
            return l;
        }
        NodeKind::GE => {
            write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
            write!(f, "  setge %al\n").unwrap();
            write!(f, "  movzb %al, %{}\n", l).unwrap();
            return l;
        }
        _ => unimplemented!(),
    }
}
