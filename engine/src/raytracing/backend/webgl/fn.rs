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
pub(crate) fn describe_webgl(reason: &JsValue) -> String {
    match reason.as_string() {
        Some(text) if !text.is_empty() => text,
        _ => format!("{reason:?}"),
    }
}

/// The human-readable stage name used in WebGL diagnostics.
///
/// # Arguments
///
/// - `bool` - Whether this is the vertex stage.
///
/// # Returns
///
/// - `&'static str` - The stage name.
fn stage_name(vertex: bool) -> &'static str {
    if vertex {
        BACKEND_WEBGL_STAGE_VERTEX
    } else {
        BACKEND_WEBGL_STAGE_FRAGMENT
    }
}

/// Compiles one GLSL stage and returns the driver's own failure log.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to compile against.
/// - `bool` - Whether this is the vertex stage.
/// - `&str` - The GLSL source.
///
/// # Returns
///
/// - `Result<WebGlShader, JsValue>` - The compiled shader, or the
///   compile log.
pub(crate) fn compile_stage(
    context: &WebGl2RenderingContext,
    vertex: bool,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    let stage: u32 = if vertex {
        WebGl2RenderingContext::VERTEX_SHADER
    } else {
        WebGl2RenderingContext::FRAGMENT_SHADER
    };
    let name: &str = stage_name(vertex);
    let Some(shader) = context.create_shader(stage) else {
        return Err(backend_error(
            ERROR_WEBGL_PROGRAM,
            format!("createShader returned null for the {name} stage"),
        ));
    };
    context.shader_source(&shader, source);
    context.compile_shader(&shader);
    let compiled: bool = context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);
    if compiled {
        return Ok(shader);
    }
    let log: String = context.get_shader_info_log(&shader).unwrap_or_default();
    context.delete_shader(Some(&shader));
    Err(backend_error(
        ERROR_WEBGL_PROGRAM,
        format!("the {name} stage failed to compile: {log}"),
    ))
}

/// Compiles and links the ray-trace program.
///
/// Both stages are compiled before the link and the link log is returned
/// verbatim on failure, so a shader error is never reported as a
/// successful frame. Every object created on the way — the first shader
/// when the second fails to compile, both when the program is refused,
/// and the program itself when the link fails — is deleted before the
/// error returns, so a failed attach leaves nothing allocated.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to build the program in.
///
/// # Returns
///
/// - `Result<WebGlProgram, JsValue>` - The linked program, or the
///   compile and link diagnostics.
pub(crate) fn link_raytrace_program(
    context: &WebGl2RenderingContext,
) -> Result<WebGlProgram, JsValue> {
    let vertex: WebGlShader = compile_stage(context, true, raytrace_glsl_vertex())?;
    let fragment: WebGlShader = match compile_stage(context, false, raytrace_glsl_fragment()) {
        Ok(value) => value,
        Err(reason) => {
            context.delete_shader(Some(&vertex));
            return Err(reason);
        }
    };
    let Some(program) = context.create_program() else {
        context.delete_shader(Some(&vertex));
        context.delete_shader(Some(&fragment));
        return Err(backend_error(
            ERROR_WEBGL_PROGRAM,
            ERROR_WEBGL_PROGRAM_NULL.to_string(),
        ));
    };
    context.attach_shader(&program, &vertex);
    context.attach_shader(&program, &fragment);
    context.link_program(&program);
    let linked: bool = context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);
    context.detach_shader(&program, &vertex);
    context.detach_shader(&program, &fragment);
    context.delete_shader(Some(&vertex));
    context.delete_shader(Some(&fragment));
    if linked {
        return Ok(program);
    }
    let log: String = context.get_program_info_log(&program).unwrap_or_default();
    context.delete_program(Some(&program));
    Err(backend_error(
        ERROR_WEBGL_PROGRAM,
        format!("the ray-trace program failed to link: {log}"),
    ))
}

