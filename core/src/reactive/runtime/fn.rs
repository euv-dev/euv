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
#[inline(always)]
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

/// Pull-verifies upstream `MemoId` dependencies for a `NodeState::Check` memo, short-circuiting on the first change.
///
/// # Arguments
///
/// - `MemoCheckDeps` - Zero-allocation inline or cursor-sliced upstream memo dependency snapshot.
///
/// # Returns
///
/// - `bool` - `true` as soon as any upstream memo recomputes to a changed value.
fn verify_check_memos(deps: MemoCheckDeps) -> bool {
    match deps {
        MemoCheckDeps::Inline(buf, count) => {
            let len: usize = count as usize;
            for parent_memo in buf.iter().copied().take(len) {
                if evaluate_memo(parent_memo) {
                    return true;
                }
            }
            false
        }
        MemoCheckDeps::Spill(start, end) => {
            let mut changed: bool = false;
            let mut cursor: usize = start;
            while cursor < end {
                let parent_opt: Option<MemoId> = with_runtime(
                    |rt: &mut ReactiveRuntime| rt.get_check_scratch().get(cursor).copied(),
                    None,
                );
                if let Some(parent_memo) = parent_opt
                    && evaluate_memo(parent_memo)
                {
                    changed = true;
                    break;
                }
                cursor += 1;
            }
            with_runtime(
                |rt: &mut ReactiveRuntime| {
                    rt.get_mut_check_scratch().truncate(start);
                },
                (),
            );
            changed
        }
    }
}

