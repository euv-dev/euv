use super::*;

/// The JS source injected once at startup to define the global walker.
///
/// The function walks `event.composedPath()` and collects every
/// `data-euv-id` value on the chain in one pass, so the per-event cost is
/// a single WASM↔JS crossing instead of two per ancestor layer
/// (`get_attribute` + `parent_node`). Ids are returned as a plain `Array`
/// of numbers: they are `usize` values well below 2^53, so they survive
/// the trip through `Float64Array` exactly, matching what the Rust caller
/// already expects.
///
/// Injecting a global at startup rather than using
/// `#[wasm_bindgen(inline_js)]` keeps the deployed artefact count stable:
/// every `inline_js` item would emit its own `pkg/snippets/.../inlineN.js`
/// file, so the file count would grow with the number of features. This
/// function is installed once, from Rust, at mount time and is then called
/// by name like any other global.
const EVENT_ID_CHAIN_JS: &str = r#"(globalThis.__euvEventIdChain = function (event, maxDepth) {
  const path = (typeof event.composedPath === 'function')
    ? event.composedPath()
    : (function () { const out = []; let n = event.target; while (n) { out.push(n); n = n.parentNode; } return out; })();
  const ids = [];
  for (let i = 0; i < path.length; i++) {
    if (maxDepth !== 0 && i >= maxDepth) { break; }
    const el = path[i];
    if (el.nodeType !== 1) { continue; }
    const id = el.getAttribute('data-euv-id');
    if (id !== null && id !== '') { ids.push(parseFloat(id)); }
  }
  return ids;
});
"#;

/// Resolves the JS `globalThis` handle for the injected-walker lookup.
///
/// Mirrors the resolver in `renderer::dom_ops` so the registry module does
/// not have to widen that helper's visibility. Falls back to `window` for
/// hosts without `globalThis`.
///
/// # Returns
///
/// - `Option<JsValue>` - The global object, or `None` when neither
///   `globalThis` nor `window` is reachable.
fn global_this() -> Option<JsValue> {
    if let Ok(value) = js_sys::eval(GLOBAL_THIS_NAME)
        && !value.is_undefined()
    {
        return Some(value);
    }
    let window_value: Window = window()?;
    Some(window_value.into())
}

/// Injects the global id-chain walker exactly once per page.
///
/// Safe to call repeatedly: the second and later calls are a single
/// global-property probe with no side effects. A page that navigates
/// within the SPA keeps the same global, so re-entry is a no-op.
///
/// # Returns
///
/// - `bool` - `true` when the global is present and callable after the call.
pub(crate) fn ensure_event_id_chain_global() -> bool {
    let Some(global_value) = global_this() else {
        return false;
    };
    let already_present: bool = js_sys::Reflect::get(
        &global_value,
        &JsValue::from_str(EVENT_ID_CHAIN_GLOBAL_NAME),
    )
    .map(|value: JsValue| value.is_function())
    .unwrap_or(false);
    if already_present {
        return true;
    }
    js_sys::eval(EVENT_ID_CHAIN_JS).is_ok()
}

/// Returns the cached injected walker, resolving the global on first use.
///
/// Falls back to `None` when the global is missing (for example a host that
/// strips it, or a caller that dispatches events before mount), letting the
/// caller use the Rust-side walk instead of panicking.
///
/// # Returns
///
/// - `Option<Function>` - The injected walker, or `None` when unavailable.
fn event_id_chain_fn() -> Option<Function> {
    EVENT_ID_CHAIN_FN.with(|cell: &RefCell<Option<Option<Function>>>| {
        if let Some(resolved) = cell.borrow().clone() {
            return resolved;
        }
        let resolved: Option<Function> = global_this()
            .and_then(|global_value: JsValue| {
                js_sys::Reflect::get(
                    &global_value,
                    &JsValue::from_str(EVENT_ID_CHAIN_GLOBAL_NAME),
                )
                .ok()
            })
            .filter(|value: &JsValue| value.is_function())
            .and_then(|value: JsValue| value.dyn_into::<Function>().ok());
        let mut slot: RefMut<'_, Option<Option<Function>>> = cell.borrow_mut();
        *slot = Some(resolved.clone());
        resolved
    })
}