/// Resolves one sampler uniform, failing when the compiler dropped it.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context holding the program.
/// - `&WebGlProgram` - The linked program.
/// - `&str` - The uniform name.
///
/// # Returns
///
/// - `Result<WebGlUniformLocation, JsValue>` - The location, or the
///   reason the uniform is missing.
pub(crate) fn resolve_sampler(
    context: &WebGl2RenderingContext,
    program: &WebGlProgram,
    name: &str,
) -> Result<WebGlUniformLocation, JsValue> {
    match context.get_uniform_location(program, name) {
        Some(location) => Ok(location),
        None => Err(backend_error(
            ERROR_WEBGL_PROGRAM,
            format!("the uniform {name} is missing from the linked program"),
        )),
    }
}

/// Names one `glGetError` code for a diagnostic.
///
/// # Arguments
///
/// - `u32` - The code `getError` reported.
///
/// # Returns
///
/// - `&'static str` - The GL name of the code.
fn gl_error_name(code: u32) -> &'static str {
    match code {
        WebGl2RenderingContext::INVALID_ENUM => WEBGL_ERROR_NAME_INVALID_ENUM,
        WebGl2RenderingContext::INVALID_VALUE => WEBGL_ERROR_NAME_INVALID_VALUE,
        WebGl2RenderingContext::INVALID_OPERATION => WEBGL_ERROR_NAME_INVALID_OPERATION,
        WebGl2RenderingContext::OUT_OF_MEMORY => WEBGL_ERROR_NAME_OUT_OF_MEMORY,
        _ => WEBGL_ERROR_NAME_UNKNOWN,
    }
}

/// Reads and clears every pending `glGetError`.
///
/// WebGL reports a rejected call through the error queue rather than by
/// throwing, so a `Result`-only check reports success for a call the
/// driver refused. Draining the queue here is what makes a subsequent
/// diagnostic attribute the error to the call that caused it instead of
/// to some later one.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to drain.
///
/// # Returns
///
/// - `String` - The distinct error names found, empty when the queue was
///   already clear.
fn drain_gl_error(context: &WebGl2RenderingContext) -> String {
    let mut names: Vec<&'static str> = Vec::new();
    while names.len() < WEBGL_ERROR_DRAIN_LIMIT {
        let code: u32 = context.get_error();
        if code == WEBGL_ERROR_NO_ERROR {
            break;
        }
        let name: &'static str = gl_error_name(code);
        if !names.contains(&name) {
            names.push(name);
        }
    }
    names.join(", ")
}

/// Fails when the context has a pending GL error.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to inspect.
/// - `&'static str` - The stable error code the failure is reported
///   under.
/// - `&str` - The call the diagnostic names as the source.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` when the queue is clear.
pub(crate) fn require_clear_gl_error(
    context: &WebGl2RenderingContext,
    code: &'static str,
    call: &str,
) -> Result<(), JsValue> {
    let names: String = drain_gl_error(context);
    if names.is_empty() {
        return Ok(());
    }
    Err(backend_error(code, format!("{call} left {names} pending")))
}

/// The number of `vec4` slots a packed float block occupies.
///
/// # Arguments
///
/// - `usize` - The packed float count.
///
/// # Returns
///
/// - `usize` - The slot count, rounded up so a block that does not fill
///   its final `vec4` still writes whole texels.
pub(crate) fn packed_slot_count(floats: usize) -> usize {
    floats.div_ceil(WEBGL_PACKED_TEXELS_PER_VEC4)
}

