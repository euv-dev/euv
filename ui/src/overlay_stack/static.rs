use super::*;

thread_local! {
    /// Active overlay stack entries ordered by registration sequence.
    pub(crate) static OVERLAY_STACK_ENTRIES: RefCell<Vec<OverlayStackEntry>> = const { RefCell::new(Vec::new()) };
    /// Monotonic overlay entry ID generator.
    pub(crate) static OVERLAY_STACK_NEXT_ID: Cell<u64> = const { Cell::new(1) };
    /// Reactive active overlay count signal notified whenever entries are pushed, removed, or cleared.
    pub(crate) static OVERLAY_STACK_COUNT_SIG: RefCell<Option<Signal<usize>>> = const { RefCell::new(None) };
}
