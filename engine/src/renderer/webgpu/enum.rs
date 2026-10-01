use super::*;

/// Errors that can occur while asynchronously initializing a `WebGpuRenderer`.
///
/// Each variant maps to one specific failure mode that the WebGPU init
/// pipeline can encounter when calling into the browser's GPU API. The
/// underlying JS error (when available) is carried as a `JsValue` so callers
/// can surface the exact diagnostic string without losing fidelity.
///
/// Instead of logging diagnostics inside the engine, `WebGpuRenderer::init`
/// returns `Result<WebGpuRenderer, WebGpuInitError>` and lets the caller
/// decide how to react — typically via `Console::error` on the example side
/// or by falling back to the Canvas 2D backend.
#[derive(Clone, Debug)]
pub enum WebGpuInitError {
    /// `Reflect::get(navigator, "webgpu")` threw an exception.
    ///
    /// Surfaced when the JavaScript binding lookup itself fails rather than
    /// simply returning `undefined`/`null`. Carries the original JS error.
    NavigatorLookup(JsValue),
    /// `navigator.gpu` is `undefined` or `null`.
    ///
    /// The browser does not expose WebGPU on the current origin. The most
    /// common causes are serving over an insecure origin (must be HTTPS or
    /// `localhost`) or running in a browser that lacks the WebGPU feature.
    NavigatorGpuMissing,
    /// `Reflect::get(gpu, "requestAdapter")` threw an exception.
    ///
    /// Carries the original JS error returned by the reflect call.
    RequestAdapterLookup(JsValue),
    /// `gpu.requestAdapter()` threw an exception synchronously.
    ///
    /// Carries the thrown JS error or value.
    RequestAdapterCall(JsValue),
    /// The adapter promise rejected, or the `INIT_PROMISE_TIMEOUT_MILLIS`
    /// race timer fired before the adapter was produced.
    ///
    /// Carries the rejection value, which may be a string, an error object,
    /// or `undefined` when the timeout won the race.
    AdapterPromise(JsValue),
    /// `requestAdapter()` resolved to `null` or `undefined`.
    ///
    /// No compatible GPU adapter exists for the requested `powerPreference`.
    AdapterUnavailable,
    /// `Reflect::get(adapter, "requestDevice")` threw an exception.
    RequestDeviceLookup(JsValue),
    /// `adapter.requestDevice()` threw an exception synchronously.
    RequestDeviceCall(JsValue),
    /// The device promise rejected, or the `INIT_PROMISE_TIMEOUT_MILLIS`
    /// race timer fired before the device was produced.
    DevicePromise(JsValue),
    /// `requestDevice()` resolved to `null` or `undefined`.
    ///
    /// The adapter could not allocate a device, typically because the
    /// adapter is in a `device-lost` state.
    DeviceUnavailable,
    /// `document.querySelector(canvas_selector)` returned `None`.
    ///
    /// The canvas element is not in the DOM yet (or its selector is wrong).
    /// Carries the selector string that was queried.
    CanvasNotFound(String),
    /// `document.querySelector(canvas_selector)` threw an exception.
    CanvasQuery(JsValue),
    /// `canvas.get_context("webgpu")` returned `None`.
    ///
    /// The canvas is already using a different context type, or WebGPU is
    /// disabled for this canvas.
    CanvasContextUnavailable,
    /// `Reflect::get(gpu, "getPreferredCanvasFormat")` threw an exception.
    PreferredFormatLookup(JsValue),
    /// `gpu.getPreferredCanvasFormat()` threw an exception synchronously.
    PreferredFormatCall(JsValue),
    /// `getPreferredCanvasFormat()` resolved to a value that is not a string.
    ///
    /// Carries the offending JS value so callers can log its type/name.
    PreferredFormatType(JsValue),
    /// `Reflect::get(context, "configure")` threw an exception.
    ConfigureLookup(JsValue),
    /// `Reflect::get(device, "queue")` threw an exception.
    QueueLookup(JsValue),
}

/// WebGPU receiver-class discriminator for the `cached_method` cache.
///
/// JS class methods live on the prototype, so the same `Function` instance
/// is returned for every receiver of a given class — the `(class, method)`
/// pair uniquely identifies the cached `Function` and no receiver identity
/// is needed. This replaces the previous stack-address keying: per-frame
/// temporary `JsValue`s (pass encoders, command encoders) reuse stack
/// slots across frames, so an address-keyed cache could return the wrong
/// class's `Function` when a `GPUComputePassEncoder` landed on a slot that
/// previously held a `GPURenderPassEncoder` (both share `setPipeline` /
/// `setBindGroup` / `end`), producing a swallowed TypeError and a silently
/// skipped GPU call.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GpuReceiverClass {
    /// `GPUDevice` (immortal, renderer-owned).
    Device,
    /// `GPUQueue` (immortal, renderer-owned).
    Queue,
    /// `GPUCanvasContext` (immortal, renderer-owned).
    Context,
    /// `GPUTexture` (per-call temporary).
    Texture,
    /// `GPUCommandEncoder` (per-frame temporary).
    CommandEncoder,
    /// `GPURenderPassEncoder` (per-pass temporary).
    RenderPass,
    /// `GPUComputePassEncoder` (per-pass temporary).
    ComputePass,
}