/// Reads a typed `Memo<T>` using fused runtime borrows (1 borrow on `Clean`, 2 borrows on `Dirty` or unchanged `Check`).
///
/// # Arguments
///
/// - `MemoId` - Target memo identifier to read and track.
///
/// # Returns
///
/// - `Option<T>` - Current computed value if alive and matching type `T`.
#[must_use]
pub(crate) fn read_memo_fused<T>(id: MemoId) -> Option<T>
where
    T: Clone + 'static,
{
    let slot: SlotId = id.slot();
    let idx: usize = slot.as_usize();
    let prep: MemoReadPrep<T> = with_runtime(
        |rt: &mut ReactiveRuntime| {
            let state: NodeState = {
                let Some(entry) = rt.get_memos().get(idx) else {
                    return MemoReadPrep::Dead;
                };
                if entry.get_generation() != slot.get_generation() {
                    return MemoReadPrep::Dead;
                }
                entry.get_state()
            };
            match state {
                NodeState::Clean => {
                    let val: Option<T> = rt.get_memos()[idx]
                        .value
                        .as_ref()
                        .and_then(|boxed: &Box<dyn Any>| boxed.downcast_ref::<T>())
                        .cloned();
                    rt.track_dependency(DependencyId::Memo(id));
                    MemoReadPrep::Ready(val)
                }
                NodeState::Dirty => {
                    let (compute, prev_value): StagedMemo = {
                        let entry: &mut MemoEntry = &mut rt.get_mut_memos()[idx];
                        let Some(compute) = entry.compute.clone() else {
                            return MemoReadPrep::Dead;
                        };
                        let prev_value: Option<Box<dyn Any>> = entry.value.take();
                        (compute, prev_value)
                    };
                    rt.get_mut_observer_stack()
                        .push(Some((SubscriberId::Memo(id), 0)));
                    MemoReadPrep::Compute((compute, prev_value))
                }
                NodeState::Check => {
                    if rt.get_memos()[idx].compute.is_none() {
                        return MemoReadPrep::Dead;
                    }
                    if let Some(check_deps) = rt.collect_check_memos(idx) {
                        MemoReadPrep::Check(check_deps)
                    } else {
                        let entry: &mut MemoEntry = &mut rt.get_mut_memos()[idx];
                        entry.set_state(NodeState::Clean);
                        let val: Option<T> = entry
                            .value
                            .as_ref()
                            .and_then(|boxed: &Box<dyn Any>| boxed.downcast_ref::<T>())
                            .cloned();
                        rt.track_dependency(DependencyId::Memo(id));
                        MemoReadPrep::Ready(val)
                    }
                }
            }
        },
        MemoReadPrep::Dead,
    );
    let staged: StagedMemo = match prep {
        MemoReadPrep::Dead => return None,
        MemoReadPrep::Ready(val) => return val,
        MemoReadPrep::Compute(staged_memo) => staged_memo,
        MemoReadPrep::Check(deps) => {
            if !verify_check_memos(deps) {
                return with_runtime(
                    |rt: &mut ReactiveRuntime| {
                        let entry: &mut MemoEntry = rt.get_mut_memos().get_mut(idx)?;
                        if entry.get_generation() != slot.get_generation() {
                            return None;
                        }
                        entry.set_state(NodeState::Clean);
                        let val: Option<T> = entry
                            .value
                            .as_ref()
                            .and_then(|boxed: &Box<dyn Any>| boxed.downcast_ref::<T>())
                            .cloned();
                        rt.track_dependency(DependencyId::Memo(id));
                        val
                    },
                    None,
                );
            }
            let staged_memo: StagedMemo = with_runtime(
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
            )?;
            staged_memo
        }
    };
    let (compute, mut value_slot): StagedMemo = staged;
    let mut guard: TrackingGuard = TrackingGuard::new();
    let eval_result: Option<bool> = match compute.try_borrow_mut() {
        Ok(mut func) => Some(func(&mut value_slot)),
        Err(_) => None,
    };
    guard.disarm();
    match eval_result {
        Some(changed) => with_runtime(
            |rt: &mut ReactiveRuntime| {
                rt.finish_tracking();
                let out_val: Option<T> = value_slot
                    .as_ref()
                    .and_then(|boxed: &Box<dyn Any>| boxed.downcast_ref::<T>())
                    .cloned();
                let (alive, target): (bool, DirtyTarget) =
                    if let Some(entry) = rt.get_mut_memos().get_mut(idx) {
                        if entry.get_generation() != slot.get_generation() {
                            (false, DirtyTarget::Empty)
                        } else {
                            entry.set_value(value_slot);
                            entry.set_state(NodeState::Clean);
                            if changed {
                                let next_ver: u64 = entry.get_version().wrapping_add(1);
                                entry.set_version(next_ver);
                                (true, entry.get_subscribers().dirty_target())
                            } else {
                                (true, DirtyTarget::Empty)
                            }
                        }
                    } else {
                        (false, DirtyTarget::Empty)
                    };
                if !alive {
                    return None;
                }
                if !matches!(target, DirtyTarget::Empty) {
                    rt.propagate_memo_dirty(idx, target);
                }
                rt.track_dependency(DependencyId::Memo(id));
                out_val
            },
            None,
        ),
        None => with_runtime(
            |rt: &mut ReactiveRuntime| {
                let _: Option<Option<(SubscriberId, usize)>> = rt.get_mut_observer_stack().pop();
                let out_val: Option<T> = value_slot
                    .as_ref()
                    .and_then(|boxed: &Box<dyn Any>| boxed.downcast_ref::<T>())
                    .cloned();
                let alive: bool = if let Some(entry) = rt.get_mut_memos().get_mut(idx)
                    && entry.get_generation() == slot.get_generation()
                {
                    entry.set_value(value_slot);
                    true
                } else {
                    false
                };
                if !alive {
                    return None;
                }
                rt.track_dependency(DependencyId::Memo(id));
                out_val
            },
            None,
        ),
    }
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
    let prep: MemoEvalPrep = with_runtime(
        |rt: &mut ReactiveRuntime| {
            let state: NodeState = {
                let Some(entry) = rt.get_memos().get(idx) else {
                    return MemoEvalPrep::Clean;
                };
                if entry.get_generation() != slot.get_generation() || entry.compute.is_none() {
                    return MemoEvalPrep::Clean;
                }
                entry.get_state()
            };
            match state {
                NodeState::Clean => MemoEvalPrep::Clean,
                NodeState::Dirty => {
                    let (compute, prev_value): StagedMemo = {
                        let entry: &mut MemoEntry = &mut rt.get_mut_memos()[idx];
                        let Some(compute) = entry.compute.clone() else {
                            return MemoEvalPrep::Clean;
                        };
                        let prev_value: Option<Box<dyn Any>> = entry.value.take();
                        (compute, prev_value)
                    };
                    rt.get_mut_observer_stack()
                        .push(Some((SubscriberId::Memo(id), 0)));
                    MemoEvalPrep::Compute((compute, prev_value))
                }
                NodeState::Check => {
                    if let Some(check_deps) = rt.collect_check_memos(idx) {
                        MemoEvalPrep::Check(check_deps)
                    } else {
                        rt.get_mut_memos()[idx].set_state(NodeState::Clean);
                        MemoEvalPrep::Clean
                    }
                }
            }
        },
        MemoEvalPrep::Clean,
    );
    let staged: StagedMemo = match prep {
        MemoEvalPrep::Clean => return false,
        MemoEvalPrep::Compute(staged_memo) => staged_memo,
        MemoEvalPrep::Check(deps) => {
            if !verify_check_memos(deps) {
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
            let Some(staged_memo) = with_runtime(
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
            ) else {
                return false;
            };
            staged_memo
        }
    };
    let (compute, mut value_slot): StagedMemo = staged;
    let mut guard: TrackingGuard = TrackingGuard::new();
    let changed: bool = match compute.try_borrow_mut() {
        Ok(mut func) => func(&mut value_slot),
        Err(_) => {
            guard.disarm();
            with_runtime(
                |rt: &mut ReactiveRuntime| {
                    let _: Option<Option<(SubscriberId, usize)>> =
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
    guard.disarm();
    with_runtime(
        |rt: &mut ReactiveRuntime| {
            rt.finish_tracking();
            let target: DirtyTarget = if let Some(entry) = rt.get_mut_memos().get_mut(idx) {
                if entry.get_generation() != slot.get_generation() {
                    return;
                }
                entry.set_value(value_slot);
                entry.set_state(NodeState::Clean);
                if changed {
                    let next_ver: u64 = entry.get_version().wrapping_add(1);
                    entry.set_version(next_ver);
                    entry.get_subscribers().dirty_target()
                } else {
                    DirtyTarget::Empty
                }
            } else {
                DirtyTarget::Empty
            };
            if !matches!(target, DirtyTarget::Empty) {
                rt.propagate_memo_dirty(idx, target);
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
    let mut guard: TrackingGuard = TrackingGuard::new();
    let ran: bool = if let Ok(mut func) = callback.try_borrow_mut() {
        func();
        true
    } else {
        false
    };
    guard.disarm();
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
    let mut step: FlushStep = with_runtime(
        |rt: &mut ReactiveRuntime| {
            if rt.get_batch_depth() > 0 || rt.get_flushing() || rt.get_pending_effects().is_empty()
            {
                return FlushStep::Done;
            }
            rt.set_flushing(true);
            rt.drain_pending_flush_step(Vec::new())
        },
        FlushStep::Done,
    );
    if matches!(step, FlushStep::Done) {
        return;
    }
    let mut reusable_batch: Vec<(u32, EffectId)> = Vec::new();
    let mut iterations: usize = 0;
    loop {
        match step {
            FlushStep::Done => break,
            FlushStep::Single(eff_id) => {
                run_effect(eff_id);
            }
            FlushStep::Batch(mut batch, is_sorted) => {
                if !is_sorted {
                    batch.sort_by_key(|pair: &(u32, EffectId)| pair.0);
                }
                for (_, eff_id) in batch.drain(..) {
                    run_effect(eff_id);
                }
                reusable_batch = batch;
            }
        }
        iterations += 1;
        if iterations >= MAX_FLUSH_ITERATIONS {
            with_runtime(
                |rt: &mut ReactiveRuntime| {
                    if reusable_batch.capacity() > rt.get_flush_batch_scratch().capacity() {
                        rt.set_flush_batch_scratch(reusable_batch);
                    }
                    rt.set_flushing(false);
                },
                (),
            );
            break;
        }
        let passed_batch: Vec<(u32, EffectId)> = take(&mut reusable_batch);
        step = with_runtime(
            |rt: &mut ReactiveRuntime| rt.drain_pending_flush_step(passed_batch),
            FlushStep::Done,
        );
    }
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
    reset_system_event_hub();
    with_runtime(
        |rt: &mut ReactiveRuntime| {
            *rt = ReactiveRuntime::default();
        },
        (),
    );
}
