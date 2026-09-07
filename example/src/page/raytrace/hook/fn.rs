use super::*;

/// Creates the RayTrace Canvas 2D tab reactive state.
///
/// # Returns
///
/// - `UseRayTrace` - The RayTrace Canvas 2D tab state.
pub(crate) fn use_raytrace_state() -> UseRayTrace {
    UseRayTrace {
        fps: App::use_signal(|| 0.0),
        running: App::use_signal(|| true),
        loop_started: App::use_signal(|| false),
        auto_rotate: App::use_signal(|| true),
        render_scale: App::use_signal(|| 1.0),
        // `loaded` is `false` while the SSAA wrapper is being acquired
        // and the first warmup frame is being traced. The view renders
        // a `c_game_loading_overlay` canvas on top of the raytrace
        // canvas for exactly this window so the user sees a centered
        // "Initializing..." line instead of a half-rendered frame. The
        // loop flips it to `true` after the first successful render,
        // matching the WebGL / WebGPU tabs' loading UX.
        loaded: App::use_signal(|| false),
        // `active` mirrors the WebGL / WebGPU tabs: `true` once the
        // SSAA canvas is acquired and frames are being traced. The view
        // does not read it directly today, but exposing it here keeps
        // the per-tab status surface uniform.
        active: App::use_signal(|| false),
    }
}

/// Creates the RayTrace WebGL tab reactive state.
///
/// # Returns
///
/// - `UseRayTraceWebGl` - The WebGL backend state.
pub(crate) fn use_raytrace_webgl_state() -> UseRayTraceWebGl {
    UseRayTraceWebGl {
        fps: App::use_signal(|| 0.0),
        running: App::use_signal(|| true),
        auto_rotate: App::use_signal(|| true),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        init_error_code: App::use_signal(|| ""),
    }
}

/// Creates the RayTrace WebGPU tab reactive state.
///
/// # Returns
///
/// - `UseRayTraceWebGpu` - The WebGPU backend state.
pub(crate) fn use_raytrace_webgpu_state() -> UseRayTraceWebGpu {
    UseRayTraceWebGpu {
        fps: App::use_signal(|| 0.0),
        running: App::use_signal(|| true),
        auto_rotate: App::use_signal(|| true),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        init_error_code: App::use_signal(|| ""),
    }
}

/// Creates the RayTrace page fullscreen overlay state signals.
///
/// Allocates hook slots in this fixed order:
///
/// 1. canvas_2d
/// 2. web_gl
/// 3. web_gpu
///
/// # Returns
///
/// - `UseRayTraceFullscreen` - The RayTrace page fullscreen state.
pub(crate) fn use_raytrace_fullscreen_state() -> UseRayTraceFullscreen {
    UseRayTraceFullscreen {
        canvas_2d: App::use_signal(|| false),
        web_gl: App::use_signal(|| false),
        web_gpu: App::use_signal(|| false),
    }
}

/// Returns `true` when no element matches the canvas selector, meaning the
/// page or tab was navigated away from and the game loop should stop.
///
/// Hook-context cleanups (`App::use_cleanup`) only run on match-arm
/// switches, not on router navigation, so RAF loops additionally guard on
/// canvas presence to avoid simulating and rendering against a detached
/// canvas forever.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the canvas element.
///
/// # Returns
///
/// - `bool` - Whether the canvas is absent from the document.
fn raytrace_canvas_detached(canvas_selector: &str) -> bool {
    window()
        .and_then(|window_value: Window| window_value.document())
        .and_then(|document: Document| document.query_selector(canvas_selector).ok().flatten())
        .is_none()
}

/// Reads the CSS pixel dimensions of a RayTrace canvas element via
/// `getBoundingClientRect`.
///
/// The rect reflects the target CSS size immediately (unlike
/// `clientWidth`/`clientHeight`, which track the backing store in Chrome
/// and would create a feedback loop if read every frame). Page-scoped
/// copy of the game-page helper: the name stays private so it cannot
/// collide with the `game_2d` / `game_3d` `read_canvas_size` re-exports.
///
/// # Arguments
///
/// - `&str` - The CSS selector for the canvas element.
///
/// # Returns
///
/// - `Option<(f64, f64)>` - The (width, height) in CSS pixels.
fn read_raytrace_canvas_size(canvas_selector: &str) -> Option<(f64, f64)> {
    let window_value: Window = window()?;
    let document_value: Document = window_value.document()?;
    let element: Element = document_value
        .query_selector(canvas_selector)
        .ok()
        .flatten()?;
    let canvas: HtmlCanvasElement = element.unchecked_into();
    let rect: DomRect = canvas.get_bounding_client_rect();
    Some((rect.width(), rect.height()))
}

/// Acquires the Canvas 2D context for the RayTrace demo canvas wrapped in
/// a `SsaaCanvas` so all ray-traced geometry is rendered into an
/// offscreen 2x backing store and downscaled onto the display canvas
/// with `imageSmoothingEnabled = true` / `imageSmoothingQuality = "high"`.
///
/// Without SSAA the ray-traced framebuffer is uploaded via
/// `put_image_data` straight to the visible canvas, so the browser
/// composites it at its native (4:3 ladder) resolution into the 3:2
/// inline CSS box. The bilinear upscale on a hard-edged framebuffer
/// produces visible ball-edge and ground-edge aliasing, exactly the
/// "all elements not anti-aliased" report the user filed.
///
/// The 2x SSAA wrapper produces a smoother downscale pass on every
/// frame: the 2D context's high-quality image smoothing averages
/// adjacent framebuffer pixels into the display canvas's backing
/// store, smoothing the otherwise-jagged edges of the ray-traced
/// spheres, the ground AABB outline, and the emissive sphere's
/// terminator.
///
/// # Returns
///
/// - `Option<(HtmlCanvasElement, SsaaCanvas)>` - The display canvas
///   plus the SSAA wrapper, or `None` if the canvas element was not
///   found or a 2D context could not be acquired.
fn acquire_raytrace_ssaa_canvas() -> Option<(HtmlCanvasElement, SsaaCanvas)> {
    let window_value: Window = window()?;
    let is_mobile: bool = window_value
        .inner_width()
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .is_some_and(|width: f64| width < 768.0);
    let scale_factor: f64 = if is_mobile { 1.0 } else { 2.0 };
    let (canvas_width, canvas_height): (f64, f64) =
        read_raytrace_canvas_size(RAYTRACE_CANVAS_SELECTOR)?;
    let ssaa_canvas: SsaaCanvas = SsaaCanvas::from_selector_with_scale(
        RAYTRACE_CANVAS_SELECTOR,
        canvas_width,
        canvas_height,
        scale_factor,
    )?;
    let document_value: Document = window_value.document()?;
    let element: Element = document_value
        .query_selector(RAYTRACE_CANVAS_SELECTOR)
        .ok()
        .flatten()?;
    let display_canvas: HtmlCanvasElement = element.unchecked_into();
    Some((display_canvas, ssaa_canvas))
}

/// Builds the static raytracing scene used by the RayTrace demo.
///
/// Four occluders: a mirror sphere in the centre (Phong specular
/// material drives the reflection), an emissive sphere in the back
/// (acts as the only secondary light source visible to bounced rays),
/// a sun sphere positioned in the direction OPPOSITE to the directional
/// sun at yaw=0 so the user can see the light source as a tangible
/// glowing object on screen, and a ground AABB below the spheres.
/// Returns the occluder list together with the eye position (kept
/// constant so specular highlights stay stable as the camera orbits).
///
/// # Returns
///
/// - `(Vec<Occluder>, Vector3D)` - The static scene occluders and the eye position.
fn build_raytrace_scene() -> (Vec<Occluder>, Vector3D) {
    let ground_min: Vector3D = Vector3D::new(-5.0, -0.6, -5.0);
    let ground_max: Vector3D = Vector3D::new(5.0, -0.5, 5.0);
    let ground_material: Material = Material::phong(Vector3D::new(0.30, 0.32, 0.36), 0.30, 24.0);
    let ground: Occluder = Occluder::aabb(ground_min, ground_max, ground_material);
    let mirror_material: Material = Material::phong(Vector3D::new(0.05, 0.05, 0.06), 1.0, 64.0);
    let mirror: Occluder = Occluder::sphere(Vector3D::new(0.0, 0.4, 0.0), 0.9, mirror_material);
    let emissive_material: Material = Material::emissive(Vector3D::new(1.0, 0.45, 0.10));
    let emissive: Occluder =
        Occluder::sphere(Vector3D::new(1.6, 0.6, -1.4), 0.45, emissive_material);
    // Sun sphere: positioned at the OPPOSITE direction of the
    // directional sun at yaw=0 (`raytrace_sun_direction(0.0)`), 8 units
    // out from origin, so the camera always sees the light source as
    // a tangible object. The position is intentionally static — the
    // direction rotates with yaw, but pinning the sun sphere at the
    // yaw=0 position keeps it in view as the user orbits and prevents
    // the bouncing reflections from losing their anchor.
    let sun_material: Material = Material::emissive(Vector3D::new(1.00, 0.95, 0.85));
    let sun: Occluder = Occluder::sphere(raytrace_sun_direction(0.0) * -8.0, 0.5, sun_material);
    let occluders: Vec<Occluder> = vec![ground, mirror, emissive, sun];
    let eye: Vector3D = Vector3D::new(0.0, 0.8, 3.5);
    (occluders, eye)
}

/// Computes the normalized directional sun direction for the current
/// orbit yaw.
///
/// Shared by the CPU lighting builder and the GPU uniform packer so all
/// three backends shade with the identical sun vector.
///
/// # Arguments
///
/// - `f64` - The current camera yaw in radians.
///
/// # Returns
///
/// - `Vector3D` - The unit sun direction.
fn raytrace_sun_direction(yaw: f64) -> Vector3D {
    Vector3D::new(-yaw.cos(), -0.5, -yaw.sin()).normalized()
}

