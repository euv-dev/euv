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
        // `loaded` is `false` while the backend is being acquired and the
        // first frame is being rendered. The view paints a
        // `c_game_loading_overlay` canvas on top of the raytrace canvas for
        // exactly this window, so the user sees a centered "Initializing..."
        // line instead of a half-rendered frame. The loop flips it to
        // `true` after the first successful render, matching the WebGL and
        // WebGPU tabs' loading UX.
        loaded: App::use_signal(|| false),
        // `active` mirrors the WebGL / WebGPU tabs: `true` only once a
        // frame has actually been rendered through the engine backend.
        // The view reads it, so a rejected frame cannot present itself as a
        // healthy renderer.
        active: App::use_signal(|| false),
        error_detail: App::use_signal(String::new),
        init_error_code: App::use_signal(|| ""),
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
        error_detail: App::use_signal(String::new),
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
        error_detail: App::use_signal(String::new),
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
/// page or tab was navigated away from and the loop should stop.
///
/// Hook-context cleanups (`App::use_cleanup`) only run on match-arm
/// switches, not on router navigation, so RAF loops additionally guard on
/// canvas presence to avoid rendering against a detached canvas forever.
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
/// `clientWidth`/`clientHeight`, which track the backing store in Chrome and
/// would create a feedback loop if read every frame). Page-scoped copy of
/// the game-page helper: the name stays private so it cannot collide with
/// the `game_2d` / `game_3d` `read_canvas_size` re-exports.
///
/// # Arguments
///
/// - `&str` - The CSS selector for the canvas element.
///
/// # Returns
///
/// - `Option<(f64, f64)>` - The `(width, height)` in CSS pixels.
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

/// Resolves a RayTrace canvas element from its CSS selector.
///
/// # Arguments
///
/// - `&str` - The CSS selector for the canvas element.
///
/// # Returns
///
/// - `Option<HtmlCanvasElement>` - The canvas, or `None` when it is absent
///   or no longer in the document.
fn query_raytrace_canvas(canvas_selector: &str) -> Option<HtmlCanvasElement> {
    let window_value: Window = window()?;
    let document_value: Document = window_value.document()?;
    let element: Element = document_value
        .query_selector(canvas_selector)
        .ok()
        .flatten()?;
    Some(element.unchecked_into())
}

/// Builds the shared ray-tracing scene used by all three RayTrace tabs.
///
/// The scene is deliberately **static**: no light rotates with the camera,
/// and no occluder is placed along a light's own direction. Every backend
/// therefore uploads it exactly once, at `RayTraceRenderer::init`, and no
/// steady-state frame pays a scene repack. It mixes all three primitive
/// kinds the engine supports so the shared-data contract is visible rather
/// than claimed:
///
/// - **ground AABB** — a wide Lambert plate that catches the directional
///   sun's shadow.
/// - **mirror sphere** — a Phong material whose specular term drives the
///   secondary bounces, so `max_bounces` is doing visible work.
/// - **emissive sphere** — a standalone `Material::emissive` body. It is
///   an *object that emits*, not a lamp representation: the new shading
///   path casts real shadow rays, so putting a point light at its centre
///   would have the sphere occlude its own light and black out the whole
///   scene. The demo therefore keeps it purely emissive and places every
///   actual light where nothing blocks it.
/// - **mirror triangle** — a two-sided triangle, present so the scene
///   carries a primitive kind that is neither sphere nor box.
///
/// # Returns
///
/// - `RayTraceScene` - The shared demo scene.
pub(crate) fn build_raytrace_scene() -> RayTraceScene {
    let ground_material: Material = Material::lambert(Vector3D::new(0.62, 0.63, 0.66));
    let ground: Occluder = Occluder::aabb(
        Vector3D::new(-6.0, -0.62, -6.0),
        Vector3D::new(6.0, -0.5, 6.0),
        ground_material,
    );
    let mirror_material: Material = Material::phong(Vector3D::new(0.86, 0.87, 0.9), 1.0, 96.0);
    let mirror: Occluder = Occluder::sphere(Vector3D::new(-0.9, 0.35, 0.2), 0.85, mirror_material);
    let emissive_material: Material = Material::emissive(Vector3D::new(1.0, 0.42, 0.12));
    let emissive: Occluder =
        Occluder::sphere(Vector3D::new(1.75, 0.55, -1.35), 0.42, emissive_material);
    let panel_material: Material = Material::phong(Vector3D::new(0.24, 0.46, 0.62), 0.9, 48.0);
    let panel: Occluder = Occluder::triangle(
        Vector3D::new(-2.7, -0.5, -2.3),
        Vector3D::new(0.4, 2.55, -3.6),
        Vector3D::new(2.3, -0.5, -1.15),
        panel_material,
    );
    let occluders: Vec<Occluder> = vec![ground, mirror, emissive, panel];
    RayTraceScene::new(occluders)
}

/// Builds the shared, static lighting for the RayTrace scene.
///
/// Every light sits in open space with a clear line to the geometry it is
/// meant to light, which matters more here than in the previous demo:
/// shading now casts exact shadow rays through the same BVH the primary
/// rays traverse, so a light hidden behind (or inside) an occluder casts
/// real, hard shadows instead of the old bounding-sphere approximation.
///
/// - **directional sun** — points down and toward the scene from open sky.
///   Directional lights have no position, so nothing can occlude them.
/// - **point fill** — hangs above and behind the mirror sphere, in free
///   space, giving the shadowed side of the scene a visible gradient.
/// - **spot rim** — aims at the emissive sphere from outside the scene to
///   make the cone term observable.
///
/// The set is built once and cloned by reference thereafter: because the
/// lights never move, no frame rebuilds it, so the packed light records
/// are byte-identical for the life of the loop.
///
/// # Returns
///
/// - `LightingUniforms` - The static demo lighting.
pub(crate) fn build_raytrace_lighting() -> LightingUniforms {
    let eye: Vector3D = Vector3D::new(
        RAYTRACE_SHADING_EYE_X,
        RAYTRACE_SHADING_EYE_Y,
        RAYTRACE_SHADING_EYE_Z,
    );
    let sun: Light = Light::new_directional(
        Vector3D::new(0.42, -0.86, 0.3).normalized(),
        Vector3D::new(1.0, 0.95, 0.84),
    );
    let fill: Light = Light::new_point(
        Vector3D::new(-2.6, 2.9, 2.4),
        Vector3D::new(0.42, 0.52, 0.78),
        2.6,
    );
    let rim: Light = Light::new_spot(
        Vector3D::new(3.1, 2.35, 1.75),
        (Vector3D::new(1.75, 0.55, -1.35) - Vector3D::new(3.1, 2.35, 1.75)).normalized(),
        Vector3D::new(0.78, 0.86, 1.0),
        3.2,
        0.62,
    );
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::new(0.09, 0.1, 0.14));
    lights.add_light(sun);
    lights.add_light(fill);
    lights.add_light(rim);
    lights
}

/// Builds the `RayTraceSettings` every tab renders with.
///
/// # Returns
///
/// - `RayTraceSettings` - The shared sampling and bounce settings.
pub(crate) fn build_raytrace_settings() -> RayTraceSettings {
    RayTraceSettings::with_quality(RAYTRACE_SAMPLES_PER_AXIS, RAYTRACE_MAX_BOUNCES)
}

/// Computes the eye position for the given orbit yaw / pitch angles.
///
/// Mirrors the orbit-to-eye conversion the 3D game page uses: the camera
/// sits on a sphere of radius [`RAYTRACE_CAMERA_DISTANCE`] centred on the
/// scene's look-at target, so dragging horizontally rotates around the
/// scene and dragging vertically tilts up / down.
///
/// # Arguments
///
/// - `f64` - The orbit yaw in radians.
/// - `f64` - The orbit pitch in radians.
///
/// # Returns
///
/// - `Vector3D` - The eye position in world space.
pub(crate) fn compute_eye_position(yaw: f64, pitch: f64) -> Vector3D {
    let cos_pitch: f64 = pitch.cos();
    let distance: f64 = RAYTRACE_CAMERA_DISTANCE;
    Vector3D::new(
        RAYTRACE_CAMERA_LOOK_AT_X + distance * yaw.sin() * cos_pitch,
        RAYTRACE_CAMERA_LOOK_AT_Y + distance * pitch.sin(),
        RAYTRACE_CAMERA_LOOK_AT_Z + distance * yaw.cos() * cos_pitch,
    )
}

