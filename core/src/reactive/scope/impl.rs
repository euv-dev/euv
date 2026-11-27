use super::*;

/// Ownership lifecycle operations for `Scope`.
impl Scope {
    /// Creates a new scope and executes `body` inside it, returning both the scope handle and result.
    ///
    /// # Arguments
    ///
    /// - `F` - Closure executed while this scope is active on the scope stack.
    ///
    /// # Returns
    ///
    /// - `(Self, R)` - The created scope handle and closure result.
    #[inline]
    pub fn run<F, R>(body: F) -> (Self, R)
    where
        F: FnOnce(Self) -> R,
    {
        let id: ScopeId = with_runtime(
            |rt: &mut ReactiveRuntime| {
                let created: ScopeId = rt.create_scope();
                rt.get_mut_scope_stack().push(created);
                created
            },
            ScopeId::default(),
        );
        let scope: Self = Self { id };
        let result: R = body(scope);
        with_runtime(
            |rt: &mut ReactiveRuntime| {
                rt.get_mut_scope_stack().pop();
            },
            (),
        );
        (scope, result)
    }

    /// Executes `body` inside an existing scope so newly created signals, memos, and effects attach to it.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Target ownership scope.
    /// - `F` - Closure executed under this scope.
    ///
    /// # Returns
    ///
    /// - `R` - Value returned by `body`.
    #[inline]
    pub fn enter<F, R>(&self, body: F) -> R
    where
        F: FnOnce() -> R,
    {
        let id: ScopeId = self.id();
        with_runtime(
            |rt: &mut ReactiveRuntime| {
                rt.get_mut_scope_stack().push(id);
            },
            (),
        );
        let result: R = body();
        with_runtime(
            |rt: &mut ReactiveRuntime| {
                rt.get_mut_scope_stack().pop();
            },
            (),
        );
        result
    }

    /// Returns the underlying `ScopeId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scope handle.
    ///
    /// # Returns
    ///
    /// - `ScopeId` - Underlying arena identifier.
    #[must_use]
    #[inline(always)]
    pub fn id(&self) -> ScopeId {
        let Self { id } = *self;
        id
    }

    /// Disposes this scope, all child scopes, owned reactive nodes, and executes all registered cleanups.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scope handle to dispose.
    #[inline]
    pub fn dispose(&self) {
        let id: ScopeId = self.id();
        let cleanups: Vec<Box<dyn FnOnce()>> = with_runtime(
            |rt: &mut ReactiveRuntime| rt.dispose_scope_collect(id),
            Vec::new(),
        );
        for cleanup in cleanups.into_iter() {
            cleanup();
        }
    }

    /// Freezes all effects owned by this scope and its descendant scopes while tracking cross-page signal dirtiness.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scope handle to freeze.
    #[inline(always)]
    pub fn freeze(&self) {
        let id: ScopeId = self.id();
        let _: bool = with_runtime(
            |rt: &mut ReactiveRuntime| rt.set_scope_frozen(id, true),
            false,
        );
    }

    /// Resumes this scope and all descendant scopes, atomically flushing any coalesced cross-page updates once.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scope handle to resume.
    #[inline(always)]
    pub fn resume(&self) {
        let id: ScopeId = self.id();
        let should_flush: bool = with_runtime(
            |rt: &mut ReactiveRuntime| rt.set_scope_frozen(id, false),
            false,
        );
        if should_flush {
            flush_effects();
        }
    }

    /// Returns `true` when this scope is currently suspended via `Scope::freeze`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Scope handle to inspect.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether this scope is frozen.
    #[must_use]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        let id: ScopeId = self.id();
        with_runtime(|rt: &mut ReactiveRuntime| rt.is_scope_frozen(id), false)
    }
}