/// Builds the per-frame lighting uniforms for the raytrace scene.
///
/// The single directional sun rotates with the current yaw so the lit
/// side of the spheres tracks the orbiting camera: as the user drags
/// the camera the highlight smoothly slides off the visible side of
/// the mirror. Ambient and the specular eye stay constant.
///
/// # Arguments
///
/// - `Vector3D` - The eye position used for specular calculations.
/// - `f64` - The current camera yaw in radians.
///
/// # Returns
///
/// - `LightingUniforms` - The per-frame lighting.
fn build_raytrace_lighting(eye: Vector3D, yaw: f64) -> LightingUniforms {
    let sun: Light =
        Light::new_directional(raytrace_sun_direction(yaw), Vector3D::new(1.0, 0.95, 0.85));
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::new(0.10, 0.10, 0.14));
    lights.add_light(sun);
    lights
}

/// Clamps an `0..=infinity` linear color channel into the `0..=1`
/// range used by the sRGB gamma curve.
///
/// # Arguments
///
/// - `f64` - The linear color channel value.
///
/// # Returns
///
/// - `f64` - The clamped value in `[0, 1]`.
fn clamp_unit(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

/// Packs a linear `0..=1` color channel into an sRGB byte, applying the
/// shared `1/2.2` gamma curve.
///
/// # Arguments
///
/// - `f64` - The linear color channel value.
///
/// # Returns
///
/// - `u8` - The gamma-corrected 8-bit channel value.
fn gamma_byte(value: f64) -> u8 {
    (clamp_unit(value).powf(1.0 / 2.2) * 255.0).round() as u8
}

/// Computes the camera basis (forward, right, up_true) for a given
/// yaw / pitch orbit position.
///
/// Mirrors the spherical-to-Cartesian conversion used by the 3D game
/// page so the two demos produce visually equivalent camera paths. The
/// `look_at` target is fixed at the scene origin's mid-height; only the
/// eye position changes with yaw/pitch.
///
/// # Arguments
///
/// - `Vector3D` - The eye position.
/// - `f64` - The orbit yaw in radians.
/// - `f64` - The orbit pitch in radians.
///
/// # Returns
///
/// - `(Vector3D, Vector3D, Vector3D)` - The (forward, right, up_true) basis.
fn build_camera_basis(eye: Vector3D, yaw: f64, pitch: f64) -> (Vector3D, Vector3D, Vector3D) {
    let look_at: Vector3D = Vector3D::new(0.0, 0.4, 0.0);
    let up: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let forward: Vector3D = (look_at - eye).normalized();
    let _ = yaw;
    let _ = pitch;
    let right: Vector3D = forward.cross(up).normalized();
    let up_true: Vector3D = right.cross(forward).normalized();
    (forward, right, up_true)
}

/// Computes the eye position for the given orbit yaw / pitch angles.
///
/// Mirrors the orbit-to-eye conversion in
/// `create_orbit_camera` from the 3D game page: the camera sits on a
/// sphere of radius `RAYTRACE_CAMERA_DISTANCE` centred on the scene's
/// look-at target, so dragging horizontally rotates around the scene
/// and dragging vertically tilts up / down.
///
/// # Arguments
///
/// - `f64` - The orbit yaw in radians.
/// - `f64` - The orbit pitch in radians.
///
/// # Returns
///
/// - `Vector3D` - The eye position in world space.
fn compute_eye_position(yaw: f64, pitch: f64) -> Vector3D {
    let cos_pitch: f64 = pitch.cos();
    let distance: f64 = RAYTRACE_CAMERA_DISTANCE;
    Vector3D::new(
        distance * yaw.sin() * cos_pitch,
        RAYTRACE_CAMERA_LOOK_AT_Y + distance * pitch.sin(),
        RAYTRACE_CAMERA_LOOK_AT_Z + distance * yaw.cos() * cos_pitch,
    )
}

/// Computes the integer backing buffer dimensions for a render-scale
/// ladder step.
///
/// # Arguments
///
/// - `f64` - The render scale from [`RAYTRACE_RENDER_SCALES`].
///
/// # Returns
///
/// - `(u32, u32)` - The `(width, height)` in pixels (always 4:3).
fn raytrace_scaled_dimensions(scale: f64) -> (u32, u32) {
    let width: u32 = (RAYTRACE_WIDTH * scale).round() as u32;
    let height: u32 = (RAYTRACE_HEIGHT * scale).round() as u32;
    (width, height)
}

/// Renders one full frame of the RayTrace demo into the RGBA byte
/// framebuffer.
///
/// Builds the camera basis once, then for every pixel in the backing
/// buffer computes a primary `Ray`, traces it through the precomputed
/// `RayTraceScene` (zero heap allocation per ray), and packs the
/// resulting linear color into four RGBA bytes with the shared `1/2.2`
/// gamma curve. Lighting is rebuilt per frame by the caller so the
/// directional sun tracks the camera orbit.
///
/// # Arguments
///
/// - `&mut [u8]` - The RGBA framebuffer (length `width * height * 4`).
/// - `u32` - The framebuffer width in pixels.
/// - `u32` - The framebuffer height in pixels.
/// - `&RayTraceScene` - The scene with precomputed shadow data.
/// - `&LightingUniforms` - The per-frame lighting.
/// - `f64` - The orbit yaw in radians.
/// - `f64` - The orbit pitch in radians.
fn render_raytrace_frame(
    buffer: &mut [u8],
    width: u32,
    height: u32,
    scene: &RayTraceScene,
    lights: &LightingUniforms,
    yaw: f64,
    pitch: f64,
) {
    let width_f64: f64 = f64::from(width);
    let height_f64: f64 = f64::from(height);
    let eye: Vector3D = compute_eye_position(yaw, pitch);
    let (forward, right, up_true): (Vector3D, Vector3D, Vector3D) =
        build_camera_basis(eye, yaw, pitch);
    let aspect: f64 = width_f64 / height_f64;
    let focal: f64 = 1.0;
    let inv_width: f64 = 1.0 / width_f64;
    let inv_height: f64 = 1.0 / height_f64;
    let sub_offsets: [(f64, f64); 4] = [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)];
    let mut index: usize = 0;
    for y in 0..height {
        for x in 0..width {
            let mut acc_r: f64 = 0.0;
            let mut acc_g: f64 = 0.0;
            let mut acc_b: f64 = 0.0;
            for (dx_off, dy_off) in sub_offsets {
                let px: f64 = f64::from(x) + dx_off;
                let py: f64 = f64::from(y) + dy_off;
                let ndc_x: f64 = (px * inv_width) * 2.0 - 1.0;
                let ndc_y: f64 = 1.0 - (py * inv_height) * 2.0;
                let dir: Vector3D =
                    (forward.scaled(focal) + right.scaled(ndc_x * aspect) + up_true.scaled(ndc_y))
                        .normalized();
                let ray: Ray = Ray::new(eye, dir);
                let color: Vector3D = scene.trace(ray, lights);
                acc_r += color.get_x();
                acc_g += color.get_y();
                acc_b += color.get_z();
            }
            buffer[index] = gamma_byte(acc_r * 0.25);
            buffer[index + 1] = gamma_byte(acc_g * 0.25);
            buffer[index + 2] = gamma_byte(acc_b * 0.25);
            buffer[index + 3] = 255;
            index += 4;
        }
    }
}

/// Paints the RayTrace CSS-framebackground (the 4:3 ladder region) onto
/// the supplied `SsaaCanvas` offscreen context and presents the
/// high-quality downscale to the display canvas.
///
/// Three concerns drive the layout:
///
/// 1. **The 4:3 ladder is smaller than the offscreen context.** The
///    `SsaaCanvas` is sized to the raytrace canvas's CSS box
///    (`c_game_canvas_wrapper`, 3:2 inline) which is wider than the
///    4:3 ladder framebuffer. A `put_image_data` upload to `(0, 0)`
///    would leave the right 12% of the offscreen backing black. The
///    visual cost is a permanently-letterboxed right edge inside the
///    SSAA buffer, which the downscale faithfully composites.
///
/// 2. **`put_image_data` is a raw upload** with no image-smoothing,
///    so the ladder framebuffer's hard edges survive the 2x SSAA
///    downscale only when the ladder size matches the offscreen
///    physical size exactly. They do not (the ladder is 320x240 at
///    scale 1.0, the offscreen is `css_w * 2 * dpr` x
///    `css_h * 2 * dpr`).
///
/// 3. **`draw_image` is the only path that respects
///    `imageSmoothingEnabled`.** We upload the ladder framebuffer
///    to a fresh 4:3 `HtmlCanvasElement` (its 2D context is
///    transparent) and then `draw_image` it onto the SSAA offscreen
///    context centered inside the wrapper's 4:3 letterbox region.
///    The drawImage call stretches the 4:3 source onto a 4:3
///    destination with `imageSmoothingEnabled = "high"`, which
///    smooths the source's hard edges on the way in. The
///    subsequent `SsaaCanvas::present` does a second
///    `imageSmoothingHigh` downscale from the offscreen to the
///    display canvas, stacking two smoothing passes on the same
///    edge pixels — exactly the dual-stage SSAA the 3D game page
///    uses for its cube and ball rendering.
///
/// The 4:3 destination rectangle is computed from the live canvas
/// CSS box: `min(width, height * 4/3)` wide, `min(height, width *
/// 3/4)` tall, centered. Inline (3:2) CSS boxes get a horizontal
/// letterbox (CSS box wider than 4:3); fullscreen (16:9) CSS boxes
/// get a vertical letterbox (CSS box taller than 4:3). Either way
/// the destination is a 4:3 rectangle that exactly fits the ladder
/// framebuffer's native aspect ratio so the drawImage call does no
/// additional aspect-ratio stretch.
///
/// # Arguments
///
/// - `&SsaaCanvas` - The SSAA wrapper whose offscreen context is
///   the draw target.
/// - `&mut [u8]` - The RGBA framebuffer (length `width * height * 4`).
/// - `u32` - The framebuffer width in pixels.
/// - `u32` - The framebuffer height in pixels.
fn present_raytrace_framebuffer(
    ssaa_canvas: &SsaaCanvas,
    buffer: &mut [u8],
    width: u32,
    height: u32,
) {
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let Some(document_value): Option<Document> = window_value.document() else {
        return;
    };
    let Ok(source_canvas): Result<HtmlCanvasElement, JsValue> = document_value
        .create_element("canvas")
        .map(|element: Element| element.unchecked_into())
    else {
        return;
    };
    source_canvas.set_width(width);
    source_canvas.set_height(height);
    let Ok(Some(source_context_object)): Result<Option<Object>, JsValue> =
        source_canvas.get_context(RAYTRACE_CONTEXT_TYPE)
    else {
        return;
    };
    let source_context: CanvasRenderingContext2d = source_context_object.unchecked_into();
    let image_data: Result<ImageData, JsValue> =
        ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(buffer), width, height);
    let Ok(image_data) = image_data else {
        return;
    };
    let _: Result<(), JsValue> = source_context.put_image_data(&image_data, 0.0, 0.0);
    let offscreen_context: &CanvasRenderingContext2d = ssaa_canvas.get_offscreen_context();
    let offscreen_width: f64 = ssaa_canvas.get_width();
    let offscreen_height: f64 = ssaa_canvas.get_height();
    let dest_w: f64 = offscreen_width.min(offscreen_height * 4.0 / 3.0);
    let dest_h: f64 = offscreen_height.min(offscreen_width * 3.0 / 4.0);
    let dest_x: f64 = (offscreen_width - dest_w) * 0.5;
    let dest_y: f64 = (offscreen_height - dest_h) * 0.5;
    let _: Result<(), JsValue> = offscreen_context
        .draw_image_with_html_canvas_element_and_dw_and_dh(
            &source_canvas,
            dest_x,
            dest_y,
            dest_w,
            dest_h,
        );
    ssaa_canvas.present();
}

