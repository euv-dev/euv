use super::*;

/// Props for the `euv_textarea` component.
///
/// A textarea is the multi-line sibling of the single-line input: the
/// value lives in a caller-owned signal so the surrounding form can read
/// the whole field state back.
#[derive(Clone, CustomDebug, Data, Default)]
pub struct EuvTextareaProps {
    /// The unique identifier of the textarea element.
    #[get(type(copy))]
    pub id: &'static str,
    /// The HTML name attribute for the textarea element.
    #[get(type(copy))]
    pub name: &'static str,
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
    /// The accessible name when no visible label is present.
    pub aria_label: &'static str,
    /// Optional keyboard handler for editor shortcuts.
    #[debug(skip)]
    pub onkeydown: Option<Rc<dyn Fn(Event)>>,
    /// An optional application class composed with the design-system textarea class.
    pub class: Css,
}
