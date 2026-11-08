use super::*;

/// Props for the [`euv_resizable_panel_group`] component.
///
/// The group carries the direction its panels are laid out in.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvResizablePanelGroupProps {
    /// The direction the panels are laid out in.
    pub direction: EuvResizableDirection,
}

/// Props for the [`euv_resizable_panel`] component.
///
/// The panel carries the share of the group it takes. The handle beside
/// it writes that same share, so a drag moves the panel.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvResizablePanelProps {
    /// The share of the group the panel takes, in percent.
    pub size: Signal<f64>,
}

/// Props for the [`euv_resizable_handle`] component.
///
/// The handle carries the share it moves, the bounds the share may
/// move within, and the direction of the group it belongs to.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvResizableHandleProps {
    /// The share of the group the panel takes, in percent.
    pub size: Signal<f64>,
    /// The smallest share the panel may take, in percent.
    pub min: f64,
    /// The largest share the panel may take, in percent.
    pub max: f64,
    /// The direction of the group the handle belongs to.
    pub direction: EuvResizableDirection,
}

/// The live state of one resizable drag.
///
/// The `mousemove` and `mouseup` handlers live here so they can be
/// taken off the document when the drag ends; dropping the session
/// drops both closures with it.
///
/// The handlers are browser-only: a host build has no pointer to
/// follow, so the session is an empty shell there.
pub(crate) struct ResizableDragSession {
    /// The handler that follows the pointer.
    #[cfg(target_arch = "wasm32")]
    pub(crate) on_move: Closure<dyn FnMut(Event)>,
    /// The handler that ends the drag.
    #[cfg(target_arch = "wasm32")]
    pub(crate) on_up: Closure<dyn FnMut(Event)>,
}