/// Builds the orbit camera for a given yaw / pitch and frame size.
///
/// The engine's `pack_raytrace_frame` derives the view basis from
/// `position` / `target` / `up` and rejects a degenerate one, so the demo
/// supplies all three and never a precomputed basis of its own. The
/// viewport is the *frame* size, not the CSS size: the engine derives the
/// aspect from the width and height it is asked to render, which is what
/// keeps the three backends from disagreeing about aspect while they
/// resize independently.
///
/// # Arguments
///
/// - `f64` - The orbit yaw in radians.
/// - `f64` - The orbit pitch in radians.
/// - `u32` - The frame width in pixels.
/// - `u32` - The frame height in pixels.
///
/// # Returns
///
/// - `Camera3D` - The orbit camera for this frame.
pub(crate) fn build_raytrace_camera(yaw: f64, pitch: f64, width: u32, height: u32) -> Camera3D {
    Camera3D::create(
        compute_eye_position(yaw, pitch),
        Vector3D::new(
            RAYTRACE_CAMERA_LOOK_AT_X,
            RAYTRACE_CAMERA_LOOK_AT_Y,
            RAYTRACE_CAMERA_LOOK_AT_Z,
        ),
        f64::from(width),
        f64::from(height),
    )
}

/// Reads the backing-store width of a tab's canvas.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the tab's canvas.
///
/// # Returns
///
/// - `u32` - The backing width in pixels, or `1` when the canvas cannot be
///   found. A `1`-pixel frame is rejected by the engine, so a vanished
///   canvas surfaces as a real error rather than a silently empty frame.
pub(crate) fn raytrace_canvas_backing_width(canvas_selector: &str) -> u32 {
    query_raytrace_canvas(canvas_selector)
        .map(|canvas: HtmlCanvasElement| canvas.width())
        .unwrap_or(1)
}

/// Reads the backing-store height of a tab's canvas.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the tab's canvas.
///
/// # Returns
///
/// - `u32` - The backing height in pixels, or `1` when the canvas cannot be
///   found.
pub(crate) fn raytrace_canvas_backing_height(canvas_selector: &str) -> u32 {
    query_raytrace_canvas(canvas_selector)
        .map(|canvas: HtmlCanvasElement| canvas.height())
        .unwrap_or(1)
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
pub(crate) fn raytrace_scaled_dimensions(scale: f64) -> (u32, u32) {
    let width: u32 = (RAYTRACE_WIDTH * scale).round().max(1.0) as u32;
    let height: u32 = (RAYTRACE_HEIGHT * scale).round().max(1.0) as u32;
    (width, height)
}

/// Extracts a human-readable detail string out of a rejected engine call.
///
/// A `JsValue` error from the backend is usually a string, but a thrown
/// object or a number is equally legal, so anything that is not a string
/// falls back to a fixed placeholder instead of being rendered as
/// `undefined` on the page.
///
/// # Arguments
///
/// - `&JsValue` - The rejected error value.
///
/// # Returns
///
/// - `String` - The detail text to log and display.
pub(crate) fn raytrace_error_detail(error: &JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| RAYTRACE_ERROR_DETAIL_UNKNOWN.to_string())
}

/// Records a rejected engine call on the tab's reactive state.
///
/// Every failure path routes through here, which is what keeps the
/// status banner honest: `active` is only ever set after a frame has
/// actually rendered, and it is cleared again the moment a frame starts
/// failing, so a dead renderer can never keep reporting itself as live.
///
/// # Arguments
///
/// - `Signal<bool>` - The tab's `active` flag.
/// - `Signal<String>` - The tab's error detail signal.
/// - `Signal<&'static str>` - The tab's stable error code signal.
/// - `&'static str` - The stable error code for this call site.
/// - `&str` - A short label naming the call that failed.
/// - `&JsValue` - The rejected error value.
pub(crate) fn raytrace_record_error(
    active: Signal<bool>,
    error_detail: Signal<String>,
    init_error_code: Signal<&'static str>,
    code: &'static str,
    call: &str,
    error: &JsValue,
) {
    let detail: String = raytrace_error_detail(error);
    Console::error(format!(
        "{RAYTRACE_LOG_PREFIX} {call} failed [{code}]: {detail}"
    ));
    error_detail.set(detail);
    init_error_code.set(code);
    active.set(false);
}

/// Clears the tab's error state after a successful frame.
///
/// # Arguments
///
/// - `Signal<bool>` - The tab's `active` flag.
/// - `Signal<String>` - The tab's error detail signal.
/// - `Signal<&'static str>` - The tab's stable error code signal.
pub(crate) fn raytrace_clear_error(
    active: Signal<bool>,
    error_detail: Signal<String>,
    init_error_code: Signal<&'static str>,
) {
    error_detail.set(String::new());
    init_error_code.set("");
    active.set(true);
}

/// Schedules a one-shot `setTimeout` whose Rust closure is owned rather
/// than leaked with `Closure::forget`.
///
/// Every entry is a `(timer handle, closure)` pair so
/// [`raytrace_release_pending_timers`] can cancel the browser timer
/// *before* dropping the closure: a timer whose Rust environment has
/// already been deallocated throws `closure invoked recursively or after
/// being dropped` at the JavaScript boundary, before any Rust code could
/// observe the teardown flag. The pairing, not the leak, is what makes
/// the release order safe.
///
/// # Arguments
///
/// - `RayTracePendingTimers` - The owner list
///   shared with the loop's teardown.
/// - `i32` - The delay in milliseconds before the body runs.
/// - `F` - The body to run once the timer fires.
fn raytrace_schedule_one_shot<F>(pending_timers: RayTracePendingTimers, millis: i32, body: F)
where
    F: FnMut() + 'static,
{
    let timer_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(body));
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let timer_callback: Function = timer_closure.as_ref().unchecked_ref::<Function>().clone();
    match window_value
        .set_timeout_with_callback_and_timeout_and_arguments_0(&timer_callback, millis)
    {
        Ok(timer_id) => {
            let mut pending_guard: RayTracePendingTimerGuard = match pending_timers.try_borrow_mut()
            {
                Ok(guard) => guard,
                Err(_contended) => return,
            };
            pending_guard.push((timer_id, timer_closure));
        }
        Err(_reason) => {}
    }
}

/// Cancels every pending one-shot timer of a loop and then drops the
/// Rust closures behind them.
///
/// The ids are collected before the owner list is emptied, and each
/// `clear_timeout` runs while no borrow of the owner list is alive, so
/// no re-entrant call can observe a contended `RefCell`.
///
/// # Arguments
///
/// - `RayTracePendingTimers` - The owner list
///   shared with the loop.
fn raytrace_release_pending_timers(pending_timers: RayTracePendingTimers) {
    let timer_ids: Vec<i32> = pending_timers
        .borrow()
        .iter()
        .map(|entry: &(i32, Closure<dyn FnMut()>)| entry.0)
        .collect();
    for timer_id in timer_ids {
        let Some(window_value): Option<Window> = window() else {
            return;
        };
        window_value.clear_timeout_with_handle(timer_id);
    }
    let mut pending_guard: RayTracePendingTimerGuard = match pending_timers.try_borrow_mut() {
        Ok(guard) => guard,
        Err(_contended) => return,
    };
    pending_guard.clear();
}

/// Sets a tab's `loaded` signal after a short delay so the loading
/// overlay is actually painted before it is removed.
///
/// `init` resolves well inside a single frame for a warm backend, and most
/// browsers collapse a mount + unmount that happens within one tick into a
/// single paint, so the overlay would never be seen. Delaying the flip
/// guarantees every tab shows the same "Initializing..." window.
///
/// # Arguments
///
/// - `RayTracePendingTimers` - The owner list
///   the timer is registered in.
/// - `Signal<bool>` - The tab's `loaded` signal to set.
/// - `i32` - The delay in milliseconds before setting the signal.
fn raytrace_set_loaded_delayed(
    pending_timers: RayTracePendingTimers,
    loaded: Signal<bool>,
    millis: i32,
) {
    raytrace_schedule_one_shot(pending_timers, millis, move || {
        loaded.set(true);
    });
}

