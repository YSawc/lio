use super::super::node::node::*;

pub fn beta(mut ns: &mut NodeSt) {
    match ns.c.value {
        NodeKind::Add | NodeKind::Sub | NodeKind::Mul | NodeKind::Div | NodeKind::Sur => {
            if ns.lhs != None && ns.rhs != None {
                match ns.lhs.as_ref().unwrap().as_ref().c.value {
                    NodeKind::Num(ln) => match ns.rhs.as_ref().unwrap().as_ref().c.value {
                        NodeKind::Num(rn) => match ns.c.value {
                            NodeKind::Add => ns.c.value = NodeKind::Num(ln + rn),
                            NodeKind::Sub => ns.c.value = NodeKind::Num(ln - rn),
                            NodeKind::Mul => ns.c.value = NodeKind::Num(ln * rn),
                            NodeKind::Div => ns.c.value = NodeKind::Num(ln / rn),
                            NodeKind::Sur => ns.c.value = NodeKind::Num(ln % rn),
                            _ => (),
                        },
                        _ => (),
                    },
                    _ => (),
                }
                ns.lhs = None;
                ns.rhs = None;
            }
        }
        NodeKind::Num(_) => (),
        _ => (),
    };
}
