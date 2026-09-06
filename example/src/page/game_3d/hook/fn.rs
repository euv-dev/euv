use super::*;

/// Reads the canvas element's CSS layout dimensions.
///
/// Uses `getBoundingClientRect()` to read the actual CSS box size
/// (width/height after layout), not `clientWidth`/`clientHeight` which
/// in Chrome track `canvas.width`/`canvas.height` (the backing-store
/// size). Reading the CSS box is critical during fullscreen
/// transitions: the moment the user clicks Enter Fullscreen the CSS
/// layout flips to the new size, but the canvas backing store still
/// holds the previous size. If we used `clientWidth` here, the
/// `WebGlRenderer::resize` / `WebGpuRenderer::resize` calls driven by
/// the debounced resize tick would receive the OLD backing dimensions
/// (already matching `canvas.width`), `if` check passes, no resize
/// happens, and the browser stretches the previous-size backing image
/// into the new CSS box - producing a visible first-frame cube
/// distortion that only recovers once the next debounced resize tick
/// reads the new CSS dimensions. `getBoundingClientRect` returns the
/// target CSS size immediately so the per-frame safety net in
/// `start_game_3d_webgl_loop` / `start_game_3d_webgpu_loop` can resize
/// the backing store to match on the very first frame.
///
/// # Arguments
///
/// - `&str` - The CSS selector for the canvas element.
///
/// # Returns
///
/// - `Option<(f64, f64)>` - The (width, height) in CSS pixels.
pub(crate) fn read_canvas_size(canvas_selector: &str) -> Option<(f64, f64)> {
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

/// Creates the 3D game reactive state signals wrapped in a `UseGame3D` struct.
///
/// # Returns
///
/// - `UseGame3D` - The 3D game state.
pub(crate) fn use_game_3d_state() -> UseGame3D {
    UseGame3D {
        running: App::use_signal(|| true),
        fps: App::use_signal(|| 0.0),
        cube_count: App::use_signal(|| 0),
        auto_rotate: App::use_signal(|| true),
        loaded: App::use_signal(|| false),
    }
}

/// Creates the initial set of cubes for the 3D scene.
///
/// # Returns
///
/// - `Vec<Cube3D>` - The initial cube list.
pub(crate) fn create_initial_cubes() -> Vec<Cube3D> {
    vec![
        Cube3D {
            position: Vector3D::new(0.0, 0.0, 0.0),
            rotation: Quaternion::identity(),
            angular_velocity: Vector3D::new(0.5, 1.0, 0.3),
            scale: 1.5,
            face_color: GAME_3D_CUBE_FACE_COLOR.to_string(),
            edge_color: GAME_3D_CUBE_EDGE_COLOR.to_string(),
        },
        Cube3D {
            position: Vector3D::new(-3.0, 0.0, 0.0),
            rotation: Quaternion::from_euler(0.0, 0.5, 0.0),
            angular_velocity: Vector3D::new(0.3, -0.7, 0.5),
            scale: 0.8,
            face_color: "#6c5ce7".to_string(),
            edge_color: "#fd79a8".to_string(),
        },
        Cube3D {
            position: Vector3D::new(3.0, 0.0, 0.0),
            rotation: Quaternion::from_euler(0.5, 0.0, 0.0),
            angular_velocity: Vector3D::new(-0.4, 0.6, -0.2),
            scale: 0.8,
            face_color: "#f5b461".to_string(),
            edge_color: "#00cec9".to_string(),
        },
        Cube3D {
            position: Vector3D::new(0.0, 2.5, 0.0),
            rotation: Quaternion::identity(),
            angular_velocity: Vector3D::new(0.8, 0.2, -0.6),
            scale: 0.6,
            face_color: "#ec524b".to_string(),
            edge_color: "#41b883".to_string(),
        },
    ]
}

/// Creates a `Camera3D` from the current yaw and pitch orbit angles.
///
/// The camera's aspect ratio is set to `canvas_width / canvas_height`
/// so the cube projection fills the entire canvas in both inline
/// (~820x547, 1.5:1) and fullscreen (~1248x750, 1.66:1) layouts. Using
/// runtime dimensions instead of the static 600x400 default keeps the
/// cubes from appearing compressed / letterboxed in fullscreen.
///
/// # Arguments
///
/// - `f64` - The orbit yaw angle in radians.
/// - `f64` - The orbit pitch angle in radians.
/// - `f64` - The canvas width in CSS pixels (camera X extent).
/// - `f64` - The canvas height in CSS pixels (camera Y extent).
///
/// # Returns
///
/// - `Camera3D` - The configured camera.
pub(crate) fn create_orbit_camera(
    yaw: f64,
    pitch: f64,
    canvas_width: f64,
    canvas_height: f64,
) -> Camera3D {
    let cos_pitch: f64 = pitch.cos();
    let position: Vector3D = Vector3D::new(
        GAME_3D_CAMERA_DISTANCE * yaw.sin() * cos_pitch,
        GAME_3D_CAMERA_DISTANCE * pitch.sin(),
        GAME_3D_CAMERA_DISTANCE * yaw.cos() * cos_pitch,
    );
    Camera3D::create(position, Vector3D::zero(), canvas_width, canvas_height)
}

/// Transforms a cube's local vertex to world space.
///
/// # Arguments
///
/// - `&Cube3D` - The cube instance.
/// - `Vector3D` - The local-space vertex.
///
/// # Returns
///
/// - `Vector3D` - The world-space vertex.
pub(crate) fn transform_cube_vertex(cube: &Cube3D, local: Vector3D) -> Vector3D {
    let scaled: Vector3D = Vector3D::new(
        local.get_x() * cube.scale * GAME_3D_CUBE_HALF_SIZE,
        local.get_y() * cube.scale * GAME_3D_CUBE_HALF_SIZE,
        local.get_z() * cube.scale * GAME_3D_CUBE_HALF_SIZE,
    );
    scaled.rotated_by(cube.rotation) + cube.position
}

/// Computes the average depth of a cube face's vertices in camera space.
///
/// # Arguments
///
/// - `&[Vector3D]` - The world-space vertices of the face.
/// - `&Camera3D` - The camera.
///
/// # Returns
///
/// - `f64` - The average z depth (negative is farther away).
pub(crate) fn face_average_depth(world_vertices: &[Vector3D], camera: &Camera3D) -> f64 {
    let view_matrix: Matrix4x4 = camera.view_matrix();
    let mut sum_z: f64 = 0.0;
    for vertex in world_vertices {
        let view_vertex: Vector3D = view_matrix.transform_point(*vertex);
        sum_z += view_vertex.get_z();
    }
    sum_z / world_vertices.len() as f64
}

/// Computes the normal of a cube face using the cross product of two edges.
///
/// # Arguments
///
/// - `&[Vector3D]` - The world-space vertices of the face (at least 3).
///
/// # Returns
///
/// - `Vector3D` - The face normal.
pub(crate) fn face_normal(world_vertices: &[Vector3D]) -> Vector3D {
    let edge_a: Vector3D = world_vertices[1] - world_vertices[0];
    let edge_b: Vector3D = world_vertices[2] - world_vertices[0];
    edge_a.cross(edge_b).normalized()
}

/// Determines whether a face is visible (back-face culling).
///
/// # Arguments
///
/// - `&[Vector3D]` - The world-space vertices of the face.
/// - `&Camera3D` - The camera.
///
/// # Returns
///
/// - `bool` - True if the face should be rendered.
pub(crate) fn is_face_visible(world_vertices: &[Vector3D], camera: &Camera3D) -> bool {
    let normal: Vector3D = face_normal(world_vertices);
    let face_center: Vector3D = world_vertices
        .iter()
        .fold(Vector3D::zero(), |acc: Vector3D, vertex: &Vector3D| {
            acc + *vertex
        })
        .scaled(1.0 / world_vertices.len() as f64);
    let view_direction: Vector3D = (face_center - camera.get_position()).normalized();
    normal.dot(view_direction) < 0.0
}

/// Renders the 3D scene onto the SSAA offscreen canvas and presents it to the display.
///
/// Clears the offscreen canvas to transparency so the CSS `background`
/// property (set to `var(--accent)`) shows through on the display canvas,
/// draws the world axes, then for each cube (back-to-front via painter's
/// algorithm) draws the visible face fills and finally the unique visible
/// edges as a separate wireframe pass. The fill/stroke separation avoids
/// stroking each shared cube edge twice (which would otherwise appear as
/// thicker lines near the inner corner where three visible faces meet).
/// Calls `present()` to downscale the high-resolution buffer onto the
/// visible canvas with high-quality image smoothing for SSAA anti-aliasing.
///
/// # Arguments
///
/// - `&SsaaCanvas` - The SSAA canvas wrapper.
/// - `&[Cube3D]` - The cube list to render.
/// - `&Camera3D` - The camera.
/// - `f64` - The canvas width in CSS pixels (clear-rect bound).
/// - `f64` - The canvas height in CSS pixels (clear-rect bound).
pub(crate) fn render_scene(
    ssaa_canvas: &SsaaCanvas,
    cubes: &[Cube3D],
    camera: &Camera3D,
    canvas_width: f64,
    canvas_height: f64,
) {
    let context: &CanvasRenderingContext2d = ssaa_canvas.get_offscreen_context();
    // Clear the entire backing buffer (not just the static 600x400
    // default) so cubes in fullscreen mode don't leave ghost trails
    // outside the cleared rect. Without this, `clear_rect` would
    // erase the top-left 600x400 region and leave the rest of the
    // larger fullscreen canvas with its previous frame's content
    // visible — appearing as a "history trail" of past cube
    // positions.
    context.clear_rect(0.0, 0.0, canvas_width, canvas_height);
    let mut cube_batches: Vec<(f64, &Cube3D, Vec<Vector3D>)> = cubes
        .iter()
        .map(|cube: &Cube3D| {
            let world_vertices: Vec<Vector3D> = GAME_3D_CUBE_VERTICES
                .iter()
                .map(|(vx, vy, vz): &(f64, f64, f64)| {
                    transform_cube_vertex(cube, Vector3D::new(*vx, *vy, *vz))
                })
                .collect();
            let depth: f64 = face_average_depth(&world_vertices, camera);
            (depth, cube, world_vertices)
        })
        .collect();
    cube_batches.sort_by(
        |a: &(f64, &Cube3D, Vec<Vector3D>), b: &(f64, &Cube3D, Vec<Vector3D>)| {
            a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal)
        },
    );
    for (_cube_depth, cube, world_vertices) in &cube_batches {
        let mut face_batches: Vec<(f64, Vec<Vector3D>)> = Vec::new();
        for (i0, i1, i2, i3) in GAME_3D_CUBE_FACES {
            let face_world: Vec<Vector3D> = vec![
                world_vertices[i0],
                world_vertices[i1],
                world_vertices[i2],
                world_vertices[i3],
            ];
            if !is_face_visible(&face_world, camera) {
                continue;
            }
            let depth: f64 = face_average_depth(&face_world, camera);
            face_batches.push((depth, face_world));
        }
        face_batches.sort_by(|a: &(f64, Vec<Vector3D>), b: &(f64, Vec<Vector3D>)| {
            a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal)
        });
        let _ = Reflect::set(
            context,
            &JsValue::from_str(GAME_3D_PROPERTY_FILL_STYLE),
            &JsValue::from_str(&cube.face_color),
        );
        for (_depth, face_world) in &face_batches {
            let screen_vertices: Vec<Vector3D> = face_world
                .iter()
                .map(|world: &Vector3D| camera.world_to_screen(*world))
                .collect();
            context.begin_path();
            context.move_to(screen_vertices[0].get_x(), screen_vertices[0].get_y());
            for screen_vertex in screen_vertices.iter().skip(1) {
                context.line_to(screen_vertex.get_x(), screen_vertex.get_y());
            }
            context.close_path();
            context.fill();
        }
        let visible_edges: Vec<(usize, usize)> = collect_visible_edges(world_vertices, camera);
        let _ = Reflect::set(
            context,
            &JsValue::from_str(GAME_3D_PROPERTY_STROKE_STYLE),
            &JsValue::from_str(&cube.edge_color),
        );
        context.set_line_width(1.5);
        context.set_line_join("miter");
        for (i_a, i_b) in &visible_edges {
            let v_a: Vector3D = world_vertices[*i_a];
            let v_b: Vector3D = world_vertices[*i_b];
            let s_a: Vector3D = camera.world_to_screen(v_a);
            let s_b: Vector3D = camera.world_to_screen(v_b);
            context.begin_path();
            context.move_to(s_a.get_x(), s_a.get_y());
            context.line_to(s_b.get_x(), s_b.get_y());
            context.stroke();
        }
    }
    ssaa_canvas.present();
}

