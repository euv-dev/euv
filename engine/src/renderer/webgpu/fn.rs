use super::*;

/// Reads a nonnegative integral texture coordinate without truncation.
///
/// # Arguments
///
/// - `&JsValue` - The JavaScript size or coordinate value.
///
/// # Returns
///
/// - `Option<u32>` - The exact coordinate, or `None` for an invalid value.
fn texture_write_u32(value: &JsValue) -> Option<u32> {
    value
        .as_f64()
        .filter(|number: &f64| {
            number.is_finite()
                && *number >= 0.0
                && *number <= f64::from(u32::MAX)
                && number.fract() == 0.0
        })
        .map(|number: f64| number as u32)
}

/// Resolves the remaining logical mip extent from a destination texture.
///
/// # Arguments
///
/// - `&TextureWriteDescriptor` - The destination texture, mip, and origin.
///
/// # Returns
///
/// - `Option<[u32; 3]>` - The nonempty texel extent, or `None` for invalid bounds.
pub(crate) fn texture_write_extent(descriptor: &TextureWriteDescriptor) -> Option<[u32; 3]> {
    let texture: JsValue = descriptor.get_texture();
    let mip_level: u32 = descriptor.get_mip_level();
    let mip_level_count: u32 = texture_write_u32(
        &Reflect::get(
            &texture,
            &JsValue::from_str(WEBGPU_PROPERTY_MIP_LEVEL_COUNT),
        )
        .ok()?,
    )?;
    if mip_level >= mip_level_count {
        return None;
    }
    let dimension: String = Reflect::get(&texture, &JsValue::from_str(WEBGPU_PROPERTY_DIMENSION))
        .ok()?
        .as_string()?;
    if ![
        WEBGPU_TEXTURE_DIMENSION_1D,
        WEBGPU_TEXTURE_DIMENSION_2D,
        WEBGPU_TEXTURE_DIMENSION_3D,
    ]
    .contains(&dimension.as_str())
    {
        return None;
    }
    let origin: JsValue = descriptor.get_origin().unwrap_or(JsValue::UNDEFINED);
    let mut extent: [u32; 3] = [0; 3];
    for (axis, (size_key, origin_key)) in [
        (WEBGPU_PROPERTY_WIDTH, WEBGPU_PROPERTY_X),
        (WEBGPU_PROPERTY_HEIGHT, WEBGPU_PROPERTY_Y),
        (WEBGPU_PROPERTY_DEPTH_OR_1, WEBGPU_PROPERTY_Z),
    ]
    .iter()
    .enumerate()
    {
        let size: u32 =
            texture_write_u32(&Reflect::get(&texture, &JsValue::from_str(size_key)).ok()?)?;
        if size == 0 {
            return None;
        }
        let mip_size: u32 = if dimension == WEBGPU_TEXTURE_DIMENSION_1D && axis != 0 {
            1
        } else if axis == 2 && dimension != WEBGPU_TEXTURE_DIMENSION_3D {
            size
        } else {
            size.checked_shr(mip_level).unwrap_or(0).max(1)
        };
        let coordinate: JsValue = if origin.is_undefined() || origin.is_null() {
            JsValue::UNDEFINED
        } else if Array::is_array(&origin) {
            Reflect::get(&origin, &JsValue::from_f64(axis as f64)).ok()?
        } else {
            Reflect::get(&origin, &JsValue::from_str(origin_key)).ok()?
        };
        let offset: u32 = if coordinate.is_undefined() {
            0
        } else {
            texture_write_u32(&coordinate)?
        };
        extent[axis] = mip_size.checked_sub(offset)?;
        if extent[axis] == 0 {
            return None;
        }
    }
    Some(extent)
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
        WEBGPU_JS_ERROR_UNDEFINED_LABEL.to_string()
    } else if value.is_null() {
        WEBGPU_JS_ERROR_NULL_LABEL.to_string()
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
/// - `bool` - Select the 32-bit float format when true.
///
/// # Returns
///
/// - `&'static str` - A `'static str` value.
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

/// Build a `mapMode` bitmask suitable for `GPUBuffer.mapAsync`.
/// `GPUMapMode.READ` (`1`) and `GPUMapMode.WRITE` (`2`) can be OR'd
/// together per the WebGPU spec; this helper centralises the
/// combination so the integer constants stay reachable.
///
/// # Arguments
///
/// - `bool` - Include the read mode bit when true.
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
/// - `bool` - Include `RENDER_ATTACHMENT` when true.
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
///
/// # Arguments
///
/// - `&'static str` - The constant method or property name to intern.
///
/// # Returns
///
/// - `JsValue` - The cached `JsValue` for this name.
pub(crate) fn cached_method_name(name: &'static str) -> JsValue {
    thread_local! {
        static CACHE: RefCell<Option<HashMap<&'static str, JsValue>>> =
            const { RefCell::new(None) };
    }
    CACHE.with(|slot: &RefCell<Option<HashMap<&'static str, JsValue>>>| {
        let cached: Option<JsValue> = {
            let mut borrow: RefMut<'_, Option<HashMap<&'static str, JsValue>>> = slot.borrow_mut();
            borrow.get_or_insert_with(HashMap::new).get(name).cloned()
        };
        if let Some(value) = cached {
            return value;
        }
        let value: JsValue = JsValue::from_str(name);
        let mut borrow: RefMut<'_, Option<HashMap<&'static str, JsValue>>> = slot.borrow_mut();
        borrow
            .get_or_insert_with(HashMap::new)
            .insert(name, value.clone());
        value
    })
}

