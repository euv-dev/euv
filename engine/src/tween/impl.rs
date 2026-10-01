use super::*;

/// Implements creation, playback control, and value sampling for `Tween`.
impl<T: Interpolable + Copy> Tween<T> {
    /// Returns the value at the start of the tween.
    ///
    /// # Returns
    ///
    /// - `T`: The start value.
    pub fn get_from(&self) -> T {
        self.from
    }

    /// Returns the value at the end of the tween.
    ///
    /// # Returns
    ///
    /// - `T`: The end value.
    pub fn get_to(&self) -> T {
        self.to
    }

    /// Returns the easing curve applied to the normalized time.
    ///
    /// # Returns
    ///
    /// - `Easing`: The easing curve.
    pub fn get_easing(&self) -> Easing {
        self.easing
    }

    /// Returns the start delay in seconds before interpolation begins.
    ///
    /// # Returns
    ///
    /// - `f64`: The delay in seconds.
    pub fn get_delay(&self) -> f64 {
        self.delay
    }

    /// Returns the time elapsed since creation, including the delay phase.
    ///
    /// # Returns
    ///
    /// - `f64`: The elapsed time in seconds.
    pub fn get_elapsed(&self) -> f64 {
        self.elapsed
    }

    /// Returns a mutable reference to the elapsed time.
    ///
    /// # Returns
    ///
    /// - `&mut f64`: The mutable elapsed time in seconds.
    pub fn get_elapsed_mut(&mut self) -> &mut f64 {
        &mut self.elapsed
    }

    /// Returns what happens when the tween reaches the end of its duration.
    ///
    /// # Returns
    ///
    /// - `AnimationMode`: The completion mode.
    pub fn get_mode(&self) -> AnimationMode {
        self.mode
    }

    /// Returns the current playback direction for ping-pong mode.
    ///
    /// # Returns
    ///
    /// - `f64`: The playback direction (1.0 = forward, -1.0 = backward).
    pub fn get_direction(&self) -> f64 {
        self.direction
    }

    /// Returns a reference to the optional completion callback slot.
    ///
    /// # Returns
    ///
    /// - `&Option<Rc<dyn Fn()>>`: The completion callback slot.
    pub fn try_get_on_complete(&self) -> &Option<Rc<dyn Fn()>> {
        &self.on_complete
    }

    /// Sets the easing curve applied to the normalized time.
    ///
    /// # Arguments
    ///
    /// - `Easing` - The easing curve to apply.
    pub fn set_easing(&mut self, easing: Easing) {
        self.easing = easing;
    }

    /// Sets the start delay in seconds before interpolation begins.
    ///
    /// # Arguments
    ///
    /// - `f64` - The delay in seconds.
    pub fn set_delay(&mut self, delay: f64) {
        self.delay = delay;
    }

    /// Sets the time elapsed since creation.
    ///
    /// # Arguments
    ///
    /// - `f64` - The elapsed time in seconds.
    pub fn set_elapsed(&mut self, elapsed: f64) {
        self.elapsed = elapsed;
    }

    /// Sets the current playback state.
    ///
    /// # Arguments
    ///
    /// - `TweenState` - The new playback state.
    pub fn set_state(&mut self, state: TweenState) {
        self.state = state;
    }

    /// Sets what happens when the tween reaches the end of its duration.
    ///
    /// # Arguments
    ///
    /// - `AnimationMode` - The completion mode.
    pub fn set_mode(&mut self, mode: AnimationMode) {
        self.mode = mode;
    }

    /// Sets the current playback direction for ping-pong mode.
    ///
    /// # Arguments
    ///
    /// - `f64` - The playback direction (1.0 = forward, -1.0 = backward).
    pub fn set_direction(&mut self, direction: f64) {
        self.direction = direction;
    }

    /// Sets the optional completion callback.
    ///
    /// # Arguments
    ///
    /// - `Option<Rc<dyn Fn()>>` - The completion callback.
    pub fn set_on_complete(&mut self, on_complete: Option<Rc<dyn Fn()>>) {
        self.on_complete = on_complete;
    }

    /// Creates a new linear tween from `from` to `to` over `duration` seconds.
    ///
    /// The tween starts in the `Delayed` state only when a delay is later
    /// attached via [`Tween::with_delay`]; by default it starts `Running`.
    ///
    /// # Arguments
    ///
    /// - `T` - The start value.
    /// - `T` - The end value.
    /// - `f64` - The interpolation duration in seconds.
    ///
    /// # Returns
    ///
    /// - `Tween<T>` - The new tween.
    pub fn create(from: T, to: T, duration: f64) -> Tween<T> {
        Tween {
            from,
            to,
            duration: duration.max(0.0),
            easing: Easing::Linear,
            delay: 0.0,
            elapsed: 0.0,
            state: TweenState::Running,
            mode: AnimationMode::Once,
            direction: TWEEN_DIRECTION_FORWARD,
            on_complete: None,
        }
    }

