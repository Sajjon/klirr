use std::fmt;
use std::str::FromStr;

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DeserializeFromStr, SerializeDisplay)]
pub enum Currency {
    /// 🇺🇸 United States Dollar (USD), used in the United States
    USD,
    /// 🇪🇺 Euro (EUR), used in eurozone countries such as Germany, France, Italy, etc.
    EUR,
    /// 🇬🇧 British Pound Sterling (GBP), used in the United Kingdom
    GBP,
    /// 🇯🇵 Japanese Yen (JPY), used in Japan
    JPY,
    /// 🇨🇦 Canadian Dollar (CAD), used in Canada
    CAD,
    /// 🇦🇺 Australian Dollar (AUD), used in Australia
    AUD,
    /// 🇨🇭 Swiss Franc (CHF), used in Switzerland and Liechtenstein
    CHF,
    /// 🇸🇪 Swedish Krona (SEK), used in Sweden
    SEK,
    /// 🇳🇴 Norwegian Krone (NOK), used in Norway
    NOK,
    /// 🇩🇰 Danish Krone (DKK), used in Denmark
    DKK,
    /// 🇨🇳 Chinese Yuan (CNY), used in China
    CNY,
    /// 🇮🇳 Indian Rupee (INR), used in India
    INR,
    /// 🇧🇷 Brazilian Real (BRL), used in Brazil
    BRL,
    /// 🇷🇺 Russian Ruble (RUB), used in Russia
    RUB,
    /// 🇿🇦 South African Rand (ZAR), used in South Africa
    ZAR,
    /// 🇲🇽 Mexican Peso (MXN), used in Mexico
    MXN,
    /// 🇳🇿 New Zealand Dollar (NZD), used in New Zealand
    NZD,
    /// 🇸🇬 Singapore Dollar (SGD), used in Singapore
    SGD,
    /// 🇭🇰 Hong Kong Dollar (HKD), used in Hong Kong
    HKD,
    /// 🇰🇷 South Korean Won (KRW), used in South Korea
    KRW,
    /// 🇸🇦 Saudi Riyal (SAR), used in Saudi Arabia
    SAR,
    /// 🇦🇪 UAE Dirham (AED), used in United Arab Emirates
    AED,
    /// 🇹🇷 Turkish Lira (TRY), used in Turkey
    TRY,
    /// 🇵🇱 Polish Zloty (PLN), used in Poland
    PLN,
    /// 🇹🇭 Thai Baht (THB), used in Thailand
    THB,
    /// 🇹🇼 New Taiwan Dollar (TWD), used in Taiwan
    TWD,
    /// 🌍 Central African CFA Franc (XAF), used in Cameroon, Central African Republic, Chad, Republic of the Congo, Equatorial Guinea, and Gabon
    XAF,
    /// 🌍 West African CFA Franc (XOF), used in Benin, Burkina Faso, Ivory Coast, Guinea-Bissau, Mali, Niger, Senegal, and Togo
    XOF,
    /// 🌎 Eastern Caribbean Dollar (XCD), used in Antigua and Barbuda, Dominica, Grenada, Saint Kitts and Nevis, Saint Lucia, Saint Vincent and the Grenadines, Anguilla, and Montserrat
    XCD,
    /// 🔗 Bitcoin (XBT), a decentralized cryptocurrency used globally
    XBT,
    /// 🔗 Ethereum (ETH), a decentralized cryptocurrency used globally
    ETH,
    /// 🔗 Radix (XRD), a decentralized cryptocurrency used globally
    XRD,
    /// 🔗 Polkadot (DOT), a decentralized cryptocurrency used globally
    DOT,
}

impl HasSample for Currency {
    fn sample() -> Self {
        Currency::EUR
    }
    fn sample_other() -> Self {
        Currency::USD
    }
}

