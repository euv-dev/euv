use super::*;

/// Categories of layout or visual defects automatically detected for AI diagnostics.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AiAnomalyKind {
    /// Visible node extends beyond the right edge of the active viewport.
    HorizontalOverflow,
    /// Visible node has negative horizontal bounds off the left viewport edge.
    NegativeBounds,
    /// Visible interactive/content node has zero width or zero height despite being displayed.
    ZeroSizeVisible,
    /// Two sibling text nodes overlap in screen space.
    TextOverlap,
}

/// High-level pointer or multi-touch gesture synthesized by AI agents.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AiGestureKind {
    /// Single touch tap at `(x, y)`.
    Tap,
    /// Rapid double touch tap at `(x, y)`.
    DoubleTap,
    /// Directional touch swipe starting at `(x, y)` displaced by `(dx, dy)`.
    Swipe,
    /// Two-finger pinch gesture centered at `(x, y)` scaled by `scale`.
    Pinch,
    /// Touch press-and-hold gesture at `(x, y)`.
    LongPress,
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
    /// Clicks exact logical coordinates `(x, y)`.
    ClickAt {
        /// Logical X coordinate.
        x: f32,
        /// Logical Y coordinate.
        y: f32,
    },
    /// Performs two consecutive clicks on the first visible node matching `selector`.
    DoubleClick {
        /// Target node locator.
        selector: UiSelector,
    },
    /// Moves the pointer over the center of the first visible node matching `selector`.
    Hover {
        /// Target node locator.
        selector: UiSelector,
    },
    /// Focuses the first visible node matching `selector`.
    Focus {
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
    /// Focuses and clears all existing characters from the matching input node.
    ClearInput {
        /// Target input locator.
        selector: UiSelector,
    },
    /// Drags from the center of `from` to the center of `to`.
    Drag {
        /// Drag start node locator.
        from: UiSelector,
        /// Drag end node locator.
        to: UiSelector,
    },
    /// Drags between explicit logical screen coordinates.
    DragPoints {
        /// Drag start X coordinate.
        from_x: f32,
        /// Drag start Y coordinate.
        from_y: f32,
        /// Drag end X coordinate.
        to_x: f32,
        /// Drag end Y coordinate.
        to_y: f32,
    },
    /// Synthesizes a multi-touch or touch gesture at `(x, y)`.
    Gesture {
        /// Gesture classification.
        gesture: AiGestureKind,
        /// Logical anchor X coordinate.
        x: f32,
        /// Logical anchor Y coordinate.
        y: f32,
        /// Displacement X delta (used by `Swipe`).
        #[serde(default)]
        dx: f32,
        /// Displacement Y delta (used by `Swipe`).
        #[serde(default)]
        dy: f32,
        /// Relative pinch scale factor (used by `Pinch`, default `1.25`).
        #[serde(default)]
        scale: f32,
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
    /// Advances `count` UI render frames.
    StepFrames {
        /// Number of frames to render (`1..=120`).
        count: usize,
    },
    /// Advances the game physics/animation simulation for `frames` ticks of `dt_secs`.
    StepGame {
        /// Per-tick delta time in seconds.
        dt_secs: f32,
        /// Number of simulation steps to run (`1..=120`).
        frames: usize,
    },
    /// Sets or clears the active runtime FPS cap (`None` = unlimited).
    SetFpsLimit {
        /// Optional target FPS cap.
        fps: Option<u32>,
    },
    /// Navigates the host application to `route` via registered `AiAppHooks`.
    Navigate {
        /// Target route string (e.g. `"/game3d"`).
        route: String,
    },
    /// Switches design scheme and/or appearance mode (`"Light"` / `"Dark"`) via `AiAppHooks`.
    SetTheme {
        /// Optional design scheme name (e.g. `"Cupertino"`, `"Linear"`).
        scheme: Option<String>,
        /// Optional appearance mode (`"Light"` or `"Dark"`).
        mode: Option<String>,
    },
    /// Invokes a custom 3D/2D game engine command via `AiAppHooks`.
    GameControl {
        /// Command name (e.g. `"camera_mode"`, `"reset"`, `"preset"`, `"sun_azimuth"`).
        command: String,
        /// Optional JSON parameter payload.
        #[serde(default)]
        payload: Value,
    },
}

/// Structured AI Schema v1 request envelope accepted by `AiSchemaProtocol::handle`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "op")]
pub enum AiSchemaRequest {
    /// Returns the formal JSON Schema specification for `euv.ai.schema.v1`.
    DescribeSchema,
    /// Captures a complete runtime, layout, performance, reactive, A11y, game, and anomaly snapshot.
    Snapshot {
        /// When `true`, includes the full recursive `layout_tree`.
        include_layout_tree: bool,
    },
    /// Returns the compiled semantic accessibility tree (`Vec<AiA11yNode>`).
    AccessibilityTree,
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
    /// Executes an interactive UI or application action and returns updated snapshot telemetry.
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
