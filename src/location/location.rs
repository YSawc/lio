#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Loc {
    pub f: u8,
    pub e: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Annot<T> {
    pub value: T,
    pub loc: Loc,
}

impl<T> Annot<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

impl Loc {
    pub fn new(f: u8, e: u8) -> Self {
        Self { f, e }
    }
}
