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
    pub ty: Vec<RetTy>,
    pub ret_nodes: Vec<NodeSt>,
    pub l: Vec<Var>,
    pub env_v: Vec<Vec<Var>>,
    pub imm_env_v: Vec<Vec<Var>>,
    pub end_of_node: bool,
    pub used_variable: Vec<String>,
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
            ty: vec![],
            ret_nodes: vec![],
            l: vec![],
            env_v: vec![],
            imm_env_v: vec![],
            end_of_node: false,
            used_variable: vec![],
        }
    }

    pub fn is_default(&self) -> bool {
        return if self.node_st_vec == vec![] {
            true
        } else {
            false
        };
    }

    pub fn set_node(&mut self, v: Vec<NodeSt>) {
        self.node_st_vec = v.to_owned();
        self.ret_nodes.push(v.last().unwrap().to_owned());
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

    pub fn none_ret_node(&self) -> bool {
        self.ret_nodes == vec![]
    }

    pub fn set_ret_node(&mut self, nds: Vec<NodeSt>) {
        for n in nds {
            self.ret_nodes.push(n);
        }
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

        let isi: bool;
        match it.peek_value() {
            TokenKind::To => {
                it.next_with_shadow();

                match it.consume_type()?.c.value {
                    NodeKind::Int => isi = true,
                    NodeKind::Nill => isi = false,
                    _ => unreachable!(),
                };
            }
            _ => {
                it.expect_peek_token(
                    TokenKind::LBrace,
                    ParseError::NotOpenedStmt(
                        it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                    ),
                )?;

                isi = false
            }
        };

        let ev = vec![g.to_owned()];

        let (mut l, uev) = Self::parse_statement(&mut it, ev)?;

        if isi {
            l.ty.push(RetTy::Int32);
        } else {
            l.ty.push(RetTy::Void);
        }

        for i in 0..l.to_owned().ret_nodes.len() {
            if l.ty[i] == RetTy::Int32 {
                match NodeSt::isi(&mut l.ret_nodes[i]) {
                    true => (),
                    false => {
                        return Err(ParseError::NotMatchReturnType(
                            l.to_owned().ret_nodes[i].c.loc.to_owned(),
                        ))
                    }
                }
            } else {
                match l.ret_nodes[i].c.value {
                    NodeKind::UnderScore | NodeKind::Default => return Ok((l, uev)),
                    _ => {
                        return Err(ParseError::NotMatchReturnType(
                            l.to_owned().ret_nodes[i].c.loc.to_owned(),
                        ))
                    }
                }
            }
        }

        Ok((l, uev))
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
                    | NodeKind::Pipe
                    | NodeKind::LTouple => {
                        // println!("n.c.value: {:?}", n.c.value);
                        match n.c.value {
                            NodeKind::Return => {
                                if it.peek_value() != TokenKind::RBrace {
                                    return Err(ParseError::OperatorAfterRetrun(it.next()));
                                }

                                self.set_end_of_node();
                                self.set_imm_env();
                                let r = beta(&mut n.to_owned().lhs.unwrap().to_owned(), self)?;
                                let nds = vec![r];
                                self.set_ret_node(nds.to_owned());
                                self.node_st_vec.append(&mut nds.to_owned());
                            }
                            NodeKind::NewAssign => {
                                let contents = n
                                    .to_owned()
                                    .lhs
                                    .unwrap()
                                    .lhs
                                    .unwrap()
                                    .ret_set
                                    .unwrap()
                                    .contents;
                                self.set_imm_env();

                                let mut vs: Vec<String> = vec![];
                                for c in contents.to_owned() {
                                    match vs.contains(&c) {
                                        true => return Err(ParseError::AssignedSameWord(n.c.loc)),
                                        false => vs.push(c),
                                    }
                                }

                                let rhs = if contents.len() <= 1 {
                                    vec![self.parse_close_imm(it)?]
                                } else {
                                    self.parse_touple(it)?;
                                    let mut t = vec![self.pop_node()];
                                    for _ in 1..contents.len() {
                                        t.push(self.pop_node());
                                    }
                                    t.reverse();
                                    t
                                };

                                let mut alns: Vec<i32> = vec![];
                                for c in contents.to_owned() {
                                    match Program::find_v(c.to_owned(), self.imm_env_v.to_owned()) {
                                        Some(f) => {
                                            match self
                                                .used_variable
                                                .contains(&f.to_owned().s.to_owned())
                                            {
                                                true => self
                                                    .used_variable
                                                    .retain(|s| s != &f.to_owned().s.to_owned()),
                                                false => {
                                                    return Err(ParseError::UnusedVariable(
                                                        f.n.c.loc,
                                                    ))
                                                }
                                            }
                                        }
                                        None => {
                                            aln += 1;
                                        }
                                    }

                                    let v = match c.as_bytes()[0] {
                                        b'_' => Var::mnew(c, n.to_owned(), aln),
                                        _ => Var::new_l(c, n.to_owned(), aln),
                                    };
                                    if v.to_owned().m {
                                        self.used_variable.push(v.to_owned().s);
                                    }
                                    self.l.push(v.to_owned());
                                    alns.push(v.to_owned().aln);
                                }

                                for i in 0..contents.len() {
                                    self.node_st_vec.push(NodeSt::ass_var(
                                        alns[i],
                                        rhs[i].to_owned(),
                                        n.to_owned().c.loc,
                                    ));
                                }
                            }
                            NodeKind::Assign => {
                                let _s: String;
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
                                let nds = vec![n];
                                self.set_ret_node(nds);
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
                            NodeKind::LTouple => {
                                self.parse_touple(it)?;
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
                                let nds = vec![n];
                                self.set_end_of_node();
                                self.set_ret_node(nds);
                            }
                            false => match it.check_evaluate_void() {
                                true => {
                                    let nds = vec![NodeSt::default()];
                                    self.set_end_of_node();
                                    self.set_ret_node(nds);
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
                    let nds = vec![n];
                    self.set_ret_node(nds.to_owned());
                    self.node_st_vec.append(&mut nds.to_owned());
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

        let if_stmts_isi = isi_all(if_stmts.to_owned().ret_nodes);

        let else_if_stmts_isi = match else_if_stmts.has_ret() {
            true => isi_all(else_if_stmts.to_owned().ret_nodes),
            false => false,
        };

        if !else_if_stmts.is_default() {
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
            if args[3] == "calc_cond" {
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
                    self.ret_nodes.push(match if_stmts_isi {
                        true => n,
                        false => NodeSt::under_score(Loc::default()),
                    });
                }

                Ok(())
            }
            true => {
                c = c.simplified();
                match c.c.value {
                    NodeKind::Num(num) => {
                        if num == 0 {
                            if self.end_of_node {
                                match else_if_stmts_isi {
                                    true => self.ret_nodes = else_if_stmts.ret_nodes,
                                    false => {
                                        self.ret_nodes.push(NodeSt::under_score(Loc::default()))
                                    }
                                }
                            }

                            self.node_st_vec
                                .append(&mut else_if_stmts.node_st_vec.to_owned());
                            return Ok(());
                        }

                        if self.end_of_node {
                            match if_stmts_isi {
                                true => self.ret_nodes = if_stmts.ret_nodes,
                                false => self.ret_nodes.push(NodeSt::under_score(Loc::default())),
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
                            match if_stmts_isi {
                                true => self.ret_nodes.push(n),
                                false => self.ret_nodes.push(NodeSt::under_score(Loc::default())),
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

        let stmt_isi = match stmts.has_ret() {
            true => isi_all(stmts.to_owned().ret_nodes),
            false => false,
        };

        if self.end_of_node {
            match stmt_isi {
                true => self.ret_nodes = stmts.to_owned().ret_nodes,
                false => self.ret_nodes.push(NodeSt::under_score(Loc::default())),
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

        let stmt_isi = match stmts.has_ret() {
            true => isi_all(stmts.to_owned().ret_nodes),
            false => false,
        };

        if self.end_of_node {
            match stmt_isi {
                true => self.ret_nodes = stmts.to_owned().ret_nodes,
                false => self.ret_nodes.push(NodeSt::under_score(Loc::default())),
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

        let nds = vec![self.parse_opened_imm(it)?];

        self.node_st_vec.append(&mut nds.to_owned());

        if it.peek_value() != TokenKind::RBrace {
            return Err(ParseError::NotClosedStmt(it.next()));
        }

        self.set_end_of_node();
        self.set_ret_node(nds);

        Ok(())
    }

    pub fn parse_touple(&mut self, it: &mut TokenIter) -> Result<(), ParseError> {
        it.next();

        it.copy_iter();
        let mut nds = vec![self.parse_opened_imm(it)?];

        while it.peek_value() == TokenKind::Comma {
            it.next();
            nds.push(self.parse_opened_imm(it)?);
        }
        it.next();

        match it.peek_value() {
            TokenKind::RBrace => self.set_end_of_node(),
            TokenKind::SemiColon => {
                it.next();
                self.node_st_vec.append(&mut nds.to_owned())
            }
            _ => unimplemented!(),
        }

        if self.end_of_node {
            self.set_ret_node(nds.to_owned());
        }

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
                    match isi_all(
                        self.node_st_vec
                            .last()
                            .unwrap()
                            .if_stmts
                            .to_owned()
                            .unwrap()
                            .ret_nodes,
                    ) {
                        true => Ok(self.pop_node()),
                        false => Err(ParseError::NotClosedImmediate(
                            it.shadow_p.peek().unwrap().loc.to_owned(),
                        )),
                    }
                }
                NodeKind::While => {
                    self.parse_while(it, n.to_owned())?;
                    match isi_all(
                        self.node_st_vec
                            .last()
                            .unwrap()
                            .stmts
                            .to_owned()
                            .unwrap()
                            .ret_nodes,
                    ) {
                        true => Ok(self.pop_node()),
                        false => Err(ParseError::NotClosedImmediate(
                            it.shadow_p.peek().unwrap().loc.to_owned(),
                        )),
                    }
                }
                NodeKind::LBrace => {
                    self.parse_stmt(it, n.to_owned())?;
                    match isi_all(
                        self.node_st_vec
                            .last()
                            .unwrap()
                            .stmts
                            .to_owned()
                            .unwrap()
                            .ret_nodes,
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

    pub fn has_ret(&self) -> bool {
        match self.ret_nodes.len() {
            0 => false,
            _ => true,
        }
    }
}
