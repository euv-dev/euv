use super::*;

thread_local! {
    /// Active overlay stack entries ordered by registration sequence.
    pub(crate) static OVERLAY_STACK_ENTRIES: RefCell<Vec<OverlayStackEntry>> = const { RefCell::new(Vec::new()) };
    /// Monotonic overlay entry ID generator.
    pub(crate) static OVERLAY_STACK_NEXT_ID: Cell<u64> = const { Cell::new(1) };
}
