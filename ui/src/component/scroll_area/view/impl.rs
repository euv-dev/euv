use super::*;

/// Disconnects native resources before callback storage is released.
impl Drop for ScrollAreaController {
    /// Runs teardown even when the hook owner is dropped without explicit cleanup.
    fn drop(&mut self) {
        self.dispose();
    }
}

/// Inherent implementation of [`ScrollAreaController`].
impl ScrollAreaController {
    /// Creates the hook-owned controller and schedules its mounted setup.
    ///
    /// # Arguments
    /// - `NodeRef<Element>` - Lifecycle-managed root reference.
    ///
    /// # Returns
    /// - `Rc<Self>` - Strong owner of all weak callbacks.
    pub(crate) fn attach(root: NodeRef<Element>) -> Rc<Self> {
        let owner: Rc<Self> = Rc::new(Self {
            root,
            parts: RefCell::new(None),
            frame: Cell::new(None),
            callback: RefCell::new(None),
            resize: RefCell::new(None),
            mutation: RefCell::new(None),
            listeners: RefCell::new(Vec::new()),
            drag: Cell::new(None),
            disposed: Cell::new(false),
        });
        let weak: Weak<Self> = Rc::downgrade(&owner);
        *owner.get_callback().borrow_mut() = Some(Closure::wrap(Box::new(move |_: f64| {
            if let Some(owner) = weak.upgrade() {
                owner.get_frame().set(None);
                if owner.get_parts().borrow().is_none() {
                    owner.mount();
                }
                owner.sync();
            }
        }) as Box<dyn FnMut(f64)>));
        owner.schedule();
        owner
    }

    /// Coalesces observer and scroll updates into one cancellable frame.
    pub(crate) fn schedule(&self) {
        if self.get_disposed().get() || self.get_frame().get().is_some() {
            return;
        }
        if let Some(window) = window()
            && let Some(callback) = self.get_callback().borrow().as_ref()
            && let Ok(id) = window.request_animation_frame(callback.as_ref().unchecked_ref())
        {
            self.get_frame().set(Some(id));
        }
    }

