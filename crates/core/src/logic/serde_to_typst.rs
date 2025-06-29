use crate::prelude::*;
use indoc::indoc;

/// Empty Marker trait
pub trait ToTypst {}

pub trait ToTypstFn: ToTypst {
    /// Converts the implementing type into a Typst function returning a dictionary.
    fn to_typst_fn(&self) -> String;
}

impl<T: ToTypst + Serialize> ToTypstFn for T {
    /// Converts this  `Serialize`able Rust struct into Typst syntax.
    fn to_typst_fn(&self) -> String {
        let value = serde_json::to_value(self).expect("Serialization failed");
        format!(
            indoc! {r#"
        #let provide() = {{
          {}
        }}
        "#},
            to_typst_value(&value, 0)
        )
    }
}

/// Recursively converts a serde_json::Value into pretty-printed Typst syntax.
fn to_typst_value(value: &Value, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    let next_indent = indent + 1;
    let next_indent_str = "  ".repeat(next_indent);

    match value {
        Value::Object(map) => {
            // Flatten single-entry enum-like objects (e.g. { "Net": 30 }) to (net: 30)
            if map.len() == 1 {
                if let Some((variant, inner)) = map.iter().next() {
                    if inner.is_number() || inner.is_string() || inner.is_object() {
                        return format!(
                            "(\n{}{}: {},\n{})",
                            next_indent_str,
                            variant.to_lowercase(),
                            to_typst_value(inner, next_indent),
                            indent_str
                        );
                    }
                }
            }

            let fields = map
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}{}: {}",
                        next_indent_str,
                        k,
                        to_typst_value(v, next_indent)
                    )
                })
                .collect::<Vec<_>>()
                .join(",\n");

            format!("(\n{},\n{})", fields, indent_str)
        }

        Value::Array(arr) => {
            let items = arr
                .iter()
                .map(|v| format!("{}{}", next_indent_str, to_typst_value(v, next_indent)))
                .collect::<Vec<_>>()
                .join(",\n");

            format!("(\n{},\n{})", items, indent_str)
        }

        Value::String(s) => format!("\"{}\"", s),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Null => "none".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    macro_rules! test_data_to_typst {
        ($sample:expr, $input:expr) => {{
            let input = $sample
                .to_partial($input)
                .unwrap()
                .to_typst(
                    ExchangeRates::builder()
                        .target_currency(Currency::EUR)
                        .rates(ExchangeRatesMap::from_iter([
                            (Currency::GBP, UnitPrice::from(1.174)),
                            (Currency::SEK, UnitPrice::from(11.05)),
                        ]))
                        .build(),
                )
                .unwrap();
            let typst = input.to_typst_fn();
            insta::assert_snapshot!(typst);
        }};
    }

    macro_rules! test_l18n_to_typst {
        ($input:expr) => {{
            let typst = $input.content().to_typst_fn();
            insta::assert_snapshot!(typst);
        }};
    }

    #[test]
    fn sample_expenses_to_typst() {
        test_data_to_typst!(
            Data::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Expenses)
                .month(YearAndMonth::sample())
                .language(Language::EN)
                .build()
        );
    }

    #[test]
    fn sample_services_to_typst() {
        test_data_to_typst!(
            Data::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Service { days_off: None })
                .month(YearAndMonth::sample())
                .language(Language::EN)
                .build()
        );
    }

    #[test]
    fn l18n_english_to_typst_macro() {
        test_l18n_to_typst!(&L18n::new(Language::EN).unwrap());
    }
}
