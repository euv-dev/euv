use super::*;

/// Separates one block of content from the next.
///
/// A marker is a row that holds an optional icon and the text beside it.
/// The variant decides how the row reads: the plain marker sits in the
/// flow, the separator is a centered rule, and the border draws the rule
/// above the content it introduces.
///
/// # Arguments
///
/// - `VirtualNode<EuvMarkerProps>` - The props node carrying the variant
///   of the marker.
///
/// # Returns
///
/// - `VirtualNode` - The euv marker virtual DOM tree.
#[component]
pub fn euv_marker(node: VirtualNode<EuvMarkerProps>) -> VirtualNode {
    let EuvMarkerProps { variant }: EuvMarkerProps = node.try_get_props().unwrap_or_default();
    let variant_attr: &str = match variant {
        EuvMarkerVariant::Default => "default",
        EuvMarkerVariant::Separator => "separator",
        EuvMarkerVariant::Border => "border",
    };
    let variant_class: fn() -> &'static Css = match variant {
        EuvMarkerVariant::Default => c_euv_marker_default,
        EuvMarkerVariant::Separator => c_euv_marker_separator,
        EuvMarkerVariant::Border => c_euv_marker_border,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "marker"
            "data-variant": variant_attr
            class: c_euv_marker()
            class: variant_class()
            children
        }
    }
}

/// Holds the icon that leads a marker.
///
/// The icon is decorative: the marker's own text already says what the
/// marker is, so a screen reader is not told about the icon twice.
///
/// # Arguments
///
/// - `VirtualNode<EuvMarkerIconProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv marker icon virtual DOM tree.
#[component]
pub fn euv_marker_icon(node: VirtualNode<EuvMarkerIconProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            "data-slot": "marker-icon"
            "aria-hidden": "true"
            class: c_euv_marker_icon()
            children
        }
    }
}

/// Holds the text of a marker.
///
/// The content may be long, so it wraps rather than pushing the icon out
/// of the row.
///
/// # Arguments
///
/// - `VirtualNode<EuvMarkerContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv marker content virtual DOM tree.
#[component]
pub fn euv_marker_content(node: VirtualNode<EuvMarkerContentProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            "data-slot": "marker-content"
            class: c_euv_marker_content()
            children
        }
    }
}
