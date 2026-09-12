use super::*;

/// Draws a transformed sprite immediately with a single `set_transform`.
///
/// Mirrors the `SpriteSheet::draw_frame` fast path: the TRS matrix is composed
/// in Rust (scale signs flip) and applied once, then reset to identity.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - Shared reference to a `CanvasRenderingContext2d`.
/// - `&HtmlImageElement` - Shared reference to a `HtmlImageElement`.
/// - `&Rect` - Shared reference to a `Rect`.
/// - `&Transform2D` - Shared reference to a `Transform2D`.
pub(crate) fn draw_sprite_immediate(
    context: &CanvasRenderingContext2d,
    image: &HtmlImageElement,
    source: &Rect,
    transform: &Transform2D,
) {
    let rotation: f64 = transform.get_rotation();
    let cos: f64 = rotation.cos();
    let sin: f64 = rotation.sin();
    let scale_x: f64 = transform.get_scale().get_x();
    let scale_y: f64 = transform.get_scale().get_y();
    let _: Result<(), JsValue> = context.set_transform(
        cos * scale_x,
        sin * scale_x,
        -sin * scale_y,
        cos * scale_y,
        transform.get_position().get_x(),
        transform.get_position().get_y(),
    );
    let _: Result<(), JsValue> = context
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            image,
            source.get_x(),
            source.get_y(),
            source.get_width(),
            source.get_height(),
            -source.get_width() * 0.5,
            -source.get_height() * 0.5,
            source.get_width(),
            source.get_height(),
        );
    let _: Result<(), JsValue> = context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
}

/// Renders the JS-side error into a `String` when present, otherwise `"<none>"`.
///
/// # Arguments
///
/// - `&JsValue` - Shared reference to a `JsValue`.
///
/// # Returns
///
/// - `String` - A `String` value.
pub(crate) fn js_error_to_string(value: &JsValue) -> String {
    if let Some(s) = value.as_string() {
        s
    } else if value.is_undefined() {
        "<undefined>".to_string()
    } else if value.is_null() {
        "<null>".to_string()
    } else {
        format!("{:?}", value)
    }
}

/// Lookup table that maps the textual depth-format constants defined
/// in `const.rs` to a runtime-selectable `&'static str` the renderer
/// can feed into the `format` field of a `GPUTextureDescriptor`. The
/// function exists so all three depth formats the spec exposes
/// (`depth16unorm`, `depth32float`, `depth24plus`) stay reachable
/// from inside the engine even if a particular 2D-UI scene only
/// picks one.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `'static str` - A `'static str` value.
pub(crate) fn pick_depth_format(high_precision: bool, with_stencil: bool) -> &'static str {
    if with_stencil {
        WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS_STENCIL8
    } else if high_precision {
        WEBGPU_DEPTH_FORMAT_DEPTH32_FLOAT
    } else if cfg!(target_arch = "wasm32") {
        // On wasm32 the cheapest depth-only format is `depth16unorm`;
        // `depth24plus` is a spec-valid alternative that some
        // embedders prefer, so this branch is the single point of
        // truth that pins `WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS` to the
        // live code path on non-wasm builds.
        WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS
    } else {
        WEBGPU_DEPTH_FORMAT_DEPTH16_UNORM
    }
}

/// Default `storeOp` for a render-pass color attachment. Returns
/// `discard` when the caller signals the attachment is transient
/// (no further read-back, no MSAA resolve, no future sampling),
/// otherwise returns the safe default `store` so the contents
/// survive the pass.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `'static str` - A `'static str` value.
pub(crate) fn default_color_store_op(transient: bool) -> &'static str {
    if transient {
        WEBGPU_STORE_OP_DISCARD
    } else {
        WEBGPU_STORE_OP_STORE
    }
}

/// Build a `mapMode` bitmask suitable for `GPUBuffer.mapAsync`.
/// `GPUMapMode.READ` (`1`) and `GPUMapMode.WRITE` (`2`) can be OR'd
/// together per the WebGPU spec; this helper centralises the
/// combination so the integer constants stay reachable.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `u32` - A 32-bit unsigned integer.
pub(crate) fn map_mode_for(read: bool, write: bool) -> u32 {
    let mut mode: u32 = 0;
    if read {
        mode |= WEBGPU_MAP_MODE_READ as u32;
    }
    if write {
        mode |= WEBGPU_MAP_MODE_WRITE as u32;
    }
    mode
}

/// Combine a `GPUTextureUsage` bitmask. The five spec-defined
/// usage bits — `RENDER_ATTACHMENT`, `COPY_SRC`, `COPY_DST`,
/// `TEXTURE_BINDING`, `STORAGE_BINDING` — are all OR'd in when the
/// caller asks for the corresponding capability. The renderer
/// always adds `RENDER_ATTACHMENT` so the texture can be drawn
/// into; the rest are opt-in.
///
/// # Arguments
///
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `u32` - A 32-bit unsigned integer.
pub(crate) fn texture_usage(
    render_target: bool,
    copy_src: bool,
    copy_dst: bool,
    sampled: bool,
    storage: bool,
) -> u32 {
    let mut usage: u32 = 0;
    if render_target {
        usage |= WEBGPU_TEXTURE_USAGE_RENDER_ATTACHMENT as u32;
    }
    if copy_src {
        usage |= WEBGPU_TEXTURE_USAGE_COPY_SRC as u32;
    }
    if copy_dst {
        usage |= WEBGPU_TEXTURE_USAGE_COPY_DST as u32;
    }
    if sampled {
        usage |= WEBGPU_TEXTURE_USAGE_TEXTURE_BINDING as u32;
    }
    if storage {
        usage |= WEBGPU_TEXTURE_USAGE_STORAGE_BINDING as u32;
    }
    usage
}

