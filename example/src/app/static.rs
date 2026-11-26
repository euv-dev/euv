use super::*;

thread_local! {
    /// Persistent live WebAssembly showcase `(App, ShowcaseState)` receiving interactive DOM canvas events.
    pub(crate) static WEB_SHOWCASE_RUNTIME: RefCell<Option<(App, ShowcaseState)>> = const { RefCell::new(None) };
    /// Coalesced dirty flag set by high-frequency DOM input listeners and flushed once per `requestAnimationFrame`.
    pub(crate) static WEB_NEEDS_FRAME: Cell<bool> = const { Cell::new(true) };
    /// Hidden DOM `<input>` element bridging browser soft keyboards and CJK/IME composition.
    pub(crate) static WEB_IME_ELEMENT: RefCell<Option<HtmlElement>> = const { RefCell::new(None) };
    /// Tracks whether a browser IME composition session is currently in progress.
    pub(crate) static WEB_IME_COMPOSING: Cell<bool> = const { Cell::new(false) };
    /// Tracks whether the hidden browser IME proxy input is currently focused.
    pub(crate) static WEB_IME_ACTIVE: Cell<bool> = const { Cell::new(false) };
}
