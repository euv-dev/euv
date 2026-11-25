use super::*;

thread_local! {
    /// Persistent live WebAssembly showcase `(App, ShowcaseState)` receiving interactive DOM canvas events.
    pub(crate) static WEB_SHOWCASE_RUNTIME: RefCell<Option<(App, ShowcaseState)>> = const { RefCell::new(None) };
    /// Coalesced dirty flag set by high-frequency DOM input listeners and flushed once per `requestAnimationFrame`.
    pub(crate) static WEB_NEEDS_FRAME: Cell<bool> = const { Cell::new(true) };
}
