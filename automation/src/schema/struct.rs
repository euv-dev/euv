use super::*;

/// Serializable rectangle geometry `(x, y, width, height)` in logical pixels.
#[derive(Clone, Copy, Data, Debug, Default, Deserialize, New, PartialEq, Serialize)]
pub struct AiRect {
    /// Left coordinate.
    #[get(type(copy))]
    pub(crate) x: f32,
    /// Top coordinate.
    #[get(type(copy))]
    pub(crate) y: f32,
    /// Width.
    #[get(type(copy))]
    pub(crate) width: f32,
    /// Height.
    #[get(type(copy))]
    pub(crate) height: f32,
}

/// Serializable window and viewport state exported to AI agents.
#[derive(Clone, Data, Debug, Deserialize, PartialEq, Serialize)]
pub struct AiWindowInfo {
    /// Window identifier.
    #[get(type(copy))]
    pub(crate) window_id: u64,
    /// Window title.
    pub(crate) title: String,
    /// Logical viewport width in points/CSS pixels.
    #[get(type(copy))]
    pub(crate) logical_width: f32,
    /// Logical viewport height in points/CSS pixels.
    #[get(type(copy))]
    pub(crate) logical_height: f32,
    /// Physical framebuffer width in pixels.
    #[get(type(copy))]
    pub(crate) physical_width: u32,
    /// Physical framebuffer height in pixels.
    #[get(type(copy))]
    pub(crate) physical_height: u32,
    /// Active DPI display scale factor.
    #[get(type(copy))]
    pub(crate) scale_factor: f32,
    /// Active responsive breakpoint label (`"Compact"`, `"Medium"`, `"Expanded"`).
    pub(crate) breakpoint: String,
    /// Active OS/app theme mode label (`"Light"`, `"Dark"`).
    pub(crate) theme_mode: String,
    /// Window focus state.
    #[get(type(copy))]
    pub(crate) focused: bool,
}

/// Serializable rendering and scene performance metrics exported to AI agents.
#[derive(Clone, Data, Debug, Deserialize, PartialEq, Serialize)]
pub struct AiPerfInfo {
    /// Monotonic frame counter.
    #[get(type(copy))]
    pub(crate) frame_index: u64,
    /// Active resolved render mode.
    pub(crate) active_mode: String,
    /// Active resolved graphics backend.
    pub(crate) active_backend: String,
    /// Scene nodes laid out in the previous frame.
    #[get(type(copy))]
    pub(crate) layout_nodes: usize,
    /// Clean layout subtrees skipped via cache.
    #[get(type(copy))]
    pub(crate) layout_skipped: usize,
    /// Display list draw command count.
    #[get(type(copy))]
    pub(crate) draw_commands: usize,
    /// GPU batches or dirty software tiles updated.
    #[get(type(copy))]
    pub(crate) draw_batches: usize,
    /// Framebuffer pixels shaded in the previous frame.
    #[get(type(copy))]
    pub(crate) pixels_shaded: usize,
    /// Deterministic frame digest.
    #[get(type(copy))]
    pub(crate) frame_hash: u64,
    /// Total live retained scene nodes.
    #[get(type(copy))]
    pub(crate) live_node_count: usize,
    /// Current retained scene mutation revision.
    #[get(type(copy))]
    pub(crate) scene_revision: u64,
}

/// Serializable pointer, focus, hover, and cursor interaction snapshot.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct AiInteractionInfo {
    /// Currently hovered node arena index, if any.
    #[get(skip)]
    pub(crate) hovered_node: Option<u32>,
    /// Currently focused node arena index, if any.
    #[get(skip)]
    pub(crate) focused_node: Option<u32>,
    /// Currently pressed node arena index, if any.
    #[get(skip)]
    pub(crate) pressed_node: Option<u32>,
    /// Active OS/Web cursor icon label.
    pub(crate) cursor: String,
    /// Latest pointer logical X coordinate.
    #[get(type(copy))]
    pub(crate) pointer_x: f32,
    /// Latest pointer logical Y coordinate.
    #[get(type(copy))]
    pub(crate) pointer_y: f32,
}

/// Serializable platform capabilities and registered dynamic OTA metadata.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct AiRuntimeMeta {
    /// Active platform target name.
    pub(crate) platform: String,
    /// Whether hardware GPU acceleration is supported.
    #[get(type(copy))]
    pub(crate) hardware_gpu: bool,
    /// Whether multi-touch input is supported.
    #[get(type(copy))]
    pub(crate) touch_input: bool,
    /// Registered custom URI protocol schemes.
    pub(crate) registered_protocols: Vec<String>,
    /// Total accessibility nodes compiled in the latest A11y tree.
    #[get(type(copy))]
    pub(crate) accessibility_node_count: usize,
}

/// Compact visual style summary attached to each `AiLayoutNode`.
#[derive(Clone, Data, Debug, Deserialize, PartialEq, Serialize)]
pub struct AiStyleSummary {
    /// CSS display mode (`"Flex"`, `"Grid"`, `"Block"`, `"None"`).
    pub(crate) display: String,
    /// Positioning scheme (`"Relative"`, `"Absolute"`, `"Fixed"`).
    pub(crate) position: String,
    /// Overflow policy (`"Visible"`, `"Hidden"`, `"Scroll"`).
    pub(crate) overflow: String,
    /// Stacking z-index.
    #[get(type(copy))]
    pub(crate) z_index: i32,
    /// Opacity in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) opacity: f32,
    /// Optional hex RGBA background color string (`"#RRGGBBAA"`).
    #[get(skip)]
    pub(crate) background_hex: Option<String>,
    /// Foreground text hex color string (`"#RRGGBBAA"`).
    pub(crate) text_color_hex: String,
    /// Font size in logical pixels.
    #[get(type(copy))]
    pub(crate) font_size: f32,
    /// Cursor icon name.
    pub(crate) cursor: String,
}

