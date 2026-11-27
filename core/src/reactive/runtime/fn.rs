use super::*;

/// Executes a closure with mutable access to the thread-local reactive runtime.
///
/// # Arguments
///
/// - `F` - Closure receiving `&mut ReactiveRuntime`.
/// - `R` - Fallback return value used if the runtime cannot be borrowed.
///
/// # Returns
///
/// - `R` - Closure result or fallback value.
pub(crate) fn with_runtime<F, R>(operation: F, fallback: R) -> R
where
    F: FnOnce(&mut ReactiveRuntime) -> R,
{
    REACTIVE_RUNTIME
        .try_with(
            |cell: &RefCell<ReactiveRuntime>| match cell.try_borrow_mut() {
                Ok(mut guard) => Some(operation(&mut guard)),
                Err(_) => None,
            },
        )
        .ok()
        .flatten()
        .unwrap_or(fallback)
}

/// Evaluates a memo if dirty or check-marked, updating its cached value outside any `RefCell` borrow.
///
/// # Arguments
///
/// - `MemoId` - Memo identifier to refresh.
///
/// # Returns
///
/// - `bool` - True when the memo value changed during recomputation.
pub(crate) fn evaluate_memo(id: MemoId) -> bool {
    let slot: SlotId = id.slot();
    let idx: usize = slot.as_usize();
    let (state, deps): (NodeState, SmallSubList<DependencyId>) = with_runtime(
        |rt: &mut ReactiveRuntime| {
            let Some(entry) = rt.get_memos().get(idx) else {
                return (NodeState::Clean, SmallSubList::Empty);
            };
            if entry.get_generation() != slot.get_generation() || entry.compute.is_none() {
                return (NodeState::Clean, SmallSubList::Empty);
            }
            let cur_state: NodeState = entry.get_state();
            if cur_state == NodeState::Check {
                (cur_state, entry.get_dependencies().clone())
            } else {
                (cur_state, SmallSubList::Empty)
            }
        },
        (NodeState::Clean, SmallSubList::Empty),
    );
    if state == NodeState::Clean {
        return false;
    }
    if state == NodeState::Check {
        let mut upstream_changed: bool = false;
        deps.for_each(|dep: DependencyId| {
            if let DependencyId::Memo(parent_memo) = dep
                && evaluate_memo(parent_memo)
            {
                upstream_changed = true;
            }
        });
        if !upstream_changed {
            with_runtime(
                |rt: &mut ReactiveRuntime| {
                    if let Some(entry) = rt.get_mut_memos().get_mut(idx)
                        && entry.get_generation() == slot.get_generation()
                    {
                        entry.set_state(NodeState::Clean);
                    }
                },
                (),
            );
            return false;
        }
    }
    let staged: Option<StagedMemo> = with_runtime(
        |rt: &mut ReactiveRuntime| {
            let entry: &mut MemoEntry = rt.get_mut_memos().get_mut(idx)?;
            if entry.get_generation() != slot.get_generation() {
                return None;
            }
            let compute: MemoComputeFn = entry.compute.clone()?;
            let prev_value: Option<Box<dyn Any>> = entry.value.take();
            rt.get_mut_observer_stack()
                .push(Some((SubscriberId::Memo(id), 0)));
            Some((compute, prev_value))
        },
        None,
    );
    let Some((compute, mut value_slot)) = staged else {
        return false;
    };
    let changed: bool = match compute.try_borrow_mut() {
        Ok(mut func) => func(&mut value_slot),
        Err(_) => {
            with_runtime(
                |rt: &mut ReactiveRuntime| {
                    rt.get_mut_observer_stack().pop();
                    if let Some(entry) = rt.get_mut_memos().get_mut(idx)
                        && entry.get_generation() == slot.get_generation()
                    {
                        entry.set_value(value_slot);
                    }
                },
                (),
            );
            return false;
        }
    };
    with_runtime(
        |rt: &mut ReactiveRuntime| {
            rt.finish_tracking();
            let should_propagate: bool = if let Some(entry) = rt.get_mut_memos().get_mut(idx) {
                if entry.get_generation() != slot.get_generation() {
                    return;
                }
                entry.set_value(value_slot);
                entry.set_state(NodeState::Clean);
                if changed {
                    let next_ver: u64 = entry.get_version().wrapping_add(1);
                    entry.set_version(next_ver);
                    !entry.get_subscribers().is_empty()
                } else {
                    false
                }
            } else {
                false
            };
            if should_propagate {
                rt.mark_memo_subscribers_dirty(idx);
            }
        },
        (),
    );
    changed
}

/// Runs a single effect outside any `RefCell` borrow while tracking its dependencies.
///
/// # Arguments
///
/// - `EffectId` - Effect identifier to execute.
pub(crate) fn run_effect(id: EffectId) {
    let slot: SlotId = id.slot();
    let callback_opt: Option<Rc<RefCell<dyn FnMut()>>> = with_runtime(
        |rt: &mut ReactiveRuntime| {
            let entry: &mut EffectEntry = rt.get_mut_effects().get_mut(slot.as_usize())?;
            if entry.get_generation() != slot.get_generation() || !entry.get_alive() {
                return None;
            }
            entry.set_scheduled(false);
            let cb: Rc<RefCell<dyn FnMut()>> = entry.callback.clone()?;
            rt.get_mut_observer_stack()
                .push(Some((SubscriberId::Effect(id), 0)));
            Some(cb)
        },
        None,
    );
    let Some(callback) = callback_opt else {
        return;
    };
    let ran: bool = if let Ok(mut func) = callback.try_borrow_mut() {
        func();
        true
    } else {
        false
    };
    with_runtime(
        |rt: &mut ReactiveRuntime| {
            if ran {
                rt.finish_tracking();
            } else {
                rt.get_mut_observer_stack().pop();
            }
        },
        (),
    );
}

