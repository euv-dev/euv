use super::*;

/// A single child-mutation op for [`apply_child_ops_batch`].
///
/// Each variant maps to exactly one web-sys call, preserving the
/// semantics of the previous per-op patch path:
/// - `InsertBefore { node, reference }` →
///   `parent.insert_before(node, Some(reference))` if reference is
///   `Some`, else `parent.append_child(node)`.
/// - `AppendChild(node)` → `parent.append_child(node)`.
/// - `RemoveChild(node)` → `parent.remove_child(node)`.
#[derive(Clone)]
pub(crate) enum ChildOp {
    /// `parent.insert_before(node, Some(reference))` — when `reference`
    /// is `None` this collapses to `append_child` on the JS side, but
    /// the JS glue handles the conversion explicitly to keep the
    /// helper's branching predictable.
    InsertBefore {
        /// The node to insert.
        node: Node,
        /// The reference node before which `node` is inserted, or
        /// `None` to append.
        reference: Option<Node>,
    },
    /// `parent.append_child(node)`.
    AppendChild(Node),
    /// `parent.remove_child(node)`.
    RemoveChild(Node),
}
