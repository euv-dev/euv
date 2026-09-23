use super::*;

/// SAFETY: `DomOpTableCell` is only mutated through `UnsafeCell`
/// interior-mutability on the WASM single-threaded runtime.
unsafe impl Sync for DomOpTableCell {}

/// Resolves (or installs on first call) the batched DOM-op helpers
/// under `globalThis.__euv_dom_ops__`.
///
/// The function table holds three JS functions:
///
/// - `set_attrs(elem, names, values)` — batched `setAttribute`.
/// - `remove_attrs(elem, names)` — batched `removeAttribute`.
/// - `child_ops(parent, ops)` — batched `insertBefore`/`appendChild`/
///   `removeChild`.
///
/// All three are constructed via `js_sys::eval` so the helpers work
/// without any setup beyond a single `globalThis` lookup. The first
/// call performs the installation; subsequent calls return the cached
/// table in a single `Reflect::get` per process.
///
/// Returns `None` if `globalThis` is unreachable (e.g. SSR). Callers
/// fall back to the per-op `web_sys` path in that case.
///
/// # Returns
///
/// - `Option<DomOpTable>` - The function table, or `None` if it could
///   not be resolved.
pub(crate) fn ensure_dom_op_table() -> Option<DomOpTable> {
    DOM_OP_TABLE_CELL.with(|cell: &DomOpTableCell| {
        let cached_ptr: *mut Option<DomOpTable> = cell.0.get();
        unsafe {
            if let Some(table) = &*cached_ptr {
                return Some(table.clone());
            }
        }
        let global_value: JsValue = global_this()?;
        let table_value: JsValue =
            match Reflect::get(&global_value, &JsValue::from_str(JS_DOM_OP_TABLE)) {
                Ok(existing) => existing,
                Err(_err) => JsValue::UNDEFINED,
            };
        let table: DomOpTable = if table_value.is_object() {
            let set_attrs: Function =
                match Reflect::get(&table_value, &JsValue::from_str(JS_DOM_OP_SET_ATTRS)) {
                    Ok(value) => match value.dyn_into::<Function>() {
                        Ok(function) => function,
                        Err(_) => return None,
                    },
                    Err(_err) => return None,
                };
            let remove_attrs: Function =
                match Reflect::get(&table_value, &JsValue::from_str(JS_DOM_OP_REMOVE_ATTRS)) {
                    Ok(value) => match value.dyn_into::<Function>() {
                        Ok(function) => function,
                        Err(_) => return None,
                    },
                    Err(_err) => return None,
                };
            let child_ops: Function =
                match Reflect::get(&table_value, &JsValue::from_str(JS_DOM_OP_CHILD_OPS)) {
                    Ok(value) => match value.dyn_into::<Function>() {
                        Ok(function) => function,
                        Err(_) => return None,
                    },
                    Err(_err) => return None,
                };
            DomOpTable {
                set_attrs,
                remove_attrs,
                child_ops,
            }
        } else {
            install_dom_op_table(&global_value)?
        };
        unsafe {
            *cached_ptr = Some(table.clone());
        }
        Some(table)
    })
}