/// Collects the unique edges of a cube that belong to at least one
/// visible (front-facing) face.
///
/// Iterates the 12 cube edges in `GAME_3D_CUBE_EDGES` and returns those
/// that are referenced by a face passing the back-face culling test. The
/// returned edges are deduplicated (an edge shared by two visible faces
/// appears only once) so the wireframe pass strokes each silhouette edge
/// exactly once, avoiding the doubled strokes that would otherwise appear
/// as "extra lines" at the inner corner of a cube's visible silhouette.
///
/// # Arguments
///
/// - `&[Vector3D]` - The cube's 8 world-space vertex positions.
/// - `&Camera3D` - The camera used for back-face culling.
///
/// # Returns
///
/// - `Vec<(usize, usize)>` - The list of unique visible edge index pairs.
fn collect_visible_edges(world_vertices: &[Vector3D], camera: &Camera3D) -> Vec<(usize, usize)> {
    let mut visible_face_edges: HashSet<(usize, usize)> = HashSet::new();
    for (i0, i1, i2, i3) in GAME_3D_CUBE_FACES {
        let face_world: Vec<Vector3D> = vec![
            world_vertices[i0],
            world_vertices[i1],
            world_vertices[i2],
            world_vertices[i3],
        ];
        if !is_face_visible(&face_world, camera) {
            continue;
        }
        let mut add = |a: usize, b: usize| {
            let key: (usize, usize) = if a < b { (a, b) } else { (b, a) };
            visible_face_edges.insert(key);
        };
        add(i0, i1);
        add(i1, i2);
        add(i2, i3);
        add(i3, i0);
    }
    GAME_3D_CUBE_EDGES
        .iter()
        .copied()
        .filter(|(a, b): &(usize, usize)| {
            let key: (usize, usize) = if a < b { (*a, *b) } else { (*b, *a) };
            visible_face_edges.contains(&key)
        })
        .collect()
}

/// Performs one physics update step on all cubes.
///
/// Integrates angular velocity into quaternion rotation for each cube.
///
/// # Arguments
///
/// - `&mut [Cube3D]` - The mutable cube slice.
/// - `f64` - The delta time in seconds.
pub(crate) fn update_cubes(cubes: &mut [Cube3D], delta_time: f64) {
    for cube in cubes.iter_mut() {
        let rotation_delta: Quaternion = Quaternion::new(
            cube.angular_velocity.get_x() * delta_time * 0.5,
            cube.angular_velocity.get_y() * delta_time * 0.5,
            cube.angular_velocity.get_z() * delta_time * 0.5,
            1.0,
        );
        cube.rotation = (rotation_delta * cube.rotation).normalized();
    }
}

/// Snapshots the current cube rotations into the previous-step buffer.
///
/// Truncates or extends the buffer to match the cube list so index `i`
/// always pairs with cube `i`.
///
/// # Arguments
///
/// - `&mut Vec<Quaternion>` - The previous-step rotation buffer to overwrite.
/// - `&[Cube3D]` - The current cube list.
pub(crate) fn snapshot_cube_rotations(prev_rotations: &mut Vec<Quaternion>, cubes: &[Cube3D]) {
    prev_rotations.truncate(cubes.len());
    for (index, cube) in cubes.iter().enumerate() {
        if index < prev_rotations.len() {
            prev_rotations[index] = cube.rotation;
        } else {
            prev_rotations.push(cube.rotation);
        }
    }
}

/// Builds a render copy of the cube list with rotations interpolated between
/// the previous physics step and the current one via `Quaternion::slerp`.
///
/// `alpha` is the leftover accumulator fraction (`accumulator / timestep`)
/// clamped to `[0.0, 1.0]`. Interpolating at render time decouples the
/// 60 Hz physics cadence from the display refresh rate: without it a 120 Hz
/// display presents each physics state twice (visible stepping), and a 60 Hz
/// display alternates zero- and double-step frames (visible judder), even
/// though the FPS counter reads high in both cases. Cubes without a previous
/// entry render at their current rotation.
///
/// # Arguments
///
/// - `&[Cube3D]` - The current cube list.
/// - `&[Quaternion]` - The previous-step rotation buffer.
/// - `f64` - The interpolation factor in `[0.0, 1.0]`.
///
/// # Returns
///
/// - `Vec<Cube3D>` - The interpolated cube list for rendering.
pub(crate) fn interpolate_cubes(
    cubes: &[Cube3D],
    prev_rotations: &[Quaternion],
    alpha: f64,
) -> Vec<Cube3D> {
    cubes
        .iter()
        .enumerate()
        .map(|(index, cube): (usize, &Cube3D)| {
            let mut render_cube: Cube3D = cube.clone();
            if let Some(prev_rotation) = prev_rotations.get(index) {
                render_cube.rotation = prev_rotation.slerp(cube.rotation, alpha);
            }
            render_cube
        })
        .collect()
}

/// Queries the canvas element and creates an `SsaaCanvas` for high-quality rendering.
///
/// Reads `canvas.clientWidth` and `canvas.clientHeight` from the live
/// DOM so the SSAA backing buffer tracks the canvas's actual rendered
/// size in both inline (~820x547) and fullscreen (~1248x750) layouts on
/// a 1280x800 viewport. Cubes are then projected with the same runtime
/// dimensions (see `Camera3D::create` and the WebGPU/WebGL resize
/// blocks), so cube movement fills the full canvas in fullscreen mode
/// instead of being bounded by the static 600x400 default.
///
/// # Returns
///
/// - `Option<SsaaCanvas>` - The SSAA canvas, or `None` if unavailable.
pub(crate) fn acquire_game_3d_ssaa_canvas() -> Option<SsaaCanvas> {
    let window_value: Window = window()?;
    let is_mobile: bool = window_value
        .inner_width()
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .is_some_and(|width: f64| width < 768.0);
    let scale_factor: f64 = if is_mobile { 1.0 } else { 2.0 };
    let (canvas_width, canvas_height): (f64, f64) = read_canvas_size(GAME_3D_CANVAS_SELECTOR)?;
    SsaaCanvas::from_selector_with_scale(
        GAME_3D_CANVAS_SELECTOR,
        canvas_width,
        canvas_height,
        scale_factor,
    )
}

