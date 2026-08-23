use super::*;

/// Represents the type of an HTML tag or a component.
///
/// Distinguishes between standard HTML elements and user-defined components.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Tag {
    /// A standard HTML element identified by its tag name.
    Element(String),
    /// A custom component type.
    Component(String),
    /// A portal placeholder whose children are rendered into a
    /// different DOM subtree than the rest of the virtual DOM.
    ///
    /// The payload is a CSS selector (`"#modal-root"`,
    /// `"body"`, `".toast-host"`) identifying the destination
    /// element. The renderer resolves the selector at mount time
    /// via `document.query_selector(...)`, falling back to
    /// `document.body()` when the selector matches nothing.
    ///
    /// Portal nodes render an HTML comment marker in their
    /// declared position so the parent's DOM tree stays
    /// structurally intact (the alternative — leaving a hole in
    /// the parent — breaks euv's positional patch loop, which
    /// walks child nodes by index).
    ///
    /// Patch semantics:
    ///
    /// - When the target selector is unchanged between renders,
    ///   the portal's children are patched in place inside the
    ///   target element.
    /// - When the target selector changes, the previous target's
    ///   children are unmounted and the new target receives a
    ///   fresh mount.
    ///
    /// Designed for modals, tooltips, dropdowns, and toasts —
    /// UI that needs to escape `overflow: hidden` parents and
    /// sit at the document root regardless of where it was
    /// declared in the virtual DOM tree.
    Portal(String),
}

/// Represents a node in the virtual DOM tree.
///
/// The core enum representing elements, text, fragments, and empty nodes.
/// The generic parameter `T` carries the component props type for component nodes.
/// For non-component nodes, `T` defaults to `()`.
pub enum VirtualNode<T = ()> {
    /// An element node with a tag, attributes, children, and optional props.
    Element {
        /// The tag type of this element.
        tag: Tag,
        /// The attributes attached to this element.
        attributes: Vec<AttributeEntry>,
        /// The child nodes.
        children: Vec<VirtualNode>,
        /// An optional key for diffing.
        key: Option<String>,
        /// The component props, present only for component nodes.
        props: Option<Box<T>>,
    },
    /// A text node containing string content and an optional reactive signal.
    Text(TextNode),
    /// A fragment of multiple nodes without a wrapper element.
    Fragment(Vec<VirtualNode>),
    /// A dynamic node that re-renders based on signal changes.
    Dynamic(DynamicNode),
    /// An empty placeholder node.
    Empty,
}