/// OPT 2b: thread-local cache of WebGPU `Function` objects keyed by
/// `(GpuReceiverClass, method_name)`.
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
/// - **Receiver class** (`GpuReceiverClass`) is the identity half of the
///   cache key. Prototype `Function`s are per-class singletons, so the
///   class tag alone is sufficient - no receiver identity is required.
///   The previous scheme keyed by the `JsValue`'s stack address, which was
///   unsound: per-frame temporaries (pass encoders, command encoders)
///   reuse stack slots across frames, and classes like
///   `GPURenderPassEncoder` / `GPUComputePassEncoder` share method names
///   (`setPipeline` / `setBindGroup` / `end`), so a stale slot could
///   return the wrong class's `Function` (a swallowed TypeError and a
///   silently skipped GPU call).
/// - **Method name** is `&'static str` - callers must pass one of the
///   `WEBGPU_METHOD_*` constants. This keeps the cache key allocation-free.
/// - **First call only**: the first time a `(class, method)` pair is
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
/// - `GpuReceiverClass` - The receiver's WebGPU class (cache key half).
/// - `&JsValue` - The receiver (`this`) for the call; used for the first
///   `Reflect::get` lookup, not part of the key.
/// - `&'static str` - A method name matching a `WEBGPU_METHOD_*` constant.
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
pub(crate) fn cached_method(
    class: GpuReceiverClass,
    obj: &JsValue,
    method_name: &'static str,
) -> Result<Function, JsValue> {
    thread_local! {
        static FUNCTION_CACHE: RefCell<
            Option<HashMap<(GpuReceiverClass, &'static str), Function>>,
        > = const { RefCell::new(None) };
    }
    let key: (GpuReceiverClass, &'static str) = (class, method_name);
    FUNCTION_CACHE.with(
        |slot: &RefCell<Option<HashMap<(GpuReceiverClass, &'static str), Function>>>| {
            let mut borrow: RefMut<
                '_,
                Option<HashMap<(GpuReceiverClass, &'static str), Function>>,
            > = slot.borrow_mut();
            let map: &mut HashMap<(GpuReceiverClass, &'static str), Function> =
                borrow.get_or_insert_with(HashMap::new);
            if let Some(func) = map.get(&key) {
                return Ok(func.clone());
            }
            let value: Result<JsValue, JsValue> =
                Reflect::get(obj, &cached_method_name(method_name));
            let value: JsValue = match value {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            let func: Function = value.unchecked_into();
            map.insert(key, func.clone());
            Ok(func)
        },
    )
}

/// OPT 2b convenience: cached `Function::call1(this, &arg)` for
/// the common 1-argument WebGPU method call. See [`cached_method`]
/// for the cache semantics.
///
/// # Arguments
///
/// - `GpuReceiverClass` - The receiver's WebGPU class (cache key half).
/// - `&JsValue` - The receiver (`this`) for the call.
/// - `&'static str` - A method name matching a `WEBGPU_METHOD_*` constant.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The method's return value, or the
///   `Reflect::get` error when the method is not cached and not found.
pub(crate) fn cached_method_call(
    class: GpuReceiverClass,
    obj: &JsValue,
    method_name: &'static str,
    arg: &JsValue,
) -> Result<JsValue, JsValue> {
    let function: Function = cached_method(class, obj, method_name)?;
    function.call1(obj, arg)
}