/// Registers non-passive event listeners directly on the given 3D canvas
/// element to prevent the page from scrolling when the mouse wheel or touch
/// gesture is used over the canvas.
///
/// The framework's event delegation system registers bubbling events on
/// `window` with the capture phase, which Chrome treats as passive by
/// default for `wheel`, `touchstart`, and `touchmove` events, making
/// `preventDefault()` ineffective. This function bypasses the framework and
/// attaches listeners directly on the element, where `preventDefault()`
/// works correctly. On desktop this prevents wheel scrolling; on mobile this
/// prevents touch scrolling as a belt-and-suspenders complement to the
/// `touch-action: none` CSS property.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the canvas element to guard.
///
/// # Returns
///
/// - `Option<CanvasGuardEntry>` - The listener closures and element for cleanup, or `None` if the canvas was not found.
pub(crate) fn register_canvas_scroll_guard(canvas_selector: &str) -> Option<CanvasGuardEntry> {
    let window: Window = window()?;
    let document: Document = window.document()?;
    let canvas: Element = document.query_selector(canvas_selector).ok().flatten()?;
    let wheel_closure: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
        event.prevent_default();
    }));
    let _ = canvas.add_event_listener_with_callback(
        GAME_3D_EVENT_WHEEL,
        wheel_closure.as_ref().unchecked_ref(),
    );
    let touch_start_closure: Closure<dyn FnMut(Event)> =
        Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
        }));
    let _ = canvas.add_event_listener_with_callback(
        GAME_3D_EVENT_TOUCH_START,
        touch_start_closure.as_ref().unchecked_ref(),
    );
    let touch_move_closure: Closure<dyn FnMut(Event)> =
        Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
        }));
    let _ = canvas.add_event_listener_with_callback(
        GAME_3D_EVENT_TOUCH_MOVE,
        touch_move_closure.as_ref().unchecked_ref(),
    );
    Some((
        vec![
            (wheel_closure, GAME_3D_EVENT_WHEEL),
            (touch_start_closure, GAME_3D_EVENT_TOUCH_START),
            (touch_move_closure, GAME_3D_EVENT_TOUCH_MOVE),
        ],
        canvas,
    ))
}

/// Draws the loading text centered on the 3D game canvas using SSAA.
///
/// Called during the startup delay before the game loop begins, so the
/// canvas shows a loading message instead of being blank. Uses an
/// `SsaaCanvas` with a 2x scale factor on desktop and 1x on mobile for
/// crisp text rendering.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `&str` - Shared reference to a `str`.
pub(crate) fn draw_game_3d_loading(target_selector: &str, color_source_selector: &str) {
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let is_mobile: bool = window_value
        .inner_width()
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .is_some_and(|width: f64| width < 768.0);
    let scale_factor: f64 = if is_mobile { 1.0 } else { 2.0 };
    let Some((canvas_width, canvas_height)) = read_canvas_size(target_selector) else {
        return;
    };
    let Some(ssaa_canvas) = SsaaCanvas::from_selector_with_scale(
        target_selector,
        canvas_width,
        canvas_height,
        scale_factor,
    ) else {
        return;
    };
    let context: &CanvasRenderingContext2d = ssaa_canvas.get_offscreen_context();
    context.clear_rect(0.0, 0.0, canvas_width, canvas_height);
    let fill_style_key: JsValue = JsValue::from_str(GAME_3D_PROPERTY_FILL_STYLE);
    // Read the computed style of the source element once so the theme
    // variables (defined on a parent container, not on the document root)
    // are inherited correctly.
    let Some(document_value): Option<Document> = window_value.document() else {
        return;
    };
    let computed_style: Option<CssStyleDeclaration> = document_value
        .query_selector(color_source_selector)
        .ok()
        .flatten()
        .and_then(|element: Element| window_value.get_computed_style(&element).ok().flatten());
    // Fill the canvas background colour first so the loading state reads as
    // a solid screen and the scene behind the overlay does not bleed through.
    let background_color: String = computed_style
        .as_ref()
        .and_then(|style: &CssStyleDeclaration| {
            style
                .get_property_value(GAME_3D_PROPERTY_BACKGROUND_COLOR)
                .ok()
        })
        .unwrap_or_default();
    if !background_color.is_empty() {
        let _ = Reflect::set(
            context,
            &fill_style_key,
            &JsValue::from_str(&background_color),
        );
        context.fill_rect(0.0, 0.0, canvas_width, canvas_height);
    }
    let font_size: f64 = canvas_height * GAME_3D_LOADING_FONT_SIZE_RATIO;
    let font: String = format!("{font_size}px {GAME_3D_LOADING_FONT_FAMILY}");
    // Read the loading text color from the CSS variable via getComputedStyle.
    let loading_color: String = computed_style
        .and_then(|style: CssStyleDeclaration| {
            style.get_property_value(GAME_3D_LOADING_COLOR_VAR).ok()
        })
        .filter(|color: &String| !color.is_empty())
        .unwrap_or_else(|| "#ffffff".to_string());
    let _ = Reflect::set(context, &fill_style_key, &JsValue::from_str(&loading_color));
    context.set_font(&font);
    context.set_text_align("center");
    context.set_text_baseline("middle");
    let _ = context.fill_text(
        GAME_3D_LOADING_TEXT,
        canvas_width * 0.5,
        canvas_height * 0.5,
    );
    ssaa_canvas.present();
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
fn set_loaded_delayed(loaded: Signal<bool>, millis: i32) {
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
pub(crate) fn game_3d_canvas_detached(canvas_selector: &str) -> bool {
    window()
        .and_then(|window_value: Window| window_value.document())
        .and_then(|document: Document| document.query_selector(canvas_selector).ok().flatten())
        .is_none()
}

/// Starts the 3D game loop driven by `requestAnimationFrame`.
///
/// Runs a fixed-timestep accumulator loop that updates cube rotation at a
/// constant rate and renders every frame, interpolating cube rotations
/// between the previous and current physics steps so motion stays smooth at
/// any display refresh rate. The canvas context is cached
/// once at startup. Updates the FPS signal approximately every second.
///
/// # Arguments
///
/// - `UseGame3D` - The game state for signal updates.
/// - `Rc<RefCell<Vec<Cube3D>>>` - The shared cube list.
/// - `CameraAngles` - The non-reactive camera orbit angles.
pub(crate) fn start_game_3d_loop(
    state: UseGame3D,
    cubes: Rc<RefCell<Vec<Cube3D>>>,
    angles: CameraAngles,
) {
    let canvas_ssaa: Rc<RefCell<Option<SsaaCanvas>>> = Rc::new(RefCell::new(None));
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let accumulator: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let guard_cell: CanvasGuardCell = Rc::new(RefCell::new(None));
    let acc_clone: Rc<Cell<f64>> = accumulator.clone();
    let last_clone: Rc<Cell<f64>> = last_time.clone();
    let frame_clone: Rc<Cell<u32>> = frame_count.clone();
    let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
    let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_clone: RafClosureCell = closure_cell.clone();
    let context_clone: Rc<RefCell<Option<SsaaCanvas>>> = canvas_ssaa.clone();
    let dirty_clone: Rc<Cell<bool>> = resize_dirty.clone();
    let prev_rotations: Rc<RefCell<Vec<Quaternion>>> = Rc::new(RefCell::new(Vec::new()));
    let prev_clone: Rc<RefCell<Vec<Quaternion>>> = prev_rotations.clone();
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        if game_3d_canvas_detached(GAME_3D_CANVAS_SELECTOR) {
            // The page or tab was navigated away from: cleanups only fire
            // on match-arm switches, so stop here instead of simulating
            // and rendering against a detached canvas forever.
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
            GAME_3D_FIXED_TIMESTEP
        } else {
            (current_time - prev).min(0.25)
        };
        last_clone.set(current_time);
        if state.get_running().get() {
            // Accumulate only while running: a paused accumulator would grow
            // unboundedly and burst catch-up physics steps on resume.
            acc_clone.set(acc_clone.get() + frame_time);
            if state.get_auto_rotate().get() {
                let yaw: f64 = angles.yaw.get() + GAME_3D_AUTO_YAW_SPEED * frame_time;
                angles.yaw.set(yaw);
            }
            while acc_clone.get() >= GAME_3D_FIXED_TIMESTEP {
                snapshot_cube_rotations(&mut prev_clone.borrow_mut(), &cubes.borrow());
                update_cubes(&mut cubes.borrow_mut(), GAME_3D_FIXED_TIMESTEP);
                acc_clone.set(acc_clone.get() - GAME_3D_FIXED_TIMESTEP);
            }
        }
        let alpha: f64 = (acc_clone.get() / GAME_3D_FIXED_TIMESTEP).clamp(0.0, 1.0);
        if dirty_clone.get() {
            *context_clone.borrow_mut() = None;
            dirty_clone.set(false);
        }
        if context_clone.borrow().is_none() {
            *context_clone.borrow_mut() = acquire_game_3d_ssaa_canvas();
        }
        if let Some(ssaa_canvas) = context_clone.borrow().as_ref() {
            // Read the live canvas dimensions every frame so the camera
            // aspect ratio (and SSAA backing buffer) tracks the canvas's
            // actual rendered size in both inline and fullscreen
            // layouts. The acquire path above already swapped from
            // `GAME_3D_CANVAS_WIDTH/HEIGHT` constants to these runtime
            // dimensions, so this lookup completes the loop.
            let (canvas_width, canvas_height): (f64, f64) =
                read_canvas_size(GAME_3D_CANVAS_SELECTOR).unwrap_or((0.0, 0.0));
            let camera: Camera3D = create_orbit_camera(
                angles.yaw.get(),
                angles.pitch.get(),
                canvas_width,
                canvas_height,
            );
            let render_cubes: Vec<Cube3D> =
                interpolate_cubes(&cubes.borrow(), &prev_clone.borrow(), alpha);
            render_scene(
                ssaa_canvas,
                &render_cubes,
                &camera,
                canvas_width,
                canvas_height,
            );
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
    let guard_for_start: CanvasGuardCell = guard_cell.clone();
    let state_for_start: UseGame3D = state;
    let start_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        state_for_start.get_loaded().set(true);
        *guard_for_start.borrow_mut() = register_canvas_scroll_guard(GAME_3D_CANVAS_SELECTOR);
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
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let timeout_id: i32 = window_value
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            &start_callback,
            GAME_3D_LOOP_START_DELAY_MILLIS,
        )
        .unwrap_or_default();
    start_timeout_clone.set(Some(timeout_id));
    let loading_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        draw_game_3d_loading(GAME_3D_CANVAS_SELECTOR, GAME_3D_CANVAS_SELECTOR);
    }));
    let loading_callback: Function = loading_closure.as_ref().unchecked_ref::<Function>().clone();
    loading_closure.forget();
    let _ =
        window_value.set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
    let debounce_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let dirty_for_event: Rc<Cell<bool>> = resize_dirty.clone();
    let timer_for_event: Rc<Cell<Option<i32>>> = debounce_timer.clone();
    let debounce_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        dirty_for_event.set(true);
    }));
    let debounce_callback: Function = debounce_closure
        .as_ref()
        .unchecked_ref::<Function>()
        .clone();
    debounce_closure.forget();
    let Some(timeout_window): Option<Window> = window() else {
        return;
    };
    App::use_window_event("resize", move || {
        let old_timer: Option<i32> = timer_for_event.get();
        if let Some(timer_id) = old_timer {
            timeout_window.clear_timeout_with_handle(timer_id);
        }
        let new_timer: i32 = timeout_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &debounce_callback,
                GAME_3D_RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        timer_for_event.set(Some(new_timer));
    });
    let guard_for_cleanup: CanvasGuardCell = guard_cell.clone();
    App::use_cleanup(move || {
        if let Some(cancel_id) = raf_id.get() {
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
        if let Some(timer_id) = debounce_timer.get() {
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            window_value.clear_timeout_with_handle(timer_id);
        }
        let _: Option<_> = closure_cell.try_take();
        if let Some((listeners, element)) = guard_for_cleanup.borrow_mut().take() {
            for (closure, event_name) in listeners {
                let _ = element.remove_event_listener_with_callback(
                    event_name,
                    closure.as_ref().unchecked_ref(),
                );
            }
        }
    });
}