/// Starts the RayTrace Canvas 2D `requestAnimationFrame` loop.
///
/// Per frame: applies auto-rotate yaw if enabled, rebuilds lighting
/// from the current yaw, re-traces every pixel into the persistent RGBA
/// framebuffer, and uploads it with one `put_image_data` call. An
/// exponential moving average of the CPU render time drives the
/// [`RAYTRACE_RENDER_SCALES`] adaptive-resolution ladder: sustained
/// frames above 115% of the 60 FPS budget step the internal resolution
/// down one rung, sustained frames below 75% step it back up one rung,
/// and sustained frames below 45% step it up two rungs at once. The
/// FPS counter uses unclamped wall-clock elapsed time so it reports
/// honest rates even below 4 FPS; the `0.25s` clamp applies only to
/// the yaw animation step. The `use_cleanup` cancellation and
/// canvas-detached guard mirror the game_2d / game_3d pattern.
///
/// # Arguments
///
/// - `UseRayTrace` - The RayTrace Canvas 2D tab state.
/// - `RayTraceCameraAngles` - The non-reactive camera orbit angles.
pub(crate) fn start_raytrace_loop(state: UseRayTrace, angles: RayTraceCameraAngles) {
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let (occluders, eye) = build_raytrace_scene();
    let scene: RayTraceScene = RayTraceScene::new(occluders);
    // The `SsaaCanvas` is rebuilt from scratch on a CSS-box resize
    // (its constructor re-acquires the display canvas, resizes the
    // display backing store, and allocates a new offscreen canvas at
    // the new scale). Storing `None` in the cache and re-acquiring on
    // divergence mirrors the game_2d Canvas 2D pattern (see
    // `handle_rescale_dirty_canvas2d` in `game_2d/hook/fn.rs`).
    let ssaa_cache: Rc<RefCell<Option<(HtmlCanvasElement, SsaaCanvas)>>> =
        Rc::new(RefCell::new(None));
    let last_canvas_size: Rc<RefCell<(f64, f64)>> = Rc::new(RefCell::new((0.0, 0.0)));
    let framebuffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
    // Start at ladder index 7 (scale 1.0): weak clients never start
    // heavy, and the controller climbs toward 4.0 only after sustained
    // fast frames prove the budget allows it.
    let scale_index: Rc<Cell<usize>> = Rc::new(Cell::new(7));
    let ema_millis: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let slow_frames: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fast_frames: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let very_fast_frames: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let last_clone: Rc<Cell<f64>> = last_time.clone();
    let frame_clone: Rc<Cell<u32>> = frame_count.clone();
    let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
    let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_clone: RafClosureCell = closure_cell.clone();
    let yaw_clone: Rc<Cell<f64>> = angles.yaw.clone();
    let pitch_clone: Rc<Cell<f64>> = angles.pitch.clone();
    let cache_clone: Rc<RefCell<Option<(HtmlCanvasElement, SsaaCanvas)>>> = ssaa_cache.clone();
    let last_size_clone: Rc<RefCell<(f64, f64)>> = last_canvas_size.clone();
    let buffer_clone: Rc<RefCell<Vec<u8>>> = framebuffer.clone();
    let scale_clone: Rc<Cell<usize>> = scale_index.clone();
    let ema_clone: Rc<Cell<f64>> = ema_millis.clone();
    let slow_clone: Rc<Cell<u32>> = slow_frames.clone();
    let fast_clone: Rc<Cell<u32>> = fast_frames.clone();
    let very_fast_clone: Rc<Cell<u32>> = very_fast_frames.clone();
    let state_for_loop: UseRayTrace = state;
    // Paint the loading overlay *before* the first frame so the user
    // sees a centered "Initializing..." line during the SSAA acquire
    // + first warmup ray pass. The 200-400 ms window is short
    // enough that the overlay usually disappears in a single frame,
    // but synchronous WASM module init can delay it further on slow
    // devices, and without this paint the canvas stays blank /
    // half-rendered for that entire window.
    if let Some(window_value) = window() {
        let loading_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            draw_game_3d_loading(RAYTRACE_LOADING_CANVAS_SELECTOR, RAYTRACE_CANVAS_SELECTOR);
        }));
        let loading_callback: Function =
            loading_closure.as_ref().unchecked_ref::<Function>().clone();
        loading_closure.forget();
        let _ = window_value
            .set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
    }
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        if raytrace_canvas_detached(RAYTRACE_CANVAS_SELECTOR) {
            return;
        }
        let Some(window_value): Option<Window> = window() else {
            return;
        };
        let Some(performance): Option<Performance> = window_value.performance() else {
            return;
        };
        let current_time: f64 = performance.now() / 1000.0;
        let prev: f64 = last_clone.get();
        // The unclamped frame time feeds the FPS counter so the reported
        // rate stays honest even when frames take multiple seconds; only
        // the yaw animation step consumes the clamped delta.
        let frame_time: f64 = if prev < 0.0 {
            1.0 / 60.0
        } else {
            current_time - prev
        };
        let anim_time: f64 = frame_time.min(0.25);
        last_clone.set(current_time);
        if state_for_loop.get_auto_rotate().get() {
            yaw_clone.set(yaw_clone.get() + RAYTRACE_AUTO_YAW_SPEED * anim_time);
        }
        let yaw: f64 = yaw_clone.get();
        let pitch: f64 = pitch_clone.get();
        if state_for_loop.get_running().get() {
            // Resize the SSAA wrapper when the live CSS box diverges
            // from the cached size. `getBoundingClientRect` is the
            // only reading that tracks the live layout; using
            // `canvas.width` / `canvas.height` would loop on the
            // backing store we just wrote.
            let (live_w, live_h): (f64, f64) =
                read_raytrace_canvas_size(RAYTRACE_CANVAS_SELECTOR).unwrap_or((0.0, 0.0));
            let (cached_w, cached_h) = *last_size_clone.borrow();
            let needs_reacquire: bool = live_w > 0.0
                && live_h > 0.0
                && (cached_w <= 0.0
                    || cached_h <= 0.0
                    || (live_w - cached_w).abs() > 1.5
                    || (live_h - cached_h).abs() > 1.5);
            if needs_reacquire {
                if let Some((_, ssaa_canvas)) = acquire_raytrace_ssaa_canvas() {
                    *cache_clone.borrow_mut() =
                        Some((ssaa_canvas.get_display_canvas().clone(), ssaa_canvas));
                }
                *last_size_clone.borrow_mut() = (live_w, live_h);
            }
            let scale: f64 = RAYTRACE_RENDER_SCALES[scale_clone.get()];
            let (frame_width, frame_height): (u32, u32) = raytrace_scaled_dimensions(scale);
            if let Some((_canvas, ssaa_canvas)) = cache_clone.borrow().as_ref() {
                let lights: LightingUniforms = build_raytrace_lighting(eye, yaw);
                let render_start: f64 = performance.now();
                {
                    let mut buffer = buffer_clone.borrow_mut();
                    let needed: usize = frame_width as usize * frame_height as usize * 4;
                    if buffer.len() != needed {
                        buffer.resize(needed, 0);
                    }
                    render_raytrace_frame(
                        &mut buffer,
                        frame_width,
                        frame_height,
                        &scene,
                        &lights,
                        yaw,
                        pitch,
                    );
                    present_raytrace_framebuffer(
                        ssaa_canvas,
                        &mut buffer,
                        frame_width,
                        frame_height,
                    );
                }
                let render_millis: f64 = performance.now() - render_start;
                let ema_prev: f64 = ema_clone.get();
                let ema: f64 = if ema_prev <= 0.0 {
                    render_millis
                } else {
                    ema_prev * (1.0 - RAYTRACE_ADAPT_EMA_ALPHA)
                        + render_millis * RAYTRACE_ADAPT_EMA_ALPHA
                };
                ema_clone.set(ema);
                if ema > RAYTRACE_ADAPT_SLOW_FRAME_MILLIS {
                    slow_clone.set(slow_clone.get() + 1);
                    fast_clone.set(0);
                    very_fast_clone.set(0);
                } else if ema < RAYTRACE_ADAPT_VERY_FAST_FRAME_MILLIS {
                    fast_clone.set(fast_clone.get() + 1);
                    very_fast_clone.set(very_fast_clone.get() + 1);
                    slow_clone.set(0);
                } else if ema < RAYTRACE_ADAPT_FAST_FRAME_MILLIS {
                    fast_clone.set(fast_clone.get() + 1);
                    slow_clone.set(0);
                    very_fast_clone.set(0);
                } else {
                    slow_clone.set(0);
                    fast_clone.set(0);
                    very_fast_clone.set(0);
                }
                let index: usize = scale_clone.get();
                let mut next: usize = index;
                if slow_clone.get() >= RAYTRACE_ADAPT_SLOW_FRAMES
                    && index + 1 < RAYTRACE_RENDER_SCALES.len()
                {
                    next = index + 1;
                } else if fast_clone.get() >= RAYTRACE_ADAPT_FAST_FRAMES && index > 0 {
                    // Sustained headroom far below the budget skips a
                    // rung so strong hardware reaches the sharp 4.0 top
                    // of the ladder in a handful of steps instead of
                    // crawling one rung at a time.
                    next = if very_fast_clone.get() >= RAYTRACE_ADAPT_FAST_FRAMES {
                        index.saturating_sub(2)
                    } else {
                        index - 1
                    };
                }
                if next != index {
                    scale_clone.set(next);
                    slow_clone.set(0);
                    fast_clone.set(0);
                    very_fast_clone.set(0);
                    state_for_loop
                        .get_render_scale()
                        .set(RAYTRACE_RENDER_SCALES[next]);
                }
                // Flip the active / loaded flags on the first successful
                // frame so the loading overlay unloads and the
                // `Status: ...` banner reports a live renderer. The
                // `loaded` set is delayed by
                // `RAYTRACE_CANVAS_2D_LOADING_MIN_MILLIS` so the
                // overlay stays painted for a minimum visible duration
                // even when the SSAA acquire + first frame finishes
                // in less than a frame budget — the same UX the
                // WebGL / WebGPU tabs use via `raytrace_set_loaded_delayed`.
                if !state_for_loop.get_active().get() {
                    state_for_loop.get_active().set(true);
                    raytrace_set_loaded_delayed_canvas2d(
                        state_for_loop.get_loaded(),
                        RAYTRACE_CANVAS_2D_LOADING_MIN_MILLIS,
                    );
                }
            }
        }
        frame_clone.set(frame_clone.get() + 1);
        fps_clone.set(fps_clone.get() + frame_time);
        if fps_clone.get() >= 1.0 {
            let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
            state_for_loop.get_fps().set(fps);
            frame_clone.set(0);
            fps_clone.set(0.0);
        }
        let Some(raf_closure_ref): Option<&'static Closure<dyn FnMut()>> = cell_clone.try_get()
        else {
            return;
        };
        let next_id: i32 = window_value
            .request_animation_frame(raf_closure_ref.as_ref().unchecked_ref())
            .unwrap_or_default();
        raf_clone.set(Some(next_id));
    }));
    let _: Result<(), _> = closure_cell.try_set(raf_closure);
    let start_timeout_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let start_timeout_clone: Rc<Cell<Option<i32>>> = start_timeout_id.clone();
    let raf_for_start: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_start: RafClosureCell = closure_cell.clone();
    let start_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let Some(start_window): Option<Window> = window() else {
            return;
        };
        let Some(start_raf_ref): Option<&'static Closure<dyn FnMut()>> = cell_for_start.try_get()
        else {
            return;
        };
        let start_id: i32 = start_window
            .request_animation_frame(start_raf_ref.as_ref().unchecked_ref())
            .unwrap_or_default();
        raf_for_start.set(Some(start_id));
    }));
    let start_callback: Function = start_closure.as_ref().unchecked_ref::<Function>().clone();
    start_closure.forget();
    let Some(start_window): Option<Window> = window() else {
        return;
    };
    let timeout_id: i32 = start_window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            &start_callback,
            RAYTRACE_LOOP_START_DELAY_MILLIS,
        )
        .unwrap_or_default();
    start_timeout_clone.set(Some(timeout_id));
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    App::use_cleanup(move || {
        if let Some(cancel_id) = raf_for_cleanup.get() {
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            let _ = window_value.cancel_animation_frame(cancel_id);
        }
        if let Some(timeout_id) = start_timeout_id.get() {
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            window_value.clear_timeout_with_handle(timeout_id);
        }
        let _: Option<_> = cell_for_cleanup.try_take();
    });
    state.get_loop_started().set(true);
}

