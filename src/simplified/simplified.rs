use super::super::location::location::*;
use super::super::node::node::*;
use super::super::node_arr::node_arr::*;

impl NodeArr {
    pub fn simplified(mut self) -> Self {
        let mut nai = self.node_st_vec.iter().peekable();
        let mut nv = vec![];
        while nai.peek() != None {
            nv.push(nai.next().unwrap().to_owned().simplified());
        }
        self.node_st_vec = nv.to_owned();
        self.ret_node_st = nv.last().unwrap().to_owned();
        self
    }
}

impl NodeSt {
    pub fn simplified(mut self) -> Self {
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
                let mut lnn = true;
                let mut rnn = true;
                let ln = self.lhs.as_ref().unwrap().as_ref().to_owned().simplified();
                let llf = ln.c.loc.f;
                let l = match ln.c.value {
                    NodeKind::Num(n) => n,
                    _ => {
                        lnn = false;
                        0
                    }
                };

                let rn = self.rhs.as_ref().unwrap().as_ref().to_owned().simplified();
                let r = match rn.c.value {
                    NodeKind::Num(n) => n,
                    _ => {
                        rnn = false;
                        0
                    }
                };

                if !lnn || !rnn {
                    self.lhs = match lnn {
                        true => Some(Box::new(NodeSt::num(
                            l,
                            Loc::new(llf, (llf as i8 + (l / 10) + 1) as u8),
                        ))),
                        false => Some(Box::new(ln)),
                    };

                    self.rhs = match rnn {
                        true => Some(Box::new(NodeSt::num(
                            r,
                            Loc::new(llf, (llf as i8 + (r / 10) + 1) as u8),
                        ))),
                        false => Some(Box::new(rn)),
                    };
                    return self;
                }

                self = match self.c.value {
                    NodeKind::Add => {
                        NodeSt::num(l + r, Loc::new(llf, (llf as i8 + ((l + r) / 10) + 1) as u8))
                    }
                    NodeKind::Sub => {
                        NodeSt::num(l - r, Loc::new(llf, (llf as i8 + ((l - r) / 10) + 1) as u8))
                    }
                    NodeKind::Mul => {
                        NodeSt::num(l * r, Loc::new(llf, (llf as i8 + ((l * r) / 10) + 1) as u8))
                    }
                    NodeKind::Div => {
                        NodeSt::num(l / r, Loc::new(llf, (llf as i8 + ((l / r) / 10) + 1) as u8))
                    }
                    NodeKind::Sur => {
                        NodeSt::num(l % r, Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8))
                    }
                    NodeKind::E => NodeSt::num(
                        (l == r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    ),
                    NodeKind::NE => NodeSt::num(
                        (l != r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    ),
                    NodeKind::L => NodeSt::num(
                        (l < r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    ),
                    NodeKind::LE => NodeSt::num(
                        (l <= r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    ),
                    NodeKind::G => NodeSt::num(
                        (l > r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    ),
                    NodeKind::GE => NodeSt::num(
                        (l >= r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    ),
                    _ => unreachable!(),
                }
            }
            NodeKind::Num(_) | NodeKind::UnderScore | NodeKind::NewVar(_) | NodeKind::Var(_) => (),
            _ => unreachable!(),
        };
        self
    }
}
