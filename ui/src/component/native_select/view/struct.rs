use super::*;

/// Props for the [`euv_native_select`] component.
///
/// The select carries the size of the control and the state a form needs
/// to read back. The options themselves are passed as children.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNativeSelectProps {
    /// The size of the control.
    pub size: EuvNativeSelectSize,
    /// The form name the control submits under.
    pub name: &'static str,
    /// Whether the control refuses input.
    pub disabled: bool,
    /// The value the control currently shows.
    ///
    /// An empty value leaves the control to the choice of the browser, and
    /// the first option is the one it shows. A value the options do not
    /// carry leaves the control as it is.
    pub value: Signal<String>,
    /// The callback that receives the value the reader chose.
    ///
    /// The control is the native one, so the change arrives as an event on
    /// the control itself and the value is read from it.
    #[debug(skip)]
    pub onchange: Option<Rc<dyn Fn(Event)>>,
}

/// Props for the [`euv_native_select_option`] component.
///
/// The option carries the value the form submits and the label the
/// reader sees.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNativeSelectOptionProps {
    /// The value the option submits under.
    pub value: &'static str,
    /// The label the reader sees.
    pub label: &'static str,
    /// Whether the option is excluded from selection.
    pub disabled: bool,
}

/// Props for the [`euv_native_select_opt_group`] component.
///
/// The group carries the label that names the options it wraps. The
/// options themselves are passed as children.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNativeSelectOptGroupProps {
    /// The label that names the group.
    pub label: &'static str,
    /// Whether every option in the group is excluded from selection.
    pub disabled: bool,
}
