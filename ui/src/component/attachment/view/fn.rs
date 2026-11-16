use super::*;

/// Carries one uploaded file.
///
/// An attachment is a chip that holds a thumbnail, the name of the file,
/// a description of it, and the actions that go with it. The state says
/// how far along the file is: an empty slot draws a dashed outline, an
/// upload in flight dims the chip, and a failure says so in the
/// destructive colour.
///
/// # Arguments
///
/// - `VirtualNode<EuvAttachmentProps>` - The props node carrying the
///   state, the size and the orientation of the chip. The parts of the
///   chip are passed as children.
///
/// # Returns
///
/// - `VirtualNode` - The euv attachment virtual DOM tree.
#[component]
pub fn euv_attachment(node: VirtualNode<EuvAttachmentProps>) -> VirtualNode {
    let EuvAttachmentProps {
        state,
        size,
        orientation,
    }: EuvAttachmentProps = node.try_get_props().unwrap_or_default();
    let state_attr: &str = match state {
        EuvAttachmentState::Idle => "idle",
        EuvAttachmentState::Uploading => "uploading",
        EuvAttachmentState::Processing => "processing",
        EuvAttachmentState::Error => "error",
        EuvAttachmentState::Done => "done",
    };
    let state_class: fn() -> &'static Css = match state {
        EuvAttachmentState::Idle => c_euv_attachment_state_idle,
        EuvAttachmentState::Uploading => c_euv_attachment_state_uploading,
        EuvAttachmentState::Processing => c_euv_attachment_state_processing,
        EuvAttachmentState::Error => c_euv_attachment_state_error,
        EuvAttachmentState::Done => c_euv_attachment_state_done,
    };
    let size_attr: &str = match size {
        EuvAttachmentSize::Default => "default",
        EuvAttachmentSize::Sm => "sm",
        EuvAttachmentSize::Xs => "xs",
    };
    let orientation_attr: &str = match orientation {
        EuvAttachmentOrientation::Horizontal => "horizontal",
        EuvAttachmentOrientation::Vertical => "vertical",
    };
    let orientation_class: fn() -> &'static Css = match orientation {
        EuvAttachmentOrientation::Horizontal => c_euv_attachment_horizontal,
        EuvAttachmentOrientation::Vertical => c_euv_attachment_vertical,
    };
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_attachment();

    html! {
            div {
                "data-slot": "attachment"
                "data-state": state_attr
                "data-size": size_attr
                "data-orientation": orientation_attr
                class: base_class

                class: orientation_class()
                class: state_class()
                children
            }
    }
}

/// Holds the thumbnail that leads an attachment.
///
/// The thumbnail is either an icon standing for the file or the image
/// itself. An icon that belongs to a failed attachment takes the
/// destructive colour, because the icon is the first thing the reader
/// sees.
///
/// # Arguments
///
/// - `VirtualNode<EuvAttachmentMediaProps>` - The props node carrying the
///   kind of media and the size of the thumbnail.
///
/// # Returns
///
/// - `VirtualNode` - The euv attachment media virtual DOM tree.
#[component]
pub fn euv_attachment_media(node: VirtualNode<EuvAttachmentMediaProps>) -> VirtualNode {
    let EuvAttachmentMediaProps { variant, size }: EuvAttachmentMediaProps =
        node.try_get_props().unwrap_or_default();
    let variant_attr: &str = match variant {
        EuvAttachmentMediaVariant::Icon => "icon",
        EuvAttachmentMediaVariant::Image => "image",
    };
    let base_class: &Css = c_euv_attachment_media();
    let variant_class: fn() -> &'static Css = match variant {
        EuvAttachmentMediaVariant::Icon => c_euv_attachment_media,
        EuvAttachmentMediaVariant::Image => c_euv_attachment_media_variant_image,
    };
    let size_class: fn() -> &'static Css = match size {
        EuvAttachmentSize::Default => c_euv_attachment_media_size_default,
        EuvAttachmentSize::Sm => c_euv_attachment_media_size_sm,
        EuvAttachmentSize::Xs => c_euv_attachment_media_size_xs,
    };
    let children: VirtualNode = node.get_children().into();

    html! {
        div {
            "data-slot": "attachment-media"
            "data-variant": variant_attr
            class: base_class
            class: variant_class()

            class: size_class()
            children
        }
    }
}

/// Holds the text of an attachment.
///
/// The content takes the room the chip has left, so the actions beside
/// it are never pushed off the end.
///
/// # Arguments
///
/// - `VirtualNode<EuvAttachmentContentProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv attachment content virtual DOM tree.
#[component]
pub fn euv_attachment_content(node: VirtualNode<EuvAttachmentContentProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "attachment-content"
            class: c_euv_attachment_content()
            children
        }
    }
}

/// Holds the name of the attached file.
///
/// The name is cut short with an ellipsis when it is too long for the
/// chip, because a file name that wraps would push the actions out of
/// the row.
///
/// # Arguments
///
/// - `VirtualNode<EuvAttachmentTitleProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv attachment title virtual DOM tree.
#[component]
pub fn euv_attachment_title(node: VirtualNode<EuvAttachmentTitleProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            "data-slot": "attachment-title"
            class: c_euv_attachment_title()
            children
        }
    }
}

/// Holds the description of the attached file.
///
/// The description is the line under the name: the size of the file, the
/// kind of file, or whatever else the reader needs to know about it.
///
/// # Arguments
///
/// - `VirtualNode<EuvAttachmentDescriptionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv attachment description virtual DOM tree.
#[component]
pub fn euv_attachment_description(node: VirtualNode<EuvAttachmentDescriptionProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            "data-slot": "attachment-description"
            class: c_euv_attachment_description()
            children
        }
    }
}

/// Holds the actions that belong to an attachment.
///
/// The actions sit at the end of the chip and never take a share of the
/// row the text needs.
///
/// # Arguments
///
/// - `VirtualNode<EuvAttachmentActionsProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv attachment actions virtual DOM tree.
#[component]
pub fn euv_attachment_actions(node: VirtualNode<EuvAttachmentActionsProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "attachment-actions"
            class: c_euv_attachment_actions()
            children
        }
    }
}

/// Holds one action that belongs to an attachment.
///
/// The action is a button that sits inside the actions row.
///
/// # Arguments
///
/// - `VirtualNode<EuvAttachmentActionProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv attachment action virtual DOM tree.
#[component]
pub fn euv_attachment_action(node: VirtualNode<EuvAttachmentActionProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            "data-slot": "attachment-action"
            type: "button"
            class: c_euv_attachment_action()
            children
        }
    }
}

/// Covers an attachment so that a click anywhere on it opens the file.
///
/// The trigger lies over the chip, but it is declared before the actions
/// in the DOM, so the actions stay reachable on top of it.
///
/// # Arguments
///
/// - `VirtualNode<EuvAttachmentTriggerProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv attachment trigger virtual DOM tree.
#[component]
pub fn euv_attachment_trigger(node: VirtualNode<EuvAttachmentTriggerProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        button {
            "data-slot": "attachment-trigger"
            type: "button"
            class: c_euv_attachment_trigger()
            children
        }
    }
}

/// Holds a row of attachments that scrolls sideways.
///
/// The row scrolls along one axis and snaps each attachment to its
/// start, so a chip never rests half way across the boundary.
///
/// # Arguments
///
/// - `VirtualNode<EuvAttachmentGroupProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv attachment group virtual DOM tree.
#[component]
pub fn euv_attachment_group(node: VirtualNode<EuvAttachmentGroupProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();

    html! {
            div {
                "data-slot": "attachment-group"
                class: c_euv_attachment_group()

                children
            }
    }
}