/// Collects the `data-euv-id` chain of an event's propagation path,
/// returning the ids in walk order (target first, `<html>` last).
///
/// This is the OPT 40 hot path for every delegated DOM event. The walk
/// itself runs in the injected global, so the per-event cost is one
/// WASM↔JS crossing for the whole chain rather than two crossings per
/// ancestor layer. It replaces a Rust loop that issued `get_attribute` +
/// `parent_node` per layer — a click in the 402-node example page measured
/// **71 `getAttribute` calls** before the change.
///
/// `max_depth` caps the walk; `0` means "walk the whole path". On failure
/// (global missing, non-array result, or a JS throw) the function falls
/// back to the pure-Rust ancestor walk so behaviour never regresses to
/// "events silently stop working".
///
/// # Arguments
///
/// - `&JsValue` - The DOM event whose propagation path is walked.
/// - `usize` - Upper bound on hops; `0` means unbounded.
///
/// # Returns
///
/// - `Float64Array` - The parsed `data-euv-id` values in walk order.
pub(crate) fn euv_event_collect_id_chain(event: &JsValue, max_depth: usize) -> Float64Array {
    if let Some(walker) = event_id_chain_fn()
        && let Some(global_value) = global_this()
        && let Ok(result) = walker.call2(&global_value, event, &JsValue::from_f64(max_depth as f64))
        && let Some(array) = result.dyn_ref::<js_sys::Array>()
    {
        let length: u32 = array.length();
        if length > 0 {
            let out: Float64Array = Float64Array::new_with_length(length);
            // One bulk copy for the whole chain: ids are `< 2^53`, exact in
            // f64, so a single `copy_from` replaces one `Array.get` per id.
            let mut ids: Vec<f64> = Vec::with_capacity(length as usize);
            for index in 0..length {
                match array.get(index).as_f64() {
                    Some(value) => ids.push(value),
                    // A non-numeric entry means the injected helper and this
                    // module disagree; drop the whole fast path rather than
                    // feed a NaN into the handler registry.
                    None => return collect_id_chain_rust(event, max_depth),
                }
            }
            out.copy_from(&ids);
            return out;
        }
        return Float64Array::new_with_length(0);
    }
    collect_id_chain_rust(event, max_depth)
}

/// Pure-Rust ancestor walk, used when the injected global is unavailable.
///
/// Costs two JS crossings per ancestor layer, so it exists only as a
/// correctness fallback for hosts where the startup injection did not run
/// (a stripped global, or events dispatched before mount).
///
/// # Arguments
///
/// - `&JsValue` - The DOM event whose target chain is walked.
/// - `usize` - Upper bound on hops; `0` means unbounded.
///
/// # Returns
///
/// - `Float64Array` - The parsed `data-euv-id` values in walk order.
fn collect_id_chain_rust(event: &JsValue, max_depth: usize) -> Float64Array {
    let event_target: Option<EventTarget> =
        js_sys::Reflect::get(event, &JsValue::from_str(EVENT_TARGET_PROP))
            .ok()
            .and_then(|value: JsValue| value.dyn_into::<EventTarget>().ok());
    let Some(target) = event_target else {
        return Float64Array::new_with_length(0);
    };
    let mut ids: Vec<f64> = Vec::new();
    let mut node: Option<Node> = Some(target.unchecked_into::<Node>());
    let mut depth: usize = 0;
    while let Some(current) = node {
        if max_depth != 0 && depth >= max_depth {
            break;
        }
        let element: Option<&Element> = current.dyn_ref::<Element>();
        if let Some(el) = element
            && let Some(id_str) = el.get_attribute(DATA_EUV_ID)
            && let Ok(parsed) = id_str.parse::<f64>()
        {
            ids.push(parsed);
        }
        depth += 1;
        node = current.parent_node();
    }
    let out: Float64Array = Float64Array::new_with_length(ids.len() as u32);
    out.copy_from(&ids);
    out
}
