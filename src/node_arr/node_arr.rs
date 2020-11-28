use super::super::location::location::*;
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

    pub fn is_default(self) -> bool {
        return if self.node_st_vec == vec![] {
            true
        } else {
            false
        };
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

    pub fn none_ret_node(&mut self) -> bool {
        self.ret_node_st == NodeSt::default()
    }

    pub fn set_ret_node(&mut self, v: NodeSt) {
        self.ret_node_st = v;
    }

    pub fn pop_node(&mut self) -> NodeSt {
        self.node_st_vec.pop().unwrap()
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

        let (mut l, uev) = Self::parse_statement(&mut it, ev)?;

        if isi {
            l.ty = RetTy::Int32;
        } else {
            l.ty = RetTy::Void;
        }

        if isi {
            match NodeSt::isi(l.to_owned().ret_node_st) {
                true => return Ok((l, uev)),
                false => {
                    return Err(ParseError::NotMatchReturnType(
                        l.to_owned().ret_node_st.c.loc,
                    ))
                }
            }
        } else {
            match l.ret_node_st.c.value {
                NodeKind::UnderScore => return Ok((l, uev)),
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

        let mut narr = Self::new();
        narr.set_env(ev);

        let uev = narr.parse_internal_statement(it)?;

        it.expect_token(
            TokenKind::RBrace,
            ParseError::NotClosedStmt(it.p.to_owned().peek().unwrap().to_owned().to_owned()),
        )?;

        Ok((narr, uev))
    }

    pub fn parse_internal_statement(
        &mut self,
        it: &mut TokenIter,
    ) -> Result<Vec<String>, ParseError> {
        let mut aln: i32 = 0;
        while it.p.peek() != None && self.end_of_node == false {
            match NodeSt::parser(it) {
                Ok(n) => match n.c.value {
                    NodeKind::Return
                    | NodeKind::NewAssign
                    | NodeKind::Assign
                    | NodeKind::UnderScore
                    | NodeKind::If
                    | NodeKind::While
                    | NodeKind::LBrace
                    | NodeKind::Pipe => {
                        // println!("n.c.value: {:?}", n.c.value);
                        match n.c.value {
                            NodeKind::Return => {
                                if it.peek_value() != TokenKind::RBrace {
                                    return Err(ParseError::OperatorAfterRetrun(it.next()));
                                }

                                self.set_end_of_node();
                                self.set_imm_env();
                                let r = beta(&mut n.to_owned().lhs.unwrap().to_owned(), self)?;
                                self.set_ret_node(r.to_owned());
                                self.node_st_vec.push(r.to_owned());
                            }
                            NodeKind::NewAssign => {
                                let sv = n
                                    .to_owned()
                                    .lhs
                                    .unwrap()
                                    .lhs
                                    .unwrap()
                                    .ret_set
                                    .unwrap()
                                    .contents;
                                self.set_imm_env();

                                let rhs = self.parse_close_imm(it)?;

                                match Program::find_v(sv[0].to_owned(), self.imm_env_v.to_owned()) {
                                    Some(f) => {
                                        match self
                                            .used_variable
                                            .contains(&f.to_owned().s.to_owned())
                                        {
                                            true => self
                                                .used_variable
                                                .retain(|s| s != &f.to_owned().s.to_owned()),
                                            false => {
                                                return Err(ParseError::UnusedVariable(f.n.c.loc))
                                            }
                                        }
                                    }
                                    None => {
                                        aln += 1;
                                    }
                                }

                                let v = match sv[0].as_bytes()[0] {
                                    b'_' => Var::mnew(sv[0].to_owned(), n.to_owned(), aln),
                                    _ => Var::new_l(sv[0].to_owned(), n.to_owned(), aln),
                                };
                                if v.to_owned().m {
                                    self.used_variable.push(v.to_owned().s);
                                }
                                self.l.push(v.to_owned());

                                let avar = NodeSt::ass_var(v.to_owned().aln, rhs, n.c.loc);
                                self.node_st_vec.push(avar);
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

                                self.set_imm_env();
                                match Program::find_v(_s.to_owned(), self.imm_env_v.to_owned()) {
                                    Some(mut f) => {
                                        // println!("a.used_variable: {:?}", a.used_variable);

                                        let rhs = self.parse_close_imm(it)?;

                                        f.n = rhs.to_owned();
                                        let ff = f.to_owned();
                                        if self.l.contains(&ff) {
                                            self.l.retain(|s| s.s != _s.to_owned());
                                            self.l.push(ff);
                                        }
                                        let rvar = NodeSt::r_var(
                                            f.to_owned().aln,
                                            rhs,
                                            n.c.loc.to_owned(),
                                        );
                                        self.node_st_vec.push(rvar);
                                    }
                                    _ => {
                                        return Err(ParseError::UndefinedVariable(
                                            n.to_owned().lhs.unwrap().c.loc,
                                        ))
                                    }
                                }
                            }
                            NodeKind::UnderScore => {
                                it.copy_iter();
                                it.expect_token(
                                    TokenKind::RBrace,
                                    ParseError::UnexpectedUnderScoreOperator(n.to_owned().c.loc),
                                )?;
                                it.back_to_shadow();

                                self.set_end_of_node();

                                let n = NodeSt::under_score(n.c.loc);
                                self.set_ret_node(n.to_owned());
                                self.node_st_vec.push(n);
                            }
                            NodeKind::If => {
                                self.parse_if(it, n)?;
                            }
                            NodeKind::While => {
                                self.parse_while(it, n)?;
                            }
                            NodeKind::LBrace => {
                                self.parse_stmt(it, n)?;
                            }
                            NodeKind::Pipe => {
                                self.parse_pipe(it)?;
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => {
                        self.set_imm_env();
                        let n = beta(&mut n.to_owned(), self)?;
                        self.node_st_vec.push(n.to_owned());

                        match it.check_evaluate_type() {
                            true => {
                                self.set_end_of_node();
                                self.set_ret_node(n.to_owned());
                            }
                            false => match it.check_evaluate_void() {
                                true => {
                                    self.set_end_of_node();
                                    self.set_ret_node(NodeSt::default());
                                }
                                false => (),
                            },
                        }
                    }
                },
                Err(e) => return Err(e),
            };

            if it.peek_value() == TokenKind::RBrace {
                if self.none_ret_node() {
                    self.set_end_of_node();
                    let n = NodeSt::under_score(Loc::default());
                    self.set_ret_node(n.to_owned());
                    self.node_st_vec.push(n);
                }
            }
        }

        println!("self.used_variable: {:?}", self.used_variable);

        for l in self.to_owned().l {
            match self.used_variable.contains(&l.to_owned().s.to_owned()) {
                true => (),
                false => return Err(ParseError::UnusedVariable(l.n.c.loc)),
            }
        }
        // println!("self.ret_node_st: {:?}", self.ret_node_st);

        let mut ugv: Vec<String> = vec![];
        for evc in self.env_v.to_owned() {
            for v in evc {
                // println!("v: {:?}", v);
                match self.used_variable.contains(&v.to_owned().s.to_owned()) {
                    true => ugv.push(v.to_owned().s.to_owned()),
                    false => (),
                }
            }
        }
        Ok(ugv)
    }
}

impl NodeArr {
    pub fn parse_if(&mut self, it: &mut TokenIter, n: NodeSt) -> Result<(), ParseError> {
        self.set_imm_env();

        let (if_stmts, uev) = Self::parse_statement(it, self.imm_env_v.to_owned())?;
        self.update_used_variable(uev);

        let (else_if_stmts, uev) = match it.p.peek().unwrap().value {
            TokenKind::Else => {
                it.next();
                Self::parse_statement(it, self.imm_env_v.to_owned())?
            }
            _ => (NodeArr::new(), vec![]),
        };
        self.update_used_variable(uev);

        let if_stmts_isi = NodeSt::isi(if_stmts.ret_node_st.to_owned());
        let else_if_stmts_isi = NodeSt::isi(else_if_stmts.ret_node_st.to_owned());

        if !else_if_stmts.to_owned().is_default() {
            if if_stmts_isi != else_if_stmts_isi {
                return Err(ParseError::NotMatchTypeAnotherOneOfStatement(
                    it.p.peek().unwrap().loc.to_owned(),
                ));
            }
        }

        if it.peek_value() == TokenKind::RBrace {
            self.set_end_of_node()
        }

        let mut c = beta(&mut n.cond.to_owned().unwrap(), self)?;

        use std::env;
        let args: Vec<String> = env::args().collect();
        let mut calc_cond = false;
        if args.len() > 3 {
            if args[3] == "calc_cond_label" {
                calc_cond = true;
            }
        }

        match calc_cond {
            false => {
                let mut n = n.to_owned();
                n.if_stmts = Some(Box::new(if_stmts.to_owned()));
                n.else_if_stmts = Some(Box::new(else_if_stmts.to_owned()));
                n.cond = Some(Box::new(c));
                self.node_st_vec.push(n.to_owned());

                if self.end_of_node {
                    self.ret_node_st = match if_stmts_isi {
                        true => n,
                        false => NodeSt::under_score(Loc::default()),
                    }
                }

                Ok(())
            }
            true => {
                c = c.simplified();
                match c.c.value {
                    NodeKind::Num(num) => {
                        if num == 0 {
                            if self.end_of_node {
                                self.ret_node_st = match else_if_stmts_isi {
                                    true => else_if_stmts.ret_node_st,
                                    false => NodeSt::under_score(Loc::default()),
                                }
                            }

                            self.node_st_vec
                                .append(&mut else_if_stmts.node_st_vec.to_owned());
                            return Ok(());
                        }

                        if self.end_of_node {
                            self.ret_node_st = match if_stmts_isi {
                                true => if_stmts.ret_node_st,
                                false => NodeSt::under_score(Loc::default()),
                            }
                        }

                        self.node_st_vec
                            .append(&mut if_stmts.node_st_vec.to_owned());
                        Ok(())
                    }
                    _ => {
                        let mut n = beta(&mut n.to_owned(), self)?;

                        n.if_stmts = Some(Box::new(if_stmts.to_owned()));
                        n.else_if_stmts = Some(Box::new(else_if_stmts.to_owned()));
                        n.cond = Some(Box::new(c));
                        self.node_st_vec.push(n.to_owned());

                        if self.end_of_node {
                            self.ret_node_st = match if_stmts_isi {
                                true => n,
                                false => NodeSt::under_score(Loc::default()),
                            }
                        }

                        Ok(())
                    }
                }
            }
        }
    }

    pub fn parse_while(&mut self, it: &mut TokenIter, n: NodeSt) -> Result<(), ParseError> {
        self.set_imm_env();
        let (stmts, uev) = Self::parse_statement(it, self.imm_env_v.to_owned())?;
        self.update_used_variable(uev);

        if it.peek_value() == TokenKind::RBrace {
            self.set_end_of_node()
        }

        let mut n = n.to_owned();
        n.stmts = Some(Box::new(stmts.to_owned()));
        let c = beta(&mut n.cond.to_owned().unwrap(), self)?;
        n.cond = Some(Box::new(c));
        self.node_st_vec.push(n.to_owned());

        let stmts_isi = NodeSt::isi(stmts.ret_node_st.to_owned());
        if self.end_of_node {
            self.ret_node_st = match stmts_isi {
                true => stmts.to_owned().ret_node_st,
                false => NodeSt::under_score(Loc::default()),
            }
        }

        return Ok(());
    }

    pub fn parse_stmt(&mut self, it: &mut TokenIter, n: NodeSt) -> Result<(), ParseError> {
        self.set_imm_env();
        let (stmts, uev) = Self::parse_statement(it, self.imm_env_v.to_owned())?;
        self.update_used_variable(uev);
        if it.peek_value() == TokenKind::RBrace {
            self.set_end_of_node()
        }

        let stmt_isi = NodeSt::isi(stmts.ret_node_st.to_owned());

        if self.end_of_node {
            self.ret_node_st = match stmt_isi {
                true => stmts.to_owned().ret_node_st,
                false => NodeSt::under_score(Loc::default()),
            }
        }

        let mut n = n.to_owned();

        n.stmts = Some(Box::new(stmts));
        self.node_st_vec.push(n.to_owned());

        return Ok(());
    }

    pub fn parse_pipe(&mut self, it: &mut TokenIter) -> Result<(), ParseError> {
        it.next();

        it.copy_iter();
        let n = self.parse_opened_imm(it)?;
        self.node_st_vec.push(n.to_owned());

        if it.peek_value() != TokenKind::RBrace {
            return Err(ParseError::NotClosedStmt(it.next()));
        }

        self.set_end_of_node();
        self.set_ret_node(n.to_owned());

        Ok(())
    }
}

impl NodeArr {
    pub fn parse_close_imm(&mut self, it: &mut TokenIter) -> Result<NodeSt, ParseError> {
        match NodeSt::parser(it) {
            Ok(n) => match n.c.value {
                NodeKind::Return
                | NodeKind::NewAssign
                | NodeKind::Assign
                | NodeKind::UnderScore => Err(ParseError::NotClosedImmediate(
                    it.shadow_p.peek().unwrap().loc.to_owned(),
                )),
                NodeKind::If => {
                    self.parse_if(it, n.to_owned())?;
                    match NodeSt::isi(
                        self.node_st_vec
                            .last()
                            .unwrap()
                            .if_stmts
                            .to_owned()
                            .unwrap()
                            .ret_node_st,
                    ) {
                        true => Ok(self.pop_node()),
                        false => Err(ParseError::NotClosedImmediate(
                            it.shadow_p.peek().unwrap().loc.to_owned(),
                        )),
                    }
                }
                NodeKind::While => {
                    self.parse_while(it, n.to_owned())?;
                    match NodeSt::isi(
                        self.node_st_vec
                            .last()
                            .unwrap()
                            .stmts
                            .to_owned()
                            .unwrap()
                            .ret_node_st,
                    ) {
                        true => Ok(self.pop_node()),
                        false => Err(ParseError::NotClosedImmediate(
                            it.shadow_p.peek().unwrap().loc.to_owned(),
                        )),
                    }
                }
                NodeKind::LBrace => {
                    self.parse_stmt(it, n.to_owned())?;
                    match NodeSt::isi(
                        self.node_st_vec
                            .last()
                            .unwrap()
                            .stmts
                            .to_owned()
                            .unwrap()
                            .ret_node_st,
                    ) {
                        true => Ok(self.pop_node()),
                        false => Err(ParseError::NotClosedImmediate(
                            it.shadow_p.peek().unwrap().loc.to_owned(),
                        )),
                    }
                }
                _ => {
                    self.set_imm_env();
                    let mut n = n;
                    n = beta(&mut n, self)?;
                    n = n.simplified();

                    it.expect_token(
                        TokenKind::SemiColon,
                        ParseError::NotClosedStmt(
                            it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                        ),
                    )?;
                    Ok(n)
                }
            },
            Err(e) => return Err(e),
        }
    }

    pub fn parse_opened_imm(&mut self, it: &mut TokenIter) -> Result<NodeSt, ParseError> {
        match NodeSt::parser(it) {
            Ok(n) => match n.c.value {
                NodeKind::Return
                | NodeKind::NewAssign
                | NodeKind::Assign
                | NodeKind::UnderScore
                | NodeKind::If
                | NodeKind::While
                | NodeKind::LBrace => Err(ParseError::NotOpenedImmediate(
                    it.shadow_p.peek().unwrap().loc.to_owned(),
                )),
                _ => {
                    self.set_imm_env();
                    let mut n = n;
                    n = beta(&mut n, self)?;
                    n = n.simplified();
                    Ok(n)
                }
            },
            Err(e) => return Err(e),
        }
    }
}

impl NodeArr {
    pub fn update_used_variable(&mut self, uvs: Vec<String>) {
        for v in uvs {
            if !self.used_variable.contains(&v) {
                self.used_variable.push(v)
            }
        }
    }
}
