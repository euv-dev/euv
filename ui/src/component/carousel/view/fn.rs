use super::*;

/// A slideshow.
///
/// Renders a `c_euv_carousel` box holding a `c_euv_carousel_track` whose
/// slides are laid out side by side and shifted with a transform, plus a
/// row of `c_euv_carousel_dot` markers and previous/next controls.
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
    let slide_count: u32 = slides.len() as u32;
    let index: u32 = if slide_count == 0 {
        0
    } else {
        active.get().min(slide_count - 1)
    };
    let track_style: String = format!("transform: translateX(-{}%);", index as i64 * 100);
    let track_ref: &str = track_style.as_str();
    let pane_nodes: Vec<VirtualNode> = slides
        .into_iter()
        .enumerate()
        .map(|(slide_index, slide): (usize, EuvCarouselSlide)| {
            let is_active: bool = slide_index as u32 == index;
            html! {
                div {
                    role: "group"
                    "aria-roledescription": CAROUSEL_SLIDE_ROLE_DESCRIPTION
                    "aria-label": format!("{CAROUSEL_SLIDE_LABEL} {} {CAROUSEL_OF_LABEL} {}", slide_index + 1, slide_count)
                    "aria-hidden": if is_active {
                        CAROUSEL_ATTR_FALSE
                    } else {
                        CAROUSEL_ATTR_TRUE
                    }
                    "data-slot": "carousel-item"
                    class: c_euv_carousel_slide()
                    key: slide_index.to_string()
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
                    type: "button"
                    "data-slot": "carousel-dot"
                    "aria-label": format!("{CAROUSEL_DOT_LABEL_PREFIX} {} {CAROUSEL_OF_LABEL} {dot_count}", dot + 1)
                    "aria-pressed": if is_active {
                        CAROUSEL_ATTR_TRUE
                    } else {
                        CAROUSEL_ATTR_FALSE
                    }
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
    let previous_disabled: bool = index == 0;
    let next_disabled: bool = index + 1 >= dot_count;
    let previous_disabled_signal: Signal<bool> = Signal::create(previous_disabled);
    let next_disabled_signal: Signal<bool> = Signal::create(next_disabled);
    let previous_on_press: Rc<dyn Fn()> =
        on_carousel_step(active, on_change.clone(), -1, dot_count);
    let next_on_press: Rc<dyn Fn()> = on_carousel_step(active, on_change.clone(), 1, dot_count);
    let previous_on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        previous_on_press();
    }));
    let next_on_click: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
        next_on_press();
    }));
    let touch_start_x: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let on_touch_start: Option<Rc<dyn Fn(Event)>> = carousel_touch_start(touch_start_x.clone());
    let on_touch_end: Option<Rc<dyn Fn(Event)>> =
        carousel_touch_end(touch_start_x.clone(), active, on_change.clone(), dot_count);
    let on_touch_cancel: Option<Rc<dyn Fn(Event)>> = carousel_touch_cancel(touch_start_x);
    html! {
        div {
            role: "region"
            "aria-roledescription": CAROUSEL_ROLE_DESCRIPTION
            "aria-label": CAROUSEL_ARIA_LABEL
            "data-slot": "carousel"
            class: c_euv_carousel()
            onkeydown: on_carousel_keydown(active, on_change.clone(), dot_count)
            ontouchstart: on_touch_start
            ontouchend: on_touch_end
            ontouchcancel: on_touch_cancel
            div {
                "data-slot": "carousel-track"
                "aria-live": CAROUSEL_LIVE_POLITE
                "aria-atomic": CAROUSEL_ATTR_TRUE
                class: c_euv_carousel_track()
                style: track_ref
                pane_nodes
            }
            if dot_count > 1 {
                div {
                    role: "group"
                    "aria-label": CAROUSEL_DOT_GROUP_LABEL
                    "data-slot": "carousel-dots"
                    class: c_euv_carousel_dots()
                    dots
                }
                button {
                    key: CAROUSEL_PREVIOUS_LABEL.to_string()
                    type: "button"
                    "data-slot": "carousel-previous"
                    "aria-label": CAROUSEL_PREVIOUS_LABEL
                    disabled: previous_disabled_signal
                    class: c_euv_carousel_previous()
                    onclick: previous_on_click
                    span {
                        "aria-hidden": CAROUSEL_ATTR_TRUE
                        CAROUSEL_PREVIOUS_GLYPH
                    }
                }
                button {
                    key: CAROUSEL_NEXT_LABEL.to_string()
                    type: "button"
                    "data-slot": "carousel-next"
                    "aria-label": CAROUSEL_NEXT_LABEL
                    disabled: next_disabled_signal
                    class: c_euv_carousel_next()
                    onclick: next_on_click
                    span {
                        "aria-hidden": CAROUSEL_ATTR_TRUE
                        CAROUSEL_NEXT_GLYPH
                    }
                }
            }
        }
    }
}

