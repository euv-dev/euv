use super::*;

/// Creates the Lighting Canvas 2D tab reactive state.
///
/// # Returns
///
/// - `UseLighting` - The Lighting Canvas 2D tab state.
pub(crate) fn use_lighting_state() -> UseLighting {
    UseLighting {
        fps: App::use_signal(|| 0.0),
        running: App::use_signal(|| true),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        render_scale: App::use_signal(|| 1.0),
        init_error_code: App::use_signal(|| ""),
    }
}

/// Creates the Lighting WebGL tab reactive state.
///
/// # Returns
///
/// - `UseLightingWebGl` - The WebGL backend state.
pub(crate) fn use_lighting_webgl_state() -> UseLightingWebGl {
    UseLightingWebGl {
        fps: App::use_signal(|| 0.0),
        running: App::use_signal(|| true),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        init_error_code: App::use_signal(|| ""),
    }
}

/// Creates the Lighting WebGPU tab reactive state.
///
/// # Returns
///
/// - `UseLightingWebGpu` - The WebGPU backend state.
pub(crate) fn use_lighting_webgpu_state() -> UseLightingWebGpu {
    UseLightingWebGpu {
        fps: App::use_signal(|| 0.0),
        running: App::use_signal(|| true),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        init_error_code: App::use_signal(|| ""),
    }
}

/// Creates the Lighting page fullscreen overlay state signals.
///
/// Allocates hook slots in this fixed order:
///
/// 1. canvas_2d
/// 2. web_gl
/// 3. web_gpu
///
/// # Returns
///
/// - `UseLightingFullscreen` - The Lighting page fullscreen state.
pub(crate) fn use_lighting_fullscreen_state() -> UseLightingFullscreen {
    UseLightingFullscreen {
        canvas_2d: App::use_signal(|| false),
        web_gl: App::use_signal(|| false),
        web_gpu: App::use_signal(|| false),
    }
}

/// Returns `true` when no element matches the canvas selector, meaning the
/// page or tab was navigated away from and the lighting loop should stop.
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
fn lighting_canvas_detached(canvas_selector: &str) -> bool {
    window()
        .and_then(|window_value: Window| window_value.document())
        .and_then(|document: Document| document.query_selector(canvas_selector).ok().flatten())
        .is_none()
}

/// Reads the CSS pixel dimensions of a Lighting canvas element via
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
fn read_lighting_canvas_size(canvas_selector: &str) -> Option<(f64, f64)> {
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

/// Acquires the 2D context for the Lighting demo canvas, resizing the
/// backing buffer to the requested pixel dimensions if needed.
///
/// Returns `None` if the canvas element cannot be found (for example
/// while the page is mid-route transition) or if a 2D context cannot be
/// acquired.
///
/// # Arguments
///
/// - `u32` - The backing buffer width in pixels.
/// - `u32` - The backing buffer height in pixels.
///
/// # Returns
///
/// - `Option<(HtmlCanvasElement, CanvasRenderingContext2d)>` - The canvas and its 2D context.
fn acquire_lighting_canvas(
    width: u32,
    height: u32,
) -> Option<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let window_value: Window = window()?;
    let document_value: Document = window_value.document()?;
    let element: Element = document_value
        .query_selector(LIGHTING_CANVAS_SELECTOR)
        .ok()
        .flatten()?;
    let canvas: HtmlCanvasElement = element.unchecked_into();
    if canvas.width() != width {
        canvas.set_width(width);
    }
    if canvas.height() != height {
        canvas.set_height(height);
    }
    let context_object: Object = canvas.get_context(LIGHTING_CONTEXT_TYPE).ok().flatten()?;
    let context: CanvasRenderingContext2d = context_object.unchecked_into();
    Some((canvas, context))
}

