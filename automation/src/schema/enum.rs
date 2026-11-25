use super::*;

/// Categories of layout or visual defects automatically detected for AI diagnostics.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AiAnomalyKind {
    /// Visible node extends beyond the right edge of the active viewport.
    HorizontalOverflow,
    /// Visible node has zero width or zero height despite being displayed.
    ZeroSizeVisible,
    /// Two sibling text nodes overlap in screen space.
    TextOverlap,
}

/// Interactive UI action executable remotely through the AI Schema v1 protocol.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub enum AiAction {
    /// Clicks the center of the first visible node matching `selector`.
    Click {
        /// Target node locator.
        selector: UiSelector,
    },
    /// Moves the pointer over the center of the first visible node matching `selector`.
    Hover {
        /// Target node locator.
        selector: UiSelector,
    },
    /// Clicks and types UTF-8 `text` into the matching input node.
    TypeText {
        /// Target input locator.
        selector: UiSelector,
        /// Text string to input.
        text: String,
    },
    /// Scrolls the container matching `selector` (or active scroll view) by `(dx, dy)`.
    Scroll {
        /// Target scroll container or child locator.
        selector: UiSelector,
        /// Horizontal scroll delta.
        dx: f32,
        /// Vertical scroll delta.
        dy: f32,
    },
    /// Dispatches a keyboard press by key name (`"Enter"`, `"Backspace"`, `"Escape"`, `"Space"`, `"ArrowRight"`, or single char).
    PressKey {
        /// Key name string.
        key: String,
    },
    /// Resizes the logical viewport and DPI scale immediately.
    Resize {
        /// New logical width.
        width: u32,
        /// New logical height.
        height: u32,
        /// New display scale factor.
        scale: f32,
    },
}

/// Structured AI Schema v1 request envelope accepted by `AiSchemaProtocol::handle`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "op")]
pub enum AiSchemaRequest {
    /// Returns the formal JSON Schema specification for `euv.ai.schema.v1`.
    DescribeSchema,
    /// Captures a complete runtime, layout, performance, service, OTA, and anomaly snapshot.
    Snapshot {
        /// When `true`, includes the full recursive `layout_tree`.
        include_layout_tree: bool,
    },
    /// Queries all visible nodes matching `selector` and returns their screen bounds and metadata.
    QueryNodes {
        /// Declarative UI locator.
        selector: UiSelector,
    },
    /// Performs a live top-layer-aware hit test at logical coordinates `(x, y)`.
    HitTest {
        /// Logical X coordinate.
        x: f32,
        /// Logical Y coordinate.
        y: f32,
    },
    /// Executes an interactive UI action and returns updated frame telemetry.
    ExecuteAction {
        /// Action command to execute.
        action: AiAction,
    },
    /// Executes a declarative multi-step `UiTestScript` and returns a structured `UiTestReport`.
    RunTestScript {
        /// Automation script payload.
        script: UiTestScript,
    },
}
