use super::*;

/// Describes a thrown [`JsValue`] without ever losing it.
///
/// # Arguments
///
/// - `&JsValue` - The thrown value.
///
/// # Returns
///
/// - `String` - The value rendered as a string, or a debug rendering
///   when the value is not convertible.
pub(crate) fn describe_webgpu(reason: &JsValue) -> String {
    match reason.as_string() {
        Some(text) if !text.is_empty() => text,
        _ => format!("{reason:?}"),
    }
}

/// Builds a plain JS object from key/value pairs.
///
/// Every WebGPU descriptor is built through this one constructor, so a
/// missing key is impossible by construction and no descriptor is
/// assembled half-populated.
///
/// # Arguments
///
/// - `&[(&str, JsValue)]` - The property names and values.
///
/// # Returns
///
/// - `Result<Object, JsValue>` - The assembled object.
pub(crate) fn descriptor(pairs: &[(&str, JsValue)]) -> Result<Object, JsValue> {
    let object: Object = Object::new();
    for (key, value) in pairs {
        Reflect::set(object.as_ref(), &JsValue::from_str(key), value)?;
    }
    Ok(object)
}

/// Reads a property off a WebGPU handle without ever calling it.
///
/// `queue` and `limits` are attributes on `GPUDevice`, not methods, so
/// they are read here rather than invoked: calling them throws
/// `queue is not callable` before the backend has allocated anything.
///
/// # Arguments
///
/// - `&JsValue` - The handle to read from.
/// - `&str` - The property name.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The property value, or the reason the
///   read threw.
pub(crate) fn attribute(target: &JsValue, name: &str) -> Result<JsValue, JsValue> {
    let value: JsValue = match Reflect::get(target, &JsValue::from_str(name)) {
        Ok(read) => read,
        Err(reason) => {
            return Err(backend_error(
                ERROR_WEBGPU_CALL,
                format!("{name} could not be read: {}", describe_webgpu(&reason)),
            ));
        }
    };
    if value.is_undefined() || value.is_null() {
        return Err(backend_error(
            ERROR_WEBGPU_CALL,
            format!("{name} is missing from this WebGPU object"),
        ));
    }
    Ok(value)
}

/// Looks up a method on a WebGPU handle.
///
/// # Arguments
///
/// - `&JsValue` - The handle to look the method up on.
/// - `&str` - The method name.
///
/// # Returns
///
/// - `Result<Function, JsValue>` - The bound method, or the reason it is
///   absent.
pub(crate) fn method(target: &JsValue, name: &str) -> Result<Function, JsValue> {
    let key: JsValue = JsValue::from_str(name);
    match Reflect::get(target, &key) {
        Ok(value) => match value.dyn_into() {
            Ok(function) => Ok(function),
            Err(_) => Err(backend_error(
                ERROR_WEBGPU_CALL,
                format!("{name} is not callable"),
            )),
        },
        Err(reason) => Err(backend_error(ERROR_WEBGPU_CALL, describe_webgpu(&reason))),
    }
}

/// Invokes a looked-up method and propagates a throw as a
/// [`backend_error`](super::backend_error).
///
/// # Arguments
///
/// - `&JsValue` - The handle to call on.
/// - `&str` - The method name, used in the diagnostic.
/// - `&JsValue` - The single argument.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The call's return value.
pub(crate) fn call1(target: &JsValue, name: &str, argument: &JsValue) -> Result<JsValue, JsValue> {
    let function: Function = method(target, name)?;
    match function.call1(target, argument) {
        Ok(value) => Ok(value),
        Err(reason) => Err(backend_error(
            ERROR_WEBGPU_CALL,
            format!("{name} threw: {}", describe_webgpu(&reason)),
        )),
    }
}

/// Calls a zero-argument method and propagates a throw.
///
/// # Arguments
///
/// - `&JsValue` - The handle to call on.
/// - `&str` - The method name.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The call's return value.
pub(crate) fn call0(target: &JsValue, name: &str) -> Result<JsValue, JsValue> {
    let function: Function = method(target, name)?;
    match function.call0(target) {
        Ok(value) => Ok(value),
        Err(reason) => Err(backend_error(
            ERROR_WEBGPU_CALL,
            format!("{name} threw: {}", describe_webgpu(&reason)),
        )),
    }
}