/// Builds the static lighting scene used by the standalone Lighting demo.
///
/// Five spheres at varied positions and sizes plus a horizontal ground
/// line at the bottom of the canvas, all authored in the fixed 320x240
/// logical scene space. Returns the sphere list and the
/// `LightingUniforms` (one directional sun + one point lamp + ambient +
/// eye) consumed by `LightingUniforms::shade`.
///
/// # Returns
///
/// - `(Vec<LightingSphere>, LightingUniforms)` - The static scene spheres and lighting.
fn build_lighting_scene() -> (Vec<LightingSphere>, LightingUniforms) {
    let width: f64 = LIGHTING_WIDTH;
    let height: f64 = LIGHTING_HEIGHT;
    let red_material: Material = Material::phong(Vector3D::new(0.85, 0.20, 0.20), 0.5, 24.0);
    let green_material: Material = Material::phong(Vector3D::new(0.20, 0.80, 0.30), 0.6, 32.0);
    let blue_material: Material = Material::phong(Vector3D::new(0.25, 0.45, 0.95), 0.4, 18.0);
    let yellow_material: Material = Material::phong(Vector3D::new(0.95, 0.85, 0.20), 0.7, 48.0);
    let magenta_material: Material = Material::lambert(Vector3D::new(0.85, 0.25, 0.75));
    let spheres: Vec<LightingSphere> = vec![
        LightingSphere {
            cx: width * 0.22,
            cy: height * 0.42,
            radius: 24.0,
            material: red_material.clone(),
        },
        LightingSphere {
            cx: width * 0.42,
            cy: height * 0.55,
            radius: 18.0,
            material: green_material.clone(),
        },
        LightingSphere {
            cx: width * 0.62,
            cy: height * 0.40,
            radius: 22.0,
            material: blue_material.clone(),
        },
        LightingSphere {
            cx: width * 0.78,
            cy: height * 0.62,
            radius: 16.0,
            material: yellow_material.clone(),
        },
        LightingSphere {
            cx: width * 0.50,
            cy: height * 0.20,
            radius: 12.0,
            material: magenta_material,
        },
    ];
    let eye: Vector3D = Vector3D::new(0.0, 0.0, LIGHTING_EYE_Z);
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::new(0.08, 0.08, 0.10));
    let sun: Light = Light::new_directional(
        Vector3D::new(-0.45, -0.55, -0.70),
        Vector3D::new(1.00, 0.95, 0.85),
    );
    // Lamp: anchored top-left of the 320x240 logical scene so the
    // user can see the light source as a tangible glowing disk. The
    // previous off-screen position (y=-10) made the lamp invisible and
    // left the rays it was supposed to cast with no visible origin.
    // The ray overlay (Bresenham pass on Canvas 2D, `line-segment`
    // blend in the WebGL / WebGPU shaders) uses this same position
    // and the five sphere centres so all three backends agree on
    // where the rays emanate from.
    let lamp: Light = Light::new_point(
        Vector3D::new(width * 0.08, height * 0.18, 0.5),
        Vector3D::new(0.40, 0.70, 1.00),
        1.4,
    );
    lights.add_light(sun);
    lights.add_light(lamp);
    (spheres, lights)
}

/// Packs a linear `0..=1` color channel into an sRGB byte, applying the
/// shared `1/2.2` gamma curve.
///
/// The lighting math runs in linear space; this gamma correction keeps
/// the visual result from looking washed-out on a standard sRGB
/// display.
///
/// # Arguments
///
/// - `f64` - The linear color channel value.
///
/// # Returns
///
/// - `u8` - The gamma-corrected 8-bit channel value.
fn gamma_byte(value: f64) -> u8 {
    let clamped: f64 = value.clamp(0.0, 1.0);
    (clamped.powf(1.0 / 2.2) * 255.0).round() as u8
}

/// Computes the integer backing buffer dimensions for a render-scale
/// ladder step.
///
/// # Arguments
///
/// - `f64` - The render scale from [`LIGHTING_RENDER_SCALES`].
///
/// # Returns
///
/// - `(u32, u32)` - The `(width, height)` in pixels (always 4:3).
fn lighting_scaled_dimensions(scale: f64) -> (u32, u32) {
    let width: u32 = (LIGHTING_WIDTH * scale).round() as u32;
    let height: u32 = (LIGHTING_HEIGHT * scale).round() as u32;
    (width, height)
}

