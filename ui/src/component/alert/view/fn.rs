use super::*;

/// An alert component that displays contextual messages with semantic styling.
///
/// Renders a styled message box using variant-specific classes. The box
/// carries `role="alert"`, so assistive technology announces it when it
/// appears. The message content is passed as children.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertProps>` - The props node containing the variant.
///
/// # Returns
///
/// - `VirtualNode` - A styled alert element.
#[component]
pub fn euv_alert(node: VirtualNode<EuvAlertProps>) -> VirtualNode {
    let EuvAlertProps { variant }: EuvAlertProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    match variant {
        AlertVariant::Error => html! {
            div {
                role: "alert"
                class: c_error_box()
                children
            }
        },
        AlertVariant::Success => html! {
            div {
                role: "alert"
                class: c_success_box()
                children
            }
        },
        AlertVariant::Default => html! {
            div {
                role: "alert"
                class: c_euv_alert_default()
                children
            }
        },
        AlertVariant::Destructive => html! {
            div {
                role: "alert"
                class: c_euv_alert_destructive()
                children
            }
        },
    }
}
