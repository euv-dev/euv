use super::*;

/// Dispatched UI event envelope carrying target node and event payload.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct UiEvent {
    /// Scene node receiving this event.
    #[get(type(copy))]
    pub(crate) target: NodeId,
    /// Specific event variant and payload.
    pub(crate) kind: UiEventKind,
}

/// Attached event listener slots on a `SceneNode`.
#[derive(Clone, CustomDebug, Data, Default)]
pub struct EventHandlers {
    /// Click / tap callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_click: Option<UiEventHandler>,
    /// Pointer down callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_pointer_down: Option<UiEventHandler>,
    /// Pointer move callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_pointer_move: Option<UiEventHandler>,
    /// Pointer up callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_pointer_up: Option<UiEventHandler>,
    /// Pointer enter callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_pointer_enter: Option<UiEventHandler>,
    /// Pointer leave callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_pointer_leave: Option<UiEventHandler>,
    /// Scroll callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_scroll: Option<UiEventHandler>,
    /// Key down callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_key_down: Option<UiEventHandler>,
    /// Text change callback for input widgets.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_change: Option<UiEventHandler>,
    /// Focus gained callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_focus: Option<UiEventHandler>,
    /// Focus lost callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_blur: Option<UiEventHandler>,
}