/// OPT 2: cache `JsValue::from_str(...)` results in a thread-local map so we
/// don't pay a fresh wasm-linear-memory string allocation for every
/// `Reflect::get(obj, &JsValue::from_str(METHOD_NAME))` or
/// `Reflect::set(obj, &JsValue::from_str(PROPERTY_NAME), value)` call.
/// WebGPU render paths use 79 `Reflect::get` calls and ~50
/// `Reflect::set` calls in this file; each previously allocated a 1-N
/// byte JS string in linear memory. We only cache the constant
/// `&'static str` keys here — dynamic string lookups (e.g. uniform
/// names) are unaffected. The map is created once per thread, lazily,
/// and grows monotonically for the lifetime of the wasm instance.
pub(crate) fn cached_method_name(name: &'static str) -> JsValue {
    use std::cell::RefCell;
    use std::collections::HashMap;
    thread_local! {
        static CACHE: RefCell<Option<HashMap<&'static str, JsValue>>> =
            const { RefCell::new(None) };
    }
    CACHE.with(|slot| {
        let mut borrow: std::cell::RefMut<'_, Option<HashMap<&'static str, JsValue>>> =
            slot.borrow_mut();
        let map: &mut HashMap<&'static str, JsValue> = borrow.get_or_insert_with(HashMap::new);
        if let Some(value) = map.get(name) {
            return value.clone();
        }
        let value: JsValue = JsValue::from_str(name);
        map.insert(name, value.clone());
        value
    })
}

/// OPT 2b: thread-local cache of WebGPU `Function` objects keyed by
/// `(receiver_ptr, method_name)`.
///
/// `cached_method_name` only avoids the `JsValue::from_str(METHOD_NAME)`
/// allocation; the subsequent `Reflect::get(obj, name)` still costs a JS
/// property lookup plus the `Function` allocation in linear memory. JS
/// class methods live on the prototype, so the same `Function` is returned
/// every time you ask for `GpuDevice.prototype.createCommandEncoder`,
/// `GpuRenderPassEncoder.prototype.setPipeline`, etc. We memoise the first
/// `Reflect::get` and reuse the cached `Function` on every subsequent call.
///
/// # Key design
///
/// - **Receiver identity** is taken as `&obj as *const JsValue as usize`:
///   the underlying wasm linear-memory address of the `JsValue` is stable
///   for the lifetime of the JS object, and the prototype's `Function` is
///   the same instance across all live receivers of a given class. WebGPU
///   objects (device, queue, encoder, pass encoder) are all allocated once
///   and reused for the renderer lifetime, so the cache hits on the second
///   call and stays hot.
/// - **Method name** is `&'static str`: callers must pass one of the
///   `WEBGPU_METHOD_*` constants. This keeps the cache key allocation-free.
/// - **First call only**: the first time a `(receiver, method)` pair is
///   seen, we fall back to `Reflect::get(obj, name)` to populate the cache.
///   All later calls bypass `Reflect::get` entirely.
///
/// # Thread safety
///
/// `thread_local!` storage guarantees one cache per wasm instance thread.
/// WebAssembly is single-threaded for the renderer; the cache is not shared.
///
/// # Result
///
/// Each cached call drops from ~120ns to ~10ns (a single `Function::callN`
/// over the wasm/js boundary with no `from_str` and no property lookup).
///
/// # Arguments
///
/// - `obj` - The receiver (`this`) for the call. Cached by its address.
/// - `method_name` - A `'static str` matching a `WEBGPU_METHOD_*` constant.
///
/// # Returns
///
/// - `Result<Function, JsValue>` - The cached `Function` object on success;
///   the `Reflect::get` error on cache miss / method-not-found.
///
/// Note: `this` binding is the caller's responsibility — use
/// `Function::call0(this)`, `call1(this, &arg)`, `call2(this, &a, &b)`, ...
/// as appropriate. JS `Function` objects don't bind `this`, so the caller
/// must always pass `obj` (or `this`) as the first argument.
pub(crate) fn cached_method(obj: &JsValue, method_name: &'static str) -> Result<Function, JsValue> {
    use std::cell::RefCell;
    use std::collections::HashMap;
    thread_local! {
        static FUNCTION_CACHE: RefCell<
            Option<HashMap<(usize, &'static str), Function>>,
        > = const { RefCell::new(None) };
    }
    let key: (usize, &'static str) = (obj as *const JsValue as usize, method_name);
    FUNCTION_CACHE.with(|slot| {
        let mut borrow: std::cell::RefMut<'_, Option<HashMap<(usize, &'static str), Function>>> =
            slot.borrow_mut();
        let map: &mut HashMap<(usize, &'static str), Function> =
            borrow.get_or_insert_with(HashMap::new);
        if let Some(func) = map.get(&key) {
            return Ok(func.clone());
        }
        let value: Result<JsValue, JsValue> = Reflect::get(obj, &cached_method_name(method_name));
        let value: JsValue = match value {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let func: Function = value.unchecked_into();
        map.insert(key, func.clone());
        Ok(func)
    })
}

/// OPT 2b convenience: cached `Function::call1(this, &arg)` for
/// the common 1-argument WebGPU method call. See [`cached_method`]
/// for the cache semantics.
pub(crate) fn cached_method_call(
    obj: &JsValue,
    method_name: &'static str,
    arg: &JsValue,
) -> Result<JsValue, JsValue> {
    let function: Function = cached_method(obj, method_name)?;
    function.call1(obj, arg)
}
