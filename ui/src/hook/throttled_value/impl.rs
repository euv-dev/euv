use super::*;

/// Inherent implementation of [`ThrottledValue`].
impl<T: Clone + PartialEq + Default + 'static> ThrottledValue<T> {
    /// Sends `next` through the throttle.
    ///
    /// - If the throttle is idle (no cooldown active), the
    ///   value is emitted immediately, and a cooldown
    ///   window of `interval_ms` opens starting at `now`.
    /// - If a cooldown is active, `next` is stored as the
    ///   next pending value. The pending value will be
    ///   committed on the next `tick` call once the
    ///   cooldown expires. Intermediate `set` calls during
    ///   the cooldown overwrite the pending slot — only
    ///   the most recent input wins.
    ///
    /// `interval_ms = 0` collapses to "every `set` is
    /// immediately committed" — the cooldown branch is
    /// never taken.
    ///
    /// # Arguments
    ///
    /// - `T: Clone + PartialEq + Default + 'static` - A generic type parameter.
    /// - `u64` - The current time in milliseconds (any monotonic
    ///   source; on the web use `performance.now()`).
    pub fn set(&self, next: T, now_ms: u64) {
        if self.interval_ms == 0 {
            self.get_value().set(next);
            self.get_pending().set(None);
            self.get_state().set(ThrottleState::Idle);
            return;
        }
        match self.get_state().get() {
            ThrottleState::Idle => {
                self.get_value().set(next);
                self.get_state().set(ThrottleState::Cooldown(now_ms));
            }
            ThrottleState::Cooldown(_) => {
                self.get_pending().set(Some(next));
            }
        }
    }

    /// Drives the throttle forward.
    ///
    /// - If idle, no-op.
    /// - If a cooldown is active and the cooldown
    ///   window has elapsed, any pending value is
    ///   committed and the state returns to `Idle`. If
    ///   nothing is pending, the state still returns to
    ///   `Idle`. The cooldown simply lapses in that case.
    ///
    /// Returns `true` when a pending value was committed,
    /// `false` otherwise.
    ///
    /// # Arguments
    ///
    /// - `u64` - The current time in milliseconds.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn tick(&self, now_ms: u64) -> bool {
        match self.get_state().get() {
            ThrottleState::Idle => false,
            ThrottleState::Cooldown(start) => {
                if now_ms.saturating_sub(start) < u64::from(self.interval_ms) {
                    return false;
                }
                let committed: bool = match self.get_pending().get() {
                    Some(pending) => {
                        self.get_value().set(pending);
                        self.get_pending().set(None);
                        true
                    }
                    None => false,
                };
                self.get_state().set(ThrottleState::Idle);
                committed
            }
        }
    }

    /// Drops any pending value and ends the cooldown.
    /// The emitted value is left untouched.
    pub fn cancel(&self) {
        self.get_pending().set(None);
        self.get_state().set(ThrottleState::Idle);
    }

    /// Returns the currently emitted value as a snapshot.
    ///
    /// # Returns
    ///
    /// - `T` - The current value (or a snapshot thereof).
    pub fn get(&self) -> T {
        self.get_value().get()
    }

    /// Returns `true` when the throttle is in a cooldown
    /// window (recent `set` that has not yet had a chance
    /// to commit any pending follow-up).
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when a throttle delay is active.
    pub fn is_throttling(&self) -> bool {
        matches!(self.get_state().get(), ThrottleState::Cooldown(_))
    }
}

/// Debug formatting for [`ThrottledValue`].
impl<T: Clone + PartialEq + Debug + Default + 'static> Display for ThrottledValue<T> {
    /// Formats the [`ThrottledValue`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `FmtResult` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match &self.get_state().get() {
            ThrottleState::Idle => {
                write!(formatter, "ThrottledValue({:?})", self.get_value().get())
            }
            ThrottleState::Cooldown(_) => {
                write!(
                    formatter,
                    "ThrottledValue(cooldown={:?})",
                    self.get_value().get()
                )
            }
        }
    }
}