/// Creates a click handler that toggles a RayTrace tab loop between
/// running and paused.
///
/// # Arguments
///
/// - `Signal<bool>` - The running signal of the active tab's loop.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle handler.
pub(crate) fn raytrace_on_toggle_pause(running: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = running.get();
        running.set(!current);
    }))
}

/// Creates a click handler that toggles camera auto-rotation.
///
/// # Arguments
///
/// - `Signal<bool>` - The auto-rotate signal of the active tab's loop.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle handler.
pub(crate) fn raytrace_on_toggle_auto_rotate(
    auto_rotate: Signal<bool>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = auto_rotate.get();
        auto_rotate.set(!current);
    }))
}

/// Creates a click handler that resets the camera orbit angles.
///
/// # Arguments
///
/// - `RayTraceCameraAngles` - The non-reactive camera orbit angles.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle handler.
pub(crate) fn raytrace_on_reset_camera(angles: RayTraceCameraAngles) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        angles.yaw.set(0.6);
        angles.pitch.set(0.25);
    }))
}

/// Reads the client X coordinate off a `MouseEvent`-like object via reflection.
///
/// # Arguments
///
/// - `&Event` - The pointer event.
///
/// # Returns
///
/// - `f64` - The client X coordinate, or `0.0` if missing.
fn client_x_from_event(event: &Event) -> f64 {
    Reflect::get(event.as_ref(), &JsValue::from_str("clientX"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default()
}

/// Reads the client Y coordinate off a `MouseEvent`-like object via reflection.
///
/// # Arguments
///
/// - `&Event` - The pointer event.
///
/// # Returns
///
/// - `f64` - The client Y coordinate, or `0.0` if missing.
fn client_y_from_event(event: &Event) -> f64 {
    Reflect::get(event.as_ref(), &JsValue::from_str("clientY"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default()
}

/// Creates a pointer move handler that updates the camera orbit angles
/// from drag movement and disables auto-rotate for the rest of the
/// session.
///
/// # Arguments
///
/// - `RayTraceCameraAngles` - The non-reactive camera orbit angles.
/// - `Signal<bool>` - The auto-rotate signal of the active tab's loop.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer move handler.
pub(crate) fn raytrace_on_pointer_move(
    angles: RayTraceCameraAngles,
    auto_rotate: Signal<bool>,
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let last: Option<(f64, f64)> = last_pointer.get();
        let Some((last_x, last_y)) = last else {
            return;
        };
        let client_x: f64 = client_x_from_event(&event);
        let client_y: f64 = client_y_from_event(&event);
        let dx: f64 = client_x - last_x;
        let dy: f64 = client_y - last_y;
        last_pointer.set(Some((client_x, client_y)));
        let yaw: f64 = angles.yaw.get() - dx * RAYTRACE_DRAG_SENSITIVITY;
        let pitch: f64 = (angles.pitch.get() + dy * RAYTRACE_DRAG_SENSITIVITY).clamp(
            -HALF_PI + RAYTRACE_PITCH_CLAMP,
            HALF_PI - RAYTRACE_PITCH_CLAMP,
        );
        angles.yaw.set(yaw);
        angles.pitch.set(pitch);
        auto_rotate.set(false);
    }))
}

/// Creates a pointer down handler that records the drag start position.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer down handler.
pub(crate) fn raytrace_on_pointer_down(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let client_x: f64 = client_x_from_event(&event);
        let client_y: f64 = client_y_from_event(&event);
        last_pointer.set(Some((client_x, client_y)));
    }))
}

/// Creates a pointer up / leave handler that clears the drag state.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer up handler.
pub(crate) fn raytrace_on_pointer_up(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        last_pointer.set(None);
    }))
}

/// Extracts the first touch's client coordinates from a `TouchEvent`.
///
/// # Arguments
///
/// - `&Event` - The native touch event.
///
/// # Returns
///
/// - `(f64, f64)` - The `(client_x, client_y)` of the first touch.
fn first_touch_client(event: &Event) -> (f64, f64) {
    let touches_value: JsValue = Reflect::get(
        event.as_ref(),
        &JsValue::from_str(RAYTRACE_EVENT_PROPERTY_TOUCHES),
    )
    .ok()
    .unwrap_or(JsValue::NULL);
    let touches: Array = touches_value.unchecked_into();
    if touches.length() == 0 {
        return (0.0, 0.0);
    }
    let touch: JsValue = touches.get(0);
    let client_x: f64 = Reflect::get(&touch, &JsValue::from_str(RAYTRACE_EVENT_PROPERTY_CLIENT_X))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default();
    let client_y: f64 = Reflect::get(&touch, &JsValue::from_str(RAYTRACE_EVENT_PROPERTY_CLIENT_Y))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default();
    (client_x, client_y)
}

/// Creates a touch start handler that records the first touch position
/// and prevents default to avoid page scrolling during camera drag.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch start handler.
pub(crate) fn raytrace_on_touch_start(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        let (client_x, client_y): (f64, f64) = first_touch_client(&event);
        last_pointer.set(Some((client_x, client_y)));
    }))
}

/// Creates a touch move handler that updates orbit angles from the
/// single-finger drag and disables auto-rotate.
///
/// # Arguments
///
/// - `RayTraceCameraAngles` - The non-reactive camera orbit angles.
/// - `Signal<bool>` - The auto-rotate signal of the active tab's loop.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch move handler.
pub(crate) fn raytrace_on_touch_move(
    angles: RayTraceCameraAngles,
    auto_rotate: Signal<bool>,
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        let last: Option<(f64, f64)> = last_pointer.get();
        let Some((last_x, last_y)) = last else {
            return;
        };
        let (client_x, client_y): (f64, f64) = first_touch_client(&event);
        let dx: f64 = client_x - last_x;
        let dy: f64 = client_y - last_y;
        last_pointer.set(Some((client_x, client_y)));
        let yaw: f64 = angles.yaw.get() - dx * RAYTRACE_DRAG_SENSITIVITY;
        let pitch: f64 = (angles.pitch.get() + dy * RAYTRACE_DRAG_SENSITIVITY).clamp(
            -HALF_PI + RAYTRACE_PITCH_CLAMP,
            HALF_PI - RAYTRACE_PITCH_CLAMP,
        );
        angles.yaw.set(yaw);
        angles.pitch.set(pitch);
        auto_rotate.set(false);
    }))
}