/// Calls a method with two arguments.
///
/// `setBindGroup` takes `(index, bindGroup)` plus an optional dynamic
/// offset array; passing an array as a single argument leaves `index`
/// undefined and the whole command buffer invalid.
///
/// # Arguments
///
/// - `&JsValue` - The handle to call on.
/// - `&str` - The method name.
/// - `&JsValue` - The first argument.
/// - `&JsValue` - The second argument.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The call's return value.
pub(crate) fn call2(
    target: &JsValue,
    name: &str,
    first: &JsValue,
    second: &JsValue,
) -> Result<JsValue, JsValue> {
    let function: Function = method(target, name)?;
    match function.call2(target, first, second) {
        Ok(value) => Ok(value),
        Err(reason) => Err(backend_error(
            ERROR_WEBGPU_CALL,
            format!("{name} threw: {}", describe_webgpu(&reason)),
        )),
    }
}

/// Issues a non-indexed draw of the geometry a render pass has bound.
///
/// `draw` takes `(vertexCount, instanceCount, firstVertex,
/// firstInstance)` as four separate arguments; handed one array instead,
/// it reads `vertexCount` as `undefined` and issues no primitives at
/// all. The four counts therefore stay four separate `Function::call4`
/// arguments here — bundling them into one JS value would silently
/// produce a frame with nothing drawn — while the Rust side carries them
/// as one [`DrawArgs`] so the counts cannot be transposed at the call
/// site.
///
/// # Arguments
///
/// - `&JsValue` - The render pass to draw in.
/// - `&DrawArgs` - The vertex count, instance count, and the first
///   vertex and instance the pass starts from.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The call's return value.
pub(crate) fn draw_vertices(pass: &JsValue, args: &DrawArgs) -> Result<JsValue, JsValue> {
    let function: Function = method(pass, BACKEND_WEBGPU_METHOD_DRAW)?;
    let vertex_count: JsValue = JsValue::from_f64(f64::from(args.get_vertex_count()));
    let instance_count: JsValue = JsValue::from_f64(f64::from(args.get_instance_count()));
    let first_vertex: JsValue = JsValue::from_f64(f64::from(args.get_first_vertex()));
    let first_instance: JsValue = JsValue::from_f64(f64::from(args.get_first_instance()));
    match function.call4(
        pass,
        &vertex_count,
        &instance_count,
        &first_vertex,
        &first_instance,
    ) {
        Ok(value) => Ok(value),
        Err(reason) => Err(backend_error(
            ERROR_WEBGPU_CALL,
            format!(
                "{BACKEND_WEBGPU_METHOD_DRAW} threw: {}",
                describe_webgpu(&reason)
            ),
        )),
    }
}

/// Awaits a JS promise, propagating its rejection.
///
/// # Arguments
///
/// - `JsValue` - The promise, or anything the call returned.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The resolved value.
pub(crate) async fn await_promise(pending: JsValue) -> Result<JsValue, JsValue> {
    let promise: Promise = pending.unchecked_into();
    match JsFuture::from(promise).await {
        Ok(value) => Ok(value),
        Err(reason) => Err(backend_error(ERROR_WEBGPU_CALL, describe_webgpu(&reason))),
    }
}

/// Closes a validation error scope and reports what it caught.
///
/// Popping is checked at all three of its layers rather than on a
/// best-effort basis, because the three failures mean different things
/// and two of them must not read as a clean scope. A throw from
/// `popErrorScope` means the scope was never closed at all; a rejected
/// promise means it was closed but its verdict is lost. Either way the
/// device never told this backend whether the scoped work was valid, so
/// both are errors in their own right instead of a `None` that would be
/// indistinguishable from "the scope was clean".
///
/// Only the third layer is a verdict: a resolved `GPUError` is the
/// message the scope caught, and an absent one means the device
/// validated the scoped work and found nothing wrong with it.
///
/// # Arguments
///
/// - `&JsValue` - The device whose error-scope stack is popped.
///
/// # Returns
///
/// - `Result<Option<String>, JsValue>` - The validation message the
///   scope caught, or `None` when the scope was clean.
pub(crate) async fn pop_validation_scope(device: &JsValue) -> Result<Option<String>, JsValue> {
    let pending: JsValue = call0(device, BACKEND_WEBGPU_METHOD_POP_SCOPE)?;
    let resolved: JsValue = await_promise(pending).await?;
    Ok(scope_message(&resolved))
}