/// Creates a click event handler that toggles the game between running and paused.
///
/// # Arguments
///
/// - `UseGame3D` - The game state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_3d_on_toggle_pause(state: UseGame3D) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_running().get();
        state.get_running().set(!current);
    }))
}

/// Creates a click event handler that toggles auto-rotation.
///
/// # Arguments
///
/// - `UseGame3D` - The game state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_3d_on_toggle_auto_rotate(state: UseGame3D) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_auto_rotate().get();
        state.get_auto_rotate().set(!current);
    }))
}

/// Creates a click event handler that resets the camera orbit angles.
///
/// # Arguments
///
/// - `CameraAngles` - The non-reactive camera orbit angles.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_3d_on_reset_camera(angles: CameraAngles) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        angles.yaw.set(0.3);
        angles.pitch.set(0.4);
    }))
}

/// Creates a pointer event handler that updates orbit angles based on drag movement.
///
/// # Arguments
///
/// - `CameraAngles` - The non-reactive camera orbit angles.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer move handler.
pub(crate) fn game_3d_on_pointer_move(
    angles: CameraAngles,
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let last: Option<(f64, f64)> = last_pointer.get();
        let Some((last_x, last_y)) = last else {
            return;
        };
        let client_x: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("clientX"))
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or_default();
        let client_y: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("clientY"))
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or_default();
        let dx: f64 = client_x - last_x;
        let dy: f64 = client_y - last_y;
        last_pointer.set(Some((client_x, client_y)));
        let yaw: f64 = angles.yaw.get() - dx * 0.01;
        let pitch: f64 = (angles.pitch.get() + dy * 0.01).clamp(
            -HALF_PI + GAME_3D_PITCH_CLAMP,
            HALF_PI - GAME_3D_PITCH_CLAMP,
        );
        angles.yaw.set(yaw);
        angles.pitch.set(pitch);
    }))
}

/// Creates a pointer event handler that records the pointer start position.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer down handler.
pub(crate) fn game_3d_on_pointer_down(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let client_x: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("clientX"))
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or_default();
        let client_y: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("clientY"))
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or_default();
        last_pointer.set(Some((client_x, client_y)));
    }))
}

/// Creates a pointer event handler that clears the pointer position.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer up handler.
pub(crate) fn game_3d_on_pointer_up(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        last_pointer.set(None);
    }))
}

/// Extracts the client coordinates of the first active touch from a `TouchEvent`.
///
/// Reads `touches[0].clientX` and `touches[0].clientY` from the event via
/// JavaScript reflection. Used by touch-specific camera drag handlers since
/// `TouchEvent` does not expose `clientX`/`clientY` directly on the event object.
///
/// # Arguments
///
/// - `&Event` - The native touch event.
///
/// # Returns
///
/// - `(f64, f64)` - The `(client_x, client_y)` coordinates of the first touch.
pub(crate) fn extract_first_touch_client(event: &Event) -> (f64, f64) {
    let touches_value: JsValue = Reflect::get(
        event.as_ref(),
        &JsValue::from_str(GAME_3D_EVENT_PROPERTY_TOUCHES),
    )
    .ok()
    .unwrap_or(JsValue::NULL);
    let touches: Array = touches_value.unchecked_into();
    if touches.length() == 0 {
        return (0.0, 0.0);
    }
    let touch: JsValue = touches.get(0);
    let client_x: f64 = Reflect::get(&touch, &JsValue::from_str(GAME_3D_EVENT_PROPERTY_CLIENT_X))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default();
    let client_y: f64 = Reflect::get(&touch, &JsValue::from_str(GAME_3D_EVENT_PROPERTY_CLIENT_Y))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default();
    (client_x, client_y)
}

/// Creates a touch event handler that records the first touch start position and
/// prevents default browser behavior to avoid page scrolling during camera drag.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch start handler.
pub(crate) fn game_3d_on_touch_start(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        let (client_x, client_y): (f64, f64) = extract_first_touch_client(&event);
        last_pointer.set(Some((client_x, client_y)));
    }))
}

/// Creates a touch event handler that updates orbit angles based on single-finger
/// drag movement and prevents default browser behavior.
///
/// # Arguments
///
/// - `CameraAngles` - The non-reactive camera orbit angles.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch move handler.
pub(crate) fn game_3d_on_touch_move(
    angles: CameraAngles,
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
        let (client_x, client_y): (f64, f64) = extract_first_touch_client(&event);
        let dx: f64 = client_x - last_x;
        let dy: f64 = client_y - last_y;
        last_pointer.set(Some((client_x, client_y)));
        let yaw: f64 = angles.yaw.get() - dx * 0.01;
        let pitch: f64 = (angles.pitch.get() + dy * 0.01).clamp(
            -HALF_PI + GAME_3D_PITCH_CLAMP,
            HALF_PI - GAME_3D_PITCH_CLAMP,
        );
        angles.yaw.set(yaw);
        angles.pitch.set(pitch);
    }))
}

/// Creates a touch event handler that clears the pointer position and prevents
/// default browser behavior.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch end handler.
pub(crate) fn game_3d_on_touch_end(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        last_pointer.set(None);
    }))
}

/// Creates the reactive state signals for the 3D WebGPU demo.
///
/// # Returns
///
/// - `UseGame3DWebGpu` - The WebGPU demo state.
pub(crate) fn use_game_3d_webgpu_state() -> UseGame3DWebGpu {
    UseGame3DWebGpu {
        fps: App::use_signal(|| 0.0),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        init_error_code: App::use_signal(|| ""),
    }
}

/// Creates the reactive state signals for the 3D WebGL demo.
///
/// # Returns
///
/// - `UseGame3DWebGl` - The WebGL demo state.
pub(crate) fn use_game_3d_webgl_state() -> UseGame3DWebGl {
    UseGame3DWebGl {
        fps: App::use_signal(|| 0.0),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        init_error_code: App::use_signal(|| ""),
    }
}

/// Parses a `#rrggbb` CSS color string into 0.0-1.0 RGB floats.
///
/// Cube face/edge colors are authored for CSS (`fillStyle`/`strokeStyle`);
/// the GPU shaders need plain floats. Malformed input falls back to white.
///
/// # Arguments
///
/// - `&str` - The CSS hex color string.
///
/// # Returns
///
/// - `(f32, f32, f32)` - The `(r, g, b)` channels in 0.0-1.0 range.
pub(crate) fn game_3d_hex_to_rgb(color: &str) -> (f32, f32, f32) {
    let hex: &str = color.strip_prefix('#').unwrap_or(color);
    let channel = |range: Range<usize>| -> f32 {
        hex.get(range)
            .and_then(|part: &str| u8::from_str_radix(part, 16).ok())
            .map(|value: u8| f32::from(value) / 255.0)
            .unwrap_or(1.0)
    };
    (channel(0..2), channel(2..4), channel(4..6))
}

