use super::*;

/// Core mixing helper for `SelectorFxHasher`.
impl SelectorFxHasher {
    /// Mixes a 64-bit word into the running FxHash state.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hasher reference.
    /// - `u64` - Word to fold into the hash accumulator.
    #[inline(always)]
    fn mix_word(&mut self, word: u64) {
        let Self { hash } = self;
        *hash = (hash.rotate_left(5) ^ word).wrapping_mul(FX_HASH_MULTIPLIER);
    }
}

/// Fast non-cryptographic `Hasher` implementation for `SelectorFxHasher`.
impl Hasher for SelectorFxHasher {
    /// Finalizes and returns the 64-bit hash digest.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Hasher reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Final 64-bit FxHash digest.
    #[inline(always)]
    fn finish(&self) -> u64 {
        let Self { hash } = *self;
        hash
    }

    /// Feeds an arbitrary byte slice into the FxHash accumulator in 8/4/2/1-byte chunks.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hasher reference.
    /// - `&[u8]` - Input byte slice.
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        while bytes.len() >= 8 {
            let chunk: [u8; 8] = [
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ];
            self.mix_word(u64::from_ne_bytes(chunk));
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            let chunk: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
            self.mix_word(u64::from(u32::from_ne_bytes(chunk)));
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            let chunk: [u8; 2] = [bytes[0], bytes[1]];
            self.mix_word(u64::from(u16::from_ne_bytes(chunk)));
            bytes = &bytes[2..];
        }
        if let Some(&last) = bytes.first() {
            self.mix_word(u64::from(last));
        }
    }

    /// Feeds a `u8` directly into the FxHash accumulator.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hasher reference.
    /// - `u8` - Input byte.
    #[inline(always)]
    fn write_u8(&mut self, i: u8) {
        self.mix_word(u64::from(i));
    }

    /// Feeds a `u16` directly into the FxHash accumulator.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hasher reference.
    /// - `u16` - Input 16-bit integer.
    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        self.mix_word(u64::from(i));
    }

    /// Feeds a `u32` directly into the FxHash accumulator.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hasher reference.
    /// - `u32` - Input 32-bit integer.
    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        self.mix_word(u64::from(i));
    }

    /// Feeds a `u64` directly into the FxHash accumulator.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hasher reference.
    /// - `u64` - Input 64-bit integer.
    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        self.mix_word(i);
    }

    /// Feeds a `usize` directly into the FxHash accumulator.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hasher reference.
    /// - `usize` - Input pointer-sized integer.
    #[inline(always)]
    fn write_usize(&mut self, i: usize) {
        self.mix_word(i as u64);
    }

    /// Feeds an `i32` directly into the FxHash accumulator.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hasher reference.
    /// - `i32` - Input signed 32-bit integer.
    #[inline(always)]
    fn write_i32(&mut self, i: i32) {
        self.mix_word(i as u64);
    }

    /// Feeds an `i64` directly into the FxHash accumulator.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hasher reference.
    /// - `i64` - Input signed 64-bit integer.
    #[inline(always)]
    fn write_i64(&mut self, i: i64) {
        self.mix_word(i as u64);
    }

    /// Feeds an `isize` directly into the FxHash accumulator.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable hasher reference.
    /// - `isize` - Input signed pointer-sized integer.
    #[inline(always)]
    fn write_isize(&mut self, i: isize) {
        self.mix_word(i as u64);
    }
}

