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
pub type SAtomType = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SAtom(SAtomType);

impl SAtom {
    pub const ZERO: SAtom = SAtom(0);
    pub const ONE: SAtom = SAtom(1);
    pub const MAX: SAtom = SAtom(SAtomType::MAX);

    fn value(self) -> SAtomType {
        self.0
    }
}

impl From<SAtomType> for SAtom {
    fn from(v: SAtomType) -> Self {
        SAtom(v)
    }
}

impl From<SAtom> for usize {
    fn from(a: SAtom) -> usize {
        a.value() as usize
    }
}

impl Display for SAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Add for SAtom {
    type Output = SAtom;
    fn add(self, rhs: SAtom) -> SAtom {
        SAtom(self.value() + rhs.value())
    }
}

// -------------------------------
// Front End Parser Types
// -------------------------------
pub type FLiteralType = i32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FLiteral(FLiteralType);

impl FLiteral {
    pub const ZERO: FLiteral = FLiteral(0);
    pub const ONE: FLiteral = FLiteral(1);
    pub const MAX: FLiteral = FLiteral(FLiteralType::MAX);

    fn value(self) -> FLiteralType {
        self.0
    }

    pub fn abs(self) -> FLiteral {
        FLiteral(self.value().abs())
    }
}

impl From<FLiteralType> for FLiteral {
    fn from(v: FLiteralType) -> Self {
        FLiteral(v)
    }
}

impl std::str::FromStr for FLiteral {
    type Err = <FLiteralType as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<FLiteralType>().map(FLiteral::from)
    }
}

impl Add for FLiteral {
    type Output = FLiteral;
    fn add(self, rhs: FLiteral) -> FLiteral {
        FLiteral(self.value() + rhs.value())
    }
}
