use super::*;

/// Browser canvas window record storing live DOM `HtmlCanvasElement` and 2D blit context.
#[derive(Clone, CustomDebug, Data)]
pub struct WebWindowRecord {
    /// Declarative window configuration.
    pub(crate) config: WindowConfig,
    /// Live window state and DPR metrics.
    #[get(type(copy))]
    pub(crate) state: WindowState,
    /// Assigned canvas registration identifier.
    #[get(type(copy))]
    pub(crate) canvas_id: u32,
    /// Backing DOM canvas element.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) canvas: Option<HtmlCanvasElement>,
    /// Cached 2D software blit context.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) ctx2d: Option<CanvasRenderingContext2d>,
}

/// Direct Browser WebAssembly (`web-sys` Canvas2D / WebGL2 / WebGPU / DOM Events) platform backend.
#[derive(Clone, Data, Debug)]
pub struct WebPlatform {
    /// Monotonic window handle allocator.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) next_id: u64,
    /// Active web canvas windows.
    pub(crate) windows: HashMap<WindowId, WebWindowRecord>,
    /// Queued browser pointer, keyboard, wheel, and resize events.
    pub(crate) pending_events: Vec<WindowEvent>,
    /// Clipboard cache.
    #[get(skip)]
    pub(crate) clipboard_cache: Option<String>,
}