/// Reads the computed CSS `background-color` of a canvas element.
///
/// The GPU canvases cannot be cleared to transparency (the WebGPU swap
/// chain uses an opaque alpha mode by default), so the demo clears to
/// the same `var!(accent)` background that shows through the
/// transparent-cleared Canvas 2D tab. Re-reading the computed style
/// also picks up theme toggles, which swap the accent color under the
/// same canvas element.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the canvas element.
///
/// # Returns
///
/// - `(f64, f64, f64)` - The `(r, g, b)` clear color in 0.0-1.0 range.
pub(crate) fn game_3d_canvas_clear_color(canvas_selector: &str) -> (f64, f64, f64) {
    let Some(window_value): Option<Window> = window() else {
        return (0.0, 0.0, 0.0);
    };
    let background: String = window_value
        .document()
        .and_then(|document: Document| document.query_selector(canvas_selector).ok().flatten())
        .and_then(|element: Element| window_value.get_computed_style(&element).ok().flatten())
        .and_then(|style: CssStyleDeclaration| style.get_property_value("background-color").ok())
        .unwrap_or_default();
    // Computed colors serialize as `rgb(r, g, b)` or `rgba(r, g, b, a)`.
    let Some(inner) = background
        .split('(')
        .nth(1)
        .and_then(|value: &str| value.strip_suffix(')'))
    else {
        return (0.0, 0.0, 0.0);
    };
    let mut channels = inner
        .split(',')
        .filter_map(|part: &str| part.trim().parse::<f64>().ok());
    let r: f64 = channels.next().unwrap_or_default() / 255.0;
    let g: f64 = channels.next().unwrap_or_default() / 255.0;
    let b: f64 = channels.next().unwrap_or_default() / 255.0;
    (r, g, b)
}

/// Packs the scene into the uniform layout consumed by the GPU cubes
/// shaders.
///
/// Layout: the column-major view-projection matrix (16 floats), the
/// camera position plus padding (4 floats), then one `CubeData` record
/// (rotation quaternion, position + scaled half-size, face color, edge
/// color; 16 floats) per cube. Cubes are sorted back-to-front with the
/// same painter's-algorithm depth key as [`render_scene`], and the
/// result is padded out to `GAME_3D_GPU_MAX_CUBES` records so the
/// fixed-size uniform layout is fully overwritten each frame and stale
/// cubes never linger.
///
/// # Arguments
///
/// - `&[Cube3D]` - The cube list for this frame.
/// - `&Camera3D` - The orbit camera for this frame.
///
/// # Returns
///
/// - `Vec<f32>` - The packed uniform data (20 + `GAME_3D_GPU_MAX_CUBES * 16` floats).
fn pack_game_3d_cubes_uniform(cubes: &[Cube3D], camera: &Camera3D) -> Vec<f32> {
    let mut sorted: Vec<(&Cube3D, f64)> = cubes
        .iter()
        .map(|cube: &Cube3D| {
            let world_vertices: Vec<Vector3D> = GAME_3D_CUBE_VERTICES
                .iter()
                .map(|(vx, vy, vz): &(f64, f64, f64)| {
                    transform_cube_vertex(cube, Vector3D::new(*vx, *vy, *vz))
                })
                .collect();
            (cube, face_average_depth(&world_vertices, camera))
        })
        .collect();
    sorted.sort_by(|a: &(&Cube3D, f64), b: &(&Cube3D, f64)| {
        let (_, depth_a) = *a;
        let (_, depth_b) = *b;
        depth_a.partial_cmp(&depth_b).unwrap_or(Ordering::Equal)
    });
    let view_proj_elements: [f64; 16] = camera.view_proj_matrix().get_elements();
    let mut data: Vec<f32> = view_proj_elements
        .iter()
        .map(|value: &f64| *value as f32)
        .collect();
    let camera_position: Vector3D = camera.get_position();
    data.extend_from_slice(&[
        camera_position.get_x() as f32,
        camera_position.get_y() as f32,
        camera_position.get_z() as f32,
        0.0,
    ]);
    for (cube, _depth) in &sorted {
        let (face_r, face_g, face_b) = game_3d_hex_to_rgb(&cube.face_color);
        let (edge_r, edge_g, edge_b) = game_3d_hex_to_rgb(&cube.edge_color);
        data.extend_from_slice(&[
            cube.rotation.get_x() as f32,
            cube.rotation.get_y() as f32,
            cube.rotation.get_z() as f32,
            cube.rotation.get_w() as f32,
            cube.position.get_x() as f32,
            cube.position.get_y() as f32,
            cube.position.get_z() as f32,
            (cube.scale * GAME_3D_CUBE_HALF_SIZE) as f32,
            face_r,
            face_g,
            face_b,
            1.0,
            edge_r,
            edge_g,
            edge_b,
            1.0,
        ]);
    }
    data.resize(20 + GAME_3D_GPU_MAX_CUBES * 16, 0.0);
    data
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
/// - `Signal<Game3DTab>` - The tab signal to update.
/// - `Game3DTab` - The tab variant to set.
/// - `UseGame3DFullscreen` - The fullscreen state to clear on switch.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the active
///   tab and clears any active fullscreen mode.
pub(crate) fn game_3d_on_tab_select(
    tab: Signal<Game3DTab>,
    value: Game3DTab,
    fullscreen: UseGame3DFullscreen,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        fullscreen.get_canvas_2d().set(false);
        fullscreen.get_web_gl().set(false);
        fullscreen.get_web_gpu().set(false);
        tab.set(value);
    }))
}

