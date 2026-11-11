use super::*;

/// One grouped, separator or option entry rendered by the Select panel.
#[derive(Clone, CustomDebug)]
pub(crate) enum SelectPanelEntry {
    /// A labelled group of options.
    Group {
        label: &'static str,
        options: Vec<EuvSelectOption>,
    },
    /// A visual separator between options.
    Separator,
    /// An ungrouped option row.
    Option(EuvSelectOption),
}