/// Renders one full frame of the Lighting demo into the RGBA byte
/// framebuffer.
///
/// The scene is authored in the fixed 320x240 logical space; the
/// framebuffer samples that space at `1 / scale` density so adaptive
/// resolution changes nothing but sharpness. Per framebuffer pixel the
/// ground row (logical y in `[187, 188)`) is shaded first with a fixed
/// up-pointing normal, then every sphere is coverage-tested at the same
/// 2x2 sub-sample offsets used historically and painted over whatever
/// came before, matching the legacy painter's order exactly. Pixels
/// outside every shape keep alpha 0 so the CSS canvas background shows
/// through, exactly like the old `clear_rect` + sparse `fill_rect`
/// path. All `shade` calls share one empty occluder slice — no heap
/// allocation happens anywhere in the frame.
///
/// # Arguments
///
/// - `&mut [u8]` - The RGBA framebuffer (length `width * height * 4`).
/// - `u32` - The framebuffer width in pixels.
/// - `u32` - The framebuffer height in pixels.
/// - `f64` - The current render scale (`framebuffer / logical`).
/// - `&[LightingSphere]` - The scene spheres in logical coordinates.
/// - `&LightingUniforms` - The scene lighting.
fn render_lighting_frame(
    buffer: &mut [u8],
    width: u32,
    height: u32,
    scale: f64,
    spheres: &[LightingSphere],
    lights: &LightingUniforms,
) {
    let occluders: [(Vector3D, f64); 0] = [];
    let ground_material: Material = Material::phong(Vector3D::new(0.55, 0.55, 0.60), 0.15, 12.0);
    let ground_y: f64 = (LIGHTING_HEIGHT * 0.78) as i32 as f64;
    let inv_scale: f64 = 1.0 / scale;
    let sub_offsets: [(f64, f64); 4] = [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)];
    let mut index: usize = 0;
    for y in 0..height {
        for x in 0..width {
            let mut red: f64 = 0.0;
            let mut green: f64 = 0.0;
            let mut blue: f64 = 0.0;
            let mut alpha: u8 = 0;
            let logical_x: f64 = (f64::from(x) + 0.5) * inv_scale;
            let logical_y: f64 = (f64::from(y) + 0.5) * inv_scale;
            if logical_y >= ground_y && logical_y < ground_y + 1.0 {
                let position: Vector3D = Vector3D::new(logical_x.floor(), ground_y, 0.0);
                let normal: Vector3D = Vector3D::new(0.0, -1.0, 0.0);
                let color: Vector3D = lights.shade(position, normal, &ground_material, &occluders);
                red = color.get_x();
                green = color.get_y();
                blue = color.get_z();
                alpha = 255;
            }
            for sphere in spheres.iter() {
                let radius_squared: f64 = sphere.radius * sphere.radius;
                let mut inside_count: u32 = 0;
                let mut sum_r: f64 = 0.0;
                let mut sum_g: f64 = 0.0;
                let mut sum_b: f64 = 0.0;
                for (dx_off, dy_off) in sub_offsets {
                    let sample_x: f64 = (f64::from(x) + dx_off) * inv_scale;
                    let sample_y: f64 = (f64::from(y) + dy_off) * inv_scale;
                    let dx: f64 = sample_x - sphere.cx;
                    let dy: f64 = sample_y - sphere.cy;
                    let distance_squared: f64 = dx * dx + dy * dy;
                    if distance_squared > radius_squared {
                        continue;
                    }
                    inside_count += 1;
                    let dz: f64 = (radius_squared - distance_squared).max(0.0).sqrt();
                    let normal: Vector3D =
                        Vector3D::new(dx / sphere.radius, dy / sphere.radius, dz / sphere.radius);
                    let position: Vector3D = Vector3D::new(sample_x, sample_y, dz / sphere.radius);
                    let color: Vector3D =
                        lights.shade(position, normal, &sphere.material, &occluders);
                    sum_r += color.get_x();
                    sum_g += color.get_y();
                    sum_b += color.get_z();
                }
                if inside_count > 0 {
                    let inv_count: f64 = 1.0 / f64::from(inside_count);
                    red = sum_r * inv_count;
                    green = sum_g * inv_count;
                    blue = sum_b * inv_count;
                    alpha = 255;
                }
            }
            buffer[index] = gamma_byte(red);
            buffer[index + 1] = gamma_byte(green);
            buffer[index + 2] = gamma_byte(blue);
            buffer[index + 3] = alpha;
            index += 4;
        }
    }
    // Ray overlay (lamps -> sphere centres, sun-disk marker): drawn
    // AFTER scene shading so the rays read as visible light beams
    // emanating from the lamp's top-left corner toward each sphere
    // rather than the (wrong) direction the Lambert term picks. Uses
    // Bresenham at the framebuffer resolution (logical_x * scale,
    // logical_y * scale) so adaptive resolution scales the rays
    // proportionally. Lines are ~1 physical-pixel thick with the
    // lamp's color at ~38% alpha so spheres show through.
    let width_i: i32 = width as i32;
    let height_i: i32 = height as i32;
    let framebuffer_width_i: i32 = (LIGHTING_WIDTH * scale).round() as i32;
    let framebuffer_height_i: i32 = (LIGHTING_HEIGHT * scale).round() as i32;
    // Lamp rays: lamp -> each of the 5 sphere centres.
    let lamp_screen_x: f64 = LIGHTING_WIDTH * 0.08;
    let lamp_screen_y: f64 = LIGHTING_HEIGHT * 0.18;
    let lamp_color: (u8, u8, u8) = (102, 178, 255); // 0.40, 0.70, 1.00
    for sphere in spheres.iter() {
        let x0: i32 = (lamp_screen_x * scale).round() as i32;
        let y0: i32 = (lamp_screen_y * scale).round() as i32;
        let x1: i32 = (sphere.cx * scale).round() as i32;
        let y1: i32 = (sphere.cy * scale).round() as i32;
        draw_ray_line(
            buffer,
            width_i,
            height_i,
            x0,
            y0,
            x1,
            y1,
            lamp_color,
            framebuffer_width_i,
            framebuffer_height_i,
        );
    }
    // Lamp source disk: bright marker at the lamp position so the
    // user can see the ray origin.
    let disk_r: i32 = (5.0 * scale).round().max(2.0) as i32;
    let cx: i32 = (lamp_screen_x * scale).round() as i32;
    let cy: i32 = (lamp_screen_y * scale).round() as i32;
    for dy in -disk_r..=disk_r {
        for dx in -disk_r..=disk_r {
            if dx * dx + dy * dy <= disk_r * disk_r {
                blend_pixel(
                    buffer,
                    width_i,
                    height_i,
                    cx + dx,
                    cy + dy,
                    (220, 240, 255),
                    framebuffer_width_i,
                    framebuffer_height_i,
                );
            }
        }
    }
    // Sun direction marker (top-right corner): the directional sun
    // has no position, so we draw a small disk at a fixed visible
    // spot and skip rays from it (the directional term in `shade`
    // already encodes the direction). Matches the visible sun in
    // the WebGL / WebGPU shaders.
    let sun_screen_x: f64 = LIGHTING_WIDTH * 0.93;
    let sun_screen_y: f64 = LIGHTING_HEIGHT * 0.12;
    let sun_color: (u8, u8, u8) = (255, 242, 217); // 1.00, 0.95, 0.85
    let sun_r: i32 = (4.0 * scale).round().max(2.0) as i32;
    let sx: i32 = (sun_screen_x * scale).round() as i32;
    let sy: i32 = (sun_screen_y * scale).round() as i32;
    for dy in -sun_r..=sun_r {
        for dx in -sun_r..=sun_r {
            if dx * dx + dy * dy <= sun_r * sun_r {
                blend_pixel(
                    buffer,
                    width_i,
                    height_i,
                    sx + dx,
                    sy + dy,
                    sun_color,
                    framebuffer_width_i,
                    framebuffer_height_i,
                );
            }
        }
    }
}

