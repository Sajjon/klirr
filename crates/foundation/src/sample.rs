/// A helper trait for constructing deterministic sample values in tests.
pub trait HasSample: Sized {
    /// Returns a canonical sample instance.
    fn sample() -> Self;
    /// Returns a second sample instance distinct from `sample`.
    fn sample_other() -> Self;
}
