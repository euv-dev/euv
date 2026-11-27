use super::*;

/// Inline small-list operations for reactive subscriber and dependency collections.
impl<T> SmallSubList<T>
where
    T: Copy + PartialEq,
{
    /// Returns `true` when zero entries are stored.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Small list reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the list contains no items.
    #[must_use]
    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::One(_) | Self::Two(_, _) => false,
            Self::Many(vec) => vec.is_empty(),
        }
    }

    /// Returns the number of stored entries.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Small list reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of items currently stored.
    #[must_use]
    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::One(_) => 1,
            Self::Two(_, _) => 2,
            Self::Many(vec) => vec.len(),
        }
    }

    /// Returns the entry at `index` if in bounds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Small list reference.
    /// - `usize` - Zero-based item index.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - Copied item at `index`, or `None` when out of bounds.
    #[must_use]
    #[inline(always)]
    pub(crate) fn get(&self, index: usize) -> Option<T> {
        match self {
            Self::Empty => None,
            Self::One(first) => {
                if index == 0 {
                    Some(*first)
                } else {
                    None
                }
            }
            Self::Two(first, second) => match index {
                0 => Some(*first),
                1 => Some(*second),
                _ => None,
            },
            Self::Many(vec) => vec.get(index).copied(),
        }
    }

    /// Checks whether `target` already exists within the prefix `0..end`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Small list reference.
    /// - `usize` - Exclusive upper index bound of the prefix.
    /// - `T` - Entry to search for.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `target` appears before index `end`.
    #[must_use]
    #[inline(always)]
    pub(crate) fn contains_prefix(&self, end: usize, target: T) -> bool {
        if end == 0 {
            return false;
        }
        match self {
            Self::Empty => false,
            Self::One(first) => *first == target,
            Self::Two(first, second) => {
                if *first == target {
                    true
                } else {
                    end > 1 && *second == target
                }
            }
            Self::Many(vec) => {
                let bound: usize = end.min(vec.len());
                vec[..bound].contains(&target)
            }
        }
    }

    /// Finds the first index `>= start` whose entry equals `target`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Small list reference.
    /// - `usize` - Inclusive start index.
    /// - `T` - Entry to locate.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - Matching index if found at or after `start`.
    #[must_use]
    #[inline(always)]
    pub(crate) fn find_from(&self, start: usize, target: T) -> Option<usize> {
        match self {
            Self::Empty => None,
            Self::One(first) => {
                if start == 0 && *first == target {
                    Some(0)
                } else {
                    None
                }
            }
            Self::Two(first, second) => {
                if start == 0 && *first == target {
                    Some(0)
                } else if start <= 1 && *second == target {
                    Some(1)
                } else {
                    None
                }
            }
            Self::Many(vec) => {
                let mut idx: usize = start;
                while idx < vec.len() {
                    if vec[idx] == target {
                        return Some(idx);
                    }
                    idx += 1;
                }
                None
            }
        }
    }

    /// Swaps two in-bounds entries at `a` and `b`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable small list reference.
    /// - `usize` - First element index.
    /// - `usize` - Second element index.
    #[inline(always)]
    pub(crate) fn swap(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        match self {
            Self::Empty | Self::One(_) => {}
            Self::Two(first, second) => {
                if (a == 0 && b == 1) || (a == 1 && b == 0) {
                    swap(first, second);
                }
            }
            Self::Many(vec) => {
                if a < vec.len() && b < vec.len() {
                    vec.swap(a, b);
                }
            }
        }
    }

    /// Appends `value` assuming uniqueness has already been established by the caller.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable small list reference.
    /// - `T` - Unique entry to append.
    #[inline(always)]
    pub(crate) fn push_unchecked(&mut self, value: T) {
        match self {
            Self::Empty => {
                *self = Self::One(value);
            }
            Self::One(first) => {
                *self = Self::Two(*first, value);
            }
            Self::Two(first, second) => {
                let mut list: Vec<T> = Vec::with_capacity(INITIAL_SUBSCRIBER_CAPACITY);
                list.push(*first);
                list.push(*second);
                list.push(value);
                *self = Self::Many(list);
            }
            Self::Many(vec) => {
                vec.push(value);
            }
        }
    }

    /// Splits off and returns all entries at indices `>= keep_len`, keeping `0..keep_len` in place.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable small list reference.
    /// - `usize` - Number of leading entries to retain.
    ///
    /// # Returns
    ///
    /// - `Self` - Removed tail entries.
    #[inline(always)]
    pub(crate) fn take_tail(&mut self, keep_len: usize) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::One(first) => {
                if keep_len == 0 {
                    let item: T = *first;
                    *self = Self::Empty;
                    Self::One(item)
                } else {
                    Self::Empty
                }
            }
            Self::Two(first, second) => match keep_len {
                0 => {
                    let a: T = *first;
                    let b: T = *second;
                    *self = Self::Empty;
                    Self::Two(a, b)
                }
                1 => {
                    let a: T = *first;
                    let b: T = *second;
                    *self = Self::One(a);
                    Self::One(b)
                }
                _ => Self::Empty,
            },
            Self::Many(vec) => {
                if keep_len >= vec.len() {
                    return Self::Empty;
                }
                let removed: usize = vec.len() - keep_len;
                if removed == 1
                    && let Some(item) = vec.pop()
                {
                    return Self::One(item);
                }
                if removed == 2
                    && let Some(second) = vec.pop()
                    && let Some(first) = vec.pop()
                {
                    return Self::Two(first, second);
                }
                Self::Many(vec.split_off(keep_len))
            }
        }
    }

    /// Appends `value` if it is not already present, promoting inline storage to `Many` only on the third unique item.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable small list reference.
    /// - `T` - Entry to insert uniquely.
    #[inline(always)]
    pub(crate) fn push_unique(&mut self, value: T) {
        match self {
            Self::Empty => {
                *self = Self::One(value);
            }
            Self::One(first) => {
                if *first != value {
                    *self = Self::Two(*first, value);
                }
            }
            Self::Two(first, second) => {
                if *first != value && *second != value {
                    let mut list: Vec<T> = Vec::with_capacity(INITIAL_SUBSCRIBER_CAPACITY);
                    list.push(*first);
                    list.push(*second);
                    list.push(value);
                    *self = Self::Many(list);
                }
            }
            Self::Many(vec) => {
                if !vec.contains(&value) {
                    vec.push(value);
                }
            }
        }
    }

    /// Removes every occurrence of `target` while preserving insertion order of remaining entries.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable small list reference.
    /// - `T` - Entry value to remove.
    #[inline(always)]
    pub(crate) fn remove_item(&mut self, target: T) {
        match self {
            Self::Empty => {}
            Self::One(first) => {
                if *first == target {
                    *self = Self::Empty;
                }
            }
            Self::Two(first, second) => {
                if *first == target {
                    *self = Self::One(*second);
                } else if *second == target {
                    *self = Self::One(*first);
                }
            }
            Self::Many(vec) => {
                vec.retain(|item: &T| *item != target);
            }
        }
    }

    /// Empties all stored items while retaining heap capacity if already promoted to `Many`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable small list reference.
    #[inline(always)]
    pub(crate) fn clear(&mut self) {
        match self {
            Self::Empty => {}
            Self::One(_) | Self::Two(_, _) => {
                *self = Self::Empty;
            }
            Self::Many(vec) => {
                vec.clear();
            }
        }
    }

    /// Visits every stored item in insertion order without allocating.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Small list reference.
    /// - `F` - Visitor closure receiving each copied item.
    #[inline(always)]
    pub(crate) fn for_each<F>(&self, mut visit: F)
    where
        F: FnMut(T),
    {
        match self {
            Self::Empty => {}
            Self::One(first) => {
                visit(*first);
            }
            Self::Two(first, second) => {
                visit(*first);
                visit(*second);
            }
            Self::Many(vec) => {
                for item in vec.iter().copied() {
                    visit(item);
                }
            }
        }
    }

    /// Drains and visits every stored item in insertion order while retaining `Many` heap capacity.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable small list reference.
    /// - `F` - Visitor closure receiving each drained item.
    #[inline(always)]
    pub(crate) fn drain_each<F>(&mut self, mut visit: F)
    where
        F: FnMut(T),
    {
        match self {
            Self::Empty => {}
            Self::One(first) => {
                let item: T = *first;
                *self = Self::Empty;
                visit(item);
            }
            Self::Two(first, second) => {
                let a: T = *first;
                let b: T = *second;
                *self = Self::Empty;
                visit(a);
                visit(b);
            }
            Self::Many(vec) => {
                for item in vec.drain(..) {
                    visit(item);
                }
            }
        }
    }
}