/// The texel row width a packed block of `floats` floats is laid out in.
///
/// A square-ish shape is chosen so a block grows in both dimensions
/// together and the shader's `slot / width` addressing keeps a short
/// row. The width is the square root of the slot count rounded up, so
/// the covered area stays within a factor of two of the block itself: a
/// block of `n` slots occupies `ceil(sqrt(n))` texels across by at most
/// `ceil(sqrt(n))` rows, and a block twice as large is the square root
/// of two times as wide and as tall, not twice.
///
/// The root is rounded up, never down, because a floor root makes the
/// final row overflow the edge. At a floor width of `floor(sqrt(n))` the
/// row count is `ceil(n / width)`, which runs past the limit well
/// before the block itself does: 1599 slots at a limit of 40 take a
/// floor width of 39 and 41 rows, and `texStorage2D` refuses an edge
/// past the limit even though the whole block fits under it. At an
/// up-rounded width the row count never exceeds the width, so every
/// block `validate_block_capacity` accepted is shaped within the limit
/// without either edge being shortened.
///
/// No edge is clamped to hide a block that does not fit: the capacity
/// check runs first and rejects anything whose edges would not fit a
/// positive `i32`, so every returned shape holds every slot.
///
/// # Arguments
///
/// - `usize` - The packed float count.
/// - `u32` - The largest texture edge this context accepts.
///
/// # Returns
///
/// - `Result<(i32, i32), JsValue>` - The row width and row count, each
///   at least one texel and holding every slot, or the reason this
///   context cannot lay the block out.
pub(crate) fn packed_shape(floats: usize, limit: u32) -> Result<(i32, i32), JsValue> {
    validate_block_capacity(floats, limit)?;
    let slots: usize = packed_slot_count(floats).max(1);
    let root: usize = slots.isqrt();
    let width: usize = root + usize::from(root.checked_mul(root) != Some(slots));
    let height: usize = slots.div_ceil(width);
    Ok((width as i32, height as i32))
}

/// Whether a packed block can be laid out at all under a texture limit.
///
/// A block whose slot count needs an edge past the limit cannot be
/// shaped into a legal texture, so it is rejected rather than truncated:
/// the shader would read past the end of a shortened upload. The limit
/// is held to the largest edge this `i32` pair can carry, so a context
/// reporting an absurd `MAX_TEXTURE_SIZE` is rejected for every block
/// it cannot actually represent instead of yielding a negative edge.
///
/// # Arguments
///
/// - `usize` - The packed float count.
/// - `u32` - The largest texture edge this context accepts.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` when some shape holds the block.
pub(crate) fn validate_block_capacity(floats: usize, limit: u32) -> Result<(), JsValue> {
    let slots: usize = packed_slot_count(floats).max(1);
    let ceiling: usize = (limit as usize).max(1).min(i32::MAX as usize);
    if slots > ceiling.saturating_mul(ceiling) {
        return Err(backend_error(
            ERROR_WEBGL_BLOCK_TOO_LARGE,
            format!("a block of {slots} vec4 slots needs a texture above {ceiling}x{ceiling}"),
        ));
    }
    Ok(())
}