/// Reads `navigator.gpu`, reporting a clear error when WebGPU is
/// absent.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The `GPU` object, or the reason it is
///   unreachable.
pub(crate) fn navigator_gpu() -> Result<JsValue, JsValue> {
    let Some(window) = window() else {
        return Err(backend_error(
            ERROR_WEBGPU_UNAVAILABLE,
            "no window is available".to_string(),
        ));
    };
    let navigator: Navigator = window.navigator();
    let key: JsValue = JsValue::from_str(BACKEND_WEBGPU_NAVIGATOR_KEY);
    let lookup: Result<JsValue, JsValue> = Reflect::get(navigator.as_ref(), &key);
    let gpu: JsValue = match lookup {
        Ok(value) => value,
        Err(reason) => {
            return Err(backend_error(
                ERROR_WEBGPU_UNAVAILABLE,
                describe_webgpu(&reason),
            ));
        }
    };
    if gpu.is_undefined() || gpu.is_null() {
        return Err(backend_error(
            ERROR_WEBGPU_UNAVAILABLE,
            ERROR_WEBGPU_UNAVAILABLE.to_string(),
        ));
    }
    Ok(gpu)
}

/// Reads the preferred canvas format from the `GPU` object.
///
/// # Arguments
///
/// - `&JsValue` - The `GPU` object.
///
/// # Returns
///
/// - `String` - The lowercase format name exactly as the browser
///   reported it, or `bgra8unorm` when it exposes no getter.
pub(crate) fn preferred_format(gpu: &JsValue) -> String {
    match call0(gpu, BACKEND_WEBGPU_PREFERRED_FORMAT) {
        Ok(value) => match value.as_string() {
            Some(text) => text,
            None => BACKEND_WEBGPU_FORMAT_BGRA8_UNORM.to_string(),
        },
        Err(_) => BACKEND_WEBGPU_FORMAT_BGRA8_UNORM.to_string(),
    }
}

/// Configures the canvas context for the device and swap-chain format.
///
/// The same format string configures the context and names the
/// pipeline's colour target, because a mismatch invalidates the whole
/// command buffer with no visible error.
///
/// # Arguments
///
/// - `&HtmlCanvasElement` - The canvas to configure.
/// - `&JsValue` - The device owning the swap chain.
/// - `&str` - The lowercase preferred format.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The configured context.
pub(crate) fn configure_context(
    canvas: &HtmlCanvasElement,
    device: &JsValue,
    format_name: &str,
) -> Result<JsValue, JsValue> {
    let lookup: Result<Option<Object>, JsValue> = canvas.get_context(WEBGPU_CONTEXT_ID);
    let context: JsValue = match lookup {
        Ok(Some(object)) => object.into(),
        Ok(None) => {
            return Err(backend_error(
                ERROR_WEBGPU_CONTEXT,
                ERROR_WEBGPU_CONTEXT.to_string(),
            ));
        }
        Err(reason) => {
            return Err(backend_error(
                ERROR_WEBGPU_CONTEXT,
                describe_webgpu(&reason),
            ));
        }
    };
    let configuration: Object = descriptor(&[
        (BACKEND_WEBGPU_PROPERTY_DEVICE, device.clone()),
        (
            BACKEND_WEBGPU_PROPERTY_FORMAT,
            JsValue::from_str(format_name),
        ),
        (
            BACKEND_WEBGPU_PROPERTY_ALPHA_MODE,
            JsValue::from_str(BACKEND_WEBGPU_ALPHA_OPAQUE),
        ),
        (
            BACKEND_WEBGPU_PROPERTY_USAGE,
            JsValue::from_f64(BACKEND_WEBGPU_TEXTURE_RENDER_ATTACHMENT as f64),
        ),
    ])?;
    match call1(
        &context,
        BACKEND_WEBGPU_METHOD_CONFIGURE,
        configuration.as_ref(),
    ) {
        Ok(_) => Ok(context),
        Err(reason) => Err(backend_error(
            ERROR_WEBGPU_CONTEXT,
            describe_webgpu(&reason),
        )),
    }
}

/// Rounds a packed float count up to the byte size a storage buffer
/// needs.
///
/// # Arguments
///
/// - `usize` - The packed float count.
///
/// # Returns
///
/// - `u32` - The byte size to allocate, saturated at the 32-bit
///   descriptor width.
pub(crate) fn packed_byte_length(floats: usize) -> u32 {
    let bytes: u64 = (floats as u64).saturating_mul(BACKEND_WEBGPU_F32_SIZE as u64);
    bytes.min(u32::MAX as u64) as u32
}

