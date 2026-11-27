use super::*;

/// Declarative semantic UI node locator shared by `UiAutomator`, AI Schema v1/v2, and `euv.ui-e2e.v1`.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum UiSelector {
    /// Matches a node whose exact rendered text or input value equals the target string.
    ByText(String),
    /// Matches a node whose text, placeholder, or value contains the substring.
    ByTextContains(String),
    /// Matches an input node whose placeholder equals the target string.
    ByPlaceholder(String),
    /// Matches an input node whose current value equals the target string.
    ByInputValue(String),
    /// Matches a node whose explicit accessibility label equals the target string.
    ByA11yLabel(String),
    /// Matches a node by its accessibility role name (e.g. `"Button"`, `"TextField"`, `"ScrollContainer"`).
    ByRole(String),
    /// Matches a node by its retained scene node kind (e.g. `"View"`, `"Text"`, `"Input"`, `"ScrollView"`, `"Canvas"`, `"Portal"`).
    ByKind(String),
    /// Matches a specific node arena index (`NodeId::get_index`).
    ByNodeIndex(u32),
    /// Matches a node by its author-declared `test_id` (from node key or `test_id:` accessibility hint).
    ByTestId(String),
    /// Matches a node by its resolved `stable_key` (independent of arena index).
    ByStableKey(String),
    /// Matches any interactive clickable or focusable node.
    Interactive,
    /// Matches the `n`-th match of an inner selector (0-based).
    Nth {
        /// Inner selector to evaluate.
        inner: Box<UiSelector>,
        /// Zero-based match index.
        index: usize,
    },
    /// Matches nodes satisfying every selector in the list.
    All(Vec<UiSelector>),
}
