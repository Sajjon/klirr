use serde_with::DeserializeFromStr;

use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    SerializeDisplay,
    DeserializeFromStr,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    Getters,
)]
#[display("#{:02x}{:02x}{:02x}", red, green, blue)]
pub struct HexColor {
    #[getset(get = "pub")]
    red: u8,
    #[getset(get = "pub")]
    green: u8,
    #[getset(get = "pub")]
    blue: u8,
}

impl Default for HexColor {
    /// Black
    fn default() -> Self {
        Self::from_str("#000000").expect("Failed to create default HexColor")
    }
}

impl HasSample for HexColor {
    fn sample() -> Self {
        Self::from_str("#8b008b").expect("Failed to create sample HexColor")
    }

    fn sample_other() -> Self {
        Self::from_str("#ff4500").expect("Failed to create sample HexColor")
    }
}

impl FromStr for HexColor {
    type Err = crate::prelude::Error;

    /// Parses a hex color string in the format "#RRGGBB" or "#RRGGBBAA".
    /// The string must start with a '#' and be followed by 6 hexadecimal
    /// digits.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    /// let color: HexColor = "#e6007a".parse().unwrap();
    /// assert_eq!(*color.red(), 230);
    /// assert_eq!(*color.green(), 0);
    /// assert_eq!(*color.blue(), 122);
    /// ```
    ///
    /// # Errors
    /// Returns an error if the string does not start with '#' or if it does not
    /// contain exactly 6 hexadecimal digits after the '#'.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') && s.len() == 7 {
            let s = &s[1..];
            let parse_u8 = |start: usize, end: usize| {
                u8::from_str_radix(&s[start..end], 16).map_err(|_| Error::InvalidHexColor {
                    invalid_string: s.to_string(),
                })
            };
            let red = parse_u8(0, 2)?;
            let green = parse_u8(2, 4)?;
            let blue = parse_u8(4, 6)?;
            Ok(Self { red, green, blue })
        } else {
            Err(Error::InvalidHexColor {
                invalid_string: s.to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    type Sut = HexColor;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn test_hex_color_from_str() {
        let color: Sut = "#e6007a".parse().unwrap();
        assert_eq!(*color.red(), 230);
        assert_eq!(*color.green(), 0);
        assert_eq!(*color.blue(), 122);
    }

    #[test]
    fn test_from_str_invalid_all_reasons() {
        let invalid_strings = [
            "#e6007",   // too short
            "#e6007a1", // too long
            "e6007a",   // missing '#'
            "#e6007g",  // invalid hex digit
            "#e6007a ", // trailing space
        ];
        for &s in &invalid_strings {
            assert!(Sut::from_str(s).is_err(), "Expected error for '{}'", s);
        }
    }

    #[test]
    fn test_hex_color_default_is_black() {
        let default_color = Sut::default();
        assert_eq!(*default_color.red(), 0);
        assert_eq!(*default_color.green(), 0);
        assert_eq!(*default_color.blue(), 0);
    }
}