/// Reads a numeric property off a WebGPU handle.
///
/// # Arguments
///
/// - `&JsValue` - The handle to read from.
/// - `&str` - The property name.
///
/// # Returns
///
/// - `Option<f64>` - The numeric value, or `None` when it is absent or
///   not a number.
pub(crate) fn number_property(target: &JsValue, name: &str) -> Option<f64> {
    match Reflect::get(target, &JsValue::from_str(name)) {
        Ok(value) => value.as_f64(),
        Err(_) => None,
    }
}

/// Reads one device limit in bytes or pixels.
///
/// A limit below one, or one that does not fit the 32-bit descriptor
/// width the rest of the backend uses, is treated as absent rather than
/// clamped: a device reporting nonsense must not turn into a silently
/// wrong ceiling.
///
/// # Arguments
///
/// - `&JsValue` - The device to query.
/// - `&str` - The limit name.
///
/// # Returns
///
/// - `u32` - The limit, or zero when the device reports none.
pub(crate) fn device_limit(device: &JsValue, name: &str) -> u32 {
    let limits: JsValue = match attribute(device, BACKEND_WEBGPU_PROPERTY_LIMITS) {
        Ok(value) => value,
        Err(_) => return 0,
    };
    match number_property(&limits, name) {
        Some(value) if value >= 1.0 && value <= u32::MAX as f64 => value as u32,
        _ => 0,
    }
}

/// Registers the device's loss and uncaptured-error handlers.
///
/// `GPUDevice.lost` is a promise that is always present, so its
/// existence proves nothing and must never be read as a liveness
/// signal. The reason arrives asynchronously instead, and the handler
/// below is what turns it into a state the next synchronous call can
/// reject with. The `uncapturederror` listener covers the other
/// direction: a validation failure that never throws is otherwise an
/// entirely silent black frame.
///
/// The loss callback is a one-shot callback owned by JavaScript, built
/// with `Closure::once_into_js`. `device.lost` settles exactly once —
/// and destroying the device settles it — so the browser may invoke
/// the callback at any point up to and including
/// [`WebGpuRayTraceBackend::release`](super::WebGpuRayTraceBackend::release),
/// while the Rust data it captures is freed the moment it does. Keeping
/// it in a struct field instead is what produced `closure invoked
/// recursively or after being dropped`: the field was cleared before
/// `destroy`, while the browser still held the very callback being
/// torn down.
///
/// The `uncapturederror` listener is not one-shot, so it is owned by
/// JavaScript too and its handle is returned so
/// [`unregister_error_listener`](super::unregister_error_listener)
/// can remove it: the device keeps a strong reference until then, so
/// the callback cannot outlive the Rust data it captures.
///
/// # Arguments
///
/// - `&JsValue` - The device to observe.
/// - `&Rc<RefCell<Option<String>>>` - The record both handlers write.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The registered `uncapturederror`
///   listener, which
///   [`unregister_error_listener`](super::unregister_error_listener)
///   must remove before the device is destroyed.
pub(crate) fn observe_device(
    device: &JsValue,
    failure: &Rc<RefCell<Option<String>>>,
) -> Result<JsValue, JsValue> {
    let record: Rc<RefCell<Option<String>>> = Rc::clone(failure);
    let lost_handler: JsValue = Closure::once_into_js(move |info: JsValue| -> () {
        record_loss(&record, &info);
    });
    let lost: JsValue = attribute(device, BACKEND_WEBGPU_PROPERTY_LOST)?;
    let chained: Function = method(&lost, BACKEND_WEBGPU_METHOD_THEN)?;
    let pending: JsValue = match chained.call1(&lost, lost_handler.as_ref()) {
        Ok(value) => value,
        Err(reason) => {
            return Err(backend_error(
                ERROR_WEBGPU_CALL,
                format!("device.lost.then threw: {}", describe_webgpu(&reason)),
            ));
        }
    };
    // The chained promise settles with the loss reason itself and
    // carries nothing this backend needs, so it is dropped rather than
    // awaited: awaiting it here would report the loss as an error
    // during construction, before the caller had a renderer to reject.
    drop(pending);
    register_error_listener(device, Rc::clone(failure))
}

