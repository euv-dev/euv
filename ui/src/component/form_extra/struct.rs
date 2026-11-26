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

/// Numeric stepper input bound to `Signal<i64>` with optional min/max bounds and step size.
#[derive(Clone, Data, New)]
pub struct EuvNumberInput {
    /// Bound integer value signal.
    #[get(type(copy))]
    pub(crate) value: Signal<i64>,
    /// Optional minimum allowed value (`None` defaults to `i64::MIN / 2`).
    #[new(skip)]
    #[get(skip)]
    pub(crate) min: Option<i64>,
    /// Optional maximum allowed value (`None` defaults to `i64::MAX / 2`).
    #[new(skip)]
    #[get(skip)]
    pub(crate) max: Option<i64>,
    /// Increment step size (`<= 0` defaults to `1`).
    #[new(skip)]
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

/// Interactive swatch palette color selector bound to `Signal<Color>`.
#[derive(Clone, Data, New)]
pub struct EuvColorPicker {
    /// Bound reactive selected color signal.
    #[get(type(copy))]
    pub(crate) value: Signal<Color>,
    /// Optional custom swatch presets (empty falls back to the built-in curated palette).
    #[new(skip)]
    pub(crate) presets: Vec<Color>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive 24-hour `HH:MM` time picker with steppers and quick presets bound to `Signal<(u8, u8)>`.
#[derive(Clone, Data, New)]
pub struct EuvTimePicker {
    /// Bound reactive `(hour, minute)` tuple signal (`hour` in `0..=23`, `minute` in `0..=59`).
    #[get(type(copy))]
    pub(crate) value: Signal<(u8, u8)>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive multi-tag chip input supporting live tag addition and removal bound to `Signal<Vec<String>>`.
#[derive(Clone, Data, New)]
pub struct EuvTagInput {
    /// Bound reactive list of active tag strings.
    #[get(type(copy))]
    pub(crate) tags: Signal<Vec<String>>,
    /// Input field placeholder text.
    #[new(skip)]
    pub(crate) placeholder: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