/// Allocates one `RGBA32F` data texture of an exact size.
///
/// The texture is only ever sampled through `texelFetch`, so both
/// filters are `NEAREST` and the wrap mode is `CLAMP_TO_EDGE`: no
/// float-filtering extension is required and no sampler can read across
/// the edge of the block. `texStorage2D` gives the texture its immutable
/// size once, so a sub-image write never re-specifies a shape.
///
/// The pending error queue is drained before the allocation and checked
/// after it, so a refused allocation that only sets `OUT_OF_MEMORY` —
/// which is how a browser reports an over-budget `texStorage2D`, with
/// no throw — is an error rather than a silently unusable texture.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to allocate in.
/// - `u32` - The texture unit this texture will be bound to.
/// - `i32` - The texture width in texels.
/// - `i32` - The texture height in texels.
///
/// # Returns
///
/// - `Result<WebGlRayTraceTexture, JsValue>` - The allocated texture.
pub(crate) fn create_packed_texture(
    context: &WebGl2RenderingContext,
    unit: u32,
    width: i32,
    height: i32,
) -> Result<WebGlRayTraceTexture, JsValue> {
    require_clear_gl_error(context, ERROR_WEBGL_TEXTURE, WEBGL_ERROR_SOURCE_PENDING)?;
    let Some(texture) = context.create_texture() else {
        return Err(backend_error(
            ERROR_WEBGL_TEXTURE,
            ERROR_WEBGL_TEXTURE_NULL.to_string(),
        ));
    };
    context.active_texture(unit);
    context.bind_texture(WEBGL_TEXTURE_TARGET_2D, Some(&texture));
    context.tex_parameteri(
        WEBGL_TEXTURE_TARGET_2D,
        WEBGL_TEXTURE_MIN_FILTER,
        WEBGL_NEAREST,
    );
    context.tex_parameteri(
        WEBGL_TEXTURE_TARGET_2D,
        WEBGL_TEXTURE_MAG_FILTER,
        WEBGL_NEAREST,
    );
    context.tex_parameteri(
        WEBGL_TEXTURE_TARGET_2D,
        WEBGL_TEXTURE_WRAP_S,
        WEBGL_CLAMP_TO_EDGE,
    );
    context.tex_parameteri(
        WEBGL_TEXTURE_TARGET_2D,
        WEBGL_TEXTURE_WRAP_T,
        WEBGL_CLAMP_TO_EDGE,
    );
    context.tex_storage_2d(
        WEBGL_TEXTURE_TARGET_2D,
        WEBGL_MIP_LEVELS_ONE,
        WEBGL_FORMAT_RGBA32F as u32,
        width,
        height,
    );
    let outcome: Result<(), JsValue> =
        require_clear_gl_error(context, ERROR_WEBGL_TEXTURE, "texStorage2D");
    match outcome {
        Ok(()) => {}
        Err(reason) => {
            context.delete_texture(Some(&texture));
            return Err(reason);
        }
    }
    Ok(WebGlRayTraceTexture {
        texture,
        width,
        height,
    })
}

/// Binds one data texture to a unit.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to bind in.
/// - `u32` - The texture unit.
/// - `&WebGlTexture` - The texture to bind.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` once the texture is bound.
pub(crate) fn bind_data_texture(
    context: &WebGl2RenderingContext,
    unit: u32,
    texture: &WebGlTexture,
) -> Result<(), JsValue> {
    context.active_texture(unit);
    context.bind_texture(WEBGL_TEXTURE_TARGET_2D, Some(texture));
    require_clear_gl_error(context, ERROR_WEBGL_CALL, "bindTexture")
}

/// Refills a packed data texture through `texSubImage2D`.
///
/// The upload length is rounded up to whole texels so a packed block
/// that does not fill its final `vec4` still writes whole texels; the
/// staging buffer is zero-extended and every float beyond the block is
/// reset to zero, so a block that shrinks cannot leave a previous
/// frame's values readable past its end. The block itself is copied
/// into the staging buffer: uploading the caller's slice directly would
/// write only as many bytes as it holds, which leaves the final partial
/// texel unwritten and makes the shader read uninitialised memory.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to upload through.
/// - `&WebGlRayTraceTexture` - The destination texture.
/// - `u32` - The texture unit the texture is bound to.
/// - `&[f32]` - The packed block.
/// - `&mut Vec<f32>` - The scratch buffer the upload is built in, reused
///   across frames.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` once the upload was accepted.
pub(crate) fn upload_packed(
    context: &WebGl2RenderingContext,
    target: &WebGlRayTraceTexture,
    unit: u32,
    packed: &[f32],
    scratch: &mut Vec<f32>,
) -> Result<(), JsValue> {
    let slots: usize = packed_slot_count(packed.len());
    if slots == 0 {
        return Err(backend_error(
            ERROR_WEBGL_EMPTY_BLOCK,
            format!("{} floats pack into no vec4 slot", packed.len()),
        ));
    }
    let rows: i32 = slots.div_ceil(target.get_width() as usize) as i32;
    let needed: usize =
        (rows as usize) * (target.get_width() as usize) * WEBGL_PACKED_TEXELS_PER_VEC4;
    if scratch.len() < needed {
        scratch.resize(needed, 0.0);
    }
    scratch[needed..].fill(0.0);
    for (index, value) in packed.iter().enumerate() {
        scratch[index] = *value;
    }
    let upload: &[f32] = &scratch[..needed];
    let view: Float32Array = Float32Array::from(upload);
    require_clear_gl_error(context, ERROR_WEBGL_UPLOAD, WEBGL_ERROR_SOURCE_PENDING)?;
    bind_data_texture(context, unit, target.get_texture())?;
    let outcome: Result<(), JsValue> = context
        .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
            WEBGL_TEXTURE_TARGET_2D,
            WEBGL_MIP_LEVEL_ZERO,
            0,
            0,
            target.get_width(),
            rows,
            WEBGL_PIXEL_FORMAT_RGBA,
            WEBGL_TYPE_FLOAT,
            Some(view.as_ref() as &Object),
        );
    match outcome {
        Ok(()) => require_clear_gl_error(context, ERROR_WEBGL_UPLOAD, "texSubImage2D"),
        Err(reason) => Err(backend_error(ERROR_WEBGL_UPLOAD, describe_webgl(&reason))),
    }
}

