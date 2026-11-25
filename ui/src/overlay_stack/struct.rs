use super::*;

/// Single active overlay entry registered in the unified modal/drawer/sheet/popover dismissal stack.
#[derive(Clone, CustomDebug, Data)]
pub struct OverlayStackEntry {
    /// Monotonic unique registration identifier.
    #[get(type(copy))]
    pub(crate) id: u64,
    /// Human-readable overlay kind or label for diagnostics and tests.
    pub(crate) label: String,
    /// Stacking z-index priority (highest z-index dismisses first on system back).
    #[get(type(copy))]
    pub(crate) z_index: i32,
    /// Closure invoked to close this overlay cleanly.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_dismiss: OverlayDismissCallback,
}

/// Unified LIFO overlay stack manager coordinating modal, drawer, sheet, alert dialog, and popover dismissal on system Back / Escape.
#[derive(Clone, Copy, Debug, Default)]
pub struct EuvOverlayStack;