/// Writes a single ray-coloured pixel using straight-alpha compositing
/// in 8-bit sRGB space. Out-of-bounds pixels (clipped by the letterbox
/// on the CSS box) are silently dropped.
///
/// # Arguments
///
/// - `&mut [u8]` - The RGBA framebuffer (length `width * height * 4`).
/// - `i32` - The framebuffer width in pixels (used as the row stride).
/// - `i32` - The framebuffer height in pixels (used for bounds checks).
/// - `i32` - The destination x coordinate in framebuffer pixels.
/// - `i32` - The destination y coordinate in framebuffer pixels.
/// - `(u8, u8, u8)` - The sRGB color to blend in.
/// - `i32` - The 4:3 letterbox width inside the framebuffer (logical 320 * scale).
/// - `i32` - The 4:3 letterbox height inside the framebuffer (logical 240 * scale).
#[allow(clippy::too_many_arguments)]
fn blend_pixel(
    buffer: &mut [u8],
    width: i32,
    height: i32,
    px: i32,
    py: i32,
    color: (u8, u8, u8),
    letterbox_w: i32,
    letterbox_h: i32,
) {
    if px < 0 || py < 0 || px >= letterbox_w || py >= letterbox_h || px >= width || py >= height {
        return;
    }
    let stride: usize = width as usize;
    let offset: usize = (py as usize) * stride * 4 + (px as usize) * 4;
    if offset + 3 >= buffer.len() {
        return;
    }
    let (src_r, src_g, src_b) = color;
    let alpha: u16 = 102; // ~40% straight alpha
    let inv_alpha: u16 = 255 - alpha;
    buffer[offset] = ((src_r as u16 * alpha + buffer[offset] as u16 * inv_alpha) / 255) as u8;
    buffer[offset + 1] =
        ((src_g as u16 * alpha + buffer[offset + 1] as u16 * inv_alpha) / 255) as u8;
    buffer[offset + 2] =
        ((src_b as u16 * alpha + buffer[offset + 2] as u16 * inv_alpha) / 255) as u8;
    buffer[offset + 3] = 255;
}

/// Bresenham line drawer that paints an RGBA line from `(x0, y0)` to
/// `(x1, y1)` in framebuffer pixels, using [`blend_pixel`] for the
/// straight-alpha compositing pass. The line is one framebuffer-pixel
/// thick (no antialiasing on the line itself — the SSAA letterbox
/// pass in the WebGL / WebGPU shaders handles smoothing there). Out
/// of-bounds pixels are silently clipped.
///
/// # Arguments
///
/// - `&mut [u8]` - The RGBA framebuffer (length `width * height * 4`).
/// - `i32` - The framebuffer width in pixels.
/// - `i32` - The framebuffer height in pixels.
/// - `i32` - Line start x in framebuffer pixels.
/// - `i32` - Line start y in framebuffer pixels.
/// - `i32` - Line end x in framebuffer pixels.
/// - `i32` - Line end y in framebuffer pixels.
/// - `(u8, u8, u8)` - The sRGB color to blend in.
/// - `i32` - The 4:3 letterbox width inside the framebuffer.
/// - `i32` - The 4:3 letterbox height inside the framebuffer.
#[allow(clippy::too_many_arguments)]
fn draw_ray_line(
    buffer: &mut [u8],
    width: i32,
    height: i32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: (u8, u8, u8),
    letterbox_w: i32,
    letterbox_h: i32,
) {
    let dx: i32 = (x1 - x0).abs();
    let dy: i32 = -(y1 - y0).abs();
    let sx: i32 = if x0 < x1 { 1 } else { -1 };
    let sy: i32 = if y0 < y1 { 1 } else { -1 };
    let mut err: i32 = dx + dy;
    let mut cx: i32 = x0;
    let mut cy: i32 = y0;
    loop {
        blend_pixel(
            buffer,
            width,
            height,
            cx,
            cy,
            color,
            letterbox_w,
            letterbox_h,
        );
        if cx == x1 && cy == y1 {
            break;
        }
        let e2: i32 = 2 * err;
        if e2 >= dy {
            err += dy;
            cx += sx;
        }
        if e2 <= dx {
            err += dx;
            cy += sy;
        }
    }
}

