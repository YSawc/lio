use super::super::location::location::*;
use super::super::node::node::*;
use super::super::node_arr::node_arr::*;

pub fn simplified(mut na: NodeArr) -> NodeArr {
    let mut nai = na.node_st_vec.iter().peekable();
    let mut nv = vec![];
    while nai.peek() != None {
        nv.push(prepare(nai.next().unwrap().to_owned()));
    }
    na.node_st_vec = nv.to_owned();
    na.ret_node_st = nv.last().unwrap().to_owned();
    na
}

pub fn prepare(ns: NodeSt) -> NodeSt {
    exec(ns)
}

pub fn exec(ns: NodeSt) -> NodeSt {
    match ns.c.value {
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
            let ln = prepare(ns.lhs.as_ref().unwrap().as_ref().to_owned());
            let llf = ln.c.loc.f;
            let l = match ln.c.value {
                NodeKind::Num(n) => n,
                _ => unreachable!(),
            };
            let r = match prepare(ns.rhs.as_ref().unwrap().as_ref().to_owned())
                .c
                .value
            {
                NodeKind::Num(n) => n,
                _ => unreachable!(),
            };
            match ns.c.value {
                NodeKind::Add => {
                    return NodeSt::number(
                        l + r,
                        Loc::new(llf, (llf as i8 + ((l + r) / 10) + 1) as u8),
                    )
                }
                NodeKind::Sub => {
                    return NodeSt::number(
                        l - r,
                        Loc::new(llf, (llf as i8 + ((l - r) / 10) + 1) as u8),
                    )
                }
                NodeKind::Mul => {
                    return NodeSt::number(
                        l * r,
                        Loc::new(llf, (llf as i8 + ((l * r) / 10) + 1) as u8),
                    )
                }
                NodeKind::Div => {
                    return NodeSt::number(
                        l / r,
                        Loc::new(llf, (llf as i8 + ((l / r) / 10) + 1) as u8),
                    )
                }
                NodeKind::Sur => {
                    return NodeSt::number(
                        l % r,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    )
                }
                NodeKind::E => {
                    return NodeSt::number(
                        (l == r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    )
                }
                NodeKind::NE => {
                    return NodeSt::number(
                        (l != r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    )
                }
                NodeKind::L => {
                    return NodeSt::number(
                        (l < r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    )
                }
                NodeKind::LE => {
                    return NodeSt::number(
                        (l <= r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    )
                }
                NodeKind::G => {
                    return NodeSt::number(
                        (l > r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    )
                }
                NodeKind::GE => {
                    return NodeSt::number(
                        (l >= r) as i8,
                        Loc::new(llf, (llf as i8 + ((l % r) / 10) + 1) as u8),
                    )
                }
                _ => unreachable!(),
            }
        }
        NodeKind::Num(_) => return ns,
        _ => unreachable!(),
    };
}
