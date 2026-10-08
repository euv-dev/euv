use super::*;

/// Implements cache query and management for `AssetCache`.
impl AssetCache {
    /// Returns the state of the asset with the given URL, or `None` if not cached.
    ///
    /// # Arguments
    ///
    /// - `U` - The `url` asset URL, any type that dereferences to a string.
    ///
    /// # Returns
    ///
    /// - `Option<AssetState>` - The asset state, or `None`.
    pub fn get_state<U>(&self, url: U) -> Option<AssetState>
    where
        U: AsRef<str>,
    {
        self.get_entries()
            .get(url.as_ref())
            .map(|entry: &AssetEntry| entry.get_state())
    }

    /// Returns the loaded image for the given URL, or `None` if not loaded.
    ///
    /// # Arguments
    ///
    /// - `U` - The `url` asset URL, any type that dereferences to a string.
    ///
    /// # Returns
    ///
    /// - `Option<HtmlImageElement>` - The loaded image, or `None`.
    pub fn get_image<U>(&self, url: U) -> Option<HtmlImageElement>
    where
        U: AsRef<str>,
    {
        let entry: &AssetEntry = self.get_entries().get(url.as_ref())?;
        if entry.get_state() != AssetState::Loaded {
            return None;
        }
        entry.try_get_image()
    }

    /// Returns `true` if all assets in the cache have finished loading.
    ///
    /// # Returns
    ///
    /// - `bool` - True if no assets are in the `Loading` state.
    pub fn is_all_loaded(&self) -> bool {
        self.get_entries()
            .values()
            .all(|entry: &AssetEntry| entry.get_state() != AssetState::Loading)
    }

    /// Returns the number of assets that have been successfully loaded.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of loaded assets.
    pub fn loaded_count(&self) -> usize {
        self.get_entries()
            .values()
            .filter(|entry: &&AssetEntry| entry.get_state() == AssetState::Loaded)
            .count()
    }

    /// Removes all entries from the cache.
    pub fn clear(&mut self) {
        self.get_mut_entries().clear();
    }
}

/// Implements `Default` for `AssetCache` as a new empty cache.
impl Default for AssetCache {
    /// Constructs a default [`AssetCache`] value.
    ///
    /// # Returns
    ///
    /// - `AssetCache` - A default-constructed instance with the documented initial state.
    fn default() -> AssetCache {
        AssetCache::new()
    }
}

