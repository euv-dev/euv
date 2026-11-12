use super::*;

/// One node of the [`euv_tree`] component.
///
/// A node is a label, the opaque value written into the selection signal
/// when its row is activated, and the child nodes rendered beneath this one. The
/// struct is recursive, so it cannot be `Copy`; it derives `Clone` so a
/// caller can build several trees from one shared template.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTreeNode {
    /// The visible node text rendered in `c_euv_tree_node_label`.
    pub label: &'static str,
    /// The opaque value written into the `selected` signal when the row is
    /// activated, and the identity the expansion state is keyed by.
    pub value: &'static str,
    /// The child nodes rendered beneath this one.
    pub children: Vec<EuvTreeNode>,
}

/// Props for the [`euv_tree`] component.
///
/// Defines the strongly-typed interface for a hierarchical tree. The
/// selection is owned by the caller through the `selected` signal, so the
/// tree takes part in two-way binding with the rest of the page; the
/// expansion state is owned by the component itself.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTreeProps {
    /// The root nodes of the tree.
    pub nodes: Vec<EuvTreeNode>,
    /// The signal holding the selected node's value.
    pub selected: Signal<String>,
    /// Optional callback invoked with the activated node's value.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
    /// Whether every node starts out expanded on the first render.
    pub default_expanded: bool,
}

/// Shared render state passed through the recursive tree renderer.
pub(super) struct TreeViewContext<'a> {
    /// The caller-owned selected-value signal.
    pub(super) selected: Signal<String>,
    /// The value currently selected by the caller.
    pub(super) selected_value: &'a str,
    /// The visible value that owns the single tab stop.
    pub(super) roving_value: &'a str,
    /// The component-owned expanded-values signal.
    pub(super) expanded: Signal<Vec<String>>,
    /// The expanded values read once for this render.
    pub(super) expanded_values: &'a [String],
    /// The callback invoked when a node is activated.
    pub(super) on_select: Option<Rc<dyn Fn(&'static str)>>,
}
