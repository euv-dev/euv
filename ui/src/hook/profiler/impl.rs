use super::*;

/// Inherent implementation of [`ProfilerHandle`].
impl ProfilerHandle {
    /// Constructs a `ProfilerHandle` with an empty entries log.
    ///
    /// Lombok `New` cannot derive this for us because the
    /// `entries` field is a `Signal<...>` rather than a plain
    /// value — we cannot synthesise a meaningful default at
    /// compile time, so the hook-context factory wires one up at
    /// runtime via `Signal::create(Vec::new())`.
    ///
    /// # Returns
    ///
    /// - `ProfilerHandle` - A profiler handle with no recorded
    ///   measurements.
    pub fn new_with_empty_entries() -> Self {
        Self {
            entries: Signal::create(Vec::new()),
        }
    }

    /// Records a fresh measurement around the given closure.
    ///
    /// Captures the start timestamp, runs `f`, captures the
    /// end timestamp, and pushes a `ProfileEntry { label,
    /// elapsed_ms, timestamp_ms }` into the entries signal.
    /// The `elapsed_ms` is `>= 0.0` by construction (start is
    /// always captured before `f` runs).
    ///
    /// # Arguments
    ///
    /// - `&str` - The label for this measurement.
    /// - `F: FnOnce() -> R` - The closure to measure. Can
    ///   return any type — the return value is forwarded to
    ///   the caller unchanged.
    ///
    /// # Returns
    ///
    /// - `R` - Whatever `f` returned.
    pub fn measure<F, R>(&self, label: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let started_ms: f64 = now_ms();
        let result: R = f();
        let ended_ms: f64 = now_ms();
        let entry: ProfileEntry =
            ProfileEntry::new(label.to_string(), ended_ms - started_ms, ended_ms);
        // Push the entry onto the existing entries vector.
        // Read-modify-write via `.get()` is necessary because
        // `Signal<T>::set` requires `T: Clone` (which
        // `Vec<ProfileEntry>` is) but does NOT take `&mut T`
        // — the entire new value is supplied as the argument.
        let mut current: Vec<ProfileEntry> = self.get_entries().get();
        current.push(entry);
        self.get_entries().set(current);
        result
    }

    /// Starts a measurement that will end later.
    ///
    /// Use this when the measured region is not a single
    /// closure — e.g. you want to bracket an async operation
    /// or a callback fired from event handling. The returned
    /// `ProfilerMark` knows the start timestamp and the
    /// entries signal; pass it to `end()` when the work
    /// completes.
    ///
    /// # Arguments
    ///
    /// - `&str` - The label for this measurement.
    ///
    /// # Returns
    ///
    /// - `ProfilerMark` - An RAII-ish guard. Call `mark.end()`
    ///   to push the `ProfileEntry`; drop without `end()` to
    ///   discard the measurement.
    pub fn begin(&self, label: &str) -> ProfilerMark {
        ProfilerMark::new(label.to_string(), now_ms(), *self.get_entries())
    }

    /// Empties the entries vector. Useful between benchmarks
    /// ("measure just this call, not the previous ones too")
    /// and in tests ("start from a clean slate").
    pub fn clear(&self) {
        self.get_entries().set(Vec::new());
    }
}

/// Inherent implementation of [`ProfilerMark`].
impl ProfilerMark {
    /// Closes the measurement started by `begin()` and pushes
    /// the resulting `ProfileEntry` into the entries signal.
    ///
    /// After calling `end()`, the marker is consumed and
    /// cannot be reused. Calling `end()` twice is a no-op on
    /// the second call — the marker is moved into the first
    /// call, so the borrow checker prevents a second
    /// invocation in well-typed code.
    pub fn end(self) {
        let ended_ms: f64 = now_ms();
        let entry: ProfileEntry = ProfileEntry::new(
            self.get_label().clone(),
            ended_ms - self.get_started_ms(),
            ended_ms,
        );
        let mut current: Vec<ProfileEntry> = self.get_entries().get();
        current.push(entry);
        self.get_entries().set(current);
    }
}

/// `ProfilerHandle` is `Copy` because `Signal<Vec<ProfileEntry>>`
/// is itself `Copy` (the registry hands out cheap `usize`
/// addresses; the vector lives in the global signal store).
impl Copy for ProfilerHandle {}
