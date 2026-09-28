use super::*;

/// One entry of the [`euv_steps`] progress stepper.
///
/// A step is a title plus an optional description; its position relative
/// to [`EuvStepsProps::current`] — not a per-step flag — decides whether
/// it renders as pending, active, or already done.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvStep {
    /// The step name rendered in `c_euv_step_title`.
    #[get(type(copy))]
    pub title: &'static str,
    /// The optional sub-copy rendered in `c_euv_step_desc`.
    #[get(type(copy))]
    pub description: &'static str,
}

/// Props for the [`euv_steps`] component.
///
/// The progress position is owned by the caller through a `Signal<usize>`
/// so advancing the stepper re-renders only the marker and class of each
/// step instead of rebuilding the list.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvStepsProps {
    /// The ordered steps to render; an empty list renders an empty row.
    pub steps: Vec<EuvStep>,
    /// The index of the step currently in progress.
    #[get(type(copy))]
    pub current: Signal<usize>,
}
