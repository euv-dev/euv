use super::*;

/// The process-wide counter handing out unique identifier numbers.
pub(crate) static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
