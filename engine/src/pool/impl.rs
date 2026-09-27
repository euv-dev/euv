use super::*;

/// Implements construction, checkout, return, and introspection for
/// [`ObjectPool`].
///
/// Checkout is LIFO on the free list (see [`ObjectPool`] for the rationale),
/// and every successful `acquire` pairs with exactly one `release`, so the
/// active count returns to zero once all outstanding values are back.
impl<T> ObjectPool<T> {
    /// Constructs a pool holding `initial` ready-to-use values.
    ///
    /// The seeded values land on the free list, so a pool built this way
    /// satisfies its first `initial.len()` acquires without a factory.
    ///
    /// # Arguments
    ///
    /// - `Vec<T>` - The values to seed the free list with.
    ///
    /// # Returns
    ///
    /// - `ObjectPool<T>` - The new pool.
    pub fn new(initial: Vec<T>) -> ObjectPool<T> {
        ObjectPool {
            active: 0,
            free: initial,
        }
    }

    /// Constructs an empty pool.
    ///
    /// The free list is reserved to [`POOL_DEFAULT_PREWARM`] so a default
    /// pool starts with a pre-sized free list; a prewarm size of zero
    /// reserves nothing and the first release grows the list.
    ///
    /// # Returns
    ///
    /// - `ObjectPool<T>` - The new pool.
    pub fn empty() -> ObjectPool<T> {
        ObjectPool {
            active: 0,
            free: Vec::with_capacity(POOL_DEFAULT_PREWARM),
        }
    }

    /// Returns the number of values currently checked out of the pool.
    ///
    /// Named `get_active` rather than `active` so it does not collide with
    /// the field of the same name.
    ///
    /// # Returns
    ///
    /// - `usize` - The active count.
    pub fn get_active(&self) -> usize {
        self.active
    }

    /// Sets the number of tracked active values.
    ///
    /// Only for pool owners that track checkouts out of band; `acquire` and
    /// `release` maintain the count themselves.
    ///
    /// # Arguments
    ///
    /// - `usize` - The new active count.
    pub fn set_active(&mut self, active: usize) {
        self.active = active;
    }

    /// Returns the values the pool owns and can hand out.
    ///
    /// # Returns
    ///
    /// - `&Vec<T>` - The free list, most-recently-released first.
    pub fn get_free(&self) -> &Vec<T> {
        &self.free
    }

    /// Returns a mutable reference to the free list.
    ///
    /// # Returns
    ///
    /// - `&mut Vec<T>` - The free list, for bulk seeding before first use.
    pub fn get_mut_free(&mut self) -> &mut Vec<T> {
        &mut self.free
    }

    /// Returns the number of values ready to be handed out.
    ///
    /// # Returns
    ///
    /// - `usize` - The free list length.
    pub fn available(&self) -> usize {
        self.get_free().len()
    }

    /// Returns the total number of values the pool tracks, active plus free.
    ///
    /// This is the pool's high-water mark: it grows only when a factory
    /// builds a value the pool could not serve, and never shrinks on a
    /// release.
    ///
    /// # Returns
    ///
    /// - `usize` - The tracked value count.
    pub fn len(&self) -> usize {
        self.get_active() + self.available()
    }

    /// Returns whether the pool tracks no values at all.
    ///
    /// # Returns
    ///
    /// - `bool` - True when both the active count and the free list are empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks a pooled value out, or reports that the free list is empty.
    ///
    /// This is the factory-free form: it never builds a new value, so it
    /// reports `None` on a miss. Use [`Self::acquire_with`] when the pool
    /// should grow on demand.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - A recycled value, or `None` if none was available.
    pub fn acquire(&mut self) -> Option<T> {
        let value: Option<T> = self.get_mut_free().pop();
        if value.is_some() {
            let active: usize = self.get_active() + 1;
            self.set_active(active);
        }
        value
    }

    /// Checks a pooled value out, calling `make` only when the free list is
    /// empty.
    ///
    /// A recycled value is returned verbatim; the factory runs at most once
    /// per acquire and its result is handed to the caller, so the pool never
    /// duplicates the value's allocation.
    ///
    /// # Arguments
    ///
    /// - `F` - Factory building a fresh value on a miss.
    ///
    /// # Returns
    ///
    /// - `T` - The recycled value, or a freshly built one.
    pub fn acquire_with<F>(&mut self, mut make: F) -> T
    where
        F: FnMut() -> T,
    {
        // Phase 1: serve from the free list when a value is parked there.
        if self.available() > 0
            && let Some(value) = self.acquire()
        {
            return value;
        }
        // Phase 2: grow on demand; a fresh value is still an outstanding checkout.
        let value: T = make();
        let active: usize = self.get_active() + 1;
        self.set_active(active);
        value
    }

    /// Returns a value to the free list without destroying it.
    ///
    /// The value is moved onto the free list, so the allocation behind it
    /// survives for the next acquire. The active count only decrements while
    /// a checkout is outstanding, which keeps the count honest even when a
    /// caller returns a value it never acquired.
    ///
    /// # Arguments
    ///
    /// - `T` - The value to make available again.
    pub fn release(&mut self, value: T) {
        let active: usize = self.get_active().saturating_sub(1);
        self.set_active(active);
        self.get_mut_free().push(value);
    }

    /// Builds `count` values up front and returns how many are available.
    ///
    /// Prewarming moves the factory cost off the first frames of a spawn
    /// loop; the pool then serves that many acquires without calling `make`
    /// again.
    ///
    /// # Arguments
    ///
    /// - `usize` - How many values to build.
    /// - `F` - Factory building each new value.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of values now available on the free list.
    pub fn prewarm<F>(&mut self, count: usize, mut make: F) -> usize
    where
        F: FnMut() -> T,
    {
        for _ in 0..count {
            let value: T = make();
            self.get_mut_free().push(value);
        }
        self.available()
    }

    /// Discards every free value and resets the active count.
    ///
    /// # Returns
    ///
    /// - `usize` - How many values were discarded from the free list.
    pub fn clear(&mut self) -> usize {
        let discarded: usize = self.available();
        self.get_mut_free().clear();
        self.set_active(0);
        discarded
    }
}

/// Implements [`Default`] for [`ObjectPool`] as a new empty pool.
impl<T> Default for ObjectPool<T> {
    /// Constructs a default [`ObjectPool`] value.
    ///
    /// # Returns
    ///
    /// - `ObjectPool<T>` - A default-constructed instance with the documented initial state.
    fn default() -> ObjectPool<T> {
        ObjectPool::empty()
    }
}

/// Implements [`Debug`] for [`ObjectPool`] showing counts rather than values.
///
/// `T` is deliberately not required to implement [`Debug`], so a pool of
/// closure or handle types still prints usefully.
impl<T> Debug for ObjectPool<T> {
    /// Formats the [`ObjectPool`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ObjectPool")
            .field("active", &self.active)
            .field("available", &self.free.len())
            .finish()
    }
}