/// Installs the batched DOM-op helpers onto `globalThis.__euv_dom_ops__`.
///
/// The helpers are intentionally tiny — they just loop over the passed
/// arrays calling `setAttribute` / `removeAttribute` / `insertBefore` /
/// `appendChild` / `removeChild` on the element. Equivalent JS cost to
/// the per-op path (N attribute writes inside the function instead of N
/// JS round-trips) but only one crossing to enter the function.
fn install_dom_op_table(global_value: &JsValue) -> Option<DomOpTable> {
    let set_attrs_source: &str = "function(elem, names, values) { \
        for (var i = 0; i < names.length; i++) { \
            elem.setAttribute(names[i], values[i]); \
        } \
    }";
    let remove_attrs_source: &str = "function(elem, names) { \
        for (var i = 0; i < names.length; i++) { \
            elem.removeAttribute(names[i]); \
        } \
    }";
    let child_ops_source: &str = "function(parent, ops) { \
        for (var i = 0; i < ops.length; i++) { \
            var op = ops[i]; \
            try { \
                if (op[0] === 0) { \
                    parent.insertBefore(op[1], op[2]); \
                } else if (op[0] === 1) { \
                    parent.appendChild(op[1]); \
                } else { \
                    parent.removeChild(op[1]); \
                } \
            } catch (e) { \
                // One failing op (e.g. a stale reference node) must not \
                // abort the rest of the batch; the Rust fallback path \
                // drops per-op errors the same way. \
            } \
        } \
    }";
    let set_attrs: Function = eval_function(set_attrs_source)?;
    let remove_attrs: Function = eval_function(remove_attrs_source)?;
    let child_ops: Function = eval_function(child_ops_source)?;
    let table_value: JsValue = js_sys::Object::new().into();
    let _ = Reflect::set(
        &table_value,
        &JsValue::from_str(JS_DOM_OP_SET_ATTRS),
        set_attrs.as_ref(),
    );
    let _ = Reflect::set(
        &table_value,
        &JsValue::from_str(JS_DOM_OP_REMOVE_ATTRS),
        remove_attrs.as_ref(),
    );
    let _ = Reflect::set(
        &table_value,
        &JsValue::from_str(JS_DOM_OP_CHILD_OPS),
        child_ops.as_ref(),
    );
    let _ = Reflect::set(
        global_value,
        &JsValue::from_str(JS_DOM_OP_TABLE),
        &table_value,
    );
    Some(DomOpTable {
        set_attrs,
        remove_attrs,
        child_ops,
    })
}

/// Compiles a JS function body via `js_sys::eval`.
fn eval_function(body: &str) -> Option<Function> {
    let wrapped: String = format!("({})", body);
    js_sys::eval(&wrapped)
        .ok()
        .and_then(|value: JsValue| value.dyn_into::<Function>().ok())
}

/// Resolves the JS `globalThis` handle.
///
/// Falls back to `window` when `globalThis` is not present (older
/// Safari / non-browser WASM hosts). Returns `None` if neither is
/// available.
fn global_this() -> Option<JsValue> {
    if let Ok(value) = js_sys::eval("globalThis")
        && !value.is_undefined() {
            return Some(value);
        }
    let window: Window = window()?;
    Some(window.into())
}

/// Returns `true` when the named attribute requires the form-property
/// dispatch path (i.e. setting `.value` / `.checked` / `.disabled` /
/// `.selected` / `.readonly` / `.multiple` on the DOM element instead
/// of calling `setAttribute`). For these attributes the renderer must
/// keep the per-element direct call to preserve the previous semantics
/// — batched `setAttribute` would silently break input/textarea/etc.
pub(crate) fn is_property_attr(name: &str) -> bool {
    name == ATTR_VALUE
        || name == ATTR_CHECKED
        || name == ATTR_DISABLED
        || name == ATTR_SELECTED
        || name == ATTR_READONLY
        || name == ATTR_MULTIPLE
}

/// Applies a batch of `setAttribute(name, value)` writes to `elem` via a
/// single JS-side function call.
///
/// On any failure (table unavailable, JS exception), the function falls
/// back to per-op `web_sys::Element::set_attribute` calls.
///
/// # Arguments
///
/// - `&Element` - The element to mutate.
/// - `&[(String, String)]` - The `(name, value)` pairs to set.
pub(crate) fn apply_set_attr_batch(element: &Element, ops: &[(String, String)]) {
    if ops.is_empty() {
        return;
    }
    let Some(table) = ensure_dom_op_table() else {
        for (name, value) in ops {
            let _: Result<(), JsValue> = element.set_attribute(name, value);
        }
        return;
    };
    let names: js_sys::Array = js_sys::Array::new_with_length(ops.len() as u32);
    let values: js_sys::Array = js_sys::Array::new_with_length(ops.len() as u32);
    for (index, (name, value)) in ops.iter().enumerate() {
        names.set(index as u32, JsValue::from_str(name));
        values.set(index as u32, JsValue::from_str(value));
    }
    let element_value: JsValue = element.clone().into();
    let result: Result<JsValue, JsValue> = table.set_attrs.call3(
        &JsValue::UNDEFINED,
        &element_value,
        names.as_ref(),
        values.as_ref(),
    );
    if result.is_err() {
        for (name, value) in ops {
            let _: Result<(), JsValue> = element.set_attribute(name, value);
        }
    }
}