/// Creates a touch end handler that clears the drag state.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch end handler.
pub(crate) fn raytrace_on_touch_end(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        last_pointer.set(None);
    }))
}

/// Packs the per-frame uniform data consumed by the WebGL and WebGPU
/// raytrace shaders.
///
/// Layout (8 `vec4` slots, matching `u_params[8]` /
/// `SceneUniforms`): orbit eye, camera forward, camera right, camera
/// up, sun direction, sun color, ambient, and canvas resolution. The
/// eye used for specular shading inside the shaders is the fixed
/// `SHADE_EYE` constant, matching the CPU path.
///
/// # Arguments
///
/// - `f64` - The orbit yaw in radians.
/// - `f64` - The orbit pitch in radians.
/// - `f64` - The canvas backing width in physical pixels.
/// - `f64` - The canvas backing height in physical pixels.
///
/// # Returns
///
/// - `Vec<f32>` - The packed uniform data (32 floats).
fn pack_raytrace_gpu_uniform(yaw: f64, pitch: f64, width: f64, height: f64) -> Vec<f32> {
    let eye: Vector3D = compute_eye_position(yaw, pitch);
    let (forward, right, up_true): (Vector3D, Vector3D, Vector3D) =
        build_camera_basis(eye, yaw, pitch);
    let sun_dir: Vector3D = raytrace_sun_direction(yaw);
    let mut data: Vec<f32> = Vec::with_capacity(RAYTRACE_GPU_UNIFORM_VEC4_COUNT * 4);
    let vectors: [Vector3D; 5] = [eye, forward, right, up_true, sun_dir];
    for vector in vectors {
        data.push(vector.get_x() as f32);
        data.push(vector.get_y() as f32);
        data.push(vector.get_z() as f32);
        data.push(0.0);
    }
    data.extend_from_slice(&[1.0, 0.95, 0.85, 0.0]);
    data.extend_from_slice(&[0.10, 0.10, 0.14, 0.0]);
    data.extend_from_slice(&[width as f32, height as f32, 0.0, 0.0]);
    data
}

/// Sets the Canvas 2D `loaded` signal after a short delay so the
/// loading overlay is actually painted before it is removed.
///
/// Mirrors [`raytrace_set_loaded_delayed`] (used by the WebGL / WebGPU
/// tabs) and shares its `GAME_3D_LOADING_MIN_MILLIS` floor so the
/// overlay stays visible for a minimum duration even on devices where
/// the first frame finishes inside the same `requestAnimationFrame`
/// tick that the overlay is mounted on. Reusing the same floor keeps
/// all three RayTrace tabs visually consistent: the user always sees
/// the "Initializing..." text for the same wall-clock window no
/// matter which tab they boot the page on or switch into.
///
/// # Arguments
///
/// - `Signal<bool>` - The Canvas 2D `loaded` signal to set.
/// - `i32` - The delay in milliseconds before setting the signal.
fn raytrace_set_loaded_delayed_canvas2d(loaded: Signal<bool>, millis: i32) {
    let loaded_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        loaded.set(true);
    }));
    let loaded_callback: Function = loaded_closure.as_ref().unchecked_ref::<Function>().clone();
    loaded_closure.forget();
    let Some(loaded_window): Option<Window> = window() else {
        return;
    };
    let _ = loaded_window
        .set_timeout_with_callback_and_timeout_and_arguments_0(&loaded_callback, millis);
}

/// Sets the backend `loaded` signal after a short delay so the loading
/// overlay is actually painted before it is removed.
///
/// Synchronous WebGL init (and fast WebGPU init) would otherwise add and
/// remove the overlay canvas within a single frame, so the browser never
/// paints the loading state when switching tabs.
///
/// # Arguments
///
/// - `Signal<bool>` - The backend `loaded` signal to set.
/// - `i32` - The delay in milliseconds before setting the signal.
fn raytrace_set_loaded_delayed(loaded: Signal<bool>, millis: i32) {
    let loaded_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        loaded.set(true);
    }));
    let loaded_callback: Function = loaded_closure.as_ref().unchecked_ref::<Function>().clone();
    loaded_closure.forget();
    let Some(loaded_window): Option<Window> = window() else {
        return;
    };
    let _ = loaded_window
        .set_timeout_with_callback_and_timeout_and_arguments_0(&loaded_callback, millis);
}

/// Registers the debounced window-resize listener shared by the WebGL
/// and WebGPU raytrace loops.
///
/// # Arguments
///
/// - `Rc<Cell<bool>>` - The resize-dirty flag set after the debounce fires.
/// - `Rc<Cell<Option<i32>>>` - The pending debounce timer handle.
fn raytrace_register_resize_debounce(
    resize_dirty: Rc<Cell<bool>>,
    resize_timer: Rc<Cell<Option<i32>>>,
) {
    let resize_dirty_for_event: Rc<Cell<bool>> = resize_dirty.clone();
    let resize_timer_for_event: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let debounce_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        resize_dirty_for_event.set(true);
    }));
    let debounce_callback: Function = debounce_closure
        .as_ref()
        .unchecked_ref::<Function>()
        .clone();
    debounce_closure.forget();
    let Some(resize_window): Option<Window> = window() else {
        return;
    };
    App::use_window_event("resize", move || {
        let old_timer: Option<i32> = resize_timer_for_event.get();
        if let Some(timer_id) = old_timer {
            let Some(clear_window): Option<Window> = window() else {
                return;
            };
            clear_window.clear_timeout_with_handle(timer_id);
        }
        let new_timer: i32 = resize_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &debounce_callback,
                GAME_3D_RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        resize_timer_for_event.set(Some(new_timer));
    });
}

