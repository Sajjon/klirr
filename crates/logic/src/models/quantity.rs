use crate::prelude::*;

/// The quantity of items, e.g. the number of days of consulting service.
#[derive(Clone, Copy, Display, Debug, PartialEq, Serialize, Deserialize, From, Deref)]
pub struct Quantity(f64);

impl HasSample for Quantity {
    fn sample() -> Self {
        Self::from(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn quantity_sample() {
        let sample = Quantity::sample();
        assert_eq!(*sample, 1.0);
    }

    #[test]
    fn quantity_display() {
        let quantity = Quantity::from(3.5);
        assert_eq!(format!("{}", quantity), "3.5");
    }
}
