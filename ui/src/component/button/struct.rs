use super::*;

/// Cross-platform interactive button component styled exclusively via `ThemeTokens`.
#[derive(Clone, CustomDebug, Data)]
pub struct EuvButton {
    /// Visible label string.
    pub(crate) label: String,
    /// Visual emphasis variant.
    #[get(type(copy))]
    pub(crate) variant: EuvButtonVariant,
    /// Control size preset.
    #[get(type(copy))]
    pub(crate) size: EuvButtonSize,
    /// Whether interaction is disabled.
    #[get(type(copy))]
    pub(crate) disabled: bool,
    /// Active theme token bundle.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
    /// Optional click event handler.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_click: Option<UiEventHandler>,
}
