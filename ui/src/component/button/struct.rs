use super::*;

/// Cross-platform interactive button component styled exclusively via `ThemeTokens`.
#[derive(Clone, CustomDebug, Data, New, Props)]
pub struct EuvButton {
    /// Visible label string.
    pub(crate) label: String,
    /// Visual emphasis variant.
    #[new(skip)]
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) variant: EuvButtonVariant,
    /// Control size preset.
    #[new(skip)]
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) size: EuvButtonSize,
    /// Whether interaction is disabled.
    #[new(skip)]
    #[get(type(copy))]
    #[prop(custom)]
    pub(crate) disabled: bool,
    /// Active theme token bundle.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Optional click event handler.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    #[prop(custom)]
    pub(crate) on_click: Option<UiEventHandler>,
}
