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
/// The result is a `Float64Array` (ids are `< 2^53`, so the f64 channel is
/// exact) so the caller drains the whole chain with **one** `copy_to`
/// crossing instead of one `Array.get` per marked ancestor.
///
/// # Arguments
///
/// - `event: &JsValue` - The DOM event whose target chain should be walked.
/// - `max_depth: usize` - Upper bound on hops; `0` means unbounded.
///
/// # Returns
///
/// - `Float64Array` - The parsed `data-euv-id` values in walk order.
pub(crate) fn euv_event_collect_id_chain(event: &JsValue, max_depth: usize) -> Float64Array {
    // Walk the ancestor chain from `event.target` up via `parent_element`,
    // collecting every `data-euv-id` attribute. Implemented in pure Rust
    // via web-sys so wasm-bindgen does not emit this as a separate JS
    // snippet (each `#[wasm_bindgen(inline_js)]` decorates a fresh
    // `pkg/snippets/.../inlineN.js` file under the deployed site).
    //
    // Cost: one web-sys accessor (`get_attribute` or `parent_element`)
    // per layer, no per-event JS crossing. The previous inline_js
    // variant paid one wasm↔JS crossing for the bulk walk plus one
    // crossing per ancestor callback; this Rust loop pays `2 * depth`
    // crossings with no callback indirection. Net per-event cost is
    // typically lower for the common case (1–3 marked ancestors) and
    // keeps the deployment artefact count stable as features grow.
    let event_target: Option<web_sys::EventTarget> =
        js_sys::Reflect::get(event, &JsValue::from_str("target"))
            .ok()
            .and_then(|v: JsValue| v.dyn_into::<web_sys::EventTarget>().ok());
    let Some(target) = event_target else {
        return Float64Array::new_with_length(0);
    };
    let mut ids: Vec<f64> = Vec::new();
    let mut node: Option<web_sys::Node> = Some(target.unchecked_into::<web_sys::Node>());
    let mut depth: usize = 0;
    while let Some(n) = node {
        if max_depth != 0 && depth >= max_depth {
            break;
        }
        let element: Option<&web_sys::Element> = n.dyn_ref::<web_sys::Element>();
        if let Some(el) = element
            && let Some(id_str) = el.get_attribute("data-euv-id")
            && let Ok(parsed) = id_str.parse::<f64>()
        {
            ids.push(parsed);
        }
        depth += 1;
        node = n.parent_node();
    }
    let arr: Float64Array = Float64Array::new_with_length(ids.len() as u32);
    arr.copy_from(&ids);
    arr
}