/// Implements asynchronous asset loading for `AssetLoader`.
impl AssetLoader {
    /// Begins loading an image asset from the given URL.
    ///
    /// Creates an `HtmlImageElement`, sets its `src`, and registers `onload`/`onerror`
    /// callbacks to update the shared cache state. The image loads asynchronously.
    ///
    /// Both callbacks decrement the shared pending counter and mark their own
    /// closure slot settled. The closures themselves are released later by
    /// [`AssetLoader::collect`], which is the only place a `Closure` may be
    /// dropped safely — see [`AssetClosures`] for why a load callback cannot
    /// free itself.
    ///
    /// # Arguments
    ///
    /// - `String` - The URL of the image to load.
    pub fn load_image(&mut self, url: String) {
        let Ok(image) = HtmlImageElement::new() else {
            return;
        };
        let entry: AssetEntry = AssetEntry::new(
            AssetType::Image,
            AssetState::Loading,
            Some(image.clone()),
            url.clone(),
        );
        self.get_cache()
            .get_mut()
            .get_mut_entries()
            .insert(url.clone(), entry);
        *self.get_pending().get_mut() += 1;
        let onload_slot: usize = self.get_closures().get().slots.len();
        let onerror_slot: usize = onload_slot + 1;
        let pending_shared: AssetPending = self.get_pending().clone();
        let pending: AssetPending = pending_shared.clone();
        let store_shared: Weak<EngineCell<AssetClosureStore>> = Rc::downgrade(self.get_closures());
        let store_weak: Weak<EngineCell<AssetClosureStore>> = store_shared.clone();
        let cache_clone: Rc<EngineCell<AssetCache>> = self.get_cache().clone();
        let url_for_onload: String = url.clone();
        let onload_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            {
                let cache_ref: &mut AssetCache = cache_clone.get_mut();
                if let Some(mut entry) = cache_ref.get_entries().get(&url_for_onload).cloned() {
                    entry.set_state(AssetState::Loaded);
                    cache_ref
                        .get_mut_entries()
                        .insert(url_for_onload.clone(), entry);
                }
            }
            *pending.get_mut() = pending.get().saturating_sub(1);
            mark_asset_closure_settled(&store_weak, onload_slot);
        }));
        let cache_clone_err: Rc<EngineCell<AssetCache>> = self.get_cache().clone();
        let pending_err: AssetPending = pending_shared.clone();
        let store_weak_err: Weak<EngineCell<AssetClosureStore>> = store_shared.clone();
        let url_for_onerror: String = url.clone();
        let onerror_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            {
                let cache_ref: &mut AssetCache = cache_clone_err.get_mut();
                if let Some(mut entry) = cache_ref.get_entries().get(&url_for_onerror).cloned() {
                    entry.set_state(AssetState::Error);
                    cache_ref
                        .get_mut_entries()
                        .insert(url_for_onerror.clone(), entry);
                }
            }
            *pending_err.get_mut() = pending_err.get().saturating_sub(1);
            mark_asset_closure_settled(&store_weak_err, onerror_slot);
        }));
        image.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
        image.set_onerror(Some(onerror_closure.as_ref().unchecked_ref()));
        image.set_src(&url);
        let store: &mut AssetClosureStore = self.get_closures().get_mut();
        store.slots.push(Some(onload_closure));
        store.settled.push(false);
        let store: &mut AssetClosureStore = self.get_closures().get_mut();
        store.slots.push(Some(onerror_closure));
        store.settled.push(false);
    }

    /// Returns the number of loads that have been requested but not settled.
    ///
    /// # Returns
    ///
    /// - `u32` - The in-flight load count.
    pub fn pending_count(&self) -> u32 {
        *self.get_pending().get()
    }

    /// Advances the loader by one engine update step.
    ///
    /// Implements [`Updatable`] so an `AssetLoader` can be registered with the
    /// scheduler's [`TaskRegistry`] and driven on every fixed step. The only
    /// work is [`AssetLoader::collect`], which is the safe point to release
    /// load callbacks that have already run.
    ///
    /// # Arguments
    ///
    /// - `f64` - The fixed delta time in seconds, unused.
    pub fn update(&mut self, delta_time: f64) {
        let _: f64 = delta_time;
        self.collect();
    }

    /// Releases the load callbacks that have already run.
    ///
    /// A `Closure` must not be dropped while JavaScript is
    /// executing it, so [`AssetLoader::load_image`] callbacks only mark their
    /// own slot settled. This method is the safe drop point: it must be called
    /// from a context that is not inside a load callback, such as the
    /// engine's update step.
    ///
    /// Slots that have not settled belong to loads still in flight and are
    /// kept alive, so only settled slots are released. The `onload` and
    /// `onerror` closures of the same asset settle independently — a
    /// successful load settles only its `onload` slot — so the paired slot is
    /// released once both halves have run.
    pub fn collect(&mut self) {
        let store: &mut AssetClosureStore = self.get_closures().get_mut();
        let mut index: usize = 0;
        while index < store.slots.len() {
            let settled: bool = store.settled.get(index).copied().unwrap_or(false);
            if settled {
                store.slots[index] = None;
                store.settled[index] = false;
            }
            index += 1;
        }
    }

    /// Returns whether all requested assets have finished loading.
    ///
    /// # Returns
    ///
    /// - `bool` - True if no assets are pending.
    pub fn is_all_loaded(&self) -> bool {
        self.get_cache().get().is_all_loaded()
    }

    /// Returns the loaded image for the given URL.
    ///
    /// # Arguments
    ///
    /// - `U` - The `url` asset URL, any type that dereferences to a string.
    ///
    /// # Returns
    ///
    /// - `Option<HtmlImageElement>` - The loaded image, or `None`.
    pub fn get_image<U>(&self, url: U) -> Option<HtmlImageElement>
    where
        U: AsRef<str>,
    {
        self.get_cache().get().get_image(url.as_ref())
    }

    /// Returns the progress ratio of loaded assets.
    ///
    /// # Returns
    ///
    /// - `f64` - The ratio in the range 0.0 to 1.0.
    pub fn progress(&self) -> f64 {
        let cache_ref: &AssetCache = self.get_cache().get();
        let total: usize = cache_ref.get_entries().len();
        if total == 0 {
            return 1.0;
        }
        cache_ref.loaded_count() as f64 / total as f64
    }
}

/// Forwards `AssetLoader::update` through the [`Updatable`] trait so a loader
/// can be registered with the scheduler's [`TaskRegistry`] and collect its
/// finished load callbacks on every fixed step. The inherent
/// [`AssetLoader::update`] method is the canonical implementation; this impl
/// exists purely for trait dispatch.
impl Updatable for AssetLoader {
    /// Advances the loader by `delta_time` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - Seconds elapsed since the previous update.
    fn update(&mut self, delta_time: f64) {
        AssetLoader::update(self, delta_time);
    }
}

/// Implements `Default` for `AssetLoader` as a new empty loader.
impl Default for AssetLoader {
    /// Constructs a default [`AssetLoader`] value.
    ///
    /// # Returns
    ///
    /// - `AssetLoader` - A default-constructed instance with the documented initial state.
    fn default() -> AssetLoader {
        let mut loader: AssetLoader = AssetLoader::new();
        loader.set_cache(Rc::new(EngineCell::new(AssetCache::default())));
        loader.set_pending(Rc::new(EngineCell::new(0)));
        loader.set_closures(Rc::new(EngineCell::new(AssetClosureStore::default())));
        loader
    }
}

/// Implements static asset creation utilities for `AssetLoader`.
impl AssetLoader {
    /// Creates an `HtmlImageElement` from the given URL without caching.
    ///
    /// The image loads asynchronously. Returns immediately with the image element
    /// whose `src` is set but may not have finished loading yet.
    ///
    /// # Arguments
    ///
    /// - `U` - The `url` image URL, any type that dereferences to a string.
    ///
    /// # Returns
    ///
    /// - `Option<HtmlImageElement>` - The image element, or `None` if creation failed.
    pub fn create_image_element<U>(url: U) -> Option<HtmlImageElement>
    where
        U: AsRef<str>,
    {
        let image: HtmlImageElement = HtmlImageElement::new().ok()?;
        image.set_src(url.as_ref());
        Some(image)
    }
}
