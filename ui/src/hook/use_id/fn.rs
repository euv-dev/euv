use super::*;

/// Obtains the [`UseId`] registered against the current hook context slot.
///
/// Behaves like `HookContext::use_hook` - the same `UseId` is returned on
/// every render at the same hook index, so the generated identifier is
/// stable across re-renders and stable across the element's lifetime.
///
/// The identifier is produced by a process-wide counter, so two
/// components in the same page never share one.
///
/// # Arguments
///
/// - `&'static str` - The prefix put in front of the counter, for
///   example `"field"`.
///
/// # Returns
///
/// - `UseId` - The identifier handle, with `get_value()` returning the
///   generated string.
pub fn use_id(prefix: &'static str) -> UseId {
    HookContext::use_hook(|| {
        let handle: UseId = UseId::new();
        let counter: usize = next_id();
        handle.get_value().set(format!("{prefix}-{counter}"));
        handle
    })
}

/// Hands out the next process-wide identifier number.
///
/// The counter is a plain atomic: the value only has to be unique, and
/// two threads racing for the same number would still produce two
/// different identifiers because the increment is atomic.
///
/// # Returns
///
/// - `usize` - The next identifier number.
fn next_id() -> usize {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
