use super::*;

/// Manual `Clone` implementation so `Memo<T>` is always `Copy`.
impl<T> Clone for Memo<T>
where
    T: 'static,
{
    /// Copies the lightweight 8-byte memo handle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Memo handle to copy.
    ///
    /// # Returns
    ///
    /// - `Self` - Identical memo handle.
    fn clone(&self) -> Self {
        *self
    }
}

/// Marker `Copy` implementation for `Memo<T>`.
impl<T> Copy for Memo<T> where T: 'static {}

/// Equality comparison by underlying `MemoId`.
impl<T> PartialEq for Memo<T>
where
    T: 'static,
{
    /// Compares two memo handles by arena identifier.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Left-hand memo handle.
    /// - `&Self` - Right-hand memo handle.
    ///
    /// # Returns
    ///
    /// - `bool` - True if both handles reference the same memo slot and generation.
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

/// Reflexive equality for `Memo<T>`.
impl<T> Eq for Memo<T> where T: 'static {}

/// Identity accessors for `Memo<T>`.
impl<T> Memo<T>
where
    T: 'static,
{
    /// Returns the underlying `MemoId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Memo handle.
    ///
    /// # Returns
    ///
    /// - `MemoId` - Underlying arena identifier.
    #[must_use]
    pub fn id(&self) -> MemoId {
        let Self { id, .. } = *self;
        id
    }
}

/// Core reactive memoization operations for `Memo<T>`.
impl<T> Memo<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Creates a new memoized derived computation with value-equality short-circuiting.
    ///
    /// # Arguments
    ///
    /// - `F` - Computation closure evaluated lazily when upstream dependencies change.
    ///
    /// # Returns
    ///
    /// - `Self` - Copyable handle to the memo.
    #[must_use]
    pub fn new<F>(mut compute: F) -> Self
    where
        F: FnMut() -> T + 'static,
    {
        let id: MemoId = with_runtime(
            |rt: &mut ReactiveRuntime| {
                rt.create_memo(move |slot: &mut Option<Box<dyn Any>>| {
                    let next_val: T = compute();
                    if let Some(prev_val) = slot
                        .as_mut()
                        .and_then(|boxed: &mut Box<dyn Any>| boxed.downcast_mut::<T>())
                    {
                        if *prev_val == next_val {
                            return false;
                        }
                        *prev_val = next_val;
                        return true;
                    }
                    *slot = Some(Box::new(next_val));
                    true
                })
            },
            MemoId::default(),
        );
        evaluate_memo(id);
        Self {
            id,
            marker: PhantomData,
        }
    }

    /// Evaluates the memo if stale, tracks it in the current observer, and returns a cloned value.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Memo handle to read.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - Current computed value if alive.
    #[must_use]
    pub fn try_get(&self) -> Option<T> {
        read_memo_fused::<T>(self.id())
    }

    /// Evaluates the memo if stale and returns the value or `T::default()` if disposed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Memo handle to read.
    ///
    /// # Returns
    ///
    /// - `T` - Computed value or default fallback.
    #[must_use]
    pub fn get(&self) -> T
    where
        T: Default,
    {
        self.try_get().unwrap_or_default()
    }

    /// Disposes this memo and detaches its dependency links.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Memo handle to dispose.
    pub fn dispose(&self) {
        let id: MemoId = self.id();
        with_runtime(
            |rt: &mut ReactiveRuntime| {
                rt.dispose_memo(id);
            },
            (),
        );
    }
}
