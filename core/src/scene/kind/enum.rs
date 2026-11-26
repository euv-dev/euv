use super::*;

/// Semantic kind of a node in the cross-platform retained scene graph.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum NodeKind {
    /// Generic layout and paint box container.
    #[default]
    View,
    /// Text run node carrying UTF-8 content.
    Text(String),
    /// Decoded bitmap image node.
    Image(ImageBuffer),
    /// Custom vector/2D canvas surface invoking a draw callback.
    Canvas,
    /// Editable text input node with caret and IME support.
    Input(InputNodeState),
    /// Clipped scrollable container node.
    ScrollView,
    /// Overlay layer rendered above normal document flow.
    Portal,
    /// Logical grouping container without its own box paint.
    Fragment,
}
