use super::*;

/// An alert component that displays contextual messages with semantic styling.
///
/// Renders a styled message box using variant-specific classes. The box
/// carries `role="alert"`, so assistive technology announces it when it
/// appears. The message content is passed as children.
///
/// The box is the surface the reference publishes: the title, the
/// description and the action are its parts, so each of them is a
/// component of its own.
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
    let base_class: &Css = match variant {
        AlertVariant::Error => c_error_box(),
        AlertVariant::Success => c_success_box(),
        AlertVariant::Default => c_euv_alert_default(),
        AlertVariant::Destructive => c_euv_alert_destructive(),
    };
    let shadcn_class: Css = {
        match variant {
            AlertVariant::Default => c_euv_alert_default().clone(),
            AlertVariant::Destructive => c_euv_alert_destructive().clone(),
            AlertVariant::Error | AlertVariant::Success => Css::default(),
        }
    };
    html! {
        div {
            role: "alert"
            class: base_class
            class: shadcn_class
            "data-slot": "alert"
            children
        }
    }
}

/// The glyph the alert leads with.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertIconProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv alert icon virtual DOM tree.
#[component]
pub fn euv_alert_icon(node: VirtualNode<EuvAlertIconProps>) -> VirtualNode {
    let EuvAlertIconProps { glyph }: EuvAlertIconProps = node.try_get_props().unwrap_or_default();
    html! {
        span {
            "data-slot": "icon"
            "aria-hidden": "true"
            class: c_euv_alert_icon()
            {
                glyph.clone()
            }
        }
    }
}

/// The title of an alert.
///
/// The title falls back to its own text when the caller gives it no
/// children, so an alert can be titled either way.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertTitleProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The alert title virtual DOM tree.
#[component]
pub fn euv_alert_title(node: VirtualNode<EuvAlertTitleProps>) -> VirtualNode {
    let EuvAlertTitleProps { text }: EuvAlertTitleProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    html! {
        div {
            class: c_euv_alert_title()
            "data-slot": "alert-title"
            if has_children {
                children
            } else {
                text.clone()
            }
        }
    }
}

/// The explanation under the name of an alert.
///
/// The description falls back to its own text when the caller gives it
/// no children, so an alert can be described either way.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertDescriptionProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The alert description virtual DOM tree.
#[component]
pub fn euv_alert_description(node: VirtualNode<EuvAlertDescriptionProps>) -> VirtualNode {
    let EuvAlertDescriptionProps { text }: EuvAlertDescriptionProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let has_children: bool = !matches!(&children, VirtualNode::Empty);
    html! {
        div {
            class: c_euv_alert_description()
            "data-slot": "alert-description"
            if has_children {
                children
            } else {
                text.clone()
            }
        }
    }
}

/// The control that acts on an alert.
///
/// The action sits at the end of the alert, so it takes no settings of
/// its own.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertActionProps>` - The props node carrying the
///   component settings.
///
/// # Returns
///
/// - `VirtualNode` - The alert action virtual DOM tree.
#[component]
pub fn euv_alert_action(node: VirtualNode<EuvAlertActionProps>) -> VirtualNode {
    let EuvAlertActionProps {}: EuvAlertActionProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_alert_action()
            "data-slot": "alert-action"
            children
        }
    }
}
