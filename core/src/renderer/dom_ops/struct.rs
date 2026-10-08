use super::*;

/// Cached table of JS batched DOM-op helpers.
///
/// Resolved lazily on first patch via [`ensure_dom_op_table`]. The
/// functions accept parallel arrays / op-tuples so a single JS-side
/// loop replaces N individual `setAttribute` / `removeAttribute` /
/// `insertBefore` / `appendChild` / `removeChild` crossings.
#[derive(Clone)]
pub(crate) struct DomOpTable {
    /// `setAttribute` batch helper: `(elem, names[], values[])`.
    pub(crate) set_attrs: Function,
    /// `removeAttribute` batch helper: `(elem, names[])`.
    pub(crate) remove_attrs: Function,
    /// Child-mutation batch helper: `(parent, ops[])`.
    pub(crate) child_ops: Function,
}

thread_local! {
    /// Per-thread cache for the JS batched DOM-op table.
    ///
    /// The cache holds three wasm-bindgen `Function` handles. Those are JS
    /// object references, not raw pointers, and they are `!Send` — which is
    /// exactly the case a `thread_local!` is for. The previous
    /// `DomOpTableCell(UnsafeCell<Option<DomOpTable>>)` +
    /// `unsafe impl Sync for DomOpTableCell {}` pair existed only to smuggle
    /// a `Function` through a `static`; with `thread_local!` the `RefCell`
    /// provides the same lazy-init-once behaviour while keeping the borrow
    /// check the `unsafe` had bypassed. A second thread installing its own
    /// table is harmless: each thread caches the same
    /// `globalThis.__euv_dom_ops__` object.
    ///
    /// The table is resolved lazily on the first patch via
    /// `Reflect::get(globalThis, "__euv_dom_ops__")` (or installed if
    /// missing); subsequent patches reuse the cached functions without any
    /// further global lookup.
    pub(crate) static DOM_OP_TABLE: RefCell<Option<DomOpTable>> = const { RefCell::new(None) };
}

/// The per-load set of names the batched DOM-op helpers are published under.
///
/// Every name is `__euv_<role>_<encoded suffix>`: the prefix makes the
/// injection identifiable, the role says which helper it is, and the suffix
/// is what makes the full name unguessable. The suffix is derived once per
/// page load from a microsecond clock and encoded through
/// `bin-encode-decode`, so an attacker who ships a payload knowing only the
/// crate version cannot pre-compute the name it needs to overwrite.
///
/// The three helper names and the table name share one suffix rather than
/// drawing independent ones, so the whole set is still a single random draw
/// and the four names stay visibly related when inspecting `globalThis`.
///
/// Constructed once and cached in `DOM_OP_NAMES`; read through
/// [`DomOpNames::get`].
#[derive(Clone)]
pub(crate) struct DomOpNames {
    /// Key of the helper-table object on `globalThis`.
    pub(crate) table: String,
    /// Key of the batched `setAttribute` helper inside the table.
    pub(crate) set_attrs: String,
    /// Key of the batched `removeAttribute` helper inside the table.
    pub(crate) remove_attrs: String,
    /// Key of the batched child-mutation helper inside the table.
    pub(crate) child_ops: String,
}
