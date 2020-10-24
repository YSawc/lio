use super::super::node::node::*;
use super::super::node_arr::node_arr::*;
use std::fs;
use std::io::Write;

static mut CC: u8 = 0;

pub fn gen_ll(na: NodeArr) {
    const DIR: &str = "workspace/tmp.ll";
    fs::File::create(DIR).unwrap();
    fs::remove_file(DIR).unwrap();
    let mut f = fs::File::create(DIR).unwrap();

    unsafe {
        CC += 1;
    }

    write!(f, "%FILE = type opaque\n").unwrap();
    write!(f, "\n").unwrap();
    write!(
        f,
        "@str = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\", align 1\n"
    )
    .unwrap();
    write!(f, "declare i32 @fprintf(%FILE*, i8*, ...)\n").unwrap();
    write!(f, "declare i32 @printf(i8*, ...)\n").unwrap();
    write!(f, "declare i32 @atoi(...)\n").unwrap();
    write!(f, "define i32 @print(i32) nounwind {{\n").unwrap();
    write!(f, "  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @str, i64 0, i64 0), i32 %0)\n").unwrap();
    write!(f, "  ret i32 %0\n").unwrap();
    write!(f, "}}\n").unwrap();
    write!(f, "define i32 @main() nounwind {{\n").unwrap();
    let mut nai = na.node_st_vec.iter().peekable();

    println!("nai: {:?}", nai);

    while nai.peek() != None {
        gen_llvm_ir(&mut f, nai.next().unwrap().to_owned());
    }

    unsafe {
        write!(f, "  %{} = load i32, i32* %{}, align 4", CC, CC - 1).unwrap();
        write!(f, "  ret i32 %{}\n", CC).unwrap();
    }
    write!(f, "}}").unwrap();
}

fn gen_llvm_ir(f: &mut fs::File, ns: NodeSt) {
    match ns.c.value {
        NodeKind::Num(n) => {
            unsafe {
                write!(f, "  %{} = alloca i32, align 4\n", CC).unwrap();
                write!(f, "  store i32 {}, i32* %{}\n", n, CC).unwrap();
                CC += 1;
            }
            return ();
        }
        // NodeKind::Return => {
        //     unsafe {
        //         write!(f, "%{} = load i32, i32* %{}, align 4", CC + 1, CC).unwrap();
        //     }
        //     unsafe {
        //         CC += 1;
        //     }
        //     unsafe {
        //         write!(f, "ret i32 %{}", CC).unwrap();
        //     }
        //     return ();
        // }
        // NodeKind::UnderScore => {
        //     unsafe { write!(f, "  mov $0, %{}\n", REGS[CC as usize]).unwrap() };
        //     return ();
        // }
        _ => (),
    }

    let _l = gen_llvm_ir(f, ns.lhs.unwrap().as_ref().to_owned());
    let _r = gen_llvm_ir(f, ns.rhs.unwrap().as_ref().to_owned());

    match ns.c.value {
        //     NodeKind::Add => {
        //         write!(f, "  add %{}, %{}\n", r, l).unwrap();
        //         return ();
        //     }
        //     NodeKind::Sub => {
        //         write!(f, "  sub %{}, %{}\n", r, l).unwrap();
        //         return ();
        //     }
        //     NodeKind::Mul => {
        //         write!(f, "  imul %{}, %{}\n", r, l).unwrap();
        //         return ();
        //     }
        //     NodeKind::Div => {
        //         write!(f, "  mov %{}, %rax\n", l).unwrap();
        //         write!(f, "  cqo\n").unwrap();
        //         write!(f, "  idiv %{}\n", r).unwrap();
        //         write!(f, "  mov %rax, %{}\n", l).unwrap();
        //         return ();
        //     }
        //     NodeKind::Sur => {
        //         write!(f, "  mov %{}, %rax\n", l).unwrap();
        //         write!(f, "  cqo\n").unwrap();
        //         write!(f, "  idiv %{}\n", r).unwrap();
        //         write!(f, "  mov %rdx, %{}\n", l).unwrap();
        //         return ();
        //     }
        //     NodeKind::E => {
        //         write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
        //         write!(f, "  sete %al\n").unwrap();
        //         write!(f, "  movzb %al, %{}\n", l).unwrap();
        //         return ();
        //     }
        //     NodeKind::NE => {
        //         write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
        //         write!(f, "  setne %al\n").unwrap();
        //         write!(f, "  movzb %al, %{}\n", l).unwrap();
        //         return ();
        //     }
        //     NodeKind::L => {
        //         write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
        //         write!(f, "  setl %al\n").unwrap();
        //         write!(f, "  movzb %al, %{}\n", l).unwrap();
        //         return ();
        //     }
        //     NodeKind::LE => {
        //         write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
        //         write!(f, "  setle %al\n").unwrap();
        //         write!(f, "  movzb %al, %{}\n", l).unwrap();
        //         return l;
        //     }
        //     NodeKind::G => {
        //         write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
        //         write!(f, "  setg %al\n").unwrap();
        //         write!(f, "  movzb %al, %{}\n", l).unwrap();
        //         return l;
        //     }
        //     NodeKind::GE => {
        //         write!(f, "  cmp %{}, %{}\n", r, l).unwrap();
        //         write!(f, "  setge %al\n").unwrap();
        //         write!(f, "  movzb %al, %{}\n", l).unwrap();
        //         return l;
        //     }
        _ => unimplemented!(),
    }
}
