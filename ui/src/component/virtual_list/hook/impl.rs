use super::*;

/// Encapsulated access to the global pending measurement set.
impl PendingMeasureCell {
    /// Returns a mutable reference to the set of pending container ids.
    fn get_mut_pending_measure() -> &'static mut HashSet<String> {
        unsafe {
            &mut *(*std::ptr::addr_of_mut!(PENDING_MEASURE_BY_ID))
                .deref()
                .get_0()
                .get()
        }
    }
}

/// Implementation of virtual list functionality.
impl UseVirtualList {
    /// Creates virtual list state signals for tracking scroll offset and viewport height.
    ///
    /// # Returns
    ///
    /// - `UseVirtualList` - The virtual list state containing scroll offset and viewport height signals.
    pub fn use_scroll_state() -> UseVirtualList {
        UseVirtualList::new(App::use_signal(|| 0), App::use_signal(|| 0))
    }

    /// Creates a scroll event handler that tracks the container scroll position and viewport height.
    ///
    /// Reads `scrollTop` and `clientHeight` from the scroll container element
    /// referenced by `VIRTUAL_LIST_CONTAINER_ID` and updates the corresponding signals.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A scroll handler for the virtual list container.
    pub fn on_scroll(self) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            if let Some(container) = Self::try_get_container() {
                let html_element: HtmlElement = container.unchecked_into();
                self.get_scroll_offset().set(html_element.scroll_top());
                self.get_viewport_height().set(html_element.client_height());
            }
        }))
    }

    /// Reads the container `clientHeight` and writes it to the viewport height signal.
    ///
    /// Should only be called when the DOM is already present (e.g. inside a resize
    /// callback or after the first paint). For initial mount use
    /// `schedule_measure` instead.
    pub fn update_viewport_height(self) {
        if let Some(container) = Self::try_get_container() {
            let html_element: HtmlElement = container.unchecked_into();
            self.get_viewport_height().set(html_element.client_height());
        }
    }

    /// Schedules a viewport height measurement on the next animation frame.
    ///
    /// Uses an atomic guard to ensure only one measurement is pending at a time —
    /// if a previous callback hasn't fired yet, subsequent calls are silently
    /// ignored. This prevents accumulating redundant animation-frame callbacks
    /// when the component re-renders frequently.
    pub fn schedule_measure(self) {
        if PENDING_MEASURE.swap(true, Ordering::Relaxed) {
            return;
        }
        let callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            PENDING_MEASURE.store(false, Ordering::Relaxed);
            self.update_viewport_height();
        }));
        let Some(window_value) = window() else {
            PENDING_MEASURE.store(false, Ordering::Relaxed);
            return;
        };
        let Ok(_) = window_value.request_animation_frame(callback.as_ref().unchecked_ref()) else {
            PENDING_MEASURE.store(false, Ordering::Relaxed);
            return;
        };
        callback.forget();
    }

    /// Schedules a viewport height measurement on the next animation frame for a specific container.
    ///
    /// Uses an atomic set guard keyed by `container_id` to ensure only one pending
    /// measurement exists per container — if a callback for the same id hasn't
    /// fired yet, subsequent calls are silently ignored. This prevents accumulating
    /// redundant animation-frame callbacks when the component re-renders frequently.
    ///
    /// # Arguments
    ///
    /// - `&str` - The container element id.
    pub(crate) fn schedule_measure_by_id(self, container_id: &str) {
        if !PendingMeasureCell::get_mut_pending_measure().insert(container_id.to_string()) {
            return;
        }

        let id: String = container_id.to_string();
        let callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            PendingMeasureCell::get_mut_pending_measure().remove(&id);
            if let Some(element) = Self::try_get_container_by_id(&id) {
                let html_element: HtmlElement = element.unchecked_into();
                self.get_viewport_height().set(html_element.client_height());
            }
        }));
        let Some(window_value) = window() else {
            PendingMeasureCell::get_mut_pending_measure().remove(container_id);
            return;
        };
        let Ok(_) = window_value.request_animation_frame(callback.as_ref().unchecked_ref()) else {
            PendingMeasureCell::get_mut_pending_measure().remove(container_id);
            return;
        };
        callback.forget();
    }

    /// Returns the virtual list container element by its default id.
    ///
    /// # Returns
    ///
    /// - `Option<Element>` - The container element, if found in the document.
    pub fn try_get_container() -> Option<Element> {
        let window_value: Window = window()?;
        let document_value: Document = window_value.document()?;
        document_value.get_element_by_id(VIRTUAL_LIST_CONTAINER_ID)
    }

    /// Returns the virtual list container element by its id.
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - The container element id.
    ///
    /// # Returns
    ///
    /// - `Option<Element>` - The container element, if found in the document.
    pub fn try_get_container_by_id<C>(container_id: C) -> Option<Element>
    where
        C: AsRef<str>,
    {
        let window_value: Window = window()?;
        let document_value: Document = window_value.document()?;
        document_value.get_element_by_id(container_id.as_ref())
    }

    /// Computes the range of visible item indices for the virtual list.
    ///
    /// Calculates the start and end indices based on the current scroll offset,
    /// viewport height, fixed item height, and total item count. Includes an
    /// overscan buffer to reduce blank areas during fast scrolling.
    ///
    /// # Arguments
    ///
    /// - `i32` - The current scroll offset in pixels.
    /// - `i32` - The current viewport height in pixels.
    /// - `usize` - The total number of items in the list.
    /// - `i32` - The fixed height of each item in pixels.
    /// - `usize` - The number of overscan items to render beyond the viewport.
    ///
    /// # Returns
    ///
    /// - `(usize, usize, usize, usize)` - The first pair is the actual
    ///   visible range without overscan; the second pair is the
    ///   rendering range including overscan.
    pub(crate) fn compute_visible_range(
        scroll_offset: i32,
        viewport_height: i32,
        total_count: usize,
        item_height: i32,
        overscan_count: usize,
    ) -> (usize, usize, usize, usize) {
        let visible_start: usize = (scroll_offset / item_height).max(0) as usize;
        let visible_count: usize = if viewport_height > 0 {
            let viewport_bottom: i32 = scroll_offset + viewport_height;
            let visible_end: usize =
                ((viewport_bottom + item_height - 1) / item_height).max(0) as usize;
            visible_end - visible_start
        } else {
            VIRTUAL_LIST_DEFAULT_VISIBLE_COUNT
        };
        let visible_end: usize = (visible_start + visible_count).min(total_count);
        let render_start: usize = visible_start.saturating_sub(overscan_count);
        let render_end: usize = (visible_end + overscan_count).min(total_count);
        (visible_start, visible_end, render_start, render_end)
    }
}
