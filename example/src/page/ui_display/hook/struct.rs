use super::*;

/// The reactive state the display demo page drives.
///
/// The page owns the signals every interactive component on it reads:
/// which carousel slide is on screen, which anchor entry is active, how
/// wide the first and second panes of the split view are, and whether the
/// back-to-top button is showing.
#[derive(Clone, Debug)]
pub(crate) struct DisplayDemoState {
    /// The index of the carousel slide that is on screen.
    pub(crate) carousel_index: Signal<u32>,
    /// The value of the anchor entry that is active.
    pub(crate) anchor_active: Signal<String>,
    /// The share of the split view the first pane takes, in percent.
    pub(crate) split_size: Signal<f64>,
    /// The complementary share of the split view the second pane takes.
    pub(crate) split_size_secondary: Signal<f64>,
    /// Whether the back-to-top button is showing.
    pub(crate) back_top_visible: Signal<bool>,
    /// Whether the spinner shows its caption next to the glyph.
    pub(crate) spinner_caption: Signal<bool>,
    /// The selected tab in the interactive reference examples.
    pub(crate) reference_tab: Signal<String>,
    /// The shared slider and progress value.
    pub(crate) reference_value: Signal<f64>,
    /// Whether the reference switch is checked.
    pub(crate) reference_checked: Signal<bool>,
    /// Whether the disabled reference controls can receive input.
    pub(crate) reference_disabled: Signal<bool>,
    /// Whether the reference dialog is open.
    pub(crate) reference_dialog_open: Signal<bool>,
}
