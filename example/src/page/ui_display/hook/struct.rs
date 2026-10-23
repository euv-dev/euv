use super::*;

/// The reactive state the display demo page drives.
///
/// The page owns the signals every interactive component on it reads:
/// which carousel slide is on screen, which anchor entry is active, how
/// wide the first pane of the split view is, and whether the
/// back-to-top button is showing.
#[derive(Clone, Debug)]
pub(crate) struct DisplayDemoState {
    /// The index of the carousel slide that is on screen.
    pub(crate) carousel_index: Signal<u32>,
    /// The value of the anchor entry that is active.
    pub(crate) anchor_active: Signal<String>,
    /// The share of the split view the first pane takes, in percent.
    pub(crate) split_size: Signal<f64>,
    /// Whether the back-to-top button is showing.
    pub(crate) back_top_visible: Signal<bool>,
}
