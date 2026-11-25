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
    /// Moves pointer hover over `selector`.
    Hover {
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
    /// Asserts at least one visible node matches `selector`.
    AssertVisible {
        /// Expected visible node locator.
        selector: UiSelector,
    },
    /// Asserts no visible node overflows the viewport right edge.
    AssertNoHorizontalOverflow,
    /// Asserts no visible sibling text nodes overlap.
    AssertNoTextOverlap,
}
