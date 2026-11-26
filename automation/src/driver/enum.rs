use super::*;

/// Error returned when a UI automation query, interaction, or assertion fails.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AutomationError {
    /// No visible node matched the requested selector.
    NodeNotFound(String),
    /// An assertion failed with an explicit diagnostic reason.
    AssertionFailed(String),
    /// Frame stepping or platform presentation returned an error.
    RuntimeError(String),
    /// Wait condition did not become true within the frame budget.
    Timeout(String),
}

/// Declarative automation step executed inside a `UiTestScript`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "step")]
pub enum UiTestStep {
    /// Clicks the node matching `selector`.
    Click {
        /// Target locator.
        selector: UiSelector,
    },
    /// Clicks exact logical coordinates `(x, y)`.
    ClickAt {
        /// Logical X coordinate.
        x: f32,
        /// Logical Y coordinate.
        y: f32,
    },
    /// Double-clicks the node matching `selector`.
    DoubleClick {
        /// Target locator.
        selector: UiSelector,
    },
    /// Moves pointer hover over `selector`.
    Hover {
        /// Target locator.
        selector: UiSelector,
    },
    /// Focuses the node matching `selector`.
    Focus {
        /// Target locator.
        selector: UiSelector,
    },
    /// Clicks and types UTF-8 text into `selector`.
    TypeText {
        /// Target input locator.
        selector: UiSelector,
        /// UTF-8 text payload.
        text: String,
    },
    /// Clears all text in the input matching `selector`.
    ClearInput {
        /// Target input locator.
        selector: UiSelector,
    },
    /// Drags from the center of `from` to the center of `to`.
    Drag {
        /// Drag start locator.
        from: UiSelector,
        /// Drag end locator.
        to: UiSelector,
    },
    /// Synthesizes a touch or multi-touch gesture.
    Gesture {
        /// Gesture classification.
        gesture: AiGestureKind,
        /// Logical anchor X coordinate.
        x: f32,
        /// Logical anchor Y coordinate.
        y: f32,
        /// Horizontal displacement delta.
        #[serde(default)]
        dx: f32,
        /// Vertical displacement delta.
        #[serde(default)]
        dy: f32,
        /// Pinch scale multiplier.
        #[serde(default)]
        scale: f32,
    },
    /// Dispatches a keyboard key press by name.
    PressKey {
        /// Key name string.
        key: String,
    },
    /// Scrolls the matching container by `(dx, dy)`.
    Scroll {
        /// Target scroll container locator.
        selector: UiSelector,
        /// Horizontal scroll delta.
        dx: f32,
        /// Vertical scroll delta.
        dy: f32,
    },
    /// Resizes viewport to `(width, height, scale)`.
    Resize {
        /// Logical width.
        width: u32,
        /// Logical height.
        height: u32,
        /// Display scale factor.
        scale: f32,
    },
    /// Advances `count` rendered frames.
    StepFrames {
        /// Number of frames to render.
        count: usize,
    },
    /// Steps game simulation for `frames` ticks of `dt_secs`.
    StepGame {
        /// Delta time per tick in seconds.
        dt_secs: f32,
        /// Number of ticks.
        frames: usize,
    },
    /// Configures the active runtime FPS limiter.
    SetFpsLimit {
        /// Optional FPS cap (`None` = unlimited).
        fps: Option<u32>,
    },
    /// Asserts at least one visible node matches `selector`.
    AssertVisible {
        /// Expected visible node locator.
        selector: UiSelector,
    },
    /// Asserts no visible node matches `selector`.
    AssertNotVisible {
        /// Expected absent node locator.
        selector: UiSelector,
    },
    /// Asserts the first matching node has exact text or input value `expected`.
    AssertTextEquals {
        /// Target node locator.
        selector: UiSelector,
        /// Expected exact string value.
        expected: String,
    },
    /// Asserts no visible node overflows the viewport right edge.
    AssertNoHorizontalOverflow,
    /// Asserts no visible sibling text nodes overlap.
    AssertNoTextOverlap,
}
