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
    pub lah: u8,
    pub assign_i: u8,
}

impl Femitter {
    pub fn new() -> Self {
        Femitter {
            rc: 1,
            vr: vec![],
            hm: FxHashMap::default(),
            lah: 0,
            assign_i: 0,
        }
    }
}

impl Program {
    pub fn gen_llvm_ir(&self) {
        const DIR: &str = "workspace/tmp.ll";
        fs::File::create(DIR).unwrap();
        fs::remove_file(DIR).unwrap();
        let mut f = fs::File::create(DIR).unwrap();

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

        if naf.ty.to_owned().pop().unwrap() == RetTy::Int32 {
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
            NodeKind::Num(_)
            | NodeKind::UnderScore
            | NodeKind::NewVar(_)
            | NodeKind::ReAssignVar(_)
            | NodeKind::GVar(_)
            | NodeKind::LVar(_)
            | NodeKind::If
            | NodeKind::While
            | NodeKind::LBrace => match ns.c.value {
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
                NodeKind::If => {
                    let if_stmts: NodeArr = *ns.to_owned().if_stmts.to_owned().unwrap();
                    let mut iif_stmts = if_stmts.node_st_vec.iter().peekable();
                    let else_if_stmts: NodeArr = *ns.to_owned().else_if_stmts.to_owned().unwrap();
                    let mut ielse_if_stmts = else_if_stmts.node_st_vec.iter().peekable();

                    let retf = if if_stmts.has_ret() {
                        write!(f, "  %{} = alloca i32, align 4\n", self.rc).unwrap();
                        true
                    } else {
                        false
                    };

                    if retf {
                        self.assign_i = self.rc;
                        self.rc += 1;
                    }

                    self.emitter(f, ns.to_owned().cond.unwrap().as_ref().to_owned());
                    write!(f, "  %{} = icmp ne i32 %{}, 0\n", self.rc, self.rc - 1).unwrap();

                    while iif_stmts.peek() != None {
                        self.calc_label(iif_stmts.next().unwrap().to_owned())
                    }

                    let stmt_lah = self.rc + self.lah + 2;
                    self.lah = 0;
                    write!(
                        f,
                        "  br i1 %{}, label %{}, label %{}\n",
                        self.rc,
                        self.rc + 1,
                        stmt_lah
                    )
                    .unwrap();
                    write!(f, "\n{}:\n", self.rc + 1).unwrap();
                    self.rc += 2;

                    let mut iif_stmts = if_stmts.node_st_vec.iter().peekable();
                    while iif_stmts.peek() != None {
                        self.emitter(f, iif_stmts.next().unwrap().to_owned())
                    }

                    while ielse_if_stmts.peek() != None {
                        self.calc_label(ielse_if_stmts.next().unwrap().to_owned())
                    }

                    let melse_stmt_lah = self.rc + self.lah + 1;
                    if retf {
                        write!(
                            f,
                            "  store i32 %{}, i32* %{}, align 4\n",
                            self.rc - 1,
                            self.assign_i
                        )
                        .unwrap();
                    }

                    self.rc += 1;
                    self.lah = 0;

                    write!(f, "  br label %{}", melse_stmt_lah).unwrap();
                    write!(f, "\n{}:\n", stmt_lah).unwrap();

                    let mut ielse_if_stmts = else_if_stmts.node_st_vec.iter().peekable();
                    while ielse_if_stmts.peek() != None {
                        self.emitter(f, ielse_if_stmts.next().unwrap().to_owned())
                    }

                    if !else_if_stmts.is_default() {
                        if retf {
                            write!(
                                f,
                                "  store i32 %{}, i32* %{}, align 4\n",
                                self.rc - 1,
                                self.assign_i
                            )
                            .unwrap();
                        }
                    }

                    self.rc += 1;

                    write!(f, "  br label %{}\n", melse_stmt_lah).unwrap();

                    write!(f, "\n{}:\n", melse_stmt_lah).unwrap();

                    if retf {
                        write!(
                            f,
                            "  %{} = load i32, i32* %{}, align 4\n",
                            self.rc, self.assign_i
                        )
                        .unwrap();
                        self.rc += 1;
                    }

                    return ();
                }
                NodeKind::While => {
                    let stmts: NodeArr = *ns.to_owned().stmts.to_owned().unwrap();
                    let mut istmts = stmts.node_st_vec.iter().peekable();

                    let retf = if stmts.has_ret() {
                        write!(f, "  %{} = alloca i32, align 4\n", self.rc).unwrap();
                        true
                    } else {
                        false
                    };

                    if retf {
                        self.assign_i = self.rc;
                        self.rc += 1;
                    }

                    let condl = self.rc;
                    write!(f, "  br label %{}\n", condl).unwrap();
                    self.rc += 1;

                    write!(f, "\n{}:\n", condl).unwrap();
                    self.emitter(f, ns.to_owned().cond.unwrap().as_ref().to_owned());
                    write!(f, "  %{} = icmp ne i32 %{}, 0\n", self.rc, self.rc - 1).unwrap();
                    while istmts.peek() != None {
                        self.calc_label(istmts.next().unwrap().to_owned())
                    }
                    let stmt_lah = self.rc + self.lah + 2;
                    write!(
                        f,
                        "  br i1 %{}, label %{}, label %{}\n",
                        self.rc,
                        self.rc + 1,
                        stmt_lah,
                    )
                    .unwrap();

                    write!(f, "\n{}:\n", self.rc + 1).unwrap();
                    self.rc += 2;
                    let mut istmts = stmts.node_st_vec.iter().peekable();
                    while istmts.peek() != None {
                        self.emitter(f, istmts.next().unwrap().to_owned())
                    }

                    if retf {
                        write!(
                            f,
                            "  store i32 %{}, i32* %{}, align 4\n",
                            self.rc - 1,
                            self.assign_i
                        )
                        .unwrap();
                    }

                    self.rc += 1;

                    write!(f, "  br label %{}\n", condl).unwrap();
                    write!(f, "\n{}:\n", stmt_lah).unwrap();

                    if retf {
                        write!(
                            f,
                            "  %{} = load i32, i32* %{}, align 4\n",
                            self.rc, self.assign_i
                        )
                        .unwrap();
                        self.rc += 1;
                    }

                    return ();
                }
                NodeKind::LBrace => {
                    let stmts: NodeArr = *ns.to_owned().stmts.to_owned().unwrap();
                    let mut istmts = stmts.node_st_vec.iter().peekable();

                    while istmts.peek() != None {
                        self.emitter(f, istmts.next().unwrap().to_owned())
                    }
                    return ();
                }
                _ => unreachable!(),
            },
            _ => (),
        }

        self.emitter(f, ns.to_owned().lhs.unwrap().as_ref().to_owned());
        self.emitter(f, ns.to_owned().rhs.unwrap().as_ref().to_owned());

        let rr = self.vr.pop().unwrap();
        let lr = self.vr.pop().unwrap();

        match ns.c.value {
            NodeKind::Add
            | NodeKind::Sub
            | NodeKind::Mul
            | NodeKind::Div
            | NodeKind::Sur
            | NodeKind::E
            | NodeKind::NE
            | NodeKind::L
            | NodeKind::LE
            | NodeKind::G
            | NodeKind::GE => match ns.c.value {
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
                            write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc)
                                .unwrap();
                        }
                        NodeKind::NE => {
                            write!(f, "  %{} = icmp ne i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                            write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc)
                                .unwrap();
                        }
                        NodeKind::L => {
                            write!(f, "  %{} = icmp slt i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                            write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc)
                                .unwrap();
                        }
                        NodeKind::LE => {
                            write!(f, "  %{} = icmp sle i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                            write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc)
                                .unwrap();
                        }
                        NodeKind::G => {
                            write!(f, "  %{} = icmp sgt i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                            write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc)
                                .unwrap();
                        }
                        NodeKind::GE => {
                            write!(f, "  %{} = icmp sge i32 %{}, %{}\n", self.rc, lr, rr).unwrap();
                            write!(f, "  %{} = zext i1 %{} to i32\n", self.rc + 1, self.rc)
                                .unwrap();
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
                _ => unreachable!(),
            },
            _ => unimplemented!(),
        }
    }

    pub fn calc_label(&mut self, ns: NodeSt) {
        // println!("ns.c.value: {:?}", ns.c.value);
        match ns.c.value {
            NodeKind::Num(_)
            | NodeKind::UnderScore
            | NodeKind::NewVar(_)
            | NodeKind::ReAssignVar(_)
            | NodeKind::GVar(_)
            | NodeKind::LVar(_) => {
                match ns.c.value {
                    NodeKind::Num(_) => {
                        self.lah += 2;
                    }
                    NodeKind::UnderScore => {}
                    NodeKind::NewVar(_) => {
                        self.lah += 1;
                        self.calc_label(ns.to_owned().rhs.unwrap().as_ref().to_owned());
                    }
                    NodeKind::ReAssignVar(_) => {
                        self.calc_label(ns.to_owned().rhs.unwrap().as_ref().to_owned());
                    }
                    NodeKind::GVar(_) => {
                        self.lah += 1;
                    }
                    NodeKind::LVar(_) => {
                        self.lah += 1;
                    }
                    _ => unreachable!(),
                }
                return ();
            }
            _ => (),
        }

        self.calc_label(ns.to_owned().lhs.unwrap().as_ref().to_owned());
        self.calc_label(ns.to_owned().rhs.unwrap().as_ref().to_owned());

        match ns.c.value {
            NodeKind::Add
            | NodeKind::Sub
            | NodeKind::Mul
            | NodeKind::Div
            | NodeKind::Sur
            | NodeKind::E
            | NodeKind::NE
            | NodeKind::L
            | NodeKind::LE
            | NodeKind::G
            | NodeKind::GE => match ns.c.value {
                NodeKind::Add => {
                    self.lah += 1;
                }
                NodeKind::Sub => {
                    self.lah += 1;
                }
                NodeKind::Mul => {
                    self.lah += 1;
                }
                NodeKind::Div => {
                    self.lah += 1;
                }
                NodeKind::Sur => {
                    self.lah += 1;
                }
                NodeKind::E
                | NodeKind::NE
                | NodeKind::L
                | NodeKind::LE
                | NodeKind::G
                | NodeKind::GE => {
                    self.lah += 4;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
