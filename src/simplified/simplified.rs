use super::super::location::location::*;
use super::super::node::node::*;

pub fn simplified(ns: NodeSt) -> NodeSt {
    exec(ns)
}

fn exec(ns: NodeSt) -> NodeSt {
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
            let ln = simplified(ns.lhs.as_ref().unwrap().as_ref().to_owned());
            let llf = ln.c.loc.f;
            let l = match ln.c.value {
                NodeKind::Num(n) => n,
                _ => unreachable!(),
            };
            let r = match simplified(ns.rhs.as_ref().unwrap().as_ref().to_owned())
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