/// Applies a batch of `removeAttribute(name)` writes to `elem` via a
/// single JS-side function call.
///
/// On any failure (table unavailable, JS exception), the function falls
/// back to per-op `web_sys::Element::remove_attribute` calls.
///
/// # Arguments
///
/// - `&Element` - The element to mutate.
/// - `&[String]` - The attribute names to remove.
pub(crate) fn apply_remove_attr_batch(element: &Element, ops: &[String]) {
    if ops.is_empty() {
        return;
    }
    let Some(table) = ensure_dom_op_table() else {
        for name in ops {
            let _: Result<(), JsValue> = element.remove_attribute(name);
        }
        return;
    };
    let names: js_sys::Array = js_sys::Array::new_with_length(ops.len() as u32);
    for (index, name) in ops.iter().enumerate() {
        names.set(index as u32, JsValue::from_str(name));
    }
    let element_value: JsValue = element.clone().into();
    let result: Result<JsValue, JsValue> =
        table
            .remove_attrs
            .call2(&JsValue::UNDEFINED, &element_value, names.as_ref());
    if result.is_err() {
        for name in ops {
            let _: Result<(), JsValue> = element.remove_attribute(name);
        }
    }
}

/// Applies a batch of child-mutation ops to `parent` via a single
/// JS-side function call.
///
/// Each op is encoded as a 3-element `Array`:
/// - `[0, node, refNode]` → `parent.insertBefore(node, refNode)`
/// - `[1, node, null]` → `parent.appendChild(node)`
/// - `[2, node, null]` → `parent.removeChild(node)`
///
/// On any failure (table unavailable, JS exception), the function falls
/// back to per-op `web_sys` calls.
///
/// # Arguments
///
/// - `&Element` - The parent element whose children are being mutated.
/// - `&[ChildOp]` - The ops to apply, in order.
pub(crate) fn apply_child_ops_batch(parent: &Element, ops: &[ChildOp]) {
    if ops.is_empty() {
        return;
    }
    let Some(table) = ensure_dom_op_table() else {
        for op in ops {
            apply_child_op_fallback(parent, op);
        }
        return;
    };
    let ops_array: js_sys::Array = js_sys::Array::new_with_length(ops.len() as u32);
    for (index, op) in ops.iter().enumerate() {
        let tuple: js_sys::Array = js_sys::Array::new_with_length(3);
        let kind: u32 = match op {
            ChildOp::InsertBefore { .. } => 0,
            ChildOp::AppendChild(_) => 1,
            ChildOp::RemoveChild(_) => 2,
        };
        let primary: JsValue = match op {
            ChildOp::InsertBefore { node, .. } => node.clone().into(),
            ChildOp::AppendChild(node) => node.clone().into(),
            ChildOp::RemoveChild(node) => node.clone().into(),
        };
        let reference: JsValue = match op {
            ChildOp::InsertBefore { reference, .. } => {
                reference.clone().map(Node::into).unwrap_or(JsValue::NULL)
            }
            _ => JsValue::NULL,
        };
        tuple.set(0, JsValue::from(kind));
        tuple.set(1, primary);
        tuple.set(2, reference);
        ops_array.set(index as u32, tuple.into());
    }
    let parent_value: JsValue = parent.clone().into();
    let result: Result<JsValue, JsValue> =
        table
            .child_ops
            .call2(&JsValue::UNDEFINED, &parent_value, ops_array.as_ref());
    if result.is_err() {
        for op in ops {
            apply_child_op_fallback(parent, op);
        }
    }
}

/// Per-op fallback used when the JS-side batched call fails. Mirrors
/// the previous patch-path behaviour exactly.
fn apply_child_op_fallback(parent: &Element, op: &ChildOp) {
    match op {
        ChildOp::InsertBefore { node, reference } => {
            let _: Result<Node, JsValue> = match reference {
                Some(reference_node) => parent.insert_before(node, Some(reference_node)),
                None => parent.append_child(node),
            };
        }
        ChildOp::AppendChild(node) => {
            let _: Result<Node, JsValue> = parent.append_child(node);
        }
        ChildOp::RemoveChild(node) => {
            let _: Result<Node, JsValue> = parent.remove_child(node);
        }
    }
}
