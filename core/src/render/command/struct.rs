use super::*;

/// Reusable backend-agnostic command buffer emitted once per frame for Software, Hardware, or Hybrid execution.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct DisplayList {
    /// Ordered drawing commands.
    pub(crate) commands: Vec<DrawCommand>,
    /// Logical viewport size associated with this display list.
    #[get(type(copy))]
    pub(crate) viewport: Size,
    /// Scale factor mapping logical coordinates to physical pixels.
    #[get(type(copy))]
    pub(crate) scale_factor: f32,
    /// Command index separating base-scene commands `[0..portal_split_index]` from top-layer portal/fixed overlays.
    #[get(type(copy))]
    pub(crate) portal_split_index: usize,
    /// Persistent scratch queue collecting deferred top-layer portal/fixed nodes across frames (cleared after `PaintPass::build`).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) portals_scratch: Vec<(i32, usize, NodeId)>,
    /// Persistent stack-allocated scratch buffer for nested z-index child sorting across frames (cleared after `PaintPass::build`).
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) z_sort_scratch: Vec<(i32, usize, NodeId)>,
    /// Persistent scratch buffer of ancestor handles leading to live `Portal` / `Position::Fixed` nodes across frames.
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) portal_ancestors_scratch: Vec<NodeId>,
}

/// Cross-frame retained `DrawCommand` fragment captured for one clean clipped container/list-row subtree.
#[derive(Clone, Debug)]
pub(crate) struct RetainedPaintFragment {
    /// Stable node identity (index and generation) owning this fragment.
    pub(crate) node_id: NodeId,
    /// Bit representation of DPI scale factor at capture time.
    pub(crate) scale_bits: u32,
    /// Bit representation of accumulated ancestor opacity at capture time.
    pub(crate) opacity_bits: u32,
    /// Physical displacement `(offset_x_phys, offset_y_phys)` at capture time.
    pub(crate) offset_phys: Point,
    /// Deterministic fingerprint of container and child layout geometry.
    pub(crate) geom_hash: u64,
    /// Deterministic fingerprint of container and child paint/text styles and content.
    pub(crate) style_text_hash: u64,
    /// Exact ordered `DrawCommand` slice emitted for this subtree.
    pub(crate) commands: Vec<DrawCommand>,
}

/// Whole-frame scene snapshot signature used for O(1) full-`DisplayList` retention when `SceneTree` is unchanged.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct PaintSceneSignature {
    /// Address of the rendered `SceneTree` instance to prevent cross-tree aliasing.
    pub(crate) tree_ptr: usize,
    /// Monotonic `SceneTree::revision` at last paint.
    pub(crate) tree_revision: u64,
    /// Viewport width bits.
    pub(crate) viewport_w_bits: u32,
    /// Viewport height bits.
    pub(crate) viewport_h_bits: u32,
    /// Active DPI scale bits.
    pub(crate) scale_bits: u32,
    /// Packed RGBA clear color.
    pub(crate) clear_rgba: u32,
    /// Packed focused node handle (`u64::MAX` when `None`).
    pub(crate) focused_packed: u64,
    /// Packed hovered node handle (`u64::MAX` when `None`).
    pub(crate) hovered_packed: u64,
    /// Packed pressed node handle (`u64::MAX` when `None`).
    pub(crate) pressed_packed: u64,
    /// Active caret blink visibility flag.
    pub(crate) caret_visible: bool,
    /// Deterministic digest of active text selection range.
    pub(crate) text_sel_sig: u64,
}

/// Bounded cross-frame cache retaining whole display lists and clean clipped subtree `DrawCommand` fragments.
#[derive(Clone, Debug, Default, Getter)]
pub struct PaintFragmentCache {
    /// True once `last_scene_sig` has been populated for the current `DisplayList`.
    #[get(skip)]
    pub(crate) has_last_scene: bool,
    /// Last rendered whole-scene signature for O(1) zero-rebuild display list reuse.
    #[get(skip)]
    pub(crate) last_scene_sig: PaintSceneSignature,
    /// True when the last rendered frame visited at least one dynamic `NodeKind::Canvas`.
    #[get(skip)]
    pub(crate) last_has_canvas: bool,
    /// Bounded power-of-two direct-mapped table of retained subtree `DrawCommand` fragments.
    #[get(skip)]
    pub(crate) slots: Vec<Option<RetainedPaintFragment>>,
    /// Cumulative count of whole-frame `DisplayList` retention hits.
    #[get(type(copy))]
    pub(crate) whole_list_hits: u64,
    /// Cumulative count of retained subtree fragment reuse hits.
    #[get(type(copy))]
    pub(crate) fragment_hits: u64,
    /// Cumulative count of retained subtree fragment capture misses.
    #[get(type(copy))]
    pub(crate) fragment_misses: u64,
    /// Cumulative count of direct-mapped fragment slot displacements.
    #[get(type(copy))]
    pub(crate) fragment_evictions: u64,
}

/// Stateless scene-to-display-list compiler supporting z-order sorting, scroll clipping, frustum culling, and retained fragment caching.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PaintPass;

/// Borrowed reusable scratch buffers threaded through recursive `PaintPass::emit_node` calls.
#[derive(Debug)]
pub(crate) struct PaintScratch<'a> {
    /// Optional deferred top-layer portal queue (`None` while painting top-layer portals).
    pub(crate) portals: Option<&'a mut Vec<(i32, usize, NodeId)>>,
    /// Stack-sliced scratch vector reused for nested z-index child sorting.
    pub(crate) z_sort: &'a mut Vec<(i32, usize, NodeId)>,
    /// Ancestor handles on paths to live `Portal` / `Position::Fixed` descendants for lossless clipped-subtree culling.
    pub(crate) portal_ancestors: &'a [NodeId],
    /// Optional cross-frame retained fragment cache (`Some` during retained builds).
    pub(crate) fragments: Option<&'a mut PaintFragmentCache>,
    /// Flag set whenever any visited node is `NodeKind::Canvas`.
    pub(crate) saw_canvas: &'a mut bool,
}
