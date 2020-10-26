use super::super::location::location::*;
use super::super::node::node::*;
use super::super::parser::error::*;
use super::super::program::program::*;
use super::super::simplified::beta::*;
// use super::super::simplified::*;
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
    pub fn new(v: Vec<NodeSt>) -> Self {
        Self {
            node_st_vec: v.to_owned(),
            ty: RetTy::Default,
            ret_node_st: v.to_owned().last().unwrap().to_owned(),
            l: vec![],
        }
    }

    pub fn w_parser(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
        g: Vec<Var>,
    ) -> Result<(Self, Vec<String>), ParseError> {
        let et = it.to_owned();
        if it.peek().unwrap().value != TokenKind::Fn {
            return Err(ParseError::OperatorOutOfFnction(
                it.next().unwrap().to_owned(),
            ));
        }

        et.to_owned().next();
        it.next();

        let mut isi: bool = false;
        if it.peek().unwrap().value == TokenKind::Int {
            et.to_owned().next();
            it.next();
            isi = true;
        }

        if it.peek() == None {
            return Err(ParseError::NotLBrace(
                et.to_owned().next().unwrap().to_owned(),
            ));
        }
        it.next();

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
                | NodeKind::NewAssign
                | NodeKind::Ident(_)
                | NodeKind::Var(_) => return Ok((l, ugv)),
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

    pub fn stp(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
        ev: Vec<Vec<Var>>,
    ) -> Result<(Self, Vec<String>), ParseError> {
        let mut uv: Vec<String> = vec![];
        let mut nv = vec![];
        let mut l: Vec<Var> = vec![];
        let mut r: NodeSt = NodeSt::default();
        let mut aln: i32 = 0;
        let mut b = false;
        while it.peek() != None && b == false {
            nv.push(match NodeSt::parser(&mut it) {
                Ok(n) => match n.c.value {
                    NodeKind::Return => {
                        if it.peek().unwrap().to_owned().to_owned().value != TokenKind::RBrace {
                            return Err(ParseError::OperatorAfterRetrun(
                                it.next().unwrap().to_owned(),
                            ));
                        }
                        b = true;

                        let mut ev = ev.to_owned();
                        ev.push(l.to_owned());
                        r = beta(&mut n.to_owned().lhs.unwrap().to_owned(), ev, &mut uv);
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
                        let mut ev = ev.to_owned();
                        ev.push(l.to_owned());
                        match Program::find_v(_s.to_owned(), ev.to_owned()) {
                            Some(mut f) => {
                                match uv.contains(&f.to_owned().s.to_owned()) {
                                    true => (),
                                    false => return Err(ParseError::UnusedVariable(f.n.c.loc)),
                                }
                                uv.retain(|s| s != &f.to_owned().s.to_owned());
                                let mut lhs = beta(
                                    &mut n.to_owned().rhs.unwrap().to_owned(),
                                    ev.to_owned(),
                                    &mut uv,
                                );
                                lhs = lhs.simplified();
                                f.n = n.to_owned();
                                let ff = f.to_owned();
                                l.retain(|s| s.s != _s.to_owned());
                                l.push(ff);

                                let avar = NodeSt::ass_var(f.to_owned().aln, lhs, n.c.loc);
                                avar
                            }
                            None => {
                                let mut lhs = beta(
                                    &mut n.to_owned().rhs.unwrap().to_owned(),
                                    ev.to_owned(),
                                    &mut uv,
                                );
                                lhs = lhs.simplified();
                                let v = match _s.as_bytes()[0] {
                                    b'_' => Var::mnew(_s, n.to_owned()),
                                    _ => Var::new(_s, n.to_owned()),
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

                        let mut ev = ev.to_owned();
                        ev.push(l.to_owned());
                        match Program::find_v(_s.to_owned(), ev.to_owned()) {
                            Some(mut f) => {
                                // println!("uv: {:?}", uv);
                                match uv.contains(&f.to_owned().s.to_owned()) {
                                    true => (),
                                    false => return Err(ParseError::UnusedVariable(f.n.c.loc)),
                                }
                                let mut n = beta(
                                    &mut n.to_owned().rhs.unwrap().to_owned(),
                                    ev.to_owned(),
                                    &mut uv,
                                );
                                n = n.simplified();
                                f.n = n;
                                let ff = f.to_owned();
                                l.retain(|s| s.s != _s.to_owned());
                                l.push(ff);
                            }
                            _ => {
                                return Err(ParseError::UndefinedVariable(
                                    n.to_owned().lhs.unwrap().c.loc,
                                ))
                            }
                        }
                        n
                    }
                    NodeKind::UnderScore => {
                        if it.peek().unwrap().to_owned().to_owned().value != TokenKind::RBrace {
                            return Err(ParseError::UnexpectedUnderScoreOperator(
                                n.to_owned().c.loc,
                            ));
                        }
                        b = true;
                        let n = NodeSt::under_score(n.c.loc);
                        r = n.to_owned();
                        n
                    }
                    NodeKind::Ident(s) => {
                        let mut ev = ev.to_owned();
                        ev.push(l.to_owned());
                        match Program::find_v(s.to_owned(), ev.to_owned()) {
                            Some(mut v) => {
                                if it.peek().unwrap().to_owned().to_owned().value
                                    == TokenKind::RBrace
                                {
                                    b = true;
                                }

                                if !uv.contains(&v.to_owned().s.to_owned()) {
                                    uv.push(v.to_owned().s.to_owned());
                                    v.aln = aln;
                                    aln += 1;
                                }

                                if b {
                                    r = v.to_owned().n.to_owned();
                                    println!();
                                }
                                let n = NodeSt::var(v.aln, n.c.loc);
                                n
                            }
                            None => {
                                return Err(ParseError::NotDefinitionVar(
                                    it.next().unwrap().to_owned(),
                                ))
                            }
                        }
                    }
                    NodeKind::If => {
                        if it.peek() == None {
                            return Err(ParseError::Eof);
                        }
                        if it.peek().unwrap().to_owned().to_owned().value == TokenKind::RBrace {
                            b = true;
                        }
                        let mut c = beta(
                            &mut n.to_owned().cond.unwrap().to_owned(),
                            ev.to_owned(),
                            &mut uv,
                        );
                        c = c.simplified();
                        match c.c.value {
                            NodeKind::Num(num) => {
                                if num == 0 {
                                    if n.to_owned().melse_stmt != None {
                                        if b {
                                            r = *n.to_owned().melse_stmt.unwrap().to_owned();
                                        }
                                        *n.to_owned().melse_stmt.unwrap().to_owned()
                                    } else {
                                        continue;
                                    }
                                } else {
                                    if b {
                                        r = *n.to_owned().stmt.unwrap().to_owned();
                                    }
                                    *n.to_owned().stmt.unwrap().to_owned()
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => {
                        if it.peek().unwrap().to_owned().to_owned().value == TokenKind::RBrace {
                            b = true;
                        }

                        let mut ev = ev.to_owned();
                        ev.push(l.to_owned());
                        let n = beta(&mut n.to_owned(), ev, &mut uv);
                        if b {
                            r = n.to_owned();
                        }
                        // println!("r: {:?}", r);
                        n
                    }
                },
                Err(e) => return Err(e),
            });
        }

        let mut a = Self::new(nv);
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
        for evc in ev {
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