    /// Resolves only immediately owned parts and installs weak native listeners.
    ///
    /// # Arguments
    /// - `&Rc<Self>` - Strong owner used only to create weak callback references.
    fn mount(self: &Rc<Self>) {
        let Some(root) = self
            .get_root()
            .get_cloned()
            .filter(|root: &Element| root.is_connected())
        else {
            return;
        };
        let Some(viewport) = scroll_area_part(&root, SCROLL_AREA_VIEWPORT) else {
            return;
        };
        let Some(bar) = scroll_area_part(&root, SCROLL_AREA_BAR) else {
            return;
        };
        let Some(thumb) = scroll_area_part(&root, SCROLL_AREA_THUMB) else {
            return;
        };
        *self.get_parts().borrow_mut() = Some(ScrollAreaParts {
            viewport,
            bar,
            thumb,
        });
        for event in SCROLL_AREA_EVENTS {
            let weak: Weak<Self> = Rc::downgrade(self);
            let callback: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
                if let Some(owner) = weak.upgrade() {
                    owner.handle(event);
                }
            })
                as Box<dyn FnMut(Event)>);
            if root
                .add_event_listener_with_callback_and_bool(
                    event,
                    callback.as_ref().unchecked_ref(),
                    true,
                )
                .is_ok()
            {
                self.get_listeners()
                    .borrow_mut()
                    .push((root.clone().into(), event, callback));
            }
        }
        self.observe();
    }

    /// Installs resize and content mutation observation without observing thumb writes.
    ///
    /// # Arguments
    /// - `&Rc<Self>` - Strong owner used only to create weak callback references.
    fn observe(self: &Rc<Self>) {
        let weak: Weak<Self> = Rc::downgrade(self);
        let callback: ScrollAreaObserverCallback = Closure::wrap(Box::new(move |_: Array| {
            if let Some(owner) = weak.upgrade() {
                owner.schedule();
            }
        })
            as Box<dyn FnMut(Array)>);
        if let Ok(observer) = ResizeObserver::new(callback.as_ref().unchecked_ref()) {
            *self.get_resize().borrow_mut() = Some((observer, callback));
            self.observe_content();
        }
        let weak: Weak<Self> = Rc::downgrade(self);
        let callback: ScrollAreaObserverCallback = Closure::wrap(Box::new(move |records: Array| {
            if let Some(owner) = weak.upgrade() {
                let relevant: bool = records.iter().any(|record: JsValue| {
                    let target: Option<Element> =
                        Reflect::get(&record, &JsValue::from_str(SCROLL_AREA_TARGET))
                            .ok()
                            .and_then(|target: JsValue| target.dyn_into::<Element>().ok());
                    let parts: Ref<'_, Option<ScrollAreaParts>> = owner.get_parts().borrow();
                    !parts.as_ref().is_some_and(|parts: &ScrollAreaParts| {
                        target.as_ref().is_some_and(|target: &Element| {
                            target == parts.get_bar().as_ref()
                                || target == parts.get_thumb().as_ref()
                        })
                    })
                });
                if relevant {
                    owner.observe_content();
                    owner.schedule();
                }
            }
        })
            as Box<dyn FnMut(Array)>);
        let Some(root) = self.get_root().get_cloned() else {
            return;
        };
        let constructor: Option<Function> =
            Reflect::get(&global(), &JsValue::from_str(SCROLL_AREA_OBSERVER))
                .ok()
                .and_then(|value: JsValue| value.dyn_into::<Function>().ok());
        if let Some(constructor) = constructor
            && let Ok(observer) = Reflect::construct(&constructor, &Array::of1(callback.as_ref()))
        {
            let options: Object = Object::new();
            for key in SCROLL_AREA_OBSERVER_OPTIONS {
                let _: Result<bool, JsValue> =
                    Reflect::set(&options, &JsValue::from_str(key), &JsValue::TRUE);
            }
            if let Ok(method) = Reflect::get(&observer, &JsValue::from_str(SCROLL_AREA_OBSERVE))
                && let Some(method) = method.dyn_ref::<Function>()
                && method.call2(&observer, root.as_ref(), &options).is_ok()
            {
                *self.get_mutation().borrow_mut() = Some((observer.into(), callback));
            }
        }
    }

    /// Observes each direct content wrapper as well as the root and viewport.
    fn observe_content(&self) {
        let observers: Ref<'_, Option<(ResizeObserver, ScrollAreaObserverCallback)>> =
            self.get_resize().borrow();
        let parts: Option<ScrollAreaParts> = self.get_parts().borrow().clone();
        if let Some((observer, _)) = observers.as_ref()
            && let Some(parts) = parts.as_ref()
        {
            observer.disconnect();
            if let Some(root) = self.get_root().get_cloned() {
                observer.observe(&root);
            }
            observer.observe(parts.get_viewport());
            let children: HtmlCollection = parts.get_viewport().children();
            for index in 0..children.length() {
                if let Some(child) = children.item(index) {
                    observer.observe(&child);
                }
            }
        }
    }

    /// Updates proportional thumb geometry without changing content layout.
    fn sync(&self) {
        if self.get_disposed().get() {
            return;
        }
        let parts: Option<ScrollAreaParts> = self.get_parts().borrow().clone();
        let Some(parts) = parts.as_ref() else {
            return;
        };
        let viewport: &HtmlElement = parts.get_viewport();
        let bar: &HtmlElement = parts.get_bar();
        let thumb: &HtmlElement = parts.get_thumb();
        let range: i32 = viewport.scroll_height() - viewport.client_height();
        let tab_index: i32 = if range > 0 && viewport.client_height() > 0 {
            0
        } else {
            -1
        };
        if viewport.tab_index() != tab_index {
            viewport.set_tab_index(tab_index);
        }
        if range <= 0 || viewport.client_height() <= 0 {
            scroll_area_style(bar, SCROLL_AREA_DISPLAY, SCROLL_AREA_NONE);
            self.cancel_drag(parts);
            return;
        }
        scroll_area_style(bar, SCROLL_AREA_DISPLAY, "flex");
        let padding: f64 = scroll_area_length(bar, SCROLL_AREA_PADDING[0])
            + scroll_area_length(bar, SCROLL_AREA_PADDING[1]);
        let (length, offset, _): (f64, f64, f64) = scroll_area_geometry(
            viewport.client_height() as f64,
            viewport.scroll_height() as f64,
            bar.client_height() as f64,
            padding,
            viewport.scroll_top() as f64,
        );
        scroll_area_style(thumb, SCROLL_AREA_HEIGHT, &format!("{length}px"));
        scroll_area_style(
            thumb,
            SCROLL_AREA_TRANSFORM,
            &format!("translateY({offset}px)"),
        );
    }

    /// Handles only events whose closest scroll root is this controller's owner.
    ///
    /// # Arguments
    /// - `Event` - Native root-capture event.
    fn handle(&self, event: Event) {
        if self.get_disposed().get() {
            return;
        }
        let Some(root) = self.get_root().get_cloned() else {
            return;
        };
        let target: Option<Element> = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<Element>().ok());
        if !target.as_ref().is_some_and(|target: &Element| {
            target.closest(SCROLL_AREA_ROOT).ok().flatten().as_ref() == Some(&root)
        }) {
            return;
        }
        let parts: Option<ScrollAreaParts> = self.get_parts().borrow().clone();
        let Some(parts) = parts.as_ref() else {
            return;
        };
        let viewport: &HtmlElement = parts.get_viewport();
        let bar: &HtmlElement = parts.get_bar();
        let thumb: &HtmlElement = parts.get_thumb();
        let kind: String = event.type_();
        if kind == SCROLL_AREA_EVENTS[0] {
            self.schedule();
            return;
        }
        if kind == SCROLL_AREA_EVENTS[6] {
            if target.as_ref() != Some(viewport.as_ref()) || event.default_prevented() {
                return;
            }
            if let Some(keyboard) = event.dyn_ref::<KeyboardEvent>()
                && !keyboard.alt_key()
                && !keyboard.ctrl_key()
                && !keyboard.meta_key()
                && let Some(offset) = scroll_area_key_offset(
                    &keyboard.key(),
                    viewport.scroll_top() as f64,
                    viewport.client_height() as f64,
                    (viewport.scroll_height() - viewport.client_height()) as f64,
                )
            {
                event.prevent_default();
                viewport.set_scroll_top(offset.round() as i32);
                self.schedule();
            }
            return;
        }
        let pointer: i32 = Reflect::get(event.as_ref(), &JsValue::from_str(SCROLL_AREA_POINTER_ID))
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or(-1.0) as i32;
        if [
            SCROLL_AREA_EVENTS[3],
            SCROLL_AREA_EVENTS[4],
            SCROLL_AREA_EVENTS[5],
        ]
        .contains(&kind.as_str())
        {
            if self
                .get_drag()
                .get()
                .is_some_and(|(id, _): (i32, f64)| id == pointer)
            {
                self.cancel_drag(parts);
            }
            return;
        }
        let Some(mouse) = event.dyn_ref::<MouseEvent>() else {
            return;
        };
        let padding_top: f64 = scroll_area_length(bar, SCROLL_AREA_PADDING[0]);
        let padding: f64 = padding_top + scroll_area_length(bar, SCROLL_AREA_PADDING[1]);
        let (length, _, range): (f64, f64, f64) = scroll_area_geometry(
            viewport.client_height() as f64,
            viewport.scroll_height() as f64,
            bar.client_height() as f64,
            padding,
            viewport.scroll_top() as f64,
        );
        if range <= 0.0 {
            return;
        }
        let top: f64 = bar.get_bounding_client_rect().top() + bar.client_top() as f64 + padding_top;
        let travel: f64 = bar.client_height() as f64 - padding - length;
        if kind == SCROLL_AREA_EVENTS[1] {
            if mouse.button() != 0
                || !target
                    .as_ref()
                    .is_some_and(|target: &Element| bar.contains(Some(target)))
            {
                return;
            }
            event.prevent_default();
            let grabbed: f64 = if target
                .as_ref()
                .is_some_and(|target: &Element| thumb.contains(Some(target)))
            {
                mouse.client_y() as f64 - thumb.get_bounding_client_rect().top()
            } else {
                length / 2.0
            };
            if bar.set_pointer_capture(pointer).is_ok() {
                self.get_drag().set(Some((pointer, grabbed)));
            }
            viewport.set_scroll_top(
                scroll_area_drag_offset(mouse.client_y() as f64 - top - grabbed, travel, range)
                    .round() as i32,
            );
            self.schedule();
        } else if kind == SCROLL_AREA_EVENTS[2]
            && let Some((id, grabbed)) = self.get_drag().get()
            && id == pointer
        {
            event.prevent_default();
            viewport.set_scroll_top(
                scroll_area_drag_offset(mouse.client_y() as f64 - top - grabbed, travel, range)
                    .round() as i32,
            );
            self.schedule();
        }
    }

    /// Clears captured pointer state on completion, cancellation or loss of overflow.
    ///
    /// # Arguments
    /// - `&ScrollAreaParts` - Mounted parts carrying the pointer capture.
    fn cancel_drag(&self, parts: &ScrollAreaParts) {
        if let Some((pointer, _)) = self.get_drag().take()
            && parts.get_bar().has_pointer_capture(pointer)
        {
            let _: Result<(), JsValue> = parts.get_bar().release_pointer_capture(pointer);
        }
    }

    /// Cancels pending frames, disconnects observers and removes exact listeners.
    pub(crate) fn dispose(&self) {
        if self.get_disposed().replace(true) {
            return;
        }
        if let Some(parts) = self.get_parts().borrow().as_ref() {
            self.cancel_drag(parts);
        }
        if let Some(frame) = self.get_frame().take()
            && let Some(window) = window()
        {
            let _: Result<(), JsValue> = window.cancel_animation_frame(frame);
        }
        for (target, name, callback) in self.get_listeners().borrow_mut().drain(..) {
            let _: Result<(), JsValue> = target.remove_event_listener_with_callback_and_bool(
                name,
                callback.as_ref().unchecked_ref(),
                true,
            );
        }
        if let Some((observer, callback)) = self.get_resize().borrow_mut().take() {
            observer.disconnect();
            drop(callback);
        }
        if let Some((observer, callback)) = self.get_mutation().borrow_mut().take() {
            if let Ok(method) = Reflect::get(&observer, &JsValue::from_str(SCROLL_AREA_DISCONNECT))
                && let Some(method) = method.dyn_ref::<Function>()
            {
                let _: Result<JsValue, JsValue> = method.call0(&observer);
            }
            drop(callback);
        }
        self.get_callback().borrow_mut().take();
        self.get_parts().borrow_mut().take();
    }
}
