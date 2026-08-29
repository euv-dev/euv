use super::*;

/// Inherent implementation of [`DebouncedValue`].
impl<T: Clone + PartialEq + Default + 'static> DebouncedValue<T> {
    /// Schedules `next` to become the emitted value. The
    /// commit happens on the next `tick` call at or after
    /// `now_ms + delay_ms`.
    ///
    /// Calling `set` repeatedly within `delay_ms` of each
    /// other means only the last value wins — that's the
    /// "debounce" contract.
    ///
    /// # Arguments
    ///
    /// - `T: Clone + PartialEq + Default + 'static` - A generic type parameter.
    /// - `u64` - The current time in milliseconds (any monotonic
    ///   source; on the web use `performance.now()`).
    pub fn set(&self, next: T, now_ms: u64) {
        self.get_state().set(DebounceState::Pending(now_ms, next));
    }

    /// Drives the state machine forward.
    ///
    /// - If the state is `Idle`, this is a no-op.
    /// - If the state is `Pending(set_at, value)`, this
    ///   emits `value` (writing it to the `value` signal)
    ///   iff `now_ms - set_at >= delay_ms`. Otherwise the
    ///   pending value is preserved and the call returns
    ///   `false`.
    ///
    /// Returns `true` when a pending value was emitted,
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
            DebounceState::Idle => false,
            DebounceState::Pending(set_at, _) => {
                if now_ms.saturating_sub(set_at) >= u64::from(self.delay_ms) {
                    let pending: T = match self.get_state().get() {
                        DebounceState::Pending(_, value) => value,
                        DebounceState::Idle => unreachable!(),
                    };
                    self.get_value().set(pending);
                    self.get_state().set(DebounceState::Idle);
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Cancels any pending value without emitting it.
    /// The emitted value is left untouched.
    pub fn cancel(&self) {
        self.get_state().set(DebounceState::Idle);
    }

    /// Returns the currently emitted value as a snapshot.
    ///
    /// # Returns
    ///
    /// - `T` - The current value (or a snapshot thereof).
    pub fn get(&self) -> T {
        self.get_value().get()
    }

    /// Returns `true` when a value is waiting to be
    /// emitted.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when a value is waiting to be emitted.
    pub fn is_pending(&self) -> bool {
        matches!(self.get_state().get(), DebounceState::Pending(_, _))
    }
}

/// Debug formatting for [`DebouncedValue`].
impl<T: Clone + PartialEq + Debug + Default + 'static> Display for DebouncedValue<T> {
    /// Formats the [`DebouncedValue`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `FmtResult` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let pending: &DebounceState<T> = &self.get_state().get();
        match pending {
            DebounceState::Idle => {
                write!(formatter, "DebouncedValue({:?})", self.get_value().get())
            }
            DebounceState::Pending(_, value) => {
                write!(formatter, "DebouncedValue(pending={value:?})")
            }
        }
    }
}