/// Grows the scene texture in place when its block no longer fits.
///
/// A block that already fits the current texture reuses it and touches
/// nothing, so a repeated update at a fixed size issues one
/// `texSubImage2D` and creates no GL object. A block that does not fit
/// is allocated once at its own shape: the new texture is bound and
/// fully written before it is adopted, so a failed allocation or a
/// failed write leaves the previous, still valid, texture in place. The
/// texture being replaced is deleted only after the swap succeeds, so a
/// rejected block never leaves the backend without a usable texture.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to allocate in.
/// - `&mut WebGlRayTraceTexture` - The current texture, replaced only
///   on success.
/// - `&mut Vec<f32>` - The scratch buffer the upload is built in.
/// - `&[f32]` - The packed scene block.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` once the block is live on the GPU.
pub(crate) fn ensure_scene_texture(
    context: &WebGl2RenderingContext,
    current: &mut WebGlRayTraceTexture,
    scratch: &mut Vec<f32>,
    packed: &[f32],
) -> Result<(), JsValue> {
    grow_texture(context, current, WEBGL_TEXTURE_UNIT_SCENE, scratch, packed)
}

/// Grows the frame texture in place when its block no longer fits.
///
/// The mirror of [`ensure_scene_texture`] for the block every frame
/// uploads, so a steady frame reuses one allocation and a larger block
/// reallocates exactly once.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to allocate in.
/// - `&mut WebGlRayTraceTexture` - The current texture, replaced only
///   on success.
/// - `&mut Vec<f32>` - The scratch buffer the upload is built in.
/// - `&[f32]` - The packed frame block.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` once the block is live on the GPU.
pub(crate) fn ensure_frame_texture(
    context: &WebGl2RenderingContext,
    current: &mut WebGlRayTraceTexture,
    scratch: &mut Vec<f32>,
    packed: &[f32],
) -> Result<(), JsValue> {
    grow_texture(context, current, WEBGL_TEXTURE_UNIT_FRAME, scratch, packed)
}

