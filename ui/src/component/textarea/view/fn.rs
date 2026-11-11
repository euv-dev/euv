use super::*;

/// A multi-line text field.
///
/// The element is a `<textarea>`: the browser gives it its own keyboard,
/// its own selection model and its own scrolling box, so a long answer is
/// written in place instead of in a single-line input that scrolls
/// sideways.
///
/// The field carries the reference sizing — a minimum height of four text
/// lines — and the placeholder reads in the muted tone.
///
/// # Arguments
///
/// - `VirtualNode<EuvTextareaProps>` - The props node carrying the id,
///   the value signal and the disabled state.
///
/// # Returns
///
/// - `VirtualNode` - The textarea virtual DOM tree.
#[component]
pub fn euv_textarea(node: VirtualNode<EuvTextareaProps>) -> VirtualNode {
    let EuvTextareaProps {
        id,
        name,
        placeholder,
        value,
        rows,
        disabled,
        oninput,
    }: EuvTextareaProps = node.try_get_props().unwrap_or_default();
    let effective_name: &'static str = if name.is_empty() { id } else { name };
    let default_rows: i32 = if cfg!(feature = "theme-shadcn") { 2 } else { 3 };
    let effective_rows: i32 = if rows > 0 { rows } else { default_rows };
    let is_disabled: bool = disabled.get();
    let base_class: &Css = c_euv_textarea();
    let shadcn_class: Css = if cfg!(feature = "theme-shadcn") {
        c_euv_textarea_shadcn().clone()
    } else {
        Css::default()
    };
    html! {
        textarea {
            id: id
            name: effective_name
            "data-slot": "textarea"
            rows: effective_rows
            placeholder: placeholder
            value: value
            class: base_class
            class: shadcn_class
            disabled: is_disabled
            oninput: oninput
        }
    }
}