/// Registers the debounced window-resize listener shared by the GPU
/// backed loops.
///
/// The listener body is registered through `App::use_window_event`, so
/// the framework owns it for exactly as long as the hook context lives
/// and unregisters it on the same cleanup that runs on a tab switch.
/// The `Closure` behind the JavaScript callback the body schedules is
/// therefore handed back as an owner cell for the caller to move into
/// its own cleanup, rather than being leaked with `Closure::forget`.
///
/// It is deliberately *not* released by [`raytrace_release_loop`]: a
/// handler that is still registered can still be invoked, and dropping
/// its callback from under it would turn the next window resize into a
/// thrown exception at the JavaScript boundary. The `cancelled` flag is
/// what makes that surviving handler inert instead.
///
/// # Arguments
///
/// - `Rc<Cell<bool>>` - The resize-dirty flag set after the debounce fires.
/// - `Rc<Cell<Option<i32>>>` - The pending debounce timer handle.
/// - `Rc<Cell<bool>>` - The loop's cancellation flag, checked before the
///   debounce timer is armed.
///
/// # Returns
///
/// - `RayTraceDebounceOwner` - The cell owning the
///   debounce closure.
fn raytrace_register_resize_debounce(
    resize_dirty: Rc<Cell<bool>>,
    resize_timer: Rc<Cell<Option<i32>>>,
    cancelled: Rc<Cell<bool>>,
) -> RayTraceDebounceOwner {
    let debounce_owner: RayTraceDebounceOwner = Rc::new(RefCell::new(None));
    let resize_dirty_for_event: Rc<Cell<bool>> = resize_dirty;
    let resize_timer_for_event: Rc<Cell<Option<i32>>> = resize_timer;
    let cancelled_for_event: Rc<Cell<bool>> = cancelled;
    let debounce_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        resize_dirty_for_event.set(true);
    }));
    let debounce_callback: Function = debounce_closure
        .as_ref()
        .unchecked_ref::<Function>()
        .clone();
    let Some(resize_window): Option<Window> = window() else {
        *debounce_owner.borrow_mut() = Some(debounce_closure);
        return debounce_owner;
    };
    App::use_window_event(RAYTRACE_EVENT_RESIZE, move || {
        if cancelled_for_event.get() {
            return;
        }
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
                RAYTRACE_RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        resize_timer_for_event.set(Some(new_timer));
    });
    *debounce_owner.borrow_mut() = Some(debounce_closure);
    debounce_owner
}

/// Reads the current device pixel ratio, rejecting nonsense values.
///
/// # Returns
///
/// - `f64` - The device pixel ratio, or `1.0` when it cannot be read or
///   is not a finite value of at least 1.
pub(crate) fn read_raytrace_dpr() -> f64 {
    let Some(window_value): Option<Window> = window() else {
        return 1.0;
    };
    Reflect::get(
        window_value.as_ref(),
        &JsValue::from_str(RAYTRACE_EVENT_PROPERTY_DEVICE_PIXEL_RATIO),
    )
    .ok()
    .and_then(|value: JsValue| value.as_f64())
    .filter(|value: &f64| value.is_finite() && *value >= 1.0)
    .unwrap_or(1.0)
}

/// Resizes a tab's engine backend to the canvas's physical size.
///
/// The renderer is the **single** commit point for the size: it commits
/// its own dimensions, the canvas backing store, and its pixel buffer
/// only after the backend has accepted the request, so a rejected size
/// leaves the canvas showing the previous, still valid, surface. This
/// wrapper therefore never touches `canvas.width` / `canvas.height`
/// itself — writing them first would break that transaction, because the
/// canvas would already be showing a surface the renderer never adopted
/// when `resize` returns `Err`.
///
/// The call is skipped entirely when the target already matches the
/// backing store: a steady frame therefore performs no DOM write, no
/// GL / GPU viewport touch, and no reallocation.
///
/// # Arguments
///
/// - `&mut RayTraceRenderer` - The engine backend.
/// - `&str` - The CSS selector of the tab's canvas.
/// - `Signal<bool>` - The tab's `active` flag.
/// - `Signal<String>` - The tab's error detail signal.
/// - `Signal<&'static str>` - The tab's stable error code signal.
///
/// # Returns
///
/// - `bool` - Whether the backend now matches the canvas.
pub(crate) fn raytrace_resize_backend_to_canvas(
    renderer: &mut RayTraceRenderer,
    canvas_selector: &str,
    active: Signal<bool>,
    error_detail: Signal<String>,
    init_error_code: Signal<&'static str>,
) -> bool {
    let Some(canvas): Option<HtmlCanvasElement> = query_raytrace_canvas(canvas_selector) else {
        return false;
    };
    let backing_width: u32 = canvas.width();
    let backing_height: u32 = canvas.height();
    let dpr: f64 = read_raytrace_dpr();
    let (css_width, css_height): (f64, f64) =
        read_raytrace_canvas_size(canvas_selector).unwrap_or((0.0, 0.0));
    if css_width <= 0.0 || css_height <= 0.0 {
        return backing_width > 0 && backing_height > 0;
    }
    let target_width: u32 = (css_width * dpr).round() as u32;
    let target_height: u32 = (css_height * dpr).round() as u32;
    if target_width == 0 || target_height == 0 {
        return backing_width > 0 && backing_height > 0;
    }
    if backing_width == target_width && backing_height == target_height {
        return true;
    }
    match renderer.resize(target_width, target_height) {
        Ok(()) => {
            raytrace_clear_error(active, error_detail, init_error_code);
            true
        }
        Err(error) => {
            raytrace_record_error(
                active,
                error_detail,
                init_error_code,
                RAYTRACE_ERROR_CODE_RESIZE,
                "resize",
                &error,
            );
            false
        }
    }
}

