use super::*;

/// Props of the [`euv_sheet`] component.
///
/// The sheet holds the open state, so the trigger and the panel always
/// agree on whether the sheet is showing.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSheetProps {
    /// Whether the sheet is open.
    pub open: Signal<bool>,
    /// The edge the sheet slides in from.
    pub side: EuvSheetSide,
}

/// Props of the [`euv_sheet_trigger`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSheetTriggerProps {
    /// Whether the sheet is open.
    pub open: Signal<bool>,
    /// The label shown inside the trigger.
    pub label: &'static str,
}

/// Props of the [`euv_sheet_overlay`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSheetOverlayProps {
    /// Whether the sheet is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_sheet_content`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSheetContentProps {
    /// Whether the sheet is open.
    pub open: Signal<bool>,
    /// The edge the sheet slides in from.
    pub side: EuvSheetSide,
}

/// Props of the [`euv_sheet_header`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvSheetHeaderProps {
    /// The accessible name of the header.
    pub label: &'static str,
}

/// Props of the [`euv_sheet_footer`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvSheetFooterProps {
    /// The accessible name of the footer.
    pub label: &'static str,
}

/// Props of the [`euv_sheet_title`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvSheetTitleProps {
    /// The title text.
    pub text: &'static str,
}

/// Props of the [`euv_sheet_description`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvSheetDescriptionProps {
    /// The description text.
    pub text: &'static str,
}

/// Props of the [`euv_sheet_close`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvSheetCloseProps {
    /// The accessible name of the button.
    pub label: &'static str,
}
