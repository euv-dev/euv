use super::*;

/// One row of a transfer list.
///
/// Carries the machine value the row is keyed by, the label the reader
/// sees, and whether the row can be marked at all.
///
/// The row is comparable so that a caller can keep a whole list of them in
/// a signal — the transfer does not own its rows, the page does.
#[derive(Clone, Copy, CustomDebug, Data, Default, New, PartialEq)]
pub struct EuvTransferItem {
    /// The machine value of the row.
    #[get(type(copy))]
    pub value: &'static str,
    /// The label the reader sees.
    #[get(type(copy))]
    pub label: &'static str,
    /// Whether the row is disabled.
    ///
    /// A disabled row cannot be marked, so clicking it never writes into
    /// the selection signal.
    #[get(type(copy))]
    pub disabled: bool,
}

/// Props for the [`euv_transfer`] component.
///
/// The component is presentational: it owns neither of the two lists and
/// only reports the moves the reader asks for. The marked rows live in a
/// caller-owned `Signal<Vec<String>>`, so the transfer takes part in
/// two-way binding with the rest of the page.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvTransferProps {
    /// The rows of the source list.
    pub source: Vec<EuvTransferItem>,
    /// The rows of the target list.
    pub target: Vec<EuvTransferItem>,
    /// The rows currently marked in either list.
    pub selected: Signal<Vec<String>>,
    /// The label of the source list's header.
    pub source_title: &'static str,
    /// The label of the target list's header.
    pub target_title: &'static str,
    /// The callback fired when a row moves from the source to the target.
    #[debug(skip)]
    pub on_move_to_target: Option<Rc<dyn Fn(&'static str)>>,
    /// The callback fired when a row moves from the target to the source.
    #[debug(skip)]
    pub on_move_to_source: Option<Rc<dyn Fn(&'static str)>>,
}
