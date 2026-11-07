use super::*;

/// Props for the `euv_input_group` component.
///
/// The group is the frame the control and its addons share: one border,
/// one focus ring, one surface.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvInputGroupProps {
    /// Whether the whole group is disabled.
    #[get(type(copy))]
    pub disabled: Signal<bool>,
}

/// Props for the `euv_input_group_addon` component.
///
/// An addon is the icon, the prefix or the suffix that sits beside the
/// control.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvInputGroupAddonProps {
    /// Where the addon sits relative to the control.
    #[get(type(copy))]
    pub align: EuvInputGroupAlign,
}

/// Props for the `euv_input_group_button` component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvInputGroupButtonProps {
    /// The size of the button.
    #[get(type(copy))]
    pub size: EuvInputGroupButtonSize,
    /// The label of the button.
    #[get(type(copy))]
    pub label: &'static str,
    /// Optional click event handler.
    #[debug(skip)]
    pub onclick: Option<Rc<dyn Fn(Event)>>,
}

/// Props for the `euv_input_group_text` component.
///
/// The text is the quiet caption that belongs to the field rather than
/// an action of its own.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvInputGroupTextProps {
    /// The text shown inside the group.
    #[get(type(copy))]
    pub text: &'static str,
}

/// Props for the `euv_input_group_input` component.
///
/// The control is the single-line field of the group; the group draws
/// the frame around it, so the field itself carries no border.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvInputGroupInputProps {
    /// The unique identifier of the input element.
    #[get(type(copy))]
    pub id: &'static str,
    /// The placeholder text shown when the field is empty.
    #[get(type(copy))]
    pub placeholder: &'static str,
    /// The current text (reactive signal).
    pub value: Signal<String>,
    /// Whether the field is disabled.
    #[get(type(copy))]
    pub disabled: Signal<bool>,
    /// Optional input event handler.
    #[debug(skip)]
    pub oninput: Option<Rc<dyn Fn(Event)>>,
}

/// Props for the `euv_input_group_textarea` component.
///
/// The control is the multi-line field of the group.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvInputGroupTextareaProps {
    /// The unique identifier of the textarea element.
    #[get(type(copy))]
    pub id: &'static str,
    /// The placeholder text shown when the field is empty.
    #[get(type(copy))]
    pub placeholder: &'static str,
    /// The current text (reactive signal).
    pub value: Signal<String>,
    /// The visible height of the field in rows. Zero means the default.
    #[get(type(copy))]
    pub rows: i32,
    /// Whether the field is disabled.
    #[get(type(copy))]
    pub disabled: Signal<bool>,
    /// Optional input event handler.
    #[debug(skip)]
    pub oninput: Option<Rc<dyn Fn(Event)>>,
}