/// Starts the RayTrace WebGL loop driven by `requestAnimationFrame`.
///
/// Renders the same scene as the Canvas 2D tab through a GLSL ES 3.00
/// fragment shader on a fullscreen triangle. The canvas backing store
/// tracks the CSS box times the device pixel ratio (synchronous
/// ResizeObserver plus a debounced window-resize flag plus a per-frame
/// divergence check, mirroring the 3D game page), and the shader
/// aspect-corrects the NDC via the resolution uniform so geometry never
/// stretches. WebGL initialization is synchronous; the `spawn_local`
/// wrapper only defers execution past the current render pass so the
/// canvas element exists in the DOM.
///
/// # Arguments
///
/// - `UseRayTraceWebGl` - The WebGL backend state for signal updates.
/// - `RayTraceCameraAngles` - The shared non-reactive camera orbit angles.
pub(crate) fn start_raytrace_webgl_loop(state: UseRayTraceWebGl, angles: RayTraceCameraAngles) {
    let init_state: UseRayTraceWebGl = state;
    let loop_state: UseRayTraceWebGl = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let renderer_rc: Rc<RefCell<Option<WebGlRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let observer_cell: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));
    raytrace_register_resize_debounce(resize_dirty.clone(), resize_timer.clone());
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let renderer_for_cleanup: Rc<RefCell<Option<WebGlRenderer>>> = renderer_rc.clone();
    let resize_timer_for_cleanup: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let cancelled_for_cleanup: Rc<Cell<bool>> = cancelled.clone();
    let observer_for_cleanup: Rc<RefCell<Option<ResizeObserver>>> = observer_cell.clone();
    App::use_cleanup(move || {
        cancelled_for_cleanup.set(true);
        // Every step is independent: a missing `window()` must never skip
        // the renderer teardown below, so no early returns here.
        if let Some(cancel_id) = raf_for_cleanup.get()
            && let Some(window_value) = window()
        {
            let _ = window_value.cancel_animation_frame(cancel_id);
        }
        if let Some(timer_id) = resize_timer_for_cleanup.get()
            && let Some(window_value) = window()
        {
            window_value.clear_timeout_with_handle(timer_id);
        }
        // Disconnect the ResizeObserver so its closure (and the renderer
        // it holds via `renderer_for_observer`) is released on tab
        // switch. Without this the observer keeps the renderer alive
        // past the loop's lifetime, holding GPU resources until GC.
        if let Some(observer) = observer_for_cleanup.borrow_mut().take() {
            observer.disconnect();
        }
        let _: Option<_> = cell_for_cleanup.try_take();
        // WebGL has no explicit `destroy()` on the context: dropping the
        // last JS reference lets the browser GC reclaim the GL context.
        let _: Option<WebGlRenderer> = renderer_for_cleanup.borrow_mut().take();
    });
    let cancelled_for_init: Rc<Cell<bool>> = cancelled.clone();
    let Some(loading_window): Option<Window> = window() else {
        return;
    };
    let loading_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        draw_game_3d_loading(
            RAYTRACE_WEBGL_LOADING_CANVAS_SELECTOR,
            RAYTRACE_WEBGL_CANVAS_SELECTOR,
        );
    }));
    let loading_callback: Function = loading_closure.as_ref().unchecked_ref::<Function>().clone();
    loading_closure.forget();
    let _ =
        loading_window.set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
    spawn_local(async move {
        if cancelled_for_init.get() {
            return;
        }
        let config: RenderConfig = RenderConfig::webgl(
            RAYTRACE_WEBGL_CANVAS_SELECTOR,
            RAYTRACE_WIDTH,
            RAYTRACE_HEIGHT,
        );
        let renderer: WebGlRenderer = match Engine::webgl_renderer(&config) {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!("[euv-engine][raytrace] webgl init failed: {error}"));
                init_state.get_init_error_code().set(error.code());
                init_state.get_loaded().set(true);
                return;
            }
        };
        let program: WebGlProgram = match renderer
            .create_program(RAYTRACE_WEBGL_VERTEX_SHADER, RAYTRACE_WEBGL_FRAGMENT_SHADER)
        {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!(
                    "[euv-engine][raytrace] webgl program failed: {error}"
                ));
                init_state.get_init_error_code().set("WEBGL_PROGRAM_ERROR");
                init_state.get_loaded().set(true);
                return;
            }
        };
        // Resolve the uniform location once after link; per-frame
        // `getUniformLocation` calls are pure overhead and the location is
        // stable for the lifetime of the program.
        let params_location: Rc<Option<WebGlUniformLocation>> =
            Rc::new(renderer.get_uniform_location(&program, "u_params[0]"));
        let clear_color: Rc<Cell<(f64, f64, f64)>> = Rc::new(Cell::new(
            game_3d_canvas_clear_color(RAYTRACE_WEBGL_CANVAS_SELECTOR),
        ));
        init_state.get_active().set(true);
        // Delay flipping `loaded` so the loading overlay stays painted for a
        // minimum visible duration even when init completes instantly.
        raytrace_set_loaded_delayed(init_state.get_loaded(), GAME_3D_LOADING_MIN_MILLIS);
        *renderer_rc.borrow_mut() = Some(renderer);
        let program_rc: Rc<WebGlProgram> = Rc::new(program);
        // Synchronous resize on CSS-box change. ResizeObserver callbacks
        // run BEFORE the browser paints the next frame, so setting
        // `canvas.width = new_w` inside the observer ensures the very
        // first paint after fullscreen enter/exit already has the new
        // backing store instead of one stretched frame of the old
        // backing. `canvas.width` is applied BEFORE `renderer.resize`
        // because the DOM setter is fast while the GL-side realloc can
        // stall the main thread.
        let renderer_for_observer: Rc<RefCell<Option<WebGlRenderer>>> = renderer_rc.clone();
        let observer_closure: Closure<dyn FnMut(js_sys::Array, ResizeObserver)> = Closure::wrap(
            Box::new(move |_entries: js_sys::Array, _obs: ResizeObserver| {
                let Some(window_value): Option<Window> = window() else {
                    return;
                };
                let Some(document_value): Option<Document> = window_value.document() else {
                    return;
                };
                let Some(element): Option<Element> = document_value
                    .query_selector(RAYTRACE_WEBGL_CANVAS_SELECTOR)
                    .ok()
                    .flatten()
                else {
                    return;
                };
                let canvas: HtmlCanvasElement = element.unchecked_into();
                let rect: DomRect = canvas.get_bounding_client_rect();
                let css_w: f64 = rect.width();
                let css_h: f64 = rect.height();
                if css_w <= 0.0 || css_h <= 0.0 {
                    return;
                }
                let dpr: f64 = Reflect::get(
                    window_value.as_ref(),
                    &JsValue::from_str("devicePixelRatio"),
                )
                .ok()
                .and_then(|v: JsValue| v.as_f64())
                .filter(|v: &f64| v.is_finite() && *v >= 1.0)
                .unwrap_or(1.0);
                let new_w: u32 = (css_w * dpr).round() as u32;
                let new_h: u32 = (css_h * dpr).round() as u32;
                let backing_w: u32 = canvas.width();
                let backing_h: u32 = canvas.height();
                if backing_w != new_w || backing_h != new_h {
                    canvas.set_width(new_w);
                    canvas.set_height(new_h);
                    if let Some(renderer) = renderer_for_observer.borrow_mut().as_mut() {
                        renderer.resize(new_w, new_h);
                    }
                }
            }),
        );
        let observer_callback: Function = observer_closure
            .as_ref()
            .unchecked_ref::<Function>()
            .clone();
        observer_closure.forget();
        if let Ok(resize_observer) = ResizeObserver::new(&observer_callback)
            && let Some(window_value) = window()
            && let Some(document_value) = window_value.document()
            && let Some(element) = document_value
                .query_selector(RAYTRACE_WEBGL_CANVAS_SELECTOR)
                .ok()
                .flatten()
        {
            resize_observer.observe(&element);
            *observer_cell.borrow_mut() = Some(resize_observer);
        }
        let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
        let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        let renderer_for_loop: Rc<RefCell<Option<WebGlRenderer>>> = renderer_rc.clone();
        let program_for_loop: Rc<WebGlProgram> = program_rc.clone();
        let params_location_for_loop: Rc<Option<WebGlUniformLocation>> = params_location.clone();
        let clear_color_for_loop: Rc<Cell<(f64, f64, f64)>> = clear_color.clone();
        let yaw_for_loop: Rc<Cell<f64>> = angles.yaw.clone();
        let pitch_for_loop: Rc<Cell<f64>> = angles.pitch.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let resize_dirty_for_loop: Rc<Cell<bool>> = resize_dirty.clone();
        let cancelled_for_loop: Rc<Cell<bool>> = cancelled.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            // Stop on tab-switch cleanup (`cancelled`) or when the canvas
            // left the document (router navigation fires no cleanup).
            if cancelled_for_loop.get() || raytrace_canvas_detached(RAYTRACE_WEBGL_CANVAS_SELECTOR)
            {
                return;
            }
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            let Some(performance): Option<Performance> = window_value.performance() else {
                return;
            };
            let current_time: f64 = performance.now() / 1000.0;
            let prev: f64 = last_clone.get();
            // Unclamped frame time feeds the FPS counter (honest rate);
            // the clamped delta only drives the yaw animation step.
            let frame_time: f64 = if prev < 0.0 {
                1.0 / 60.0
            } else {
                current_time - prev
            };
            let anim_time: f64 = frame_time.min(0.25);
            last_clone.set(current_time);
            if loop_state.get_auto_rotate().get() {
                yaw_for_loop.set(yaw_for_loop.get() + RAYTRACE_AUTO_YAW_SPEED * anim_time);
            }
            let resize_dirty_frame: bool = if resize_dirty_for_loop.get() {
                resize_dirty_for_loop.set(false);
                true
            } else {
                false
            };
            let Some(window_for_dpr): Option<Window> = window() else {
                return;
            };
            let dpr: f64 = Reflect::get(
                window_for_dpr.as_ref(),
                &JsValue::from_str("devicePixelRatio"),
            )
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .filter(|value: &f64| value.is_finite() && *value >= 1.0)
            .unwrap_or(1.0);
            // Read the canvas's CSS pixel dimensions via
            // `getBoundingClientRect` (NOT `clientWidth`/`clientHeight` -
            // the latter track the backing store in Chrome and would
            // create a feedback loop if read every frame).
            let (canvas_width, canvas_height): (f64, f64) =
                read_raytrace_canvas_size(RAYTRACE_WEBGL_CANVAS_SELECTOR).unwrap_or((0.0, 0.0));
            let new_physical_width: u32 = (canvas_width * dpr).round() as u32;
            let new_physical_height: u32 = (canvas_height * dpr).round() as u32;
            if let Some(renderer) = renderer_for_loop.borrow_mut().as_mut() {
                // Resize the WebGL backing store every frame the CSS box
                // diverges from `canvas.width` / `canvas.height`. The
                // per-frame check collapses the stretched-frame window
                // after fullscreen transitions to a single frame; the
                // ResizeObserver path normally beats it to zero.
                if new_physical_width > 0 && new_physical_height > 0 {
                    let backing_w: u32 = renderer.get_canvas().width();
                    let backing_h: u32 = renderer.get_canvas().height();
                    if backing_w != new_physical_width || backing_h != new_physical_height {
                        renderer.get_canvas().set_width(new_physical_width);
                        renderer.get_canvas().set_height(new_physical_height);
                        renderer.resize(new_physical_width, new_physical_height);
                    }
                }
                if resize_dirty_frame {
                    renderer.resize(new_physical_width, new_physical_height);
                }
                if loop_state.get_running().get() {
                    let backing_w: f64 = f64::from(renderer.get_canvas().width());
                    let backing_h: f64 = f64::from(renderer.get_canvas().height());
                    let uniform_data: Vec<f32> = pack_raytrace_gpu_uniform(
                        yaw_for_loop.get(),
                        pitch_for_loop.get(),
                        backing_w,
                        backing_h,
                    );
                    renderer.set_uniform_4fv(
                        &program_for_loop,
                        params_location_for_loop.as_ref().as_ref(),
                        &uniform_data,
                    );
                    // Refresh the clear color every frame so a theme
                    // toggle takes effect within one paint.
                    let next_clear: (f64, f64, f64) =
                        game_3d_canvas_clear_color(RAYTRACE_WEBGL_CANVAS_SELECTOR);
                    if clear_color_for_loop.get() != next_clear {
                        clear_color_for_loop.set(next_clear);
                    }
                    let (r, g, b) = clear_color_for_loop.get();
                    renderer.render_frame(&program_for_loop, (r, g, b, 1.0), 3);
                }
            }
            frame_clone.set(frame_clone.get() + 1);
            fps_clone.set(fps_clone.get() + frame_time);
            if fps_clone.get() >= 1.0 {
                let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
                loop_state.get_fps().set(fps);
                frame_clone.set(0);
                fps_clone.set(0.0);
            }
            let Some(raf_closure_ref): Option<&'static Closure<dyn FnMut()>> = cell_clone.try_get()
            else {
                return;
            };
            let next_id: i32 = window_value
                .request_animation_frame(raf_closure_ref.as_ref().unchecked_ref())
                .unwrap_or_default();
            if cancelled_for_loop.get() {
                raf_clone.set(None);
            } else {
                raf_clone.set(Some(next_id));
            }
        }));
        let _: Result<(), _> = closure_cell.try_set(raf_closure);
        let Some(start_window): Option<Window> = window() else {
            return;
        };
        let Some(start_raf_ref): Option<&'static Closure<dyn FnMut()>> = closure_cell.try_get()
        else {
            return;
        };
        let start_id: i32 = start_window
            .request_animation_frame(start_raf_ref.as_ref().unchecked_ref())
            .unwrap_or_default();
        raf_id.set(Some(start_id));
    });
}

