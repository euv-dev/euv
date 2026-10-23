use super::*;

/// An image with a placeholder and a failure state.
///
/// Renders the picture in a `c_euv_image` box. While the picture is
/// still loading the box carries `c_euv_image_placeholder` and shows
/// the placeholder text; once it has loaded the box falls back to the
/// plain image class. A picture that fails to load leaves the box on
/// `c_euv_image_failed`, so the region never collapses to a broken
/// icon.
///
/// # Arguments
///
/// - `VirtualNode<EuvImageProps>` - The props node containing the
///   image configuration.
///
/// # Returns
///
/// - `VirtualNode` - The image virtual DOM tree.
#[component]
pub fn euv_image(node: VirtualNode<EuvImageProps>) -> VirtualNode {
    let EuvImageProps {
        src,
        alt,
        placeholder,
        width,
        height,
    }: EuvImageProps = node.try_get_props().unwrap_or_default();
    let status: Signal<EuvImageStatus> =
        HookContext::use_hook(|| Signal::create(EuvImageStatus::Loading));
    let current: EuvImageStatus = status.get();
    let size_style: String = image_size_style(width, height);
    let size_ref: &str = size_style.as_str();
    html! {
        div {
            class: c_euv_image()
            class: match current {
                EuvImageStatus::Loaded => c_euv_image(),
                EuvImageStatus::Failed => c_euv_image_failed(),
                EuvImageStatus::Loading => c_euv_image_placeholder(),
            }
            style: size_ref
            if current != EuvImageStatus::Loaded && !placeholder.is_empty() {
                span {
                    placeholder
                }
            }
            img {
                src: src
                alt: alt
                loading: "lazy"
                onload: on_image_loaded(status)
                onerror: on_image_failed(status)
            }
        }
    }
}

/// Builds the inline size declaration for the picture box.
///
/// The box is sized by the caller when both dimensions are given, and
/// left to the stylesheet otherwise — a picture that only knows its
/// width would otherwise be stretched into a square.
///
/// # Arguments
///
/// - `Option<u32>` - The explicit width in pixels.
///
/// # Returns
///
/// - `String` - The inline style, empty when no size was given.
fn image_size_style(width: Option<u32>, height: Option<u32>) -> String {
    match (width, height) {
        (Some(width), Some(height)) => format!("width: {width}px; height: {height}px;"),
        (Some(width), None) => format!("width: {width}px;"),
        (None, Some(height)) => format!("height: {height}px;"),
        (None, None) => String::new(),
    }
}

/// Builds the handler that marks the picture as loaded.
///
/// # Arguments
///
/// - `Signal<EuvImageStatus>` - The status signal updated on load.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The load handler.
fn on_image_loaded(status: Signal<EuvImageStatus>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        status.set(EuvImageStatus::Loaded);
    }))
}

/// Builds the handler that marks the picture as failed.
///
/// # Arguments
///
/// - `Signal<EuvImageStatus>` - The status signal updated on error.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The error handler.
fn on_image_failed(status: Signal<EuvImageStatus>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        status.set(EuvImageStatus::Failed);
    }))
}