/// Panic-safe tracking guard helpers.
impl TrackingGuard {
    /// Creates an armed `TrackingGuard` protecting the newly pushed observer frame.
    ///
    /// # Returns
    ///
    /// - `Self` - Armed unwind guard.
    #[must_use]
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self { active: true }
    }

    /// Disarms the guard before normal completion pops the tracking frame explicitly.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable guard reference.
    #[inline(always)]
    pub(crate) fn disarm(&mut self) {
        let Self { active } = self;
        *active = false;
    }
}

/// Dirty-propagation target extraction for subscriber small-lists.
impl SmallSubList<SubscriberId> {
    /// Extracts a zero-allocation copyable dispatch target for dirty propagation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Subscriber list reference.
    ///
    /// # Returns
    ///
    /// - `DirtyTarget` - Inline target for 0..=2 subscribers or `Many` heap fallback.
    #[must_use]
    #[inline(always)]
    pub(crate) fn dirty_target(&self) -> DirtyTarget {
        match self {
            Self::Empty => DirtyTarget::Empty,
            Self::One(sub) => DirtyTarget::One(*sub),
            Self::Two(first, second) => DirtyTarget::Two(*first, *second),
            Self::Many(vec) => {
                if vec.is_empty() {
                    DirtyTarget::Empty
                } else {
                    DirtyTarget::Many
                }
            }
        }
    }
}

/// Unwind cleanup popping any leaked observer frame if a reactive computation panics.
impl Drop for TrackingGuard {
    /// Pops the leaked frame from `observer_stack` when unwinding while still armed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable guard being dropped.
    fn drop(&mut self) {
        let Self { active } = self;
        if *active {
            with_runtime(
                |rt: &mut ReactiveRuntime| {
                    let _: Option<Option<(SubscriberId, usize)>> =
                        rt.get_mut_observer_stack().pop();
                },
                (),
            );
        }
    }
}

/// Default initialization for `ReactiveRuntime`.
impl Default for ReactiveRuntime {
    /// Creates an empty reactive runtime with preallocated arena buffers.
    ///
    /// # Returns
    ///
    /// - `Self` - The initialized runtime instance.
    fn default() -> Self {
        Self {
            signals: Vec::with_capacity(INITIAL_ARENA_CAPACITY),
            free_signals: Vec::new(),
            memos: Vec::with_capacity(INITIAL_ARENA_CAPACITY),
            free_memos: Vec::new(),
            effects: Vec::with_capacity(INITIAL_ARENA_CAPACITY),
            free_effects: Vec::new(),
            scopes: Vec::with_capacity(INITIAL_ARENA_CAPACITY),
            free_scopes: Vec::new(),
            observer_stack: Vec::new(),
            scope_stack: Vec::new(),
            pending_effects: Vec::new(),
            dirty_queue: Vec::with_capacity(INITIAL_SUBSCRIBER_CAPACITY),
            check_scratch: Vec::new(),
            flush_batch_scratch: Vec::new(),
            batch_depth: 0,
            flushing: false,
            clock: 0,
            global_states: HashMap::new(),
            named_global_states: HashMap::new(),
            event_channels: HashMap::new(),
        }
    }
}

/// Core arena operations for `ReactiveRuntime`.
impl ReactiveRuntime {
    /// Allocates a new signal slot storing `initial_value`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `T` - Initial signal value.
    ///
    /// # Returns
    ///
    /// - `SignalId` - Handle for the newly created signal.
    pub(crate) fn create_signal<T>(&mut self, initial_value: T) -> SignalId
    where
        T: 'static,
    {
        let boxed: Box<dyn Any> = Box::new(initial_value);
        let id: SignalId = if let Some(index) = self.get_mut_free_signals().pop() {
            let entry: &mut SignalEntry = &mut self.get_mut_signals()[index as usize];
            let next_gen: u32 = entry.get_generation().wrapping_add(1);
            entry.set_generation(next_gen);
            entry.set_version(0);
            entry.set_value(Some(boxed));
            entry.get_mut_subscribers().clear();
            SignalId::from_slot(SlotId::new(index, next_gen))
        } else {
            let index: u32 = self.get_signals().len() as u32;
            self.get_mut_signals().push(SignalEntry {
                generation: 1,
                version: 0,
                value: Some(boxed),
                subscribers: SmallSubList::Empty,
            });
            SignalId::from_slot(SlotId::new(index, 1))
        };
        if let Some(scope_id) = self.get_scope_stack().last().copied() {
            self.attach_signal_to_scope(scope_id, id);
        }
        id
    }

    /// Allocates a new root-lifetime signal in the arena without binding it to the active transient `Scope`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reactive runtime.
    /// - `T` - Initial signal value.
    ///
    /// # Returns
    ///
    /// - `SignalId` - Newly allocated detached signal identifier.
    pub(crate) fn create_detached_signal<T>(&mut self, initial_value: T) -> SignalId
    where
        T: 'static,
    {
        let boxed: Box<dyn Any> = Box::new(initial_value);
        if let Some(index) = self.get_mut_free_signals().pop() {
            let entry: &mut SignalEntry = &mut self.get_mut_signals()[index as usize];
            let next_gen: u32 = entry.get_generation().wrapping_add(1);
            entry.set_generation(next_gen);
            entry.set_version(0);
            entry.set_value(Some(boxed));
            entry.get_mut_subscribers().clear();
            SignalId::from_slot(SlotId::new(index, next_gen))
        } else {
            let index: u32 = self.get_signals().len() as u32;
            self.get_mut_signals().push(SignalEntry {
                generation: 1,
                version: 0,
                value: Some(boxed),
                subscribers: SmallSubList::Empty,
            });
            SignalId::from_slot(SlotId::new(index, 1))
        }
    }

    /// Registers or replaces a root-lifetime global state signal keyed by `TypeId::of::<T>()`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reactive runtime.
    /// - `T` - Initial or replacement global state value.
    ///
    /// # Returns
    ///
    /// - `(SignalId, bool)` - Tuple of `(signal_id, should_flush)` when updating an existing slot.
    pub(crate) fn provide_global_state<T>(&mut self, initial: T) -> (SignalId, bool)
    where
        T: 'static,
    {
        let key: TypeId = TypeId::of::<T>();
        if let Some(existing_id) = self.get_global_states().get(&key).copied() {
            let slot: SlotId = existing_id.slot();
            let alive: bool = self
                .get_signals()
                .get(slot.as_usize())
                .map(|e: &SignalEntry| {
                    e.get_generation() == slot.get_generation() && e.value.is_some()
                })
                .unwrap_or(false);
            if alive {
                let (_, flush): (bool, bool) = self.write_signal(existing_id, initial);
                return (existing_id, flush);
            }
        }
        let id: SignalId = self.create_detached_signal(initial);
        let _: Option<SignalId> = self.get_mut_global_states().insert(key, id);
        (id, false)
    }

    /// Looks up a registered root-lifetime global state signal keyed by `TypeId::of::<T>()`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reactive runtime reference.
    ///
    /// # Returns
    ///
    /// - `Option<SignalId>` - Registered signal handle if present.
    pub(crate) fn lookup_global_state<T>(&self) -> Option<SignalId>
    where
        T: 'static,
    {
        self.get_global_states().get(&TypeId::of::<T>()).copied()
    }

    /// Looks up a registered global state signal or initializes it lazily with `init`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reactive runtime.
    /// - `F` - Lazy initializer invoked only when absent.
    ///
    /// # Returns
    ///
    /// - `SignalId` - Existing or newly created global state signal handle.
    pub(crate) fn get_or_init_global_state<T, F>(&mut self, init: F) -> SignalId
    where
        T: 'static,
        F: FnOnce() -> T,
    {
        let key: TypeId = TypeId::of::<T>();
        if let Some(existing_id) = self.get_global_states().get(&key).copied() {
            return existing_id;
        }
        let id: SignalId = self.create_detached_signal(init());
        let _: Option<SignalId> = self.get_mut_global_states().insert(key, id);
        id
    }