/// Starts the RayTrace WebGPU loop driven by `requestAnimationFrame`.
///
/// Renders the same scene as the Canvas 2D tab through a WGSL fragment
/// shader on a fullscreen triangle, fed by a single 8-`vec4` uniform
/// buffer at `@group(0) @binding(0)` updated once per frame. WebGPU
/// initialization is asynchronous (adapter + device promises raced
/// against a timeout inside the engine), so the whole init runs inside
/// `spawn_local` with a cancellation guard for tab switches. On failure
/// the error code is surfaced to the status banner and the loop exits
/// quietly.
///
/// # Arguments
///
/// - `UseRayTraceWebGpu` - The WebGPU backend state for signal updates.
/// - `RayTraceCameraAngles` - The shared non-reactive camera orbit angles.
pub(crate) fn start_raytrace_webgpu_loop(state: UseRayTraceWebGpu, angles: RayTraceCameraAngles) {
    let init_state: UseRayTraceWebGpu = state;
    let loop_state: UseRayTraceWebGpu = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let renderer_rc: Rc<RefCell<Option<WebGpuRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let observer_cell: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));
    raytrace_register_resize_debounce(resize_dirty.clone(), resize_timer.clone());
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let renderer_for_cleanup: Rc<RefCell<Option<WebGpuRenderer>>> = renderer_rc.clone();
    let resize_timer_for_cleanup: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let cancelled_for_cleanup: Rc<Cell<bool>> = cancelled.clone();
    let observer_for_cleanup: Rc<RefCell<Option<ResizeObserver>>> = observer_cell.clone();
    App::use_cleanup(move || {
        cancelled_for_cleanup.set(true);
        // Every step is independent: a missing `window()` must never skip
        // the renderer teardown below, so no early returns here.
        if let Some(cancel_id) = raf_for_cleanup.get()
            && let Some(window_value) = window()
        {
            let _ = window_value.cancel_animation_frame(cancel_id);
        }
        if let Some(timer_id) = resize_timer_for_cleanup.get()
            && let Some(window_value) = window()
        {
            window_value.clear_timeout_with_handle(timer_id);
        }
        // Disconnect the ResizeObserver so its closure (and the renderer
        // it holds via `renderer_for_observer`) is released on tab
        // switch. Without this the observer keeps the renderer alive
        // past the loop's lifetime, holding GPU resources until GC.
        if let Some(observer) = observer_for_cleanup.borrow_mut().take() {
            observer.disconnect();
        }
        let _: Option<_> = cell_for_cleanup.try_take();
        // Release GPU resources before dropping the renderer so the
        // device and swap chain are freed eagerly. Without this the
        // old GPU device can linger until GC, causing a fresh
        // WebGpuRenderer::init() either to reuse the dead device
        // (silent black canvas) or to fail to acquire a new one.
        if let Some(renderer) = renderer_for_cleanup.borrow_mut().take() {
            renderer.dispose();
        }
    });
    let cancelled_for_init: Rc<Cell<bool>> = cancelled.clone();
    let Some(loading_window): Option<Window> = window() else {
        return;
    };
    let loading_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        draw_game_3d_loading(
            RAYTRACE_WEBGPU_LOADING_CANVAS_SELECTOR,
            RAYTRACE_WEBGPU_CANVAS_SELECTOR,
        );
    }));
    let loading_callback: Function = loading_closure.as_ref().unchecked_ref::<Function>().clone();
    loading_closure.forget();
    let _ =
        loading_window.set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
    spawn_local(async move {
        let config: RenderConfig = RenderConfig::webgpu(
            RAYTRACE_WEBGPU_CANVAS_SELECTOR,
            RAYTRACE_WIDTH,
            RAYTRACE_HEIGHT,
        );
        let renderer: Result<WebGpuRenderer, WebGpuInitError> =
            Engine::webgpu_renderer(&config).await;
        if cancelled_for_init.get() {
            // The tab was switched away while the adapter / device promises
            // were in flight. A successfully-created renderer still owns a
            // live GPU device at this point; destroy it eagerly instead of
            // leaving it for GC to discover.
            if let Ok(stale_renderer) = &renderer {
                stale_renderer.dispose();
            }
            return;
        }
        let renderer: WebGpuRenderer = match renderer {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!(
                    "[euv-engine][raytrace] webgpu init failed: {error}"
                ));
                init_state.get_init_error_code().set(error.code());
                init_state.get_loaded().set(true);
                return;
            }
        };
        let pipeline: JsValue = renderer.create_render_pipeline(RAYTRACE_WEBGPU_SHADER);
        let uniform_buffer: JsValue =
            renderer.create_uniform_buffer(&[0.0; RAYTRACE_GPU_UNIFORM_VEC4_COUNT * 4]);
        let bind_group: JsValue = renderer.create_uniform_bind_group(&pipeline, &uniform_buffer);
        let clear_color: Rc<Cell<(f64, f64, f64)>> = Rc::new(Cell::new(
            game_3d_canvas_clear_color(RAYTRACE_WEBGPU_CANVAS_SELECTOR),
        ));
        init_state.get_active().set(true);
        // Delay flipping `loaded` so the loading overlay stays painted for a
        // minimum visible duration even when init completes instantly.
        raytrace_set_loaded_delayed(init_state.get_loaded(), GAME_3D_LOADING_MIN_MILLIS);
        *renderer_rc.borrow_mut() = Some(renderer);
        let pipeline_rc: Rc<JsValue> = Rc::new(pipeline);
        let buffer_rc: Rc<JsValue> = Rc::new(uniform_buffer);
        let bind_group_rc: Rc<JsValue> = Rc::new(bind_group);
        // Synchronous resize on CSS-box change; `canvas.width` is applied
        // BEFORE `renderer.resize(...)` so the first paint after a
        // fullscreen transition already has a correctly-sized backing
        // store (mirrors the 3D game page's WebGPU tab).
        let renderer_for_observer: Rc<RefCell<Option<WebGpuRenderer>>> = renderer_rc.clone();
        let observer_closure: Closure<dyn FnMut(js_sys::Array, ResizeObserver)> = Closure::wrap(
            Box::new(move |_entries: js_sys::Array, _obs: ResizeObserver| {
                let Some(window_value): Option<Window> = window() else {
                    return;
                };
                let Some(document_value): Option<Document> = window_value.document() else {
                    return;
                };
                let Some(element): Option<Element> = document_value
                    .query_selector(RAYTRACE_WEBGPU_CANVAS_SELECTOR)
                    .ok()
                    .flatten()
                else {
                    return;
                };
                let canvas: HtmlCanvasElement = element.unchecked_into();
                let rect: DomRect = canvas.get_bounding_client_rect();
                let css_w: f64 = rect.width();
                let css_h: f64 = rect.height();
                if css_w <= 0.0 || css_h <= 0.0 {
                    return;
                }
                let dpr: f64 = Reflect::get(
                    window_value.as_ref(),
                    &JsValue::from_str("devicePixelRatio"),
                )
                .ok()
                .and_then(|v: JsValue| v.as_f64())
                .filter(|v: &f64| v.is_finite() && *v >= 1.0)
                .unwrap_or(1.0);
                let new_w: u32 = (css_w * dpr).round() as u32;
                let new_h: u32 = (css_h * dpr).round() as u32;
                let backing_w: u32 = canvas.width();
                let backing_h: u32 = canvas.height();
                if backing_w != new_w || backing_h != new_h {
                    canvas.set_width(new_w);
                    canvas.set_height(new_h);
                    if let Some(renderer) = renderer_for_observer.borrow_mut().as_mut() {
                        renderer.resize(new_w, new_h);
                    }
                }
            }),
        );
        let observer_callback: Function = observer_closure
            .as_ref()
            .unchecked_ref::<Function>()
            .clone();
        observer_closure.forget();
        if let Ok(resize_observer) = ResizeObserver::new(&observer_callback)
            && let Some(window_value) = window()
            && let Some(document_value) = window_value.document()
            && let Some(element) = document_value
                .query_selector(RAYTRACE_WEBGPU_CANVAS_SELECTOR)
                .ok()
                .flatten()
        {
            resize_observer.observe(&element);
            *observer_cell.borrow_mut() = Some(resize_observer);
        }
        let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
        let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        let renderer_for_loop: Rc<RefCell<Option<WebGpuRenderer>>> = renderer_rc.clone();
        let pipeline_for_loop: Rc<JsValue> = pipeline_rc.clone();
        let buffer_for_loop: Rc<JsValue> = buffer_rc.clone();
        let bind_group_for_loop: Rc<JsValue> = bind_group_rc.clone();
        let clear_color_for_loop: Rc<Cell<(f64, f64, f64)>> = clear_color.clone();
        let yaw_for_loop: Rc<Cell<f64>> = angles.yaw.clone();
        let pitch_for_loop: Rc<Cell<f64>> = angles.pitch.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let resize_dirty_for_loop: Rc<Cell<bool>> = resize_dirty.clone();
        let cancelled_for_loop: Rc<Cell<bool>> = cancelled.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            // Stop on tab-switch cleanup (`cancelled`) or when the canvas
            // left the document (router navigation fires no cleanup).
            if cancelled_for_loop.get() || raytrace_canvas_detached(RAYTRACE_WEBGPU_CANVAS_SELECTOR)
            {
                return;
            }
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            let Some(performance): Option<Performance> = window_value.performance() else {
                return;
            };
            let current_time: f64 = performance.now() / 1000.0;
            let prev: f64 = last_clone.get();
            // Unclamped frame time feeds the FPS counter (honest rate);
            // the clamped delta only drives the yaw animation step.
            let frame_time: f64 = if prev < 0.0 {
                1.0 / 60.0
            } else {
                current_time - prev
            };
            let anim_time: f64 = frame_time.min(0.25);
            last_clone.set(current_time);
            if loop_state.get_auto_rotate().get() {
                yaw_for_loop.set(yaw_for_loop.get() + RAYTRACE_AUTO_YAW_SPEED * anim_time);
            }
            let resize_dirty_frame: bool = if resize_dirty_for_loop.get() {
                resize_dirty_for_loop.set(false);
                true
            } else {
                false
            };
            let Some(window_for_dpr): Option<Window> = window() else {
                return;
            };
            let dpr: f64 = Reflect::get(
                window_for_dpr.as_ref(),
                &JsValue::from_str("devicePixelRatio"),
            )
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .filter(|value: &f64| value.is_finite() && *value >= 1.0)
            .unwrap_or(1.0);
            let (canvas_width, canvas_height): (f64, f64) =
                read_raytrace_canvas_size(RAYTRACE_WEBGPU_CANVAS_SELECTOR).unwrap_or((0.0, 0.0));
            let new_physical_width: u32 = (canvas_width * dpr).round() as u32;
            let new_physical_height: u32 = (canvas_height * dpr).round() as u32;
            // Borrow the renderer exactly once for the entire frame via
            // `borrow_mut().as_mut()` so the RefMut guard releases
            // automatically when this block exits, avoiding the
            // `RefCell already borrowed` panic a second borrow would hit.
            if let Some(renderer) = renderer_for_loop.borrow_mut().as_mut() {
                if new_physical_width > 0 && new_physical_height > 0 {
                    let backing_w: u32 = renderer.get_canvas().width();
                    let backing_h: u32 = renderer.get_canvas().height();
                    if backing_w != new_physical_width || backing_h != new_physical_height {
                        renderer.get_canvas().set_width(new_physical_width);
                        renderer.get_canvas().set_height(new_physical_height);
                        let _ = renderer.resize(new_physical_width, new_physical_height);
                    }
                }
                if resize_dirty_frame {
                    let _ = renderer.resize(new_physical_width, new_physical_height);
                }
                if loop_state.get_running().get() {
                    let backing_w: f64 = f64::from(renderer.get_canvas().width());
                    let backing_h: f64 = f64::from(renderer.get_canvas().height());
                    let uniform_data: Vec<f32> = pack_raytrace_gpu_uniform(
                        yaw_for_loop.get(),
                        pitch_for_loop.get(),
                        backing_w,
                        backing_h,
                    );
                    renderer.update_uniform_buffer(&buffer_for_loop, &uniform_data);
                    // Refresh the clear color every frame so a theme
                    // toggle takes effect within one paint.
                    let next_clear: (f64, f64, f64) =
                        game_3d_canvas_clear_color(RAYTRACE_WEBGPU_CANVAS_SELECTOR);
                    if clear_color_for_loop.get() != next_clear {
                        clear_color_for_loop.set(next_clear);
                    }
                    let (r, g, b) = clear_color_for_loop.get();
                    renderer.render_frame_with_bind_group(
                        &pipeline_for_loop,
                        &bind_group_for_loop,
                        (r, g, b, 1.0),
                        3,
                    );
                }
            }
            frame_clone.set(frame_clone.get() + 1);
            fps_clone.set(fps_clone.get() + frame_time);
            if fps_clone.get() >= 1.0 {
                let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
                loop_state.get_fps().set(fps);
                frame_clone.set(0);
                fps_clone.set(0.0);
            }
            let Some(raf_closure_ref): Option<&'static Closure<dyn FnMut()>> = cell_clone.try_get()
            else {
                return;
            };
            let next_id: i32 = window_value
                .request_animation_frame(raf_closure_ref.as_ref().unchecked_ref())
                .unwrap_or_default();
            if cancelled_for_loop.get() {
                raf_clone.set(None);
            } else {
                raf_clone.set(Some(next_id));
            }
        }));
        let _: Result<(), _> = closure_cell.try_set(raf_closure);
        let Some(start_window): Option<Window> = window() else {
            return;
        };
        let Some(start_raf_ref): Option<&'static Closure<dyn FnMut()>> = closure_cell.try_get()
        else {
            return;
        };
        let start_id: i32 = start_window
            .request_animation_frame(start_raf_ref.as_ref().unchecked_ref())
            .unwrap_or_default();
        raf_id.set(Some(start_id));
    });
}

