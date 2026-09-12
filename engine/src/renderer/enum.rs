use super::*;

/// Defines how new pixels are composited with existing pixels on the canvas.
///
/// Maps directly to the CSS `globalCompositeOperation` property.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BlendMode {
    /// The source is drawn over the destination (default alpha blending).
    #[default]
    Normal,
    /// The source color is multiplied with the destination, producing a darker result.
    Multiply,
    /// The source and destination are inverted, multiplied, then inverted again.
    Screen,
    /// The source and destination colors are added together, clamped to maximum brightness.
    Lighter,
    /// Combines `Multiply` and `Screen` based on the destination color.
    Overlay,
    /// Keeps the darker of the source and destination per channel.
    Darken,
    /// Keeps the lighter of the source and destination per channel.
    Lighten,
    /// Dodges the destination color brightening it based on the source.
    ColorDodge,
    /// Burns the destination color darkening it based on the source.
    ColorBurn,
    /// A harsher version of `Overlay` using the source color as the filter.
    HardLight,
    /// A softer version of `Overlay` using the source color as the filter.
    SoftLight,
    /// Subtracts the darker color from the lighter color per channel.
    Difference,
    /// Similar to `Difference` but with lower contrast.
    Exclusion,
    /// Uses the hue of the source with the saturation and luminosity of the destination.
    Hue,
    /// Uses the saturation of the source with the hue and luminosity of the destination.
    Saturation,
    /// Uses the hue and saturation of the source with the luminosity of the destination.
    Color,
    /// Uses the luminosity of the source with the hue and saturation of the destination.
    Luminosity,
}

/// A single deferred draw operation recorded into a `DrawList`.
///
/// Commands carry the resolved style for the operation (fill/stroke color, line
/// width) so that replay can group consecutive same-style shapes into a single
/// path and skip redundant canvas state changes. Colors are stored as `Color`
/// and converted to CSS strings only at replay time.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawCommand {
    /// Fills a rectangle. Carries the fill color.
    FillRect {
        /// The top-left position in world space.
        position: Vector2D,
        /// The width in pixels.
        width: f64,
        /// The height in pixels.
        height: f64,
        /// The fill color.
        color: Color,
    },
    /// Strokes the outline of a rectangle. Carries stroke color and line width.
    StrokeRect {
        /// The top-left position in world space.
        position: Vector2D,
        /// The width in pixels.
        width: f64,
        /// The height in pixels.
        height: f64,
        /// The stroke color.
        color: Color,
        /// The stroke line width in pixels.
        line_width: f64,
    },
    /// Fills a circle. Carries the fill color.
    FillCircle {
        /// The center in world space.
        center: Vector2D,
        /// The radius in pixels.
        radius: f64,
        /// The fill color.
        color: Color,
    },
    /// Strokes the outline of a circle. Carries stroke color and line width.
    StrokeCircle {
        /// The center in world space.
        center: Vector2D,
        /// The radius in pixels.
        radius: f64,
        /// The stroke color.
        color: Color,
        /// The stroke line width in pixels.
        line_width: f64,
    },
    /// Draws a line segment. Carries stroke color and line width.
    Line {
        /// The start point in world space.
        start: Vector2D,
        /// The end point in world space.
        end: Vector2D,
        /// The stroke color.
        color: Color,
        /// The stroke line width in pixels.
        line_width: f64,
    },
    /// Fills text at a position. Carries the fill color and font.
    FillText {
        /// The text to draw.
        text: String,
        /// The position in world space.
        position: Vector2D,
        /// The fill color.
        color: Color,
        /// The CSS font string.
        font: String,
    },
    /// Draws a transformed sprite sub-region (image, source rect, TRS transform).
    DrawSprite {
        /// The image to draw.
        image: HtmlImageElement,
        /// The source rectangle within the image.
        source: Rect,
        /// The world-space transform (position, rotation, scale). Scale signs flip.
        transform: Transform2D,
    },
    /// Draws an image sub-region at a destination rect (no rotation).
    DrawImageRect {
        /// The image to draw.
        image: HtmlImageElement,
        /// The source rectangle within the image.
        source: Rect,
        /// The destination top-left position in world space.
        dest_position: Vector2D,
        /// The destination width in pixels.
        dest_width: f64,
        /// The destination height in pixels.
        dest_height: f64,
    },
    /// Applies a global alpha to all subsequent commands until changed.
    SetGlobalAlpha {
        /// The alpha value in the range 0.0 to 1.0.
        alpha: f64,
    },
    /// Applies a blend mode to all subsequent commands until changed.
    SetBlendMode {
        /// The blend mode to apply.
        mode: BlendMode,
    },
}

/// Rendering quality preset controlling anti-aliasing smoothing strategy.
///
/// Maps to the canvas `imageSmoothingQuality` value plus an explicit
/// `imageSmoothingEnabled` toggle. Combined with a CSS `image-rendering:
/// pixelated` rule on the consumer side, `Low` produces crisp pixel-art
/// rendering while `High` produces smooth vector-style rendering.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum RenderQuality {
    /// Fastest rendering, pixelated scaling.
    ///
    /// Disables `imageSmoothingEnabled` on the canvas context and sets
    /// `imageSmoothingQuality = "low"`. Pair with CSS `image-rendering:
    /// pixelated` for sharp nearest-neighbour scaling.
    Low,
    /// Balanced rendering with default smoothing quality.
    ///
    /// Sets `imageSmoothingQuality = "medium"`.
    Medium,
    /// Highest fidelity rendering with smooth edges and high-quality scaling.
    ///
    /// Sets `imageSmoothingQuality = "high"`. Best for vector-style content
    /// on HiDPI displays. This is the default — when no explicit quality is
    /// requested, the engine errs on the side of visual fidelity rather than
    /// performance, since users typically notice aliasing artifacts before
    /// they notice a few extra milliseconds of GPU time.
    #[default]
    High,
}

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

