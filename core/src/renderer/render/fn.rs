use super::*;

/// Returns the cached `Document` for the current page, falling back to
/// `window().document()` on the first call. `Document` is page-scoped (it
/// stays valid until the document is replaced), so a single resolved
/// reference is safe to reuse across the lifetime of an `euv-example`
/// mount. Subsequent calls just clone the cached handle, eliminating the
/// two JS-boundary crossings (`window()` + `document()`) every DOM node
/// creation used to pay.
///
/// OPT 8: per-page `Document` cache via `thread_local!`. The lazy
/// `OnceCell`-style fallback makes this safe even before
/// `App::mount` has finished initialising.
///
/// # Returns
///
/// - `Option<Document>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn cached_document() -> Option<Document> {
    DOCUMENT_CACHE.with(|cell: &UnsafeCell<Option<Document>>| {
        let cached_ptr: *mut Option<Document> = cell.get();
        unsafe {
            if let Some(doc) = &*cached_ptr {
                return Some(doc.clone());
            }
        }
        let window_value: Window = window()?;
        let document: Document = window_value.document()?;
        DOCUMENT_CACHE.with(|cell: &UnsafeCell<Option<Document>>| unsafe {
            *cell.get() = Some(document.clone());
        });
        Some(document)
    })
}

/// Appends a sequence of pre-built DOM nodes to a parent element.
///
/// OPT 13: when the input contains two or more nodes, the writes are
/// funnelled through a `DocumentFragment` so the parent only sees a
/// single `append_child` call. The browser then performs one layout
/// invalidation for the whole batch instead of one per node — typically
/// a 2-10× wall-clock win on tree mounts with many siblings (e.g.
/// euv-example's 77-div initial render).
///
/// When the input has zero or one nodes the helper falls back to the
/// direct `append_child` path so the single-child case pays zero
/// fragment-allocation overhead.
///
/// # Arguments
///
/// - `&Element` - The parent DOM element receiving the children.
/// - `impl IntoIterator<Item = Node>` - The DOM nodes to attach, in
///   their final sibling order.
///
/// # Returns
///
/// - `()` - The appends are best-effort; per-call JS errors are dropped
///   to match the previous per-node behaviour.
pub(crate) fn append_nodes(parent: &Element, nodes: impl IntoIterator<Item = Node>) {
    let mut iter = nodes.into_iter();
    let Some(first) = iter.next() else {
        return;
    };
    let Some(second) = iter.next() else {
        let _: Result<Node, JsValue> = parent.append_child(&first);
        return;
    };
    // Two or more children: build a fragment, append every node into it,
    // then graft the fragment onto the parent in a single JS round-trip.
    let document: Document = match cached_document() {
        Some(doc) => doc,
        None => {
            // Without a Document we can't make a fragment — fall back
            // to per-node appends to preserve the old behaviour rather
            // than silently dropping children.
            let _: Result<Node, JsValue> = parent.append_child(&first);
            let _: Result<Node, JsValue> = parent.append_child(&second);
            for node in iter {
                let _: Result<Node, JsValue> = parent.append_child(&node);
            }
            return;
        }
    };
    let fragment: DocumentFragment = document.create_document_fragment();
    let _: Result<Node, JsValue> = fragment.append_child(&first);
    let _: Result<Node, JsValue> = fragment.append_child(&second);
    for node in iter {
        let _: Result<Node, JsValue> = fragment.append_child(&node);
    }
    let fragment_node: Node = fragment.into();
    let _: Result<Node, JsValue> = parent.append_child(&fragment_node);
}