/// Flushes all pending reactive effects in topological depth order when not inside a batch.
pub fn flush_effects() {
    let should_start: bool = with_runtime(
        |rt: &mut ReactiveRuntime| {
            if rt.get_batch_depth() > 0 || rt.get_flushing() || rt.get_pending_effects().is_empty()
            {
                return false;
            }
            rt.set_flushing(true);
            true
        },
        false,
    );
    if !should_start {
        return;
    }
    let mut iterations: usize = 0;
    while iterations < MAX_FLUSH_ITERATIONS {
        let single_opt: Option<EffectId> = with_runtime(
            |rt: &mut ReactiveRuntime| {
                if rt.get_pending_effects().len() == 1 {
                    rt.get_mut_pending_effects().pop()
                } else {
                    None
                }
            },
            None,
        );
        if let Some(eff_id) = single_opt {
            run_effect(eff_id);
            iterations += 1;
            continue;
        }
        let mut batch_ids: Vec<(u32, EffectId)> = with_runtime(
            |rt: &mut ReactiveRuntime| {
                let pending: Vec<EffectId> = take(rt.get_mut_pending_effects());
                let mut pairs: Vec<(u32, EffectId)> = Vec::with_capacity(pending.len());
                for eff_id in pending.into_iter() {
                    let slot: SlotId = eff_id.slot();
                    if let Some(entry) = rt.get_effects().get(slot.as_usize())
                        && entry.get_generation() == slot.get_generation()
                        && entry.get_alive()
                    {
                        pairs.push((entry.get_depth(), eff_id));
                    }
                }
                pairs
            },
            Vec::new(),
        );
        if batch_ids.is_empty() {
            break;
        }
        batch_ids.sort_by_key(|pair: &(u32, EffectId)| pair.0);
        for (_, eff_id) in batch_ids.into_iter() {
            run_effect(eff_id);
        }
        iterations += 1;
    }
    with_runtime(
        |rt: &mut ReactiveRuntime| {
            rt.set_flushing(false);
        },
        (),
    );
}

/// Coalesces multiple signal writes inside `operation` into a single reactive flush pass.
///
/// # Arguments
///
/// - `F` - Closure executing reactive writes.
///
/// # Returns
///
/// - `R` - Value returned by `operation`.
pub fn batch<F, R>(operation: F) -> R
where
    F: FnOnce() -> R,
{
    with_runtime(
        |rt: &mut ReactiveRuntime| {
            let next_depth: u32 = rt.get_batch_depth().saturating_add(1);
            rt.set_batch_depth(next_depth);
        },
        (),
    );
    let result: R = operation();
    let should_flush: bool = with_runtime(
        |rt: &mut ReactiveRuntime| {
            let next_depth: u32 = rt.get_batch_depth().saturating_sub(1);
            rt.set_batch_depth(next_depth);
            next_depth == 0
        },
        false,
    );
    if should_flush {
        flush_effects();
    }
    result
}

/// Executes `operation` without tracking any reactive signal or memo reads.
///
/// # Arguments
///
/// - `F` - Closure whose reactive reads should remain untracked.
///
/// # Returns
///
/// - `R` - Value returned by `operation`.
pub fn untracked<F, R>(operation: F) -> R
where
    F: FnOnce() -> R,
{
    with_runtime(
        |rt: &mut ReactiveRuntime| {
            rt.get_mut_observer_stack().push(None);
        },
        (),
    );
    let result: R = operation();
    with_runtime(
        |rt: &mut ReactiveRuntime| {
            rt.get_mut_observer_stack().pop();
        },
        (),
    );
    result
}

/// Registers a cleanup callback on the currently active reactive scope.
///
/// # Arguments
///
/// - `F` - Cleanup callback executed once when the active scope disposes.
pub fn on_cleanup<F>(cleanup: F)
where
    F: FnOnce() + 'static,
{
    with_runtime(
        |rt: &mut ReactiveRuntime| {
            rt.register_cleanup(cleanup);
        },
        (),
    );
}

/// Returns diagnostic statistics for the current thread's reactive runtime.
///
/// # Returns
///
/// - `ReactiveStats` - Snapshot of live signals, memos, effects, scopes, and clock.
#[must_use]
pub fn reactive_stats() -> ReactiveStats {
    with_runtime(
        |rt: &mut ReactiveRuntime| rt.stats(),
        ReactiveStats::default(),
    )
}

/// Resets the current thread's reactive runtime to an empty state.
pub fn reset_reactive_runtime() {
    with_runtime(
        |rt: &mut ReactiveRuntime| {
            *rt = ReactiveRuntime::default();
        },
        (),
    );
}