/// Registers the `uncapturederror` listener on a device.
///
/// # Arguments
///
/// - `&JsValue` - The device to observe.
/// - `Rc<RefCell<Option<String>>>` - The record the handler writes.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The registered listener handle.
fn register_error_listener(
    device: &JsValue,
    failure: Rc<RefCell<Option<String>>>,
) -> Result<JsValue, JsValue> {
    let function: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new(move |event: JsValue| {
        record_error_event(&failure, &event);
    }) as Box<dyn FnMut(JsValue)>);
    let function_value: JsValue = function.into_js_value();
    let add: Function = method(device, BACKEND_WEBGPU_METHOD_ADD_LISTENER)?;
    let registered: Result<JsValue, JsValue> = add.call3(
        device,
        &JsValue::from_str(BACKEND_WEBGPU_EVENT_UNCAPTURED),
        function_value.as_ref(),
        &JsValue::UNDEFINED,
    );
    registered?;
    Ok(function_value)
}

/// Removes a registered `uncapturederror` listener from its device.
///
/// The device holds its own strong reference to the listener until it
/// is removed, so leaving it registered after the backend released its
/// buffers would keep a Rust callback reachable from a device this
/// backend no longer owns. Removal happens while the device is still
/// alive because a destroyed device cannot invoke it, and a device
/// that rejects the removal is already unusable.
///
/// # Arguments
///
/// - `&JsValue` - The device to stop observing.
/// - `&JsValue` - The listener handle `register_error_listener`
///   returned.
pub(crate) fn unregister_error_listener(device: &JsValue, listener: &JsValue) {
    if let Ok(remove) = method(device, BACKEND_WEBGPU_METHOD_REMOVE_LISTENER) {
        let removed: Result<JsValue, JsValue> = remove.call2(
            device,
            &JsValue::from_str(BACKEND_WEBGPU_EVENT_UNCAPTURED),
            listener,
        );
        let _: Result<JsValue, JsValue> = removed;
    }
}

/// Records the diagnostic an `uncapturederror` event carries.
///
/// # Arguments
///
/// - `&Rc<RefCell<Option<String>>>` - The shared record.
/// - `&JsValue` - The dispatched event.
fn record_error_event(failure: &Rc<RefCell<Option<String>>>, event: &JsValue) {
    let detail: String = match attribute(event, BACKEND_WEBGPU_PROPERTY_ERROR) {
        Ok(error) => scope_message(&error).unwrap_or_else(|| WEBGPU_SCOPE_SILENT.to_string()),
        Err(reason) => describe_webgpu(&reason),
    };
    record_failure(failure, detail);
}

/// Records a device loss as a terminal failure.
///
/// The loss overwrites any transient validation error already in the
/// record. Recording only when the record is empty would let a
/// `uncapturederror` from an earlier command mask the loss entirely —
/// and that record *is* drained, so the device loss would disappear
/// with it and later calls would report success on a dead device. A
/// loss is strictly more severe than anything already recorded, so it
/// is the one event allowed to replace what is there.
///
/// # Arguments
///
/// - `&Rc<RefCell<Option<String>>>` - The shared record.
/// - `&JsValue` - The resolved `GPUDeviceLostInfo`.
fn record_loss(failure: &Rc<RefCell<Option<String>>>, info: &JsValue) {
    let detail: String = lost_reason(info);
    if let Ok(mut cell) = failure.try_borrow_mut() {
        *cell = Some(detail);
    }
}

/// Records a failure the first time it is seen.
///
/// # Arguments
///
/// - `&Rc<RefCell<Option<String>>>` - The shared record.
/// - `String` - The diagnostic to remember.
fn record_failure(failure: &Rc<RefCell<Option<String>>>, detail: String) {
    if let Ok(mut cell) = failure.try_borrow_mut()
        && cell.is_none()
    {
        *cell = Some(detail);
    }
}

/// Extracts the reason a device was lost.
///
/// # Arguments
///
/// - `&JsValue` - The resolved `GPUDeviceLostInfo`.
///
/// # Returns
///
/// - `String` - The reason, or the placeholder when the object carried
///   none.
pub(crate) fn lost_reason(info: &JsValue) -> String {
    match attribute(info, BACKEND_WEBGPU_LOST_REASON) {
        Ok(reason) => match reason.as_string() {
            Some(text) if !text.is_empty() => {
                format!("{ERROR_WEBGPU_DEVICE_LOST}: {text}")
            }
            _ => ERROR_WEBGPU_DEVICE_LOST.to_string(),
        },
        Err(reason) => format!("{ERROR_WEBGPU_DEVICE_LOST}: {}", describe_webgpu(&reason)),
    }
}

