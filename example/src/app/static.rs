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
    /// Retained browser `requestAnimationFrame` callback cancelled on background and restarted on foreground.
    pub(crate) static WEB_RAF_CALLBACK: RefCell<Option<WebRafCallback>> = const { RefCell::new(None) };
    /// Active browser `requestAnimationFrame` handle (`None` while background-frozen so 0 frames fire).
    pub(crate) static WEB_RAF_ID: Cell<Option<i32>> = const { Cell::new(None) };
    /// Last browser rAF timestamp in milliseconds (`0.0` on boot or foreground resume to prevent dt spikes).
    pub(crate) static WEB_RAF_LAST_TS: Cell<f64> = const { Cell::new(0.0) };
    /// Accumulated seconds for host display refresh-rate detection in the browser rAF loop.
    pub(crate) static WEB_RAF_ACCUM_SECS: Cell<f32> = const { Cell::new(0.0) };
    /// Accumulated frame counter for host display refresh-rate detection in the browser rAF loop.
    pub(crate) static WEB_RAF_ACCUM_FRAMES: Cell<u32> = const { Cell::new(0) };
}
