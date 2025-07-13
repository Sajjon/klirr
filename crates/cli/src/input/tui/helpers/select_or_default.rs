use crate::prelude::{Result, Select};
pub fn select_or_default<'a, 'b: 'a, S, T, F>(
    selector: Option<S>,
    target: S,
    default: &'b T,
    builder: F,
) -> Result<T>
where
    S: Select,
    F: FnOnce(&'a T) -> Result<T>,
    T: Clone,
{
    if selector
        .as_ref()
        .map(|s| s.includes(target))
        .unwrap_or(selector.is_none())
    {
        builder(default)
    } else {
        Ok(default.clone())
    }
}