/// The viewport the slides travel inside.
///
/// The viewport clips the track, so a slide that sits beside the visible
/// one is hidden rather than spilling out of the carousel.
///
/// # Arguments
///
/// - `VirtualNode<EuvCarouselContentProps>` - The props node containing the
///   carousel configuration.
///
/// # Returns
///
/// - `VirtualNode` - The carousel virtual DOM tree.
#[component]
pub fn euv_carousel_content(node: VirtualNode<EuvCarouselContentProps>) -> VirtualNode {
    let EuvCarouselContentProps { orientation }: EuvCarouselContentProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            "data-slot": "carousel-content"
            class: c_euv_carousel_content()
            div {
                class: if orientation == EuvCarouselOrientation::Vertical {
                    c_euv_carousel_content_track_vertical()
                } else {
                    c_euv_carousel_content_track()
                }
                children
            }
        }
    }
}

/// One slide of the carousel.
///
/// # Arguments
///
/// - `VirtualNode<EuvCarouselItemProps>` - The props node containing the
///   carousel configuration.
///
/// # Returns
///
/// - `VirtualNode` - The carousel virtual DOM tree.
#[component]
pub fn euv_carousel_item(node: VirtualNode<EuvCarouselItemProps>) -> VirtualNode {
    let EuvCarouselItemProps { orientation }: EuvCarouselItemProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            role: "group"
            "aria-roledescription": "slide"
            "data-slot": "carousel-item"
            class: if orientation == EuvCarouselOrientation::Vertical {
                c_euv_carousel_item_vertical()
            } else {
                c_euv_carousel_item()
            }
            children
        }
    }
}

/// The button that steps the carousel forward.
///
/// # Arguments
///
/// - `VirtualNode<EuvCarouselNextProps>` - The props node containing the
///   carousel configuration.
///
/// # Returns
///
/// - `VirtualNode` - The carousel virtual DOM tree.
#[component]
pub fn euv_carousel_next(node: VirtualNode<EuvCarouselNextProps>) -> VirtualNode {
    let EuvCarouselNextProps {
        orientation,
        label,
        disabled,
        on_press,
    }: EuvCarouselNextProps = node.try_get_props().unwrap_or_default();
    let accessible_label: &str = if label.is_empty() {
        CAROUSEL_NEXT_LABEL
    } else {
        label
    };
    let is_disabled: bool = disabled || on_press.is_none();
    let disabled_signal: Signal<bool> = Signal::create(is_disabled);
    let on_click: Option<Rc<dyn Fn(Event)>> = if is_disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            if let Some(callback) = &on_press {
                callback();
            }
        }))
    };
    html! {
        button {
            type: "button"
            "data-slot": "carousel-next"
            "aria-label": accessible_label
            disabled: disabled_signal
            class: if orientation == EuvCarouselOrientation::Vertical {
                c_euv_carousel_next_vertical()
            } else {
                c_euv_carousel_next()
            }
            onclick: on_click
            span {
                "aria-hidden": CAROUSEL_ATTR_TRUE
                CAROUSEL_NEXT_GLYPH
            }
        }
    }
}

/// The button that steps the carousel back.
///
/// # Arguments
///
/// - `VirtualNode<EuvCarouselPreviousProps>` - The props node containing the
///   carousel configuration.
///
/// # Returns
///
/// - `VirtualNode` - The carousel virtual DOM tree.
#[component]
pub fn euv_carousel_previous(node: VirtualNode<EuvCarouselPreviousProps>) -> VirtualNode {
    let EuvCarouselPreviousProps {
        orientation,
        label,
        disabled,
        on_press,
    }: EuvCarouselPreviousProps = node.try_get_props().unwrap_or_default();
    let accessible_label: &str = if label.is_empty() {
        CAROUSEL_PREVIOUS_LABEL
    } else {
        label
    };
    let is_disabled: bool = disabled || on_press.is_none();
    let disabled_signal: Signal<bool> = Signal::create(is_disabled);
    let on_click: Option<Rc<dyn Fn(Event)>> = if is_disabled {
        None
    } else {
        Some(Rc::new(move |_: Event| {
            if let Some(callback) = &on_press {
                callback();
            }
        }))
    };
    html! {
        button {
            type: "button"
            "data-slot": "carousel-previous"
            "aria-label": accessible_label
            disabled: disabled_signal
            class: if orientation == EuvCarouselOrientation::Vertical {
                c_euv_carousel_previous_vertical()
            } else {
                c_euv_carousel_previous()
            }
            onclick: on_click
            span {
                "aria-hidden": CAROUSEL_ATTR_TRUE
                CAROUSEL_PREVIOUS_GLYPH
            }
        }
    }
}

