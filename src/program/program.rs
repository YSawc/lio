use super::super::location::location::*;
// use super::super::node::node::*;
use super::super::node::node::*;
use super::super::node_arr::node_arr::*;
use super::super::parser::error::*;
// use super::super::parser::parser::*;
use super::super::simplified::beta::*;
// use super::super::simplified::*;
use super::super::token::token::*;
use super::super::var::var::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    pub g: Vec<Var>,
    pub na: Vec<NodeArr>,
}

impl Program {
    pub fn new(g: Vec<Var>, na: Vec<NodeArr>) -> Self {
        Self { g, na }
    }
}

impl Program {
    pub fn find_v(s: String, ev: Vec<Vec<Var>>) -> Option<Var> {
        let ie = ev.iter().peekable().rev();

        for evs in ie {
            for v in evs {
                // println!("v: {:?}", v);
                if v.s == s {
                    return Some(v.to_owned());
                }
            }
        }
        None
    }

    pub fn w_parser(vt: Vec<Token>) -> Result<Self, ParseError> {
        let mut na: Vec<NodeArr> = vec![];
        let mut it = vt.iter().peekable();
        let g: Vec<Var> = Self::stp(&mut it)?;

        let (n, mut ugv) = NodeArr::w_parser(&mut it, g.to_owned())?;
        for l in g.to_owned() {
            if l.to_owned().s.to_owned().as_bytes()[0] == b'_' {
                ugv.push(l.to_owned().s);
            }
        }
        for l in g.to_owned() {
            match ugv.contains(&l.to_owned().s.to_owned()) {
                true => (),
                false => return Err(ParseError::UnusedVariable(l.n.c.loc)),
            }
        }
        na.push(n);
        Ok(Self::new(g, na))
    }

    pub fn stp(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<Vec<Var>, ParseError> {
        let mut uv: Vec<String> = vec![];
        let mut g: Vec<Var> = vec![];
        let mut b = false;
        while it.peek().unwrap().value != TokenKind::Fn && b == false {
            let et = it.to_owned();
            match NodeSt::parser(&mut it)? {
                n => match n.c.value {
                    NodeKind::Fn => {
                        b = true;
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
                        let mut ev = vec![];
                        ev.push(g.to_owned());
                        match NodeArr::find_l(_s.to_owned(), g.to_owned()) {
                            Some(mut f) => {
                                match uv.contains(&f.to_owned().s.to_owned()) {
                                    true => (),
                                    false => return Err(ParseError::UnusedVariable(f.n.c.loc)),
                                }
                                uv.retain(|s| s != &f.to_owned().s.to_owned());
                                let n =
                                    beta(&mut n.to_owned().rhs.unwrap().to_owned(), ev, &mut uv);
                                n.simplified();
                                f.n = n;
                                let ff = f.to_owned();
                                g.retain(|s| s.s != _s.to_owned());
                                g.push(ff);
                            }
                            _ => {
                                let n =
                                    beta(&mut n.to_owned().rhs.unwrap().to_owned(), ev, &mut uv);
                                n.simplified();
                                let v = match _s.as_bytes()[0] {
                                    b'_' => Var::mnew(_s, n),
                                    _ => Var::new(_s, n),
                                };
                                if v.m {
                                    uv.push(v.to_owned().s);
                                }
                                g.push(v);
                            }
                        }
                        continue;
                    }
                    _ => {
                        return Err(ParseError::OperatorOutOfFnction(
                            et.to_owned().next().unwrap().to_owned(),
                        ))
                    }
                },
            }
        }
        println!("g: {:?}", g);
        return Ok(g);
    }
}
