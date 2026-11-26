use super::*;

/// Keyed O(1) selection operations for `Selector<K>`.
impl<K> Selector<K>
where
    K: Clone + Eq + Hash + 'static,
{
    /// Creates a selector tracking the key returned by `source`.
    ///
    /// # Arguments
    ///
    /// - `F` - Reactive key producer closure.
    ///
    /// # Returns
    ///
    /// - `Self` - Selector instance supporting O(1) per-key selection checks.
    #[must_use]
    pub fn new<F>(mut source: F) -> Self
    where
        F: FnMut() -> K + 'static,
    {
        let active: Rc<RefCell<Option<K>>> = Rc::new(RefCell::new(None));
        let subscribers: Rc<RefCell<HashMap<K, Signal<bool>>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let active_clone: Rc<RefCell<Option<K>>> = active.clone();
        let subs_clone: Rc<RefCell<HashMap<K, Signal<bool>>>> = subscribers.clone();
        let watcher: Effect = Effect::new(move || {
            let next_key: K = source();
            let prev_key: Option<K> = active_clone
                .try_borrow_mut()
                .ok()
                .and_then(|mut guard: RefMut<'_, Option<K>>| guard.replace(next_key.clone()));
            if prev_key.as_ref() == Some(&next_key) {
                return;
            }
            let (old_sig, new_sig): (Option<Signal<bool>>, Option<Signal<bool>>) = subs_clone
                .try_borrow()
                .ok()
                .map(|map: Ref<'_, HashMap<K, Signal<bool>>>| {
                    let old_s: Option<Signal<bool>> =
                        prev_key.as_ref().and_then(|k: &K| map.get(k).copied());
                    let new_s: Option<Signal<bool>> = map.get(&next_key).copied();
                    (old_s, new_s)
                })
                .unwrap_or((None, None));
            match (old_sig, new_sig) {
                (Some(old_s), Some(new_s)) => {
                    batch(|| {
                        let _: bool = old_s.set(false);
                        let _: bool = new_s.set(true);
                    });
                }
                (Some(old_s), None) => {
                    let _: bool = old_s.set(false);
                }
                (None, Some(new_s)) => {
                    let _: bool = new_s.set(true);
                }
                (None, None) => {}
            }
        });
        Self {
            active,
            subscribers,
            watcher,
        }
    }

    /// Reactively returns whether `key` is currently selected, subscribing only to `key`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Selector reference.
    /// - `K` - Key to test for active selection.
    ///
    /// # Returns
    ///
    /// - `bool` - True when `key` equals the active selected key.
    #[must_use]
    pub fn is_selected(&self, key: K) -> bool {
        let Self {
            active,
            subscribers,
            ..
        } = self;
        let existing: Option<Signal<bool>> = subscribers
            .try_borrow()
            .ok()
            .and_then(|map: Ref<'_, HashMap<K, Signal<bool>>>| map.get(&key).copied());
        let sig: Signal<bool> = match existing {
            Some(found) => found,
            None => {
                let is_active: bool = active
                    .try_borrow()
                    .ok()
                    .and_then(|guard: Ref<'_, Option<K>>| guard.as_ref().map(|cur: &K| *cur == key))
                    .unwrap_or(false);
                let created: Signal<bool> = Signal::new(is_active);
                if let Ok(mut map) = subscribers.try_borrow_mut() {
                    map.insert(key, created);
                }
                created
            }
        };
        sig.get()
    }

    /// Disposes the selector's watcher effect and per-key signals.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Selector to dispose.
    pub fn dispose(&self) {
        let Self {
            subscribers,
            watcher,
            ..
        } = self;
        watcher.dispose();
        let signals: Vec<Signal<bool>> = subscribers
            .try_borrow_mut()
            .ok()
            .map(|mut map: RefMut<'_, HashMap<K, Signal<bool>>>| {
                map.drain().map(|(_, s): (K, Signal<bool>)| s).collect()
            })
            .unwrap_or_default();
        for sig in signals.into_iter() {
            sig.dispose();
        }
    }
}
