use super::*;

/// An empty state, for a region that has nothing to show.
///
/// Renders a `c_euv_empty` block holding an optional `c_euv_empty_icon`
/// glyph, the `c_euv_empty_text` headline, an optional
/// `c_euv_empty_desc` explanation, and an optional action.
///
/// The action is a real `euv_button` in the `Primary` variant — the
/// project standard forbids hand-rolled buttons — and it only appears
/// when `action_label` is not empty.
///
/// # Arguments
///
/// - `VirtualNode<EuvEmptyProps>` - The props node containing the empty
///   state configuration.
///
/// # Returns
///
/// - `VirtualNode` - The empty state virtual DOM tree.
#[component]
pub fn euv_empty(node: VirtualNode<EuvEmptyProps>) -> VirtualNode {
    let EuvEmptyProps {
        icon,
        title,
        desc,
        action_label,
        on_action,
    }: EuvEmptyProps = node.try_get_props().unwrap_or_default();
    let has_action: bool = !action_label.is_empty();
    html! {
        div {
            class: c_euv_empty()
            if !icon.is_empty() {
                div {
                    class: c_euv_empty_icon()
                    icon
                }
            }
            div {
                class: c_euv_empty_text()
                title
            }
            if !desc.is_empty() {
                div {
                    class: c_euv_empty_desc()
                    desc
                }
            }
            if has_action {
                div {
                    class: c_euv_empty_action()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: action_label
                        onclick: on_action
                    }
                }
            }
        }
    }
}
