use super::*;

/// One row of the option panel: a group caption or an option.
///
/// The panel is rendered as a flat list of these rows so the caption of a
/// group always sits immediately above the options it introduces.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SelectPanelEntry {
    /// The caption row introducing a group of options.
    Caption(&'static str),
    /// An option row.
    Option(EuvSelectOption),
}