/// Reports whether a recorded diagnostic describes a lost device.
///
/// The record mixes two events that need opposite lifetimes: a lost
/// device is permanent, while an uncaptured validation error concerns
/// one command. Telling them apart by the prefix the loss handler
/// writes keeps the permanent one permanent without adding a second
/// field whose two halves could disagree.
///
/// # Arguments
///
/// - `&str` - The recorded diagnostic.
///
/// # Returns
///
/// - `bool` - `true` when the device itself was lost.
pub(crate) fn is_device_lost(recorded: &str) -> bool {
    recorded.starts_with(ERROR_WEBGPU_DEVICE_LOST)
}

/// Rejects an operation because the browser already reported the
/// device lost or the last command invalid.
///
/// # Arguments
///
/// - `Option<String>` - The recorded failure, when one is pending.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Err` carrying the recorded failure, or
///   `Ok` when nothing is pending.
pub(crate) fn rejection(pending: Option<String>) -> Result<(), JsValue> {
    match pending {
        Some(detail) => Err(backend_error(ERROR_WEBGPU_DEVICE_LOST, detail)),
        None => Ok(()),
    }
}

/// Populates a popped error scope with the validation message.
///
/// # Arguments
///
/// - `&JsValue` - The resolved `popErrorScope` value.
///
/// # Returns
///
/// - `Option<String>` - The message, or `None` when the scope was
///   clean.
pub(crate) fn scope_message(resolved: &JsValue) -> Option<String> {
    if resolved.is_null() || resolved.is_undefined() {
        return None;
    }
    match Reflect::get(resolved, &JsValue::from_str(BACKEND_WEBGPU_ERROR_MESSAGE)) {
        Ok(message) => match message.as_string() {
            Some(text) if !text.is_empty() => Some(text),
            _ => Some(WEBGPU_SCOPE_SILENT.to_string()),
        },
        Err(_) => Some(WEBGPU_SCOPE_SILENT.to_string()),
    }
}

/// Joins the shader compilation errors into one diagnostic.
///
/// Only `error`-kind messages are fatal; a warning must not fail an
/// otherwise usable module.
///
/// # Arguments
///
/// - `&JsValue` - The resolved compilation info.
///
/// # Returns
///
/// - `String` - Every error message, or an empty string when the module
///   compiled cleanly.
pub(crate) fn compilation_messages(info: &JsValue) -> String {
    let messages: JsValue =
        match Reflect::get(info, &JsValue::from_str(BACKEND_WEBGPU_PROPERTY_MESSAGES)) {
            Ok(value) => value,
            Err(_) => return String::new(),
        };
    let Ok(list) = messages.dyn_into::<Array>() else {
        return String::new();
    };
    let mut collected: String = String::new();
    for entry in list.iter() {
        let kind: String = match Reflect::get(
            &entry,
            &JsValue::from_str(BACKEND_WEBGPU_PROPERTY_MESSAGE_TYPE),
        ) {
            Ok(value) => value.as_string().unwrap_or_default(),
            Err(_) => String::new(),
        };
        if kind != BACKEND_WEBGPU_MESSAGE_ERROR {
            continue;
        }
        let text: String = match Reflect::get(
            &entry,
            &JsValue::from_str(BACKEND_WEBGPU_PROPERTY_MESSAGE_TEXT),
        ) {
            Ok(value) => value.as_string().unwrap_or_default(),
            Err(_) => String::new(),
        };
        let line: f64 = Reflect::get(&entry, &JsValue::from_str(BACKEND_WEBGPU_PROPERTY_LINE_NUM))
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or(0.0);
        collected.push_str(&format!("line {}: {text}\n", line as i64));
    }
    collected
}

