use std::ops::Deref;

use derive_more::Display;

/// Number of characters in an ISO 3166-1 alpha-2 country code, e.g. `SE`.
const COUNTRY_CODE_LEN: usize = 2;

/// An [ISO 3166-1 alpha-2][iso] country code, e.g. `SE` for Sweden, `GB` for
/// the United Kingdom or `US` for the United States.
///
/// Always exactly two ASCII uppercase letters. Used to look up public holidays
/// for the country of a vendor.
///
/// [iso]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
#[derive(Clone, Debug, Display, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CountryCode(String);

/// Error returned when constructing a [`CountryCode`] from an invalid string.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display(
    "Invalid ISO 3166-1 alpha-2 country code: '{invalid}', expected exactly two ASCII letters."
)]
pub struct InvalidCountryCode {
    /// The offending input string.
    pub invalid: String,
}

impl std::error::Error for InvalidCountryCode {}

impl CountryCode {
    /// Constructs a [`CountryCode`] from a known-valid static code.
    ///
    /// Used internally by [`Self::from_country_name`] where the codes come from
    /// a curated table and are guaranteed valid.
    fn from_static(code: &'static str) -> Self {
        Self::new(code).expect("Curated country codes must be valid")
    }

    /// Constructs a [`CountryCode`], validating that `code` is exactly two ASCII
    /// letters. The code is upper-cased.
    ///
    /// # Errors
    /// Returns [`InvalidCountryCode`] if `code` is not two ASCII letters.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_foundation;
    /// use klirr_foundation::*;
    ///
    /// assert_eq!(CountryCode::new("se").unwrap().as_str(), "SE");
    /// assert!(CountryCode::new("Sweden").is_err());
    /// assert!(CountryCode::new("S1").is_err());
    /// ```
    pub fn new(code: impl AsRef<str>) -> Result<Self, InvalidCountryCode> {
        let code = code.as_ref();
        let is_valid =
            code.len() == COUNTRY_CODE_LEN && code.chars().all(|c| c.is_ascii_alphabetic());
        if is_valid {
            Ok(Self(code.to_ascii_uppercase()))
        } else {
            Err(InvalidCountryCode {
                invalid: code.to_owned(),
            })
        }
    }

    /// Returns the two-letter code, e.g. `"SE"`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Resolves a free-text country name (as stored in a postal address) into an
    /// ISO 3166-1 alpha-2 code, handling common aliases (`"England"`, `"UK"` →
    /// `GB`; `"USA"` → `US`; `"Sverige"` → `SE`). The two-letter code itself is
    /// also accepted.
    ///
    /// Returns `None` for unrecognized names — callers should degrade gracefully
    /// (skip holiday deduction) rather than fail.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_foundation;
    /// use klirr_foundation::*;
    ///
    /// assert_eq!(CountryCode::from_country_name("Sweden").unwrap().as_str(), "SE");
    /// assert_eq!(CountryCode::from_country_name("England").unwrap().as_str(), "GB");
    /// assert_eq!(CountryCode::from_country_name("  united states of america ").unwrap().as_str(), "US");
    /// assert_eq!(CountryCode::from_country_name("SE").unwrap().as_str(), "SE");
    /// assert!(CountryCode::from_country_name("Atlantis").is_none());
    /// ```
    pub fn from_country_name(name: impl AsRef<str>) -> Option<Self> {
        let normalized = name.as_ref().trim().to_ascii_lowercase();
        code_for_normalized_country_name(&normalized).map(Self::from_static)
    }
}

