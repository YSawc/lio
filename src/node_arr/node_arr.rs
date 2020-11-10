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
    pub used_variable: Vec<String>,
    pub ret_n: NodeSt,
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
            used_variable: vec![],
            ret_n: NodeSt::default(),
        }
    }

    pub fn set_node(&mut self, v: Vec<NodeSt>) {
        self.node_st_vec = v.to_owned();
        self.ret_node_st = v.last().unwrap().to_owned();
    }

    pub fn set_end_of_node(&mut self) {
        self.end_of_node = true
    }

    pub fn set_env(&mut self, v: Vec<Vec<Var>>) {
        self.env_v = v;
    }

    pub fn set_imm_env(&mut self) {
        self.imm_env_v = self.env_v.to_owned();
        self.imm_env_v.push(self.l.to_owned());
    }

    pub fn set_ret_node(&mut self, v: NodeSt) {
        self.ret_node_st = v;
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

        let isi = {
            match it.peek_value() {
                TokenKind::Int => {
                    it.next_with_shadow();
                    true
                }
                _ => false,
            }
        };

        let mut ev: Vec<Vec<Var>> = vec![];
        ev.push(g.to_owned());

        let (mut l, ugv) = Self::parse_statement(&mut it, ev)?;

        if isi {
            l.ty = RetTy::Int32;
        } else {
            l.ty = RetTy::Void;
        }

        if isi {
            match NodeSt::isi(l.to_owned().ret_node_st) {
                true => return Ok((l, ugv)),
                false => {
                    return Err(ParseError::NotMatchReturnType(
                        l.to_owned().ret_node_st.c.loc,
                    ))
                }
            }
        } else {
            match l.ret_node_st.c.value {
                NodeKind::UnderScore => return Ok((l, ugv)),
                _ => {
                    return Err(ParseError::NotMatchReturnType(
                        l.to_owned().ret_node_st.c.loc,
                    ))
                }
            }
        }
    }

    pub fn parse_statement(
        it: &mut TokenIter,
        ev: Vec<Vec<Var>>,
    ) -> Result<(Self, Vec<String>), ParseError> {
        it.expect_token(
            TokenKind::LBrace,
            ParseError::NotOpenedStmt(it.p.to_owned().peek().unwrap().to_owned().to_owned()),
        )?;

        let (a, ugv) = Self::parse_internal_statement(it, ev)?;

        it.expect_token(
            TokenKind::RBrace,
            ParseError::NotOpenedStmt(it.p.to_owned().peek().unwrap().to_owned().to_owned()),
        )?;

        Ok((a, ugv))
    }

    pub fn parse_internal_statement(
        it: &mut TokenIter,
        ev: Vec<Vec<Var>>,
    ) -> Result<(Self, Vec<String>), ParseError> {
        let mut a = Self::new();
        a.set_env(ev);
        let mut aln: i32 = 0;
        while it.p.peek() != None && a.end_of_node == false {
            match NodeSt::parser(it) {
                Ok(n) => match n.c.value {
                    NodeKind::Return
                    | NodeKind::NewAssign
                    | NodeKind::Assign
                    | NodeKind::UnderScore
                    | NodeKind::If => {
                        // println!("n.c.value: {:?}", n.c.value);
                        match n.c.value {
                            NodeKind::Return => {
                                if it.peek_value() != TokenKind::RBrace {
                                    return Err(ParseError::OperatorAfterRetrun(it.next()));
                                }

                                a.set_end_of_node();
                                a.set_imm_env();
                                let r = beta(&mut n.to_owned().lhs.unwrap().to_owned(), &mut a)?;
                                a.set_ret_node(r.to_owned());
                                a.node_st_vec.push(r.to_owned());
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
                                match Program::find_v(_s.to_owned(), a.imm_env_v.to_owned()) {
                                    Some(f) => {
                                        match a.used_variable.contains(&f.to_owned().s.to_owned()) {
                                            true => (),
                                            false => {
                                                return Err(ParseError::UnusedVariable(f.n.c.loc))
                                            }
                                        }
                                        a.used_variable.retain(|s| s != &f.to_owned().s.to_owned());
                                        let mut lhs = beta(
                                            &mut n.to_owned().rhs.unwrap().to_owned(),
                                            &mut a,
                                        )?;
                                        lhs = lhs.simplified();

                                        let v = match _s.as_bytes()[0] {
                                            b'_' => Var::mnew(_s, n.to_owned(), aln),
                                            _ => Var::new_l(_s, n.to_owned(), aln),
                                        };
                                        if v.to_owned().m {
                                            a.used_variable.push(v.to_owned().s);
                                        }
                                        a.l.push(v.to_owned());

                                        let avar = NodeSt::ass_var(v.to_owned().aln, lhs, n.c.loc);
                                        a.node_st_vec.push(avar);
                                    }
                                    None => {
                                        let mut lhs = beta(
                                            &mut n.to_owned().rhs.unwrap().to_owned(),
                                            &mut a,
                                        )?;
                                        lhs = lhs.simplified();
                                        aln += 1;
                                        let v = match _s.as_bytes()[0] {
                                            b'_' => Var::mnew(_s, n.to_owned(), aln),
                                            _ => Var::new_l(_s, n.to_owned(), aln),
                                        };
                                        if v.to_owned().m {
                                            a.used_variable.push(v.to_owned().s);
                                        }
                                        a.l.push(v.to_owned());

                                        let avar = NodeSt::ass_var(v.to_owned().aln, lhs, n.c.loc);
                                        a.node_st_vec.push(avar);
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
                                match Program::find_v(_s.to_owned(), a.imm_env_v.to_owned()) {
                                    Some(mut f) => {
                                        // println!("uv: {:?}", uv);
                                        match a.used_variable.contains(&f.to_owned().s.to_owned()) {
                                            true => (),
                                            false => {
                                                return Err(ParseError::UnusedVariable(f.n.c.loc))
                                            }
                                        }
                                        let mut lhs = beta(
                                            &mut n.to_owned().rhs.unwrap().to_owned(),
                                            &mut a,
                                        )?;
                                        lhs = lhs.simplified();
                                        f.n = n.to_owned();
                                        let ff = f.to_owned();
                                        a.l.retain(|s| s.s != _s.to_owned());
                                        a.l.push(ff);
                                        let rvar = NodeSt::r_var(
                                            f.to_owned().aln,
                                            lhs,
                                            n.c.loc.to_owned(),
                                        );
                                        a.node_st_vec.push(rvar);
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
                                a.set_ret_node(n.to_owned());
                                a.node_st_vec.push(n);
                            }
                            NodeKind::If => {
                                if it.p.peek() == None {
                                    return Err(ParseError::Eof);
                                }

                                if it.peek_value() == TokenKind::RBrace {
                                    a.set_end_of_node()
                                }

                                a.set_imm_env();

                                let if_stmts = Self::parse_statement(it, a.imm_env_v.to_owned())?.0;

                                let mut _else_if_stmts: NodeArr = NodeArr::new();
                                match it.p.peek().unwrap().value {
                                    TokenKind::Else => {
                                        it.next();
                                        _else_if_stmts =
                                            Self::parse_statement(it, a.imm_env_v.to_owned())?.0;
                                    }
                                    _ => (),
                                };

                                let if_stmts_isi = NodeSt::isi(if_stmts.ret_node_st.to_owned());
                                let else_if_stmts_isi =
                                    NodeSt::isi(_else_if_stmts.ret_node_st.to_owned());

                                if if_stmts_isi != else_if_stmts_isi {
                                    return Err(ParseError::NotMatchTypeAnotherOneOfStatement(
                                        it.p.peek().unwrap().loc.to_owned(),
                                    ));
                                }

                                use std::env;
                                let args: Vec<String> = env::args().collect();
                                let mut calc_if_label = false;
                                if args.len() > 3 {
                                    if args[3] == "calc_if_label" {
                                        calc_if_label = true;
                                    }
                                }

                                match calc_if_label {
                                    false => {
                                        let mut n = n.to_owned();
                                        n.if_stmts = Some(Box::new(if_stmts.to_owned()));
                                        n.else_if_stmts = Some(Box::new(_else_if_stmts.to_owned()));
                                        a.node_st_vec.push(n);
                                    }
                                    true => {
                                        let mut c = beta(&mut n.to_owned().cond.unwrap(), &mut a)?;
                                        c = c.simplified();

                                        match c.c.value {
                                            NodeKind::Num(num) => {
                                                if num == 0 {
                                                    if _else_if_stmts.node_st_vec != vec![] {
                                                        a.node_st_vec.append(
                                                            &mut _else_if_stmts
                                                                .node_st_vec
                                                                .to_owned(),
                                                        );
                                                    } else {
                                                        continue;
                                                    }
                                                }

                                                a.node_st_vec
                                                    .append(&mut if_stmts.node_st_vec.to_owned());
                                            }
                                            _ => {
                                                a.set_imm_env();
                                                let n = beta(&mut n.to_owned(), &mut a)?;
                                                match n.to_owned().cond.unwrap().c.value {
                                                    NodeKind::Ident(s) => {
                                                        a.set_imm_env();
                                                        match Program::find_v(
                                                            s.to_owned(),
                                                            a.imm_env_v.to_owned(),
                                                        ) {
                                                            Some(mut v) => {
                                                                if it.peek_value()
                                                                    == TokenKind::RBrace
                                                                {
                                                                    a.set_end_of_node()
                                                                }

                                                                if !a.used_variable.contains(
                                                                    &v.to_owned().s.to_owned(),
                                                                ) {
                                                                    a.used_variable.push(
                                                                        v.to_owned().s.to_owned(),
                                                                    );
                                                                    v.aln = aln;
                                                                    aln += 1;
                                                                }

                                                                if a.end_of_node {
                                                                    a.set_ret_node(
                                                                        v.to_owned().n.to_owned(),
                                                                    );
                                                                }

                                                                let mut n = match v.gf {
                                                                    true => NodeSt::g_var(
                                                                        s,
                                                                        n.to_owned().c.loc,
                                                                    ),
                                                                    false => NodeSt::l_var(
                                                                        v.aln, n.c.loc,
                                                                    ),
                                                                };
                                                                n.cond =
                                                                    Some(Box::new(n.to_owned()));
                                                                a.node_st_vec.push(n);
                                                            }
                                                            _ => unimplemented!(),
                                                        }
                                                    }
                                                    _ => unimplemented!(),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => {
                        a.set_imm_env();
                        let n = beta(&mut n.to_owned(), &mut a)?;
                        a.node_st_vec.push(n.to_owned());

                        match it.check_evaluate_type() {
                            true => {
                                a.set_end_of_node();
                                a.set_ret_node(n.to_owned());
                            }
                            false => match it.check_evaluate_void() {
                                true => {
                                    a.set_end_of_node();
                                    a.set_ret_node(NodeSt::default());
                                }
                                false => (),
                            },
                        }
                    }
                },
                Err(e) => return Err(e),
            };
        }

        println!("a.used_variable: {:?}", a.used_variable);

        for l in a.to_owned().l {
            match a.used_variable.contains(&l.to_owned().s.to_owned()) {
                true => (),
                false => return Err(ParseError::UnusedVariable(l.n.c.loc)),
            }
        }
        // println!("ret_node_st: {:?}", a.ret_node_st);

        let mut ugv: Vec<String> = vec![];
        for evc in a.env_v.to_owned() {
            for v in evc {
                // println!("v: {:?}", v);
                match a.used_variable.contains(&v.to_owned().s.to_owned()) {
                    true => ugv.push(v.to_owned().s.to_owned()),
                    false => (),
                }
            }
        }
        Ok((a, ugv))
    }
}
