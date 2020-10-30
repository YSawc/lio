use super::super::node::node::*;
use super::super::node_arr::node_arr::*;
use super::super::program::program::*;
use rustc_hash::FxHashMap;
use std::fs;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Femitter {
    pub rc: u8,
    pub vr: Vec<u8>,
    pub hm: FxHashMap<i32, u8>,
}

impl Femitter {
    pub fn new() -> Self {
        Femitter {
            rc: 1,
            vr: vec![],
            hm: FxHashMap::default(),
        }
    }
}

impl Program {
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
        write!(f, "}}\n\n").unwrap();

        let mut gi = self.g.iter().peekable();
        while gi.peek() != None {
            let g = gi.next().unwrap();
            write!(
                f,
                "@{} = dso_local global i32 {}, align 4\n\n",
                g.s,
                g.n.get_num(),
            )
            .unwrap();
        }

        write!(f, "define i32 @main() nounwind {{\n").unwrap();

        let mut fm = Femitter::new();
        let naf = self.na.iter().next().unwrap();
        let mut nai = naf.node_st_vec.iter().peekable();
        while nai.peek() != None {
            fm.fgen(&mut f, nai.next().unwrap().to_owned())
        }

        if naf.ty == RetTy::Int32 {
            write!(f, "  %{} = add nsw i32 %{}, 0\n", fm.rc, fm.rc - 1).unwrap();
            write!(f, "  ret i32 %{}\n", fm.rc).unwrap();
        } else {
            write!(f, "  ret i32 0\n").unwrap();
        }
        write!(f, "}}").unwrap();
    }
}

impl Femitter {
    pub fn ggen(&mut self, f: &mut fs::File, ns: NodeSt) {
        self.emitter(f, ns)
    }

    pub fn gemitter(&mut self, f: &mut fs::File, ns: NodeSt) {
        match ns.c.value {
            NodeKind::GVar(s) => {
                write!(f, "  @{} = dso_local global i32 {}, align 4\n", self.rc, s).unwrap();
                self.rc += 1;
                return ();
            }
            _ => unimplemented!(),
        }
    }

    pub fn fgen(&mut self, f: &mut fs::File, ns: NodeSt) {
        self.emitter(f, ns)
    }

    pub fn emitter(&mut self, f: &mut fs::File, ns: NodeSt) {
        // println!("ns.c.value: {:?}", ns.c.value);
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
                self.vr.push(self.rc + 1);
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
            NodeKind::ReAssignVar(i) => {
                self.emitter(f, ns.to_owned().rhs.unwrap().as_ref().to_owned());
                write!(
                    f,
                    "  store i32 %{}, i32* %{}, align 4\n",
                    self.rc - 1,
                    self.hm.get(&i).unwrap()
                )
                .unwrap();
                return ();
            }
            NodeKind::GVar(s) => {
                write!(f, "  %{} = load i32, i32* @{}, align 4\n", self.rc, s).unwrap();
                self.vr.push(self.rc);
                self.rc += 1;
                return ();
            }
            NodeKind::LVar(i) => {
                write!(
                    f,
                    "  %{} = load i32, i32* %{}, align 4\n",
                    self.rc,
                    self.hm.get(&i).unwrap()
                )
                .unwrap();
                self.vr.push(self.rc);
                self.rc += 1;
                return ();
            }
            _ => (),
        }

        self.emitter(f, ns.to_owned().lhs.unwrap().as_ref().to_owned());
        self.emitter(f, ns.to_owned().rhs.unwrap().as_ref().to_owned());

        let rr = self.vr.pop().unwrap();
        let lr = self.vr.pop().unwrap();

        match ns.c.value {
            NodeKind::Add => {
                write!(f, "  %{} = add nsw i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                self.vr.push(self.rc);
                self.rc += 1;
            }
            NodeKind::Sub => {
                write!(f, "  %{} = sub nsw i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                self.vr.push(self.rc);
                self.rc += 1;
            }
            NodeKind::Mul => {
                write!(f, "  %{} = mul nsw i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                self.vr.push(self.rc);
                self.rc += 1;
            }
            NodeKind::Div => {
                write!(f, "  %{} = sdiv i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                self.vr.push(self.rc);
                self.rc += 1;
            }
            NodeKind::Sur => {
                write!(f, "  %{} = srem i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                self.vr.push(self.rc);
                self.rc += 1;
            }
            NodeKind::E
            | NodeKind::NE
            | NodeKind::L
            | NodeKind::LE
            | NodeKind::G
            | NodeKind::GE => {
                match ns.c.value {
                    NodeKind::E => {
                        write!(f, "  %{} = icmp eq i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                        write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc).unwrap();
                    }
                    NodeKind::NE => {
                        write!(f, "  %{} = icmp ne i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                        write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc).unwrap();
                    }
                    NodeKind::L => {
                        write!(f, "  %{} = icmp slt i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                        write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc).unwrap();
                    }
                    NodeKind::LE => {
                        write!(f, "  %{} = icmp sle i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                        write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc).unwrap();
                    }
                    NodeKind::G => {
                        write!(f, "  %{} = icmp sgt i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                        write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc).unwrap();
                    }
                    NodeKind::GE => {
                        write!(f, "  %{} = icmp sge i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                        write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc).unwrap();
                    }
                    _ => unreachable!(),
                }
                write!(f, "  %{} = alloca i32, align 4\n", self.rc + 2).unwrap();
                write!(
                    f,
                    "  store i32 %{}, i32* %{}, align 4\n",
                    self.rc + 1,
                    self.rc + 2
                )
                .unwrap();
                write!(
                    f,
                    "  %{} = load i32, i32* %{}, align 4\n",
                    self.rc + 3,
                    self.rc + 2
                )
                .unwrap();
                self.vr.push(self.rc + 3);
                self.rc += 4;
            }
            _ => unimplemented!(),
        }
    }
}
