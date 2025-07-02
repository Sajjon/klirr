use crate::prelude::*;
use derive_more::FromStr;

/// A wrapper around `rust_decimal::Decimal` which serializes to and from `f64`.
///
/// We have a specific need for f64 as underlying type when serialized into JSON,
/// since we bridge to Typst dictionary and we must be able to perform arithmetic
/// with values as numbers in Typst - which we cannot do with Strings.
///
/// We don't need the full precision of `rust_decimal::Decimal` in this context,
/// so we use `f64` for serialization and deserialization, which allows us to easily
/// bridge to Typst.
#[derive(
    Clone,
    Copy,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    Debug,
    From,
    FromStr,
    Deref,
    derive_more::Mul,
    derive_more::Add,
    derive_more::AddAssign,
)]
#[from(rust_decimal::Decimal, u8, i32)]
pub struct Decimal(rust_decimal::Decimal);

impl Decimal {
    pub const ZERO: Self = Self(rust_decimal::Decimal::ZERO);
    pub const ONE: Self = Self(rust_decimal::Decimal::ONE);
}

use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

impl TryFrom<Decimal> for f64 {
    type Error = crate::Error;
    fn try_from(value: Decimal) -> Result<Self> {
        value
            .0
            .to_f64()
            .ok_or_else(|| Error::InvalidDecimalToF64Conversion {
                value: value.to_string(),
            })
    }
}
impl TryFrom<f64> for Decimal {
    type Error = crate::Error;
    fn try_from(value: f64) -> Result<Self> {
        rust_decimal::Decimal::from_f64(value)
            .ok_or(Error::InvalidDecimalFromF64Conversion { value })
            .map(Decimal)
    }
}
impl Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        f64::try_from(*self)
            .map_err(serde::ser::Error::custom)
            .and_then(|f| f.serialize(serializer))
    }
}

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        f64::deserialize(deserializer)
            .and_then(|f| Decimal::try_from(f).map_err(serde::de::Error::custom))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;
    use test_log::test;

    #[test]
    fn test_serde() {
        assert_ron_snapshot!(Decimal::from(dec!(3.14159265)));
    }

    #[test]
    fn test_decimal_from_f64_nan() {
        let result = Decimal::try_from(f64::NAN);
        assert!(result.is_err());
    }
}
