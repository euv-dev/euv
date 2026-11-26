use super::*;

/// Typed cross-component event emission and scoped subscription for `EventBus`.
impl EventBus {
    /// Emits a typed custom event `E` to all active cross-component listeners (frozen listeners coalesce until resumed).
    ///
    /// # Arguments
    ///
    /// - `E` - Custom event payload implementing `CustomEvent`.
    pub fn emit<E>(event: E)
    where
        E: CustomEvent,
    {
        let id: SignalId = with_runtime(
            |rt: &mut ReactiveRuntime| rt.ensure_event_channel::<E>(),
            SignalId::default(),
        );
        let channel: Signal<(u64, Option<E>)> = Signal::from_raw_id(id);
        let _: bool = channel.update(|slot: &mut (u64, Option<E>)| {
            slot.0 = slot.0.wrapping_add(1);
            slot.1 = Some(event);
        });
    }

    /// Subscribes a scope-owned listener invoked whenever custom event `E` is emitted across any component or page.
    ///
    /// # Arguments
    ///
    /// - `F` - Event handler receiving `&E` (automatically cleaned up on unmount and paused during `PageRetention::Freeze`).
    ///
    /// # Returns
    ///
    /// - `Effect` - Scope-owned subscription effect handle.
    #[must_use]
    pub fn on<E, F>(mut handler: F) -> Effect
    where
        E: CustomEvent,
        F: FnMut(&E) + 'static,
    {
        let id: SignalId = with_runtime(
            |rt: &mut ReactiveRuntime| rt.ensure_event_channel::<E>(),
            SignalId::default(),
        );
        let channel: Signal<(u64, Option<E>)> = Signal::from_raw_id(id);
        let seen_seq: Rc<Cell<u64>> = Rc::new(Cell::new(channel.get_untracked().0));
        Effect::new(move || {
            let (seq, payload): (u64, Option<E>) = channel.get();
            if seq != seen_seq.replace(seq)
                && let Some(ev) = payload.as_ref()
            {
                handler(ev);
            }
        })
    }
}
