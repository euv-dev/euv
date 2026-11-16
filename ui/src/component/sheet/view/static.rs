use super::*;

thread_local! {
    /// Weak per-document leases never keep a detached document or sheet alive.
    #[cfg(target_arch = "wasm32")]
    pub(crate) static SHEET_SCROLL_LOCKS: SheetScrollRegistry = const { RefCell::new(Vec::new()) };
}
