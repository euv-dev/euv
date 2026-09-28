use super::*;

/// Props for the `euv_progress` component.
///
/// Defines the strongly-typed interface for a determinate progress bar.
/// The fill level is owned by the caller through a `Signal<f64>` so the
/// bar can be driven by a download, a form submission, or a timer.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvProgressProps {
    /// The completion percentage in the `0.0..=100.0` range; values
    /// outside it are clamped by [`progress_percent_clamp`].
    #[get(type(copy))]
    pub percent: Signal<f64>,
    /// The label text displayed beside the bar.
    #[get(type(copy))]
    pub label: &'static str,
    /// Whether the bar is currently advancing; drives the animated
    /// running style instead of the settled one.
    #[get(type(copy))]
    pub active: Signal<bool>,
}
