use super::*;

/// The size of the custom Select trigger.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvSelectSize {
    /// The standard Select trigger size.
    #[default]
    Default,
    /// The compact Select trigger size.
    Sm,
}

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