/// Uploads the RGBA framebuffer to the canvas in a single
/// `put_image_data` call.
///
/// Replaces the old per-pixel `fillStyle` + `fill_rect` path, which
/// cost one `format!` allocation plus two JS crossings per pixel per
/// frame.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - The target 2D context.
/// - `&mut [u8]` - The RGBA framebuffer (length `width * height * 4`).
/// - `u32` - The framebuffer width in pixels.
/// - `u32` - The framebuffer height in pixels.
fn present_lighting_framebuffer(
    context: &CanvasRenderingContext2d,
    buffer: &mut [u8],
    width: u32,
    height: u32,
) {
    let image_data: Result<ImageData, JsValue> =
        ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(buffer), width, height);
    if let Ok(image_data) = image_data {
        let _: Result<(), JsValue> = context.put_image_data(&image_data, 0.0, 0.0);
    }
}

/// Starts the Lighting Canvas 2D `requestAnimationFrame` loop.
///
/// The scene is fully static and re-shaded into the persistent RGBA
/// framebuffer every frame, then uploaded with one `put_image_data`
/// call. An exponential moving average of the CPU render time drives
/// the [`LIGHTING_RENDER_SCALES`] adaptive-resolution ladder: sustained
/// frames above 115% of the 60 FPS budget step the internal resolution
/// down one rung, sustained frames below 75% step it back up one rung,
/// and sustained frames below 45% step it up two rungs at once. The
/// FPS counter uses unclamped wall-clock elapsed time so it reports
/// honest rates; there is no animation step to clamp for. The
/// `use_cleanup` cancellation and canvas-detached guard mirror the
/// raytrace pattern.
///
/// # Arguments
///
/// - `UseLighting` - The Lighting Canvas 2D tab state.
pub(crate) fn start_lighting_loop(state: UseLighting) {
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let (spheres, lights) = build_lighting_scene();
    let canvas_cache: Rc<RefCell<Option<(HtmlCanvasElement, CanvasRenderingContext2d)>>> =
        Rc::new(RefCell::new(None));
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
    let cache_clone: Rc<RefCell<Option<(HtmlCanvasElement, CanvasRenderingContext2d)>>> =
        canvas_cache.clone();
    let buffer_clone: Rc<RefCell<Vec<u8>>> = framebuffer.clone();
    let scale_clone: Rc<Cell<usize>> = scale_index.clone();
    let ema_clone: Rc<Cell<f64>> = ema_millis.clone();
    let slow_clone: Rc<Cell<u32>> = slow_frames.clone();
    let fast_clone: Rc<Cell<u32>> = fast_frames.clone();
    let very_fast_clone: Rc<Cell<u32>> = very_fast_frames.clone();
    // Paint the loading overlay *before* the first frame so the user
    // sees a centered "Initializing..." line during the Canvas 2D
    // context acquire + first warmup frame. The 200-400 ms window is
    // short enough that the overlay usually disappears in a single
    // frame, but synchronous WASM module init can delay it further on
    // slow devices, and without this paint the canvas stays blank /
    // half-rendered for that entire window.
    // Block-scoped instead of `let-else`: a `let-else` here would
    // return from `start_lighting_loop` early when `window()` is None,
    // skipping the `loop_started` set + `use_cleanup` registration +
    // `request_animation_frame` boot — so the loading overlay would
    // never clear even after the canvas mounts. The raytrace loop
    // uses the same pattern (see raytrace/hook/fn.rs:596).
    if let Some(loading_window) = window() {
        let loading_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            draw_game_3d_loading(LIGHTING_LOADING_CANVAS_SELECTOR, LIGHTING_CANVAS_SELECTOR);
        }));
        let loading_callback: Function =
            loading_closure.as_ref().unchecked_ref::<Function>().clone();
        loading_closure.forget();
        let _ = loading_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
    }
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        if lighting_canvas_detached(LIGHTING_CANVAS_SELECTOR) {
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
        // rate stays honest even when frames take multiple seconds.
        let frame_time: f64 = if prev < 0.0 {
            1.0 / 60.0
        } else {
            current_time - prev
        };
        last_clone.set(current_time);
        if state.get_running().get() {
            let scale: f64 = LIGHTING_RENDER_SCALES[scale_clone.get()];
            let (frame_width, frame_height): (u32, u32) = lighting_scaled_dimensions(scale);
            let mut cache = cache_clone.borrow_mut();
            let cached_valid: bool = cache.as_ref().is_some_and(
                |(canvas, _): &(HtmlCanvasElement, CanvasRenderingContext2d)| canvas.is_connected(),
            );
            if !cached_valid {
                *cache = acquire_lighting_canvas(frame_width, frame_height);
            }
            if let Some((canvas, context)) = cache.as_ref() {
                if canvas.width() != frame_width {
                    canvas.set_width(frame_width);
                }
                if canvas.height() != frame_height {
                    canvas.set_height(frame_height);
                }
                let render_start: f64 = performance.now();
                {
                    let mut buffer = buffer_clone.borrow_mut();
                    let needed: usize = frame_width as usize * frame_height as usize * 4;
                    if buffer.len() != needed {
                        buffer.resize(needed, 0);
                    }
                    render_lighting_frame(
                        &mut buffer,
                        frame_width,
                        frame_height,
                        scale,
                        &spheres,
                        &lights,
                    );
                    present_lighting_framebuffer(context, &mut buffer, frame_width, frame_height);
                }
                let render_millis: f64 = performance.now() - render_start;
                let ema_prev: f64 = ema_clone.get();
                let ema: f64 = if ema_prev <= 0.0 {
                    render_millis
                } else {
                    ema_prev * (1.0 - LIGHTING_ADAPT_EMA_ALPHA)
                        + render_millis * LIGHTING_ADAPT_EMA_ALPHA
                };
                ema_clone.set(ema);
                if ema > LIGHTING_ADAPT_SLOW_FRAME_MILLIS {
                    slow_clone.set(slow_clone.get() + 1);
                    fast_clone.set(0);
                    very_fast_clone.set(0);
                } else if ema < LIGHTING_ADAPT_VERY_FAST_FRAME_MILLIS {
                    fast_clone.set(fast_clone.get() + 1);
                    very_fast_clone.set(very_fast_clone.get() + 1);
                    slow_clone.set(0);
                } else if ema < LIGHTING_ADAPT_FAST_FRAME_MILLIS {
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
                if slow_clone.get() >= LIGHTING_ADAPT_SLOW_FRAMES
                    && index + 1 < LIGHTING_RENDER_SCALES.len()
                {
                    next = index + 1;
                } else if fast_clone.get() >= LIGHTING_ADAPT_FAST_FRAMES && index > 0 {
                    // Sustained headroom far below the budget skips a
                    // rung so strong hardware reaches the sharp 4.0 top
                    // of the ladder in a handful of steps instead of
                    // crawling one rung at a time.
                    next = if very_fast_clone.get() >= LIGHTING_ADAPT_FAST_FRAMES {
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
                    state.get_render_scale().set(LIGHTING_RENDER_SCALES[next]);
                }
                // Flip the active / loaded flags on the first successful
                // frame so the loading overlay unloads and the
                // `Status: ...` banner reports a live renderer. The
                // `loaded` set is delayed by
                // `GAME_3D_LOADING_MIN_MILLIS` so the overlay stays
                // painted for a minimum visible duration even when the
                // Canvas 2D acquire + first warmup frame finishes in
                // less than a frame budget — the same UX the WebGL /
                // WebGPU tabs use via `lighting_set_loaded_delayed`.
                if !state.get_active().get() {
                    state.get_active().set(true);
                    lighting_set_loaded_delayed(state.get_loaded(), GAME_3D_LOADING_MIN_MILLIS);
                }
            }
        }
        frame_clone.set(frame_clone.get() + 1);
        fps_clone.set(fps_clone.get() + frame_time);
        if fps_clone.get() >= 1.0 {
            let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
            state.get_fps().set(fps);
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
            LIGHTING_LOOP_START_DELAY_MILLIS,
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

/// Creates a click handler that toggles a Lighting tab loop between
/// running and paused.
///
/// # Arguments
///
/// - `Signal<bool>` - The running signal of the active tab's loop.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle handler.
pub(crate) fn lighting_on_toggle_pause(running: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = running.get();
        running.set(!current);
    }))
}

/// Packs the per-frame uniform data consumed by the WebGL and WebGPU
/// lighting shaders.
///
/// Layout (2 `vec4` slots, matching `u_params[2]` / `SceneUniforms`):
/// canvas backing resolution and the canvas's computed CSS background
/// color, which the shaders paint behind the letterboxed scene so the
/// result matches the transparent-cleared Canvas 2D tab.
///
/// # Arguments
///
/// - `f64` - The canvas backing width in physical pixels.
/// - `f64` - The canvas backing height in physical pixels.
/// - `(f64, f64, f64)` - The canvas's computed CSS background color.
///
/// # Returns
///
/// - `Vec<f32>` - The packed uniform data (8 floats).
fn pack_lighting_gpu_uniform(width: f64, height: f64, background: (f64, f64, f64)) -> Vec<f32> {
    let (red, green, blue) = background;
    vec![
        width as f32,
        height as f32,
        0.0,
        0.0,
        red as f32,
        green as f32,
        blue as f32,
        1.0,
    ]
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
fn lighting_set_loaded_delayed(loaded: Signal<bool>, millis: i32) {
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
/// and WebGPU lighting loops.
///
/// # Arguments
///
/// - `Rc<Cell<bool>>` - The resize-dirty flag set after the debounce fires.
/// - `Rc<Cell<Option<i32>>>` - The pending debounce timer handle.
fn lighting_register_resize_debounce(
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

/// Starts the Lighting WebGL loop driven by `requestAnimationFrame`.
///
/// Renders the same analytic 2D scene as the Canvas 2D tab through a
/// GLSL ES 3.00 fragment shader on a fullscreen triangle. The canvas
/// backing store tracks the CSS box times the device pixel ratio
/// (synchronous ResizeObserver plus a debounced window-resize flag plus
/// a per-frame divergence check, mirroring the 3D game page), and the
/// shader letterboxes the 4:3 logical scene with a uniform scale so the
/// spheres never stretch. WebGL initialization is synchronous; the
/// `spawn_local` wrapper only defers execution past the current render
/// pass so the canvas element exists in the DOM.
///
/// # Arguments
///
/// - `UseLightingWebGl` - The WebGL backend state for signal updates.
pub(crate) fn start_lighting_webgl_loop(state: UseLightingWebGl) {
    let init_state: UseLightingWebGl = state;
    let loop_state: UseLightingWebGl = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let renderer_rc: Rc<RefCell<Option<WebGlRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let observer_cell: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));
    lighting_register_resize_debounce(resize_dirty.clone(), resize_timer.clone());
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
            LIGHTING_WEBGL_LOADING_CANVAS_SELECTOR,
            LIGHTING_WEBGL_CANVAS_SELECTOR,
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
            LIGHTING_WEBGL_CANVAS_SELECTOR,
            LIGHTING_WIDTH,
            LIGHTING_HEIGHT,
        );
        let renderer: WebGlRenderer = match Engine::webgl_renderer(&config) {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!("[euv-engine][lighting] webgl init failed: {error}"));
                init_state.get_init_error_code().set(error.code());
                init_state.get_loaded().set(true);
                return;
            }
        };
        let program: WebGlProgram = match renderer
            .create_program(LIGHTING_WEBGL_VERTEX_SHADER, LIGHTING_WEBGL_FRAGMENT_SHADER)
        {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!(
                    "[euv-engine][lighting] webgl program failed: {error}"
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
        init_state.get_active().set(true);
        // Delay flipping `loaded` so the loading overlay stays painted for a
        // minimum visible duration even when init completes instantly.
        lighting_set_loaded_delayed(init_state.get_loaded(), GAME_3D_LOADING_MIN_MILLIS);
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
                    .query_selector(LIGHTING_WEBGL_CANVAS_SELECTOR)
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
                .query_selector(LIGHTING_WEBGL_CANVAS_SELECTOR)
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
            if cancelled_for_loop.get() || lighting_canvas_detached(LIGHTING_WEBGL_CANVAS_SELECTOR)
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
            // The unclamped frame time feeds the FPS counter so the
            // reported rate stays honest.
            let frame_time: f64 = if prev < 0.0 {
                1.0 / 60.0
            } else {
                current_time - prev
            };
            last_clone.set(current_time);
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
                read_lighting_canvas_size(LIGHTING_WEBGL_CANVAS_SELECTOR).unwrap_or((0.0, 0.0));
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
                    // Refresh the background color every frame so a theme
                    // toggle takes effect within one paint.
                    let background: (f64, f64, f64) =
                        game_3d_canvas_clear_color(LIGHTING_WEBGL_CANVAS_SELECTOR);
                    let uniform_data: Vec<f32> =
                        pack_lighting_gpu_uniform(backing_w, backing_h, background);
                    renderer.set_uniform_4fv(
                        &program_for_loop,
                        params_location_for_loop.as_ref().as_ref(),
                        &uniform_data,
                    );
                    renderer.render_frame(
                        &program_for_loop,
                        (background.0, background.1, background.2, 1.0),
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

/// Starts the Lighting WebGPU loop driven by `requestAnimationFrame`.
///
/// Renders the same analytic 2D scene as the Canvas 2D tab through a
/// WGSL fragment shader on a fullscreen triangle, fed by a single
/// 2-`vec4` uniform buffer at `@group(0) @binding(0)` updated once per
/// frame. WebGPU initialization is asynchronous (adapter + device
/// promises raced against a timeout inside the engine), so the whole
/// init runs inside `spawn_local` with a cancellation guard for tab
/// switches. On failure the error code is surfaced to the status banner
/// and the loop exits quietly.
///
/// # Arguments
///
/// - `UseLightingWebGpu` - The WebGPU backend state for signal updates.
pub(crate) fn start_lighting_webgpu_loop(state: UseLightingWebGpu) {
    let init_state: UseLightingWebGpu = state;
    let loop_state: UseLightingWebGpu = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let renderer_rc: Rc<RefCell<Option<WebGpuRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let observer_cell: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));
    lighting_register_resize_debounce(resize_dirty.clone(), resize_timer.clone());
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
            LIGHTING_WEBGPU_LOADING_CANVAS_SELECTOR,
            LIGHTING_WEBGPU_CANVAS_SELECTOR,
        );
    }));
    let loading_callback: Function = loading_closure.as_ref().unchecked_ref::<Function>().clone();
    loading_closure.forget();
    let _ =
        loading_window.set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
    spawn_local(async move {
        let config: RenderConfig = RenderConfig::webgpu(
            LIGHTING_WEBGPU_CANVAS_SELECTOR,
            LIGHTING_WIDTH,
            LIGHTING_HEIGHT,
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
                    "[euv-engine][lighting] webgpu init failed: {error}"
                ));
                init_state.get_init_error_code().set(error.code());
                init_state.get_loaded().set(true);
                return;
            }
        };
        let pipeline: JsValue = renderer.create_render_pipeline(LIGHTING_WEBGPU_SHADER);
        let uniform_buffer: JsValue =
            renderer.create_uniform_buffer(&[0.0; LIGHTING_GPU_UNIFORM_VEC4_COUNT * 4]);
        let bind_group: JsValue = renderer.create_uniform_bind_group(&pipeline, &uniform_buffer);
        init_state.get_active().set(true);
        // Delay flipping `loaded` so the loading overlay stays painted for a
        // minimum visible duration even when init completes instantly.
        lighting_set_loaded_delayed(init_state.get_loaded(), GAME_3D_LOADING_MIN_MILLIS);
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
                    .query_selector(LIGHTING_WEBGPU_CANVAS_SELECTOR)
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
                .query_selector(LIGHTING_WEBGPU_CANVAS_SELECTOR)
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
            if cancelled_for_loop.get() || lighting_canvas_detached(LIGHTING_WEBGPU_CANVAS_SELECTOR)
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
            // The unclamped frame time feeds the FPS counter so the
            // reported rate stays honest.
            let frame_time: f64 = if prev < 0.0 {
                1.0 / 60.0
            } else {
                current_time - prev
            };
            last_clone.set(current_time);
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
                read_lighting_canvas_size(LIGHTING_WEBGPU_CANVAS_SELECTOR).unwrap_or((0.0, 0.0));
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
                    // Refresh the background color every frame so a theme
                    // toggle takes effect within one paint.
                    let background: (f64, f64, f64) =
                        game_3d_canvas_clear_color(LIGHTING_WEBGPU_CANVAS_SELECTOR);
                    let uniform_data: Vec<f32> =
                        pack_lighting_gpu_uniform(backing_w, backing_h, background);
                    renderer.update_uniform_buffer(&buffer_for_loop, &uniform_data);
                    renderer.render_frame_with_bind_group(
                        &pipeline_for_loop,
                        &bind_group_for_loop,
                        (background.0, background.1, background.2, 1.0),
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
/// - `Signal<LightingTab>` - The tab signal to update.
/// - `LightingTab` - The tab variant to set.
/// - `UseLightingFullscreen` - The fullscreen state to clear on switch.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the active
///   tab and clears any active fullscreen mode.
pub(crate) fn lighting_on_tab_select(
    tab: Signal<LightingTab>,
    value: LightingTab,
    fullscreen: UseLightingFullscreen,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        fullscreen.get_canvas_2d().set(false);
        fullscreen.get_web_gl().set(false);
        fullscreen.get_web_gpu().set(false);
        tab.set(value);
    }))
}

