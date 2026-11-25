use super::*;

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
            batch_depth: 0,
            flushing: false,
            clock: 0,
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
                subscribers: Vec::with_capacity(INITIAL_SUBSCRIBER_CAPACITY),
            });
            SignalId::from_slot(SlotId::new(index, 1))
        };
        if let Some(scope_id) = self.get_scope_stack().last().copied() {
            self.attach_signal_to_scope(scope_id, id);
        }
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

    /// Writes a new signal value and propagates dirty notifications to subscribers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SignalId` - Signal identifier to update.
    /// - `T` - Replacement value.
    ///
    /// # Returns
    ///
    /// - `bool` - True when the signal was alive and updated.
    pub(crate) fn write_signal<T>(&mut self, id: SignalId, value: T) -> bool
    where
        T: 'static,
    {
        let slot: SlotId = id.slot();
        let Some(entry) = self.get_mut_signals().get_mut(slot.as_usize()) else {
            return false;
        };
        if entry.get_generation() != slot.get_generation() || entry.value.is_none() {
            return false;
        }
        entry.set_value(Some(Box::new(value)));
        let next_ver: u64 = entry.get_version().wrapping_add(1);
        entry.set_version(next_ver);
        let subscribers: Vec<SubscriberId> = entry.get_subscribers().clone();
        let next_clock: u64 = self.get_clock().wrapping_add(1);
        self.set_clock(next_clock);
        self.mark_subscribers_dirty(&subscribers);
        true
    }

    /// Writes a new signal value only when it differs from the stored value.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SignalId` - Signal identifier to update.
    /// - `T` - Candidate value compared using `PartialEq`.
    ///
    /// # Returns
    ///
    /// - `bool` - True when the value changed and subscribers were notified.
    pub(crate) fn write_signal_if_changed<T>(&mut self, id: SignalId, value: T) -> bool
    where
        T: PartialEq + 'static,
    {
        let slot: SlotId = id.slot();
        let unchanged: bool = if let Some(entry) = self.get_signals().get(slot.as_usize()) {
            if entry.get_generation() != slot.get_generation() {
                return false;
            }
            match entry
                .value
                .as_ref()
                .and_then(|boxed: &Box<dyn Any>| boxed.downcast_ref::<T>())
            {
                Some(current) => *current == value,
                None => return false,
            }
        } else {
            return false;
        };
        if unchanged {
            return false;
        }
        self.write_signal(id, value)
    }

    /// Allocates a new memo slot with a type-erased computation closure.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `F` - Memo computation closure returning `(Box<dyn Any>, bool)`.
    ///
    /// # Returns
    ///
    /// - `MemoId` - Handle for the newly created memo.
    pub(crate) fn create_memo<F>(&mut self, compute: F) -> MemoId
    where
        F: FnMut(Option<&dyn Any>) -> (Box<dyn Any>, bool) + 'static,
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
                dependencies: Vec::new(),
                subscribers: Vec::with_capacity(INITIAL_SUBSCRIBER_CAPACITY),
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
            EffectId::from_slot(SlotId::new(index, next_gen))
        } else {
            let index: u32 = self.get_effects().len() as u32;
            self.get_mut_effects().push(EffectEntry {
                generation: 1,
                alive: true,
                scheduled: false,
                callback: Some(callback_rc),
                dependencies: Vec::new(),
                depth,
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

    /// Records `dependency` on the currently executing observer if one exists.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `DependencyId` - The dependency accessed during evaluation.
    pub(crate) fn track_dependency(&mut self, dependency: DependencyId) {
        let Some(Some(subscriber)) = self.get_observer_stack().last().copied() else {
            return;
        };
        match dependency {
            DependencyId::Signal(sig_id) => {
                let slot: SlotId = sig_id.slot();
                if let Some(entry) = self.get_mut_signals().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                    && !entry.get_subscribers().contains(&subscriber)
                {
                    entry.get_mut_subscribers().push(subscriber);
                }
            }
            DependencyId::Memo(memo_id) => {
                let slot: SlotId = memo_id.slot();
                if let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                    && !entry.get_subscribers().contains(&subscriber)
                {
                    entry.get_mut_subscribers().push(subscriber);
                }
            }
        }
        match subscriber {
            SubscriberId::Memo(memo_id) => {
                let slot: SlotId = memo_id.slot();
                if let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                    && !entry.get_dependencies().contains(&dependency)
                {
                    entry.get_mut_dependencies().push(dependency);
                }
            }
            SubscriberId::Effect(effect_id) => {
                let slot: SlotId = effect_id.slot();
                if let Some(entry) = self.get_mut_effects().get_mut(slot.as_usize())
                    && entry.get_generation() == slot.get_generation()
                    && !entry.get_dependencies().contains(&dependency)
                {
                    entry.get_mut_dependencies().push(dependency);
                }
            }
        }
    }

    /// Propagates dirty flags downstream through memos and queues affected effects.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `&[SubscriberId]` - Direct subscribers of the mutated source.
    pub(crate) fn mark_subscribers_dirty(&mut self, subscribers: &[SubscriberId]) {
        let mut queue: Vec<(SubscriberId, bool)> = Vec::with_capacity(subscribers.len());
        for subscriber in subscribers.iter().copied() {
            queue.push((subscriber, true));
        }
        while let Some((sub, is_direct)) = queue.pop() {
            match sub {
                SubscriberId::Memo(memo_id) => {
                    let slot: SlotId = memo_id.slot();
                    let next_subs: Vec<SubscriberId> =
                        if let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize()) {
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
                            entry.get_subscribers().clone()
                        } else {
                            continue;
                        };
                    for child_sub in next_subs.into_iter() {
                        queue.push((child_sub, false));
                    }
                }
                SubscriberId::Effect(effect_id) => {
                    self.schedule_effect(effect_id);
                }
            }
        }
    }

    /// Marks an effect as scheduled and appends it to the pending flush queue.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `EffectId` - Effect identifier to enqueue.
    pub(crate) fn schedule_effect(&mut self, effect_id: EffectId) {
        let slot: SlotId = effect_id.slot();
        if let Some(entry) = self.get_mut_effects().get_mut(slot.as_usize())
            && entry.get_generation() == slot.get_generation()
            && entry.get_alive()
            && !entry.get_scheduled()
        {
            entry.set_scheduled(true);
            self.get_mut_pending_effects().push(effect_id);
        }
    }

    /// Detaches a subscriber from all of its previously tracked upstream dependencies.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the reactive runtime.
    /// - `SubscriberId` - Subscriber whose dependency links should be cleared.
    pub(crate) fn clear_dependencies(&mut self, subscriber: SubscriberId) {
        let deps: Vec<DependencyId> = match subscriber {
            SubscriberId::Memo(memo_id) => {
                let slot: SlotId = memo_id.slot();
                match self.get_mut_memos().get_mut(slot.as_usize()) {
                    Some(entry) if entry.get_generation() == slot.get_generation() => {
                        take(entry.get_mut_dependencies())
                    }
                    _ => Vec::new(),
                }
            }
            SubscriberId::Effect(effect_id) => {
                let slot: SlotId = effect_id.slot();
                match self.get_mut_effects().get_mut(slot.as_usize()) {
                    Some(entry) if entry.get_generation() == slot.get_generation() => {
                        take(entry.get_mut_dependencies())
                    }
                    _ => Vec::new(),
                }
            }
        };
        for dep in deps.into_iter() {
            match dep {
                DependencyId::Signal(sig_id) => {
                    let slot: SlotId = sig_id.slot();
                    if let Some(entry) = self.get_mut_signals().get_mut(slot.as_usize())
                        && entry.get_generation() == slot.get_generation()
                    {
                        entry
                            .get_mut_subscribers()
                            .retain(|item: &SubscriberId| *item != subscriber);
                    }
                }
                DependencyId::Memo(memo_id) => {
                    let slot: SlotId = memo_id.slot();
                    if let Some(entry) = self.get_mut_memos().get_mut(slot.as_usize())
                        && entry.get_generation() == slot.get_generation()
                    {
                        entry
                            .get_mut_subscribers()
                            .retain(|item: &SubscriberId| *item != subscriber);
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
        if let Some(entry) = self.get_mut_scopes().get_mut(slot.as_usize())
            && entry.get_generation() == slot.get_generation()
            && entry.get_alive()
        {
            entry.get_mut_effects().push(effect_id);
        }
    }
}
