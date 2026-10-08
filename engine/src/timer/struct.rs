use super::*;

/// A countdown timer that fires after a fixed duration, optionally repeating.
///
/// Timers are updated with the frame delta time; [`Timer::update`] returns
/// the number of times the timer fired during that update (0 or 1 for
/// one-shot timers, possibly more for repeating timers after a long frame).
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Timer {
    /// The countdown duration in seconds.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) duration: f64,
    /// Whether the timer restarts automatically after firing.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) repeating: bool,
    /// The time accumulated since the timer started or last fired.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) elapsed: f64,
    /// Whether the timer is currently paused.
    #[get(type(copy))]
    #[new(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) paused: bool,
    /// Whether a one-shot timer has fired and stopped.
    #[get(type(copy))]
    #[new(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) finished: bool,
}
