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
    pub(crate) fn read_signal_cloned<T>(&mut self, id: SignalId) -> Option<T>
    where
        T: Clone + 'static,
    {
        self.track_dependency(DependencyId::Signal(id));
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
    pub(crate) fn write_signal<T>(&mut self, id: SignalId, value: T) -> (bool, bool)
    where
        T: 'static,
    {
        let slot: SlotId = id.slot();
        let idx: usize = slot.as_usize();
        let Some(entry) = self.get_mut_signals().get_mut(idx) else {
            return (false, false);
        };
        if entry.get_generation() != slot.get_generation() || entry.value.is_none() {
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
        let has_subs: bool = !entry.get_subscribers().is_empty();
        let next_clock: u64 = self.get_clock().wrapping_add(1);
        self.set_clock(next_clock);
        if has_subs {
            self.mark_signal_subscribers_dirty(idx);
        }
        let should_flush: bool =
            self.get_batch_depth() == 0 && !self.get_pending_effects().is_empty();
        (true, should_flush)
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
        let has_subs: bool = !entry.get_subscribers().is_empty();
        let next_clock: u64 = self.get_clock().wrapping_add(1);
        self.set_clock(next_clock);
        if has_subs {
            self.mark_signal_subscribers_dirty(idx);
        }
        let should_flush: bool =
            self.get_batch_depth() == 0 && !self.get_pending_effects().is_empty();
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
                    dirty_queue.push((subscriber, true));
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
                    dirty_queue.push((subscriber, true));
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
        while let Some((sub, is_direct)) = dirty_queue.pop() {
            match sub {
                SubscriberId::Memo(memo_id) => {
                    let slot: SlotId = memo_id.slot();
                    if let Some(entry) = memos.get_mut(slot.as_usize()) {
                        if entry.get_generation() != slot.get_generation()
                            || entry.compute.is_none()
                        {
                            continue;
                        }
                        let target_state: NodeState = if is_direct {
                            NodeState::Dirty
                        } else {
                            NodeState::Check
                        };
                        if entry.get_state() == NodeState::Dirty
                            || (entry.get_state() == NodeState::Check
                                && target_state == NodeState::Check)
                        {
                            continue;
                        }
                        entry.set_state(target_state);
                        entry.get_subscribers().for_each(|child_sub: SubscriberId| {
                            dirty_queue.push((child_sub, false));
                        });
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
}
