use super::*;

/// Builds the inline CSS that anchors the popover body to one side of the
/// trigger.
///
/// The offsets match the `c_euv_tooltip_bubble_*` classes so a popover and a
/// tooltip opened from the same trigger sit in the same place. Kept inline
/// because the body is positioned by a single `style` attribute rather than
/// by four more always-injected classes.
///
/// # Arguments
///
/// - `EuvTooltipPlacement` - The side of the trigger to anchor the body to.
///
/// # Returns
///
/// - `String` - The `style` attribute value.
fn popover_body_style(placement: EuvTooltipPlacement) -> String {
    match placement {
        EuvTooltipPlacement::Top => {
            String::from("bottom: calc(100% + 8px); left: 50%; transform: translateX(-50%);")
        }
        EuvTooltipPlacement::Bottom => {
            String::from("top: calc(100% + 8px); left: 50%; transform: translateX(-50%);")
        }
        EuvTooltipPlacement::Left => {
            String::from("top: 50%; right: calc(100% + 8px); transform: translateY(-50%);")
        }
        EuvTooltipPlacement::Right => {
            String::from("top: 50%; left: calc(100% + 8px); transform: translateY(-50%);")
        }
    }
}

/// A click-toggled rich panel anchored to a caller-owned trigger.
///
/// The trigger lives outside the popover — the caller renders it and points it
/// at [`on_popover_toggle`] — so a page can keep its own button markup while
/// still sharing the open signal with the panel. Mirroring [`euv_drawer`],
/// the panel body is always mounted and its visibility is driven by reactive
/// classes: `c_euv_popover_body_open` while the signal is set,
/// `c_euv_popover_body_closed` otherwise. The open / closed pair carries the
/// border, opacity and visibility, so a single class swap is the whole
/// animation and the closed panel never enters the hit-test path.
///
/// The wrapper only positions the body relatively, so the trigger and the
/// panel share one stacking context and the panel is never clipped by an
/// ancestor with `overflow: hidden`.
///
/// # Arguments
///
/// - `VirtualNode<EuvPopoverProps>` - The props node containing the open
///   signal, the panel title and the panel placement.
///
/// # Returns
///
/// - `VirtualNode` - The popover wrapper and its always-mounted panel body.
#[component]
pub fn euv_popover(node: VirtualNode<EuvPopoverProps>) -> VirtualNode {
    let EuvPopoverProps {
        open,
        title,
        placement,
    }: EuvPopoverProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let body_style: String = popover_body_style(placement);
    let body_style_ref: &str = body_style.as_str();
    html! {
        div {
            class: c_euv_popover()
            div {
                class: c_euv_popover_body()
                class: if { open } {
                    c_euv_popover_body_open()
                } else {
                    c_euv_popover_body_closed()
                }
                style: body_style_ref
                div {
                    class: c_euv_popover_header()
                    h3 {
                        class: c_euv_popover_title()
                        title
                    }
                }
                children
            }
        }
    }
}

/// Builds the click handler that toggles the popover open signal.
///
/// Attach it to the caller's own trigger element — the popover renders only
/// the panel, so the trigger stays under the caller's control.
///
/// # Arguments
///
/// - `Signal<bool>` - The caller-owned open state signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle click handler.
pub fn on_popover_toggle(open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let is_open: bool = open.get();
        open.set(!is_open);
    }))
}
