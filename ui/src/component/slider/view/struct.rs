use super::*;

/// The multiplier converting a `0.0..=1.0` ratio into a `0.0..=100.0`
/// percentage.
const PERCENT_SCALE: f64 = 100.0;

/// Props for the `euv_slider` component.
///
/// Defines the strongly-typed interface for a range slider. The value is
/// owned by the caller through a `Signal<f64>` so the slider can take
/// part in two-way binding with the rest of the page.
#[derive(Clone, CustomDebug, Data, Default)]
pub struct EuvSliderProps {
    /// The unique identifier for the range input element.
    #[get(type(copy))]
    pub id: &'static str,
    /// The HTML name attribute for the range input element.
    #[get(type(copy))]
    pub name: &'static str,
    /// The lowest selectable value.
    #[get(type(copy))]
    pub min: f64,
    /// The highest selectable value.
    #[get(type(copy))]
    pub max: f64,
    /// The granularity of the selectable values.
    #[get(type(copy))]
    pub step: f64,
    /// The signal bound to the current slider value.
    #[get(type(copy))]
    pub value: Signal<f64>,
    /// The label text displayed above the slider track.
    #[get(type(copy))]
    pub label: &'static str,
    /// Optional input event handler; the built-in handler is used when
    /// this is `None`.
    #[debug(skip)]
    pub oninput: Option<Rc<dyn Fn(Event)>>,
}

/// Stateless helper functions backing the `euv_slider` component.
///
/// The slider needs two operations that are pure functions of their
/// arguments — percentage mapping and the value-writing input handler.
/// Grouping them here keeps the `#[component]` body free of arithmetic
/// and event plumbing.
pub struct EuvSliderHelpers;

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
