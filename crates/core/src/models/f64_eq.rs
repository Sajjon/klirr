use std::cmp::Ordering;

use crate::prelude::*;

/// A floating point number that is comparable with a tolerance, guaranteed to
/// not be NaN or +/- infinity.
#[derive(
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    derive_more::Debug,
    Display,
    derive_more::Deref,
    derive_more::Into,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::Mul,
    derive_more::Div,
)]
#[debug("{}", self.0)]
#[display("{}", self.0)]
#[serde(transparent)]
pub struct F64(f64);

impl From<f64> for F64 {
    /// Panics if the value is NaN or infinite.
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<i32> for F64 {
    fn from(value: i32) -> Self {
        Self::new(value as f64)
    }
}

impl FromStr for F64 {
    type Err = crate::Error;

    /// Parses a string slice to create an `F64`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<f64>().map_err(|e| Error::InvalidF64String {
            bad_value: s.to_string(),
            reason: format!("{:?}", e),
        })?;
        Ok(Self::new(value))
    }
}

impl F64 {
    /// Panics if the value is NaN or infinite.
    #[must_use]
    pub const fn new(value: f64) -> Self {
        assert!(
            !value.is_nan() && !value.is_infinite(),
            "F64 cannot be NaN or infinite"
        );
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

    #[test]
    fn test_from_str_valid() {
        let sut = Sut::from_str("1.23").expect("Failed to parse F64 from string");
        assert_eq!(sut, Sut::new(1.23));
    }

    #[test]
    fn test_from_str_invalid_all_reasons() {
        let invalid_strings = ["abc", "1.23.45", "1,23"];
        for &s in &invalid_strings {
            let result = Sut::from_str(s);
            assert!(result.is_err(), "Expected error for '{}', but got Ok", s);
        }
    }

    #[test]
    #[should_panic(expected = "F64 cannot be NaN or infinite")]
    fn test_from_str_panics_on_nan() {
        let _ = Sut::from_str("NaN");
    }

    #[test]
    #[should_panic(expected = "F64 cannot be NaN or infinite")]
    fn test_from_str_panics_on_inf() {
        let _ = Sut::from_str("inf");
    }

    #[test]
    #[should_panic(expected = "F64 cannot be NaN or infinite")]
    fn test_from_str_panics_on_negative_inf() {
        let _ = Sut::from_str("-inf");
    }

    #[test]
    #[should_panic(expected = "F64 cannot be NaN or infinite")]
    fn test_new_panics_on_nan() {
        let _ = Sut::new(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "F64 cannot be NaN or infinite")]
    fn test_new_panics_on_infinite() {
        let _ = Sut::new(f64::INFINITY);
    }

    #[test]
    fn test_from_i32() {
        let sut = Sut::from(42);
        assert_eq!(sut, Sut::new(42.0));
    }
}
