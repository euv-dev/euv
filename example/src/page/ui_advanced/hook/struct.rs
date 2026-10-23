use super::*;

/// The reactive state the advanced demo page drives.
///
/// Every control on the page owns its selection through a signal, so
/// the page can read the current value back after the reader changes
/// it.
#[derive(Clone, Debug)]
pub(crate) struct AdvancedDemoState {
    /// The time the picker is showing.
    pub(crate) time_value: Signal<String>,
    /// The rows the transfer marks in either list.
    pub(crate) transfer_selected: Signal<Vec<String>>,
    /// The value the tree-select's chosen node.
    pub(crate) tree_value: Signal<String>,
    /// The path the cascader has chosen.
    pub(crate) cascader_value: Signal<Vec<String>>,
    /// The text the mentions field holds.
    pub(crate) mention_value: Signal<String>,
    /// The rows the transfer shows in the source list.
    pub(crate) transfer_source: Signal<Vec<EuvTransferItem>>,
    /// The rows the transfer shows in the target list.
    pub(crate) transfer_target: Signal<Vec<EuvTransferItem>>,
}
