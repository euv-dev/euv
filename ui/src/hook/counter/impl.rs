use super::*;

/// Inherent implementation of [`Counter`].
impl Counter {
    /// Adds `step` to the value, clamping into `[min, max]`
    /// afterwards. If the counter is already at `max`, the
    /// value is left at `max`.
    pub fn increment(&self) {
        let current: i32 = self.get_value().get();
        let mut next: i32 = current.saturating_add(self.step);
        if let Some(max) = self.max {
            next = next.min(max);
        }
        if let Some(min) = self.min {
            next = next.max(min);
        }
        self.get_value().set(next);
    }

    /// Subtracts `step` from the value, clamping into
    /// `[min, max]` afterwards. If the counter is already
    /// at `min`, the value is left at `min`.
    pub fn decrement(&self) {
        let current: i32 = self.get_value().get();
        let mut next: i32 = current.saturating_sub(self.step);
        if let Some(min) = self.min {
            next = next.max(min);
        }
        if let Some(max) = self.max {
            next = next.min(max);
        }
        self.get_value().set(next);
    }

    /// Replaces the value with `next`, clamping into
    /// `[min, max]` if bounds are set.
    ///
    /// # Arguments
    ///
    /// - `i32` - A 32-bit signed integer (`i32`).
    pub fn set(&self, next: i32) {
        let clamped: i32 = match (self.min, self.max) {
            (Some(min), Some(max)) => next.clamp(min, max),
            (Some(min), None) => next.max(min),
            (None, Some(max)) => next.min(max),
            (None, None) => next,
        };
        self.get_value().set(clamped);
    }

    /// Replaces the value with `next` without clamping.
    /// Bypasses both the `min` and `max` bounds. Use
    /// when you genuinely want to push the counter outside
    /// its configured range (e.g., to "reset to a
    /// deliberately out-of-range sentinel" or to recover
    /// from an invalid configuration).
    ///
    /// # Arguments
    ///
    /// - `i32` - A 32-bit signed integer (`i32`).
    pub fn set_unchecked(&self, next: i32) {
        self.get_value().set(next);
    }

    /// Returns `true` when the value is at the configured
    /// `max` (or always `false` if unbounded above).
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the value is at the configured maximum.
    pub fn is_at_max(&self) -> bool {
        match self.max {
            Some(max) => self.get_value().get() >= max,
            None => false,
        }
    }

    /// Returns `true` when the value is at the configured
    /// `min` (or always `false` if unbounded below).
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the value is at the configured minimum.
    pub fn is_at_min(&self) -> bool {
        match self.min {
            Some(min) => self.get_value().get() <= min,
            None => false,
        }
    }

    /// Returns the current value as a snapshot.
    ///
    /// # Returns
    ///
    /// - `i32` - The current value (or a snapshot thereof).
    pub fn get(&self) -> i32 {
        self.get_value().get()
    }
}

/// Formatting / debug-printing for [`Counter`].
impl Display for Counter {
    /// Formats the [`Counter`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "Counter({})", self.get_value().get())
    }
}
