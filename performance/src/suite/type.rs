use super::*;

/// Named benchmark entry pairing a canonical metric ID with its runner function.
pub(crate) type BenchEntry = (&'static str, fn() -> BenchMetric);
