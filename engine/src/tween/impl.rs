use super::*;

/// Implements creation, playback control, and value sampling for `Tween`.
impl<T: Interpolable + Copy> Tween<T> {
    /// Creates a new linear tween from `from` to `to` over `duration` seconds.
    ///
    /// The tween starts in the `Delayed` state only when a delay is later
    /// attached via [`Tween::with_delay`]; by default it starts `Running`.
    ///
    /// # Arguments
    ///
    /// - `T: Interpolable + Copy` - The start value.
    /// - `T: Interpolable + Copy` - The end value.
    /// - `f64` - The interpolation duration in seconds.
    ///
    /// # Returns
    ///
    /// - `Tween<T: Interpolable + Copy>` - The new tween.
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
        self.easing = easing;
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
        self.delay = delay.max(0.0);
        if self.delay > 0.0 && self.state == TweenState::Running && self.elapsed == 0.0 {
            self.state = TweenState::Delayed;
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
        self.mode = mode;
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
        self.on_complete = Some(on_complete);
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
    /// - `T: Interpolable + Copy` - The current interpolated value.
    pub fn update(&mut self, delta_time: f64) -> T {
        if self.state == TweenState::Paused || self.state == TweenState::Finished {
            return self.value();
        }
        self.elapsed += delta_time.max(0.0);
        if self.state == TweenState::Delayed {
            if self.elapsed < self.delay {
                return self.from;
            }
            self.state = TweenState::Running;
        }
        let active_elapsed: f64 = self.elapsed - self.delay;
        if self.duration <= 0.0 || active_elapsed >= self.duration {
            self.complete_cycle(active_elapsed);
        }
        self.value()
    }

    /// Returns the current interpolated value without advancing time.
    ///
    /// # Returns
    ///
    /// - `T: Interpolable + Copy` - The current eased value.
    pub fn value(&self) -> T {
        let progress: f64 = self.eased_progress();
        if self.direction == TWEEN_DIRECTION_BACKWARD {
            return self.from.lerp(self.to, 1.0 - progress);
        }
        self.from.lerp(self.to, progress)
    }

    /// Returns the eased progress of the current cycle in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `f64` - The eased progress.
    pub fn eased_progress(&self) -> f64 {
        if self.duration <= 0.0 {
            return 1.0;
        }
        let active_elapsed: f64 = (self.elapsed - self.delay).max(0.0);
        let raw: f64 = (active_elapsed / self.duration).min(1.0);
        self.easing.evaluate(raw)
    }

    /// Returns the raw (uneased) progress of the current cycle.
    ///
    /// # Returns
    ///
    /// - `f64` - The raw progress in the range 0.0 to 1.0.
    pub fn raw_progress(&self) -> f64 {
        if self.duration <= 0.0 {
            return 1.0;
        }
        ((self.elapsed - self.delay).max(0.0) / self.duration).min(1.0)
    }

    /// Pauses the tween.
    pub fn pause(&mut self) {
        if self.state == TweenState::Running || self.state == TweenState::Delayed {
            self.state = TweenState::Paused;
        }
    }

    /// Resumes a paused tween.
    pub fn resume(&mut self) {
        if self.state == TweenState::Paused {
            if self.elapsed < self.delay {
                self.state = TweenState::Delayed;
            } else {
                self.state = TweenState::Running;
            }
        }
    }

    /// Resets the tween to its initial state so it can be replayed.
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.direction = TWEEN_DIRECTION_FORWARD;
        self.state = if self.delay > 0.0 {
            TweenState::Delayed
        } else {
            TweenState::Running
        };
    }

    /// Returns whether the tween has finished (`AnimationMode::Once` only).
    ///
    /// # Returns
    ///
    /// - `bool` - True if the tween is finished.
    pub fn is_finished(&self) -> bool {
        self.state == TweenState::Finished
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
        let overflow: f64 = if self.duration > 0.0 {
            active_elapsed % self.duration
        } else {
            0.0
        };
        match self.mode {
            AnimationMode::Once => {
                self.elapsed = self.delay + self.duration;
                self.state = TweenState::Finished;
            }
            AnimationMode::Loop => {
                self.elapsed = self.delay + overflow;
            }
            AnimationMode::PingPong => {
                self.elapsed = self.delay + overflow;
                self.direction = -self.direction;
            }
        }
        if let Some(on_complete) = &self.on_complete {
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
            from: self.from,
            to: self.to,
            duration: self.duration,
            easing: self.easing,
            delay: self.delay,
            elapsed: self.elapsed,
            state: self.state,
            mode: self.mode,
            direction: self.direction,
            on_complete: self.on_complete.clone(),
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
            .debug_struct("Tween")
            .field("from", &self.from)
            .field("to", &self.to)
            .field("duration", &self.duration)
            .field("easing", &self.easing)
            .field("delay", &self.delay)
            .field("elapsed", &self.elapsed)
            .field("state", &self.state)
            .field("mode", &self.mode)
            .field("direction", &self.direction)
            .finish_non_exhaustive()
    }
}
