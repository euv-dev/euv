use super::*;

/// Props for the [`euv_scroll_area`] component.
///
/// The area is a frame around a scrolling viewport: the frame keeps the
/// content inside its own bounds, and the viewport is what scrolls.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvScrollAreaProps {
    /// The accessible name of the scrolling region.
    pub label: &'static str,
}

/// DOM references scoped to the immediate owning root.
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Getter)]
pub(crate) struct ScrollAreaParts {
    /// Mounted scrolling viewport.
    #[get(pub(crate))]
    pub(crate) viewport: HtmlElement,
    /// Mounted vertical track.
    #[get(pub(crate))]
    pub(crate) bar: HtmlElement,
    /// Mounted vertical thumb.
    #[get(pub(crate))]
    pub(crate) thumb: HtmlElement,
}

/// Weak-callback controller lifetime owned by the component hook.
#[cfg(target_arch = "wasm32")]
#[derive(Getter)]
pub(crate) struct ScrollAreaController {
    /// Lifecycle-managed root reference.
    #[get(pub(crate))]
    pub(crate) root: NodeRef<Element>,
    /// Resolved public parts after mounting.
    #[get(pub(crate))]
    pub(crate) parts: RefCell<Option<ScrollAreaParts>>,
    /// Pending animation-frame identity.
    #[get(pub(crate))]
    pub(crate) frame: Cell<Option<i32>>,
    /// Retained weak animation callback.
    #[get(pub(crate))]
    pub(crate) callback: RefCell<Option<Closure<dyn FnMut(f64)>>>,
    /// Resize observer and callback ownership.
    #[get(pub(crate))]
    pub(crate) resize: RefCell<Option<(ResizeObserver, ScrollAreaObserverCallback)>>,
    /// Mutation observer and callback ownership.
    #[get(pub(crate))]
    pub(crate) mutation: RefCell<Option<(JsValue, ScrollAreaObserverCallback)>>,
    /// Exact listener identities for teardown.
    #[get(pub(crate))]
    pub(crate) listeners: RefCell<Vec<ScrollAreaListener>>,
    /// Captured pointer and thumb grab offset in CSS pixels.
    #[get(pub(crate))]
    pub(crate) drag: Cell<Option<(i32, f64)>>,
    /// Whether teardown has completed.
    #[get(pub(crate))]
    pub(crate) disposed: Cell<bool>,
}