/// Fast open-addressed hash table operations for `SelectorTable<K>`.
impl<K> SelectorTable<K>
where
    K: Eq + Hash,
{
    /// Creates an empty table with zero heap allocations until the first insert.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty selector lookup table.
    #[must_use]
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self {
            entries: Vec::new(),
            len: 0,
        }
    }

    /// Computes the 64-bit FxHash digest for `key`.
    ///
    /// # Arguments
    ///
    /// - `&K` - Key reference to hash.
    ///
    /// # Returns
    ///
    /// - `u64` - Computed 64-bit hash value.
    #[must_use]
    #[inline(always)]
    pub(crate) fn hash_key(key: &K) -> u64 {
        let mut hasher: SelectorFxHasher = SelectorFxHasher::default();
        key.hash(&mut hasher);
        hasher.finish()
    }

    /// Looks up the `Signal<bool>` registered for `key`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Table reference.
    /// - `&K` - Key to look up.
    ///
    /// # Returns
    ///
    /// - `Option<Signal<bool>>` - Registered signal if present.
    #[must_use]
    #[inline(always)]
    pub(crate) fn get(&self, key: &K) -> Option<Signal<bool>> {
        let Self { entries, .. } = self;
        if entries.is_empty() {
            return None;
        }
        let hash: u64 = Self::hash_key(key);
        self.get_with_hash(key, hash)
    }

    /// Looks up `key` using a precomputed FxHash digest `hash`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Table reference.
    /// - `&K` - Key to look up.
    /// - `u64` - Precomputed 64-bit FxHash digest of `key`.
    ///
    /// # Returns
    ///
    /// - `Option<Signal<bool>>` - Registered signal if present.
    #[must_use]
    #[inline(always)]
    pub(crate) fn get_with_hash(&self, key: &K, hash: u64) -> Option<Signal<bool>> {
        let Self { entries, .. } = self;
        let cap: usize = entries.len();
        if cap == 0 {
            return None;
        }
        let mask: usize = cap - 1;
        let mut idx: usize = (hash as usize) & mask;
        loop {
            let slot: &SelectorEntry<K> = entries[idx].as_ref()?;
            if slot.hash == hash && slot.key == *key {
                return Some(slot.signal);
            }
            idx = (idx + 1) & mask;
        }
    }

    /// Inserts or updates `(key, signal)` using a precomputed FxHash digest `hash`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable table reference.
    /// - `K` - Key to insert.
    /// - `u64` - Precomputed 64-bit FxHash digest of `key`.
    /// - `Signal<bool>` - Boolean selection signal for `key`.
    pub(crate) fn insert_with_hash(&mut self, key: K, hash: u64, signal: Signal<bool>) {
        let Self { entries, len } = self;
        let cur_cap: usize = entries.len();
        if cur_cap == 0 {
            *entries = (0..SELECTOR_INITIAL_BUCKETS).map(|_: usize| None).collect();
        } else if (*len + 1) * 10 >= cur_cap * 7 {
            let next_cap: usize = cur_cap.saturating_mul(2);
            let mut next_entries: Vec<Option<SelectorEntry<K>>> =
                (0..next_cap).map(|_: usize| None).collect();
            let next_mask: usize = next_cap - 1;
            for entry in entries.drain(..).flatten() {
                let mut idx: usize = (entry.hash as usize) & next_mask;
                while next_entries[idx].is_some() {
                    idx = (idx + 1) & next_mask;
                }
                next_entries[idx] = Some(entry);
            }
            *entries = next_entries;
        }
        let mask: usize = entries.len() - 1;
        let mut idx: usize = (hash as usize) & mask;
        loop {
            match &mut entries[idx] {
                Some(slot) => {
                    if slot.hash == hash && slot.key == key {
                        slot.signal = signal;
                        return;
                    }
                    idx = (idx + 1) & mask;
                }
                slot @ None => {
                    *slot = Some(SelectorEntry { hash, key, signal });
                    *len += 1;
                    return;
                }
            }
        }
    }

    /// Drains all stored per-key signals and resets the table to empty.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable table reference.
    ///
    /// # Returns
    ///
    /// - `Vec<Signal<bool>>` - All previously registered boolean selection signals.
    pub(crate) fn drain_signals(&mut self) -> Vec<Signal<bool>> {
        let Self { entries, len } = self;
        let mut out: Vec<Signal<bool>> = Vec::with_capacity(*len);
        for entry in entries.drain(..).flatten() {
            out.push(entry.signal);
        }
        *len = 0;
        out
    }
}

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
        let state: Rc<RefCell<SelectorState<K>>> = Rc::new(RefCell::new(SelectorState {
            active_key: None,
            active_sig: None,
            table: SelectorTable::new(),
        }));
        let state_clone: Rc<RefCell<SelectorState<K>>> = state.clone();
        let watcher: Effect = Effect::new(move || {
            let next_key: K = source();
            let (old_id, new_id): (Option<SignalId>, Option<SignalId>) = state_clone
                .try_borrow_mut()
                .ok()
                .map(|mut guard: RefMut<'_, SelectorState<K>>| {
                    let SelectorState {
                        active_key,
                        active_sig,
                        table,
                    } = &mut *guard;
                    if active_key.as_ref() == Some(&next_key) {
                        return (None, None);
                    }
                    let old_sig: Option<Signal<bool>> = *active_sig;
                    let new_sig: Option<Signal<bool>> = table.get(&next_key);
                    *active_key = Some(next_key);
                    *active_sig = new_sig;
                    (
                        old_sig.map(|sig: Signal<bool>| sig.id()),
                        new_sig.map(|sig: Signal<bool>| sig.id()),
                    )
                })
                .unwrap_or((None, None));
            if old_id.is_none() && new_id.is_none() {
                return;
            }
            let should_flush: bool = with_runtime(
                |rt: &mut ReactiveRuntime| rt.write_selector_pair(old_id, new_id),
                false,
            );
            if should_flush {
                flush_effects();
            }
        });
        Self { state, watcher }
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
        let Self { state, .. } = self;
        let hash: u64 = SelectorTable::<K>::hash_key(&key);
        let sig: Signal<bool> = match state.try_borrow_mut() {
            Ok(mut guard) => {
                let SelectorState {
                    active_key,
                    active_sig,
                    table,
                } = &mut *guard;
                if let Some(found) = table.get_with_hash(&key, hash) {
                    found
                } else {
                    let is_active: bool = active_key.as_ref().is_some_and(|cur: &K| *cur == key);
                    let created: Signal<bool> = Signal::new(is_active);
                    table.insert_with_hash(key, hash, created);
                    if is_active {
                        *active_sig = Some(created);
                    }
                    created
                }
            }
            Err(_) => return false,
        };
        sig.get()
    }

    /// Disposes the selector's watcher effect and per-key signals.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Selector to dispose.
    pub fn dispose(&self) {
        let Self { state, watcher } = self;
        watcher.dispose();
        let signals: Vec<Signal<bool>> = state
            .try_borrow_mut()
            .ok()
            .map(|mut guard: RefMut<'_, SelectorState<K>>| {
                let SelectorState {
                    active_key,
                    active_sig,
                    table,
                } = &mut *guard;
                *active_key = None;
                *active_sig = None;
                table.drain_signals()
            })
            .unwrap_or_default();
        if !signals.is_empty() {
            with_runtime(
                |rt: &mut ReactiveRuntime| {
                    for sig in signals.into_iter() {
                        rt.dispose_signal(sig.id());
                    }
                },
                (),
            );
        }
    }
}
