use std::cmp::Ordering;

use crate::prelude::*;

/// A floating point number that is comparable with a tolerance.
#[derive(
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    derive_more::Debug,
    Display,
    derive_more::FromStr,
    derive_more::Deref,
    derive_more::From,
    derive_more::Into,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::Mul,
    derive_more::Div,
)]
#[from(f64, i32)]
#[debug("{}", self.0)]
#[display("{}", self.0)]
#[serde(transparent)]
pub struct F64(f64);

impl F64 {
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
    pub const ZERO: Self = Self::new(0.0);
    pub const ONE: Self = Self::new(1.0);

    #[must_use]
    pub fn is_approx_eq(&self, other: &Self) -> bool {
        (*self - *other).abs() < 1e-8
    }
}

impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}
impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for F64 {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff = self.0 - other.0;
        if diff == 0.0 {
            Ordering::Equal
        } else if diff.is_sign_positive() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}
impl Eq for F64 {}
impl std::hash::Hash for F64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use test_log::test;

    use super::*;
    type Sut = F64;

    #[test]
    fn new() {
        assert_eq!(Sut::new(0.0).0, 0.0);
        assert_eq!(Sut::new(1.0).0, 1.0);
        assert_eq!(Sut::new(2.0), Sut::new(2.0));
        assert_ne!(Sut::new(0.0), Sut::new(1.0));
    }

    #[test]
    fn consts() {
        assert_eq!(Sut::ZERO, Sut::new(0.0));
    }

    #[test]
    fn ord() {
        let a = Sut::from(0.0000000000000001);
        let b = Sut::from(0.000_000_000_000_000_2);
        let c = Sut::from(0.000_000_000_000_000_3);
        assert!(b < c);
        assert!(b > a);
        assert!(a < b);
        assert!(a < c);
        assert!(c > a);
        assert!(c > b);
    }

    #[test]
    fn ord_negative() {
        let a = Sut::ZERO;
        let b = Sut::from(-0.000_000_000_000_000_2);
        let c = Sut::from(-0.000_000_000_000_000_3);
        let d = Sut::from(0.0000000000000001);
        assert!(a > b);
        assert!(a > c);
        assert!(b < a);
        assert!(c < a);
        assert!(b < d);
        assert!(c < d);
        assert!(c < b);
        assert!(b > c);
    }

    #[test]
    fn test_hash() {
        assert_eq!(
            HashSet::<Sut>::from_iter([Sut::ZERO, Sut::ZERO, Sut::from(1.1), Sut::from(1.1)]).len(),
            2
        );
    }
}
