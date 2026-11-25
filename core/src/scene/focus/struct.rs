use super::*;

/// Tracks active focus, hover, and pointer-press targets across the retained scene graph.
#[derive(Clone, Copy, Data, Debug, Default, Eq, PartialEq)]
pub struct FocusManager {
    /// Currently focused node receiving keyboard and IME input.
    #[get(skip)]
    pub(crate) focused: Option<NodeId>,
    /// Node currently hovered by the primary pointer.
    #[get(skip)]
    pub(crate) hovered: Option<NodeId>,
    /// Node currently pressed by the primary pointer.
    #[get(skip)]
    pub(crate) pressed: Option<NodeId>,
}