    /// Sets the easing curve, replacing the default `Easing::Linear`.
    ///
    /// # Arguments
    ///
    /// - `Easing` - The easing curve to apply.
    ///
    /// # Returns
    ///
    /// - `Tween<T>` - The tween, for chaining.
    pub fn with_easing(mut self, easing: Easing) -> Tween<T> {
        self.set_easing(easing);
        self
    }

    /// Sets a start delay in seconds. While the delay elapses the tween
    /// reports its `from` value and stays in the `Delayed` state.
    ///
    /// # Arguments
    ///
    /// - `f64` - The delay in seconds.
    ///
    /// # Returns
    ///
    /// - `Tween<T>` - The tween, for chaining.
    pub fn with_delay(mut self, delay: f64) -> Tween<T> {
        self.set_delay(delay.max(0.0));
        if self.get_delay() > 0.0
            && self.get_state() == TweenState::Running
            && self.get_elapsed() == 0.0
        {
            self.set_state(TweenState::Delayed);
        }
        self
    }

    /// Sets the completion mode (`Once`, `Loop`, or `PingPong`), replacing
    /// the default `AnimationMode::Once`.
    ///
    /// # Arguments
    ///
    /// - `AnimationMode` - The completion mode.
    ///
    /// # Returns
    ///
    /// - `Tween<T>` - The tween, for chaining.
    pub fn with_mode(mut self, mode: AnimationMode) -> Tween<T> {
        self.set_mode(mode);
        self
    }

    /// Attaches a callback fired every time the tween completes a cycle
    /// (once for `Once` mode, every wrap for `Loop` and `PingPong`).
    ///
    /// # Arguments
    ///
    /// - `Rc<dyn Fn()>` - The completion callback.
    ///
    /// # Returns
    ///
    /// - `Tween<T>` - The tween, for chaining.
    pub fn with_on_complete(mut self, on_complete: Rc<dyn Fn()>) -> Tween<T> {
        self.set_on_complete(Some(on_complete));
        self
    }

    /// Advances the tween by the given delta time and returns the current
    /// eased value.
    ///
    /// Has no effect while the tween is `Paused` or `Finished`.
    ///
    /// # Arguments
    ///
    /// - `f64` - The time elapsed since the last update, in seconds.
    ///
    /// # Returns
    ///
    /// - `T` - The current interpolated value.
    pub fn update(&mut self, delta_time: f64) -> T {
        if self.get_state() == TweenState::Paused || self.get_state() == TweenState::Finished {
            return self.value();
        }
        *self.get_elapsed_mut() += delta_time.max(0.0);
        if self.get_state() == TweenState::Delayed {
            if self.get_elapsed() < self.get_delay() {
                return self.get_from();
            }
            self.set_state(TweenState::Running);
        }
        let active_elapsed: f64 = self.get_elapsed() - self.get_delay();
        if self.get_duration() <= 0.0 || active_elapsed >= self.get_duration() {
            self.complete_cycle(active_elapsed);
        }
        self.value()
    }

    /// Returns the current interpolated value without advancing time.
    ///
    /// # Returns
    ///
    /// - `T` - The current eased value.
    pub fn value(&self) -> T {
        let progress: f64 = self.eased_progress();
        if self.get_direction() == TWEEN_DIRECTION_BACKWARD {
            return self.get_from().lerp(self.get_to(), 1.0 - progress);
        }
        self.get_from().lerp(self.get_to(), progress)
    }

    /// Returns the eased progress of the current cycle in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `f64` - The eased progress.
    pub fn eased_progress(&self) -> f64 {
        if self.get_duration() <= 0.0 {
            return 1.0;
        }
        let active_elapsed: f64 = (self.get_elapsed() - self.get_delay()).max(0.0);
        let raw: f64 = (active_elapsed / self.get_duration()).min(1.0);
        self.get_easing().evaluate(raw)
    }

    /// Returns the raw (uneased) progress of the current cycle.
    ///
    /// # Returns
    ///
    /// - `f64` - The raw progress in the range 0.0 to 1.0.
    pub fn raw_progress(&self) -> f64 {
        if self.get_duration() <= 0.0 {
            return 1.0;
        }
        ((self.get_elapsed() - self.get_delay()).max(0.0) / self.get_duration()).min(1.0)
    }

