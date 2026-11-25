use super::*;

/// Accessible form control label.
#[derive(Clone, Data, New)]
pub struct EuvLabel {
    /// Label text.
    pub(crate) text: String,
    /// Displays a destructive required indicator when `true`.
    #[get(type(copy))]
    pub(crate) required: bool,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Structured form wrapper stacking labeled fields with uniform vertical spacing.
#[derive(Clone, Data, New)]
pub struct EuvForm {
    /// Ordered form field nodes.
    #[new(skip)]
    pub(crate) fields: Vec<NodeId>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Multi-line text input surface bound to `Signal<String>`.
#[derive(Clone, Data, New)]
pub struct EuvTextarea {
    /// Bound string signal.
    #[get(type(copy))]
    pub(crate) value: Signal<String>,
    /// Placeholder text.
    pub(crate) placeholder: String,
    /// Visible height in logical pixels.
    #[get(type(copy))]
    pub(crate) height: f32,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Numeric stepper input bound to `Signal<i64>` with min/max bounds.
#[derive(Clone, Data, New)]
pub struct EuvNumberInput {
    /// Bound integer value signal.
    #[get(type(copy))]
    pub(crate) value: Signal<i64>,
    /// Minimum allowed value.
    #[get(type(copy))]
    pub(crate) min: i64,
    /// Maximum allowed value.
    #[get(type(copy))]
    pub(crate) max: i64,
    /// Increment step size.
    #[get(type(copy))]
    pub(crate) step: i64,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Mutually exclusive radio button group bound to `Signal<usize>`.
#[derive(Clone, Data, New)]
pub struct EuvRadioGroup {
    /// Option labels.
    pub(crate) options: Vec<String>,
    /// Selected option index signal.
    #[get(type(copy))]
    pub(crate) selected: Signal<usize>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Continuous range slider bound to `Signal<f32>` in `[min, max]`.
#[derive(Clone, Data, New)]
pub struct EuvSlider {
    /// Reactive numeric value signal.
    #[get(type(copy))]
    pub(crate) value: Signal<f32>,
    /// Minimum slider value.
    #[get(type(copy))]
    pub(crate) min: f32,
    /// Maximum slider value.
    #[get(type(copy))]
    pub(crate) max: f32,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Dropdown option selector bound to `Signal<usize>` with reactive menu toggle.
#[derive(Clone, Data, New)]
pub struct EuvSelect {
    /// Ordered selectable option labels.
    pub(crate) options: Vec<String>,
    /// Active selected option index.
    #[get(type(copy))]
    pub(crate) selected: Signal<usize>,
    /// Reactive dropdown open flag.
    #[get(type(copy))]
    pub(crate) open: Signal<bool>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive 1..=N star rating control bound to `Signal<usize>`.
#[derive(Clone, Data, New)]
pub struct EuvRating {
    /// Current rating value (`0..=max_stars`).
    #[get(type(copy))]
    pub(crate) value: Signal<usize>,
    /// Maximum star count.
    #[get(type(copy))]
    pub(crate) max_stars: usize,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
