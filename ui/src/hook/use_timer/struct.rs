use super::*;

/// Reactive state for a repeating timer.
///
/// The handle carries the number of ticks that have fired and the
/// period they fire at.
///
/// The tick count is a `Signal<u64>`: a render closure that reads it
/// re-renders on every tick, and nothing else in the tree is disturbed.
///
/// The interval is owned by the framework and is cleared when the
/// component unmounts, so a timer cannot outlive its owner.
#[derive(Clone, CustomDebug, Data)]
pub struct UseTimer {
    /// How many times the timer has fired.
    #[get(type(copy))]
    pub ticks: Signal<u64>,
    /// The period in milliseconds.
    #[get(type(copy))]
    pub interval_ms: u32,
}