/// Builds a handler that moves one slide in the given direction.
///
/// # Arguments
///
/// - `Signal<u32>` - The active slide signal.
/// - `Option<Rc<dyn Fn(u32)>>` - The callback invoked after the active slide changes.
/// - `i8` - The direction to move (`-1` for previous, `1` for next).
/// - `u32` - The number of slides.
///
/// # Returns
///
/// - `Rc<dyn Fn()>` - The slide-change handler.
fn on_carousel_step(
    active: Signal<u32>,
    on_change: Option<Rc<dyn Fn(u32)>>,
    direction: i8,
    slide_count: u32,
) -> Rc<dyn Fn()> {
    Rc::new(move || {
        if slide_count == 0 {
            return;
        }
        let current_index: u32 = active.get().min(slide_count - 1);
        let target_index: Option<u32> = match direction {
            -1 if current_index > 0 => Some(current_index - 1),
            1 if current_index + 1 < slide_count => Some(current_index + 1),
            _ => None,
        };
        if let Some(target_index) = target_index {
            active.set(target_index);
            if let Some(callback) = &on_change {
                callback(target_index);
            }
        }
    })
}

/// Builds the keyboard handler for the carousel region.
///
/// Arrow keys move between adjacent slides; Home and End select the bounds.
///
/// # Arguments
///
/// - `Signal<u32>` - The active slide signal.
/// - `Option<Rc<dyn Fn(u32)>>` - The callback invoked after the active slide changes.
/// - `u32` - The number of slides.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The keyboard handler, absent for an empty carousel.
fn on_carousel_keydown(
    active: Signal<u32>,
    on_change: Option<Rc<dyn Fn(u32)>>,
    slide_count: u32,
) -> Option<Rc<dyn Fn(Event)>> {
    if slide_count == 0 {
        return None;
    }
    Some(Rc::new(move |event: Event| {
        let Ok(keyboard_event) = event.dyn_into::<KeyboardEvent>() else {
            return;
        };
        let current_index: u32 = active.get().min(slide_count - 1);
        let target_index: u32 = match keyboard_event.key().as_str() {
            CAROUSEL_KEY_ARROW_LEFT if current_index > 0 => current_index - 1,
            CAROUSEL_KEY_ARROW_RIGHT if current_index + 1 < slide_count => current_index + 1,
            CAROUSEL_KEY_HOME => 0,
            CAROUSEL_KEY_END => slide_count - 1,
            _ => return,
        };
        if target_index == current_index {
            return;
        }
        keyboard_event.prevent_default();
        active.set(target_index);
        if let Some(callback) = &on_change {
            callback(target_index);
        }
    }))
}

/// Captures the starting position of a single-touch carousel gesture.
///
/// Touches that begin on a button are left to the button's native activation.
///
/// # Arguments
///
/// - `Rc<Cell<Option<i32>>>` - Storage shared with the touch-end handler.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The touch-start handler.
fn carousel_touch_start(start_x: Rc<Cell<Option<i32>>>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        start_x.set(None);
        let Some(target) = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok())
        else {
            return;
        };
        if target
            .closest(CAROUSEL_BUTTON_SELECTOR)
            .ok()
            .flatten()
            .is_some()
        {
            return;
        }
        let touch_event: &TouchEvent = event.unchecked_ref::<TouchEvent>();
        let touches: TouchList = touch_event.touches();
        if touches.length() == 1
            && let Some(touch) = touches.get(0)
        {
            start_x.set(Some(touch.client_x()));
        }
    }))
}

/// Advances the carousel after a horizontal swipe.
///
/// # Arguments
///
/// - `Rc<Cell<Option<i32>>>` - The starting position captured on touch-start.
/// - `Signal<u32>` - The active slide signal.
/// - `Option<Rc<dyn Fn(u32)>>` - The callback invoked after the active slide changes.
/// - `u32` - The number of slides.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The touch-end handler, absent for fewer than two slides.
fn carousel_touch_end(
    start_x: Rc<Cell<Option<i32>>>,
    active: Signal<u32>,
    on_change: Option<Rc<dyn Fn(u32)>>,
    slide_count: u32,
) -> Option<Rc<dyn Fn(Event)>> {
    if slide_count < 2 {
        return None;
    }
    Some(Rc::new(move |event: Event| {
        let Some(start_x) = start_x.take() else {
            return;
        };
        let touch_event: &TouchEvent = event.unchecked_ref::<TouchEvent>();
        let touches: TouchList = touch_event.changed_touches();
        let Some(touch) = touches.get(0) else {
            return;
        };
        let distance: i32 = touch.client_x() - start_x;
        if distance.abs() < CAROUSEL_SWIPE_THRESHOLD {
            return;
        }
        let current_index: u32 = active.get().min(slide_count - 1);
        let target_index: Option<u32> = if distance < 0 && current_index + 1 < slide_count {
            Some(current_index + 1)
        } else if distance > 0 && current_index > 0 {
            Some(current_index - 1)
        } else {
            None
        };
        if let Some(target_index) = target_index {
            event.prevent_default();
            active.set(target_index);
            if let Some(callback) = &on_change {
                callback(target_index);
            }
        }
    }))
}

/// Clears the starting position when a touch gesture is cancelled.
///
/// # Arguments
///
/// - `Rc<Cell<Option<i32>>>` - Storage shared with the touch-start handler.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The touch-cancel handler.
fn carousel_touch_cancel(start_x: Rc<Cell<Option<i32>>>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        start_x.set(None);
    }))
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
