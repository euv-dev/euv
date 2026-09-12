use super::*;

/// Collects the `data-euv-id` chain of an event's ancestor path entirely in
/// JS, returning the ids in walk order (target first, `<html>` last).
///
/// This replaces the previous Rust-side loop in `dispatch_delegated_event`
/// that walked the ancestor chain one layer per round-trip (`get_attribute`
/// + `parent_element` = 2 JS crossings per layer; a depth-10 click cost 20
/// crossings), and the later callback-based variant that still paid one
/// JS→WASM callback invocation per marked ancestor PLUS a full
/// `HandlerRegistryMap` clone and one `Closure` allocation per event.
/// Collecting the id chain in JS collapses the walk to a single
/// `#[wasm_bindgen]` call per event and lets Rust look up at most one
/// handler against the live registry — no per-event registry snapshot.
///
/// `max_depth` caps the ancestor walk; passing `0` (per the call-site
/// convention in `dispatch_delegated_event`) means "walk until `<html>`".
///
/// # Arguments
///
/// - `event: &JsValue` - The DOM event whose target chain should be walked.
/// - `max_depth: usize` - Upper bound on hops; `0` means unbounded.
///
/// # Returns
///
/// - `Array` - The parsed `data-euv-id` values in walk order.
#[wasm_bindgen(inline_js = r#"
export function euv_event_collect_id_chain(event, max_depth) {
    const ids = [];
    let node = event.target;
    let depth = 0;
    // Unbounded when max_depth is 0 (per the call site convention in
    // dispatch_delegated_event — passing 0 means "walk until <html>").
    // Otherwise count `event.target` itself as depth 1.
    while (node) {
        if (max_depth !== 0 && depth >= max_depth) {
            break;
        }
        // Only DOM Elements carry data-euv-id; skip text nodes cheaply.
        if (node.nodeType === 1) {
            const id = node.getAttribute && node.getAttribute("data-euv-id");
            if (id !== null && id !== undefined && id !== "") {
                // parseInt is native (no string alloc on the WASM side);
                // NaN check guards against malformed attribute values.
                const parsed = parseInt(id, 10);
                if (!isNaN(parsed)) {
                    ids.push(parsed);
                }
            }
        }
        node = node.parentElement;
        depth += 1;
    }
    return ids;
}
"#)]
extern "C" {
    pub(crate) fn euv_event_collect_id_chain(event: &JsValue, max_depth: usize) -> Array;
}