/// Enters landscape fullscreen mode for the Lighting page on the active
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
pub(crate) fn enter_lighting_fullscreen(tab: Signal<bool>) {
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

/// Exits landscape fullscreen mode for the Lighting page on the active
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
pub(crate) fn exit_lighting_fullscreen(tab: Signal<bool>) {
    tab.set(false);
    UseEuvLayout::apply_cached_insets();
    // See `enter_lighting_fullscreen` - dispatch a synthetic `resize`
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
pub(crate) fn exit_lighting_fullscreen_from_popstate(tab: Signal<bool>) {
    tab.set(false);
    UseEuvLayout::apply_cached_insets();
    // See `enter_lighting_fullscreen` for why we dispatch a synthetic
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
/// button while the Lighting page is in landscape fullscreen mode.
///
/// Watches all three tab-specific fullscreen signals in a fixed order
/// (Canvas 2D, WebGL, WebGPU). When any one is `true`, the corresponding
/// `exit_lighting_fullscreen_from_popstate` runs and the guard returns
/// `true` to consume the `popstate` event. Otherwise returns `false` so
/// the overlay stack or router can handle the back navigation normally.
///
/// Returns the guard ID so the page can unregister it on unmount.
///
/// # Arguments
///
/// - `UseLightingFullscreen` - The Lighting page fullscreen state.
///
/// # Returns
///
/// - `usize` - The popstate guard ID.
pub(crate) fn use_lighting_fullscreen_popstate(state: UseLightingFullscreen) -> usize {
    Router::register_popstate_guard(Rc::new(move || {
        if state.get_canvas_2d().get() {
            exit_lighting_fullscreen_from_popstate(state.get_canvas_2d());
            true
        } else if state.get_web_gl().get() {
            exit_lighting_fullscreen_from_popstate(state.get_web_gl());
            true
        } else if state.get_web_gpu().get() {
            exit_lighting_fullscreen_from_popstate(state.get_web_gpu());
            true
        } else {
            false
        }
    }))
}

/// Creates a click event handler that enters landscape fullscreen mode
/// for the Lighting page.
///
/// Delegates to [`enter_lighting_fullscreen`], which sets the active
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
pub(crate) fn lighting_on_enter_fullscreen(tab: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        enter_lighting_fullscreen(tab);
    }))
}

/// Creates a click event handler that exits landscape fullscreen mode
/// for the Lighting page.
///
/// Delegates to [`exit_lighting_fullscreen`], which clears the active
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
pub(crate) fn lighting_on_exit_fullscreen(tab: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        exit_lighting_fullscreen(tab);
        Router::overlay_back(None);
    }))
}
