use super::super::location::location::*;
use super::super::node::node::*;
use super::super::node_arr::node_arr::*;

pub fn simplified(mut na: NodeArr) -> NodeArr {
    let mut nai = na.node_st_vec.iter().peekable();
    let mut nv = vec![];
    while nai.peek() != None {
        nv.push(nai.next().unwrap().to_owned().simplified());
    }
    na.node_st_vec = nv.to_owned();
    na.ret_node_st = nv.last().unwrap().to_owned();
    na
}

impl NodeSt {
    pub fn simplified(&self) -> Self {
        match self.c.value {
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
            | NodeKind::GE => {
                let ln = self.lhs.as_ref().unwrap().as_ref().to_owned().simplified();
                let llf = ln.c.loc.f;
                let l = match ln.c.value {
                    NodeKind::Num(n) => n,
                    _ => unreachable!(),
                };
                let r = match self
                    .rhs
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .to_owned()
                    .simplified()
                    .c
                    .value
                {
                    NodeKind::Num(n) => n,
                    _ => unreachable!(),
                };
                match self.c.value {
                    NodeKind::Add => {
                        return NodeSt::num(
                            l + r,
                            Loc::new(llf, (llf as i8 + ((l + r) / 10) + 1) as u8),
                        )
                    }
                    NodeKind::Sub => {
                        return NodeSt::num(
                            l - r,
                            Loc::new(llf, (llf as i8 + ((l - r) / 10) + 1) as u8),
                        )
                    }
                    NodeKind::Mul => {
                        return NodeSt::num(
                            l * r,
                            Loc::new(llf, (llf as i8 + ((l * r) / 10) + 1) as u8),
                        )
                    }
                    NodeKind::Div => {
                        return NodeSt::num(
                            l / r,
                            Loc::new(llf, (llf as i8 + ((l / r) / 10) + 1) as u8),
                        )
                    }
                    NodeKind::Sur => {
                        return NodeSt::num(
                            l % r,
                            Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                        )
                    }
                    NodeKind::E => {
                        return NodeSt::num(
                            (l == r) as i8,
                            Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                        )
                    }
                    NodeKind::NE => {
                        return NodeSt::num(
                            (l != r) as i8,
                            Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                        )
                    }
                    NodeKind::L => {
                        return NodeSt::num(
                            (l < r) as i8,
                            Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                        )
                    }
                    NodeKind::LE => {
                        return NodeSt::num(
                            (l <= r) as i8,
                            Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                        )
                    }
                    NodeKind::G => {
                        return NodeSt::num(
                            (l > r) as i8,
                            Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                        )
                    }
                    NodeKind::GE => {
                        return NodeSt::num(
                            (l >= r) as i8,
                            Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                        )
                    }
                    _ => unreachable!(),
                }
            }
            NodeKind::Num(_) | NodeKind::UnderScore => return self.to_owned(),
            _ => unreachable!(),
        };
    }
}
