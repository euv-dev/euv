use super::*;

/// Lifecycle operations for `Effect`.
impl Effect {
    /// Creates and immediately runs a reactive side effect, auto-tracking all signals and memos read.
    ///
    /// # Arguments
    ///
    /// - `F` - Effect callback executed initially and on every tracked dependency change.
    ///
    /// # Returns
    ///
    /// - `Self` - Handle to the registered effect.
    pub fn new<F>(callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        let id: EffectId = with_runtime(
            |rt: &mut ReactiveRuntime| rt.create_effect(callback),
            EffectId::default(),
        );
        run_effect(id);
        Self { id }
    }

    /// Returns the underlying `EffectId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Effect handle.
    ///
    /// # Returns
    ///
    /// - `EffectId` - Underlying arena identifier.
    #[must_use]
    pub fn id(&self) -> EffectId {
        let Self { id } = *self;
        id
    }

    /// Disposes this effect so it no longer reacts to upstream changes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Effect handle to dispose.
    pub fn dispose(&self) {
        let id: EffectId = self.id();
        with_runtime(
            |rt: &mut ReactiveRuntime| {
                rt.dispose_effect(id);
            },
            (),
        );
    }
}