/// Writes a packed block into a storage buffer in place.
///
/// The block is exposed to JS as a view over wasm linear memory rather
/// than copied into a fresh typed array: `writeBuffer` copies the bytes
/// synchronously, so a copy per frame would double the traffic of every
/// upload for nothing. No allocation and no `await` sit between the view
/// and the call that consumes it, so the memory cannot move underneath
/// it.
///
/// # Arguments
///
/// - `&JsValue` - The queue to write through.
/// - `&JsValue` - The destination buffer.
/// - `&[f32]` - The packed block.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` once the write was issued.
pub(crate) fn write_packed(
    queue: &JsValue,
    buffer: &JsValue,
    packed: &[f32],
) -> Result<(), JsValue> {
    if packed.is_empty() {
        return Ok(());
    }
    // SAFETY: the view aliases `packed` and is consumed by the
    // `writeBuffer` call on the next two lines. Nothing allocates, is
    // yielded to, or runs arbitrary JS in between, so linear memory
    // cannot grow and detach the view before `writeBuffer` has copied
    // it.
    let view: Float32Array = unsafe { Float32Array::view(packed) };
    let function: Function = method(queue, BACKEND_WEBGPU_METHOD_WRITE_BUFFER)?;
    match function.call3(queue, buffer, &JsValue::from_f64(0.0), view.as_ref()) {
        Ok(_) => Ok(()),
        Err(reason) => Err(backend_error(
            ERROR_WEBGPU_CALL,
            format!("writeBuffer threw: {}", describe_webgpu(&reason)),
        )),
    }
}

/// Destroys one buffer, reporting a failure instead of dropping it.
///
/// # Arguments
///
/// - `&JsValue` - The buffer to destroy.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` once the buffer is destroyed.
pub(crate) fn destroy_buffer(buffer: &JsValue) -> Result<(), JsValue> {
    match call0(buffer, BACKEND_WEBGPU_METHOD_DESTROY) {
        Ok(_) => Ok(()),
        Err(reason) => Err(backend_error(ERROR_WEBGPU_CALL, describe_webgpu(&reason))),
    }
}

/// Releases a device this backend requested.
///
/// # Arguments
///
/// - `&JsValue` - The device to destroy.
pub(crate) fn destroy_device(device: &JsValue) {
    let _: Result<JsValue, JsValue> = call0(device, BACKEND_WEBGPU_METHOD_DESTROY);
}

/// Releases everything a partially built backend has acquired so far.
///
/// WebGPU allocates nothing at `requestDevice` time, so a failure
/// after the device exists leaks exactly that device. Anything acquired
/// before the failure is released too, so a retry starts from a clean
/// slate instead of compounding the loss.
///
/// # Arguments
///
/// - `&JsValue` - The device to destroy.
/// - `Option<&JsValue>` - The scene buffer, when one exists.
/// - `Option<&JsValue>` - The frame buffer, when one exists.
/// - `Option<&JsValue>` - The configured context, when one exists.
///
/// # Returns
///
/// - `()` - Cleanup never fails; a half-built backend has no caller to
///   report to.
pub(crate) fn abandon(
    device: &JsValue,
    scene: Option<&JsValue>,
    frame: Option<&JsValue>,
    context: Option<&JsValue>,
) {
    if let Some(buffer) = scene {
        let _: Result<(), JsValue> = destroy_buffer(buffer);
    }
    if let Some(buffer) = frame {
        let _: Result<(), JsValue> = destroy_buffer(buffer);
    }
    if let Some(surface) = context {
        let _: Result<JsValue, JsValue> = call0(surface, BACKEND_WEBGPU_METHOD_UNCONFIGURE);
    }
    destroy_device(device);
}

/// Builds the bind-group layout both packed buffers declare.
///
/// # Arguments
///
/// - `&JsValue` - The device to declare the layout on.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The layout object.
pub(crate) fn packed_layout(device: &JsValue) -> Result<JsValue, JsValue> {
    let scene_entry: Object = descriptor(&[
        (
            BACKEND_WEBGPU_PROPERTY_BINDING,
            JsValue::from_f64(BACKEND_WEBGPU_BINDING_SCENE as f64),
        ),
        (
            BACKEND_WEBGPU_PROPERTY_VISIBILITY,
            JsValue::from_f64(BACKEND_WEBGPU_VISIBILITY_FRAGMENT as f64),
        ),
        (
            BACKEND_WEBGPU_PROPERTY_BUFFER,
            descriptor(&[(
                BACKEND_WEBGPU_PROPERTY_TYPE,
                JsValue::from_str(BACKEND_WEBGPU_BUFFER_READ_ONLY_STORAGE),
            )])?
            .into(),
        ),
    ])?;
    let frame_entry: Object = descriptor(&[
        (
            BACKEND_WEBGPU_PROPERTY_BINDING,
            JsValue::from_f64(BACKEND_WEBGPU_BINDING_FRAME as f64),
        ),
        (
            BACKEND_WEBGPU_PROPERTY_VISIBILITY,
            JsValue::from_f64(BACKEND_WEBGPU_VISIBILITY_FRAGMENT as f64),
        ),
        (
            BACKEND_WEBGPU_PROPERTY_BUFFER,
            descriptor(&[(
                BACKEND_WEBGPU_PROPERTY_TYPE,
                JsValue::from_str(BACKEND_WEBGPU_BUFFER_READ_ONLY_STORAGE),
            )])?
            .into(),
        ),
    ])?;
    let entries: Array = Array::of2(&scene_entry, &frame_entry);
    let layout_descriptor: Object =
        descriptor(&[(BACKEND_WEBGPU_PROPERTY_ENTRIES, entries.into())])?;
    call1(
        device,
        BACKEND_WEBGPU_METHOD_CREATE_LAYOUT,
        layout_descriptor.as_ref(),
    )
}

