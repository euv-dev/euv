use super::*;

/// Computed layout geometry cached on every scene node.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct LayoutBox {
    /// Absolute border-box bounds in root logical coordinates.
    #[get(type(copy))]
    pub(crate) bounds: Rect,
    /// Inner content-box bounds inside padding and border.
    #[get(type(copy))]
    pub(crate) content_bounds: Rect,
    /// Intrinsic scrollable content size.
    #[get(type(copy))]
    pub(crate) content_size: Size,
    /// Active scroll offset inside scroll containers.
    #[get(type(copy))]
    pub(crate) scroll_offset: Point,
}

/// Interactive visual state flags on a scene node.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, PartialEq)]
pub struct InteractionState {
    /// Whether the node can receive keyboard focus.
    #[get(type(copy))]
    pub(crate) focusable: bool,
    /// Whether the node currently holds keyboard focus.
    #[get(type(copy))]
    pub(crate) focused: bool,
    /// Whether a pointer currently hovers over this node.
    #[get(type(copy))]
    pub(crate) hovered: bool,
    /// Whether a pointer is actively pressed on this node.
    #[get(type(copy))]
    pub(crate) pressed: bool,
}

/// Retained scene node stored contiguously inside `SceneTree`.
#[derive(CustomDebug, Data)]
pub struct SceneNode {
    /// Generation tag matching live `NodeId` handles.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) generation: u32,
    /// Whether this arena slot is currently occupied.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) alive: bool,
    /// Semantic kind and node-specific payload.
    pub(crate) kind: NodeKind,
    /// Compiled layout, paint, and typography style.
    #[get(type(copy))]
    pub(crate) style: Style,
    /// Optional hover paint override for zero-layout-cost hover feedback.
    #[get(skip)]
    pub(crate) hover_style: Option<PaintStyle>,
    /// Computed layout box and scroll offsets.
    #[get(type(copy))]
    pub(crate) layout_box: LayoutBox,
    /// Cached previous layout constraints for clean-subtree O(1) skipping.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) last_constraint: Option<AvailableSpace>,
    /// Surgical dirty bitmask.
    #[get(type(copy))]
    pub(crate) dirty: DirtyFlags,
    /// Parent node handle if attached to the tree.
    #[get(skip)]
    pub(crate) parent: Option<NodeId>,
    /// Ordered child node handles.
    pub(crate) children: Vec<NodeId>,
    /// Optional reconciliation key for keyed list items.
    #[get(skip)]
    pub(crate) key: Option<String>,
    /// Interactive focus/hover/pressed state.
    #[get(type(copy))]
    pub(crate) interaction: InteractionState,
    /// Event callbacks attached to this node.
    pub(crate) handlers: EventHandlers,
    /// Optional reactive ownership scope disposed when this node is removed.
    #[get(skip)]
    pub(crate) scope: Option<Scope>,
    /// Optional custom 2D canvas draw callback.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) canvas_painter: Option<CanvasDrawCallback>,
    /// Optional explicit semantic accessibility metadata for screen readers and ARIA bridges.
    #[get(skip)]
    pub(crate) a11y: Option<A11yProps>,
    /// Optional virtualized scroll metrics `(scroll_y, total_content_height)` for overlay scrollbar thumb accuracy.
    #[get(skip)]
    pub(crate) virtual_scroll: Option<(f32, f32)>,
}
