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

/// `Sync` wrapper around `Option<DomOpTable>` for `thread_local!`
/// storage.
pub(crate) struct DomOpTableCell(pub(crate) UnsafeCell<Option<DomOpTable>>);

thread_local! {
    /// Per-thread cache for the JS batched DOM-op table. The first
    /// patch triggers the `Reflect::get(globalThis, "__euv_dom_ops__")`
    /// lookup (or installs the helpers if missing); subsequent patches
    /// reuse the cached functions without any further global lookup.
    pub static DOM_OP_TABLE_CELL: DomOpTableCell =
        DomOpTableCell(const { UnsafeCell::new(None) });
}