/// Hierarchical scene and layout node exported by AI Schema v1.
#[derive(Clone, Data, Debug, Deserialize, PartialEq, Serialize)]
pub struct AiLayoutNode {
    /// Arena index of the retained scene node.
    #[get(type(copy))]
    pub(crate) node_index: u32,
    /// Generation counter of the retained scene node.
    #[get(type(copy))]
    pub(crate) generation: u32,
    /// Retained node kind (`"View"`, `"Text"`, `"Input"`, `"ScrollView"`, `"Canvas"`, `"Portal"`).
    pub(crate) kind: String,
    /// Inferred semantic accessibility role.
    pub(crate) role: String,
    /// Optional static text content.
    #[get(skip)]
    pub(crate) text: Option<String>,
    /// Optional input placeholder text.
    #[get(skip)]
    pub(crate) placeholder: Option<String>,
    /// Optional current input value.
    #[get(skip)]
    pub(crate) value: Option<String>,
    /// Screen-space logical bounding rectangle (after ancestor scroll offsets).
    #[get(type(copy))]
    pub(crate) screen_bounds: AiRect,
    /// Raw laid-out bounding rectangle in parent flow space.
    #[get(type(copy))]
    pub(crate) layout_bounds: AiRect,
    /// Intrinsic aggregate content size `(width, height)`.
    #[get(type(copy))]
    pub(crate) content_size: AiRect,
    /// Active scroll offset `(x, y)` for scroll containers.
    #[get(type(copy))]
    pub(crate) scroll_x: f32,
    /// Active scroll offset Y.
    #[get(type(copy))]
    pub(crate) scroll_y: f32,
    /// Maximum vertical scroll extent.
    #[get(type(copy))]
    pub(crate) max_scroll_y: f32,
    /// Whether the node is visible (`Display != None`, `opacity > 0`, positive dimensions).
    #[get(type(copy))]
    pub(crate) visible: bool,
    /// Whether the node accepts pointer clicks, input, or pointer cursor interaction.
    #[get(type(copy))]
    pub(crate) interactive: bool,
    /// Whether the pointer is currently hovering this node.
    #[get(type(copy))]
    pub(crate) hovered: bool,
    /// Whether this node holds keyboard focus.
    #[get(type(copy))]
    pub(crate) focused: bool,
    /// Whether this node is actively pressed.
    #[get(type(copy))]
    pub(crate) pressed: bool,
    /// Compact visual style summary.
    pub(crate) style: AiStyleSummary,
    /// Ordered child layout nodes.
    pub(crate) children: Vec<AiLayoutNode>,
}

/// Automatically detected layout or visual anomaly reported in `AiRuntimeSnapshot`.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct AiLayoutAnomaly {
    /// Classification of the detected anomaly.
    #[get(type(copy))]
    pub(crate) kind: AiAnomalyKind,
    /// Affected node arena index.
    #[get(type(copy))]
    pub(crate) node_index: u32,
    /// Human- and AI-readable diagnostic message.
    pub(crate) message: String,
    /// Screen-space bounds of the offending node.
    #[get(type(copy))]
    pub(crate) bounds: AiRect,
}

/// Complete AI Schema v1 (`euv.ai.schema.v1`) runtime snapshot.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct AiRuntimeSnapshot {
    /// Protocol version tag (`"euv.ai.schema.v1"`).
    pub(crate) protocol_version: String,
    /// Window and viewport metrics.
    pub(crate) window: AiWindowInfo,
    /// Rendering and layout telemetry.
    pub(crate) performance: AiPerfInfo,
    /// Pointer, focus, hover, and cursor state.
    pub(crate) interaction: AiInteractionInfo,
    /// Platform capabilities and accessibility counts.
    pub(crate) runtime: AiRuntimeMeta,
    /// Optional hierarchical scene & layout tree.
    #[get(skip)]
    pub(crate) layout_tree: Option<AiLayoutNode>,
    /// Automatically detected layout/visual anomalies.
    pub(crate) anomalies: Vec<AiLayoutAnomaly>,
}

/// Result of an AI Schema `HitTest` query at logical coordinates `(x, y)`.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct AiHitTestResult {
    /// Hit node arena index if any node matched `(x, y)`.
    #[get(skip)]
    pub(crate) hit_node_index: Option<u32>,
    /// Hit node semantic role.
    #[get(skip)]
    pub(crate) hit_role: Option<String>,
    /// Hit node text or value if present.
    #[get(skip)]
    pub(crate) hit_text: Option<String>,
    /// Ancestor chain of node indices from hit leaf up to root.
    pub(crate) ancestor_indices: Vec<u32>,
}

/// Unified AI Schema v1 JSON response envelope.
#[derive(Clone, Data, Debug, Deserialize, New, PartialEq, Serialize)]
pub struct AiSchemaResponse {
    /// Protocol version (`"euv.ai.schema.v1"`).
    pub(crate) protocol_version: String,
    /// Whether the request succeeded.
    #[get(type(copy))]
    pub(crate) ok: bool,
    /// Optional error message when `ok == false`.
    #[get(skip)]
    pub(crate) error: Option<String>,
    /// Structured JSON payload matching the executed operation.
    pub(crate) data: Value,
}

/// Stateless AI Schema v1 protocol dispatcher and runtime inspector facade.
#[derive(Clone, Copy, Debug, Default)]
pub struct AiSchemaProtocol;
