use super::*;

/// Inherent implementation of [`UseResizeObserver`].
impl UseResizeObserver {
    /// Creates a handle with the size starting at zero.
    ///
    /// # Returns
    ///
    /// - `UseResizeObserver` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            target: NodeRef::new(),
            width: Signal::create(0.0),
            height: Signal::create(0.0),
            teardown: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the current width as a snapshot.
    ///
    /// # Returns
    ///
    /// - `f64` - The content-box width in CSS pixels.
    pub fn current_width(&self) -> f64 {
        self.get_width().get()
    }

    /// Returns the current height as a snapshot.
    ///
    /// # Returns
    ///
    /// - `f64` - The content-box height in CSS pixels.
    pub fn current_height(&self) -> f64 {
        self.get_height().get()
    }

    /// Creates the observer and starts watching the element.
    ///
    /// The element does not exist while the first render is being
    /// built, so the `observe` call is deferred until after the DOM
    /// has been committed.
    pub(crate) fn attach(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let width: Signal<f64> = self.get_width();
            let height: Signal<f64> = self.get_height();
            let closure: Closure<dyn FnMut(Array)> =
                Closure::wrap(Box::new(move |entries: Array| {
                    for entry in entries.iter() {
                        let Ok(entry) = entry.dyn_into::<ResizeObserverEntry>() else {
                            continue;
                        };
                        let rect: DomRectReadOnly = entry.content_rect();
                        width.set(rect.width());
                        height.set(rect.height());
                    }
                }));
            let Ok(observer) = ResizeObserver::new(closure.as_ref().unchecked_ref()) else {
                return;
            };
            let target: NodeRef<Element> = self.get_target().clone();
            let observed: ResizeObserver = observer.clone();
            after_mount(move || {
                if let Some(element) = target.get_cloned() {
                    observed.observe(&element);
                }
            });
            let disconnect: Rc<RefCell<Option<ResizeObserver>>> =
                Rc::new(RefCell::new(Some(observer)));
            *self.get_teardown().borrow_mut() = Some(Box::new(move || {
                if let Some(observer) = disconnect.borrow_mut().take() {
                    observer.disconnect();
                }
            }));
        }
    }

    /// Disconnects the observer.
    pub(crate) fn detach(&self) {
        let taken: Option<Box<dyn Fn()>> = self.get_teardown().borrow_mut().take();
        if let Some(teardown) = taken {
            teardown();
        }
    }
}