/// Starts the 3D WebGPU cubes loop driven by `requestAnimationFrame`.
///
/// Mirrors [`start_game_3d_loop`]: the same fixed-timestep quaternion
/// integration runs on the shared cube list and the same orbit camera
/// (auto-rotation plus pointer/touch drag) drives the view, but rendering
/// goes through a WGSL pipeline that draws every cube as 12
/// shader-generated triangles with per-cube transform and colors uploaded
/// to a uniform buffer each frame. The canvas is cleared to the element's
/// computed CSS background color so the WebGPU output matches the
/// transparent-cleared Canvas 2D tab exactly.
///
/// # Arguments
///
/// - `UseGame3DWebGpu` - The WebGPU backend state for signal updates.
/// - `UseGame3D` - The shared game state (running/auto_rotate signals).
/// - `Rc<RefCell<Vec<Cube3D>>>` - The shared cube list.
/// - `CameraAngles` - The non-reactive camera orbit angles.
pub(crate) fn start_game_3d_webgpu_loop(
    state: UseGame3DWebGpu,
    game: UseGame3D,
    cubes: Rc<RefCell<Vec<Cube3D>>>,
    angles: CameraAngles,
) {
    let init_state: UseGame3DWebGpu = state;
    let loop_state: UseGame3DWebGpu = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let renderer_rc: Rc<RefCell<Option<WebGpuRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let guard_cell: CanvasGuardCell = Rc::new(RefCell::new(None));
    let observer_cell: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));
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
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let renderer_for_cleanup: Rc<RefCell<Option<WebGpuRenderer>>> = renderer_rc.clone();
    let resize_timer_for_cleanup: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let cancelled_for_cleanup: Rc<Cell<bool>> = cancelled.clone();
    let guard_for_cleanup: CanvasGuardCell = guard_cell.clone();
    let observer_for_cleanup: Rc<RefCell<Option<ResizeObserver>>> = observer_cell.clone();
    App::use_cleanup(move || {
        cancelled_for_cleanup.set(true);
        if let Some(cancel_id) = raf_for_cleanup.get() {
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            let _ = window_value.cancel_animation_frame(cancel_id);
        }
        if let Some(timer_id) = resize_timer_for_cleanup.get() {
            let Some(window_value): Option<Window> = window() else {
                return;
            };
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
        if let Some((listeners, element)) = guard_for_cleanup.borrow_mut().take() {
            for (closure, event_name) in listeners {
                let _ = element.remove_event_listener_with_callback(
                    event_name,
                    closure.as_ref().unchecked_ref(),
                );
            }
        }
    });
    let cancelled_for_init: Rc<Cell<bool>> = cancelled.clone();
    let Some(loading_window): Option<Window> = window() else {
        return;
    };
    let loading_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        draw_game_3d_loading(
            GAME_3D_WEBGPU_LOADING_CANVAS_SELECTOR,
            GAME_3D_WEBGPU_CANVAS_SELECTOR,
        );
    }));
    let loading_callback: Function = loading_closure.as_ref().unchecked_ref::<Function>().clone();
    loading_closure.forget();
    let _ =
        loading_window.set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
    spawn_local(async move {
        let config: RenderConfig = RenderConfig::webgpu(
            GAME_3D_WEBGPU_CANVAS_SELECTOR,
            GAME_3D_CANVAS_WIDTH,
            GAME_3D_CANVAS_HEIGHT,
        );
        let renderer: Result<WebGpuRenderer, WebGpuInitError> =
            Engine::webgpu_renderer(&config).await;
        if cancelled_for_init.get() {
            return;
        }
        let renderer: WebGpuRenderer = match renderer {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!("[euv-engine][game_3d] webgpu init failed: {error}"));
                init_state.get_init_error_code().set(error.code());
                init_state.get_loaded().set(true);
                return;
            }
        };
        let pipeline: JsValue = renderer.create_render_pipeline(GAME_3D_WEBGPU_SHADER);
        let uniform_buffer: JsValue =
            renderer.create_uniform_buffer(&vec![0.0; 20 + GAME_3D_GPU_MAX_CUBES * 16]);
        let bind_group: JsValue = renderer.create_uniform_bind_group(&pipeline, &uniform_buffer);
        *guard_cell.borrow_mut() = register_canvas_scroll_guard(GAME_3D_WEBGPU_CANVAS_SELECTOR);
        let clear_color: Rc<Cell<(f64, f64, f64)>> = Rc::new(Cell::new(
            game_3d_canvas_clear_color(GAME_3D_WEBGPU_CANVAS_SELECTOR),
        ));
        let accumulator: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        init_state.get_active().set(true);
        // Delay flipping `loaded` so the loading overlay stays painted for a
        // minimum visible duration even when init completes instantly.
        set_loaded_delayed(init_state.get_loaded(), GAME_3D_LOADING_MIN_MILLIS);
        *renderer_rc.borrow_mut() = Some(renderer);
        let pipeline_rc: Rc<JsValue> = Rc::new(pipeline);
        let buffer_rc: Rc<JsValue> = Rc::new(uniform_buffer);
        let bind_group_rc: Rc<JsValue> = Rc::new(bind_group);
        // Synchronous resize on CSS-box change. ResizeObserver callbacks
        // run BEFORE the browser paints the next frame, so setting
        // `canvas.width = new_w` inside the observer ensures the very
        // first paint after `enter_game_3d_fullscreen` /
        // `exit_game_3d_fullscreen` already has the new backing store.
        // Without this, the raf-based safety net was leaving a 1-frame
        // (~16ms) window where the browser painted the previous-size
        // backing image stretched into the new CSS box - visibly
        // distorting the cube faces until the next raf cycle resized
        // the backing. With the observer, that window collapses to a
        // single sub-millisecond observer callback that fires before
        // any frame is committed.
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
                    .query_selector(GAME_3D_WEBGPU_CANVAS_SELECTOR)
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
                    // IMPORTANT: set the canvas backing size FIRST,
                    // BEFORE calling `renderer.resize(...)`. Setting
                    // `canvas.width` is a fast DOM-only operation that
                    // updates the backing store synchronously and
                    // commits the new dimensions before the next paint.
                    // `renderer.resize(...)` on the other hand is heavy
                    // - it reconfigures the WebGL/WebGPU swap chain,
                    // reallocates textures, and recompiles shaders,
                    // all of which can stall the main thread for
                    // 100-200ms while the GPU processes the request.
                    // If we call `renderer.resize` first, the browser
                    // paints 6-12 frames at 16ms cadence during the
                    // stall using the OLD backing in the NEW CSS box -
                    // the visible cube distortion this PR is trying to
                    // eliminate. By resizing `canvas.width` first the
                    // backing matches the CSS box immediately, then
                    // `renderer.resize` re-allocates the GPU side
                    // resources but the next paint at least has a
                    // backing store that matches the CSS box aspect
                    // ratio (cubes just don't render correctly until
                    // GPU realloc completes - they go briefly blank,
                    // not visibly stretched).
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
                .query_selector(GAME_3D_WEBGPU_CANVAS_SELECTOR)
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
        let acc_clone: Rc<Cell<f64>> = accumulator.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let resize_dirty_for_loop: Rc<Cell<bool>> = resize_dirty.clone();
        let cancelled_for_loop: Rc<Cell<bool>> = cancelled.clone();
        let prev_rotations: Rc<RefCell<Vec<Quaternion>>> = Rc::new(RefCell::new(Vec::new()));
        let prev_for_loop: Rc<RefCell<Vec<Quaternion>>> = prev_rotations.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            // Stop on tab-switch cleanup (`cancelled`) or when the canvas
            // left the document (router navigation fires no cleanup).
            if cancelled_for_loop.get() || game_3d_canvas_detached(GAME_3D_WEBGPU_CANVAS_SELECTOR) {
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
                GAME_3D_FIXED_TIMESTEP
            } else {
                (current_time - prev).min(0.25)
            };
            last_clone.set(current_time);
            if game.get_running().get() {
                // Accumulate only while running: a paused accumulator would grow
                // unboundedly and burst catch-up physics steps on resume.
                acc_clone.set(acc_clone.get() + frame_time);
                if game.get_auto_rotate().get() {
                    let yaw: f64 = angles.yaw.get() + GAME_3D_AUTO_YAW_SPEED * frame_time;
                    angles.yaw.set(yaw);
                }
                while acc_clone.get() >= GAME_3D_FIXED_TIMESTEP {
                    snapshot_cube_rotations(&mut prev_for_loop.borrow_mut(), &cubes.borrow());
                    update_cubes(&mut cubes.borrow_mut(), GAME_3D_FIXED_TIMESTEP);
                    acc_clone.set(acc_clone.get() - GAME_3D_FIXED_TIMESTEP);
                }
            }
            let alpha: f64 = (acc_clone.get() / GAME_3D_FIXED_TIMESTEP).clamp(0.0, 1.0);
            // The resize-debounce path only clears the flag and computes
            // the new dimensions. The actual `renderer.resize(...)` call
            // is folded into the render block below so we hold
            // `renderer_for_loop.borrow_mut()` exactly once per frame.
            // Otherwise we previously panicked with `RefCell already
            // borrowed` when both blocks tried to borrow the same cell.
            let resize_dirty: bool = if resize_dirty_for_loop.get() {
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
            // Read the canvas's CSS pixel dimensions (clientWidth / clientHeight)
            // so the GPU backing store grows with the canvas when the user
            // enters or exits fullscreen. Mirrors the same swap in
            // game_2d/hook/fn.rs.
            let (canvas_width, canvas_height): (f64, f64) =
                read_canvas_size(GAME_3D_WEBGPU_CANVAS_SELECTOR).unwrap_or((0.0, 0.0));
            let new_physical_width: u32 = (canvas_width * dpr).round() as u32;
            let new_physical_height: u32 = (canvas_height * dpr).round() as u32;
            // Borrow the renderer exactly once for the entire frame. We
            // use `borrow_mut().as_mut()` (NOT `borrow_mut().take()`) so
            // we do not have to write back - the RefMut guard releases
            // automatically when this block exits, avoiding a second
            // `borrow_mut()` call that previously panicked with
            // `RefCell already borrowed`.
            if let Some(renderer) = renderer_for_loop.borrow_mut().as_mut() {
                // Resize the WebGPU backing store every frame the CSS box
                // diverges from `canvas.width` / `canvas.height`. Reading
                // `getBoundingClientRect` (CSS layout box, not backing
                // store) means this comparison is stable: a resize only
                // fires when the layout actually changes, not when our own
                // `canvas.width` write updates the backing store.
                //
                // Without this per-frame check, the synthetic `resize`
                // event dispatched by `enter_game_3d_fullscreen` /
                // `exit_game_3d_fullscreen` fires before the euv
                // signal-driven DOM re-render flips the canvas CSS class
                // (100ms debounce). During that gap the canvas DOM
                // element already has its new CSS box but the backing
                // store still holds the previous size, so the browser
                // stretches the OLD-size backing image into the NEW CSS
                // box - producing a visible first-frame cube distortion
                // (~120ms) that only recovers once the debounced resize
                // tick reads the new CSS dimensions and resizes.
                //
                // Resizing here on the very first frame we observe the
                // CSS change collapses the distortion to a single frame.
                if new_physical_width > 0 && new_physical_height > 0 {
                    let backing_w: u32 = renderer.get_canvas().width();
                    let backing_h: u32 = renderer.get_canvas().height();
                    if backing_w != new_physical_width || backing_h != new_physical_height {
                        renderer.get_canvas().set_width(new_physical_width);
                        renderer.get_canvas().set_height(new_physical_height);
                        let _ = renderer.resize(new_physical_width, new_physical_height);
                    }
                }
                if resize_dirty {
                    let _ = renderer.resize(new_physical_width, new_physical_height);
                }
                let camera: Camera3D = create_orbit_camera(
                    angles.yaw.get(),
                    angles.pitch.get(),
                    canvas_width,
                    canvas_height,
                );
                let render_cubes: Vec<Cube3D> =
                    interpolate_cubes(&cubes.borrow(), &prev_for_loop.borrow(), alpha);
                let uniform_data: Vec<f32> = pack_game_3d_cubes_uniform(&render_cubes, &camera);
                let vertex_count: u32 = (render_cubes.len() * 36) as u32;
                renderer.update_uniform_buffer(&buffer_for_loop, &uniform_data);
                // Refresh the clear color every frame so a theme toggle
                // takes effect within one paint. The computed style is
                // cached by the engine after the first read, so the only
                // per-frame cost is a small string parse and equality
                // check; the GPU clear value is only re-uploaded when the
                // tuple actually changes.
                let next_clear: (f64, f64, f64) =
                    game_3d_canvas_clear_color(GAME_3D_WEBGPU_CANVAS_SELECTOR);
                if clear_color_for_loop.get() != next_clear {
                    clear_color_for_loop.set(next_clear);
                }
                let (r, g, b) = clear_color_for_loop.get();
                renderer.render_frame_with_bind_group(
                    &pipeline_for_loop,
                    &bind_group_for_loop,
                    (r, g, b, 1.0),
                    vertex_count,
                );
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

/// Starts the 3D WebGL cubes loop driven by `requestAnimationFrame`.
///
/// Mirrors [`start_game_3d_loop`]: the same fixed-timestep quaternion
/// integration runs on the shared cube list and the same orbit camera
/// (auto-rotation plus pointer/touch drag) drives the view, but rendering
/// goes through a GLSL ES 3.00 program that draws every cube as 12
/// shader-generated triangles with per-cube transform and colors uploaded
/// to `vec4` uniform arrays each frame. The canvas is cleared to the
/// element's computed CSS background color so the WebGL output matches
/// the transparent-cleared Canvas 2D tab exactly. WebGL initialization is
/// synchronous; the `spawn_local` wrapper only defers execution past the
/// current render pass so the canvas element exists in the DOM.
///
/// # Arguments
///
/// - `UseGame3DWebGl` - The WebGL backend state for signal updates.
/// - `UseGame3D` - The shared game state (running/auto_rotate signals).
/// - `Rc<RefCell<Vec<Cube3D>>>` - The shared cube list.
/// - `CameraAngles` - The non-reactive camera orbit angles.
pub(crate) fn start_game_3d_webgl_loop(
    state: UseGame3DWebGl,
    game: UseGame3D,
    cubes: Rc<RefCell<Vec<Cube3D>>>,
    angles: CameraAngles,
) {
    let init_state: UseGame3DWebGl = state;
    let loop_state: UseGame3DWebGl = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let renderer_rc: Rc<RefCell<Option<WebGlRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let guard_cell: CanvasGuardCell = Rc::new(RefCell::new(None));
    let observer_cell: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));
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
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let renderer_for_cleanup: Rc<RefCell<Option<WebGlRenderer>>> = renderer_rc.clone();
    let resize_timer_for_cleanup: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let cancelled_for_cleanup: Rc<Cell<bool>> = cancelled.clone();
    let guard_for_cleanup: CanvasGuardCell = guard_cell.clone();
    let observer_for_cleanup: Rc<RefCell<Option<ResizeObserver>>> = observer_cell.clone();
    App::use_cleanup(move || {
        cancelled_for_cleanup.set(true);
        if let Some(cancel_id) = raf_for_cleanup.get() {
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            let _ = window_value.cancel_animation_frame(cancel_id);
        }
        if let Some(timer_id) = resize_timer_for_cleanup.get() {
            let Some(window_value): Option<Window> = window() else {
                return;
            };
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
        if let Some((listeners, element)) = guard_for_cleanup.borrow_mut().take() {
            for (closure, event_name) in listeners {
                let _ = element.remove_event_listener_with_callback(
                    event_name,
                    closure.as_ref().unchecked_ref(),
                );
            }
        }
    });
    let cancelled_for_init: Rc<Cell<bool>> = cancelled.clone();
    let Some(loading_window): Option<Window> = window() else {
        return;
    };
    let loading_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        draw_game_3d_loading(
            GAME_3D_WEBGL_LOADING_CANVAS_SELECTOR,
            GAME_3D_WEBGL_CANVAS_SELECTOR,
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
            GAME_3D_WEBGL_CANVAS_SELECTOR,
            GAME_3D_CANVAS_WIDTH,
            GAME_3D_CANVAS_HEIGHT,
        );
        let renderer: WebGlRenderer = match Engine::webgl_renderer(&config) {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!("[euv-engine][game_3d] webgl init failed: {error}"));
                init_state.get_init_error_code().set(error.code());
                init_state.get_loaded().set(true);
                return;
            }
        };
        let program: WebGlProgram = match renderer
            .create_program(GAME_3D_WEBGL_VERTEX_SHADER, GAME_3D_WEBGL_FRAGMENT_SHADER)
        {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!(
                    "[euv-engine][game_3d] webgl program failed: {error}"
                ));
                init_state.get_init_error_code().set("WEBGL_PROGRAM_ERROR");
                init_state.get_loaded().set(true);
                return;
            }
        };
        // Resolve uniform locations once after link; per-frame
        // `getUniformLocation` calls are pure overhead and locations are
        // stable for the lifetime of the program.
        let view_proj_location: Rc<Option<WebGlUniformLocation>> =
            Rc::new(renderer.get_uniform_location(&program, "u_view_proj[0]"));
        let camera_pos_location: Rc<Option<WebGlUniformLocation>> =
            Rc::new(renderer.get_uniform_location(&program, "u_camera_pos"));
        let cubes_location: Rc<Option<WebGlUniformLocation>> =
            Rc::new(renderer.get_uniform_location(&program, "u_cubes[0]"));
        *guard_cell.borrow_mut() = register_canvas_scroll_guard(GAME_3D_WEBGL_CANVAS_SELECTOR);
        let clear_color: Rc<Cell<(f64, f64, f64)>> = Rc::new(Cell::new(
            game_3d_canvas_clear_color(GAME_3D_WEBGL_CANVAS_SELECTOR),
        ));
        let accumulator: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        init_state.get_active().set(true);
        // Delay flipping `loaded` so the loading overlay stays painted for a
        // minimum visible duration even when init completes instantly.
        set_loaded_delayed(init_state.get_loaded(), GAME_3D_LOADING_MIN_MILLIS);
        *renderer_rc.borrow_mut() = Some(renderer);
        let program_rc: Rc<WebGlProgram> = Rc::new(program);
        // Synchronous resize on CSS-box change. ResizeObserver callbacks
        // run BEFORE the browser paints the next frame, so setting
        // `canvas.width = new_w` inside the observer ensures the very
        // first paint after `enter_game_3d_fullscreen` /
        // `exit_game_3d_fullscreen` already has the new backing store.
        // Without this, the raf-based safety net was leaving a 1-frame
        // (~16ms) window where the browser painted the previous-size
        // backing image stretched into the new CSS box - visibly
        // distorting the cube faces until the next raf cycle resized
        // the backing. With the observer, that window collapses to a
        // single sub-millisecond observer callback that fires before
        // any frame is committed.
        //
        // Apply `canvas.width = new_w` BEFORE `renderer.resize(...)`.
        // The DOM setter is a fast operation that commits the new
        // backing size synchronously, but `renderer.resize` reconfigures
        // the WebGL context (viewport, framebuffer, textures) which
        // can stall the main thread for 100-200ms while the GPU
        // processes the swap-chain realloc. Doing `canvas.width` first
        // means the next paint has a correctly-sized backing store even
        // before `renderer.resize` returns, so cubes render without the
        // aspect-ratio distortion that would otherwise show for 6-12
        // frames while the GPU is busy.
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
                    .query_selector(GAME_3D_WEBGL_CANVAS_SELECTOR)
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
                .query_selector(GAME_3D_WEBGL_CANVAS_SELECTOR)
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
        let view_proj_location_for_loop: Rc<Option<WebGlUniformLocation>> =
            view_proj_location.clone();
        let camera_pos_location_for_loop: Rc<Option<WebGlUniformLocation>> =
            camera_pos_location.clone();
        let cubes_location_for_loop: Rc<Option<WebGlUniformLocation>> = cubes_location.clone();
        let clear_color_for_loop: Rc<Cell<(f64, f64, f64)>> = clear_color.clone();
        let acc_clone: Rc<Cell<f64>> = accumulator.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let resize_dirty_for_loop: Rc<Cell<bool>> = resize_dirty.clone();
        let cancelled_for_loop: Rc<Cell<bool>> = cancelled.clone();
        let prev_rotations: Rc<RefCell<Vec<Quaternion>>> = Rc::new(RefCell::new(Vec::new()));
        let prev_for_loop: Rc<RefCell<Vec<Quaternion>>> = prev_rotations.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            // Stop on tab-switch cleanup (`cancelled`) or when the canvas
            // left the document (router navigation fires no cleanup).
            if cancelled_for_loop.get() || game_3d_canvas_detached(GAME_3D_WEBGL_CANVAS_SELECTOR) {
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
                GAME_3D_FIXED_TIMESTEP
            } else {
                (current_time - prev).min(0.25)
            };
            last_clone.set(current_time);
            if game.get_running().get() {
                // Accumulate only while running: a paused accumulator would grow
                // unboundedly and burst catch-up physics steps on resume.
                acc_clone.set(acc_clone.get() + frame_time);
                if game.get_auto_rotate().get() {
                    let yaw: f64 = angles.yaw.get() + GAME_3D_AUTO_YAW_SPEED * frame_time;
                    angles.yaw.set(yaw);
                }
                while acc_clone.get() >= GAME_3D_FIXED_TIMESTEP {
                    snapshot_cube_rotations(&mut prev_for_loop.borrow_mut(), &cubes.borrow());
                    update_cubes(&mut cubes.borrow_mut(), GAME_3D_FIXED_TIMESTEP);
                    acc_clone.set(acc_clone.get() - GAME_3D_FIXED_TIMESTEP);
                }
            }
            let alpha: f64 = (acc_clone.get() / GAME_3D_FIXED_TIMESTEP).clamp(0.0, 1.0);
            let resize_dirty: bool = if resize_dirty_for_loop.get() {
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
            // the latter in Chrome track `canvas.width`, the backing-store
            // size, and would create a feedback loop if read every frame).
            // `getBoundingClientRect` returns the CSS box which is in sync
            // with layout, so the resize below only fires when the layout
            // actually changes - it does NOT loop.
            let (canvas_width, canvas_height): (f64, f64) =
                read_canvas_size(GAME_3D_WEBGL_CANVAS_SELECTOR).unwrap_or((0.0, 0.0));
            let new_physical_width: u32 = (canvas_width * dpr).round() as u32;
            let new_physical_height: u32 = (canvas_height * dpr).round() as u32;
            if let Some(renderer) = renderer_for_loop.borrow_mut().as_mut() {
                // Resize the WebGL backing store every frame the CSS box
                // diverges from `canvas.width` / `canvas.height`. Reading
                // `getBoundingClientRect` (CSS layout box, not backing
                // store) means this comparison is stable: a resize only
                // fires when the layout actually changes, not when our own
                // `canvas.width` write updates the backing store.
                //
                // The DOM-side `canvas.width = new_w` setter is applied
                // FIRST so the backing store matches the CSS box before
                // the next browser paint. `renderer.resize(...)` is
                // then called to reconfigure the WebGL context (viewport,
                // framebuffer, textures); this is a heavy GPU-side
                // operation that can stall the main thread for 100-200ms
                // during a swap-chain realloc. Doing `canvas.width`
                // first means the next paint has a correctly-sized
                // backing store even before `renderer.resize` returns,
                // so cubes render without the aspect-ratio distortion
                // that would otherwise show for 6-12 frames while the
                // GPU is busy.
                if new_physical_width > 0 && new_physical_height > 0 {
                    let backing_w: u32 = renderer.get_canvas().width();
                    let backing_h: u32 = renderer.get_canvas().height();
                    if backing_w != new_physical_width || backing_h != new_physical_height {
                        renderer.get_canvas().set_width(new_physical_width);
                        renderer.get_canvas().set_height(new_physical_height);
                        renderer.resize(new_physical_width, new_physical_height);
                    }
                }
                if resize_dirty {
                    renderer.resize(new_physical_width, new_physical_height);
                }
                let camera: Camera3D = create_orbit_camera(
                    angles.yaw.get(),
                    angles.pitch.get(),
                    canvas_width,
                    canvas_height,
                );
                let render_cubes: Vec<Cube3D> =
                    interpolate_cubes(&cubes.borrow(), &prev_for_loop.borrow(), alpha);
                let uniform_data: Vec<f32> = pack_game_3d_cubes_uniform(&render_cubes, &camera);
                let vertex_count: i32 = (render_cubes.len() * 36) as i32;
                renderer.set_uniform_4fv(
                    &program_for_loop,
                    view_proj_location_for_loop.as_ref().as_ref(),
                    &uniform_data[0..16],
                );
                renderer.set_uniform_4fv(
                    &program_for_loop,
                    camera_pos_location_for_loop.as_ref().as_ref(),
                    &uniform_data[16..20],
                );
                renderer.set_uniform_4fv(
                    &program_for_loop,
                    cubes_location_for_loop.as_ref().as_ref(),
                    &uniform_data[20..],
                );
                // Refresh the clear color every frame so a theme toggle
                // takes effect within one paint. The computed style is
                // cached by the engine after the first read, so the only
                // per-frame cost is a small string parse and equality
                // check; the GPU clear value is only re-uploaded when the
                // tuple actually changes.
                let next_clear: (f64, f64, f64) =
                    game_3d_canvas_clear_color(GAME_3D_WEBGL_CANVAS_SELECTOR);
                if clear_color_for_loop.get() != next_clear {
                    clear_color_for_loop.set(next_clear);
                }
                let (r, g, b) = clear_color_for_loop.get();
                renderer.render_frame(&program_for_loop, (r, g, b, 1.0), vertex_count);
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

/// Creates the 3D game fullscreen reactive state signals.
///
/// Allocates hook slots in this fixed order:
/// 1. canvas_2d
/// 2. web_gl
/// 3. web_gpu
///
/// # Returns
///
/// - `UseGame3DFullscreen` - A `UseGame3DFullscreen` value.
pub(crate) fn use_game_3d_fullscreen_state() -> UseGame3DFullscreen {
    UseGame3DFullscreen {
        canvas_2d: App::use_signal(|| false),
        web_gl: App::use_signal(|| false),
        web_gpu: App::use_signal(|| false),
    }
}

/// Enters landscape fullscreen mode for the 3D game on the active tab.
///
/// Sets the tab-specific fullscreen signal, pushes a browser history
/// entry so the system back button exits fullscreen instead of
/// navigating away, then flushes the cached safe-area insets to the
/// newly-mounted overlay container. The canvas element is *not*
/// recreated — the active tab's `<canvas>` is re-keyed to live inside
/// `c_game_container_fullscreen` instead of its inline slot, so the
/// running game loop, cube list, FPS counter, and pause state all
/// survive the transition.
///
/// # Arguments
///
/// - `UseGame3DFullscreen` - The 3D game fullscreen state.
/// - `Signal<bool>` - The fullscreen signal for the active tab.
pub(crate) fn enter_game_3d_fullscreen(state: UseGame3DFullscreen, tab: Signal<bool>) {
    tab.set(true);
    let _ = state;
    Router::overlay_push_state();
    UseEuvLayout::apply_cached_insets();
    // Dispatch a `resize` event on the window so the existing
    // `App::use_window_event("resize", ...)` handler fires and the
    // 3D game loop's `resize_dirty` flag is set. That causes the
    // loop to re-acquire the SSAA canvas with the new (fullscreen)
    // dimensions read from `canvas.clientWidth` / `clientHeight`,
    // and re-call `WebGpuRenderer::resize` / `WebGlRenderer::resize`
    // for the GPU canvases so the cube projection fills the new
    // canvas size. Mirrors the same hook in
    // `example/src/page/game_2d/hook/fn.rs::enter_game_2d_fullscreen`.
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let event: Result<Event, JsValue> = Event::new("resize");
    if let Ok(event) = event {
        let _ = window_value.dispatch_event(&event);
    }
}

/// Exits landscape fullscreen mode for the 3D game on the active tab.
///
/// Used by the in-overlay Exit button. Clears the active tab's fullscreen
/// signal and re-applies the safe-area insets to whatever overlay
/// containers are now mounted.
///
/// # Arguments
///
/// - `Signal<bool>` - The fullscreen signal for the active tab.
pub(crate) fn exit_game_3d_fullscreen(tab: Signal<bool>) {
    tab.set(false);
    UseEuvLayout::apply_cached_insets();
    // See `enter_game_3d_fullscreen` - dispatch a synthetic `resize`
    // event so the game loop re-acquires the canvas with the inline
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
/// entry. Used when the exit is triggered by the system back button:
/// the `popstate` event itself has already consumed the `pushState`
/// entry that was created when entering fullscreen, so calling
/// `history.back()` again would over-consume the history stack.
///
/// # Arguments
///
/// - `Signal<bool>` - The fullscreen signal for the active tab.
pub(crate) fn exit_game_3d_fullscreen_from_popstate(tab: Signal<bool>) {
    tab.set(false);
    UseEuvLayout::apply_cached_insets();
    // See `enter_game_3d_fullscreen` for why we dispatch a synthetic
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
/// button while the 3D game is in landscape fullscreen mode.
///
/// Watches all three tab-specific fullscreen signals. When any one is
/// `true`, the corresponding `exit_game_3d_fullscreen_from_popstate`
/// runs and the guard returns `true` to consume the `popstate` event.
/// Otherwise returns `false` so the overlay stack or router can handle
/// the back navigation normally.
///
/// Returns the guard ID so the page can unregister it on unmount.
///
/// # Arguments
///
/// - `UseGame3DFullscreen` - The 3D game fullscreen state.
///
/// # Returns
///
/// - `usize` - The popstate guard ID.
pub(crate) fn use_game_3d_fullscreen_popstate(state: UseGame3DFullscreen) -> usize {
    Router::register_popstate_guard(Rc::new(move || {
        if state.get_canvas_2d().get() {
            exit_game_3d_fullscreen_from_popstate(state.get_canvas_2d());
            true
        } else if state.get_web_gl().get() {
            exit_game_3d_fullscreen_from_popstate(state.get_web_gl());
            true
        } else if state.get_web_gpu().get() {
            exit_game_3d_fullscreen_from_popstate(state.get_web_gpu());
            true
        } else {
            false
        }
    }))
}

/// Creates a click event handler that enters landscape fullscreen mode for the 3D game.
///
/// Delegates to [`enter_game_3d_fullscreen`], which sets the active
/// tab's fullscreen signal, pushes a history entry, and reapplies
/// safe-area insets to the newly-mounted overlay container. The canvas
/// itself is not recreated — the running game loop, cube list, FPS
/// counter, and pause state all survive the transition.
///
/// # Arguments
///
/// - `UseGame3DFullscreen` - The 3D game fullscreen state.
/// - `Signal<bool>` - The fullscreen signal for the active tab.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_3d_on_enter_fullscreen(
    state: UseGame3DFullscreen,
    tab: Signal<bool>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        enter_game_3d_fullscreen(state, tab);
    }))
}

/// Creates a click event handler that exits landscape fullscreen mode for the 3D game.
///
/// Delegates to [`exit_game_3d_fullscreen`], which clears the active
/// tab's fullscreen signal and reapplies safe-area insets. The
/// `history.back()` call inside [`Router::overlay_back`] consumes the
/// browser history entry that was pushed on enter.
///
/// # Arguments
///
/// - `Signal<bool>` - The fullscreen signal for the active tab.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_3d_on_exit_fullscreen(tab: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        exit_game_3d_fullscreen(tab);
        Router::overlay_back(None);
    }))
}
