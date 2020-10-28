use super::super::node::node::*;
use super::super::node_arr::node_arr::*;
use rustc_hash::FxHashMap;
use std::fs;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Femitter {
    pub rc: u8,
    pub lr: u8,
    pub rr: u8,
    pub hm: FxHashMap<i32, u8>,
}

impl Femitter {
    pub fn new() -> Self {
        Femitter {
            rc: 1,
            lr: 0,
            rr: 0,
            hm: FxHashMap::default(),
        }
    }
}

impl NodeArr {
    pub fn gen_llvm_ir(&self) {
        const DIR: &str = "workspace/tmp.ll";
        fs::File::create(DIR).unwrap();
        fs::remove_file(DIR).unwrap();
        let mut f = fs::File::create(DIR).unwrap();

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

        let mut fm = Femitter::new();
        let mut nai = self.node_st_vec.iter().peekable();
        while nai.peek() != None {
            fm.emitter(&mut f, nai.next().unwrap().to_owned())
        }

        if self.ty == RetTy::Int32 {
            write!(f, "  %{} = add nsw i32 %{}, 0\n", fm.rc, fm.rc - 1).unwrap();
            write!(f, "  ret i32 %{}\n", fm.rc).unwrap();
        } else {
            write!(f, "  ret i32 0\n").unwrap();
        }
        write!(f, "}}").unwrap();
    }
}

impl Femitter {
    pub fn fgen(&mut self, f: &mut fs::File, ns: NodeSt) {
        self.emitter(f, ns)
    }
    pub fn emitter(&mut self, f: &mut fs::File, ns: NodeSt) {
        match ns.c.value {
            NodeKind::Num(n) => {
                write!(f, "  %{} = alloca i32, align 4\n", self.rc).unwrap();
                write!(f, "  store i32 {}, i32* %{}, align 4\n", n, self.rc).unwrap();
                write!(
                    f,
                    "  %{} = load i32, i32* %{}, align 4\n",
                    self.rc + 1,
                    self.rc
                )
                .unwrap();

                self.rr = self.rc;
                self.rc += 2;
                return ();
            }
            NodeKind::UnderScore => {
                return ();
            }
            NodeKind::NewVar(i) => {
                self.emitter(f, ns.to_owned().rhs.unwrap().as_ref().to_owned());
                self.hm.insert(i, self.rc);
                write!(f, "  %{} = alloca i32, align 4\n", self.rc).unwrap();
                write!(
                    f,
                    "  store i32 %{}, i32* %{}, align 4\n",
                    self.rc - 1,
                    self.rc
                )
                .unwrap();
                self.rc += 1;
                return ();
            }
            NodeKind::Var(i) => {
                write!(
                    f,
                    "  %{} = load i32, i32* %{}, align 4\n",
                    self.rc,
                    self.hm.get(&i).unwrap()
                )
                .unwrap();
                self.rc += 1;
                return ();
            }
            _ => (),
        }

        let _l = self.emitter(f, ns.lhs.unwrap().as_ref().to_owned());
        let _r = self.emitter(f, ns.rhs.unwrap().as_ref().to_owned());

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
}
