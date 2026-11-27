use super::*;

/// Cross-platform reactive text input field with placeholder, masking, and change callback.
#[derive(Clone, CustomDebug, Data, New, Props)]
pub struct EuvInput {
    /// Reactive bound value signal.
    #[get(type(copy))]
    pub(crate) value: Signal<String>,
    /// Placeholder text.
    pub(crate) placeholder: String,
    /// Password masking flag.
    #[new(skip)]
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) password: bool,
    /// Disabled flag.
    #[new(skip)]
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) disabled: bool,
    /// Removes outer border and background for seamless embedding inside `EuvInputGroup`.
    #[new(skip)]
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) borderless: bool,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Labeled form field wrapper combining label, control node, and optional error caption.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvField {
    /// Field label.
    pub(crate) label: String,
    /// Optional validation error message.
    #[new(skip)]
    #[prop(custom)]
    pub(crate) error: String,
    /// Mounted input/select control node.
    #[get(type(copy))]
    pub(crate) control: NodeId,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Cross-platform reactive boolean toggle switch.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvSwitch {
    /// Reactive checked state signal.
    #[get(type(copy))]
    pub(crate) checked: Signal<bool>,
    /// Label text displayed alongside the toggle.
    pub(crate) label: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Cross-platform reactive checkbox control.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvCheckbox {
    /// Reactive checked state signal.
    #[get(type(copy))]
    pub(crate) checked: Signal<bool>,
    /// Label text displayed next to the checkbox.
    pub(crate) label: String,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Cross-platform segmented option selector.
#[derive(Clone, Data, Debug, New, Props)]
pub struct EuvSegmented {
    /// Ordered option labels.
    pub(crate) options: Vec<String>,
    /// Reactive active index signal.
    #[get(type(copy))]
    pub(crate) selected: Signal<usize>,
    /// Active theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
