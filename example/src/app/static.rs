use super::*;

thread_local! {
    /// Persistent live WebAssembly showcase `(App, ShowcaseState)` receiving interactive DOM canvas events.
    pub(crate) static WEB_SHOWCASE_RUNTIME: RefCell<Option<(App, ShowcaseState)>> = const { RefCell::new(None) };
}