/// Creates a click event handler that sets the active tab and exits
/// any in-flight landscape fullscreen mode before switching.
///
/// Tab switches destroy the previous arm's DOM subtree (the match
/// expression rebuilds from scratch on arm change), so any tab's
/// `c_game_container_fullscreen` overlay is unmounted along with the
/// rest of that arm. The per-tab fullscreen signals are page-scoped
/// `Signal<bool>` instances, however — they survive arm destruction
/// because they are registered with the page-level HookContext, not
/// the per-arm one. Without explicit cleanup the next time the user
/// revisits that tab the overlay re-mounts even though they did not
/// press Enter Fullscreen again. Clearing all three signals on every
/// tab change keeps fullscreen state strictly co-extensive with the
/// user's last explicit enter/exit action.
///
/// # Arguments
///
/// - `Signal<RayTraceTab>` - The tab signal to update.
/// - `RayTraceTab` - The tab variant to set.
/// - `UseRayTraceFullscreen` - The fullscreen state to clear on switch.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the active
///   tab and clears any active fullscreen mode.
pub(crate) fn raytrace_on_tab_select(
    tab: Signal<RayTraceTab>,
    value: RayTraceTab,
    fullscreen: UseRayTraceFullscreen,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        fullscreen.get_canvas_2d().set(false);
        fullscreen.get_web_gl().set(false);
        fullscreen.get_web_gpu().set(false);
        tab.set(value);
    }))
}

/// Enters landscape fullscreen mode for the RayTrace page on the active
/// tab.
///
/// Sets the tab-specific fullscreen signal, pushes a history entry so
/// the system back button can exit, and re-applies safe-area insets to
/// the newly-mounted overlay container. A synthetic window `resize`
/// event is dispatched so the WebGL / WebGPU loops resize their backing
/// stores to the new CSS box; the Canvas 2D tab keeps its fixed 4:3
/// backing and relies on `object-fit: contain` letterboxing, so it
/// ignores the event.
///
/// # Arguments
///
/// - `Signal<bool>` - The fullscreen signal for the active tab.
pub(crate) fn enter_raytrace_fullscreen(tab: Signal<bool>) {
    tab.set(true);
    Router::overlay_push_state();
    UseEuvLayout::apply_cached_insets();
    // Dispatch a `resize` event on the window so the GPU-backed loops'
    // `App::use_window_event("resize", ...)` handlers fire and their
    // `resize_dirty` flags are set. Mirrors
    // `game_3d/hook/fn.rs::enter_game_3d_fullscreen`.
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let event: Result<Event, JsValue> = Event::new("resize");
    if let Ok(event) = event {
        let _ = window_value.dispatch_event(&event);
    }
}

/// Exits landscape fullscreen mode for the RayTrace page on the active
/// tab.
///
/// Used by the in-overlay Exit button. Clears the fullscreen signal and
/// re-applies the safe-area insets. The `history.back()` call inside
/// `Router::overlay_back` consumes the browser history entry that was
/// pushed on enter.
///
/// # Arguments
///
/// - `Signal<bool>` - The fullscreen signal for the active tab.
pub(crate) fn exit_raytrace_fullscreen(tab: Signal<bool>) {
    tab.set(false);
    UseEuvLayout::apply_cached_insets();
    // See `enter_raytrace_fullscreen` - dispatch a synthetic `resize`
    // event so the GPU-backed loops re-acquire the inline canvas
    // dimensions.
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let event: Result<Event, JsValue> = Event::new("resize");
    if let Ok(event) = event {
        let _ = window_value.dispatch_event(&event);
    }
}

/// Exits landscape fullscreen mode without consuming a browser history
/// entry.
///
/// Used when the exit is triggered by the system back button: the
/// `popstate` event itself has already consumed the `pushState` entry
/// that was created when entering fullscreen, so calling
/// `history.back()` again would over-consume the history stack.
///
/// # Arguments
///
/// - `Signal<bool>` - The fullscreen signal for the active tab.
pub(crate) fn exit_raytrace_fullscreen_from_popstate(tab: Signal<bool>) {
    tab.set(false);
    UseEuvLayout::apply_cached_insets();
    // See `enter_raytrace_fullscreen` for why we dispatch a synthetic
    // `resize` event here.
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let event: Result<Event, JsValue> = Event::new("resize");
    if let Ok(event) = event {
        let _ = window_value.dispatch_event(&event);
    }
}

/// Subscribes to browser `popstate` events to handle the system back
/// button while the RayTrace page is in landscape fullscreen mode.
///
/// Watches all three tab-specific fullscreen signals in a fixed order
/// (Canvas 2D, WebGL, WebGPU). When any one is `true`, the corresponding
/// `exit_raytrace_fullscreen_from_popstate` runs and the guard returns
/// `true` to consume the `popstate` event. Otherwise returns `false` so
/// the overlay stack or router can handle the back navigation normally.
///
/// Returns the guard ID so the page can unregister it on unmount.
///
/// # Arguments
///
/// - `UseRayTraceFullscreen` - The RayTrace page fullscreen state.
///
/// # Returns
///
/// - `usize` - The popstate guard ID.
pub(crate) fn use_raytrace_fullscreen_popstate(state: UseRayTraceFullscreen) -> usize {
    Router::register_popstate_guard(Rc::new(move || {
        if state.get_canvas_2d().get() {
            exit_raytrace_fullscreen_from_popstate(state.get_canvas_2d());
            true
        } else if state.get_web_gl().get() {
            exit_raytrace_fullscreen_from_popstate(state.get_web_gl());
            true
        } else if state.get_web_gpu().get() {
            exit_raytrace_fullscreen_from_popstate(state.get_web_gpu());
            true
        } else {
            false
        }
    }))
}

/// Creates a click event handler that enters landscape fullscreen mode
/// for the RayTrace page.
///
/// Delegates to [`enter_raytrace_fullscreen`], which sets the active
/// tab's fullscreen signal, pushes a history entry, and reapplies
/// safe-area insets to the newly-mounted overlay container. The canvas
/// itself is not recreated — the running loop, FPS counter, and pause
/// state all survive the transition.
///
/// # Arguments
///
/// - `Signal<bool>` - The fullscreen signal for the active tab.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn raytrace_on_enter_fullscreen(tab: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        enter_raytrace_fullscreen(tab);
    }))
}

/// Creates a click event handler that exits landscape fullscreen mode
/// for the RayTrace page.
///
/// Delegates to [`exit_raytrace_fullscreen`], which clears the active
/// tab's fullscreen signal and reapplies safe-area insets. The
/// `history.back()` call inside `Router::overlay_back` consumes the
/// browser history entry that was pushed on enter.
///
/// # Arguments
///
/// - `Signal<bool>` - The fullscreen signal for the active tab.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn raytrace_on_exit_fullscreen(tab: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        exit_raytrace_fullscreen(tab);
        Router::overlay_back(None);
    }))
}
