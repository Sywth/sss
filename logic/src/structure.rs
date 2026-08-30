#![allow(unused)]

/// Logical literal representation
///
/// encoded as:
/// ```text
/// [ n bit token | 1 negation bit ]
/// ```
/// where `n` = `32`, `64` ...
///
/// Token of id `0` gets rejected as
/// we cannot map true `0` and negated `0` cleanly
/// to an integer
#[derive(Debug, Clone, PartialEq, Eq)]
struct Lit {
    v: u32,
}

#[derive(Debug)]
enum LitCastError {
    Zero,
    Overflow,
}

// Cast int to our literal
impl TryFrom<i32> for Lit {
    type Error = LitCastError;

    fn try_from(l: i32) -> Result<Self, Self::Error> {
        if l == 0 {
            tracing::error!(?l, "cannot cast from int zero");
            return Err(LitCastError::Zero);
        }

        let token = l.unsigned_abs();
        if (token & (1_u32 << 31)) != 0 {
            tracing::error!(?l, "int too large to be cast to literal");
            return Err(LitCastError::Overflow);
        }

        let is_negated = l < 0;
        Ok(Self {
            // bool as uint is 1 for true and 0 for false in rust
            v: (token << 1) | (is_negated as u32),
        })
    }
}

// Cast our literal to int
impl TryFrom<Lit> for i32 {
    type Error = LitCastError;

    fn try_from(l: Lit) -> Result<Self, Self::Error> {
        let token = l.v >> 1;
        if token == 0 {
            tracing::error!("token was 0, bad state");
            return Err(Self::Error::Zero);
        }

        // multiply by +1 for truths and -1 for falsities
        let magnitude = token as i32;
        Ok(magnitude * (1 - (l.is_negated() as i32) * 2))
    }
}

impl Lit {
    fn get_token(&self) -> u32 {
        self.v >> 1
    }

    fn is_negated(&self) -> bool {
        (self.v & 1_u32) == 1
    }

    fn negate(&self) -> Self {
        Self { v: self.v ^ 1 }
    }
}

// ===============================||
// ------------------------ Tests |>
// ===============================||
#[test]
fn test_parse_cycle() {
    for value in [1, -1, 359, -983, i32::MAX, -i32::MAX] {
        assert_eq!(
            i32::try_from(Lit::try_from(value).unwrap()).unwrap(),
            value
        );
    }
}

#[test]
fn test_zero_returns_err() {
    assert!(matches!(Lit::try_from(0), Err(LitCastError::Zero)));
}

#[test]
fn test_i32_min_returns_token_overflow() {
    assert!(matches!(
        Lit::try_from(i32::MIN),
        Err(LitCastError::Overflow)
    ));
}

#[test]
fn test_upper_bound_is_safe() {
    let safe_lb = i32::MIN + 1;
    let safe_lb_lit = Lit::try_from(safe_lb).unwrap();

    assert!(safe_lb_lit.is_negated());
    assert_eq!(safe_lb_lit.get_token(), safe_lb.unsigned_abs());
}

#[test]
fn test_zero_token_returns_err() {
    assert!(matches!(
        i32::try_from(Lit { v: 0 }),
        Err(LitCastError::Zero)
    ));
    assert!(matches!(
        i32::try_from(Lit { v: 1 }),
        Err(LitCastError::Zero)
    ));
}

#[test]
fn test_negated_literals() {
    let mut lit: Lit = Lit::try_from(4159).unwrap().negate();
    assert!(lit.is_negated());
    assert_eq!(i32::try_from(lit).unwrap(), -4159);
}

#[test]
fn test_negate_symmetry() {
    for value in [1, -1, i32::MAX, -i32::MAX] {
        let lit: Lit = Lit::try_from(value).unwrap();
        let lit = lit.negate();
        assert_eq!(i32::try_from(lit).unwrap(), -value);
    }
}

#[test]
fn test_double_negate() {
    let lit: Lit = Lit::try_from(232452).unwrap();
    assert_eq!(lit.negate().negate(), lit);
}
