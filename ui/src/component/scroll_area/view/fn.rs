use super::*;

/// Computes thumb length, offset and scroll range from the track's inner length.
///
/// # Arguments
/// - `f64` - Viewport client height.
/// - `f64` - Full scrollable content height.
/// - `f64` - Track client height, excluding borders.
/// - `f64` - Combined vertical track padding.
/// - `f64` - Current vertical scroll offset.
///
/// # Returns
/// - `(f64, f64, f64)` - Thumb length, thumb offset and scroll range.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn scroll_area_geometry(
    viewport: f64,
    content: f64,
    track: f64,
    padding: f64,
    scroll: f64,
) -> (f64, f64, f64) {
    let available: f64 = (track - padding).max(0.0);
    let range: f64 = (content - viewport).max(0.0);
    let length: f64 = if content > 0.0 {
        (available * viewport / content)
            .max(SCROLL_AREA_MIN_THUMB)
            .min(available)
    } else {
        available
    };
    let offset: f64 = if range > 0.0 {
        (available - length) * scroll.clamp(0.0, range) / range
    } else {
        0.0
    };
    (length, offset, range)
}

/// Maps an inner-track thumb position to its bounded vertical scroll offset.
///
/// # Arguments
/// - `f64` - Desired thumb offset within the inner track.
/// - `f64` - Available thumb travel.
/// - `f64` - Viewport scroll range.
///
/// # Returns
/// - `f64` - Bounded scroll offset.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn scroll_area_drag_offset(position: f64, travel: f64, range: f64) -> f64 {
    if travel > 0.0 {
        position.clamp(0.0, travel) / travel * range
    } else {
        0.0
    }
}

/// Maps a focused viewport's native keyboard command to a bounded scroll offset.
///
/// # Arguments
/// - `&str` - Native keyboard key.
/// - `f64` - Current scroll offset.
/// - `f64` - Viewport client height.
/// - `f64` - Full scroll range.
///
/// # Returns
/// - `Option<f64>` - New scroll offset for supported commands.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn scroll_area_key_offset(
    key: &str,
    scroll: f64,
    viewport: f64,
    range: f64,
) -> Option<f64> {
    let index: usize = SCROLL_AREA_KEYS
        .iter()
        .position(|candidate: &&str| *candidate == key)?;
    let next: f64 = match index {
        0 => 0.0,
        1 => range,
        2 => scroll - viewport,
        3 => scroll + viewport,
        4 => scroll - 40.0,
        _ => scroll + 40.0,
    };
    Some(next.clamp(0.0, range.max(0.0)))
}

/// Installs a controller whose callbacks only weakly reference their hook owner.
///
/// # Arguments
/// - `NodeRef<Element>` - Lifecycle-managed root reference.
#[cfg(target_arch = "wasm32")]
fn use_scroll_area_controller(root: NodeRef<Element>) {
    let controller: Rc<ScrollAreaController> =
        HookContext::use_hook(move || ScrollAreaController::attach(root));
    App::use_cleanup(move || controller.dispose());
}

/// Finds a part only when its nearest root is the immediate owner.
///
/// # Arguments
/// - `&Element` - Owning root.
/// - `&str` - Stable public part selector.
///
/// # Returns
/// - `Option<HtmlElement>` - Immediate owner's mounted part.
#[cfg(target_arch = "wasm32")]
pub(super) fn scroll_area_part(root: &Element, selector: &str) -> Option<HtmlElement> {
    let nodes: NodeList = root.query_selector_all(selector).ok()?;
    (0..nodes.length())
        .filter_map(|index: u32| nodes.item(index))
        .filter_map(|node: Node| node.dyn_into::<HtmlElement>().ok())
        .find(|part: &HtmlElement| {
            part.closest(SCROLL_AREA_ROOT).ok().flatten().as_ref() == Some(root)
        })
}

/// Reads a CSS length without assuming a caller's track padding.
///
/// # Arguments
/// - `&HtmlElement` - Track element.
/// - `&str` - Computed property name.
///
/// # Returns
/// - `f64` - CSS pixel length, defaulting to zero.
#[cfg(target_arch = "wasm32")]
pub(super) fn scroll_area_length(element: &HtmlElement, property: &str) -> f64 {
    window()
        .and_then(|window: Window| window.get_computed_style(element).ok().flatten())
        .and_then(|style: CssStyleDeclaration| style.get_property_value(property).ok())
        .and_then(|value: String| value.trim_end_matches("px").parse::<f64>().ok())
        .unwrap_or_default()
}

/// Updates a style property only when its value actually changes.
///
/// # Arguments
/// - `&HtmlElement` - Target part.
/// - `&str` - Property name.
/// - `&str` - Property value.
#[cfg(target_arch = "wasm32")]
pub(super) fn scroll_area_style(element: &HtmlElement, property: &str, value: &str) {
    let style: CssStyleDeclaration = element.style();
    if style.get_property_value(property).ok().as_deref() != Some(value) {
        let _: Result<(), JsValue> = style.set_property(property, value);
    }
}

/// A frame around a scrolling viewport.
///
/// The frame hides what overflows it, the viewport inside it scrolls,
/// and the bar drawn along the edge is the reader's handle on that
/// scroll. The three are one component, so a caller never has to wire
/// them together by hand.
///
/// The frame is the `scroll-area` part of the reference and the viewport
/// inside it is the `scroll-area-viewport` part, so the two can be
/// addressed from the DOM the way the reference addresses them.
///
/// # Arguments
///
/// - `VirtualNode<EuvScrollAreaProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv scroll area virtual DOM tree.
#[component]
pub fn euv_scroll_area(node: VirtualNode<EuvScrollAreaProps>) -> VirtualNode {
    let EuvScrollAreaProps { label }: EuvScrollAreaProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let root: NodeRef<Element> = App::use_node_ref();
    #[cfg(target_arch = "wasm32")]
    use_scroll_area_controller(root.clone());
    html! {
        div {
            ref: root
            role: "region"
            "aria-label": label
            "data-slot": "scroll-area"
            class: c_euv_scroll_area()
            euv_scroll_area_viewport {
                children
            }
            euv_scroll_bar {}
        }
    }
}

/// The part of a scroll area that scrolls.
///
/// The viewport is the `scroll-area-viewport` part of the reference: it
/// holds the content and takes the scroll, while the frame around it
/// keeps the content inside its own bounds.
///
/// # Arguments
///
/// - `VirtualNode<EuvScrollAreaProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv scroll area viewport virtual DOM tree.
#[component]
pub fn euv_scroll_area_viewport(node: VirtualNode<EuvScrollAreaProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "scroll-area-viewport"
            tabindex: "0"
            class: c_euv_scroll_area_viewport()
            children
        }
    }
}

/// The handle the reader drags to scroll the viewport.
///
/// # Arguments
///
/// - `VirtualNode<EuvScrollAreaProps>` - The props node carrying the component settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv scroll bar virtual DOM tree.
#[component]
pub fn euv_scroll_bar(node: VirtualNode<EuvScrollAreaProps>) -> VirtualNode {
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "scroll-bar"
            "aria-hidden": "true"
            class: c_euv_scroll_bar()
            div {
                "data-slot": "scroll-bar-thumb"
                class: c_euv_scroll_bar_thumb()
            }
            children
        }
    }
}
