use super::super::node::node::*;
use super::super::parser::error::*;
use super::super::program::program::*;
use super::super::simplified::beta::*;
use super::super::token::token::*;
use super::super::var::var::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetTy {
    Int32,
    Void,
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeArr {
    pub node_st_vec: Vec<NodeSt>,
    pub ty: RetTy,
    pub ret_node_st: NodeSt,
    pub l: Vec<Var>,
    pub env_v: Vec<Vec<Var>>,
    pub imm_env_v: Vec<Vec<Var>>,
    pub end_of_node: bool,
}

impl NodeArr {
    pub fn find_l(s: String, vv: Vec<Var>) -> Option<Var> {
        for v in vv {
            if v.s == s {
                return Some(v);
            }
        }
        None
    }
}

impl NodeArr {
    pub fn new() -> Self {
        Self {
            node_st_vec: vec![],
            ty: RetTy::Default,
            ret_node_st: NodeSt::default(),
            l: vec![],
            env_v: vec![],
            imm_env_v: vec![],
            end_of_node: false,
        }
    }

    pub fn set_node(&mut self, v: Vec<NodeSt>) {
        self.node_st_vec = v.to_owned();
        self.ret_node_st = v.to_owned().last().unwrap().to_owned();
    }

    pub fn set_end_of_node(&mut self) {
        self.end_of_node = true
    }

    pub fn set_env(&mut self, v: Vec<Vec<Var>>) {
        self.env_v = v;
    }

    pub fn set_imm_env(&mut self) {
        self.imm_env_v = self.env_v.to_owned();
    }

    pub fn add_imm_env(&mut self, v: Vec<Var>) {
        self.imm_env_v.push(v);
    }
}

impl NodeArr {
    pub fn w_parser(
        mut it: &mut TokenIter,
        g: Vec<Var>,
    ) -> Result<(Self, Vec<String>), ParseError> {
        it.copy_iter();
        if it.peek_value() != TokenKind::Fn {
            return Err(ParseError::OperatorOutOfFnction(it.peek_shadow()));
        }

        it.next_with_shadow();

        let mut isi: bool = false;
        if it.peek_value() == TokenKind::Int {
            it.shadow_p.to_owned().next();
            it.p.next();
            isi = true;
        }

        if it.p.peek() == None {
            return Err(ParseError::NotLBrace(it.peek_shadow()));
        }
        it.p.next();

        let mut ev: Vec<Vec<Var>> = vec![];
        ev.push(g.to_owned());
        let (mut l, ugv) = Self::stp(&mut it, ev)?;

        if isi {
            l.ty = RetTy::Int32;
        } else {
            l.ty = RetTy::Void;
        }

        if isi {
            match l.to_owned().to_owned().ret_node_st.c.value {
                NodeKind::Num(_)
                | NodeKind::Add
                | NodeKind::Sub
                | NodeKind::Mul
                | NodeKind::Div
                | NodeKind::E
                | NodeKind::NE
                | NodeKind::L
                | NodeKind::LE
                | NodeKind::G
                | NodeKind::GE
                | NodeKind::NewAssignG
                | NodeKind::NewAssignL
                | NodeKind::NewAssign
                | NodeKind::Assign
                | NodeKind::Ident(_)
                | NodeKind::GVar(_)
                | NodeKind::LVar(_) => return Ok((l, ugv)),
                _ => {
                    return Err(ParseError::NotMatchReturnType(
                        l.to_owned().to_owned().ret_node_st.c.loc,
                    ))
                }
            }
        } else {
            match l.to_owned().to_owned().ret_node_st.c.value {
                NodeKind::UnderScore => return Ok((l, ugv)),
                _ => {
                    return Err(ParseError::NotMatchReturnType(
                        l.to_owned().to_owned().ret_node_st.c.loc,
                    ))
                }
            }
        }
    }

    pub fn stp(it: &mut TokenIter, ev: Vec<Vec<Var>>) -> Result<(Self, Vec<String>), ParseError> {
        let mut a = Self::new();
        a.set_env(ev.to_owned());
        let mut uv: Vec<String> = vec![];
        let mut nv = vec![];
        let mut l: Vec<Var> = vec![];
        let mut r: NodeSt = NodeSt::default();
        let mut aln: i32 = 0;
        while it.p.peek() != None && a.end_of_node == false {
            nv.push(match NodeSt::parser(it) {
                Ok(n) => match n.c.value {
                    NodeKind::Return
                    | NodeKind::NewAssign
                    | NodeKind::Assign
                    | NodeKind::UnderScore
                    | NodeKind::Ident(_)
                    | NodeKind::If => {
                        match n.c.value {
                            NodeKind::Return => {
                                if it.peek_value() != TokenKind::RBrace {
                                    return Err(ParseError::OperatorAfterRetrun(it.next()));
                                }

                                a.set_end_of_node();
                                a.set_imm_env();
                                a.add_imm_env(l.to_owned());
                                r = beta(
                                    &mut n.to_owned().lhs.unwrap().to_owned(),
                                    a.imm_env_v.to_owned(),
                                    &mut uv,
                                );
                                r.to_owned()
                            }
                            NodeKind::NewAssign => {
                                let mut _s = String::new();
                                match n.to_owned().lhs.unwrap().c.value {
                                    NodeKind::Ident(si) => _s = si,
                                    _ => {
                                        match n.to_owned().lhs.unwrap().lhs.unwrap().c.value {
                                            NodeKind::Ident(si) => _s = si,
                                            _ => unreachable!(),
                                        };
                                    }
                                }
                                a.set_imm_env();
                                a.add_imm_env(l.to_owned());
                                match Program::find_v(_s.to_owned(), a.imm_env_v.to_owned()) {
                                    Some(f) => {
                                        match uv.contains(&f.to_owned().s.to_owned()) {
                                            true => (),
                                            false => {
                                                return Err(ParseError::UnusedVariable(f.n.c.loc))
                                            }
                                        }
                                        uv.retain(|s| s != &f.to_owned().s.to_owned());
                                        let mut lhs = beta(
                                            &mut n.to_owned().rhs.unwrap().to_owned(),
                                            a.imm_env_v.to_owned(),
                                            &mut uv,
                                        );
                                        lhs = lhs.simplified();

                                        let v = match _s.as_bytes()[0] {
                                            b'_' => Var::mnew(_s, n.to_owned(), aln),
                                            _ => Var::new_l(_s, n.to_owned(), aln),
                                        };
                                        if v.to_owned().m {
                                            uv.push(v.to_owned().s);
                                        }
                                        l.push(v.to_owned());

                                        let avar = NodeSt::ass_var(v.to_owned().aln, lhs, n.c.loc);
                                        avar
                                    }
                                    None => {
                                        let mut lhs = beta(
                                            &mut n.to_owned().rhs.unwrap().to_owned(),
                                            a.imm_env_v.to_owned(),
                                            &mut uv,
                                        );
                                        lhs = lhs.simplified();
                                        aln += 1;
                                        let v = match _s.as_bytes()[0] {
                                            b'_' => Var::mnew(_s, n.to_owned(), aln),
                                            _ => Var::new_l(_s, n.to_owned(), aln),
                                        };
                                        if v.to_owned().m {
                                            uv.push(v.to_owned().s);
                                        }
                                        l.push(v.to_owned());

                                        let avar = NodeSt::ass_var(v.to_owned().aln, lhs, n.c.loc);
                                        avar
                                    }
                                }
                            }
                            NodeKind::Assign => {
                                let mut _s = String::new();
                                match n.to_owned().lhs.unwrap().c.value {
                                    NodeKind::Ident(si) => _s = si,
                                    _ => {
                                        match n.to_owned().lhs.unwrap().lhs.unwrap().c.value {
                                            NodeKind::Ident(si) => _s = si,
                                            _ => unreachable!(),
                                        };
                                    }
                                }

                                a.set_imm_env();
                                a.add_imm_env(l.to_owned());
                                match Program::find_v(_s.to_owned(), a.imm_env_v.to_owned()) {
                                    Some(mut f) => {
                                        // println!("uv: {:?}", uv);
                                        match uv.contains(&f.to_owned().s.to_owned()) {
                                            true => (),
                                            false => {
                                                return Err(ParseError::UnusedVariable(f.n.c.loc))
                                            }
                                        }
                                        let mut lhs = beta(
                                            &mut n.to_owned().rhs.unwrap().to_owned(),
                                            a.imm_env_v.to_owned(),
                                            &mut uv,
                                        );
                                        lhs = lhs.simplified();
                                        f.n = n.to_owned();
                                        let ff = f.to_owned();
                                        l.retain(|s| s.s != _s.to_owned());
                                        l.push(ff);
                                        let rvar = NodeSt::r_var(f.to_owned().aln, lhs, n.c.loc);
                                        rvar
                                    }
                                    _ => {
                                        return Err(ParseError::UndefinedVariable(
                                            n.to_owned().lhs.unwrap().c.loc,
                                        ))
                                    }
                                }
                            }
                            NodeKind::UnderScore => {
                                if it.peek_value() != TokenKind::RBrace {
                                    return Err(ParseError::UnexpectedUnderScoreOperator(
                                        n.to_owned().c.loc,
                                    ));
                                }
                                a.set_end_of_node();

                                let n = NodeSt::under_score(n.c.loc);
                                r = n.to_owned();
                                n
                            }
                            NodeKind::Ident(s) => {
                                a.set_imm_env();
                                a.add_imm_env(l.to_owned());
                                match Program::find_v(s.to_owned(), a.imm_env_v.to_owned()) {
                                    Some(mut v) => {
                                        if it.peek_value() == TokenKind::RBrace {
                                            a.set_end_of_node();
                                        }

                                        if !uv.contains(&v.to_owned().s.to_owned()) {
                                            uv.push(v.to_owned().s.to_owned());
                                            v.aln = aln;
                                            aln += 1;
                                        }

                                        if a.end_of_node {
                                            r = v.to_owned().n.to_owned();
                                        }

                                        let mut _n: NodeSt = NodeSt::default();
                                        if v.gf == 1 {
                                            _n = NodeSt::g_var(s, n.c.loc);
                                        } else {
                                            _n = NodeSt::l_var(v.aln, n.c.loc);
                                        }
                                        _n
                                    }
                                    None => return Err(ParseError::NotDefinitionVar(it.next())),
                                }
                            }
                            NodeKind::If => {
                                if it.p.peek() == None {
                                    return Err(ParseError::Eof);
                                }
                                if it.peek_value() == TokenKind::RBrace {
                                    a.set_end_of_node()
                                }

                                a.set_imm_env();
                                a.add_imm_env(l.to_owned());
                                let mut c = beta(
                                    &mut n.to_owned().cond.unwrap(),
                                    a.imm_env_v.to_owned(),
                                    &mut uv,
                                );
                                c = c.simplified();
                                match c.c.value {
                                    NodeKind::Num(num) => {
                                        if num == 0 {
                                            if n.to_owned().else_if_stmts != None {
                                                let (else_if_stmts, _) = NodeSt::statement_parser(
                                                    n.to_owned()
                                                        .else_if_stmts
                                                        .unwrap()
                                                        .as_ref()
                                                        .to_owned(),
                                                    a.imm_env_v.to_owned(),
                                                )?;

                                                if a.end_of_node {
                                                    r = else_if_stmts
                                                        .to_owned()
                                                        .last()
                                                        .unwrap()
                                                        .to_owned()
                                                }
                                                let mut n = n.to_owned();
                                                n.else_if_stmts = Some(Box::new(else_if_stmts));
                                                n
                                            } else {
                                                continue;
                                            }
                                        } else {
                                            let (if_stmts, _) = NodeSt::statement_parser(
                                                n.to_owned().if_stmts.unwrap().as_ref().to_owned(),
                                                a.imm_env_v.to_owned(),
                                            )?;
                                            if a.end_of_node {
                                                r = if_stmts.to_owned().last().unwrap().to_owned();
                                            }

                                            let mut n = n.to_owned();
                                            n.if_stmts = Some(Box::new(if_stmts));
                                            n
                                        }
                                    }
                                    _ => {
                                        a.set_imm_env();
                                        a.add_imm_env(l.to_owned());
                                        let n = beta(
                                            &mut n.to_owned(),
                                            a.imm_env_v.to_owned(),
                                            &mut uv,
                                        );
                                        match n.to_owned().cond.unwrap().c.value {
                                            NodeKind::Ident(s) => {
                                                a.set_imm_env();
                                                a.add_imm_env(l.to_owned());
                                                match Program::find_v(
                                                    s.to_owned(),
                                                    a.imm_env_v.to_owned(),
                                                ) {
                                                    Some(mut v) => {
                                                        if it.peek_value() == TokenKind::RBrace {
                                                            a.set_end_of_node()
                                                        }

                                                        if !uv.contains(&v.to_owned().s.to_owned())
                                                        {
                                                            uv.push(v.to_owned().s.to_owned());
                                                            v.aln = aln;
                                                            aln += 1;
                                                        }

                                                        if a.end_of_node {
                                                            r = v.to_owned().n.to_owned();
                                                        }

                                                        let mut _n: NodeSt = NodeSt::default();
                                                        if v.gf == 1 {
                                                            _n = NodeSt::g_var(
                                                                s,
                                                                n.to_owned().c.loc,
                                                            );
                                                        } else {
                                                            _n = NodeSt::l_var(
                                                                v.aln,
                                                                n.to_owned().c.loc,
                                                            );
                                                        }
                                                        let mut n = n.to_owned();
                                                        n.cond = Some(Box::new(_n));
                                                        n
                                                    }
                                                    _ => unimplemented!(),
                                                }
                                            }
                                            _ => unimplemented!(),
                                        }
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => {
                        if it.peek_value() == TokenKind::RBrace {
                            a.set_end_of_node()
                        }

                        a.set_imm_env();
                        a.add_imm_env(l.to_owned());
                        let n = beta(&mut n.to_owned(), a.imm_env_v.to_owned(), &mut uv);
                        if a.end_of_node {
                            r = n.to_owned();
                        }
                        // println!("r: {:?}", r);
                        n
                    }
                },
                Err(e) => return Err(e),
            });
        }

        a.set_node(nv);
        a.l = l;
        a.ret_node_st = r;
        println!("uv: {:?}", uv);

        for l in a.to_owned().l {
            match uv.contains(&l.to_owned().s.to_owned()) {
                true => (),
                false => return Err(ParseError::UnusedVariable(l.n.c.loc)),
            }
        }
        // println!("ret_node_st: {:?}", a.ret_node_st);

        let mut ugv: Vec<String> = vec![];
        for evc in a.env_v.to_owned() {
            for v in evc {
                // println!("v: {:?}", v);
                match uv.contains(&v.to_owned().s.to_owned()) {
                    true => ugv.push(v.to_owned().s.to_owned()),
                    false => (),
                }
            }
        }
        Ok((a, ugv))
    }
}

impl NodeSt {
    pub fn statement_parser(
        vn: Vec<NodeSt>,
        ev: Vec<Vec<Var>>,
    ) -> Result<(Vec<Self>, Vec<String>), ParseError> {
        let mut min = vn.iter().peekable();
        let mut uv: Vec<String> = vec![];
        let mut nv = vec![];
        let l: Vec<Var> = vec![];

        while min.to_owned().peek() != None {
            nv.push(match min.to_owned().peek().unwrap().c.value {
                _ => {
                    let mut ev = ev.to_owned();
                    ev.push(l.to_owned());
                    let n = beta(&mut min.next().unwrap().to_owned(), ev, &mut uv);
                    n
                }
            });
        }

        Ok((nv, uv))
    }
}