// Display implementation to return ISO code
impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::CAD => "CAD",
            Currency::AUD => "AUD",
            Currency::CHF => "CHF",
            Currency::SEK => "SEK",
            Currency::NOK => "NOK",
            Currency::DKK => "DKK",
            Currency::CNY => "CNY",
            Currency::INR => "INR",
            Currency::BRL => "BRL",
            Currency::RUB => "RUB",
            Currency::ZAR => "ZAR",
            Currency::MXN => "MXN",
            Currency::NZD => "NZD",
            Currency::SGD => "SGD",
            Currency::HKD => "HKD",
            Currency::KRW => "KRW",
            Currency::SAR => "SAR",
            Currency::AED => "AED",
            Currency::TRY => "TRY",
            Currency::PLN => "PLN",
            Currency::THB => "THB",
            Currency::TWD => "TWD",
            Currency::XAF => "XAF",
            Currency::XOF => "XOF",
            Currency::XCD => "XCD",
            Currency::XBT => "XBT",
            Currency::ETH => "ETH",
            Currency::XRD => "XRD",
            Currency::DOT => "DOT",
        };
        write!(f, "{}", code)
    }
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            "CAD" => Ok(Currency::CAD),
            "AUD" => Ok(Currency::AUD),
            "CHF" => Ok(Currency::CHF),
            "SEK" => Ok(Currency::SEK),
            "NOK" => Ok(Currency::NOK),
            "DKK" => Ok(Currency::DKK),
            "CNY" => Ok(Currency::CNY),
            "INR" => Ok(Currency::INR),
            "BRL" => Ok(Currency::BRL),
            "RUB" => Ok(Currency::RUB),
            "ZAR" => Ok(Currency::ZAR),
            "MXN" => Ok(Currency::MXN),
            "NZD" => Ok(Currency::NZD),
            "SGD" => Ok(Currency::SGD),
            "HKD" => Ok(Currency::HKD),
            "KRW" => Ok(Currency::KRW),
            "SAR" => Ok(Currency::SAR),
            "AED" => Ok(Currency::AED),
            "TRY" => Ok(Currency::TRY),
            "PLN" => Ok(Currency::PLN),
            "THB" => Ok(Currency::THB),
            "TWD" => Ok(Currency::TWD),
            "XAF" => Ok(Currency::XAF),
            "XOF" => Ok(Currency::XOF),
            "XCD" => Ok(Currency::XCD),
            "XBT" => Ok(Currency::XBT),
            "ETH" => Ok(Currency::ETH),
            "XRD" => Ok(Currency::XRD),
            "DOT" => Ok(Currency::DOT),
            _ => Err(format!("unknown currency code: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use test_log::test;

    type Sut = Currency;

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
    fn test_display() {
        assert_eq!(Sut::USD.to_string(), "USD");
        assert_eq!(Sut::EUR.to_string(), "EUR");
        assert_eq!(Sut::GBP.to_string(), "GBP");
        assert_eq!(Sut::JPY.to_string(), "JPY");
        assert_eq!(Sut::CAD.to_string(), "CAD");
        assert_eq!(Sut::AUD.to_string(), "AUD");
        assert_eq!(Sut::CHF.to_string(), "CHF");
        assert_eq!(Sut::SEK.to_string(), "SEK");
        assert_eq!(Sut::NOK.to_string(), "NOK");
        assert_eq!(Sut::DKK.to_string(), "DKK");
        assert_eq!(Sut::CNY.to_string(), "CNY");
        assert_eq!(Sut::INR.to_string(), "INR");
        assert_eq!(Sut::BRL.to_string(), "BRL");
        assert_eq!(Sut::RUB.to_string(), "RUB");
        assert_eq!(Sut::ZAR.to_string(), "ZAR");
        assert_eq!(Sut::MXN.to_string(), "MXN");
        assert_eq!(Sut::NZD.to_string(), "NZD");
        assert_eq!(Sut::SGD.to_string(), "SGD");
        assert_eq!(Sut::HKD.to_string(), "HKD");
        assert_eq!(Sut::KRW.to_string(), "KRW");
        assert_eq!(Sut::SAR.to_string(), "SAR");
        assert_eq!(Sut::AED.to_string(), "AED");
        assert_eq!(Sut::TRY.to_string(), "TRY");
        assert_eq!(Sut::PLN.to_string(), "PLN");
        assert_eq!(Sut::THB.to_string(), "THB");
        assert_eq!(Sut::TWD.to_string(), "TWD");
        assert_eq!(Sut::XAF.to_string(), "XAF");
        assert_eq!(Sut::XOF.to_string(), "XOF");
        assert_eq!(Sut::XCD.to_string(), "XCD");
        assert_eq!(Sut::XBT.to_string(), "XBT");
        assert_eq!(Sut::ETH.to_string(), "ETH");
        assert_eq!(Sut::XRD.to_string(), "XRD");
        assert_eq!(Sut::DOT.to_string(), "DOT");
    }

    #[test]
    fn test_debug() {
        assert_debug_snapshot!(Sut::USD, @"USD");
        assert_debug_snapshot!(Sut::EUR, @"EUR");
        assert_debug_snapshot!(Sut::GBP, @"GBP");
        assert_debug_snapshot!(Sut::JPY, @"JPY");
        assert_debug_snapshot!(Sut::CAD, @"CAD");
        assert_debug_snapshot!(Sut::AUD, @"AUD");
        assert_debug_snapshot!(Sut::CHF, @"CHF");
        assert_debug_snapshot!(Sut::SEK, @"SEK");
        assert_debug_snapshot!(Sut::NOK, @"NOK");
        assert_debug_snapshot!(Sut::DKK, @"DKK");
        assert_debug_snapshot!(Sut::CNY, @"CNY");
        assert_debug_snapshot!(Sut::INR, @"INR");
        assert_debug_snapshot!(Sut::BRL, @"BRL");
        assert_debug_snapshot!(Sut::RUB, @"RUB");
        assert_debug_snapshot!(Sut::ZAR, @"ZAR");
        assert_debug_snapshot!(Sut::MXN, @"MXN");
        assert_debug_snapshot!(Sut::NZD, @"NZD");
        assert_debug_snapshot!(Sut::SGD, @"SGD");
        assert_debug_snapshot!(Sut::HKD, @"HKD");
        assert_debug_snapshot!(Sut::KRW, @"KRW");
        assert_debug_snapshot!(Sut::SAR, @"SAR");
        assert_debug_snapshot!(Sut::AED, @"AED");
        assert_debug_snapshot!(Sut::TRY, @"TRY");
        assert_debug_snapshot!(Sut::PLN, @"PLN");
        assert_debug_snapshot!(Sut::THB, @"THB");
        assert_debug_snapshot!(Sut::TWD, @"TWD");
        assert_debug_snapshot!(Sut::XAF, @"XAF");
        assert_debug_snapshot!(Sut::XOF, @"XOF");
        assert_debug_snapshot!(Sut::XCD, @"XCD");
        assert_debug_snapshot!(Sut::XBT, @"XBT");
        assert_debug_snapshot!(Sut::ETH, @"ETH");
        assert_debug_snapshot!(Sut::XRD, @"XRD");
        assert_debug_snapshot!(Sut::DOT, @"DOT");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Sut::from_str("USD").unwrap(), Sut::USD);
        assert_eq!(Sut::from_str("EUR").unwrap(), Sut::EUR);
        assert_eq!(Sut::from_str("GBP").unwrap(), Sut::GBP);
        assert_eq!(Sut::from_str("JPY").unwrap(), Sut::JPY);
        assert_eq!(Sut::from_str("CAD").unwrap(), Sut::CAD);
        assert_eq!(Sut::from_str("AUD").unwrap(), Sut::AUD);
        assert_eq!(Sut::from_str("CHF").unwrap(), Sut::CHF);
        assert_eq!(Sut::from_str("SEK").unwrap(), Sut::SEK);
        assert_eq!(Sut::from_str("NOK").unwrap(), Sut::NOK);
        assert_eq!(Sut::from_str("DKK").unwrap(), Sut::DKK);
        assert_eq!(Sut::from_str("CNY").unwrap(), Sut::CNY);
        assert_eq!(Sut::from_str("INR").unwrap(), Sut::INR);
        assert_eq!(Sut::from_str("BRL").unwrap(), Sut::BRL);
        assert_eq!(Sut::from_str("RUB").unwrap(), Sut::RUB);
        assert_eq!(Sut::from_str("ZAR").unwrap(), Sut::ZAR);
        assert_eq!(Sut::from_str("MXN").unwrap(), Sut::MXN);
        assert_eq!(Sut::from_str("NZD").unwrap(), Sut::NZD);
        assert_eq!(Sut::from_str("SGD").unwrap(), Sut::SGD);
        assert_eq!(Sut::from_str("HKD").unwrap(), Sut::HKD);
        assert_eq!(Sut::from_str("KRW").unwrap(), Sut::KRW);
        assert_eq!(Sut::from_str("SAR").unwrap(), Sut::SAR);
        assert_eq!(Sut::from_str("AED").unwrap(), Sut::AED);
        assert_eq!(Sut::from_str("TRY").unwrap(), Sut::TRY);
        assert_eq!(Sut::from_str("PLN").unwrap(), Sut::PLN);
        assert_eq!(Sut::from_str("THB").unwrap(), Sut::THB);
        assert_eq!(Sut::from_str("TWD").unwrap(), Sut::TWD);
        assert_eq!(Sut::from_str("XAF").unwrap(), Sut::XAF);
        assert_eq!(Sut::from_str("XOF").unwrap(), Sut::XOF);
        assert_eq!(Sut::from_str("XCD").unwrap(), Sut::XCD);
        assert_eq!(Sut::from_str("XBT").unwrap(), Sut::XBT);
        assert_eq!(Sut::from_str("ETH").unwrap(), Sut::ETH);
        assert_eq!(Sut::from_str("XRD").unwrap(), Sut::XRD);
        assert_eq!(Sut::from_str("DOT").unwrap(), Sut::DOT);
    }

    #[test]
    fn sample() {
        let sample_currency = Sut::sample();
        assert_eq!(sample_currency, Sut::EUR);
    }
}
