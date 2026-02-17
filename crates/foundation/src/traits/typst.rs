use indoc::indoc;
use serde::Serialize;
use serde_json::{self, Value};

/// Empty Marker trait
pub trait ToTypst {}

pub trait ToTypstFn {
    /// Converts the implementing type into a Typst function returning a dictionary.
    fn to_typst_fn(&self) -> String;

    /// Returns the family names of the fonts used in the given layout.
    fn used_fonts(&self) -> std::collections::HashSet<String> {
        let typst = self.to_typst_fn();
        let mut fonts = std::collections::HashSet::new();
        for line in typst.lines() {
            // we will now for each line check for patterns:
            // '    #set text(font: "CMU Serif", size: 12pt)'
            // and extract `CMU Serif` as a String
            if let Some(font) = line.split("font: ").nth(1) {
                let font_name = font
                    .split(',')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .trim_matches('"')
                    .to_string();
                if !font_name.is_empty() {
                    fonts.insert(font_name);
                }
            }
        }
        fonts
    }
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
    use serde::Serialize;

    #[derive(Serialize)]
    struct Dummy {
        title: String,
        count: usize,
        tags: Vec<String>,
    }

    impl ToTypst for Dummy {}

    #[test]
    fn converts_struct_to_typst_function() {
        let data = Dummy {
            title: "Example".to_string(),
            count: 2,
            tags: vec!["a".into(), "b".into()],
        };
        let typst = data.to_typst_fn();
        assert!(typst.contains("#let provide"));
        assert!(typst.contains("title: \"Example\""));
        assert!(typst.contains("count: 2"));
        assert!(typst.contains("tags: ("));
    }
}
