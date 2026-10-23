use super::*;

/// Inherent implementation of [`Animator`].
impl Animator {
    /// Returns an animator with the default settings.
    pub fn new() -> Self {
        Self {
            from: Vec::new(),
            to: Vec::new(),
            current: Vec::new(),
            tween: Tween::create(0.0, 1.0, 0.0),
            duration: DEFAULT_ANIMATION_DURATION,
            delay: 0.0,
            initial: None,
        }
    }

    /// Returns the duration of one animation, in milliseconds.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn duration(&self) -> f64 {
        self.get_duration()
    }

    /// Sets the duration of one animation, in milliseconds.
    ///
    /// A duration of zero turns the animation off, which is what a
    /// server-side render or a benchmark wants. The new duration
    /// applies to the animation that starts next.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn set_duration(&mut self, duration: f64) -> &mut Self {
        *self.get_mut_duration() = duration.max(0.0);
        self
    }

    /// Returns the delay before the animation starts, in milliseconds.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn delay(&self) -> f64 {
        self.get_delay()
    }

    /// Sets the delay before the animation starts, in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn set_delay(&mut self, delay: f64) -> &mut Self {
        let value: f64 = delay.max(0.0);
        *self.get_mut_delay() = value;
        self.get_mut_tween()
            .set_delay(value / MILLISECONDS_PER_SECOND);
        self
    }

    /// Returns the easing of the animation.
    ///
    /// # Returns
    ///
    /// - `Easing` - A `Easing` value.
    pub fn easing(&self) -> Easing {
        self.get_tween().get_easing()
    }

    /// Sets the easing of the animation.
    ///
    /// # Arguments
    ///
    /// - `Easing` - A `Easing` parameter.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn set_easing(&mut self, easing: Easing) -> &mut Self {
        self.get_mut_tween().set_easing(easing);
        self
    }

    /// Returns `true` when an animation is running.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_running(&self) -> bool {
        !self.get_tween().is_finished()
    }

    /// Pauses the animation.
    ///
    /// The values stay where they are until the animation is resumed.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn pause(&mut self) -> &mut Self {
        self.get_mut_tween().pause();
        self
    }

    /// Resumes a paused animation.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn resume(&mut self) -> &mut Self {
        self.get_mut_tween().resume();
        self
    }

    /// Starts the animation over.
    ///
    /// The values go back to where the animation started, and the
    /// animation runs again from there.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn reset(&mut self) -> &mut Self {
        self.get_mut_tween().reset();
        let from: Vec<Vec<f64>> = self.get_from().clone();
        self.set_current(from);
        self
    }

    /// Remembers the option the chart started with.
    ///
    /// # Arguments
    ///
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn remember_initial(&mut self, option: &ChartOption) -> &mut Self {
        self.set_initial(Some(option.clone()));
        self
    }

    /// Starts an animation towards the given option.
    ///
    /// The values the chart shows now become the starting point of the
    /// animation; a series that is new grows from zero.
    ///
    /// # Arguments
    ///
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn begin(&mut self, option: &ChartOption) -> &mut Self {
        let target: Vec<Vec<f64>> = collect_values(option);
        if self.get_duration() <= 0.0 {
            self.set_current(target.clone());
            self.set_to(target.clone());
            self.set_from(target);
            // A tween of no duration is over before it starts.
            let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 0.0);
            tween.set_state(TweenState::Finished);
            self.set_tween(tween);
            return self;
        }
        // A series that was not there before starts at zero, which is
        // what makes a new series grow out of the axis.
        let from: Vec<Vec<f64>> = align_values(self.get_current(), &target);
        self.set_from(from.clone());
        self.set_to(target.clone());
        self.set_current(from);
        // The tween is the clock: it is rebuilt for every animation,
        // because its duration is fixed when it is created.
        let tween: Tween<f64> =
            Tween::create(0.0, 1.0, self.get_duration() / MILLISECONDS_PER_SECOND)
                .with_easing(self.get_tween().get_easing())
                .with_delay(self.get_delay() / MILLISECONDS_PER_SECOND);
        self.set_tween(tween);
        self
    }

    /// Advances the animation by the given number of milliseconds.
    ///
    /// Returns `true` when the animation is still running.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn advance(&mut self, delta: f64) -> bool {
        if self.get_tween().is_finished() {
            return false;
        }
        self.get_mut_tween()
            .update(delta.max(0.0) / MILLISECONDS_PER_SECOND);
        let progress: f64 = self.get_tween().value();
        let current: Vec<Vec<f64>> = interpolate(self.get_from(), self.get_to(), progress);
        self.set_current(current);
        !self.get_tween().is_finished()
    }

    /// Writes the animated values into the option.
    ///
    /// The option's series data is replaced by the animated values, so
    /// that the layout — and therefore the picture — follows the
    /// animation.
    ///
    /// # Arguments
    ///
    /// - `&mut ChartOption` - Mutable reference to a `ChartOption` (mutated in place).
    pub fn apply(&self, option: &mut ChartOption) {
        if self.get_current().is_empty() {
            return;
        }
        for (index, series) in option.series.iter_mut().enumerate() {
            let Some(values) = self.get_current().get(index) else {
                continue;
            };
            for (data_index, data) in series.data.iter_mut().enumerate() {
                let Some(value) = values.get(data_index) else {
                    continue;
                };
                write_value(data, *value);
            }
        }
    }
}

/// Default construction for [`Animator`].
impl Default for Animator {
    /// Constructs a default [`Animator`] value.
    fn default() -> Self {
        Self::new()
    }
}