    /// Registers or replaces a string-keyed root-lifetime named global state signal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reactive runtime.
    /// - `&str` - Global state channel name.
    /// - `T` - Initial or replacement value.
    ///
    /// # Returns
    ///
    /// - `(SignalId, bool)` - Tuple of `(signal_id, should_flush)`.
    pub(crate) fn provide_named_global_state<T>(
        &mut self,
        name: &str,
        initial: T,
    ) -> (SignalId, bool)
    where
        T: 'static,
    {
        if let Some(existing_id) = self.get_named_global_states().get(name).copied() {
            let slot: SlotId = existing_id.slot();
            let alive: bool = self
                .get_signals()
                .get(slot.as_usize())
                .map(|e: &SignalEntry| {
                    e.get_generation() == slot.get_generation() && e.value.is_some()
                })
                .unwrap_or(false);
            if alive {
                let (_, flush): (bool, bool) = self.write_signal(existing_id, initial);
                return (existing_id, flush);
            }
        }
        let id: SignalId = self.create_detached_signal(initial);
        let _: Option<SignalId> = self
            .get_mut_named_global_states()
            .insert(name.to_string(), id);
        (id, false)
    }

    /// Looks up a string-keyed named global state signal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reactive runtime reference.
    /// - `&str` - Global state channel name.
    ///
    /// # Returns
    ///
    /// - `Option<SignalId>` - Registered signal handle if present.
    pub(crate) fn lookup_named_global_state(&self, name: &str) -> Option<SignalId> {
        self.get_named_global_states().get(name).copied()
    }

    /// Looks up or allocates the detached event channel signal for custom event type `E`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reactive runtime.
    ///
    /// # Returns
    ///
    /// - `SignalId` - Detached `Signal<(u64, Option<E>)>` channel identifier.
    pub(crate) fn ensure_event_channel<E>(&mut self) -> SignalId
    where
        E: CustomEvent,
    {
        let key: TypeId = TypeId::of::<E>();
        if let Some(existing_id) = self.get_event_channels().get(&key).copied() {
            return existing_id;
        }
        let initial: (u64, Option<E>) = (0, None);
        let id: SignalId = self.create_detached_signal(initial);
        let _: Option<SignalId> = self.get_mut_event_channels().insert(key, id);
        id
    }

    /// Reads a signal value via cloning inside the arena and records dependency tracking.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SignalId` - Signal identifier to read.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - Cloned signal value if the slot is alive and matches type `T`.
    #[inline]
    pub(crate) fn read_signal_cloned<T>(&mut self, id: SignalId) -> Option<T>
    where
        T: Clone + 'static,
    {
        if !self.get_observer_stack().is_empty() {
            self.track_dependency(DependencyId::Signal(id));
        }
        let slot: SlotId = id.slot();
        let entry: &SignalEntry = self.get_signals().get(slot.as_usize())?;
        if entry.get_generation() != slot.get_generation() {
            return None;
        }
        let boxed: &Box<dyn Any> = entry.value.as_ref()?;
        boxed.downcast_ref::<T>().cloned()
    }

    /// Reads a signal value without recording any reactive dependency.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reference to the reactive runtime.
    /// - `SignalId` - Signal identifier to inspect.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - Cloned signal value if alive and matching type `T`.
    #[inline]
    pub(crate) fn read_signal_untracked<T>(&self, id: SignalId) -> Option<T>
    where
        T: Clone + 'static,
    {
        let slot: SlotId = id.slot();
        let entry: &SignalEntry = self.get_signals().get(slot.as_usize())?;
        if entry.get_generation() != slot.get_generation() {
            return None;
        }
        let boxed: &Box<dyn Any> = entry.value.as_ref()?;
        boxed.downcast_ref::<T>().cloned()
    }

    /// Returns whether a live signal slot currently has one or more active subscribers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reference to the reactive runtime.
    /// - `SignalId` - Signal identifier to inspect.
    ///
    /// # Returns
    ///
    /// - `bool` - True when the signal is alive and observed by at least one subscriber.
    #[inline]
    pub(crate) fn signal_has_subscribers(&self, id: SignalId) -> bool {
        let slot: SlotId = id.slot();
        self.get_signals()
            .get(slot.as_usize())
            .map(|entry: &SignalEntry| {
                entry.get_generation() == slot.get_generation()
                    && entry.value.is_some()
                    && !entry.get_subscribers().is_empty()
            })
            .unwrap_or(false)
    }

    /// Writes a new signal value in place and propagates dirty notifications without heap allocations.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SignalId` - Signal identifier to update.
    /// - `T` - Replacement value.
    ///
    /// # Returns
    ///
    /// - `(bool, bool)` - Tuple `(updated, should_flush)` indicating whether the signal was alive and whether effects are pending.
    #[inline]
    pub(crate) fn write_signal<T>(&mut self, id: SignalId, value: T) -> (bool, bool)
    where
        T: 'static,
    {
        let slot: SlotId = id.slot();
        let idx: usize = slot.as_usize();
        let Some(entry) = self.get_mut_signals().get_mut(idx) else {
            return (false, false);
        };
        let stale: bool = entry.get_generation() != slot.get_generation();
        if stale || entry.value.is_none() {
            return (false, false);
        }
        if let Some(existing) = entry
            .value
            .as_mut()
            .and_then(|boxed: &mut Box<dyn Any>| boxed.downcast_mut::<T>())
        {
            *existing = value;
        } else {
            entry.set_value(Some(Box::new(value)));
        }
        let next_ver: u64 = entry.get_version().wrapping_add(1);
        entry.set_version(next_ver);
        let target: DirtyTarget = entry.get_subscribers().dirty_target();
        let next_clock: u64 = self.get_clock().wrapping_add(1);
        self.set_clock(next_clock);
        if matches!(target, DirtyTarget::Empty) {
            return (true, false);
        }
        self.propagate_signal_dirty(idx, target);
        let should_flush: bool = self.get_batch_depth() == 0
            && !self.get_flushing()
            && !self.get_pending_effects().is_empty();
        (true, should_flush)
    }

    /// Takes a live signal's boxed payload for zero-clone in-place mutation outside the runtime borrow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SignalId` - Signal identifier whose payload should be temporarily taken.
    ///
    /// # Returns
    ///
    /// - `Option<Box<dyn Any>>` - Existing boxed value when the slot is alive and matches type `T`.
    #[inline]
    pub(crate) fn take_signal_box<T>(&mut self, id: SignalId) -> Option<Box<dyn Any>>
    where
        T: 'static,
    {
        let slot: SlotId = id.slot();
        let entry: &mut SignalEntry = self.get_mut_signals().get_mut(slot.as_usize())?;
        if entry.get_generation() != slot.get_generation() {
            return None;
        }
        if !entry
            .value
            .as_ref()
            .is_some_and(|boxed: &Box<dyn Any>| boxed.is::<T>())
        {
            return None;
        }
        entry.value.take()
    }

    /// Restores a mutated signal payload box, bumps version and clock, and returns immediately when unobserved.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SignalId` - Signal identifier receiving its mutated payload.
    /// - `Box<dyn Any>` - Mutated payload box previously taken via `take_signal_box`.
    ///
    /// # Returns
    ///
    /// - `(bool, bool)` - Tuple `(updated, should_flush)` indicating whether the signal was alive and whether effects must flush.
    #[inline]
    pub(crate) fn commit_signal_box(&mut self, id: SignalId, boxed: Box<dyn Any>) -> (bool, bool) {
        let slot: SlotId = id.slot();
        let idx: usize = slot.as_usize();
        let Some(entry) = self.get_mut_signals().get_mut(idx) else {
            return (false, false);
        };
        if entry.get_generation() != slot.get_generation() {
            return (false, false);
        }
        entry.set_value(Some(boxed));
        let next_ver: u64 = entry.get_version().wrapping_add(1);
        entry.set_version(next_ver);
        let target: DirtyTarget = entry.get_subscribers().dirty_target();
        let next_clock: u64 = self.get_clock().wrapping_add(1);
        self.set_clock(next_clock);
        if matches!(target, DirtyTarget::Empty) {
            return (true, false);
        }
        self.propagate_signal_dirty(idx, target);
        let should_flush: bool = self.get_batch_depth() == 0
            && !self.get_flushing()
            && !self.get_pending_effects().is_empty();
        (true, should_flush)
    }