    /// Pauses the tween.
    pub fn pause(&mut self) {
        if self.get_state() == TweenState::Running || self.get_state() == TweenState::Delayed {
            self.set_state(TweenState::Paused);
        }
    }

    /// Resumes a paused tween.
    pub fn resume(&mut self) {
        if self.get_state() == TweenState::Paused {
            if self.get_elapsed() < self.get_delay() {
                self.set_state(TweenState::Delayed);
            } else {
                self.set_state(TweenState::Running);
            }
        }
    }

    /// Resets the tween to its initial state so it can be replayed.
    pub fn reset(&mut self) {
        self.set_elapsed(0.0);
        self.set_direction(TWEEN_DIRECTION_FORWARD);
        self.set_state(if self.get_delay() > 0.0 {
            TweenState::Delayed
        } else {
            TweenState::Running
        });
    }

    /// Returns whether the tween has finished (`AnimationMode::Once` only).
    ///
    /// # Returns
    ///
    /// - `bool` - True if the tween is finished.
    pub fn is_finished(&self) -> bool {
        self.get_state() == TweenState::Finished
    }

    /// Returns the current playback state.
    ///
    /// # Returns
    ///
    /// - `TweenState` - The playback state.
    pub fn get_state(&self) -> TweenState {
        self.state
    }

    /// Returns the configured duration in seconds.
    ///
    /// # Returns
    ///
    /// - `f64` - The duration.
    pub fn get_duration(&self) -> f64 {
        self.duration
    }

    /// Handles a completed cycle according to the configured mode.
    ///
    /// # Arguments
    ///
    /// - `f64` - The active (post-delay) elapsed time at completion.
    fn complete_cycle(&mut self, active_elapsed: f64) {
        let overflow: f64 = if self.get_duration() > 0.0 {
            active_elapsed % self.get_duration()
        } else {
            0.0
        };
        match self.get_mode() {
            AnimationMode::Once => {
                self.set_elapsed(self.get_delay() + self.get_duration());
                self.set_state(TweenState::Finished);
            }
            AnimationMode::Loop => {
                self.set_elapsed(self.get_delay() + overflow);
            }
            AnimationMode::PingPong => {
                self.set_elapsed(self.get_delay() + overflow);
                self.set_direction(-self.get_direction());
            }
        }
        if let Some(on_complete) = self.try_get_on_complete() {
            on_complete();
        }
    }
}

/// Forwards `Tween::update` through the [`Updatable`] trait so tweens can
/// participate in the same generic update loop as entities, animators,
/// scenes, and physics worlds.
impl<T: Interpolable + Copy> Updatable for Tween<T> {
    /// Advances the simulation by `delta_time` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - Seconds elapsed since the previous update.
    fn update(&mut self, delta_time: f64) {
        let _: T = Tween::update(self, delta_time);
    }
}

impl<T: Interpolable + Copy> Clone for Tween<T> {
    /// Clones the [`Tween`] by reusing shared, cheap-to-clone state where possible.
    ///
    /// # Returns
    ///
    /// - `Tween<T>` - A clone that shares the same underlying storage where applicable.
    fn clone(&self) -> Tween<T> {
        Tween {
            from: self.get_from(),
            to: self.get_to(),
            duration: self.get_duration(),
            easing: self.get_easing(),
            delay: self.get_delay(),
            elapsed: self.get_elapsed(),
            state: self.get_state(),
            mode: self.get_mode(),
            direction: self.get_direction(),
            on_complete: self.try_get_on_complete().clone(),
        }
    }
}

impl<T: Interpolable + Copy + Debug> Debug for Tween<T> {
    /// Formats the [`Tween`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(TWEEN_DEBUG_NAME)
            .field(TWEEN_FIELD_FROM, &self.from)
            .field(TWEEN_FIELD_TO, &self.to)
            .field(TWEEN_FIELD_DURATION, &self.duration)
            .field(TWEEN_FIELD_EASING, &self.easing)
            .field(TWEEN_FIELD_DELAY, &self.delay)
            .field(TWEEN_FIELD_ELAPSED, &self.elapsed)
            .field(TWEEN_FIELD_STATE, &self.state)
            .field(TWEEN_FIELD_MODE, &self.mode)
            .field(TWEEN_FIELD_DIRECTION, &self.direction)
            .finish_non_exhaustive()
    }
}
