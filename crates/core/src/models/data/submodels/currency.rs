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

    #[test]
    fn test_display() {
        assert_eq!(Currency::USD.to_string(), "USD");
        assert_eq!(Currency::EUR.to_string(), "EUR");
        assert_eq!(Currency::GBP.to_string(), "GBP");
        assert_eq!(Currency::JPY.to_string(), "JPY");
        assert_eq!(Currency::CAD.to_string(), "CAD");
        assert_eq!(Currency::AUD.to_string(), "AUD");
        assert_eq!(Currency::CHF.to_string(), "CHF");
        assert_eq!(Currency::SEK.to_string(), "SEK");
        assert_eq!(Currency::NOK.to_string(), "NOK");
        assert_eq!(Currency::DKK.to_string(), "DKK");
        assert_eq!(Currency::CNY.to_string(), "CNY");
        assert_eq!(Currency::INR.to_string(), "INR");
        assert_eq!(Currency::BRL.to_string(), "BRL");
        assert_eq!(Currency::RUB.to_string(), "RUB");
        assert_eq!(Currency::ZAR.to_string(), "ZAR");
        assert_eq!(Currency::MXN.to_string(), "MXN");
        assert_eq!(Currency::NZD.to_string(), "NZD");
        assert_eq!(Currency::SGD.to_string(), "SGD");
        assert_eq!(Currency::HKD.to_string(), "HKD");
        assert_eq!(Currency::KRW.to_string(), "KRW");
        assert_eq!(Currency::SAR.to_string(), "SAR");
        assert_eq!(Currency::AED.to_string(), "AED");
        assert_eq!(Currency::TRY.to_string(), "TRY");
        assert_eq!(Currency::PLN.to_string(), "PLN");
        assert_eq!(Currency::THB.to_string(), "THB");
        assert_eq!(Currency::TWD.to_string(), "TWD");
        assert_eq!(Currency::XAF.to_string(), "XAF");
        assert_eq!(Currency::XOF.to_string(), "XOF");
        assert_eq!(Currency::XCD.to_string(), "XCD");
        assert_eq!(Currency::XBT.to_string(), "XBT");
        assert_eq!(Currency::ETH.to_string(), "ETH");
        assert_eq!(Currency::XRD.to_string(), "XRD");
        assert_eq!(Currency::DOT.to_string(), "DOT");
    }

    #[test]
    fn test_debug() {
        assert_debug_snapshot!(Currency::USD, @"USD");
        assert_debug_snapshot!(Currency::EUR, @"EUR");
        assert_debug_snapshot!(Currency::GBP, @"GBP");
        assert_debug_snapshot!(Currency::JPY, @"JPY");
        assert_debug_snapshot!(Currency::CAD, @"CAD");
        assert_debug_snapshot!(Currency::AUD, @"AUD");
        assert_debug_snapshot!(Currency::CHF, @"CHF");
        assert_debug_snapshot!(Currency::SEK, @"SEK");
        assert_debug_snapshot!(Currency::NOK, @"NOK");
        assert_debug_snapshot!(Currency::DKK, @"DKK");
        assert_debug_snapshot!(Currency::CNY, @"CNY");
        assert_debug_snapshot!(Currency::INR, @"INR");
        assert_debug_snapshot!(Currency::BRL, @"BRL");
        assert_debug_snapshot!(Currency::RUB, @"RUB");
        assert_debug_snapshot!(Currency::ZAR, @"ZAR");
        assert_debug_snapshot!(Currency::MXN, @"MXN");
        assert_debug_snapshot!(Currency::NZD, @"NZD");
        assert_debug_snapshot!(Currency::SGD, @"SGD");
        assert_debug_snapshot!(Currency::HKD, @"HKD");
        assert_debug_snapshot!(Currency::KRW, @"KRW");
        assert_debug_snapshot!(Currency::SAR, @"SAR");
        assert_debug_snapshot!(Currency::AED, @"AED");
        assert_debug_snapshot!(Currency::TRY, @"TRY");
        assert_debug_snapshot!(Currency::PLN, @"PLN");
        assert_debug_snapshot!(Currency::THB, @"THB");
        assert_debug_snapshot!(Currency::TWD, @"TWD");
        assert_debug_snapshot!(Currency::XAF, @"XAF");
        assert_debug_snapshot!(Currency::XOF, @"XOF");
        assert_debug_snapshot!(Currency::XCD, @"XCD");
        assert_debug_snapshot!(Currency::XBT, @"XBT");
        assert_debug_snapshot!(Currency::ETH, @"ETH");
        assert_debug_snapshot!(Currency::XRD, @"XRD");
        assert_debug_snapshot!(Currency::DOT, @"DOT");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Currency::from_str("USD").unwrap(), Currency::USD);
        assert_eq!(Currency::from_str("EUR").unwrap(), Currency::EUR);
        assert_eq!(Currency::from_str("GBP").unwrap(), Currency::GBP);
        assert_eq!(Currency::from_str("JPY").unwrap(), Currency::JPY);
        assert_eq!(Currency::from_str("CAD").unwrap(), Currency::CAD);
        assert_eq!(Currency::from_str("AUD").unwrap(), Currency::AUD);
        assert_eq!(Currency::from_str("CHF").unwrap(), Currency::CHF);
        assert_eq!(Currency::from_str("SEK").unwrap(), Currency::SEK);
        assert_eq!(Currency::from_str("NOK").unwrap(), Currency::NOK);
        assert_eq!(Currency::from_str("DKK").unwrap(), Currency::DKK);
        assert_eq!(Currency::from_str("CNY").unwrap(), Currency::CNY);
        assert_eq!(Currency::from_str("INR").unwrap(), Currency::INR);
        assert_eq!(Currency::from_str("BRL").unwrap(), Currency::BRL);
        assert_eq!(Currency::from_str("RUB").unwrap(), Currency::RUB);
        assert_eq!(Currency::from_str("ZAR").unwrap(), Currency::ZAR);
        assert_eq!(Currency::from_str("MXN").unwrap(), Currency::MXN);
        assert_eq!(Currency::from_str("NZD").unwrap(), Currency::NZD);
        assert_eq!(Currency::from_str("SGD").unwrap(), Currency::SGD);
        assert_eq!(Currency::from_str("HKD").unwrap(), Currency::HKD);
        assert_eq!(Currency::from_str("KRW").unwrap(), Currency::KRW);
        assert_eq!(Currency::from_str("SAR").unwrap(), Currency::SAR);
        assert_eq!(Currency::from_str("AED").unwrap(), Currency::AED);
        assert_eq!(Currency::from_str("TRY").unwrap(), Currency::TRY);
        assert_eq!(Currency::from_str("PLN").unwrap(), Currency::PLN);
        assert_eq!(Currency::from_str("THB").unwrap(), Currency::THB);
        assert_eq!(Currency::from_str("TWD").unwrap(), Currency::TWD);
        assert_eq!(Currency::from_str("XAF").unwrap(), Currency::XAF);
        assert_eq!(Currency::from_str("XOF").unwrap(), Currency::XOF);
        assert_eq!(Currency::from_str("XCD").unwrap(), Currency::XCD);
        assert_eq!(Currency::from_str("XBT").unwrap(), Currency::XBT);
        assert_eq!(Currency::from_str("ETH").unwrap(), Currency::ETH);
        assert_eq!(Currency::from_str("XRD").unwrap(), Currency::XRD);
        assert_eq!(Currency::from_str("DOT").unwrap(), Currency::DOT);
    }

    #[test]
    fn sample() {
        let sample_currency = Currency::sample();
        assert_eq!(sample_currency, Currency::EUR);
    }
}
