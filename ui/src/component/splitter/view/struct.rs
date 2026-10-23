use super::*;

/// Props for the [`euv_splitter`] component.
///
/// Defines the strongly-typed interface for a resizable split view: how
/// much of the space the first pane takes, and the bounds the drag may
/// move it within.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSplitterProps {
    /// The share of the width the first pane takes, in percent.
    ///
    /// The caller owns the value, so the size survives re-renders and
    /// can be seeded from a stored preference.
    pub size: Signal<f64>,
    /// The smallest share the first pane may take, in percent.
    pub min: f64,
    /// The largest share the first pane may take, in percent.
    pub max: f64,
}

/// The live state of one splitter drag.
///
/// The `mousemove` and `mouseup` handlers live here so they can be
/// taken off the document when the drag ends; dropping the session
/// drops both closures with it.
///
/// The handlers are browser-only: a host build has no pointer to
/// follow, so the session is an empty shell there.
pub(crate) struct DragSession {
    /// The handler that follows the pointer.
    #[cfg(target_arch = "wasm32")]
    pub(crate) on_move: Closure<dyn FnMut(Event)>,
    /// The handler that ends the drag.
    #[cfg(target_arch = "wasm32")]
    pub(crate) on_up: Closure<dyn FnMut(Event)>,
}
