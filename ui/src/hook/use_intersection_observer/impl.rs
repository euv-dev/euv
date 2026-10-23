use super::*;

/// Inherent implementation of [`UseIntersectionObserver`].
impl UseIntersectionObserver {
    /// Creates a handle with the visibility starting at `false`.
    ///
    /// # Returns
    ///
    /// - `UseIntersectionObserver` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            target: NodeRef::new(),
            visible: Signal::create(false),
            ratio: Signal::create(0.0),
            teardown: Rc::new(RefCell::new(None)),
        }
    }

    /// Returns the current visibility as a snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the element is intersecting the viewport.
    pub fn is_visible(&self) -> bool {
        self.get_visible().get()
    }

    /// Returns the current visible fraction as a snapshot.
    ///
    /// # Returns
    ///
    /// - `f64` - The visible fraction, from 0 to 1.
    pub fn current_ratio(&self) -> f64 {
        self.get_ratio().get()
    }

    /// Creates the observer and starts watching the element.
    ///
    /// The element does not exist while the first render is being
    /// built, so the `observe` call is deferred until after the DOM
    /// has been committed.
    pub(crate) fn attach(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let visible: Signal<bool> = self.get_visible();
            let ratio: Signal<f64> = self.get_ratio();
            let closure: Closure<dyn FnMut(Array)> =
                Closure::wrap(Box::new(move |entries: Array| {
                    for entry in entries.iter() {
                        let Ok(entry) = entry.dyn_into::<IntersectionObserverEntry>() else {
                            continue;
                        };
                        visible.set(entry.is_intersecting());
                        ratio.set(entry.intersection_ratio());
                    }
                }));
            let Ok(observer) = IntersectionObserver::new(closure.as_ref().unchecked_ref()) else {
                return;
            };
            let target: NodeRef<Element> = self.get_target().clone();
            let observed: IntersectionObserver = observer.clone();
            after_mount(move || {
                if let Some(element) = target.get_cloned() {
                    observed.observe(&element);
                }
            });
            let disconnect: Rc<RefCell<Option<IntersectionObserver>>> =
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
