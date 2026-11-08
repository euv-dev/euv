use super::*;

/// Props for the [`euv_input_otp`] component.
///
/// The field owns a single input and draws one slot per character. The
/// text stays in the caller's signal, so the field takes part in the
/// two-way binding of the page it lives in.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvInputOtpProps {
    /// The text the field is showing.
    pub value: Signal<String>,
    /// The number of slots the field draws.
    pub length: usize,
    /// Whether the field is disabled.
    pub disabled: bool,
    /// The accessible name of the field.
    pub label: &'static str,
}

/// Props of the [`euv_input_otp_group`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvInputOtpGroupProps {
    /// The accessible name of the group.
    pub label: &'static str,
}

/// Props of the [`euv_input_otp_slot`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvInputOtpSlotProps {
    /// The character drawn in the slot.
    pub char: &'static str,
    /// Whether the slot is the one the caret is in.
    pub active: bool,
}
