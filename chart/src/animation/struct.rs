use super::*;

/// Animates the values of a chart's series.
///
/// The animator keeps the values the chart showed last and the values it
/// should show now, and interpolates between them. The chart's layout
/// is recomputed on every frame of the animation, so the shapes really
/// move rather than jumping at the end.
///
/// The clock and the easing come from `euv-engine`: the chart's motion
/// is the framework's motion, down to the delay and the pause. The
/// engine's tween counts in seconds and the chart's API counts in
/// milliseconds, and the conversion happens here and nowhere else.
#[derive(Clone, Data, Debug)]
pub struct Animator {
    /// The values each series animates from.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) from: Vec<Vec<f64>>,
    /// The values each series animates to.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) to: Vec<Vec<f64>>,
    /// The values the chart shows right now.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) current: Vec<Vec<f64>>,
    /// The tween that drives the progress of the animation.
    ///
    /// The tween carries the clock, the easing, and the delay. It runs
    /// from zero to one, and the chart interpolates its own values with
    /// the progress it reports.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tween: Tween<f64>,
    /// The duration of one animation, in milliseconds.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) duration: f64,
    /// The delay before the animation starts, in milliseconds.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) delay: f64,
    /// The option the chart was created with, for the restore action.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) initial: Option<ChartOption>,
}
