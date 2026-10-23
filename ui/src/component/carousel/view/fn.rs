use super::*;

/// A slideshow.
///
/// Renders a `c_euv_carousel` box holding a `c_euv_carousel_track` whose
/// slides are laid out side by side and shifted with a transform, plus a
/// row of `c_euv_carousel_dot` markers for the slides.
///
/// The track is shifted with an inline transform rather than by
/// swapping slides: every slide stays mounted, so a slide that is
/// halfway out of view is still in the DOM and the transition is a
/// single animated property instead of a tree rebuild.
///
/// # Arguments
///
/// - `VirtualNode<EuvCarouselProps>` - The props node containing the
///   carousel configuration.
///
/// # Returns
///
/// - `VirtualNode` - The carousel virtual DOM tree.
#[component]
pub fn euv_carousel(node: VirtualNode<EuvCarouselProps>) -> VirtualNode {
    let EuvCarouselProps {
        slides,
        active,
        on_change,
    }: EuvCarouselProps = node.try_get_props().unwrap_or_default();
    let index: u32 = active.get();
    let track_style: String = format!("transform: translateX(-{}%);", index as i64 * 100);
    let track_ref: &str = track_style.as_str();
    let pane_nodes: Vec<VirtualNode> = slides
        .into_iter()
        .map(|slide: EuvCarouselSlide| {
            html! {
                div {
                    class: c_euv_carousel_slide()
                    div {
                        class: c_euv_text_title()
                        slide.title
                    }
                    if !slide.desc.is_empty() {
                        div {
                            class: c_euv_text_secondary()
                            slide.desc
                        }
                    }
                }
            }
        })
        .collect();
    let dot_count: u32 = pane_nodes.len() as u32;
    let dots: Vec<VirtualNode> = (0..dot_count)
        .map(|dot: u32| {
            let is_active: bool = dot == index;
            html! {
                button {
                    class: if is_active {
                        c_euv_carousel_dot_active()
                    } else {
                        c_euv_carousel_dot()
                    }
                    key: dot.to_string()
                    onclick: on_carousel_dot(active, on_change.clone(), dot)
                }
            }
        })
        .collect();
    html! {
        div {
            class: c_euv_carousel()
            div {
                class: c_euv_carousel_track()
                style: track_ref
                pane_nodes
            }
            if dot_count > 1 {
                div {
                    class: c_euv_carousel_dots()
                    dots
                }
            }
        }
    }
}

/// Builds the handler that moves the carousel to a slide.
///
/// # Arguments
///
/// - `Signal<u32>` - The index signal updated by the handler.
/// - `Option<Rc<dyn Fn(u32)>>` - The change callback.
/// - `u32` - The index of the slide that owns the handler.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn on_carousel_dot(
    active: Signal<u32>,
    on_change: Option<Rc<dyn Fn(u32)>>,
    index: u32,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        active.set(index);
        if let Some(callback) = &on_change {
            callback(index);
        }
    }))
}
