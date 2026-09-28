use super::*;

impl EuvSliderHelpers {
    /// Maps `value` onto the `0.0..=100.0` percentage range spanned by
    /// `min..=max`.
    ///
    /// A degenerate range (`min == max`) reports `0.0` instead of
    /// dividing by zero, and values outside the range are clamped so a
    /// mis-typed signal can never produce a nonsensical percentage.
    ///
    /// # Arguments
    ///
    /// - `f64` - The value to map.
    /// - `f64` - The low end of the range.
    /// - `f64` - The high end of the range.
    ///
    /// # Returns
    ///
    /// - `f64` - The percentage position of `value` within the range.
    pub fn percent(value: f64, min: f64, max: f64) -> f64 {
        if max <= min {
            return 0.0;
        }
        let ratio: f64 = (value - min) / (max - min);
        let clamped: f64 = ratio.clamp(0.0, 1.0);
        clamped * PERCENT_SCALE
    }

    /// Builds an input handler that writes the range value into `signal`.
    ///
    /// The parsed value is clamped to the `[min, max]` window so a
    /// keyboard-driven overshoot cannot push the signal outside the
    /// range the component advertises.
    ///
    /// # Arguments
    ///
    /// - `Signal<f64>` - The signal updated with the new value.
    /// - `f64` - The low end of the accepted range.
    /// - `f64` - The high end of the accepted range.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - An input handler writing the
    ///   clamped range value into the signal.
    pub fn on_slider_input(signal: Signal<f64>, min: f64, max: f64) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            let raw: Option<f64> = event.target().and_then(|target: EventTarget| {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    return input.value().parse::<f64>().ok();
                }
                None
            });
            if let Some(parsed) = raw {
                signal.set(parsed.clamp(min, max));
            }
        }))
    }
}