/// Releases every resource a RayTrace loop owns, on both teardown paths.
///
/// `App::use_cleanup` fires on a match-arm switch, but a router
/// navigation mounts a different page without running any cleanup, so a
/// loop whose canvas left the document reaches its own teardown from the
/// animation-frame body instead. Both paths converge on this one
/// function, which is idempotent: every step either checks a handle or
/// takes a value out of its owner, so running it twice releases nothing
/// twice.
///
/// The animation-frame closure is safe to take even when this runs from
/// inside that very closure, which is the case on the router-navigation
/// path. wasm-bindgen 0.2.129 makes `makeMutClosure` bump the closure's
/// own reference count for the duration of the call and release it in a
/// `finally`, so dropping the Rust `Closure` mid-invocation only lowers
/// the count to one and the Rust environment stays alive until the
/// callback returns. The frame body can therefore keep using every cell
/// it captured after this function returns.
///
/// # Arguments
///
/// - `Rc<Cell<bool>>` - The cancellation flag the loop body checks.
/// - `Rc<Cell<Option<i32>>>` - The pending animation-frame handle.
/// - `RafClosureCell` - The cell owning the animation-frame closure.
/// - `RayTracePendingTimers` - The owner list of
///   one-shot timers this loop armed.
/// - `Rc<RefCell<Option<RayTraceRenderer>>>` - The cell owning the engine
///   backend.
fn raytrace_release_loop(
    cancelled: Rc<Cell<bool>>,
    raf_id: Rc<Cell<Option<i32>>>,
    closure_cell: RafClosureCell,
    pending_timers: RayTracePendingTimers,
    renderer_rc: Rc<RefCell<Option<RayTraceRenderer>>>,
) {
    cancelled.set(true);
    if let Some(cancel_id) = raf_id.get()
        && let Some(window_value) = window()
    {
        let _: Result<(), JsValue> = window_value.cancel_animation_frame(cancel_id);
    }
    raf_id.set(None);
    raytrace_release_pending_timers(pending_timers);
    let _: Option<Closure<dyn FnMut()>> = closure_cell.try_take();
    if let Some(mut renderer) = renderer_rc
        .try_borrow_mut()
        .ok()
        .and_then(|mut slot: RefMut<'_, Option<RayTraceRenderer>>| slot.take())
    {
        renderer.dispose();
    }
}

/// Starts the RayTrace Canvas 2D `requestAnimationFrame` loop.
///
/// The loop owns exactly one [`RayTraceRenderer`] built through the
/// engine's `RayTraceRenderer::init` with the requested
/// [`RayTraceBackend::Canvas2D`]. Every frame hands that same backend the
/// orbit camera, the shared lighting, and the shared settings; the scene is
/// uploaded once at `init` and never re-sent, because neither the geometry
/// nor the lights move with the camera.
///
/// An exponential moving average of the measured render time drives the
/// [`RAYTRACE_RENDER_SCALES`] adaptive-resolution ladder: sustained
/// frames above 115% of the 60 FPS budget step the internal resolution
/// down one rung, sustained frames below 75% step it back up, and
/// sustained frames below 45% step it up two rungs at once. The reported
/// FPS uses unclamped wall-clock elapsed time so it stays honest even
/// below 4 FPS; the `0.25s` clamp applies only to the yaw animation step.
///
/// A rung change is a real size change, so it is committed through the
/// same [`raytrace_resize_backend_to_canvas`] path the CSS box uses, and
/// only on the frames where the rung actually moved: the camera is built
/// for the *previous* frame's size, the new size is adopted after that
/// frame renders, and a rejected resize leaves both the backend and the
/// canvas on their old size instead of presenting a mismatch.
///
/// # Arguments
///
/// - `UseRayTrace` - The RayTrace Canvas 2D tab state.
/// - `RayTraceCameraAngles` - The non-reactive camera orbit angles.
pub(crate) fn start_raytrace_loop(state: UseRayTrace, angles: RayTraceCameraAngles) {
    let init_state: UseRayTrace = state;
    let loop_state: UseRayTrace = state;
    let renderer_rc: Rc<RefCell<Option<RayTraceRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let pending_timers: RayTracePendingTimers = Rc::new(RefCell::new(Vec::new()));
    let renderer_for_cleanup: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let cancelled_for_cleanup: Rc<Cell<bool>> = cancelled.clone();
    let timers_for_cleanup: RayTracePendingTimers = pending_timers.clone();
    // `dispose` is idempotent by contract, so calling it on the cleanup
    // path and again on the `Drop` of the dropped renderer cannot
    // double-free: the second call is a no-op on an already-disposed
    // backend. Taking the value out of the cell releases the last Rust
    // reference as well, which is what lets the renderer run its `Drop`.
    App::use_cleanup(move || {
        raytrace_release_loop(
            cancelled_for_cleanup,
            raf_for_cleanup,
            cell_for_cleanup,
            timers_for_cleanup,
            renderer_for_cleanup,
        );
    });
    let cancelled_for_init: Rc<Cell<bool>> = cancelled.clone();
    let pending_timers_for_init: RayTracePendingTimers = pending_timers.clone();
    // Owned by the same pending-timer list as every other one-shot, so
    // teardown drops the Rust closure instead of leaking it. The DOM
    // still holds a live reference to the trampoline until the timer
    // fires, which is what the timeout handle then detaches.
    raytrace_schedule_one_shot(pending_timers_for_init.clone(), 0, || {
        draw_game_3d_loading(RAYTRACE_LOADING_CANVAS_SELECTOR, RAYTRACE_CANVAS_SELECTOR);
    });
    spawn_local(async move {
        let Some(canvas): Option<HtmlCanvasElement> =
            query_raytrace_canvas(RAYTRACE_CANVAS_SELECTOR)
        else {
            init_state.get_active().set(false);
            init_state
                .get_init_error_code()
                .set(RAYTRACE_ERROR_CODE_INIT);
            init_state.get_error_detail().set(String::new());
            init_state.get_loaded().set(true);
            return;
        };
        // The scene and the lights are both static, so both are built
        // exactly once here and handed to the engine. From this point on
        // no frame allocates, clones, or re-packs either one.
        let scene: RayTraceScene = build_raytrace_scene();
        let renderer: RayTraceRenderer =
            match RayTraceRenderer::init(canvas, RayTraceBackend::Canvas2D, &scene).await {
                Ok(value) => value,
                Err(error) => {
                    raytrace_record_error(
                        init_state.get_active(),
                        init_state.get_error_detail(),
                        init_state.get_init_error_code(),
                        RAYTRACE_ERROR_CODE_INIT,
                        "canvas2d init",
                        &error,
                    );
                    init_state.get_loaded().set(true);
                    return;
                }
            };
        if cancelled_for_init.get() {
            let mut stale: RayTraceRenderer = renderer;
            stale.dispose();
            return;
        }
        Console::log(format!(
            "{RAYTRACE_LOG_PREFIX} {RAYTRACE_LOG_BACKEND_READY} (2d)"
        ));
        *renderer_rc.borrow_mut() = Some(renderer);
        raytrace_set_loaded_delayed(
            pending_timers_for_init,
            init_state.get_loaded(),
            RAYTRACE_LOADING_MIN_MILLIS,
        );
        let lights: LightingUniforms = build_raytrace_lighting();
        let settings: RayTraceSettings = build_raytrace_settings();
        let scale_index: Rc<Cell<usize>> = Rc::new(Cell::new(RAYTRACE_RENDER_SCALE_START_INDEX));
        let ema_millis: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        let slow_frames: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let fast_frames: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let very_fast_frames: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
        let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        let renderer_for_loop: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let scale_clone: Rc<Cell<usize>> = scale_index.clone();
        let ema_clone: Rc<Cell<f64>> = ema_millis.clone();
        let slow_clone: Rc<Cell<u32>> = slow_frames.clone();
        let fast_clone: Rc<Cell<u32>> = fast_frames.clone();
        let very_fast_clone: Rc<Cell<u32>> = very_fast_frames.clone();
        let yaw_clone: Rc<Cell<f64>> = angles.yaw.clone();
        let pitch_clone: Rc<Cell<f64>> = angles.pitch.clone();
        let cancelled_for_loop: Rc<Cell<bool>> = cancelled.clone();
        let raf_for_detach: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_for_detach: RafClosureCell = closure_cell.clone();
        let timers_for_detach: RayTracePendingTimers = pending_timers.clone();
        let renderer_for_detach: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
        let timers_for_loop: RayTracePendingTimers = pending_timers.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            if cancelled_for_loop.get() {
                return;
            }
            if raytrace_canvas_detached(RAYTRACE_CANVAS_SELECTOR) {
                // Router navigation mounts another page without running
                // any hook cleanup, so the loop has to reach its own
                // teardown. The frame body is already unwound by the time
                // this call returns, and the release itself defers the
                // last drop to a microtask, so the closure being taken
                // from underneath its own frame is safe.
                raytrace_release_loop(
                    cancelled_for_loop.clone(),
                    raf_for_detach.clone(),
                    cell_for_detach.clone(),
                    timers_for_detach.clone(),
                    renderer_for_detach.clone(),
                );
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
            if loop_state.get_auto_rotate().get() {
                yaw_clone.set(yaw_clone.get() + RAYTRACE_AUTO_YAW_SPEED * anim_time);
            }
            if loop_state.get_running().get() {
                let scale: f64 = RAYTRACE_RENDER_SCALES[scale_clone.get()];
                let (frame_width, frame_height): (u32, u32) = raytrace_scaled_dimensions(scale);
                let camera: Camera3D = build_raytrace_camera(
                    yaw_clone.get(),
                    pitch_clone.get(),
                    frame_width,
                    frame_height,
                );
                let render_start: f64 = performance.now();
                let render_result: Result<(), JsValue> =
                    match renderer_for_loop.borrow_mut().as_mut() {
                        Some(renderer) => renderer.render(&camera, &lights, &settings),
                        None => Err(JsValue::NULL),
                    };
                let render_millis: f64 = performance.now() - render_start;
                match render_result {
                    Ok(()) => {
                        raytrace_clear_error(
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                        );
                        if !loop_state.get_loaded().get() {
                            raytrace_set_loaded_delayed(
                                timers_for_loop.clone(),
                                loop_state.get_loaded(),
                                RAYTRACE_LOADING_MIN_MILLIS,
                            );
                        }
                    }
                    Err(error) => {
                        raytrace_record_error(
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                            RAYTRACE_ERROR_CODE_RENDER,
                            "canvas2d render",
                            &error,
                        );
                    }
                }
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
                    // Sustained headroom far below the budget skips a rung
                    // so strong hardware reaches the sharp top of the ladder
                    // in a handful of steps instead of crawling one at a
                    // time.
                    next = if very_fast_clone.get() >= RAYTRACE_ADAPT_FAST_FRAMES {
                        index.saturating_sub(2)
                    } else {
                        index - 1
                    };
                }
                if next != index {
                    // A rung change is a real surface change: the frame
                    // above just rendered at `index`, so the new size is
                    // committed now, through the one path that owns the
                    // canvas backing store. A rejected resize reports
                    // itself and leaves both the backend and the canvas
                    // on the size the frame used, so the ladder's
                    // `scale_index` is rolled back with it instead of
                    // leaving the next frame building a camera for a
                    // surface the backend does not have.
                    let (next_width, next_height): (u32, u32) =
                        raytrace_scaled_dimensions(RAYTRACE_RENDER_SCALES[next]);
                    let resized: bool = match renderer_for_loop.try_borrow_mut().ok() {
                        Some(mut renderer_slot) => match renderer_slot.as_mut() {
                            Some(renderer) => renderer.resize(next_width, next_height).is_ok(),
                            None => false,
                        },
                        None => false,
                    };
                    if resized {
                        scale_clone.set(next);
                        slow_clone.set(0);
                        fast_clone.set(0);
                        very_fast_clone.set(0);
                        loop_state
                            .get_render_scale()
                            .set(RAYTRACE_RENDER_SCALES[next]);
                    } else {
                        raytrace_record_error(
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                            RAYTRACE_ERROR_CODE_RESIZE,
                            "render scale",
                            &JsValue::NULL,
                        );
                        // Reset the streak so the controller does not
                        // re-attempt the same rejected rung on the very
                        // next frame.
                        slow_clone.set(0);
                        fast_clone.set(0);
                        very_fast_clone.set(0);
                    }
                }
            }
            // Only a frame that actually rendered is a rendered frame:
            // counting the ticks a paused loop still receives would let
            // the FPS readout claim 60 FPS while nothing is drawn.
            if loop_state.get_running().get() {
                frame_clone.set(frame_clone.get() + 1);
                fps_clone.set(fps_clone.get() + frame_time);
                if fps_clone.get() >= 1.0 {
                    let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
                    loop_state.get_fps().set(fps);
                    frame_clone.set(0);
                    fps_clone.set(0.0);
                }
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
        let raf_for_start: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_for_start: RafClosureCell = closure_cell.clone();
        // The delayed kickoff is a one-shot timer owned by the same
        // pending-timer list as the loading flip, so teardown cancels it
        // and drops its closure instead of leaking it. The list is
        // drained on teardown and is not reused afterwards, so no pruning
        // of already-fired entries is needed.
        raytrace_schedule_one_shot(
            pending_timers.clone(),
            RAYTRACE_LOOP_START_DELAY_MILLIS,
            move || {
                let Some(start_window): Option<Window> = window() else {
                    return;
                };
                let Some(start_raf_ref): Option<&'static Closure<dyn FnMut()>> =
                    cell_for_start.try_get()
                else {
                    return;
                };
                let start_id: i32 = start_window
                    .request_animation_frame(start_raf_ref.as_ref().unchecked_ref())
                    .unwrap_or_default();
                raf_for_start.set(Some(start_id));
            },
        );
    });
}

/// Starts the RayTrace WebGL loop driven by `requestAnimationFrame`.
///
/// The loop owns exactly one [`RayTraceRenderer`] built through the
/// engine's `RayTraceRenderer::init` with the requested
/// [`RayTraceBackend::WebGl2`]. The demo owns no GLSL and no WGSL of its
/// own: the engine compiles and links its shared shaders during `init`,
/// uploads the packed scene once, and every frame after that is a
/// camera / lights / settings update against persistent engine-side
/// textures and program objects.
///
/// # Arguments
///
/// - `UseRayTraceWebGl` - The WebGL backend state for signal updates.
/// - `RayTraceCameraAngles` - The shared non-reactive camera orbit angles.
pub(crate) fn start_raytrace_webgl_loop(state: UseRayTraceWebGl, angles: RayTraceCameraAngles) {
    let init_state: UseRayTraceWebGl = state;
    let loop_state: UseRayTraceWebGl = state;
    let renderer_rc: Rc<RefCell<Option<RayTraceRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let observer_cell: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));
    // Owns the ResizeObserver's Rust callback. The observer is built
    // asynchronously below, but its closure outlives this function, so
    // the slot is declared here where the cleanup can already reach it.
    let observer_closure_cell: RayTraceObserverClosureOwner = Rc::new(RefCell::new(None));
    let pending_timers: RayTracePendingTimers = Rc::new(RefCell::new(Vec::new()));
    let debounce_owner: RayTraceDebounceOwner = raytrace_register_resize_debounce(
        resize_dirty.clone(),
        resize_timer.clone(),
        cancelled.clone(),
    );
    let renderer_for_cleanup: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let cancelled_for_cleanup: Rc<Cell<bool>> = cancelled.clone();
    let resize_timer_for_cleanup: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let observer_for_cleanup: Rc<RefCell<Option<ResizeObserver>>> = observer_cell.clone();
    let observer_closure_for_cleanup: RayTraceObserverClosureOwner = observer_closure_cell.clone();
    let timers_for_cleanup: RayTracePendingTimers = pending_timers.clone();
    let debounce_for_cleanup: RayTraceDebounceOwner = debounce_owner;
    App::use_cleanup(move || {
        raytrace_release_loop(
            cancelled_for_cleanup,
            raf_for_cleanup,
            cell_for_cleanup,
            timers_for_cleanup,
            renderer_for_cleanup,
        );
        // Every step below is independent: a missing `window()` must
        // never skip the observer teardown, so no early returns here.
        if let Some(timer_id) = resize_timer_for_cleanup.get()
            && let Some(window_value) = window()
        {
            window_value.clear_timeout_with_handle(timer_id);
        }
        // Disconnect the ResizeObserver so its closure (and the renderer
        // it holds) is released on tab switch. Without this the observer
        // keeps the renderer alive past the loop's lifetime, holding GPU
        // resources until GC.
        if let Some(observer) = observer_for_cleanup
            .try_borrow_mut()
            .ok()
            .and_then(|mut observer_slot: RefMut<'_, Option<ResizeObserver>>| observer_slot.take())
        {
            observer.disconnect();
        }
        // Dropped only after `disconnect()`: while the observer is live
        // the browser may call the trampoline at any point, and a freed
        // one aborts the whole WASM instance.
        let _: Option<Closure<dyn FnMut(js_sys::Array, ResizeObserver)>> =
            observer_closure_for_cleanup
                .try_borrow_mut()
                .ok()
                .and_then(|mut closure_slot: RayTraceObserverCallbackSlot<'_>| closure_slot.take());
        // Released only here: the resize listener stays registered
        // across a router navigation, and dropping its callback while it
        // can still be invoked would make the next window resize throw.
        let _: Option<Closure<dyn FnMut()>> = debounce_for_cleanup
            .try_borrow_mut()
            .ok()
            .and_then(|mut debounce_slot: RayTraceDebounceCallbackSlot<'_>| debounce_slot.take());
    });
    let cancelled_for_init: Rc<Cell<bool>> = cancelled.clone();
    let pending_timers_for_init: RayTracePendingTimers = pending_timers.clone();
    let observer_closure_cell_for_init: RayTraceObserverClosureOwner =
        observer_closure_cell.clone();
    // Owned by the same pending-timer list as every other one-shot, so
    // teardown drops the Rust closure instead of leaking it. The DOM
    // still holds a live reference to the trampoline until the timer
    // fires, which is what the timeout handle then detaches.
    raytrace_schedule_one_shot(pending_timers_for_init.clone(), 0, || {
        draw_game_3d_loading(
            RAYTRACE_WEBGL_LOADING_CANVAS_SELECTOR,
            RAYTRACE_WEBGL_CANVAS_SELECTOR,
        );
    });
    spawn_local(async move {
        let Some(canvas): Option<HtmlCanvasElement> =
            query_raytrace_canvas(RAYTRACE_WEBGL_CANVAS_SELECTOR)
        else {
            init_state.get_active().set(false);
            init_state
                .get_init_error_code()
                .set(RAYTRACE_ERROR_CODE_INIT);
            init_state.get_error_detail().set(String::new());
            init_state.get_loaded().set(true);
            return;
        };
        // Static scene: built once, uploaded once, never re-sent. The GPU
        // backends repack it into the shared `vec4` layout inside `init`.
        let scene: RayTraceScene = build_raytrace_scene();
        let renderer: RayTraceRenderer =
            match RayTraceRenderer::init(canvas, RayTraceBackend::WebGl2, &scene).await {
                Ok(value) => value,
                Err(error) => {
                    raytrace_record_error(
                        init_state.get_active(),
                        init_state.get_error_detail(),
                        init_state.get_init_error_code(),
                        RAYTRACE_ERROR_CODE_INIT,
                        "webgl2 init",
                        &error,
                    );
                    init_state.get_loaded().set(true);
                    return;
                }
            };
        if cancelled_for_init.get() {
            let mut stale: RayTraceRenderer = renderer;
            stale.dispose();
            return;
        }
        Console::log(format!(
            "{RAYTRACE_LOG_PREFIX} {RAYTRACE_LOG_BACKEND_READY} (gl)"
        ));
        *renderer_rc.borrow_mut() = Some(renderer);
        raytrace_set_loaded_delayed(
            pending_timers_for_init,
            init_state.get_loaded(),
            RAYTRACE_LOADING_MIN_MILLIS,
        );
        let lights: LightingUniforms = build_raytrace_lighting();
        let settings: RayTraceSettings = build_raytrace_settings();
        // Synchronous resize on CSS-box change. ResizeObserver callbacks run
        // BEFORE the browser paints the next frame, so resizing inside the
        // observer collapses the stretched-frame window after a fullscreen
        // transition to zero.
        let renderer_for_observer: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
        let observer_closure: Closure<dyn FnMut(js_sys::Array, ResizeObserver)> = Closure::wrap(
            Box::new(move |_entries: js_sys::Array, _obs: ResizeObserver| {
                let renderer_for_frame: Rc<RefCell<Option<RayTraceRenderer>>> =
                    renderer_for_observer.clone();
                let resize_accepted: bool = match renderer_for_frame.try_borrow_mut() {
                    Ok(mut renderer_slot) => match renderer_slot.as_mut() {
                        Some(renderer) => raytrace_resize_backend_to_canvas(
                            renderer,
                            RAYTRACE_WEBGL_CANVAS_SELECTOR,
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                        ),
                        // The renderer is gone: a tab switch already
                        // disposed it, so there is no surface left to
                        // resize and nothing to report.
                        None => true,
                    },
                    Err(_contended) => false,
                };
                if !resize_accepted {
                    // `raytrace_resize_backend_to_canvas` already recorded
                    // the engine's own reason against the status banner;
                    // this only makes the rejected observer pass visible
                    // in the console, so a silently frozen surface after a
                    // fullscreen transition is traceable.
                    Console::error(format!(
                        "{RAYTRACE_LOG_PREFIX} resize observer refused the new CSS box; \
                         the backend keeps its previous size"
                    ));
                }
            }),
        );
        let observer_callback: Function = observer_closure
            .as_ref()
            .unchecked_ref::<Function>()
            .clone();
        // Ownership moves into the slot the cleanup owns. Dropping it here
        // would let the browser invoke a freed trampoline: the observer
        // keeps firing after this async init returns.
        if let Ok(mut closure_slot) = observer_closure_cell_for_init.try_borrow_mut() {
            *closure_slot = Some(observer_closure);
        }
        if let Ok(resize_observer) = ResizeObserver::new(&observer_callback)
            && let Some(window_value) = window()
            && let Some(document_value) = window_value.document()
            && let Some(element) = document_value
                .query_selector(RAYTRACE_WEBGL_CANVAS_SELECTOR)
                .ok()
                .flatten()
        {
            resize_observer.observe(&element);
            // `try_borrow_mut`, not `borrow_mut`: the tab-switch cleanup
            // registered above also mutably borrows this same cell from
            // `HookContext::switch_arm`, and this backend's init is
            // asynchronous, so the arm can switch while init is still in
            // flight and the two borrows genuinely overlap. A
            // BorrowMutError in WASM aborts the whole app with no catchable
            // panic. On contention the observer is dropped here and the loop
            // runs without live resize notifications — the per-frame
            // backing-store divergence check still resizes.
            if let Ok(mut observer_slot) = observer_cell.try_borrow_mut() {
                *observer_slot = Some(resize_observer);
            }
        }
        let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
        let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        let renderer_for_loop: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let yaw_clone: Rc<Cell<f64>> = angles.yaw.clone();
        let pitch_clone: Rc<Cell<f64>> = angles.pitch.clone();
        let resize_dirty_for_loop: Rc<Cell<bool>> = resize_dirty.clone();
        let cancelled_for_loop: Rc<Cell<bool>> = cancelled.clone();
        let raf_for_detach: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_for_detach: RafClosureCell = closure_cell.clone();
        let timers_for_detach: RayTracePendingTimers = pending_timers.clone();
        let renderer_for_detach: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
        let timers_for_loop: RayTracePendingTimers = pending_timers.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            if cancelled_for_loop.get() {
                return;
            }
            if raytrace_canvas_detached(RAYTRACE_WEBGL_CANVAS_SELECTOR) {
                // Router navigation mounts another page without running
                // any hook cleanup, so the loop has to reach its own
                // teardown. The ResizeObserver is intentionally *not*
                // disconnected here: it is still observed and can still
                // fire, so dropping it from under the browser would leave
                // a live observer pointing at a disposed backend. It is
                // disconnected by the hook cleanup, and until then its
                // resize call is a no-op because the renderer is gone.
                raytrace_release_loop(
                    cancelled_for_loop.clone(),
                    raf_for_detach.clone(),
                    cell_for_detach.clone(),
                    timers_for_detach.clone(),
                    renderer_for_detach.clone(),
                );
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
                yaw_clone.set(yaw_clone.get() + RAYTRACE_AUTO_YAW_SPEED * anim_time);
            }
            if resize_dirty_for_loop.get() {
                resize_dirty_for_loop.set(false);
            }
            // A paused loop renders nothing: the previous frame stays on
            // screen and the backend is not touched at all, which is what
            // the pause button promises and what keeps a paused tab from
            // burning a GPU pass per animation frame.
            let running: bool = loop_state.get_running().get();
            let render_result: Option<Result<(), JsValue>> = if running {
                renderer_for_loop.try_borrow_mut().ok().and_then(
                    |mut renderer_slot: RefMut<'_, Option<RayTraceRenderer>>| {
                        let renderer: &mut RayTraceRenderer = renderer_slot.as_mut()?;
                        // The renderer owns the canvas backing store, so a
                        // rejected resize leaves both on the old size. The
                        // frame is then skipped rather than drawn against
                        // a size the backend does not have.
                        if !raytrace_resize_backend_to_canvas(
                            renderer,
                            RAYTRACE_WEBGL_CANVAS_SELECTOR,
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                        ) {
                            return None;
                        }
                        // The contract exposes no canvas accessor on the
                        // renderer, so the backing store is read back off
                        // the DOM element the loop already owns.
                        let width: u32 =
                            raytrace_canvas_backing_width(RAYTRACE_WEBGL_CANVAS_SELECTOR);
                        let height: u32 =
                            raytrace_canvas_backing_height(RAYTRACE_WEBGL_CANVAS_SELECTOR);
                        let camera: Camera3D = build_raytrace_camera(
                            yaw_clone.get(),
                            pitch_clone.get(),
                            width,
                            height,
                        );
                        Some(renderer.render(&camera, &lights, &settings))
                    },
                )
            } else {
                None
            };
            if running {
                match render_result {
                    Some(Ok(())) => {
                        raytrace_clear_error(
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                        );
                        if !loop_state.get_loaded().get() {
                            raytrace_set_loaded_delayed(
                                timers_for_loop.clone(),
                                loop_state.get_loaded(),
                                RAYTRACE_LOADING_MIN_MILLIS,
                            );
                        }
                    }
                    Some(Err(error)) => {
                        raytrace_record_error(
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                            RAYTRACE_ERROR_CODE_RENDER,
                            "webgl2 render",
                            &error,
                        );
                    }
                    // A rejected resize and a missing renderer both land
                    // here: neither produced a frame. The resize failure
                    // already recorded its own code and detail, so it is
                    // not overwritten with a generic render rejection.
                    // Control falls through to the frame re-registration
                    // below either way, which is what lets the loop
                    // recover once the resize succeeds again.
                    None => {
                        if loop_state.get_error_detail().get().is_empty() {
                            raytrace_record_error(
                                loop_state.get_active(),
                                loop_state.get_error_detail(),
                                loop_state.get_init_error_code(),
                                RAYTRACE_ERROR_CODE_RENDER,
                                "webgl2 render",
                                &JsValue::NULL,
                            );
                        }
                    }
                }
            }
            // Only a frame that actually rendered counts toward the rate.
            if running {
                frame_clone.set(frame_clone.get() + 1);
                fps_clone.set(fps_clone.get() + frame_time);
                if fps_clone.get() >= 1.0 {
                    let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
                    loop_state.get_fps().set(fps);
                    frame_clone.set(0);
                    fps_clone.set(0.0);
                }
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
/// The loop owns exactly one [`RayTraceRenderer`] built through the
/// engine's `RayTraceRenderer::init` with the requested
/// [`RayTraceBackend::WebGpu`]. `init` is asynchronous (it awaits the
/// adapter and device, then awaits shader compilation), so the whole
/// sequence runs inside `spawn_local` behind a cancellation guard for tab
/// switches. A rejected init surfaces its error on the status banner and
/// the loop exits quietly rather than falling back to another backend.
///
/// # Arguments
///
/// - `UseRayTraceWebGpu` - The WebGPU backend state for signal updates.
/// - `RayTraceCameraAngles` - The shared non-reactive camera orbit angles.
pub(crate) fn start_raytrace_webgpu_loop(state: UseRayTraceWebGpu, angles: RayTraceCameraAngles) {
    let init_state: UseRayTraceWebGpu = state;
    let loop_state: UseRayTraceWebGpu = state;
    let renderer_rc: Rc<RefCell<Option<RayTraceRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let observer_cell: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));
    // Owns the ResizeObserver's Rust callback. The observer is built
    // asynchronously below, but its closure outlives this function, so
    // the slot is declared here where the cleanup can already reach it.
    let observer_closure_cell: RayTraceObserverClosureOwner = Rc::new(RefCell::new(None));
    let pending_timers: RayTracePendingTimers = Rc::new(RefCell::new(Vec::new()));
    let debounce_owner: RayTraceDebounceOwner = raytrace_register_resize_debounce(
        resize_dirty.clone(),
        resize_timer.clone(),
        cancelled.clone(),
    );
    let renderer_for_cleanup: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let cancelled_for_cleanup: Rc<Cell<bool>> = cancelled.clone();
    let resize_timer_for_cleanup: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let observer_for_cleanup: Rc<RefCell<Option<ResizeObserver>>> = observer_cell.clone();
    let observer_closure_for_cleanup: RayTraceObserverClosureOwner = observer_closure_cell.clone();
    let timers_for_cleanup: RayTracePendingTimers = pending_timers.clone();
    let debounce_for_cleanup: RayTraceDebounceOwner = debounce_owner;
    App::use_cleanup(move || {
        // `dispose` frees the owned buffers, textures, and device eagerly
        // instead of leaving them to GC, which is what lets a fresh
        // `init` acquire a live device rather than reusing a dead one.
        raytrace_release_loop(
            cancelled_for_cleanup,
            raf_for_cleanup,
            cell_for_cleanup,
            timers_for_cleanup,
            renderer_for_cleanup,
        );
        // Every step below is independent: a missing `window()` must
        // never skip the observer teardown, so no early returns here.
        if let Some(timer_id) = resize_timer_for_cleanup.get()
            && let Some(window_value) = window()
        {
            window_value.clear_timeout_with_handle(timer_id);
        }
        if let Some(observer) = observer_for_cleanup
            .try_borrow_mut()
            .ok()
            .and_then(|mut observer_slot: RefMut<'_, Option<ResizeObserver>>| observer_slot.take())
        {
            observer.disconnect();
        }
        // Dropped only after `disconnect()`: while the observer is live
        // the browser may call the trampoline at any point, and a freed
        // one aborts the whole WASM instance.
        let _: Option<Closure<dyn FnMut(js_sys::Array, ResizeObserver)>> =
            observer_closure_for_cleanup
                .try_borrow_mut()
                .ok()
                .and_then(|mut closure_slot: RayTraceObserverCallbackSlot<'_>| closure_slot.take());
        // Released only here: the resize listener stays registered
        // across a router navigation, and dropping its callback while it
        // can still be invoked would make the next window resize throw.
        let _: Option<Closure<dyn FnMut()>> = debounce_for_cleanup
            .try_borrow_mut()
            .ok()
            .and_then(|mut debounce_slot: RayTraceDebounceCallbackSlot<'_>| debounce_slot.take());
    });
    let cancelled_for_init: Rc<Cell<bool>> = cancelled.clone();
    let pending_timers_for_init: RayTracePendingTimers = pending_timers.clone();
    let observer_closure_cell_for_init: RayTraceObserverClosureOwner =
        observer_closure_cell.clone();
    // Owned by the same pending-timer list as every other one-shot, so
    // teardown drops the Rust closure instead of leaking it. The DOM
    // still holds a live reference to the trampoline until the timer
    // fires, which is what the timeout handle then detaches.
    raytrace_schedule_one_shot(pending_timers_for_init.clone(), 0, || {
        draw_game_3d_loading(
            RAYTRACE_WEBGPU_LOADING_CANVAS_SELECTOR,
            RAYTRACE_WEBGPU_CANVAS_SELECTOR,
        );
    });
    spawn_local(async move {
        let Some(canvas): Option<HtmlCanvasElement> =
            query_raytrace_canvas(RAYTRACE_WEBGPU_CANVAS_SELECTOR)
        else {
            init_state.get_active().set(false);
            init_state
                .get_init_error_code()
                .set(RAYTRACE_ERROR_CODE_INIT);
            init_state.get_error_detail().set(String::new());
            init_state.get_loaded().set(true);
            return;
        };
        let scene: RayTraceScene = build_raytrace_scene();
        let renderer: RayTraceRenderer =
            match RayTraceRenderer::init(canvas, RayTraceBackend::WebGpu, &scene).await {
                Ok(value) => value,
                Err(error) => {
                    raytrace_record_error(
                        init_state.get_active(),
                        init_state.get_error_detail(),
                        init_state.get_init_error_code(),
                        RAYTRACE_ERROR_CODE_INIT,
                        "webgpu init",
                        &error,
                    );
                    init_state.get_loaded().set(true);
                    return;
                }
            };
        if cancelled_for_init.get() {
            // The tab was switched away while the adapter / device promises
            // were in flight. A successfully-created renderer still owns a
            // live GPU device at this point; destroy it eagerly instead of
            // leaving it for GC to discover.
            let mut stale: RayTraceRenderer = renderer;
            stale.dispose();
            return;
        }
        Console::log(format!(
            "{RAYTRACE_LOG_PREFIX} {RAYTRACE_LOG_BACKEND_READY} (gpu)"
        ));
        *renderer_rc.borrow_mut() = Some(renderer);
        raytrace_set_loaded_delayed(
            pending_timers_for_init,
            init_state.get_loaded(),
            RAYTRACE_LOADING_MIN_MILLIS,
        );
        let lights: LightingUniforms = build_raytrace_lighting();
        let settings: RayTraceSettings = build_raytrace_settings();
        let renderer_for_observer: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
        let observer_closure: Closure<dyn FnMut(js_sys::Array, ResizeObserver)> = Closure::wrap(
            Box::new(move |_entries: js_sys::Array, _obs: ResizeObserver| {
                let renderer_for_frame: Rc<RefCell<Option<RayTraceRenderer>>> =
                    renderer_for_observer.clone();
                let resize_accepted: bool = match renderer_for_frame.try_borrow_mut() {
                    Ok(mut renderer_slot) => match renderer_slot.as_mut() {
                        Some(renderer) => raytrace_resize_backend_to_canvas(
                            renderer,
                            RAYTRACE_WEBGPU_CANVAS_SELECTOR,
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                        ),
                        // The renderer is gone: a tab switch already
                        // disposed it, so there is no surface left to
                        // resize and nothing to report.
                        None => true,
                    },
                    Err(_contended) => false,
                };
                if !resize_accepted {
                    // `raytrace_resize_backend_to_canvas` already recorded
                    // the engine's own reason against the status banner;
                    // this only makes the rejected observer pass visible
                    // in the console, so a silently frozen surface after a
                    // fullscreen transition is traceable.
                    Console::error(format!(
                        "{RAYTRACE_LOG_PREFIX} resize observer refused the new CSS box; \
                         the backend keeps its previous size"
                    ));
                }
            }),
        );
        let observer_callback: Function = observer_closure
            .as_ref()
            .unchecked_ref::<Function>()
            .clone();
        // Ownership moves into the slot the cleanup owns. Dropping it here
        // would let the browser invoke a freed trampoline: the observer
        // keeps firing after this async init returns.
        if let Ok(mut closure_slot) = observer_closure_cell_for_init.try_borrow_mut() {
            *closure_slot = Some(observer_closure);
        }
        if let Ok(resize_observer) = ResizeObserver::new(&observer_callback)
            && let Some(window_value) = window()
            && let Some(document_value) = window_value.document()
            && let Some(element) = document_value
                .query_selector(RAYTRACE_WEBGPU_CANVAS_SELECTOR)
                .ok()
                .flatten()
        {
            resize_observer.observe(&element);
            // See the WebGL loop for why `try_borrow_mut` and not
            // `borrow_mut`.
            if let Ok(mut observer_slot) = observer_cell.try_borrow_mut() {
                *observer_slot = Some(resize_observer);
            }
        }
        let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
        let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        let renderer_for_loop: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let yaw_clone: Rc<Cell<f64>> = angles.yaw.clone();
        let pitch_clone: Rc<Cell<f64>> = angles.pitch.clone();
        let resize_dirty_for_loop: Rc<Cell<bool>> = resize_dirty.clone();
        let cancelled_for_loop: Rc<Cell<bool>> = cancelled.clone();
        let raf_for_detach: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_for_detach: RafClosureCell = closure_cell.clone();
        let timers_for_detach: RayTracePendingTimers = pending_timers.clone();
        let renderer_for_detach: Rc<RefCell<Option<RayTraceRenderer>>> = renderer_rc.clone();
        let timers_for_loop: RayTracePendingTimers = pending_timers.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            if cancelled_for_loop.get() {
                return;
            }
            if raytrace_canvas_detached(RAYTRACE_WEBGPU_CANVAS_SELECTOR) {
                // Router navigation mounts another page without running
                // any hook cleanup, so the loop has to reach its own
                // teardown. The ResizeObserver is intentionally *not*
                // disconnected here: it is still observed and can still
                // fire, so dropping it from under the browser would leave
                // a live observer pointing at a disposed backend. It is
                // disconnected by the hook cleanup, and until then its
                // resize call is a no-op because the renderer is gone.
                raytrace_release_loop(
                    cancelled_for_loop.clone(),
                    raf_for_detach.clone(),
                    cell_for_detach.clone(),
                    timers_for_detach.clone(),
                    renderer_for_detach.clone(),
                );
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
            let frame_time: f64 = if prev < 0.0 {
                1.0 / 60.0
            } else {
                current_time - prev
            };
            let anim_time: f64 = frame_time.min(0.25);
            last_clone.set(current_time);
            if loop_state.get_auto_rotate().get() {
                yaw_clone.set(yaw_clone.get() + RAYTRACE_AUTO_YAW_SPEED * anim_time);
            }
            if resize_dirty_for_loop.get() {
                resize_dirty_for_loop.set(false);
            }
            // A paused loop renders nothing: the previous frame stays on
            // screen and the device is not touched at all, which is what
            // the pause button promises and what keeps a paused tab from
            // burning a GPU pass per animation frame.
            let running: bool = loop_state.get_running().get();
            let render_result: Option<Result<(), JsValue>> = if running {
                renderer_for_loop.try_borrow_mut().ok().and_then(
                    |mut renderer_slot: RefMut<'_, Option<RayTraceRenderer>>| {
                        let renderer: &mut RayTraceRenderer = renderer_slot.as_mut()?;
                        // The renderer owns the canvas backing store, so a
                        // rejected resize leaves both on the old size. The
                        // frame is then skipped rather than drawn against
                        // a size the backend does not have.
                        if !raytrace_resize_backend_to_canvas(
                            renderer,
                            RAYTRACE_WEBGPU_CANVAS_SELECTOR,
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                        ) {
                            return None;
                        }
                        // The contract exposes no canvas accessor on the
                        // renderer, so the backing store is read back off
                        // the DOM element the loop already owns.
                        let width: u32 =
                            raytrace_canvas_backing_width(RAYTRACE_WEBGPU_CANVAS_SELECTOR);
                        let height: u32 =
                            raytrace_canvas_backing_height(RAYTRACE_WEBGPU_CANVAS_SELECTOR);
                        let camera: Camera3D = build_raytrace_camera(
                            yaw_clone.get(),
                            pitch_clone.get(),
                            width,
                            height,
                        );
                        Some(renderer.render(&camera, &lights, &settings))
                    },
                )
            } else {
                None
            };
            if running {
                match render_result {
                    Some(Ok(())) => {
                        raytrace_clear_error(
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                        );
                        if !loop_state.get_loaded().get() {
                            raytrace_set_loaded_delayed(
                                timers_for_loop.clone(),
                                loop_state.get_loaded(),
                                RAYTRACE_LOADING_MIN_MILLIS,
                            );
                        }
                    }
                    Some(Err(error)) => {
                        raytrace_record_error(
                            loop_state.get_active(),
                            loop_state.get_error_detail(),
                            loop_state.get_init_error_code(),
                            RAYTRACE_ERROR_CODE_RENDER,
                            "webgpu render",
                            &error,
                        );
                    }
                    // A rejected resize and a missing renderer both land
                    // here: neither produced a frame. The resize failure
                    // already recorded its own code and detail, so it is
                    // not overwritten with a generic render rejection.
                    // Control falls through to the frame re-registration
                    // below either way, which is what lets the loop
                    // recover once the resize succeeds again.
                    None => {
                        if loop_state.get_error_detail().get().is_empty() {
                            raytrace_record_error(
                                loop_state.get_active(),
                                loop_state.get_error_detail(),
                                loop_state.get_init_error_code(),
                                RAYTRACE_ERROR_CODE_RENDER,
                                "webgpu render",
                                &JsValue::NULL,
                            );
                        }
                    }
                }
            }
            // Only a frame that actually rendered counts toward the rate.
            if running {
                frame_clone.set(frame_clone.get() + 1);
                fps_clone.set(fps_clone.get() + frame_time);
                if fps_clone.get() >= 1.0 {
                    let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
                    loop_state.get_fps().set(fps);
                    frame_clone.set(0);
                    fps_clone.set(0.0);
                }
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

/// Reads the client X coordinate off a `MouseEvent`-like object via
/// reflection.
///
/// # Arguments
///
/// - `&Event` - The pointer event.
///
/// # Returns
///
/// - `f64` - The client X coordinate, or `0.0` if missing.
fn client_x_from_event(event: &Event) -> f64 {
    Reflect::get(
        event.as_ref(),
        &JsValue::from_str(RAYTRACE_EVENT_PROPERTY_CLIENT_X),
    )
    .ok()
    .and_then(|value: JsValue| value.as_f64())
    .unwrap_or_default()
}

/// Reads the client Y coordinate off a `MouseEvent`-like object via
/// reflection.
///
/// # Arguments
///
/// - `&Event` - The pointer event.
///
/// # Returns
///
/// - `f64` - The client Y coordinate, or `0.0` if missing.
fn client_y_from_event(event: &Event) -> f64 {
    Reflect::get(
        event.as_ref(),
        &JsValue::from_str(RAYTRACE_EVENT_PROPERTY_CLIENT_Y),
    )
    .ok()
    .and_then(|value: JsValue| value.as_f64())
    .unwrap_or_default()
}

/// Applies a drag delta to the shared orbit angles and disables
/// auto-rotate.
///
/// # Arguments
///
/// - `f64` - The horizontal drag delta in CSS pixels.
/// - `f64` - The vertical drag delta in CSS pixels.
/// - `RayTraceCameraAngles` - The shared orbit angles.
/// - `Signal<bool>` - The auto-rotate signal to disable.
fn raytrace_apply_drag(dx: f64, dy: f64, angles: RayTraceCameraAngles, auto_rotate: Signal<bool>) {
    let yaw: f64 = angles.yaw.get() - dx * RAYTRACE_DRAG_SENSITIVITY;
    let pitch: f64 = (angles.pitch.get() + dy * RAYTRACE_DRAG_SENSITIVITY).clamp(
        -FRAC_PI_2 + RAYTRACE_PITCH_CLAMP,
        FRAC_PI_2 - RAYTRACE_PITCH_CLAMP,
    );
    angles.yaw.set(yaw);
    angles.pitch.set(pitch);
    auto_rotate.set(false);
}

/// Creates a pointer move handler that updates the camera orbit angles
/// from drag movement and disables auto-rotate for the rest of the
/// session.
///
/// # Arguments
///
/// - `RayTraceCameraAngles` - The shared orbit angles.
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
        last_pointer.set(Some((client_x, client_y)));
        raytrace_apply_drag(
            client_x - last_x,
            client_y - last_y,
            angles.clone(),
            auto_rotate,
        );
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
/// - `RayTraceCameraAngles` - The shared orbit angles.
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
        last_pointer.set(Some((client_x, client_y)));
        raytrace_apply_drag(
            client_x - last_x,
            client_y - last_y,
            angles.clone(),
            auto_rotate,
        );
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

/// Creates a click event handler that sets the active tab and exits
/// any in-flight landscape fullscreen mode before switching.
///
/// Tab switches destroy the previous arm's DOM subtree (the match
/// expression rebuilds from scratch on arm change), so any tab's
/// `c_game_container_fullscreen` overlay is unmounted along with the
/// rest of that arm. The per-tab fullscreen signals, however, are
/// page-scoped `Signal<bool>` instances that survive arm destruction
/// because they are registered with the page-level HookContext, not the
/// per-arm one. Without explicit cleanup the next time the user revisits
/// that tab the overlay re-mounts even though they did not press Enter
/// Fullscreen again. Clearing all three signals on every tab change keeps
/// fullscreen state strictly co-extensive with the user's last explicit
/// enter/exit action.
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
/// Sets the tab-specific fullscreen signal, pushes a history entry so the
/// system back button can exit, and re-applies safe-area insets to the
/// newly-mounted overlay container. A synthetic window `resize` event is
/// dispatched so the GPU-backed loops resize their backing stores to the
/// new CSS box.
///
/// # Arguments
///
/// - `Signal<bool>` - The fullscreen signal for the active tab.
pub(crate) fn enter_raytrace_fullscreen(tab: Signal<bool>) {
    tab.set(true);
    Router::overlay_push_state();
    UseEuvLayout::apply_cached_insets();
    // Dispatch a `resize` event on the window so the GPU-backed loops'
    // `App::use_window_event(EVENT_RESIZE, ...)` handlers fire and their
    // `resize_dirty` flags are set.
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let event: Result<Event, JsValue> = Event::new(RAYTRACE_EVENT_RESIZE);
    if let Ok(event) = event {
        let _: Result<bool, JsValue> = window_value.dispatch_event(&event);
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
    let event: Result<Event, JsValue> = Event::new(RAYTRACE_EVENT_RESIZE);
    if let Ok(event) = event {
        let _: Result<bool, JsValue> = window_value.dispatch_event(&event);
    }
}

/// Exits landscape fullscreen mode without consuming a browser history
/// entry.
///
/// Used when the exit is triggered by the system back button: the
/// `popstate` event itself has already consumed the `pushState` entry
/// that was created when entering fullscreen, so calling `history.back()`
/// again would over-consume the history stack.
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
    let event: Result<Event, JsValue> = Event::new(RAYTRACE_EVENT_RESIZE);
    if let Ok(event) = event {
        let _: Result<bool, JsValue> = window_value.dispatch_event(&event);
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
