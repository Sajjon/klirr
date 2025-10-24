use indoc::indoc;
use serde::Serialize;
use serde_json::{self, Value};

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
