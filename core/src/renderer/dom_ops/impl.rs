use super::*;

/// Process-wide cache of the per-load DOM-op names.
///
/// `OnceLock` is the standard library's run-once cell: the closure runs at
/// most once, every later reader blocks until that first write lands, and
/// `get_or_init` is safe to call from any thread. The names are pure data
/// (`String`, no JS handles), so unlike [`DOM_OP_TABLE`] they need no
/// `thread_local!` — one set of names is correct for the whole process, and
/// two threads racing to build it cannot produce two different sets.
static DOM_OP_NAMES: OnceLock<DomOpNames> = OnceLock::new();

impl DomOpNames {
    /// Returns the per-load DOM-op names, building them on first call.
    ///
    /// This is the only read path. Callers must not cache the result
    /// themselves: the whole point of the indirection is that the name is
    /// fixed for the lifetime of the page but not knowable in advance, so
    /// every consumer resolves it through here and the install path and the
    /// lookup path can never disagree about a name.
    ///
    /// # Returns
    ///
    /// - `&'static DomOpNames` - The cached names, shared by every caller.
    pub(crate) fn get() -> &'static DomOpNames {
        DOM_OP_NAMES.get_or_init(build_dom_op_names)
    }

    /// Installs `names` as the process-wide names, if none are set yet.
    ///
    /// The installer calls this before it touches `globalThis`, so the names
    /// are fixed before the first property read. `OnceLock`'s run-once
    /// guarantee is what makes that safe: a second call, or a call after
    /// [`get`] has already initialised the cell, is refused rather than
    /// allowed to swap the names out from under a live table.
    ///
    /// # Arguments
    ///
    /// - `DomOpNames` - The names to publish.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if these names were installed, `false` if names were
    ///   already present.
    pub(crate) fn set(names: DomOpNames) -> bool {
        DOM_OP_NAMES.set(names).is_ok()
    }

    /// Builds a fresh name set from the current clock, without caching it.
    ///
    /// Split out from [`get`] so a caller that needs to *claim* the names can
    /// build a set and hand it to [`set`] rather than letting `get` build one
    /// implicitly. Building costs one clock read and one encode; it happens
    /// once per process, on the first patch.
    ///
    /// # Returns
    ///
    /// - `DomOpNames` - A newly built, uncached name set.
    pub(crate) fn build() -> DomOpNames {
        build_dom_op_names()
    }
}
