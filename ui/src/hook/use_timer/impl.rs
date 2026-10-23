use super::*;

/// Inherent implementation of [`UseTimer`].
impl UseTimer {
    /// Creates a handle with the tick count starting at zero.
    ///
    /// # Arguments
    ///
    /// - `u32` - The period in milliseconds.
    ///
    /// # Returns
    ///
    /// - `UseTimer` - The fresh handle.
    pub(crate) fn new(interval_ms: u32) -> Self {
        Self {
            ticks: Signal::create(0),
            interval_ms,
        }
    }

    /// Returns the tick count as a snapshot.
    ///
    /// # Returns
    ///
    /// - `u64` - How many times the timer has fired.
    pub fn current_ticks(&self) -> u64 {
        self.get_ticks().get()
    }

    /// Resets the tick count to zero.
    pub fn reset(&self) {
        self.get_ticks().set(0);
    }
}