/// Errors that can occur while initializing a `WebGlRenderer`.
///
/// WebGL context acquisition is synchronous, so the failure modes are far
/// fewer than `WebGpuInitError`: the canvas must resolve and the browser
/// must hand back a `WebGl2RenderingContext`. Each variant maps to one
/// specific failure mode; the caller decides how to surface it (typically
/// via `Console::error` on the example side).
#[derive(Clone, Debug)]
pub enum WebGlInitError {
    /// `document.querySelector(canvas_selector)` returned `None`.
    ///
    /// The canvas element is not in the DOM yet (or its selector is wrong).
    /// Carries the selector string that was queried.
    CanvasNotFound(String),
    /// `document.querySelector(canvas_selector)` threw an exception.
    CanvasQuery(JsValue),
    /// `canvas.get_context("webgl2")` returned `None`.
    ///
    /// The browser does not support WebGL 2, or the canvas is already bound
    /// to a different context type.
    ContextUnavailable,
    /// `canvas.get_context("webgl2")` threw an exception.
    ContextLookup(JsValue),
    /// The object returned by `canvas.get_context("webgl2")` could not be
    /// cast to `WebGl2RenderingContext`.
    ContextCast,
}

/// Errors that can occur while building a WebGL shader program.
///
/// Each variant carries the browser-provided info log so the caller can
/// surface the exact GLSL diagnostic without losing fidelity.
#[derive(Clone, Debug)]
pub enum WebGlProgramError {
    /// Vertex or fragment shader compilation failed.
    ///
    /// Carries the shader info log returned by `getShaderInfoLog`.
    ShaderCompile(String),
    /// Program linking failed (or `createProgram` returned `None`).
    ///
    /// Carries the program info log returned by `getProgramInfoLog`.
    ProgramLink(String),
}

/// Whether a vertex buffer is consumed per-vertex or per-instance.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum VertexStepMode {
    /// Advance the buffer one vertex at a time.
    #[default]
    Vertex,
    /// Advance the buffer one entry at a time, for all vertices of an
    /// instance.
    Instance,
}

/// A single binding entry inside a `BindGroupDescriptor`.
#[derive(Clone, Debug)]
pub enum BindGroupEntry {
    /// A uniform / storage buffer binding.
    ///
    /// In WGSL terms, the buffer's `usage` must include `UNIFORM` for
    /// `var<uniform>` bindings and `STORAGE` for `var<storage>` bindings.
    Buffer {
        /// The binding slot (matches `@binding(N)` in the shader).
        binding: u32,
        /// The `GpuBuffer` handle.
        buffer: JsValue,
        /// The byte offset into the buffer where the binding starts.
        offset: u64,
        /// The size in bytes of the binding. `None` means "until the end
        /// of the buffer".
        size: Option<u64>,
    },
    /// A read-write storage texture binding.
    ///
    /// The `GpuTexture` must have been created with `STORAGE_BINDING`
    /// in its `usage` flag. Combine with `view` (a `GpuTextureView`)
    /// obtained from `GpuTexture.createView()`.
    StorageTexture {
        /// The binding slot.
        binding: u32,
        /// The `GpuTextureView` handle.
        view: JsValue,
        /// `true` for `texture_storage_2d<format, read>` bindings,
        /// `false` for `texture_storage_2d<format, read_write>` bindings.
        read_only: bool,
    },
    /// A sampled texture binding.
    Texture {
        /// The binding slot.
        binding: u32,
        /// The `GpuTextureView` handle.
        view: JsValue,
    },
    /// A sampler binding.
    Sampler {
        /// The binding slot.
        binding: u32,
        /// The `GpuSampler` handle.
        sampler: JsValue,
    },
}

/// The resource kind bound at a single slot of a `BindGroupLayoutEntry`.
#[derive(Clone, Debug)]
pub enum BindGroupEntryType {
    /// `GpuBufferBindingLayout { type: "uniform" }`.
    UniformBuffer,
    /// `GpuBufferBindingLayout { type: "storage" | "read-only-storage" }`.
    StorageBuffer {
        /// `true` → `"read-only-storage"`, `false` → `"storage"`.
        read_only: bool,
    },
    /// `GpuTextureBindingLayout`.
    SampledTexture {
        /// One of `"float"`, `"unfilterable-float"`, `"depth"`, `"sint"`, `"uint"`.
        sample_type: String,
        /// `true` if the bound texture is multisampled (matches MSAA render-target sampling).
        multisampled: bool,
    },
    /// `GpuStorageTextureBindingLayout`.
    StorageTexture {
        /// `true` → `"read-only"`, `false` → `"read-write"`.
        read_only: bool,
        /// Texture format string (e.g. `"rgba8unorm"`, `"r32float"`).
        format: String,
    },
    /// `GpuSamplerBindingLayout`.
    Sampler {
        /// `true` for filtering samplers (linear interpolation).
        filtering: bool,
        /// `true` for comparison samplers (depth-texture sampling).
        comparison: bool,
    },
}
