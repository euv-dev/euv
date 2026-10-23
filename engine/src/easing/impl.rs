use super::*;

/// Implements easing curve evaluation for `Easing`.
impl Easing {
    /// Evaluates the easing curve at the given normalized time.
    ///
    /// The input is clamped into the range 0.0 to 1.0 before evaluation,
    /// so callers may pass slightly out-of-range accumulators without
    /// producing out-of-range output.
    ///
    /// # Arguments
    ///
    /// - `f64` - The normalized time, typically in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `f64` - The eased value, where 0.0 maps to the start and 1.0 to the end.
    pub fn evaluate(&self, t: f64) -> f64 {
        let t: f64 = Numeric::clamp(t, 0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::InQuad => t * t,
            Easing::OutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::InOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Easing::InCubic => t * t * t,
            Easing::OutCubic => 1.0 - (1.0 - t).powi(3),
            Easing::InOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::InQuart => t * t * t * t,
            Easing::OutQuart => 1.0 - (1.0 - t).powi(4),
            Easing::InOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
                }
            }
            Easing::InQuint => t * t * t * t * t,
            Easing::OutQuint => 1.0 - (1.0 - t).powi(5),
            Easing::InOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
                }
            }
            Easing::InSine => 1.0 - (t * HALF_PI).cos(),
            Easing::OutSine => (t * HALF_PI).sin(),
            Easing::InOutSine => -((t * PI).cos() - 1.0) / 2.0,
            Easing::InExpo => {
                if t == 0.0 {
                    0.0
                } else {
                    2.0_f64.powf(10.0 * t - 10.0)
                }
            }
            Easing::OutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f64.powf(-10.0 * t)
                }
            }
            Easing::InOutExpo => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    2.0_f64.powf(20.0 * t - 10.0) / 2.0
                } else {
                    (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0
                }
            }
            Easing::InCirc => 1.0 - (1.0 - t * t).sqrt(),
            Easing::OutCirc => (1.0 - (t - 1.0).powi(2)).sqrt(),
            Easing::InOutCirc => {
                if t < 0.5 {
                    (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
                } else {
                    ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
                }
            }
            Easing::InBack => {
                let c: f64 = BACK_OVERSHOOT;
                (c + 1.0) * t * t * t - c * t * t
            }
            Easing::OutBack => {
                let c: f64 = BACK_OVERSHOOT;
                1.0 + (c + 1.0) * (t - 1.0).powi(3) + c * (t - 1.0).powi(2)
            }
            Easing::InOutBack => {
                let c: f64 = BACK_OVERSHOOT_INOUT;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((c + 1.0) * 2.0 * t - c)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c + 1.0) * (2.0 * t - 2.0) + c) + 2.0) / 2.0
                }
            }
            Easing::InElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    -2.0_f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * ELASTIC_PERIOD).sin()
                }
            }
            Easing::OutElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * ELASTIC_PERIOD).sin() + 1.0
                }
            }
            Easing::InOutElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    -(2.0_f64.powf(20.0 * t - 10.0)
                        * ((20.0 * t - 11.125) * ELASTIC_PERIOD_INOUT).sin())
                        / 2.0
                } else {
                    (2.0_f64.powf(-20.0 * t + 10.0)
                        * ((20.0 * t - 11.125) * ELASTIC_PERIOD_INOUT).sin())
                        / 2.0
                        + 1.0
                }
            }
            Easing::InBounce => 1.0 - Easing::OutBounce.evaluate(1.0 - t),
            Easing::OutBounce => Easing::bounce_out(t),
            Easing::InOutBounce => {
                if t < 0.5 {
                    (1.0 - Easing::bounce_out(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + Easing::bounce_out(2.0 * t - 1.0)) / 2.0
                }
            }
        }
    }

    /// Applies this easing curve to interpolate between two scalar values.
    ///
    /// # Arguments
    ///
    /// - `f64` - The start value.
    ///
    /// # Returns
    ///
    /// - `f64` - The eased interpolation between `start` and `end`.
    pub fn interpolate(&self, start: f64, end: f64, t: f64) -> f64 {
        Numeric::lerp(start, end, self.evaluate(t))
    }

    /// The shared `OutBounce` curve, factored out so `InBounce` and
    /// `InOutBounce` can mirror it without duplicating the subdivision table.
    ///
    /// # Arguments
    ///
    /// - `f64` - The normalized time in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `f64` - The bounced value.
    fn bounce_out(t: f64) -> f64 {
        let n: f64 = BOUNCE_DIVISIONS;
        if t < 1.0 / n {
            7.5625 * t * t
        } else if t < 2.0 / n {
            let t: f64 = t - 1.5 / n;
            7.5625 * t * t + 0.75
        } else if t < 2.5 / n {
            let t: f64 = t - 2.25 / n;
            7.5625 * t * t + 0.9375
        } else {
            let t: f64 = t - 2.625 / n;
            7.5625 * t * t + 0.984375
        }
    }
}