    /// Writes a signal value and propagates dirtiness without checking flush state, returning whether subscribers existed.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SignalId` - Signal identifier to update.
    /// - `T` - Replacement value.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the signal was alive and had at least one downstream subscriber.
    #[inline]
    fn write_signal_no_flush<T>(&mut self, id: SignalId, value: T) -> bool
    where
        T: 'static,
    {
        let slot: SlotId = id.slot();
        let idx: usize = slot.as_usize();
        let Some(entry) = self.get_mut_signals().get_mut(idx) else {
            return false;
        };
        let stale: bool = entry.get_generation() != slot.get_generation();
        if stale || entry.value.is_none() {
            return false;
        }
        if let Some(existing) = entry
            .value
            .as_mut()
            .and_then(|boxed: &mut Box<dyn Any>| boxed.downcast_mut::<T>())
        {
            *existing = value;
        } else {
            entry.set_value(Some(Box::new(value)));
        }
        let next_ver: u64 = entry.get_version().wrapping_add(1);
        entry.set_version(next_ver);
        let target: DirtyTarget = entry.get_subscribers().dirty_target();
        let next_clock: u64 = self.get_clock().wrapping_add(1);
        self.set_clock(next_clock);
        if matches!(target, DirtyTarget::Empty) {
            return false;
        }
        self.propagate_signal_dirty(idx, target);
        true
    }

    /// Atomically updates an optional deselected and newly selected `Selector` signal pair in one runtime borrow.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `Option<SignalId>` - Previously selected key signal to set to `false`, if any.
    /// - `Option<SignalId>` - Newly selected key signal to set to `true`, if any.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when at least one signal had subscribers and effects must be flushed now.
    pub(crate) fn write_selector_pair(
        &mut self,
        old_id: Option<SignalId>,
        new_id: Option<SignalId>,
    ) -> bool {
        let mut any_subs: bool = false;
        if let Some(id) = old_id
            && self.write_signal_no_flush(id, false)
        {
            any_subs = true;
        }
        if let Some(id) = new_id
            && self.write_signal_no_flush(id, true)
        {
            any_subs = true;
        }
        any_subs
            && self.get_batch_depth() == 0
            && !self.get_flushing()
            && !self.get_pending_effects().is_empty()
    }

    /// Writes a new signal value in place only when it differs from the stored value.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SignalId` - Signal identifier to update.
    /// - `T` - Candidate value compared using `PartialEq`.
    ///
    /// # Returns
    ///
    /// - `(bool, bool)` - Tuple `(changed, should_flush)` indicating whether the value changed and whether effects are pending.
    #[inline]
    pub(crate) fn write_signal_if_changed<T>(&mut self, id: SignalId, value: T) -> (bool, bool)
    where
        T: PartialEq + 'static,
    {
        let slot: SlotId = id.slot();
        let idx: usize = slot.as_usize();
        let Some(entry) = self.get_mut_signals().get_mut(idx) else {
            return (false, false);
        };
        if entry.get_generation() != slot.get_generation() {
            return (false, false);
        }
        let Some(current) = entry
            .value
            .as_mut()
            .and_then(|boxed: &mut Box<dyn Any>| boxed.downcast_mut::<T>())
        else {
            return (false, false);
        };
        if *current == value {
            return (false, false);
        }
        *current = value;
        let next_ver: u64 = entry.get_version().wrapping_add(1);
        entry.set_version(next_ver);
        let target: DirtyTarget = entry.get_subscribers().dirty_target();
        let next_clock: u64 = self.get_clock().wrapping_add(1);
        self.set_clock(next_clock);
        if matches!(target, DirtyTarget::Empty) {
            return (true, false);
        }
        self.propagate_signal_dirty(idx, target);
        let should_flush: bool = self.get_batch_depth() == 0
            && !self.get_flushing()
            && !self.get_pending_effects().is_empty();
        (true, should_flush)
    }

    /// Allocates a new memo slot with an in-place type-erased computation closure.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `F` - Memo computation closure mutating the cached value in place and returning `changed`.
    ///
    /// # Returns
    ///
    /// - `MemoId` - Handle for the newly created memo.
    pub(crate) fn create_memo<F>(&mut self, compute: F) -> MemoId
    where
        F: FnMut(&mut Option<Box<dyn Any>>) -> bool + 'static,
    {
        let depth: u32 = (self.get_observer_stack().len() as u32).saturating_add(1);
        let compute_rc: MemoComputeFn = Rc::new(RefCell::new(compute));
        let id: MemoId = if let Some(index) = self.get_mut_free_memos().pop() {
            let entry: &mut MemoEntry = &mut self.get_mut_memos()[index as usize];
            let next_gen: u32 = entry.get_generation().wrapping_add(1);
            entry.set_generation(next_gen);
            entry.set_version(0);
            entry.set_state(NodeState::Dirty);
            entry.set_value(None);
            entry.set_compute(Some(compute_rc));
            entry.get_mut_dependencies().clear();
            entry.get_mut_subscribers().clear();
            entry.set_depth(depth);
            MemoId::from_slot(SlotId::new(index, next_gen))
        } else {
            let index: u32 = self.get_memos().len() as u32;
            self.get_mut_memos().push(MemoEntry {
                generation: 1,
                version: 0,
                state: NodeState::Dirty,
                value: None,
                compute: Some(compute_rc),
                dependencies: SmallSubList::Empty,
                subscribers: SmallSubList::Empty,
                depth,
            });
            MemoId::from_slot(SlotId::new(index, 1))
        };
        if let Some(scope_id) = self.get_scope_stack().last().copied() {
            self.attach_memo_to_scope(scope_id, id);
        }
        id
    }

