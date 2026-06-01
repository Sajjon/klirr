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

/// Error returned when constructing a [`CountryCode`] from an unsupported or
/// malformed string.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display(
    "Invalid country code: '{invalid}', expected a supported two-letter ISO 3166-1 alpha-2 code."
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

    /// Constructs a [`CountryCode`], validating that `code` is a two-letter code
    /// the holiday API actually supports. The code is upper-cased.
    ///
    /// Well-formed-but-unsupported codes (e.g. `"ZZ"`, or `"UK"` whose ISO code
    /// is `GB`) are rejected, so a `CountryCode` always denotes a country we can
    /// fetch holidays for. Use [`Self::from_country_name`] to resolve aliases
    /// such as `"UK"` to their ISO code.
    ///
    /// # Errors
    /// Returns [`InvalidCountryCode`] if `code` is not a supported two-letter
    /// ISO 3166-1 alpha-2 code.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_foundation;
    /// use klirr_foundation::*;
    ///
    /// assert_eq!(CountryCode::new("se").unwrap().as_str(), "SE");
    /// assert!(CountryCode::new("Sweden").is_err());
    /// assert!(CountryCode::new("S1").is_err());
    /// assert!(CountryCode::new("ZZ").is_err()); // well-formed but unsupported
    /// assert!(CountryCode::new("UK").is_err()); // ISO code is GB
    /// ```
    pub fn new(code: impl AsRef<str>) -> Result<Self, InvalidCountryCode> {
        let code = code.as_ref();
        let well_formed =
            code.len() == COUNTRY_CODE_LEN && code.chars().all(|c| c.is_ascii_alphabetic());
        let upper = code.to_ascii_uppercase();
        if well_formed && is_supported_code(&upper) {
            Ok(Self(upper))
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
    /// ISO 3166-1 alpha-2 code, covering every country served by the holiday API
    /// and handling common aliases (`"England"`, `"UK"` → `GB`; `"USA"` → `US`;
    /// `"Sverige"` → `SE`) plus accent-free spellings (`"Turkey"`, `"Aland
    /// Islands"`). The two-letter code itself is also accepted.
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
    /// assert_eq!(CountryCode::from_country_name("Türkiye").unwrap().as_str(), "TR");
    /// assert_eq!(CountryCode::from_country_name("Turkey").unwrap().as_str(), "TR");
    /// assert_eq!(CountryCode::from_country_name("SE").unwrap().as_str(), "SE");
    /// assert!(CountryCode::from_country_name("Atlantis").is_none());
    /// ```
    pub fn from_country_name(name: impl AsRef<str>) -> Option<Self> {
        let normalized = name.as_ref().trim().to_lowercase();
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

/// Returns `true` if `upper` (an upper-cased two-letter code) is a country the
/// holiday API serves. A code is supported iff it appears in the resolver table
/// as its own self-mapping.
fn is_supported_code(upper: &str) -> bool {
    code_for_normalized_country_name(&upper.to_ascii_lowercase()).is_some_and(|code| code == upper)
}

/// Maps a normalized (trimmed, Unicode-lower-cased) country name or two-letter
/// code to its ISO 3166-1 alpha-2 code.
///
/// This is intentionally a flat lookup table covering every country served by
/// the [Nager.Date](https://date.nager.at/) holiday API (the only countries we
/// can fetch holidays for). Each arm lists the English name, the ISO code, an
/// accent-free spelling, and notable aliases. A missing entry simply means no
/// holiday deduction for that vendor country.
fn code_for_normalized_country_name(normalized: &str) -> Option<&'static str> {
    let code = match normalized {
        "andorra" | "ad" => "AD",
        "albania" | "al" => "AL",
        "armenia" | "am" => "AM",
        "argentina" | "ar" => "AR",
        "austria" | "at" | "osterreich" | "österreich" => "AT",
        "australia" | "au" => "AU",
        "åland islands" | "ax" | "aland islands" => "AX",
        "bosnia and herzegovina" | "ba" => "BA",
        "barbados" | "bb" => "BB",
        "bangladesh" | "bd" => "BD",
        "belgium" | "be" | "belgique" | "belgië" | "belgie" => "BE",
        "bulgaria" | "bg" => "BG",
        "benin" | "bj" => "BJ",
        "bolivia" | "bo" => "BO",
        "brazil" | "br" | "brasil" => "BR",
        "bahamas" | "bs" => "BS",
        "botswana" | "bw" => "BW",
        "belarus" | "by" => "BY",
        "belize" | "bz" => "BZ",
        "canada" | "ca" => "CA",
        "dr congo" | "cd" | "democratic republic of the congo" | "congo kinshasa" => "CD",
        "congo" | "cg" | "republic of the congo" | "congo brazzaville" => "CG",
        "switzerland" | "ch" | "schweiz" | "suisse" => "CH",
        "chile" | "cl" => "CL",
        "china" | "cn" => "CN",
        "colombia" | "co" => "CO",
        "costa rica" | "cr" => "CR",
        "cuba" | "cu" => "CU",
        "cyprus" | "cy" => "CY",
        "czechia" | "cz" | "czech republic" => "CZ",
        "germany" | "de" | "deutschland" => "DE",
        "denmark" | "dk" | "danmark" => "DK",
        "dominican republic" | "do" => "DO",
        "ecuador" | "ec" => "EC",
        "estonia" | "ee" => "EE",
        "egypt" | "eg" => "EG",
        "spain" | "es" | "espana" | "españa" => "ES",
        "finland" | "fi" | "suomi" => "FI",
        "faroe islands" | "fo" => "FO",
        "france" | "fr" => "FR",
        "gabon" | "ga" => "GA",
        "united kingdom" | "gb" | "uk" | "u.k." | "great britain" | "britain" | "england"
        | "scotland" | "wales" | "northern ireland" => "GB",
        "grenada" | "gd" => "GD",
        "georgia" | "ge" => "GE",
        "guernsey" | "gg" => "GG",
        "ghana" | "gh" => "GH",
        "gibraltar" | "gi" => "GI",
        "greenland" | "gl" => "GL",
        "gambia" | "gm" => "GM",
        "greece" | "gr" => "GR",
        "guatemala" | "gt" => "GT",
        "guyana" | "gy" => "GY",
        "hong kong" | "hk" => "HK",
        "honduras" | "hn" => "HN",
        "croatia" | "hr" => "HR",
        "haiti" | "ht" => "HT",
        "hungary" | "hu" => "HU",
        "indonesia" | "id" => "ID",
        "ireland" | "ie" | "eire" | "éire" => "IE",
        "isle of man" | "im" => "IM",
        "iceland" | "is" | "island" | "ísland" => "IS",
        "italy" | "it" | "italia" => "IT",
        "jersey" | "je" => "JE",
        "jamaica" | "jm" => "JM",
        "japan" | "jp" => "JP",
        "kenya" | "ke" => "KE",
        "south korea" | "kr" | "korea" => "KR",
        "kazakhstan" | "kz" => "KZ",
        "liechtenstein" | "li" => "LI",
        "lesotho" | "ls" => "LS",
        "lithuania" | "lt" => "LT",
        "luxembourg" | "lu" => "LU",
        "latvia" | "lv" => "LV",
        "morocco" | "ma" => "MA",
        "monaco" | "mc" => "MC",
        "moldova" | "md" => "MD",
        "montenegro" | "me" => "ME",
        "madagascar" | "mg" => "MG",
        "north macedonia" | "mk" | "macedonia" => "MK",
        "mongolia" | "mn" => "MN",
        "montserrat" | "ms" => "MS",
        "malta" | "mt" => "MT",
        "mexico" | "mx" | "méxico" => "MX",
        "mozambique" | "mz" => "MZ",
        "namibia" | "na" => "NA",
        "niger" | "ne" => "NE",
        "nigeria" | "ng" => "NG",
        "nicaragua" | "ni" => "NI",
        "netherlands" | "nl" | "the netherlands" | "holland" | "nederland" => "NL",
        "norway" | "no" | "norge" => "NO",
        "new zealand" | "nz" => "NZ",
        "panama" | "pa" => "PA",
        "peru" | "pe" => "PE",
        "papua new guinea" | "pg" => "PG",
        "philippines" | "ph" => "PH",
        "poland" | "pl" | "polska" => "PL",
        "puerto rico" | "pr" => "PR",
        "portugal" | "pt" => "PT",
        "paraguay" | "py" => "PY",
        "romania" | "ro" => "RO",
        "serbia" | "rs" => "RS",
        "russia" | "ru" | "russian federation" => "RU",
        "seychelles" | "sc" => "SC",
        "sweden" | "se" | "sverige" => "SE",
        "singapore" | "sg" => "SG",
        "slovenia" | "si" => "SI",
        "svalbard and jan mayen" | "sj" => "SJ",
        "slovakia" | "sk" => "SK",
        "san marino" | "sm" => "SM",
        "suriname" | "sr" => "SR",
        "el salvador" | "sv" => "SV",
        "tunisia" | "tn" => "TN",
        "türkiye" | "tr" | "turkiye" | "turkey" => "TR",
        "ukraine" | "ua" => "UA",
        "uganda" | "ug" => "UG",
        "united states"
        | "us"
        | "united states of america"
        | "usa"
        | "u.s.a."
        | "u.s."
        | "america" => "US",
        "uruguay" | "uy" => "UY",
        "vatican city" | "va" | "vatican" | "holy see" => "VA",
        "venezuela" | "ve" => "VE",
        "vietnam" | "vn" | "viet nam" => "VN",
        "south africa" | "za" => "ZA",
        "zimbabwe" | "zw" => "ZW",
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
    fn new_rejects_wellformed_but_unsupported_codes() {
        // Two ASCII letters but not a supported ISO/Nager code.
        assert!(Sut::new("ZZ").is_err());
        assert!(Sut::new("QQ").is_err());
        // "UK" is a common alias but not an ISO code (that's GB).
        assert!(Sut::new("UK").is_err());
        assert_eq!(Sut::from_country_name("UK").unwrap().as_str(), "GB");
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

    /// Every ISO code served by the Nager.Date holiday API. Used to assert the
    /// resolver covers the full set and that each code is a valid `CountryCode`.
    const ALL_SUPPORTED_CODES: &[&str] = &[
        "AD", "AL", "AM", "AR", "AT", "AU", "AX", "BA", "BB", "BD", "BE", "BG", "BJ", "BO", "BR",
        "BS", "BW", "BY", "BZ", "CA", "CD", "CG", "CH", "CL", "CN", "CO", "CR", "CU", "CY", "CZ",
        "DE", "DK", "DO", "EC", "EE", "EG", "ES", "FI", "FO", "FR", "GA", "GB", "GD", "GE", "GG",
        "GH", "GI", "GL", "GM", "GR", "GT", "GY", "HK", "HN", "HR", "HT", "HU", "ID", "IE", "IM",
        "IS", "IT", "JE", "JM", "JP", "KE", "KR", "KZ", "LI", "LS", "LT", "LU", "LV", "MA", "MC",
        "MD", "ME", "MG", "MK", "MN", "MS", "MT", "MX", "MZ", "NA", "NE", "NG", "NI", "NL", "NO",
        "NZ", "PA", "PE", "PG", "PH", "PL", "PR", "PT", "PY", "RO", "RS", "RU", "SC", "SE", "SG",
        "SI", "SJ", "SK", "SM", "SR", "SV", "TN", "TR", "UA", "UG", "US", "UY", "VA", "VE", "VN",
        "ZA", "ZW",
    ];

    #[test]
    fn supports_full_nager_country_set() {
        assert_eq!(
            ALL_SUPPORTED_CODES.len(),
            122,
            "resolver country set changed unexpectedly"
        );
    }

    #[test]
    fn every_supported_code_is_a_valid_country_code() {
        for code in ALL_SUPPORTED_CODES {
            assert!(
                Sut::new(code).is_ok(),
                "{code} should be a valid CountryCode"
            );
        }
    }

    #[test]
    fn every_supported_code_self_resolves() {
        for code in ALL_SUPPORTED_CODES {
            // Both the bare code and its lower-cased form must resolve to itself.
            assert_eq!(
                Sut::from_country_name(code).unwrap().as_str(),
                *code,
                "upper {code}"
            );
            assert_eq!(
                Sut::from_country_name(code.to_lowercase())
                    .unwrap()
                    .as_str(),
                *code,
                "lower {code}"
            );
        }
    }

    #[test]
    fn resolves_accented_and_ascii_spellings() {
        let cases = [
            ("Türkiye", "TR"),
            ("turkiye", "TR"),
            ("Turkey", "TR"),
            ("Åland Islands", "AX"),
            ("aland islands", "AX"),
            ("España", "ES"),
            ("Espana", "ES"),
            ("México", "MX"),
            ("Mexico", "MX"),
        ];
        for (name, code) in cases {
            assert_eq!(
                Sut::from_country_name(name).unwrap().as_str(),
                code,
                "{name}"
            );
        }
    }

    #[test]
    fn resolves_canonical_names_across_regions() {
        let cases = [
            ("Norway", "NO"),
            ("Germany", "DE"),
            ("Japan", "JP"),
            ("Brazil", "BR"),
            ("South Africa", "ZA"),
            ("New Zealand", "NZ"),
            ("Bosnia and Herzegovina", "BA"),
            ("Hong Kong", "HK"),
            ("Vatican City", "VA"),
            ("Papua New Guinea", "PG"),
        ];
        for (name, code) in cases {
            assert_eq!(
                Sut::from_country_name(name).unwrap().as_str(),
                code,
                "{name}"
            );
        }
    }
}