/// Reallocates and re-uploads one data texture when its block outgrows
/// it, mutating it in place only after the new texture is live.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to allocate in.
/// - `&mut WebGlRayTraceTexture` - The texture to grow in place.
/// - `u32` - The texture unit the texture is bound to.
/// - `&mut Vec<f32>` - The scratch buffer the upload is built in.
/// - `&[f32]` - The packed block.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` once the block is live on the GPU.
fn grow_texture(
    context: &WebGl2RenderingContext,
    current: &mut WebGlRayTraceTexture,
    unit: u32,
    scratch: &mut Vec<f32>,
    packed: &[f32],
) -> Result<(), JsValue> {
    let slots: usize = packed_slot_count(packed.len());
    if slots == 0 {
        return Err(backend_error(
            ERROR_WEBGL_EMPTY_BLOCK,
            format!("{} floats pack into no vec4 slot", packed.len()),
        ));
    }
    let held: usize = (current.get_width() as usize) * (current.get_height() as usize);
    if slots > held {
        let limit: u32 = texture_size_limit(context);
        let (width, height): (i32, i32) = packed_shape(packed.len(), limit)?;
        let grown: WebGlRayTraceTexture = create_packed_texture(context, unit, width, height)?;
        match upload_packed(context, &grown, unit, packed, scratch) {
            Ok(()) => {}
            Err(reason) => {
                context.delete_texture(Some(grown.get_texture()));
                return Err(reason);
            }
        }
        context.delete_texture(Some(current.get_texture()));
        current.set_texture(grown.get_texture().clone());
        current.set_width(grown.get_width());
        current.set_height(grown.get_height());
        return Ok(());
    }
    upload_packed(context, current, unit, packed, scratch)
}

/// Reads the `MAX_TEXTURE_SIZE` of a context.
///
/// The query goes through `getParameter` rather than the constant on the
/// binding, because the constant is the interface's minimum while the
/// value the driver actually enforces is what an allocation is checked
/// against.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to query.
///
/// # Returns
///
/// - `u32` - The reported edge, or the guaranteed minimum when the
///   query returns nothing usable.
pub(crate) fn texture_size_limit(context: &WebGl2RenderingContext) -> u32 {
    let queried: Result<JsValue, JsValue> =
        context.get_parameter(WebGl2RenderingContext::MAX_TEXTURE_SIZE);
    match queried {
        Ok(value) => match value.as_f64() {
            Some(number) if number >= 1.0 && number.is_finite() => number as u32,
            _ => WEBGL_TEXTURE_SIZE_LIMIT_FALLBACK,
        },
        Err(_) => WEBGL_TEXTURE_SIZE_LIMIT_FALLBACK,
    }
}

/// Reads the per-axis `MAX_VIEWPORT_DIMS` of a context.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to query.
///
/// # Returns
///
/// - `u32` - The smaller reported axis, or the guaranteed bound when
///   the query returns nothing usable.
pub(crate) fn viewport_dimension_limit(context: &WebGl2RenderingContext) -> u32 {
    let queried: Result<JsValue, JsValue> =
        context.get_parameter(WebGl2RenderingContext::MAX_VIEWPORT_DIMS);
    match queried {
        Ok(value) => match Int32Array::new(&value).to_vec() {
            axes if axes.len() == WEBGL_VIEWPORT_AXIS_COUNT => {
                let first: i32 = axes[0];
                let second: i32 = axes[1];
                if first <= 0 || second <= 0 {
                    WEBGL_VIEWPORT_DIMENSION_FALLBACK
                } else {
                    (first.min(second) as u32).min(WEBGL_VIEWPORT_DIMENSION_CEILING)
                }
            }
            _ => WEBGL_VIEWPORT_DIMENSION_FALLBACK,
        },
        Err(_) => WEBGL_VIEWPORT_DIMENSION_FALLBACK,
    }
}

/// Rejects a surface whose dimensions exceed this context's limits.
///
/// The check is per axis and never clamps: a caller that asks for a
/// surface the driver would refuse gets an error and keeps the previous,
/// still valid, size.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to inspect for a lost
///   state.
/// - `u32` - The requested width in pixels.
/// - `u32` - The requested height in pixels.
/// - `u32` - The largest surface dimension this context accepts.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` when the size is renderable.
pub(crate) fn validate_webgl_surface(
    context: &WebGl2RenderingContext,
    width: u32,
    height: u32,
    limit: u32,
) -> Result<(), JsValue> {
    if context.is_context_lost() {
        return Err(backend_error(
            ERROR_WEBGL_CONTEXT_LOST,
            ERROR_WEBGL_CONTEXT_LOST.to_string(),
        ));
    }
    if width > limit || height > limit {
        return Err(backend_error(
            ERROR_WEBGL_SURFACE_LIMIT,
            format!("requested {width}x{height}, over the {limit} pixel per-axis limit"),
        ));
    }
    Ok(())
}

