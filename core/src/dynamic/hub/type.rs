use super::*;

/// Zero-context native Rust UI builder closure used for hot-swapping components or pages.
pub type DynamicBuilderFn = Rc<dyn Fn() -> NodeId>;

/// Route-aware fallback builder closure invoked by `DynamicPageOutlet` when no dynamic page overrides a route.
pub type DynamicRouteFallbackFn = Rc<dyn Fn(&str) -> NodeId>;

/// Callback closure invoked when a user interacts with a dynamically updated control.
pub type DynamicActionCallback = Rc<RefCell<dyn FnMut(&DynamicActionEvent)>>;

/// Internal staged commit tuple returned after applying a page-level dynamic update.
pub(crate) type PageApplyCommit = (
    Signal<u64>,
    Signal<u64>,
    Signal<DynamicTelemetry>,
    DynamicTelemetry,
    bool,
);

/// Internal staged commit tuple returned after rolling back a page-level dynamic update.
pub(crate) type PageRollbackCommit = (
    u64,
    Signal<u64>,
    Signal<u64>,
    Signal<DynamicTelemetry>,
    DynamicTelemetry,
    bool,
);

/// Internal staged commit tuple returned after validating and applying an atomic bundle.
pub(crate) type BundleApplyCommit = (
    Vec<Signal<u64>>,
    bool,
    Signal<u64>,
    Signal<DynamicTelemetry>,
    DynamicTelemetry,
);
