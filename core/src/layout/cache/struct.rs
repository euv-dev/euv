use super::*;

/// Available layout space passed down during a layout pass and cached for clean-subtree skipping.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct AvailableSpace {
    /// Available horizontal extent in logical pixels.
    #[get(type(copy))]
    pub(crate) width: f32,
    /// Available vertical extent in logical pixels.
    #[get(type(copy))]
    pub(crate) height: f32,
    /// Root viewport size for `vw`/`vh` length resolution.
    #[get(type(copy))]
    pub(crate) viewport: Size,
    /// Origin X coordinate where this node is placed.
    #[get(type(copy))]
    pub(crate) origin_x: f32,
    /// Origin Y coordinate where this node is placed.
    #[get(type(copy))]
    pub(crate) origin_y: f32,
    /// Whether the parent stretched this node's width to fill `width` when its declared width is `Length::Auto`.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) stretch_width: bool,
    /// Whether the parent stretched this node's height to fill `height` when its declared height is `Length::Auto`.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) stretch_height: bool,
}

/// Internal container constraint bundle passed to child layout solvers.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub(crate) struct ContainerFrame {
    /// Parent layout properties.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) parent_layout: LayoutStyle,
    /// Inner available dimensions inside padding and border.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner_avail: Size,
    /// Root viewport logical size.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) viewport: Size,
    /// Content origin in root coordinates.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content_origin: Point,
    /// Whether the container's outer width is definite (explicit length, aspect ratio, or parent stretch).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) definite_width: bool,
    /// Whether the container's outer height is definite (explicit length, aspect ratio, or parent stretch).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) definite_height: bool,
}

/// Telemetry counters recorded per layout pass.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct LayoutStats {
    /// Number of nodes laid out during this pass.
    #[get(type(copy))]
    pub(crate) nodes_computed: usize,
    /// Number of clean subtrees skipped in O(1) via constraint caching.
    #[get(type(copy))]
    pub(crate) subtrees_skipped: usize,
}

/// Compact direct-mapped/set-associative cache slot storing a measured text bounding box.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct TextSizeEntry {
    /// Primary 64-bit hash of the measured UTF-8 text bytes.
    pub(crate) hash_a: u64,
    /// Secondary independent 64-bit hash of the measured UTF-8 text bytes.
    pub(crate) hash_b: u64,
    /// Exact byte length of the measured text string.
    pub(crate) text_len: u32,
    /// Normalized IEEE-754 bits of `TextStyle::font_size`.
    pub(crate) font_size_bits: u32,
    /// Normalized IEEE-754 bits of `TextStyle::line_height`.
    pub(crate) line_height_bits: u32,
    /// Normalized IEEE-754 bits of `TextStyle::letter_spacing`.
    pub(crate) letter_spacing_bits: u32,
    /// Normalized IEEE-754 bits of `max_width`, or `TEXT_SIZE_MAX_WIDTH_NONE_BITS` when `None`.
    pub(crate) max_width_bits: u32,
    /// Cached measured bounding width in logical pixels.
    pub(crate) width: f32,
    /// Cached measured bounding height in logical pixels.
    pub(crate) height: f32,
    /// Packed typography flags (`family`, `weight`, `overflow`, `italic`, and occupied bit).
    pub(crate) style_flags: u16,
}

/// Bounded 2-way set-associative cache mapping `(text, style, max_width)` to measured `Size`.
#[derive(Clone, Debug, Default)]
pub(crate) struct TextSizeCache {
    /// Lazily allocated flat table of `TEXT_SIZE_CACHE_SLOTS` entries (`2` ways per set).
    pub(crate) entries: Vec<TextSizeEntry>,
}

/// Compact single-cache-line scratch record tracking a flex child across probe and placement passes.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct FlexItemRecord {
    /// Scene node handle of the flex child.
    pub(crate) node_id: NodeId,
    /// Current target size updated by flex grow and auto-cross re-probing.
    pub(crate) size: Size,
    /// Border-box size produced by the initial probe pass.
    pub(crate) probed_size: Size,
    /// Logical origin where the child currently resides after the probe pass.
    pub(crate) probed_origin: Point,
    /// Outer margin insets copied once from `LayoutStyle`.
    pub(crate) margin: Edges,
    /// Flex grow factor copied once from `LayoutStyle`.
    pub(crate) flex_grow: f32,
    /// Effective `align-self` override copied once from `LayoutStyle`.
    pub(crate) align_self: AlignSelf,
    /// Overflow clipping behavior copied once from `LayoutStyle`.
    pub(crate) overflow: Overflow,
    /// Packed boolean flags (`FLEX_ITEM_*`) describing child sizing constraints and probe state.
    pub(crate) flags: u16,
}
