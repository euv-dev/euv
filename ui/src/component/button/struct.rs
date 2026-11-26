use super::*;

/// Cross-platform interactive button component styled exclusively via `ThemeTokens`.
#[derive(Clone, CustomDebug, Data, New)]
pub struct EuvButton {
    /// Visible label string.
    pub(crate) label: String,
    /// Visual emphasis variant.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) variant: EuvButtonVariant,
    /// Control size preset.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) size: EuvButtonSize,
    /// Whether interaction is disabled.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) disabled: bool,
    /// Active theme token bundle.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Optional click event handler.
    #[new(skip)]
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_click: Option<UiEventHandler>,
}
