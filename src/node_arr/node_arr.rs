use super::super::location::location::*;
use super::super::node::node::*;
use super::super::parser::error::*;
use super::super::simplified::*;
use super::super::token::token::*;
use super::super::var::var::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeArr {
    pub node_st_vec: Vec<NodeSt>,
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
            ret_node_st: v.to_owned().last().unwrap().to_owned(),
            l: vec![],
        }
    }

    pub fn w_parser(vt: Vec<Token>) -> Result<Self, ParseError> {
        let mut it = vt.iter().peekable();

        let et = it.to_owned();
        if it.peek().unwrap().value != TokenKind::Fn {
            return Err(ParseError::OperatorOutOfFnction(
                it.next().unwrap().to_owned(),
            ));
        }

        et.to_owned().next();
        it.next();

        if it.peek() == None {
            return Err(ParseError::NotLBrace(
                et.to_owned().next().unwrap().to_owned(),
            ));
        }
        it.next();

        Self::stp(&mut it)
    }

    pub fn stp(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<Self, ParseError> {
        let mut uv: Vec<String> = vec![];
        let mut nv = vec![];
        let mut l: Vec<Var> = vec![];
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
                        n
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

                        match Self::find_l(_s.to_owned(), l.to_owned()) {
                            Some(mut f) => {
                                match uv.contains(&f.to_owned().s.to_owned()) {
                                    true => (),
                                    false => return Err(ParseError::UnusedVariable(f.n.c.loc)),
                                }
                                let mut n = vex(
                                    &mut n.to_owned().rhs.unwrap().to_owned(),
                                    l.to_owned(),
                                    &mut uv,
                                );
                                n = simplified::exec(n);
                                f.n = n;
                                let ff = f.to_owned();
                                l.retain(|s| s.s != _s.to_owned());
                                l.push(ff);
                            }
                            _ => {
                                let mut n = vex(
                                    &mut n.to_owned().rhs.unwrap().to_owned(),
                                    l.to_owned(),
                                    &mut uv,
                                );
                                n = simplified::exec(n);
                                let v = Var::new(_s, n);
                                l.push(v);
                            }
                        }
                        continue;
                    }
                    NodeKind::Ident(s) => match Self::find_l(s, l.to_owned()) {
                        Some(v) => {
                            if it.peek().unwrap().to_owned().to_owned().value == TokenKind::RBrace {
                                b = true;
                            }
                            uv.push(v.s);
                            v.n
                        }
                        None => {
                            return Err(ParseError::NotDefinitionVar(it.next().unwrap().to_owned()))
                        }
                    },
                    NodeKind::If => {
                        if it.peek().unwrap().to_owned().to_owned().value == TokenKind::RBrace {
                            b = true;
                        }
                        let mut c = vex(
                            &mut n.to_owned().cond.unwrap().to_owned(),
                            l.to_owned(),
                            &mut uv,
                        );
                        c = simplified::exec(c);
                        match c.c.value {
                            NodeKind::Num(num) => {
                                if num == 0 {
                                    if n.to_owned().melse_stmt != None {
                                        *n.to_owned().melse_stmt.unwrap().to_owned()
                                    } else {
                                        continue;
                                    }
                                } else {
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

                        let n = vex(&mut n.to_owned(), l.to_owned(), &mut uv);
                        n
                    }
                },
                Err(e) => return Err(e),
            });
        }

        let mut a = Self::new(nv);
        a.l = l;
        println!("uv: {:?}", uv);

        for l in a.to_owned().l {
            match uv.contains(&l.to_owned().s.to_owned()) {
                true => (),
                false => return Err(ParseError::UnusedVariable(l.n.c.loc)),
            }
        }

        Ok(a)
    }
}