    /// Allocates a new effect slot and schedules it for immediate execution.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `F` - Effect callback closure.
    ///
    /// # Returns
    ///
    /// - `EffectId` - Handle for the newly created effect.
    pub(crate) fn create_effect<F>(&mut self, callback: F) -> EffectId
    where
        F: FnMut() + 'static,
    {
        let depth: u32 = (self.get_observer_stack().len() as u32).saturating_add(1);
        let callback_rc: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(callback));
        let id: EffectId = if let Some(index) = self.get_mut_free_effects().pop() {
            let entry: &mut EffectEntry = &mut self.get_mut_effects()[index as usize];
            let next_gen: u32 = entry.get_generation().wrapping_add(1);
            entry.set_generation(next_gen);
            entry.set_alive(true);
            entry.set_scheduled(false);
            entry.set_callback(Some(callback_rc));
            entry.get_mut_dependencies().clear();
            entry.set_depth(depth);
            entry.set_frozen_depth(0);
            entry.set_pending_while_frozen(false);
            EffectId::from_slot(SlotId::new(index, next_gen))
        } else {
            let index: u32 = self.get_effects().len() as u32;
            self.get_mut_effects().push(EffectEntry {
                generation: 1,
                alive: true,
                scheduled: false,
                callback: Some(callback_rc),
                dependencies: SmallSubList::Empty,
                depth,
                frozen_depth: 0,
                pending_while_frozen: false,
            });
            EffectId::from_slot(SlotId::new(index, 1))
        };
        if let Some(scope_id) = self.get_scope_stack().last().copied() {
            self.attach_effect_to_scope(scope_id, id);
        }
        id
    }

    /// Creates a new ownership scope and optionally links it to the active parent scope.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    ///
    /// # Returns
    ///
    /// - `ScopeId` - Identifier for the newly created scope.
    pub(crate) fn create_scope(&mut self) -> ScopeId {
        let id: ScopeId = if let Some(index) = self.get_mut_free_scopes().pop() {
            let entry: &mut ScopeEntry = &mut self.get_mut_scopes()[index as usize];
            let next_gen: u32 = entry.get_generation().wrapping_add(1);
            entry.set_generation(next_gen);
            entry.set_alive(true);
            entry.set_frozen(false);
            entry.get_mut_signals().clear();
            entry.get_mut_memos().clear();
            entry.get_mut_effects().clear();
            entry.get_mut_children().clear();
            entry.get_mut_cleanups().clear();
            ScopeId::from_slot(SlotId::new(index, next_gen))
        } else {
            let index: u32 = self.get_scopes().len() as u32;
            self.get_mut_scopes().push(ScopeEntry {
                generation: 1,
                alive: true,
                frozen: false,
                signals: Vec::new(),
                memos: Vec::new(),
                effects: Vec::new(),
                children: Vec::new(),
                cleanups: Vec::new(),
            });
            ScopeId::from_slot(SlotId::new(index, 1))
        };
        if let Some(parent_id) = self.get_scope_stack().last().copied() {
            let parent_slot: SlotId = parent_id.slot();
            if let Some(parent) = self.get_mut_scopes().get_mut(parent_slot.as_usize())
                && parent.get_generation() == parent_slot.get_generation()
                && parent.get_alive()
            {
                parent.get_mut_children().push(id);
            }
        }
        id
    }

    /// Registers a cleanup callback on the currently active scope.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `F` - Cleanup closure invoked when the active scope disposes.
    pub(crate) fn register_cleanup<F>(&mut self, cleanup: F)
    where
        F: FnOnce() + 'static,
    {
        if let Some(scope_id) = self.get_scope_stack().last().copied() {
            let slot: SlotId = scope_id.slot();
            if let Some(entry) = self.get_mut_scopes().get_mut(slot.as_usize())
                && entry.get_generation() == slot.get_generation()
                && entry.get_alive()
            {
                entry.get_mut_cleanups().push(Box::new(cleanup));
            }
        }
    }

    /// Records `dependency` on the currently executing observer using O(1) cursor reuse on stable graphs.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `DependencyId` - The dependency accessed during evaluation.
    pub(crate) fn track_dependency(&mut self, dependency: DependencyId) {
        let Some(Some((subscriber, cursor))) = self.get_observer_stack().last().copied() else {
            return;
        };
        let reuse_outcome: Option<usize> = match subscriber {
            SubscriberId::Memo(memo_id) => {
                let slot: SlotId = memo_id.slot();
                let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize()) else {
                    return;
                };
                if entry.get_generation() != slot.get_generation() {
                    return;
                }
                let deps: &mut SmallSubList<DependencyId> = entry.get_mut_dependencies();
                if deps.get(cursor) == Some(dependency) {
                    Some(cursor + 1)
                } else if deps.contains_prefix(cursor, dependency) {
                    return;
                } else if let Some(found_idx) = deps.find_from(cursor + 1, dependency) {
                    deps.swap(cursor, found_idx);
                    Some(cursor + 1)
                } else {
                    None
                }
            }
            SubscriberId::Effect(effect_id) => {
                let slot: SlotId = effect_id.slot();
                let Some(entry) = self.get_mut_effects().get_mut(slot.as_usize()) else {
                    return;
                };
                if entry.get_generation() != slot.get_generation() {
                    return;
                }
                let deps: &mut SmallSubList<DependencyId> = entry.get_mut_dependencies();
                if deps.get(cursor) == Some(dependency) {
                    Some(cursor + 1)
                } else if deps.contains_prefix(cursor, dependency) {
                    return;
                } else if let Some(found_idx) = deps.find_from(cursor + 1, dependency) {
                    deps.swap(cursor, found_idx);
                    Some(cursor + 1)
                } else {
                    None
                }
            }
        };
        if let Some(next_cursor) = reuse_outcome {
            if let Some(Some((_, top_cursor))) = self.get_mut_observer_stack().last_mut() {
                *top_cursor = next_cursor;
            }
            return;
        }
        self.detach_tail_dependencies(subscriber, cursor);
        match dependency {
            DependencyId::Signal(sig_id) => {
                let slot: SlotId = sig_id.slot();
                if let Some(entry) = self.get_mut_signals().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                {
                    entry.get_mut_subscribers().push_unique(subscriber);
                }
            }
            DependencyId::Memo(memo_id) => {
                let slot: SlotId = memo_id.slot();
                if let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                {
                    entry.get_mut_subscribers().push_unique(subscriber);
                }
            }
        }
        match subscriber {
            SubscriberId::Memo(memo_id) => {
                let slot: SlotId = memo_id.slot();
                if let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                {
                    entry.get_mut_dependencies().push_unchecked(dependency);
                }
            }
            SubscriberId::Effect(effect_id) => {
                let slot: SlotId = effect_id.slot();
                if let Some(entry) = self.get_mut_effects().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                {
                    entry.get_mut_dependencies().push_unchecked(dependency);
                }
            }
        }
        if let Some(Some((_, top_cursor))) = self.get_mut_observer_stack().last_mut() {
            *top_cursor = cursor + 1;
        }
    }

    /// Pops the top tracking frame and unsubscribes any stale tail dependencies beyond its final cursor.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    pub(crate) fn finish_tracking(&mut self) {
        if let Some(Some((subscriber, cursor))) = self.get_mut_observer_stack().pop() {
            self.detach_tail_dependencies(subscriber, cursor);
        }
    }

    /// Truncates `subscriber`'s dependency list to `keep_len` and unsubscribes from any removed tail dependencies.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SubscriberId` - Observer whose tail dependencies should be trimmed.
    /// - `usize` - Number of active leading dependencies to retain.
    fn detach_tail_dependencies(&mut self, subscriber: SubscriberId, keep_len: usize) {
        let stale: SmallSubList<DependencyId> = match subscriber {
            SubscriberId::Memo(memo_id) => {
                let slot: SlotId = memo_id.slot();
                match self.get_mut_memos().get_mut(slot.as_usize()) {
                    Some(entry) if entry.get_generation() == slot.get_generation() => {
                        let deps: &mut SmallSubList<DependencyId> = entry.get_mut_dependencies();
                        if deps.len() <= keep_len {
                            return;
                        }
                        deps.take_tail(keep_len)
                    }
                    _ => return,
                }
            }
            SubscriberId::Effect(effect_id) => {
                let slot: SlotId = effect_id.slot();
                match self.get_mut_effects().get_mut(slot.as_usize()) {
                    Some(entry) if entry.get_generation() == slot.get_generation() => {
                        let deps: &mut SmallSubList<DependencyId> = entry.get_mut_dependencies();
                        if deps.len() <= keep_len {
                            return;
                        }
                        deps.take_tail(keep_len)
                    }
                    _ => return,
                }
            }
        };
        stale.for_each(|dep: DependencyId| match dep {
            DependencyId::Signal(sig_id) => {
                let slot: SlotId = sig_id.slot();
                if let Some(entry) = self.get_mut_signals().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                {
                    entry.get_mut_subscribers().remove_item(subscriber);
                }
            }
            DependencyId::Memo(memo_id) => {
                let slot: SlotId = memo_id.slot();
                if let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                {
                    entry.get_mut_subscribers().remove_item(subscriber);
                }
            }
        });
    }

    /// Drains pending effects into the reusable batch buffer while preserving `pending_effects` capacity and tracking depth order.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `Vec<(u32, EffectId)>` - Reusable caller-owned batch buffer returned from the prior flush step.
    ///
    /// # Returns
    ///
    /// - `FlushStep` - Next single effect, batch with sortedness flag, or `FlushStep::Done`.
    pub(crate) fn drain_pending_flush_step(
        &mut self,
        mut batch: Vec<(u32, EffectId)>,
    ) -> FlushStep {
        let pending_len: usize = self.get_pending_effects().len();
        if pending_len == 0 {
            if batch.capacity() > self.get_flush_batch_scratch().capacity() {
                self.set_flush_batch_scratch(batch);
            }
            self.set_flushing(false);
            return FlushStep::Done;
        }
        if pending_len == 1 {
            if batch.capacity() > self.get_flush_batch_scratch().capacity() {
                self.set_flush_batch_scratch(batch);
            }
            if let Some(eff_id) = self.get_mut_pending_effects().pop() {
                return FlushStep::Single(eff_id);
            }
            self.set_flushing(false);
            return FlushStep::Done;
        }
        if batch.capacity() < self.get_flush_batch_scratch().capacity() {
            batch = take(self.get_mut_flush_batch_scratch());
        }
        let Self {
            effects,
            pending_effects,
            ..
        } = self;
        batch.clear();
        if batch.capacity() < pending_effects.len() {
            batch.reserve(pending_effects.len() - batch.capacity());
        }
        let mut prev_depth: u32 = 0;
        let mut is_sorted: bool = true;
        for eff_id in pending_effects.drain(..) {
            let slot: SlotId = eff_id.slot();
            if let Some(entry) = effects.get(slot.as_usize())
                && entry.get_generation() == slot.get_generation()
                && entry.get_alive()
            {
                let depth: u32 = entry.get_depth();
                if depth < prev_depth {
                    is_sorted = false;
                }
                prev_depth = depth;
                batch.push((depth, eff_id));
            }
        }
        if batch.is_empty() {
            if batch.capacity() > self.get_flush_batch_scratch().capacity() {
                self.set_flush_batch_scratch(batch);
            }
            self.set_flushing(false);
            FlushStep::Done
        } else {
            FlushStep::Batch(batch, is_sorted)
        }
    }

    /// Extracts upstream `MemoId` dependencies for `NodeState::Check` memo slot `memo_idx` without heap allocation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `usize` - Memo arena slot index.
    ///
    /// # Returns
    ///
    /// - `Option<MemoCheckDeps>` - Snapshot of upstream memo dependencies, or `None` when zero upstream memos exist.
    pub(crate) fn collect_check_memos(&mut self, memo_idx: usize) -> Option<MemoCheckDeps> {
        let Self {
            memos,
            check_scratch,
            ..
        } = self;
        let entry: &MemoEntry = memos.get(memo_idx)?;
        let mut inline_buf: [MemoId; 4] = [MemoId::default(); 4];
        let mut inline_len: usize = 0;
        let mut spill_start: Option<usize> = None;
        entry.get_dependencies().for_each(|dep: DependencyId| {
            if let DependencyId::Memo(parent_memo) = dep {
                if spill_start.is_some() {
                    check_scratch.push(parent_memo);
                } else if inline_len < inline_buf.len() {
                    inline_buf[inline_len] = parent_memo;
                    inline_len += 1;
                } else {
                    let start: usize = check_scratch.len();
                    check_scratch.extend_from_slice(&inline_buf);
                    check_scratch.push(parent_memo);
                    spill_start = Some(start);
                }
            }
        });
        if let Some(start) = spill_start {
            Some(MemoCheckDeps::Spill(start, check_scratch.len()))
        } else if inline_len == 0 {
            None
        } else {
            Some(MemoCheckDeps::Inline(inline_buf, inline_len as u8))
        }
    }

    /// Propagates dirty flags downstream from a signal using inline fast paths for 1..=2 subscribers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `usize` - Signal arena slot index (used only for `DirtyTarget::Many`).
    /// - `DirtyTarget` - Pre-extracted subscriber dispatch target.
    #[inline]
    pub(crate) fn propagate_signal_dirty(&mut self, signal_idx: usize, target: DirtyTarget) {
        match target {
            DirtyTarget::Empty => {}
            DirtyTarget::One(sub) => {
                self.mark_single_subscriber_dirty(sub, false);
            }
            DirtyTarget::Two(first, second) => {
                self.mark_single_subscriber_dirty(first, false);
                self.mark_single_subscriber_dirty(second, false);
            }
            DirtyTarget::Many => {
                self.mark_signal_subscribers_dirty(signal_idx);
            }
        }
    }

    /// Propagates dirty flags downstream from a recomputed memo using inline fast paths for 1..=2 subscribers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `usize` - Memo arena slot index (used only for `DirtyTarget::Many`).
    /// - `DirtyTarget` - Pre-extracted subscriber dispatch target.
    #[inline]
    pub(crate) fn propagate_memo_dirty(&mut self, memo_idx: usize, target: DirtyTarget) {
        match target {
            DirtyTarget::Empty => {}
            DirtyTarget::One(sub) => {
                self.mark_single_subscriber_dirty(sub, true);
            }
            DirtyTarget::Two(first, second) => {
                self.mark_single_subscriber_dirty(first, true);
                self.mark_single_subscriber_dirty(second, true);
            }
            DirtyTarget::Many => {
                self.mark_memo_subscribers_dirty(memo_idx);
            }
        }
    }

    /// Schedules a single dirty effect without touching `dirty_queue`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `EffectId` - Effect identifier to schedule or flag while frozen.
    /// - `bool` - `true` when propagating from a pulled memo recomputation.
    #[inline(always)]
    fn schedule_dirty_effect(&mut self, effect_id: EffectId, from_memo: bool) {
        let Self {
            effects,
            observer_stack,
            pending_effects,
            ..
        } = self;
        if from_memo
            && !observer_stack.is_empty()
            && observer_stack
                .iter()
                .any(|frame: &Option<(SubscriberId, usize)>| {
                    matches!(frame, Some((SubscriberId::Effect(active), _)) if *active == effect_id)
                })
        {
            return;
        }
        let slot: SlotId = effect_id.slot();
        if let Some(entry) = effects.get_mut(slot.as_usize())
            && entry.get_generation() == slot.get_generation()
            && entry.get_alive()
        {
            if entry.get_frozen_depth() > 0 {
                entry.set_pending_while_frozen(true);
            } else if !entry.get_scheduled() {
                entry.set_scheduled(true);
                pending_effects.push(effect_id);
            }
        }
    }

    /// Marks a single direct subscriber dirty and walks linear 1-subscriber memo chains in registers before spilling to `dirty_queue`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SubscriberId` - Direct downstream subscriber.
    /// - `bool` - `true` when propagating from a pulled memo recomputation.
    #[inline]
    fn mark_single_subscriber_dirty(&mut self, first_sub: SubscriberId, from_memo: bool) {
        let mut cur_sub: SubscriberId = first_sub;
        let mut target_state: NodeState = NodeState::Dirty;
        loop {
            match cur_sub {
                SubscriberId::Effect(effect_id) => {
                    self.schedule_dirty_effect(effect_id, from_memo);
                    return;
                }
                SubscriberId::Memo(memo_id) => {
                    let slot: SlotId = memo_id.slot();
                    let idx: usize = slot.as_usize();
                    let next_target: DirtyTarget = {
                        let Some(entry) = self.get_mut_memos().get_mut(idx) else {
                            return;
                        };
                        let stale: bool = entry.get_generation() != slot.get_generation();
                        if stale || entry.compute.is_none() {
                            return;
                        }
                        let cur_state: NodeState = entry.get_state();
                        let check_covered: bool =
                            cur_state == NodeState::Check && target_state == NodeState::Check;
                        if cur_state == NodeState::Dirty || check_covered {
                            return;
                        }
                        entry.set_state(target_state);
                        entry.get_subscribers().dirty_target()
                    };
                    match next_target {
                        DirtyTarget::Empty => return,
                        DirtyTarget::One(next_sub) => {
                            cur_sub = next_sub;
                            target_state = NodeState::Check;
                        }
                        DirtyTarget::Two(first, second) => {
                            let dq: &mut Vec<(SubscriberId, NodeState)> =
                                self.get_mut_dirty_queue();
                            dq.clear();
                            dq.push((first, NodeState::Check));
                            dq.push((second, NodeState::Check));
                            self.drain_dirty_queue(from_memo);
                            return;
                        }
                        DirtyTarget::Many => {
                            let Self {
                                memos, dirty_queue, ..
                            } = self;
                            dirty_queue.clear();
                            if let Some(entry) = memos.get(idx)
                                && let SmallSubList::Many(vec) = entry.get_subscribers()
                            {
                                for child_sub in vec.iter().copied() {
                                    dirty_queue.push((child_sub, NodeState::Check));
                                }
                            }
                            self.drain_dirty_queue(from_memo);
                            return;
                        }
                    }
                }
            }
        }
    }

    /// Propagates dirty flags downstream from signal slot `signal_idx` using the reusable scratch queue.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `usize` - Signal arena slot index.
    pub(crate) fn mark_signal_subscribers_dirty(&mut self, signal_idx: usize) {
        let Self {
            signals,
            dirty_queue,
            ..
        } = self;
        dirty_queue.clear();
        if let Some(entry) = signals.get(signal_idx) {
            entry
                .get_subscribers()
                .for_each(|subscriber: SubscriberId| {
                    dirty_queue.push((subscriber, NodeState::Dirty));
                });
        }
        self.drain_dirty_queue(false);
    }

    /// Propagates dirty flags downstream from memo slot `memo_idx` using the reusable scratch queue.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `usize` - Memo arena slot index.
    pub(crate) fn mark_memo_subscribers_dirty(&mut self, memo_idx: usize) {
        let Self {
            memos, dirty_queue, ..
        } = self;
        dirty_queue.clear();
        if let Some(entry) = memos.get(memo_idx) {
            entry
                .get_subscribers()
                .for_each(|subscriber: SubscriberId| {
                    dirty_queue.push((subscriber, NodeState::Dirty));
                });
        }
        self.drain_dirty_queue(true);
    }

    /// Drains the reusable `dirty_queue`, marking downstream memos and scheduling effects with zero allocations.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `bool` - `true` when propagating from a pulled memo recomputation (skipping currently executing effects on `observer_stack`).
    fn drain_dirty_queue(&mut self, from_memo: bool) {
        let Self {
            memos,
            effects,
            observer_stack,
            pending_effects,
            dirty_queue,
            ..
        } = self;
        let check_running: bool = from_memo && !observer_stack.is_empty();
        while let Some((sub, target_state)) = dirty_queue.pop() {
            match sub {
                SubscriberId::Memo(memo_id) => {
                    let slot: SlotId = memo_id.slot();
                    if let Some(entry) = memos.get_mut(slot.as_usize()) {
                        let stale: bool = entry.get_generation() != slot.get_generation();
                        if stale || entry.compute.is_none() {
                            continue;
                        }
                        let cur_state: NodeState = entry.get_state();
                        let check_covered: bool =
                            cur_state == NodeState::Check && target_state == NodeState::Check;
                        if cur_state == NodeState::Dirty || check_covered {
                            continue;
                        }
                        entry.set_state(target_state);
                        match entry.get_subscribers() {
                            SmallSubList::Empty => {}
                            SmallSubList::One(child_sub) => {
                                dirty_queue.push((*child_sub, NodeState::Check));
                            }
                            SmallSubList::Two(first, second) => {
                                dirty_queue.push((*first, NodeState::Check));
                                dirty_queue.push((*second, NodeState::Check));
                            }
                            SmallSubList::Many(vec) => {
                                for child_sub in vec.iter().copied() {
                                    dirty_queue.push((child_sub, NodeState::Check));
                                }
                            }
                        }
                    }
                }
                SubscriberId::Effect(effect_id) => {
                    if check_running
                        && observer_stack.iter().any(
                            |frame: &Option<(SubscriberId, usize)>| {
                                matches!(frame, Some((SubscriberId::Effect(active), _)) if *active == effect_id)
                            },
                        )
                    {
                        continue;
                    }
                    let slot: SlotId = effect_id.slot();
                    if let Some(entry) = effects.get_mut(slot.as_usize())
                        && entry.get_generation() == slot.get_generation()
                        && entry.get_alive()
                    {
                        if entry.get_frozen_depth() > 0 {
                            entry.set_pending_while_frozen(true);
                        } else if !entry.get_scheduled() {
                            entry.set_scheduled(true);
                            pending_effects.push(effect_id);
                        }
                    }
                }
            }
        }
    }

    /// Detaches a subscriber from all of its previously tracked upstream dependencies while preserving heap capacity.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SubscriberId` - Subscriber whose dependency links should be cleared.
    pub(crate) fn clear_dependencies(&mut self, subscriber: SubscriberId) {
        let mut deps: SmallSubList<DependencyId> = match subscriber {
            SubscriberId::Memo(memo_id) => {
                let slot: SlotId = memo_id.slot();
                match self.get_mut_memos().get_mut(slot.as_usize()) {
                    Some(entry) if entry.get_generation() == slot.get_generation() => {
                        take(entry.get_mut_dependencies())
                    }
                    _ => SmallSubList::Empty,
                }
            }
            SubscriberId::Effect(effect_id) => {
                let slot: SlotId = effect_id.slot();
                match self.get_mut_effects().get_mut(slot.as_usize()) {
                    Some(entry) if entry.get_generation() == slot.get_generation() => {
                        take(entry.get_mut_dependencies())
                    }
                    _ => SmallSubList::Empty,
                }
            }
        };
        if deps.is_empty() {
            return;
        }
        deps.drain_each(|dep: DependencyId| match dep {
            DependencyId::Signal(sig_id) => {
                let slot: SlotId = sig_id.slot();
                if let Some(entry) = self.get_mut_signals().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                {
                    entry.get_mut_subscribers().remove_item(subscriber);
                }
            }
            DependencyId::Memo(memo_id) => {
                let slot: SlotId = memo_id.slot();
                if let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                {
                    entry.get_mut_subscribers().remove_item(subscriber);
                }
            }
        });
        if matches!(deps, SmallSubList::Many(_)) {
            match subscriber {
                SubscriberId::Memo(memo_id) => {
                    let slot: SlotId = memo_id.slot();
                    if let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize())
                        && entry.get_generation() == slot.get_generation()
                    {
                        entry.set_dependencies(deps);
                    }
                }
                SubscriberId::Effect(effect_id) => {
                    let slot: SlotId = effect_id.slot();
                    if let Some(entry) = self.get_mut_effects().get_mut(slot.as_usize())
                        && entry.get_generation() == slot.get_generation()
                    {
                        entry.set_dependencies(deps);
                    }
                }
            }
        }
    }

    /// Disposes a signal slot and returns it to the free list.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SignalId` - Signal identifier to dispose.
    pub(crate) fn dispose_signal(&mut self, id: SignalId) {
        let slot: SlotId = id.slot();
        if let Some(entry) = self.get_mut_signals().get_mut(slot.as_usize())
            && entry.get_generation() == slot.get_generation()
            && entry.value.is_some()
        {
            entry.set_value(None);
            entry.get_mut_subscribers().clear();
            let next_gen: u32 = entry.get_generation().wrapping_add(1);
            entry.set_generation(next_gen);
            self.get_mut_free_signals().push(slot.get_index());
        }
    }

    /// Disposes a memo slot and detaches its dependencies.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `MemoId` - Memo identifier to dispose.
    pub(crate) fn dispose_memo(&mut self, id: MemoId) {
        self.clear_dependencies(SubscriberId::Memo(id));
        let slot: SlotId = id.slot();
        if let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize())
            && entry.get_generation() == slot.get_generation()
            && entry.compute.is_some()
        {
            entry.set_value(None);
            entry.set_compute(None);
            entry.get_mut_subscribers().clear();
            let next_gen: u32 = entry.get_generation().wrapping_add(1);
            entry.set_generation(next_gen);
            self.get_mut_free_memos().push(slot.get_index());
        }
    }

    /// Disposes an effect slot and detaches its dependencies.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `EffectId` - Effect identifier to dispose.
    pub(crate) fn dispose_effect(&mut self, id: EffectId) {
        self.clear_dependencies(SubscriberId::Effect(id));
        let slot: SlotId = id.slot();
        if let Some(entry) = self.get_mut_effects().get_mut(slot.as_usize())
            && entry.get_generation() == slot.get_generation()
            && entry.get_alive()
        {
            entry.set_alive(false);
            entry.set_scheduled(false);
            entry.set_frozen_depth(0);
            entry.set_pending_while_frozen(false);
            entry.set_callback(None);
            let next_gen: u32 = entry.get_generation().wrapping_add(1);
            entry.set_generation(next_gen);
            self.get_mut_free_effects().push(slot.get_index());
        }
    }

    /// Collects all resources belonging to a scope and marks them disposed, returning cleanup closures.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `ScopeId` - Scope identifier to dispose recursively.
    ///
    /// # Returns
    ///
    /// - `Vec<Box<dyn FnOnce()>>` - Cleanup callbacks to execute outside the runtime borrow.
    pub(crate) fn dispose_scope_collect(&mut self, id: ScopeId) -> Vec<Box<dyn FnOnce()>> {
        let mut all_cleanups: Vec<Box<dyn FnOnce()>> = Vec::new();
        let mut stack: Vec<ScopeId> = vec![id];
        while let Some(current_id) = stack.pop() {
            let slot: SlotId = current_id.slot();
            let (children, effects, memos, signals, cleanups): ScopeDrain =
                match self.get_mut_scopes().get_mut(slot.as_usize()) {
                    Some(entry)
                        if entry.get_generation() == slot.get_generation() && entry.get_alive() =>
                    {
                        entry.set_alive(false);
                        let next_gen: u32 = entry.get_generation().wrapping_add(1);
                        entry.set_generation(next_gen);
                        let out: ScopeDrain = (
                            take(entry.get_mut_children()),
                            take(entry.get_mut_effects()),
                            take(entry.get_mut_memos()),
                            take(entry.get_mut_signals()),
                            take(entry.get_mut_cleanups()),
                        );
                        self.get_mut_free_scopes().push(slot.get_index());
                        out
                    }
                    _ => continue,
                };
            for child in children.into_iter() {
                stack.push(child);
            }
            for eff in effects.into_iter() {
                self.dispose_effect(eff);
            }
            for memo in memos.into_iter() {
                self.dispose_memo(memo);
            }
            for sig in signals.into_iter() {
                self.dispose_signal(sig);
            }
            all_cleanups.extend(cleanups);
        }
        all_cleanups
    }

    /// Computes live diagnostic statistics for the runtime arena.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reference to the reactive runtime.
    ///
    /// # Returns
    ///
    /// - `ReactiveStats` - Snapshot of live node counts and write clock.
    #[must_use]
    pub fn stats(&self) -> ReactiveStats {
        let live_signals: usize = self
            .get_signals()
            .iter()
            .filter(|entry: &&SignalEntry| entry.value.is_some())
            .count();
        let live_memos: usize = self
            .get_memos()
            .iter()
            .filter(|entry: &&MemoEntry| entry.compute.is_some())
            .count();
        let live_effects: usize = self
            .get_effects()
            .iter()
            .filter(|entry: &&EffectEntry| entry.get_alive())
            .count();
        let live_scopes: usize = self
            .get_scopes()
            .iter()
            .filter(|entry: &&ScopeEntry| entry.get_alive())
            .count();
        ReactiveStats::new(
            live_signals,
            live_memos,
            live_effects,
            live_scopes,
            self.get_clock(),
        )
    }

    /// Attaches a signal identifier to an active scope entry.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `ScopeId` - Target scope identifier.
    /// - `SignalId` - Signal identifier to attach.
    fn attach_signal_to_scope(&mut self, scope_id: ScopeId, signal_id: SignalId) {
        let slot: SlotId = scope_id.slot();
        if let Some(entry) = self.get_mut_scopes().get_mut(slot.as_usize())
            && entry.get_generation() == slot.get_generation()
            && entry.get_alive()
        {
            entry.get_mut_signals().push(signal_id);
        }
    }

    /// Attaches a memo identifier to an active scope entry.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `ScopeId` - Target scope identifier.
    /// - `MemoId` - Memo identifier to attach.
    fn attach_memo_to_scope(&mut self, scope_id: ScopeId, memo_id: MemoId) {
        let slot: SlotId = scope_id.slot();
        if let Some(entry) = self.get_mut_scopes().get_mut(slot.as_usize())
            && entry.get_generation() == slot.get_generation()
            && entry.get_alive()
        {
            entry.get_mut_memos().push(memo_id);
        }
    }

    /// Attaches an effect identifier to an active scope entry.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `ScopeId` - Target scope identifier.
    /// - `EffectId` - Effect identifier to attach.
    fn attach_effect_to_scope(&mut self, scope_id: ScopeId, effect_id: EffectId) {
        let slot: SlotId = scope_id.slot();
        let scope_is_frozen: bool = if let Some(entry) =
            self.get_mut_scopes().get_mut(slot.as_usize())
            && entry.get_generation() == slot.get_generation()
            && entry.get_alive()
        {
            entry.get_mut_effects().push(effect_id);
            entry.get_frozen()
        } else {
            false
        };
        if scope_is_frozen {
            let eff_slot: SlotId = effect_id.slot();
            if let Some(eff) = self.get_mut_effects().get_mut(eff_slot.as_usize())
                && eff.get_generation() == eff_slot.get_generation()
            {
                let next_d: u32 = eff.get_frozen_depth().saturating_add(1);
                eff.set_frozen_depth(next_d);
            }
        }
    }

    /// Recursively freezes or resumes `id` and all nested child scopes, coalescing any pending dirty effects on resume.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `ScopeId` - Root scope identifier to freeze or resume.
    /// - `bool` - `true` to freeze; `false` to resume.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when resuming scheduled one or more deferred effects that require flushing.
    pub(crate) fn set_scope_frozen(&mut self, id: ScopeId, freeze: bool) -> bool {
        let root_slot: SlotId = id.slot();
        let should_change: bool = self
            .get_scopes()
            .get(root_slot.as_usize())
            .map(|entry: &ScopeEntry| {
                entry.get_generation() == root_slot.get_generation()
                    && entry.get_alive()
                    && entry.get_frozen() != freeze
            })
            .unwrap_or(false);
        if !should_change {
            return false;
        }
        let mut stack: Vec<ScopeId> = vec![id];
        let mut any_resumed_dirty: bool = false;
        while let Some(cur_id) = stack.pop() {
            let slot: SlotId = cur_id.slot();
            let (children, effects): (Vec<ScopeId>, Vec<EffectId>) =
                match self.get_mut_scopes().get_mut(slot.as_usize()) {
                    Some(entry)
                        if entry.get_generation() == slot.get_generation() && entry.get_alive() =>
                    {
                        if cur_id == id {
                            entry.set_frozen(freeze);
                        }
                        (entry.get_children().clone(), entry.get_effects().clone())
                    }
                    _ => continue,
                };
            for child in children.into_iter() {
                stack.push(child);
            }
            for eff_id in effects.into_iter() {
                let eslot: SlotId = eff_id.slot();
                let mut schedule_now: bool = false;
                if let Some(eff) = self.get_mut_effects().get_mut(eslot.as_usize())
                    && eff.get_generation() == eslot.get_generation()
                    && eff.get_alive()
                {
                    if freeze {
                        let next_d: u32 = eff.get_frozen_depth().saturating_add(1);
                        eff.set_frozen_depth(next_d);
                    } else {
                        let next_d: u32 = eff.get_frozen_depth().saturating_sub(1);
                        eff.set_frozen_depth(next_d);
                        if next_d == 0 && eff.get_pending_while_frozen() {
                            eff.set_pending_while_frozen(false);
                            if !eff.get_scheduled() {
                                eff.set_scheduled(true);
                                schedule_now = true;
                            }
                        }
                    }
                }
                if schedule_now {
                    self.get_mut_pending_effects().push(eff_id);
                    any_resumed_dirty = true;
                }
            }
        }
        any_resumed_dirty
    }

    /// Returns whether scope `id` is currently alive and frozen.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reactive runtime reference.
    /// - `ScopeId` - Scope identifier to query.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the scope is frozen.
    pub(crate) fn is_scope_frozen(&self, id: ScopeId) -> bool {
        let slot: SlotId = id.slot();
        self.get_scopes()
            .get(slot.as_usize())
            .map(|entry: &ScopeEntry| {
                entry.get_generation() == slot.get_generation()
                    && entry.get_alive()
                    && entry.get_frozen()
            })
            .unwrap_or(false)
    }

    /// Returns the currently active reactive `ScopeId` on the top of the scope stack, if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reactive runtime reference.
    ///
    /// # Returns
    ///
    /// - `Option<ScopeId>` - Active scope identifier or `None` outside `Scope::run`/`Scope::enter`.
    #[must_use]
    pub(crate) fn current_scope_id(&self) -> Option<ScopeId> {
        self.get_scope_stack().last().copied()
    }

    /// Returns `true` when `candidate` equals `root` or is a live descendant scope of `root`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reactive runtime reference.
    /// - `ScopeId` - Scope identifier to test.
    /// - `ScopeId` - Subtree root scope identifier.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether `candidate` belongs to `root`'s scope hierarchy.
    #[must_use]
    pub(crate) fn is_scope_in_subtree(&self, candidate: ScopeId, root: ScopeId) -> bool {
        if candidate == root {
            return true;
        }
        let mut stack: Vec<ScopeId> = vec![root];
        while let Some(cur) = stack.pop() {
            let slot: SlotId = cur.slot();
            if let Some(entry) = self.get_scopes().get(slot.as_usize())
                && entry.get_generation() == slot.get_generation()
                && entry.get_alive()
            {
                for child in entry.get_children().iter().copied() {
                    if child == candidate {
                        return true;
                    }
                    stack.push(child);
                }
            }
        }
        false
    }

    /// Returns `true` when `id` itself or any live ancestor scope is currently frozen.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reactive runtime reference.
    /// - `ScopeId` - Scope identifier to test.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether `id` is suspended by freezing.
    #[must_use]
    pub(crate) fn is_scope_effectively_frozen(&self, id: ScopeId) -> bool {
        if self.is_scope_frozen(id) {
            return true;
        }
        for (idx, entry) in self.get_scopes().iter().enumerate() {
            if entry.get_alive() && entry.get_frozen() {
                let frozen_id: ScopeId =
                    ScopeId::from_slot(SlotId::new(idx as u32, entry.get_generation()));
                if self.is_scope_in_subtree(id, frozen_id) {
                    return true;
                }
            }
        }
        false
    }
}
