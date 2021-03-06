use super::super::node::node::*;
use super::super::node_arr::node_arr::*;
use super::super::parser::error::*;
use super::super::simplified::beta::*;
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

    pub fn w_parser(vt: &mut Vec<Token>) -> Result<Self, ParseError> {
        let mut na: Vec<NodeArr> = vec![];
        let mut it = TokenIter::new(vt);
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

    pub fn stp(it: &mut TokenIter) -> Result<Vec<Var>, ParseError> {
        let mut a = NodeArr::new();
        let mut aln: i32 = 0;
        while it.peek_value() != TokenKind::Fn && a.end_of_node == false {
            it.copy_iter();
            match NodeSt::parser(it)? {
                n => match n.c.value {
                    NodeKind::Fn => {
                        a.set_end_of_node();
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

                        a.set_imm_env();
                        match NodeArr::find_l(sv[0].to_owned(), a.l.to_owned()) {
                            Some(mut f) => {
                                match a.used_variable.contains(&f.to_owned().s.to_owned()) {
                                    true => (),
                                    false => return Err(ParseError::UnusedVariable(f.n.c.loc)),
                                }
                                a.used_variable.retain(|s| s != &f.to_owned().s.to_owned());

                                let mut rhs = a.parse_close_imm(it)?;
                                let mut rhs = beta(&mut rhs, &mut a)?;
                                rhs = rhs.simplified();

                                f.n = rhs;
                                let ff = f.to_owned();
                                a.l.retain(|s| s.s != sv[0].to_owned());
                                a.l.push(ff);
                            }
                            _ => {
                                let mut rhs = a.parse_close_imm(it)?;
                                let mut rhs = beta(&mut rhs, &mut a)?;
                                rhs = rhs.simplified();

                                match rhs.c.value {
                                    NodeKind::Num(_) => (),
                                    _ => {
                                        return Err(ParseError::NotACompileTimeConstant(rhs.c.loc))
                                    }
                                }
                                aln += 1;
                                let v = match sv[0].to_owned().as_bytes()[0] {
                                    b'_' => Var::gmnew(sv[0].to_owned(), rhs, aln),
                                    _ => Var::new_g(sv[0].to_owned(), rhs, aln),
                                };
                                if v.m {
                                    a.used_variable.push(v.to_owned().s);
                                }
                                a.l.push(v);
                            }
                        }
                        continue;
                    }
                    _ => return Err(ParseError::OperatorOutOfFnction(it.peek_shadow())),
                },
            }
        }
        println!("a.l (gloval variable): {:?}", a.l);
        return Ok(a.l);
    }
}
