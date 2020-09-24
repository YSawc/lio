#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loc {
    pub f: u8,
    pub e: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annot<T> {
    pub value: T,
    pub loc: Loc,
}

impl<T> Annot<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}
