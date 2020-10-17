use super::super::location::location::*;
// use super::super::node::node::*;
use super::super::node::node::*;
use super::super::node_arr::node_arr::*;
use super::super::parser::error::*;
// use super::super::parser::parser::*;
use super::super::simplified::beta::*;
use super::super::simplified::*;
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

        na.push(NodeArr::w_parser(&mut it, g.to_owned())?);
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
            match NodeSt::parser(&mut it) {
                Ok(n) => match n.c.value {
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
                                let mut n =
                                    beta(&mut n.to_owned().rhs.unwrap().to_owned(), ev, &mut uv);
                                n = simplified::exec(n);
                                f.n = n;
                                let ff = f.to_owned();
                                g.retain(|s| s.s != _s.to_owned());
                                g.push(ff);
                            }
                            _ => {
                                let mut n =
                                    beta(&mut n.to_owned().rhs.unwrap().to_owned(), ev, &mut uv);
                                n = simplified::exec(n);
                                let v = Var::new(_s, n);
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
                Err(e) => return Err(e),
            }
        }

        println!("g: {:?}", g);
        return Ok(g);
    }
}
