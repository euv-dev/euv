use super::*;

/// Props for the [`euv_tree_select`] component.
///
/// The tree the panel shows is a list of [`EuvTreeNode`]s — the same node
/// type the [`euv_tree`] component renders — so one tree can be shared
/// between a standalone tree and a select. The selected value lives in a
/// caller-owned `Signal<String>`, so the control takes part in two-way
/// binding with the rest of the page; the open state of the panel is
/// owned by the component itself.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTreeSelectProps {
    /// The tree the panel shows.
    pub nodes: Vec<EuvTreeNode>,
    /// The value of the selected node.
    pub value: Signal<String>,
    /// The text shown when nothing is selected.
    pub placeholder: &'static str,
    /// The group label above the control.
    pub label: &'static str,
    /// The callback fired when a node is chosen.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}
