use super::*;

/// Default 60Hz fixed-timestep `FrameClock` with 8-substep safety cap.
impl Default for FrameClock {
    /// Creates a `FrameClock` configured for 60Hz fixed simulation steps.
    ///
    /// # Returns
    ///
    /// - `Self` - Default 60Hz game clock.
    fn default() -> Self {
        Self::new(1.0 / 60.0, 8)
    }
}

/// Simulation step scheduling and frame telemetry updates for `FrameClock`.
impl FrameClock {
    /// Advances the clock by `raw_dt_secs` and returns how many fixed simulation steps to execute.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable frame clock.
    /// - `f32` - Unclamped elapsed seconds since the previous frame.
    ///
    /// # Returns
    ///
    /// - `u32` - Number of fixed-timestep updates to run this frame.
    pub fn tick(&mut self, raw_dt_secs: f32) -> u32 {
        let safe_fixed: f32 = self.get_fixed_dt().max(0.000_1);
        let dt: f32 = if raw_dt_secs.is_finite() {
            raw_dt_secs.clamp(0.0, 0.25)
        } else {
            0.0
        };
        self.set_delta_secs(dt);
        self.set_elapsed_secs(self.get_elapsed_secs() + (dt as f64));
        self.set_frame_count(self.get_frame_count().saturating_add(1));
        if dt > 0.0 {
            let inst_fps: f32 = 1.0 / dt;
            let prev_fps: f32 = self.get_fps();
            let smoothed: f32 = if prev_fps <= 0.0 {
                inst_fps
            } else {
                prev_fps * 0.85 + inst_fps * 0.15
            };
            self.set_fps(smoothed);
        }
        let mut acc: f32 = self.get_accumulator() + dt;
        let max_steps: u32 = self.get_max_substeps().max(1);
        let mut steps: u32 = 0;
        while acc >= safe_fixed && steps < max_steps {
            acc -= safe_fixed;
            steps += 1;
        }
        if steps == max_steps && acc >= safe_fixed {
            acc = 0.0;
        }
        self.set_accumulator(acc);
        self.set_alpha((acc / safe_fixed).clamp(0.0, 1.0));
        self.set_fixed_step_count(self.get_fixed_step_count().saturating_add(steps as u64));
        steps
    }
}