impl Deref for CountryCode {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for CountryCode {
    type Err = InvalidCountryCode;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// Maps a normalized (trimmed, lower-cased) country name or two-letter code to
/// its ISO 3166-1 alpha-2 code.
///
/// This is intentionally a flat lookup table — it mirrors the breadth of the
/// holiday API's supported countries and is expected to grow by simple
/// extension. Names listed here cover the most common spellings and aliases; a
/// missing entry simply means no holiday deduction for that vendor country.
fn code_for_normalized_country_name(normalized: &str) -> Option<&'static str> {
    let code = match normalized {
        // Nordics
        "sweden" | "sverige" | "se" => "SE",
        "norway" | "norge" | "no" => "NO",
        "denmark" | "danmark" | "dk" => "DK",
        "finland" | "suomi" | "fi" => "FI",
        "iceland" | "ísland" | "island" | "is" => "IS",
        // United Kingdom & constituent countries
        "united kingdom" | "uk" | "u.k." | "great britain" | "britain" | "england" | "scotland"
        | "wales" | "northern ireland" | "gb" => "GB",
        "ireland" | "éire" | "eire" | "ie" => "IE",
        // North America
        "united states"
        | "united states of america"
        | "usa"
        | "u.s.a."
        | "u.s."
        | "america"
        | "us" => "US",
        "canada" | "ca" => "CA",
        "mexico" | "méxico" | "mx" => "MX",
        // Western & Central Europe
        "germany" | "deutschland" | "de" => "DE",
        "france" | "fr" => "FR",
        "spain" | "españa" | "espana" | "es" => "ES",
        "portugal" | "pt" => "PT",
        "italy" | "italia" | "it" => "IT",
        "netherlands" | "the netherlands" | "holland" | "nederland" | "nl" => "NL",
        "belgium" | "belgië" | "belgique" | "be" => "BE",
        "luxembourg" | "lu" => "LU",
        "switzerland" | "schweiz" | "suisse" | "ch" => "CH",
        "austria" | "österreich" | "osterreich" | "at" => "AT",
        "poland" | "polska" | "pl" => "PL",
        "czechia" | "czech republic" | "cz" => "CZ",
        "slovakia" | "sk" => "SK",
        "hungary" | "hu" => "HU",
        "greece" | "gr" => "GR",
        "romania" | "ro" => "RO",
        "bulgaria" | "bg" => "BG",
        "croatia" | "hr" => "HR",
        "slovenia" | "si" => "SI",
        "estonia" | "ee" => "EE",
        "latvia" | "lv" => "LV",
        "lithuania" | "lt" => "LT",
        // Rest of the world (common business locales)
        "australia" | "au" => "AU",
        "new zealand" | "nz" => "NZ",
        "japan" | "jp" => "JP",
        "south korea" | "korea" | "kr" => "KR",
        "china" | "cn" => "CN",
        "india" | "in" => "IN",
        "singapore" | "sg" => "SG",
        "hong kong" | "hk" => "HK",
        "brazil" | "brasil" | "br" => "BR",
        "argentina" | "ar" => "AR",
        "south africa" | "za" => "ZA",
        _ => return None,
    };
    Some(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use test_log::test;

    type Sut = CountryCode;

    #[test]
    fn new_uppercases() {
        assert_eq!(Sut::new("se").unwrap().as_str(), "SE");
        assert_eq!(Sut::new("Gb").unwrap().as_str(), "GB");
    }

    #[test]
    fn new_rejects_invalid() {
        assert!(Sut::new("Sweden").is_err());
        assert!(Sut::new("S").is_err());
        assert!(Sut::new("S1").is_err());
        assert!(Sut::new("").is_err());
    }

    #[test]
    fn from_str_delegates_to_new() {
        assert_eq!(Sut::from_str("us").unwrap().as_str(), "US");
        assert!(Sut::from_str("nope-nope").is_err());
    }

    #[test]
    fn resolves_sweden_variants() {
        for name in ["Sweden", "sverige", "SE", " se ", "SWEDEN"] {
            assert_eq!(
                Sut::from_country_name(name).unwrap().as_str(),
                "SE",
                "{name}"
            );
        }
    }

    #[test]
    fn resolves_uk_aliases() {
        for name in [
            "United Kingdom",
            "UK",
            "England",
            "Scotland",
            "Great Britain",
            "GB",
        ] {
            assert_eq!(
                Sut::from_country_name(name).unwrap().as_str(),
                "GB",
                "{name}"
            );
        }
    }

    #[test]
    fn resolves_us_aliases() {
        for name in ["United States", "USA", "U.S.A.", "America", "US"] {
            assert_eq!(
                Sut::from_country_name(name).unwrap().as_str(),
                "US",
                "{name}"
            );
        }
    }

    #[test]
    fn unknown_country_is_none() {
        assert!(Sut::from_country_name("Atlantis").is_none());
        assert!(Sut::from_country_name("").is_none());
    }

    #[test]
    fn display_equals_as_str() {
        let code = Sut::new("de").unwrap();
        assert_eq!(code.to_string(), code.as_str());
        assert_eq!(&*code, "DE");
    }

    #[test]
    fn invalid_country_code_error_displays_input() {
        let error = Sut::new("toolong").unwrap_err();
        assert!(error.to_string().contains("toolong"));
    }
}
