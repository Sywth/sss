use std::{fmt::Display, ops::Add};

// -------------------------------
// Formula Semantics
// -------------------------------

// Should be powerful enough for all of first order logic
pub enum Formula<VarType, FomrulaIdType> {
    Top,
    Bot,

    Con(FomrulaIdType, FomrulaIdType),
    Dis(FomrulaIdType, FomrulaIdType),

    Imp(FomrulaIdType, FomrulaIdType),

    ForAll(VarType, FomrulaIdType),
    Exists(VarType, FomrulaIdType),
}

// -------------------------------
// Solver Back End Types
// -------------------------------
pub type AtomType = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Atom(AtomType);

impl Atom {
    pub const ZERO: Atom = Atom(0);
    pub const ONE: Atom = Atom(1);
    pub const MAX: Atom = Atom(AtomType::MAX);

    fn value(self) -> AtomType {
        self.0
    }
}

impl From<AtomType> for Atom {
    fn from(v: AtomType) -> Self {
        Atom(v)
    }
}

impl From<Atom> for usize {
    fn from(a: Atom) -> usize {
        a.value() as usize
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Add for Atom {
    type Output = Atom;
    fn add(self, rhs: Atom) -> Atom {
        Atom(self.value() + rhs.value())
    }
}

// -------------------------------
// Front End Parser Types
// -------------------------------
pub type SymbolType = [u8; 16];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(SymbolType);

impl Symbol {
    fn value(self) -> SymbolType {
        self.0
    }
}

impl From<SymbolType> for Symbol {
    fn from(v: SymbolType) -> Self {
        Symbol(v)
    }
}