/// Builds the bind group pairing both packed storage buffers.
///
/// The resource is wrapped in `{ buffer }` rather than passed bare.
/// Newer specifications admit the shorthand, but the explicit form is
/// the one every shipping browser accepts, and a bare handle here
/// invalidates the group rather than raising anything catchable.
///
/// # Arguments
///
/// - `&JsValue` - The device to create the bind group on.
/// - `&JsValue` - The packed scene buffer.
/// - `&JsValue` - The packed frame buffer.
/// - `&JsValue` - The persistent layout.
///
/// # Returns
///
/// - `Result<JsValue, JsValue>` - The bind group.
pub(crate) fn bind_group_of(
    device: &JsValue,
    scene: &JsValue,
    frame: &JsValue,
    layout: &JsValue,
) -> Result<JsValue, JsValue> {
    let scene_resource: Object = descriptor(&[(BACKEND_WEBGPU_PROPERTY_BUFFER, scene.clone())])?;
    let frame_resource: Object = descriptor(&[(BACKEND_WEBGPU_PROPERTY_BUFFER, frame.clone())])?;
    let scene_entry: Object = descriptor(&[
        (
            BACKEND_WEBGPU_PROPERTY_BINDING,
            JsValue::from_f64(BACKEND_WEBGPU_BINDING_SCENE as f64),
        ),
        (BACKEND_WEBGPU_PROPERTY_RESOURCE, scene_resource.into()),
    ])?;
    let frame_entry: Object = descriptor(&[
        (
            BACKEND_WEBGPU_PROPERTY_BINDING,
            JsValue::from_f64(BACKEND_WEBGPU_BINDING_FRAME as f64),
        ),
        (BACKEND_WEBGPU_PROPERTY_RESOURCE, frame_resource.into()),
    ])?;
    let group_descriptor: Object = descriptor(&[
        (
            BACKEND_WEBGPU_PROPERTY_ENTRIES,
            Array::of2(&scene_entry, &frame_entry).into(),
        ),
        (BACKEND_WEBGPU_PROPERTY_LAYOUT, layout.clone()),
    ])?;
    call1(
        device,
        BACKEND_WEBGPU_METHOD_CREATE_BIND_GROUP,
        group_descriptor.as_ref(),
    )
}

/// Builds the single colour attachment a fullscreen draw renders
/// into, cleared to opaque black.
///
/// # Arguments
///
/// - `&JsValue` - The swap-chain texture's view.
///
/// # Returns
///
/// - `Result<Object, JsValue>` - The attachment descriptor.
pub(crate) fn clear_attachment(view: &JsValue) -> Result<Object, JsValue> {
    let channel: JsValue = JsValue::from_f64(0.0);
    let opaque: JsValue = JsValue::from_f64(1.0);
    let red: JsValue = channel.clone();
    let green: JsValue = channel.clone();
    let blue: JsValue = channel;
    let clear: JsValue = Array::of4(&red, &green, &blue, &opaque).into();
    descriptor(&[
        (BACKEND_WEBGPU_PROPERTY_VIEW, view.clone()),
        (BACKEND_WEBGPU_PROPERTY_CLEAR_VALUE, clear),
        (
            BACKEND_WEBGPU_PROPERTY_LOAD_OP,
            JsValue::from_str(BACKEND_WEBGPU_LOAD_CLEAR),
        ),
        (
            BACKEND_WEBGPU_PROPERTY_STORE_OP,
            JsValue::from_str(BACKEND_WEBGPU_STORE_STORE),
        ),
    ])
}
