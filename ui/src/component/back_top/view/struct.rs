use super::*;

/// Props for the [`euv_back_top`] component.
///
/// Defines the strongly-typed interface for a back-to-top button: when
/// it is on screen, what it says, and what happens when it is taken.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvBackTopProps {
    /// Whether the button is on screen.
    ///
    /// The caller owns the visibility — it is usually driven by the
    /// page's scroll offset, through `use_scroll`.
    pub visible: Signal<bool>,
    /// The label on the button.
    pub label: &'static str,
    /// The callback fired when the button is taken.
    #[debug(skip)]
    pub on_click: Option<Rc<dyn Fn(Event)>>,
}
