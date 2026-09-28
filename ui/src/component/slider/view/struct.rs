use super::*;

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