/// Applies a full-surface viewport derived from the drawing buffer.
///
/// The canvas width and height may be reset by the browser — setting
/// either one resets the viewport to the new default — so the viewport
/// is re-applied from `drawingBufferWidth` and `drawingBufferHeight`
/// before every draw rather than once per resize. That is also what
/// keeps a device-pixel-ratio change from drawing into a corner.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context to set the viewport in.
///
/// # Returns
///
/// - `Result<(), JsValue>` - `Ok` once the viewport covers the whole
///   drawing buffer.
pub(crate) fn apply_full_viewport(context: &WebGl2RenderingContext) -> Result<(), JsValue> {
    let width: i32 = context.drawing_buffer_width();
    let height: i32 = context.drawing_buffer_height();
    if width <= 0 || height <= 0 {
        return Err(backend_error(
            ERROR_WEBGL_EMPTY_DRAWING_BUFFER,
            format!("the drawing buffer is {width}x{height}"),
        ));
    }
    require_clear_gl_error(context, ERROR_WEBGL_CALL, WEBGL_ERROR_SOURCE_PENDING)?;
    context.viewport(0, 0, width, height);
    require_clear_gl_error(context, ERROR_WEBGL_CALL, "viewport")
}

/// Acquires the WebGL 2 context of a canvas and reports why it failed.
///
/// # Arguments
///
/// - `&HtmlCanvasElement` - The canvas to draw into.
///
/// # Returns
///
/// - `Result<WebGl2RenderingContext, JsValue>` - The context, or the
///   reason the browser refused one.
pub(crate) fn attach_context(
    canvas: &HtmlCanvasElement,
) -> Result<WebGl2RenderingContext, JsValue> {
    let lookup: Result<Option<Object>, JsValue> = canvas.get_context(WEBGL_CONTEXT_ID_2D);
    let context_object: Object = match lookup {
        Ok(Some(object)) => object,
        Ok(None) => {
            return Err(backend_error(
                ERROR_WEBGL_UNAVAILABLE,
                ERROR_WEBGL_UNAVAILABLE.to_string(),
            ));
        }
        Err(reason) => {
            return Err(backend_error(
                ERROR_WEBGL_UNAVAILABLE,
                describe_webgl(&reason),
            ));
        }
    };
    match context_object.dyn_into() {
        Ok(context) => Ok(context),
        Err(_) => Err(backend_error(
            ERROR_WEBGL_UNAVAILABLE,
            ERROR_WEBGL_UNAVAILABLE.to_string(),
        )),
    }
}

/// Releases every GL object a half-built attach owns.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - The context the objects belong to.
/// - `Option<&WebGlProgram>` - The program, when one was linked.
/// - `Option<&WebGlVertexArrayObject>` - The vertex array, when one was
///   created.
/// - `Option<&WebGlRayTraceTexture>` - The scene texture, when one was
///   allocated.
/// - `Option<&WebGlRayTraceTexture>` - The frame texture, when one was
///   allocated.
pub(crate) fn release_partial(
    context: &WebGl2RenderingContext,
    program: Option<&WebGlProgram>,
    vao: Option<&WebGlVertexArrayObject>,
    scene_texture: Option<&WebGlRayTraceTexture>,
    frame_texture: Option<&WebGlRayTraceTexture>,
) {
    if let Some(texture) = frame_texture {
        context.delete_texture(Some(texture.get_texture()));
    }
    if let Some(texture) = scene_texture {
        context.delete_texture(Some(texture.get_texture()));
    }
    if let Some(array) = vao {
        context.delete_vertex_array(Some(array));
    }
    if let Some(linked) = program {
        context.delete_program(Some(linked));
    }
}
