use super::*;

/// Creates the 2D bouncing balls game reactive state signals wrapped in a `UseGame2D` struct.
///
/// # Returns
///
/// - `UseGame2D` - The 2D game state.
pub(crate) fn use_game_2d_state() -> UseGame2D {
    UseGame2D {
        running: App::use_signal(|| true),
        fps: App::use_signal(|| 0.0),
        ball_count: App::use_signal(|| 0),
        total_spawned: App::use_signal(|| 0),
        loaded: App::use_signal(|| false),
    }
}

/// Returns a random ball color from the predefined palette.
///
/// # Returns
///
/// - `&'static str` - A CSS color string.
pub(crate) fn random_ball_color() -> &'static str {
    let index: usize = (js_sys::Math::random() * GAME_2D_BALL_COLORS.len() as f64) as usize;
    GAME_2D_BALL_COLORS[index % GAME_2D_BALL_COLORS.len()]
}

/// Returns a random ball radius within the allowed range.
///
/// The constants `GAME_2D_BALL_MIN_RADIUS` / `GAME_2D_BALL_MAX_RADIUS`
/// express the desired ball radius range as a fraction of the canvas
/// width: 4 / 600 = 0.67% and 14 / 600 = 2.33% of a 600px-wide default
/// canvas. Multiplying by the live canvas width keeps balls looking
/// proportionally the same in both inline (~820px) and fullscreen
/// (~1248px) layouts, instead of being a fixed pixel size that appears
/// disproportionately large in the smaller canvas and disproportionately
/// small in the larger one. The upper bound is intentionally tight so
/// 100 balls can stack comfortably in the 600x400 inline canvas without
/// triggering `GAME_2D_MAX_BALL_AREA_RATIO`'s jam path.
///
/// # Returns
///
/// - `f64` - The radius in CSS pixels, scaled to the current canvas width.
pub(crate) fn random_ball_radius() -> f64 {
    let raw: f64 = js_sys::Math::random();
    let fraction: f64 =
        GAME_2D_BALL_MIN_RADIUS + raw * (GAME_2D_BALL_MAX_RADIUS - GAME_2D_BALL_MIN_RADIUS);
    let canvas_width: f64 = read_canvas_size(GAME_2D_CANVAS_SELECTOR)
        .map(|(w, _)| w)
        .unwrap_or(GAME_2D_CANVAS_WIDTH);
    fraction * (canvas_width / GAME_2D_CANVAS_WIDTH)
}

/// Creates a new ball at the given position with a random upward velocity.
///
/// # Arguments
///
/// - `Vector2D` - The spawn position.
///
/// # Returns
///
/// - `Ball` - The newly created ball.
pub(crate) fn create_ball(position: Vector2D) -> Ball {
    let angle: f64 = js_sys::Math::random() * PI - PI * 0.5;
    let speed: f64 = GAME_2D_SPAWN_VELOCITY + js_sys::Math::random() * GAME_2D_SPAWN_VELOCITY;
    Ball {
        position,
        velocity: Vector2D::new(angle.cos() * speed, -angle.sin() * speed.abs()),
        radius: random_ball_radius(),
        color: random_ball_color().to_string(),
    }
}

/// Creates a click event handler that spawns a new ball at the click position.
///
/// # Arguments
///
/// - `UseGame2D` - The 2D game state.
/// - `Rc<RefCell<Vec<Ball>>>` - The shared ball list.
/// - `CanvasCache` - The cached canvas element reference.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_2d_on_spawn_ball(
    state: UseGame2D,
    balls: Rc<RefCell<Vec<Ball>>>,
    canvas_cache: CanvasCache,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let current_count: usize = state.get_ball_count().get();
        if current_count >= GAME_2D_MAX_BALLS {
            return;
        }
        let (client_x, client_y): (f64, f64) = extract_mouse_client_position(&event);
        let Some(canvas_element) = canvas_cache.0.borrow().as_ref().cloned() else {
            return;
        };
        let rect: DomRect = canvas_element.get_bounding_client_rect();
        let canvas_width: f64 = canvas_element.client_width() as f64;
        let canvas_height: f64 = canvas_element.client_height() as f64;
        let position: Vector2D =
            map_client_to_canvas(client_x, client_y, &rect, canvas_width, canvas_height);
        let ball: Ball = create_ball(position);
        balls.borrow_mut().push(ball);
        let new_count: usize = balls.borrow().len();
        state.get_ball_count().set(new_count);
        let total: usize = state.get_total_spawned().get();
        state.get_total_spawned().set(total + 1);
    }))
}

/// Creates a click event handler that toggles the 2D game between running and paused.
///
/// # Arguments
///
/// - `UseGame2D` - The 2D game state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_2d_on_toggle_pause(state: UseGame2D) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_running().get();
        state.get_running().set(!current);
    }))
}

/// Creates a click event handler that enters landscape fullscreen mode for the 2D game.
///
/// Delegates to [`enter_game_2d_fullscreen`], which sets the active tab's
/// fullscreen signal, pushes a history entry, and reapplies safe-area
/// insets to the newly-mounted overlay container. The canvas itself is
/// not recreated — the running game loop, ball list, FPS counter, and
/// pause state all survive the transition.
///
/// # Arguments
///
/// - `UseGame2DFullscreen` - The 2D game fullscreen state.
/// - `Signal<bool>` - The fullscreen signal for the active tab.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_2d_on_enter_fullscreen(
    state: UseGame2DFullscreen,
    tab: Signal<bool>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        enter_game_2d_fullscreen(state, tab);
    }))
}

/// Creates a click event handler that exits landscape fullscreen mode for the 2D game.
///
/// Delegates to [`exit_game_2d_fullscreen`], which clears the active
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
pub(crate) fn game_2d_on_exit_fullscreen(tab: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        exit_game_2d_fullscreen(tab);
        Router::overlay_back(None);
    }))
}

/// Creates a click event handler that clears all balls from the canvas.
///
/// # Arguments
///
/// - `UseGame2D` - The 2D game state.
/// - `Rc<RefCell<Vec<Ball>>>` - The shared ball list.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_2d_on_clear(
    state: UseGame2D,
    balls: Rc<RefCell<Vec<Ball>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        balls.borrow_mut().clear();
        state.get_ball_count().set(0);
    }))
}

/// Extracts the client (viewport) coordinates from a mouse event.
///
/// # Arguments
///
/// - `&Event` - The mouse event.
///
/// # Returns
///
/// - `(f64, f64)` - The `(client_x, client_y)` coordinates.
pub(crate) fn extract_mouse_client_position(event: &Event) -> (f64, f64) {
    let mouse_event: &MouseEvent = event.unchecked_ref();
    (
        f64::from(mouse_event.client_x()),
        f64::from(mouse_event.client_y()),
    )
}

/// Extracts the client coordinates of the first changed touch from a `TouchEvent`.
///
/// Reads `changedTouches[0].clientX` and `changedTouches[0].clientY` from the
/// event via direct cast. Used by the touch spawn handler since
/// `TouchEvent` does not expose `clientX`/`clientY` directly on the event object.
///
/// # Arguments
///
/// - `&Event` - The native touch event.
///
/// # Returns
///
/// - `(f64, f64)` - The `(client_x, client_y)` coordinates of the first changed touch.
pub(crate) fn extract_touch_client_position(event: &Event) -> (f64, f64) {
    let touch_event: &TouchEvent = event.unchecked_ref();
    let touches: TouchList = touch_event.changed_touches();
    if touches.length() == 0 {
        return (0.0, 0.0);
    }
    let touch: Option<Touch> = touches.get(0);
    let Some(touch) = touch else {
        return (0.0, 0.0);
    };
    (f64::from(touch.client_x()), f64::from(touch.client_y()))
}

/// Creates a touch event handler that spawns a new ball at the touch position
/// and prevents default browser behavior to avoid click delay and page scrolling.
///
/// # Arguments
///
/// - `UseGame2D` - The 2D game state.
/// - `Rc<RefCell<Vec<Ball>>>` - The shared ball list.
/// - `CanvasCache` - The cached canvas element reference.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch start handler.
pub(crate) fn game_2d_on_touch_spawn_ball(
    state: UseGame2D,
    balls: Rc<RefCell<Vec<Ball>>>,
    canvas_cache: CanvasCache,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        let current_count: usize = state.get_ball_count().get();
        if current_count >= GAME_2D_MAX_BALLS {
            return;
        }
        let (client_x, client_y): (f64, f64) = extract_touch_client_position(&event);
        let Some(canvas_element) = canvas_cache.0.borrow().as_ref().cloned() else {
            return;
        };
        let rect: DomRect = canvas_element.get_bounding_client_rect();
        let canvas_width: f64 = canvas_element.client_width() as f64;
        let canvas_height: f64 = canvas_element.client_height() as f64;
        let position: Vector2D =
            map_client_to_canvas(client_x, client_y, &rect, canvas_width, canvas_height);
        let ball: Ball = create_ball(position);
        balls.borrow_mut().push(ball);
        let new_count: usize = balls.borrow().len();
        state.get_ball_count().set(new_count);
        let total: usize = state.get_total_spawned().get();
        state.get_total_spawned().set(total + 1);
    }))
}

/// Maps viewport client coordinates to canvas-internal coordinates.
///
/// The canvas-internal coordinate space now matches the canvas's actual
/// CSS pixel dimensions (read from `canvas.clientWidth` / `clientHeight`
/// at acquire time) instead of the static 600x400 default, so balls in
/// fullscreen mode are positioned in the full canvas rectangle, not
/// inside a 600x400 logical rectangle.
///
/// # Arguments
///
/// - `f64` - The client x coordinate.
/// - `f64` - The client y coordinate.
/// - `&DomRect` - The cached canvas bounding rect.
/// - `f64` - The canvas-internal width in CSS pixels.
/// - `f64` - The canvas-internal height in CSS pixels.
///
/// # Returns
///
/// - `Vector2D` - The canvas-space position.
pub(crate) fn map_client_to_canvas(
    client_x: f64,
    client_y: f64,
    canvas_rect: &DomRect,
    canvas_width: f64,
    canvas_height: f64,
) -> Vector2D {
    let rect_width: f64 = canvas_rect.width();
    let rect_height: f64 = canvas_rect.height();
    if rect_width < EPSILON || rect_height < EPSILON {
        return Vector2D::zero();
    }
    let scale_x: f64 = canvas_width / rect_width;
    let scale_y: f64 = canvas_height / rect_height;
    Vector2D::new(
        (client_x - canvas_rect.left()) * scale_x,
        (client_y - canvas_rect.top()) * scale_y,
    )
}

/// Consumes the `resize_dirty` debounce flag and rescales the ball list to
/// match the current canvas CSS box, returning `true` if a rescale actually
/// happened (the renderer's backing-store resize uses this to fire exactly
/// once per resize tick).
///
/// Two paths trigger the rescale:
///
/// 1. **Debounce-driven path** — `use_window_event("resize", ...)` (or the
///    synthetic `resize` event dispatched by `enter_game_2d_fullscreen`)
///    sets `resize_dirty_for_loop`. When the loop ticks and the flag is
///    set, this helper resizes the ball positions.
///
/// 2. **CSS-mismatch safety net** — the synthetic `resize` event sometimes
///    fires while the euv signal-driven DOM re-render is still pending,
///    so the debounce flag can be set *and* the canvas CSS box can still
///    be at the OLD dimensions when the loop ticks next. In that case the
///    debounce-driven path rescales against stale dimensions. To recover,
///    this helper also compares the current CSS box against the cached
///    one and runs an extra rescale when they diverge.
///
/// Running this *before* `update_balls` is what preserves ball motion
/// across fullscreen transitions: with the rescale done first, the wall
/// collision clamp inside `update_balls` no longer pins every ball to
/// the floor of the smaller canvas before the rescale can proportionally
/// shrink their positions. (The previous ordering — rescale *after*
/// `update_balls` — visibly reset balls to `y = radius` on exit.)
///
/// `canvas_selector` is unused here but kept in the signature for symmetry
/// with the Canvas 2D helper, which needs it to read the live CSS box via
/// `read_canvas_size`.
pub(crate) fn handle_rescale_dirty(
    resize_dirty_for_loop: &Rc<Cell<bool>>,
    last_canvas_size_for_loop: &Rc<RefCell<(f64, f64)>>,
    balls: &Rc<RefCell<Vec<Ball>>>,
    prev_for_loop: &Rc<RefCell<Vec<Vector2D>>>,
    canvas_cache: &CanvasCache,
    _canvas_selector: &'static str,
) -> bool {
    if !resize_dirty_for_loop.get() {
        return false;
    }
    resize_dirty_for_loop.set(false);
    let (new_w, new_h): (f64, f64) = canvas_cache
        .0
        .borrow()
        .as_ref()
        .map(|canvas| (canvas.client_width() as f64, canvas.client_height() as f64))
        .unwrap_or((0.0, 0.0));
    let (old_w, old_h) = *last_canvas_size_for_loop.borrow();
    if old_w > 0.0 && old_h > 0.0 && new_w > 0.0 && new_h > 0.0 {
        rescale_balls_to_canvas(
            &mut balls.borrow_mut(),
            &mut prev_for_loop.borrow_mut(),
            old_w,
            old_h,
            new_w,
            new_h,
        );
    }
    if new_w > 0.0 && new_h > 0.0 {
        *last_canvas_size_for_loop.borrow_mut() = (new_w, new_h);
    }
    true
}

/// Canvas 2D variant of [`handle_rescale_dirty`].
///
/// Resets the SSAA wrapper on a successful rescale (the Canvas 2D path
/// needs to re-acquire the SSAA canvas against the new CSS box; the
/// WebGL / WebGPU paths handle their own backing-store resize inside the
/// render block). Also runs the CSS-mismatch safety net that used to live
/// inline in `start_game_2d_loop`.
pub(crate) fn handle_rescale_dirty_canvas2d(
    resize_dirty_for_loop: &Rc<Cell<bool>>,
    last_canvas_size_for_loop: &Rc<RefCell<(f64, f64)>>,
    balls: &Rc<RefCell<Vec<Ball>>>,
    prev_for_loop: &Rc<RefCell<Vec<Vector2D>>>,
    canvas_cache: &CanvasCache,
    context_clone: &Rc<RefCell<Option<SsaaCanvas>>>,
) {
    let (css_w, css_h): (f64, f64) =
        read_canvas_size(GAME_2D_CANVAS_SELECTOR).unwrap_or((0.0, 0.0));
    let (cached_w, cached_h) = *last_canvas_size_for_loop.borrow();
    let css_mismatch: bool = css_w > 0.0
        && css_h > 0.0
        && (cached_w <= 0.0
            || cached_h <= 0.0
            || (css_w - cached_w).abs() > 1.5
            || (css_h - cached_h).abs() > 1.5);
    if resize_dirty_for_loop.get() {
        resize_dirty_for_loop.set(false);
        if cached_w > 0.0 && cached_h > 0.0 && css_w > 0.0 && css_h > 0.0 {
            rescale_balls_to_canvas(
                &mut balls.borrow_mut(),
                &mut prev_for_loop.borrow_mut(),
                cached_w,
                cached_h,
                css_w,
                css_h,
            );
        }
        if css_w > 0.0 && css_h > 0.0 {
            *last_canvas_size_for_loop.borrow_mut() = (css_w, css_h);
            // Drop the SSAA wrapper and the cached canvas element so
            // the next acquire runs against the new CSS box.
            *context_clone.borrow_mut() = None;
            *canvas_cache.0.borrow_mut() = None;
        }
    } else if css_mismatch && css_w > 0.0 && css_h > 0.0 {
        // CSS-mismatch safety net: the synthetic resize event
        // dispatched on fullscreen enter/exit fires while the signal-
        // driven DOM re-render is still pending, so the debounce flag
        // may not be set yet even though the canvas CSS box has
        // already changed. Detect the divergence and rescale anyway.
        rescale_balls_to_canvas(
            &mut balls.borrow_mut(),
            &mut prev_for_loop.borrow_mut(),
            cached_w,
            cached_h,
            css_w,
            css_h,
        );
        *last_canvas_size_for_loop.borrow_mut() = (css_w, css_h);
        *context_clone.borrow_mut() = None;
        *canvas_cache.0.borrow_mut() = None;
    }
}

/// Performs one physics update step on all balls.
///
/// Subdivides `delta_time` into `GAME_2D_PHYSICS_SUBSTEPS` smaller slices,
/// applying gravity, integrating velocity and position, handling wall
/// collisions with restitution, and resolving ball-to-ball collisions with
/// impulse-based response in each substep. The ball-to-ball pass is itself
/// repeated `GAME_2D_COLLISION_ITERATIONS` times per substep to converge on a
/// non-overlapping configuration when many balls are in contact.
///
/// # Arguments
///
/// - `&mut [Ball]` - The mutable ball slice.
/// - `f64` - The delta time in seconds.
pub(crate) fn update_balls(
    balls: &mut [Ball],
    delta_time: f64,
    canvas_width: f64,
    canvas_height: f64,
) {
    let sub_dt: f64 = delta_time / GAME_2D_PHYSICS_SUBSTEPS as f64;
    let gravity: Vector2D = Vector2D::new(0.0, GAME_2D_GRAVITY);
    for _ in 0..GAME_2D_PHYSICS_SUBSTEPS {
        let damping: f64 = (1.0 - GAME_2D_LINEAR_DAMPING * sub_dt).max(0.0);
        for ball in balls.iter_mut() {
            ball.velocity += gravity.scaled(sub_dt);
            ball.velocity = ball.velocity.scaled(damping);
            ball.position += ball.velocity.scaled(sub_dt);
        }
        for ball in balls.iter_mut() {
            resolve_wall_collision(ball, canvas_width, canvas_height);
        }
        for _ in 0..GAME_2D_COLLISION_ITERATIONS {
            let count: usize = balls.len();
            for i in 0..count {
                for j in (i + 1)..count {
                    let (left, right) = balls.split_at_mut(j);
                    resolve_ball_collision(&mut left[i], &mut right[0]);
                }
            }
        }
        // Last-resort convergence pass: shrink any ball that is still
        // severely overlapped after the iterative solver finished. Runs
        // once per substep so the cost is O(N^2) regardless of how
        // many iterations the main collision loop performed. Without
        // this pass, balls in a high-density pile can remain mutually
        // overlapping after every iteration; gravity then re-pushes
        // them together next substep and the impulse step oscillates
        // them vertically forever.
        resolve_stuck_balls(balls);
    }
}

/// Resolves a collision between a ball and the canvas walls.
///
/// Reflects velocity with restitution and clamps position inside bounds.
/// Bounds are passed in as `canvas_width` / `canvas_height` so the wall
/// rectangle tracks the canvas's actual CSS pixel dimensions in both
/// inline (~820x547) and fullscreen (~1248x750) layouts, instead of the
/// static 600x400 default.
///
/// # Arguments
///
/// - `&mut Ball` - The ball to check and correct.
/// - `f64` - The canvas width in CSS pixels (wall X bound).
/// - `f64` - The canvas height in CSS pixels (wall Y bound).
pub(crate) fn resolve_wall_collision(ball: &mut Ball, canvas_width: f64, canvas_height: f64) {
    if ball.position.get_x() - ball.radius < 0.0 {
        ball.position.set_x(ball.radius);
        let velocity_x: f64 = ball.velocity.get_x();
        ball.velocity.set_x(velocity_x.abs() * GAME_2D_RESTITUTION);
    }
    if ball.position.get_x() + ball.radius > canvas_width {
        ball.position.set_x(canvas_width - ball.radius);
        let velocity_x: f64 = ball.velocity.get_x();
        ball.velocity.set_x(-velocity_x.abs() * GAME_2D_RESTITUTION);
    }
    if ball.position.get_y() - ball.radius < 0.0 {
        ball.position.set_y(ball.radius);
        let velocity_y: f64 = ball.velocity.get_y();
        ball.velocity.set_y(velocity_y.abs() * GAME_2D_RESTITUTION);
    }
    if ball.position.get_y() + ball.radius > canvas_height {
        ball.position.set_y(canvas_height - ball.radius);
        let velocity_y: f64 = ball.velocity.get_y();
        ball.velocity.set_y(-velocity_y.abs() * GAME_2D_RESTITUTION);
    }
}

/// Rescales ball positions and the previous-step buffer so they
/// remain proportional when the canvas switches between inline
/// (~820x547) and fullscreen (~1248x750) layouts.
///
/// Without this scaling, balls that were spawned in fullscreen retain
/// their fullscreen coordinates after exiting, and `resolve_wall_collision`
/// clamps them all to the floor of the smaller inline canvas — producing
/// a dense pile-up that the impulse solver cannot separate (balls all
/// want the same y, gravity keeps pulling them back, collision
/// iterations are bounded).
///
/// **The ball radius is intentionally NOT rescaled.** Each ball's
/// `radius` is set at spawn time from `random_ball_radius` (which
/// already scales to the current canvas width), and it stays at that
/// absolute pixel value for the ball's lifetime. Rescaling radius on
/// every fullscreen toggle would let the radius drift — a 30px ball
/// spawned in inline becomes 41px after entering fullscreen (1.371x
/// scale), then 37px after exiting (0.657x scale), losing roughly 10%
/// per round-trip and visibly shrinking over a few cycles. Keeping
/// radius fixed at the spawn-time value gives a stable visual size that
/// does not depend on how many times the user has toggled fullscreen.
///
/// `old_width` / `old_height` are the canvas dimensions the balls were
/// last physics-stepped against; `new_width` / `new_height` are the
/// dimensions they will be physics-stepped against next. Ball
/// positions are rescaled by `new/old` per axis. Positions are clamped
/// to `[radius, new_w - radius]` x `[radius, new_h - radius]` as a
/// safety net for any ball that ended up outside the new bounds due
/// to compounded clamping or ball-to-ball overlap.
///
/// The `prev_positions` buffer is rescaled in lockstep so the
/// `interpolate_balls` extrapolation between physics steps keeps
/// producing visually consistent positions across the resize.
///
/// **Ball count trimming.** When the new canvas is smaller than the
/// old one (e.g. exiting fullscreen), the total ball cross-section
/// area (`pi * sum(radius^2)`) is recomputed against the new canvas
/// area. If it exceeds [`GAME_2D_MAX_BALL_AREA_RATIO`] of the canvas
/// area, the oldest balls are trimmed from the front of the list
/// (which preserves the most recently spawned / active balls) until
/// the ratio is at or below the limit. The `prev_positions` buffer is
/// truncated in lockstep so `interpolate_balls` does not index past
/// the end of the live ball list. Without this trim, a fullscreen
/// session that spawned 80+ balls would dump all of them into a
/// 600x400 inline canvas — far too many for the bounded impulse
/// solver to separate, producing the same dense floor-pile that the
/// rescaling alone was originally added to prevent. The trim is a
/// no-op when the new canvas is *larger* than the old one (entering
/// fullscreen from inline) because density only goes down in that
/// direction.
///
/// # Arguments
///
/// - `&mut Vec<Ball>` - The mutable ball list (length may shrink).
/// - `&mut Vec<Vector2D>` - The previous-step position buffer
///   (truncated to match the trimmed ball count).
/// - `f64` - The previous canvas width in CSS pixels.
/// - `f64` - The previous canvas height in CSS pixels.
/// - `f64` - The new canvas width in CSS pixels.
/// - `f64` - The new canvas height in CSS pixels.
pub(crate) fn rescale_balls_to_canvas(
    balls: &mut Vec<Ball>,
    prev_positions: &mut Vec<Vector2D>,
    old_width: f64,
    old_height: f64,
    new_width: f64,
    new_height: f64,
) {
    if old_width <= 0.0 || old_height <= 0.0 || new_width <= 0.0 || new_height <= 0.0 {
        return;
    }
    let scale_x: f64 = new_width / old_width;
    let scale_y: f64 = new_height / old_height;
    for ball in balls.iter_mut() {
        ball.position.set_x(ball.position.get_x() * scale_x);
        ball.position.set_y(ball.position.get_y() * scale_y);
        // Radius is intentionally preserved — see the function-level
        // doc for why. Only position is rescaled to the new canvas.
        let max_x: f64 = (new_width - ball.radius).max(ball.radius);
        let max_y: f64 = (new_height - ball.radius).max(ball.radius);
        if ball.position.get_x() < ball.radius {
            ball.position.set_x(ball.radius);
        } else if ball.position.get_x() > max_x {
            ball.position.set_x(max_x);
        }
        if ball.position.get_y() < ball.radius {
            ball.position.set_y(ball.radius);
        } else if ball.position.get_y() > max_y {
            ball.position.set_y(max_y);
        }
    }
    for prev in prev_positions.iter_mut() {
        prev.set_x(prev.get_x() * scale_x);
        prev.set_y(prev.get_y() * scale_y);
    }
    // Ball count trim: when the new canvas is smaller than the old one,
    // a previously-spawned dense ball list now overflows the available
    // area. Trim oldest balls (front of list, since `create_ball` always
    // `push`es) until the area ratio is at or below the cap. Only
    // triggered when the new area is *smaller* than the old area to
    // avoid penalising the player for entering fullscreen.
    if new_width * new_height < old_width * old_height {
        let canvas_area: f64 = new_width * new_height;
        if canvas_area > 0.0 {
            // Compute total ball cross-section area using a running sum;
            // bail out of the trim loop as soon as the ratio is below
            // the cap to avoid an unnecessary O(N) walk when the list
            // already fits.
            let mut total_ball_area: f64 = balls
                .iter()
                .map(|b| b.radius * b.radius * std::f64::consts::PI)
                .sum();
            let cap: f64 = canvas_area * GAME_2D_MAX_BALL_AREA_RATIO;
            let mut trim_count: usize = 0;
            while total_ball_area > cap && trim_count < balls.len() {
                let removed: &Ball = &balls[trim_count];
                total_ball_area -= removed.radius * removed.radius * std::f64::consts::PI;
                trim_count += 1;
            }
            if trim_count > 0 {
                balls.drain(..trim_count);
                // Truncate the prev-positions buffer in lockstep so the
                // interpolation step never indexes past the trimmed
                // ball count. `snapshot_ball_positions` truncates it
                // anyway on the next physics tick, but doing it here
                // keeps the rendered position consistent for the
                // single frame between rescale and the next snapshot.
                if prev_positions.len() > balls.len() {
                    prev_positions.truncate(balls.len());
                } else if prev_positions.len() < balls.len() {
                    // Pad with the current ball positions so the
                    // interpolator has something to read; newly
                    // surviving balls render at their current
                    // position (no interpolation) until the next
                    // snapshot tick.
                    while prev_positions.len() < balls.len() {
                        let idx: usize = prev_positions.len();
                        prev_positions.push(balls[idx].position);
                    }
                }
            }
        }
    }
}

/// Resolves a collision between two balls using positional projection
/// plus an impulse response that dissipates energy on contact.
///
/// The solver has two distinct phases:
///
/// 1. **Positional projection** (always, when overlapping). Each ball is
///    moved along the contact normal so the overlap is fully closed.
///    Crucially, the projection uses *inverse-mass* weighting so the
///    heavier ball moves less than the lighter one — this prevents the
///    "tug of war" failure mode where two equal-mass balls in a tightly
///    packed pile end up oscillating back and forth instead of coming
///    to rest. Without this phase, the impulse step alone can never
///    eliminate overlap because impulses only change velocity, not
///    position; the next gravity step would re-create the overlap,
///    producing the infinite-collision-loop regression that this
///    function exists to prevent.
///
/// 2. **Impulse response** (only when the balls are converging along
///    the contact normal, i.e. `velocity_along_normal < 0`). When the
///    balls are already separating (e.g. an earlier iteration in the
///    same substep pushed them apart), the impulse step is skipped so
///    the restitution coefficient is never applied as a *gain* on the
///    separating velocity. Otherwise, energy is preserved via the
///    standard `-(1 + e) * v_n / (1/m_a + 1/m_b)` formula and the
///    `GAME_2D_RESTITUTION` coefficient is applied multiplicatively to
///    the post-impulse normal velocity so the bounce dissipates over
///    time instead of running away.
///
/// # Arguments
///
/// - `&mut Ball` - The first ball.
/// - `&mut Ball` - The second ball.
pub(crate) fn resolve_ball_collision(a: &mut Ball, b: &mut Ball) {
    let delta: Vector2D = b.position - a.position;
    let distance: f64 = delta.magnitude();
    let radius_sum: f64 = a.radius + b.radius;
    if distance >= radius_sum {
        return;
    }
    let normal: Vector2D = if distance < EPSILON {
        // Two perfectly co-located balls: pick an arbitrary stable
        // direction so the projection still separates them on the next
        // physics step. The previous behaviour used a fixed `right()`
        // unit vector, which is fine — neither ball is preferred.
        Vector2D::right()
    } else {
        delta.scaled(1.0 / distance)
    };
    let overlap: f64 = radius_sum - distance;
    let mass_a: f64 = a.radius * a.radius;
    let mass_b: f64 = b.radius * b.radius;
    let total_mass: f64 = mass_a + mass_b;
    // Phase 1: positional projection. Fully close the overlap along the
    // contact normal, weighted by inverse mass so the heavier ball moves
    // less. This is what makes the solver *converge* under high density:
    // even when balls are jammed and the impulse step cannot separate
    // them any further, every iteration of the outer loop closes more of
    // the overlap until the balls are physically non-overlapping.
    a.position -= normal.scaled(overlap * (mass_b / total_mass));
    b.position += normal.scaled(overlap * (mass_a / total_mass));
    // Phase 2: impulse response. Only apply when the balls are
    // *converging* along the contact normal; if they are already
    // separating (e.g. an earlier iteration in the same substep pushed
    // them apart and gravity has not yet pulled them back together),
    // skip the impulse to avoid amplifying the separation into an
    // unrealistic bounce.
    let relative_velocity: Vector2D = b.velocity - a.velocity;
    let velocity_along_normal: f64 = relative_velocity.dot(normal);
    if velocity_along_normal > 0.0 {
        return;
    }
    let impulse_magnitude: f64 =
        -(1.0 + GAME_2D_RESTITUTION) * velocity_along_normal / (1.0 / mass_a + 1.0 / mass_b);
    let impulse: Vector2D = normal.scaled(impulse_magnitude);
    a.velocity -= impulse.scaled(1.0 / mass_a);
    b.velocity += impulse.scaled(1.0 / mass_b);
}

/// Last-resort fallback for jammed balls.
///
/// Runs after `GAME_2D_COLLISION_ITERATIONS` of [`resolve_ball_collision`]
/// passes have failed to fully clear overlap because the canvas is too
/// densely packed. Counts the number of times a single ball was still
/// overlapping a neighbour by more than [`GAME_2D_STUCK_MIN_OVERLAP`]
/// pixels after the iterative solver finished; if the count exceeds
/// `stuck_threshold`, the ball's `radius` is multiplied by
/// [`GAME_2D_STUCK_RADIUS_SHRINK`] so its visual size shrinks
/// imperceptibly (~3%) while its effective collision footprint
/// decreases enough to slip into the remaining gap on the next substep.
///
/// The shrink is intentionally permanent (`radius` is updated in place)
/// rather than a per-substep multiplier: repeatedly oscillating the
/// radius around the jam threshold would itself create a visible
/// pulsing effect. The 3% shrink applied once per jam event is below
/// perceptual noise for a ball whose radius is `>= 8px` (0.24px,
/// smaller than a single anti-aliasing band) but cumulatively opens
/// enough slack for the solver to converge on subsequent substeps.
///
/// `stuck_threshold = total_balls / 4` is a heuristic that tolerates a
/// handful of genuine pairwise contacts (e.g. the bottom row of a stable
/// stack where each ball legitimately touches two neighbours) without
/// triggering the shrink on every contact.
pub(crate) fn resolve_stuck_balls(balls: &mut [Ball]) {
    let count: usize = balls.len();
    if count == 0 {
        return;
    }
    let stuck_threshold: usize = count / 4;
    let mut stuck_count: Vec<u32> = vec![0; count];
    for i in 0..count {
        let (left, right) = balls.split_at(i + 1);
        let ball_i: &Ball = &left[i];
        for (offset, ball_j) in right.iter().enumerate() {
            let j: usize = i + 1 + offset;
            let dx: f64 = ball_j.position.get_x() - ball_i.position.get_x();
            let dy: f64 = ball_j.position.get_y() - ball_i.position.get_y();
            let distance_sq: f64 = dx * dx + dy * dy;
            let radius_sum: f64 = ball_i.radius + ball_j.radius;
            // Only count "real" overlap: a half-pixel sliver below the
            // stuck threshold is treated as converged to avoid the shrink
            // firing on every contact in a stable stack.
            if distance_sq < (radius_sum - GAME_2D_STUCK_MIN_OVERLAP).max(0.0).powi(2) {
                stuck_count[i] = stuck_count[i].saturating_add(1);
                stuck_count[j] = stuck_count[j].saturating_add(1);
            }
        }
    }
    for (ball, &hits) in balls.iter_mut().zip(stuck_count.iter()) {
        if hits as usize > stuck_threshold {
            ball.radius *= GAME_2D_STUCK_RADIUS_SHRINK;
        }
    }
}

/// Snapshots the current ball positions into the previous-step buffer.
///
/// Truncates stale entries after a Clear and seeds newly spawned balls with
/// their current position so they render uninterpolated until the next
/// physics step.
///
/// # Arguments
///
/// - `&mut Vec<Vector2D>` - The previous-step position buffer to overwrite.
/// - `&[Ball]` - The current ball list.
pub(crate) fn snapshot_ball_positions(prev_positions: &mut Vec<Vector2D>, balls: &[Ball]) {
    prev_positions.truncate(balls.len());
    for (index, ball) in balls.iter().enumerate() {
        if index < prev_positions.len() {
            prev_positions[index] = ball.position;
        } else {
            prev_positions.push(ball.position);
        }
    }
}

/// Builds a render copy of the ball list with positions interpolated between
/// the previous physics step and the current one.
///
/// `alpha` is the leftover accumulator fraction (`accumulator / timestep`)
/// clamped to `[0.0, 1.0]`. Interpolating at render time decouples the
/// 60 Hz physics cadence from the display refresh rate: without it a 120 Hz
/// display presents each physics state twice (visible stepping), and a 60 Hz
/// display alternates zero- and double-step frames (visible judder), even
/// though the FPS counter reads high in both cases. Balls without a previous
/// entry (just spawned) render at their current position.
///
/// # Arguments
///
/// - `&[Ball]` - The current ball list.
/// - `&[Vector2D]` - The previous-step position buffer.
/// - `f64` - The interpolation factor in `[0.0, 1.0]`.
///
/// # Returns
///
/// - `Vec<Ball>` - The interpolated ball list for rendering.
pub(crate) fn interpolate_balls(
    balls: &[Ball],
    prev_positions: &[Vector2D],
    alpha: f64,
) -> Vec<Ball> {
    balls
        .iter()
        .enumerate()
        .map(|(index, ball): (usize, &Ball)| {
            let mut render_ball: Ball = ball.clone();
            if let Some(prev_position) = prev_positions.get(index) {
                render_ball.position = prev_position.lerp(ball.position, alpha);
            }
            render_ball
        })
        .collect()
}

/// Renders all balls onto the supplied SSAA canvas and presents the result.
///
/// The clear rect uses the canvas's actual CSS pixel dimensions so the
/// entire backing buffer is wiped before each redraw, in both inline and
/// fullscreen layouts. Draws onto the offscreen context using logical
/// CSS-pixel coordinates, then delegates to `present()` for HiDPI-
/// friendly downscaling.
///
/// # Arguments
///
/// - `&SsaaCanvas` - The SSAA canvas wrapper.
/// - `&[Ball]` - The ball list to render.
/// - `f64` - The canvas width in CSS pixels (clear-rect bound).
/// - `f64` - The canvas height in CSS pixels (clear-rect bound).
pub(crate) fn render_balls_with_ssaa(
    ssaa_canvas: &SsaaCanvas,
    balls: &[Ball],
    canvas_width: f64,
    canvas_height: f64,
) {
    let context: &CanvasRenderingContext2d = ssaa_canvas.get_offscreen_context();
    context.clear_rect(0.0, 0.0, canvas_width, canvas_height);
    let fill_style_key: JsValue = JsValue::from_str(GAME_2D_PROPERTY_FILL_STYLE);
    for ball in balls {
        let _ = Reflect::set(context, &fill_style_key, &JsValue::from_str(&ball.color));
        context.begin_path();
        let _ = context.arc(
            ball.position.get_x(),
            ball.position.get_y(),
            ball.radius,
            0.0,
            std::f64::consts::TAU,
        );
        context.fill();
    }
    ssaa_canvas.present();
}

/// Queries the 2D game canvas element and constructs an SSAA wrapper for it.
///
/// Picks the SSAA scale factor via the same desktop/mobile heuristic used
/// for the 3D game (2x on desktop, 1x on mobile). The DPR multiplier is
/// applied automatically inside `SsaaCanvas::from_selector_with_scale`.
///
/// Returns the underlying display element alongside the SSAA wrapper so
/// that click handlers can map viewport coordinates into canvas space.
///
/// # Returns
///
/// Reads the canvas element's CSS layout dimensions.
///
/// Uses `getBoundingClientRect()` to read the actual CSS box size
/// (width/height after layout), not `clientWidth`/`clientHeight` which
/// in Chrome track `canvas.width`/`canvas.height` (the backing-store
/// size). Reading the CSS box is critical during fullscreen
/// transitions: the moment the user clicks Enter Fullscreen the CSS
/// layout flips to the new size, but the canvas backing store still
/// holds the previous size. If we used `clientWidth` here, the first
/// resize tick would re-acquire the SSAA wrapper at the OLD backing
/// dimensions, never call `set_width`, and leave the canvas display
/// showing the previous-size content stretched to the new CSS box —
/// a visible "first-frame distortion" that recovers only once the
/// browser repaints after our later frames have grown the backing
/// store. `getBoundingClientRect` returns the target CSS size
/// immediately, so `SsaaCanvas::from_selector_with_scale` resizes the
/// backing store to match on the very first post-resize frame.
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

/// Acquires the 2D game canvas and its SSAA wrapper, sized to the
/// canvas's current CSS pixel dimensions.
///
/// Reads `canvas.clientWidth` and `canvas.clientHeight` from the
/// live DOM so the SSAA backing buffer tracks the canvas's actual
/// rendered size in both inline (~820x547) and fullscreen
/// (~1248x750) layouts on a 1280x800 viewport. The game physics
/// bounds (resolve_wall_collision), click mapping
/// (map_client_to_canvas), and clear_rect calls all read the same
/// runtime dimensions via `read_canvas_size` so balls render and
/// bounce against the actual canvas edges instead of the static
/// 600x400 default.
///
/// # Returns
///
/// - `Option<(HtmlCanvasElement, SsaaCanvas)>` - The display canvas plus
///   the SSAA wrapper, or `None` if the canvas element was not found.
pub(crate) fn acquire_game_2d_ssaa_canvas() -> Option<(HtmlCanvasElement, SsaaCanvas)> {
    let window_value: Window = window()?;
    let is_mobile: bool = window_value
        .inner_width()
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .is_some_and(|width: f64| width < 768.0);
    let scale_factor: f64 = if is_mobile { 1.0 } else { 2.0 };
    let (canvas_width, canvas_height): (f64, f64) = read_canvas_size(GAME_2D_CANVAS_SELECTOR)?;
    let ssaa_canvas: SsaaCanvas = SsaaCanvas::from_selector_with_scale(
        GAME_2D_CANVAS_SELECTOR,
        canvas_width,
        canvas_height,
        scale_factor,
    )?;
    let document_value: Document = window_value.document()?;
    let element: Element = document_value
        .query_selector(GAME_2D_CANVAS_SELECTOR)
        .ok()
        .flatten()?;
    let display_canvas: HtmlCanvasElement = element.unchecked_into();
    Some((display_canvas, ssaa_canvas))
}

/// Draws the loading text centered on a 2D canvas using SSAA.
///
/// Used by the Canvas 2D tab (drawn directly on the game canvas) and by
/// the WebGPU/WebGL tabs (drawn on a separate 2D overlay canvas stacked
/// above the GPU canvas, since the GPU canvas cannot be drawn into via a
/// 2D context). `target_selector` is the canvas the text is rendered
/// onto; `color_source_selector` is the element whose computed style is
/// queried for the `--text-on-accent` CSS variable so the text colour
/// matches the surrounding theme. The two selectors coincide for the
/// Canvas 2D tab; for the GPU tabs the overlay canvas is the target
/// while the GPU canvas is the colour source.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `&str` - Shared reference to a `str`.
pub(crate) fn draw_game_2d_loading(target_selector: &str, color_source_selector: &str) {
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
    let fill_style_key: JsValue = JsValue::from_str(GAME_2D_PROPERTY_FILL_STYLE);
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
                .get_property_value(GAME_2D_PROPERTY_BACKGROUND_COLOR)
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
    let font_size: f64 = canvas_height * GAME_2D_LOADING_FONT_SIZE_RATIO;
    let font: String = format!("{font_size}px {GAME_2D_LOADING_FONT_FAMILY}");
    // Read the loading text color from the CSS variable via getComputedStyle.
    let loading_color: String = computed_style
        .and_then(|style: CssStyleDeclaration| {
            style.get_property_value(GAME_2D_LOADING_COLOR_VAR).ok()
        })
        .filter(|color: &String| !color.is_empty())
        .unwrap_or_else(|| "#ffffff".to_string());
    let _ = Reflect::set(context, &fill_style_key, &JsValue::from_str(&loading_color));
    context.set_font(&font);
    context.set_text_align("center");
    context.set_text_baseline("middle");
    let _ = context.fill_text(
        GAME_2D_LOADING_TEXT,
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

/// Starts the 2D game loop driven by `requestAnimationFrame`.
///
/// Runs a fixed-timestep accumulator loop that updates physics at a constant
/// rate and renders every frame, interpolating ball positions between the
/// previous and current physics steps so motion stays smooth at any display
/// refresh rate. The canvas context is cached once at startup
/// to avoid per-frame DOM queries. Updates the FPS signal approximately every
/// second.
///
/// # Arguments
///
/// - `UseGame2D` - The 2D game state for signal updates.
/// - `Rc<RefCell<Vec<Ball>>>` - The shared ball list.
/// - `CanvasCache` - The shared canvas element cache for event handlers.
pub(crate) fn start_game_2d_loop(
    state: UseGame2D,
    balls: Rc<RefCell<Vec<Ball>>>,
    canvas_cache: CanvasCache,
) {
    let canvas_ssaa: Rc<RefCell<Option<SsaaCanvas>>> = Rc::new(RefCell::new(None));
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let accumulator: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let prev_positions: Rc<RefCell<Vec<Vector2D>>> = Rc::new(RefCell::new(Vec::new()));
    // Tracks the canvas dimensions the balls were last physics-stepped
    // against, so a fullscreen <-> inline transition can rescale ball
    // positions and radii in lockstep with the new backing buffer.
    let last_canvas_size: Rc<RefCell<(f64, f64)>> = Rc::new(RefCell::new((0.0, 0.0)));
    let acc_clone: Rc<Cell<f64>> = accumulator.clone();
    let last_clone: Rc<Cell<f64>> = last_time.clone();
    let frame_clone: Rc<Cell<u32>> = frame_count.clone();
    let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
    let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_clone: RafClosureCell = closure_cell.clone();
    let context_clone: Rc<RefCell<Option<SsaaCanvas>>> = canvas_ssaa.clone();
    let dirty_clone: Rc<Cell<bool>> = resize_dirty.clone();
    let prev_clone: Rc<RefCell<Vec<Vector2D>>> = prev_positions.clone();
    let last_canvas_size_clone: Rc<RefCell<(f64, f64)>> = last_canvas_size.clone();
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        if game_2d_canvas_detached(GAME_2D_CANVAS_SELECTOR) {
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
            GAME_2D_FIXED_TIMESTEP
        } else {
            (current_time - prev).min(0.25)
        };
        last_clone.set(current_time);
        // Resize-rescale must run BEFORE the physics tick so that
        // `update_balls` does not first clamp positions against the
        // new (smaller) canvas and then have the rescale try to
        // proportionally shrink those clamped values. See the WebGL /
        // WebGPU loops for the full rationale; the same physics and
        // the same regression live here. `handle_rescale_dirty` also
        // runs a CSS-mismatch safety net, replacing the per-frame
        // `css_mismatch` block that used to live in the `else` arm of
        // the resize check below.
        handle_rescale_dirty_canvas2d(
            &dirty_clone,
            &last_canvas_size_clone,
            &balls,
            &prev_clone,
            &canvas_cache,
            &context_clone,
        );
        if state.get_running().get() {
            // Accumulate only while running: a paused accumulator would grow
            // unboundedly and burst catch-up physics steps on resume.
            acc_clone.set(acc_clone.get() + frame_time);
            while acc_clone.get() >= GAME_2D_FIXED_TIMESTEP {
                snapshot_ball_positions(&mut prev_clone.borrow_mut(), &balls.borrow());
                let (cw, ch): (f64, f64) = canvas_cache
                    .0
                    .borrow()
                    .as_ref()
                    .map(|canvas| (canvas.client_width() as f64, canvas.client_height() as f64))
                    .unwrap_or((0.0, 0.0));
                update_balls(&mut balls.borrow_mut(), GAME_2D_FIXED_TIMESTEP, cw, ch);
                acc_clone.set(acc_clone.get() - GAME_2D_FIXED_TIMESTEP);
            }
        }
        let alpha: f64 = (acc_clone.get() / GAME_2D_FIXED_TIMESTEP).clamp(0.0, 1.0);
        // Resize handling now lives in `handle_rescale_dirty_canvas2d`
        // above (runs before the physics tick to preserve velocity).
        // The old in-loop blocks that lived here were prone to wall
        // clamping the balls against the new smaller canvas before
        // the rescale proportionally shrank those clamped values,
        // visibly resetting motion on fullscreen exit.
        if context_clone.borrow().is_none()
            && let Some((canvas_el, ssaa_canvas)) = acquire_game_2d_ssaa_canvas()
        {
            *canvas_cache.0.borrow_mut() = Some(canvas_el);
            *context_clone.borrow_mut() = Some(ssaa_canvas);
        }
        if let Some(ssaa_canvas) = context_clone.borrow().as_ref() {
            let (canvas_width, canvas_height): (f64, f64) = canvas_cache
                .0
                .borrow()
                .as_ref()
                .map(|canvas| (canvas.client_width() as f64, canvas.client_height() as f64))
                .unwrap_or((0.0, 0.0));
            let render_balls: Vec<Ball> =
                interpolate_balls(&balls.borrow(), &prev_clone.borrow(), alpha);
            render_balls_with_ssaa(ssaa_canvas, &render_balls, canvas_width, canvas_height);
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
    let state_for_start: UseGame2D = state;
    let start_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        state_for_start.get_loaded().set(true);
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
            GAME_2D_LOOP_START_DELAY_MILLIS,
        )
        .unwrap_or_default();
    start_timeout_clone.set(Some(timeout_id));
    let loading_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        draw_game_2d_loading(GAME_2D_CANVAS_SELECTOR, GAME_2D_CANVAS_SELECTOR);
    }));
    let loading_callback: Function = loading_closure.as_ref().unchecked_ref::<Function>().clone();
    loading_closure.forget();
    let _ =
        start_window.set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
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
                GAME_2D_RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        timer_for_event.set(Some(new_timer));
    });
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
    });
}

/// Creates the reactive state signals for the 2D WebGPU demo.
///
/// Allocates hook slots in this fixed order:
/// 1. fps
/// 2. loaded
/// 3. active
/// 4. loop_started
/// 5. init_error_code
///
/// # Returns
///
/// - `UseGame2DWebGpu` - A `UseGame2DWebGpu` value.
pub(crate) fn use_game_2d_webgpu_state() -> UseGame2DWebGpu {
    UseGame2DWebGpu {
        fps: App::use_signal(|| 0.0),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        init_error_code: App::use_signal(|| ""),
    }
}

/// Creates the reactive state signals for the 2D WebGL demo tab.
///
/// Allocates hook slots in this fixed order:
/// 1. fps
/// 2. loaded
/// 3. active
/// 4. loop_started
/// 5. init_error_code
///
/// # Returns
///
/// - `UseGame2DWebGl` - The WebGL demo state.
pub(crate) fn use_game_2d_webgl_state() -> UseGame2DWebGl {
    UseGame2DWebGl {
        fps: App::use_signal(|| 0.0),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        init_error_code: App::use_signal(|| ""),
    }
}

/// Queries a canvas element by CSS selector.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the canvas element.
///
/// # Returns
///
/// - `Option<HtmlCanvasElement>` - The canvas element, if present.
pub(crate) fn game_2d_canvas_element(canvas_selector: &str) -> Option<HtmlCanvasElement> {
    let window_value: Window = window()?;
    let document_value: Document = window_value.document()?;
    let element: Element = document_value
        .query_selector(canvas_selector)
        .ok()
        .flatten()?;
    Some(element.unchecked_into())
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
pub(crate) fn game_2d_canvas_detached(canvas_selector: &str) -> bool {
    game_2d_canvas_element(canvas_selector).is_none()
}

/// Parses a `#rrggbb` CSS color string into 0.0-1.0 RGB floats.
///
/// Ball colors come from the `GAME_2D_BALL_COLORS` palette, which is
/// authored for CSS (`fillStyle`); the GPU shaders need plain floats.
/// Malformed input falls back to white.
///
/// # Arguments
///
/// - `&str` - The CSS hex color string.
///
/// # Returns
///
/// - `(f32, f32, f32)` - The `(r, g, b)` channels in 0.0-1.0 range.
pub(crate) fn game_2d_hex_to_rgb(color: &str) -> (f32, f32, f32) {
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
pub(crate) fn game_2d_canvas_clear_color(canvas_selector: &str) -> (f64, f64, f64) {
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

/// Converts one ball into its GPU record: `(x, y, radius, unused)` plus
/// `(r, g, b, a)`, matching the `BallData` layout in the balls shaders.
///
/// The radius is multiplied by the live `dpr` so the shader's per-fragment
/// `dot(uv, uv) > 1.0` discard draws a disc that fills `radius * dpr`
/// physical pixels. The Canvas 2D tab draws the same `radius` CSS units
/// through a 2x SSAA backing store and then downscales, so it already
/// anti-aliases the edge over `2 * radius` physical pixels and appears at
/// ~`radius` CSS pixels. Multiplying by `dpr` here makes the WebGL /
/// WebGPU paths land on the same visual size at DPR=2 (and matches
/// exactly at DPR=1 because no scaling is needed).
///
/// # Arguments
///
/// - `&Ball` - The ball to convert.
/// - `f64` - The live `window.devicePixelRatio` (`>= 1.0`).
///
/// # Returns
///
/// - `([f32; 4], [f32; 4])` - Position-and-radius and color vec4s.
///
/// Packs one ball into the vec4 pair consumed by the WebGPU and WebGL
/// ball shaders: position and radius in the first vec4, RGB color plus
/// alpha in the second.
///
/// **The radius is written in CSS-pixel units, NOT multiplied by `dpr`.**
/// The shader projects positions into clip space using the CSS-pixel
/// `u_canvas_size` uniform, so `radius` lives in the same CSS-unit space
/// as `ball.position`. The renderer's backing store is sized to
/// `client_width * dpr` (i.e. physical pixels), but the shader's clip
/// space is the same [-1, 1] NDC regardless of backing resolution — the
/// viewport maps NDC onto the entire physical backing, so a CSS-unit
/// radius of `r` lands on `r * dpr` physical pixels and reads back as
/// `r` CSS pixels after the browser's automatic downscale to the
/// element's CSS box. That is exactly the visual size the Canvas 2D
/// tab produces via the SSAA back-and-downscale path, so the two
/// renderers agree without any explicit DPR compensation.
///
/// The `dpr` parameter is intentionally ignored and is kept on the
/// signature only so callers can pass it through alongside the other
/// paths and we can revisit this if a future renderer breaks the
/// clip-space-mapping assumption.
fn game_2d_ball_gpu_record(ball: &Ball, _dpr: f64) -> ([f32; 4], [f32; 4]) {
    let (r, g, b) = game_2d_hex_to_rgb(&ball.color);
    (
        [
            ball.position.get_x() as f32,
            ball.position.get_y() as f32,
            ball.radius as f32,
            0.0,
        ],
        [r, g, b, 1.0],
    )
}

/// Packs the ball list into the uniform layout consumed by the WebGPU
/// balls shader: a `canvas_size` vec2 plus padding, followed by one
/// interleaved `BallData` (pos_radius, color) per ball. The result is
/// always padded out to `GAME_2D_MAX_BALLS` entries so the fixed-size
/// uniform buffer is fully overwritten each frame and stale balls from
/// before a Clear never linger.
///
/// # Arguments
///
/// - `&[Ball]` - The ball list for this frame.
/// - `f64` - The canvas width in CSS pixels (u-vec2 canvas_size.x).
/// - `f64` - The canvas height in CSS pixels (u-vec2 canvas_size.y).
/// - `f64` - The live `window.devicePixelRatio` (`>= 1.0`), forwarded to
///   [`game_2d_ball_gpu_record`] so each ball's on-screen radius scales
///   with the backing store and visually matches the Canvas 2D tab.
///
/// # Returns
///
/// - `Vec<f32>` - The packed uniform data (4 + `GAME_2D_MAX_BALLS * 8` floats).
fn pack_game_2d_balls_webgpu(
    balls: &[Ball],
    canvas_width: f64,
    canvas_height: f64,
    dpr: f64,
) -> Vec<f32> {
    let mut data: Vec<f32> = vec![canvas_width as f32, canvas_height as f32, 0.0, 0.0];
    for ball in balls {
        let (pos_radius, color) = game_2d_ball_gpu_record(ball, dpr);
        data.extend_from_slice(&pos_radius);
        data.extend_from_slice(&color);
    }
    data.resize(4 + GAME_2D_MAX_BALLS * 8, 0.0);
    data
}

/// Packs the ball list into the two parallel `vec4` uniform arrays the
/// WebGL balls shader consumes: per-ball `(x, y, radius, unused)` and
/// `(r, g, b, a)`. Only the prefix actually drawn is uploaded; elements
/// beyond `balls.len()` keep their previous values but are never
/// referenced by the `ball_count * 6` vertices in the draw call.
///
/// # Arguments
///
/// - `&[Ball]` - The ball list for this frame.
/// - `f64` - The live `window.devicePixelRatio` (`>= 1.0`), forwarded to
///   [`game_2d_ball_gpu_record`] so each ball's on-screen radius scales
///   with the backing store and visually matches the Canvas 2D tab.
///
/// # Returns
///
/// - `(Vec<f32>, Vec<f32>)` - Position-and-radius and color arrays.
fn pack_game_2d_balls_webgl(balls: &[Ball], dpr: f64) -> (Vec<f32>, Vec<f32>) {
    let mut pos_radius: Vec<f32> = Vec::with_capacity(balls.len() * 4);
    let mut colors: Vec<f32> = Vec::with_capacity(balls.len() * 4);
    for ball in balls {
        let (ball_pos_radius, ball_color) = game_2d_ball_gpu_record(ball, dpr);
        pos_radius.extend_from_slice(&ball_pos_radius);
        colors.extend_from_slice(&ball_color);
    }
    (pos_radius, colors)
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
/// - `Signal<Game2DTab>` - The tab signal to update.
/// - `Game2DTab` - The tab variant to set.
/// - `UseGame2DFullscreen` - The fullscreen state to clear on switch.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the active
///   tab and clears any active fullscreen mode.
pub(crate) fn game_2d_on_tab_select(
    tab: Signal<Game2DTab>,
    value: Game2DTab,
    fullscreen: UseGame2DFullscreen,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        fullscreen.get_canvas_2d().set(false);
        fullscreen.get_web_gl().set(false);
        fullscreen.get_web_gpu().set(false);
        tab.set(value);
    }))
}

/// Starts the 2D WebGPU bouncing balls loop driven by `requestAnimationFrame`.
///
/// Mirrors [`start_game_2d_loop`]: the same fixed-timestep physics runs on
/// the shared ball list, but rendering goes through a WGSL pipeline that
/// draws every ball as a shader-generated quad with per-ball position,
/// radius, and color uploaded to a uniform buffer each frame. The canvas
/// is cleared to the element's computed CSS background color so the
/// WebGPU output matches the transparent-cleared Canvas 2D tab exactly.
///
/// # Arguments
///
/// - `UseGame2DWebGpu` - The WebGPU backend state for signal updates.
/// - `UseGame2D` - The shared game state (running/fps signals).
/// - `Rc<RefCell<Vec<Ball>>>` - The shared ball list.
/// - `CanvasCache` - The shared canvas element cache for event handlers.
pub(crate) fn start_game_2d_webgpu_loop(
    state: UseGame2DWebGpu,
    game: UseGame2D,
    balls: Rc<RefCell<Vec<Ball>>>,
    canvas_cache: CanvasCache,
) {
    let init_state: UseGame2DWebGpu = state;
    let loop_state: UseGame2DWebGpu = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let renderer_rc: Rc<RefCell<Option<WebGpuRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
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
                GAME_2D_RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        resize_timer_for_event.set(Some(new_timer));
    });
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let renderer_for_cleanup: Rc<RefCell<Option<WebGpuRenderer>>> = renderer_rc.clone();
    let resize_timer_for_cleanup: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let cancelled_for_cleanup: Rc<Cell<bool>> = cancelled.clone();
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
        draw_game_2d_loading(
            GAME_2D_WEBGPU_LOADING_CANVAS_SELECTOR,
            GAME_2D_WEBGPU_CANVAS_SELECTOR,
        );
    }));
    let loading_callback: Function = loading_closure.as_ref().unchecked_ref::<Function>().clone();
    loading_closure.forget();
    let _ =
        loading_window.set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
    spawn_local(async move {
        let config: RenderConfig = RenderConfig::webgpu(
            GAME_2D_WEBGPU_CANVAS_SELECTOR,
            GAME_2D_CANVAS_WIDTH,
            GAME_2D_CANVAS_HEIGHT,
        );
        let renderer: Result<WebGpuRenderer, WebGpuInitError> =
            Engine::webgpu_renderer(&config).await;
        if cancelled_for_init.get() {
            return;
        }
        let renderer: WebGpuRenderer = match renderer {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!("[euv-engine][game_2d] webgpu init failed: {error}"));
                init_state.get_init_error_code().set(error.code());
                init_state.get_loaded().set(true);
                return;
            }
        };
        let pipeline: JsValue = renderer.create_render_pipeline(GAME_2D_WEBGPU_SHADER);
        let uniform_buffer: JsValue =
            renderer.create_uniform_buffer(&vec![0.0; 4 + GAME_2D_MAX_BALLS * 8]);
        let bind_group: JsValue = renderer.create_uniform_bind_group(&pipeline, &uniform_buffer);
        *canvas_cache.0.borrow_mut() = game_2d_canvas_element(GAME_2D_WEBGPU_CANVAS_SELECTOR);
        let clear_color: Rc<Cell<(f64, f64, f64)>> = Rc::new(Cell::new(
            game_2d_canvas_clear_color(GAME_2D_WEBGPU_CANVAS_SELECTOR),
        ));
        let accumulator: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        init_state.get_active().set(true);
        // Delay flipping `loaded` so the loading overlay stays painted for a
        // minimum visible duration even when init completes instantly.
        set_loaded_delayed(init_state.get_loaded(), GAME_2D_LOADING_MIN_MILLIS);
        *renderer_rc.borrow_mut() = Some(renderer);
        let pipeline_rc: Rc<JsValue> = Rc::new(pipeline);
        let buffer_rc: Rc<JsValue> = Rc::new(uniform_buffer);
        let bind_group_rc: Rc<JsValue> = Rc::new(bind_group);
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
        let prev_positions: Rc<RefCell<Vec<Vector2D>>> = Rc::new(RefCell::new(Vec::new()));
        let prev_for_loop: Rc<RefCell<Vec<Vector2D>>> = prev_positions.clone();
        // Tracks the canvas dimensions the balls were last physics-stepped
        // against, so a fullscreen <-> inline transition can rescale ball
        // positions and radii in lockstep with the new backing buffer.
        let last_canvas_size: Rc<RefCell<(f64, f64)>> = Rc::new(RefCell::new((0.0, 0.0)));
        let last_canvas_size_for_loop: Rc<RefCell<(f64, f64)>> = last_canvas_size.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            // Stop on tab-switch cleanup (`cancelled`) or when the canvas
            // left the document (router navigation fires no cleanup).
            if cancelled_for_loop.get() || game_2d_canvas_detached(GAME_2D_WEBGPU_CANVAS_SELECTOR) {
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
                GAME_2D_FIXED_TIMESTEP
            } else {
                (current_time - prev).min(0.25)
            };
            last_clone.set(current_time);
            // Resize-rescale must run BEFORE the physics tick so that
            // `update_balls` does not first clamp positions against the
            // new (smaller) canvas and then have the rescale try to
            // proportionally shrink those clamped values. With the
            // previous order, exiting fullscreen with many balls
            // pushed every ball to `y = radius` (wall clamp), then the
            // rescale halved those clamped y values — visually the
            // balls appeared to reset to the floor and lost all
            // motion, which is the regression this reordering fixes.
            // The helper also runs a CSS-mismatch safety net in case
            // the debounced `resize` listener was missed (e.g. when
            // the synthetic event fired while the signal-driven DOM
            // re-render was still pending).
            let resize_dirty: bool = handle_rescale_dirty(
                &resize_dirty_for_loop,
                &last_canvas_size_for_loop,
                &balls,
                &prev_for_loop,
                &canvas_cache,
                GAME_2D_WEBGPU_CANVAS_SELECTOR,
            );
            if game.get_running().get() {
                // Accumulate only while running: a paused accumulator would grow
                // unboundedly and burst catch-up physics steps on resume.
                acc_clone.set(acc_clone.get() + frame_time);
                while acc_clone.get() >= GAME_2D_FIXED_TIMESTEP {
                    snapshot_ball_positions(&mut prev_for_loop.borrow_mut(), &balls.borrow());
                    let (cw, ch): (f64, f64) = canvas_cache
                        .0
                        .borrow()
                        .as_ref()
                        .map(|canvas| (canvas.client_width() as f64, canvas.client_height() as f64))
                        .unwrap_or((0.0, 0.0));
                    update_balls(&mut balls.borrow_mut(), GAME_2D_FIXED_TIMESTEP, cw, ch);
                    acc_clone.set(acc_clone.get() - GAME_2D_FIXED_TIMESTEP);
                }
            }
            let alpha: f64 = (acc_clone.get() / GAME_2D_FIXED_TIMESTEP).clamp(0.0, 1.0);
            // The renderer's own backing-store resize is folded into
            // the render block below so we hold
            // `renderer_for_loop.borrow_mut()` exactly once per frame
            // — otherwise we previously panicked with `RefCell
            // already borrowed` when both blocks tried to borrow the
            // same cell. The `resize_dirty` boolean is already bound
            // above by the `handle_rescale_dirty` call.
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
            // Read the canvas's CSS pixel dimensions (clientWidth /
            // clientHeight) on the resize tick so the GPU backing store
            // grows with the canvas when the user enters or exits
            // fullscreen. The WebGPU / WebGL canvases have their own
            // resize path here (see `renderer.resize` below) that is
            // driven by the same `resize_dirty` debounce, so we just
            // swap the constant dimensions for runtime ones.
            let (canvas_width, canvas_height): (f64, f64) = canvas_cache
                .0
                .borrow()
                .as_ref()
                .map(|canvas| (canvas.client_width() as f64, canvas.client_height() as f64))
                .unwrap_or((0.0, 0.0));
            let new_physical_width: u32 = (canvas_width * dpr).round() as u32;
            let new_physical_height: u32 = (canvas_height * dpr).round() as u32;
            // Borrow the renderer exactly once for the entire frame. We
            // use `borrow_mut().as_mut()` (NOT `borrow_mut().take()`) so
            // we do not have to write back - the RefMut guard releases
            // automatically when this block exits, avoiding a second
            // `borrow_mut()` call that previously panicked with
            // `RefCell already borrowed`.
            if let Some(renderer) = renderer_for_loop.borrow_mut().as_mut() {
                // Per-frame CSS-vs-backing safety net for the same
                // reason documented in `start_game_3d_webgl_loop` /
                // `start_game_3d_webgpu_loop`: the synthetic `resize`
                // event debounce fires while the canvas DOM still has
                // the previous CSS box, leaving a multi-frame window
                // where the browser paints the OLD-size backing image
                // stretched into the NEW CSS box. Apply `canvas.width`
                // BEFORE `renderer.resize(...)` so the backing store
                // matches the CSS box on the very next paint cycle.
                // The earlier worry about "syncing every frame reads its
                // own writes and grows exponentially" is now obsolete
                // because `read_canvas_size` returns the CSS layout box
                // (not `canvas.width`), so the comparison is stable.
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
                let render_balls: Vec<Ball> =
                    interpolate_balls(&balls.borrow(), &prev_for_loop.borrow(), alpha);
                let uniform_data: Vec<f32> =
                    pack_game_2d_balls_webgpu(&render_balls, canvas_width, canvas_height, dpr);
                let vertex_count: u32 = (render_balls.len() * 6) as u32;
                renderer.update_uniform_buffer(&buffer_for_loop, &uniform_data);
                // Refresh the clear color every frame so a theme toggle
                // takes effect within one paint. The computed style is
                // cached by the engine after the first read, so the only
                // per-frame cost is a small string parse and equality
                // check; the GPU clear value is only re-uploaded when the
                // tuple actually changes.
                let next_clear: (f64, f64, f64) =
                    game_2d_canvas_clear_color(GAME_2D_WEBGPU_CANVAS_SELECTOR);
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

/// Creates the 2D game fullscreen reactive state signals.
///
/// Allocates hook slots in this fixed order:
/// 1. canvas_2d
/// 2. web_gl
/// 3. web_gpu
///
/// # Returns
///
/// - `UseGame2DFullscreen` - A `UseGame2DFullscreen` value.
pub(crate) fn use_game_2d_fullscreen_state() -> UseGame2DFullscreen {
    UseGame2DFullscreen {
        canvas_2d: App::use_signal(|| false),
        web_gl: App::use_signal(|| false),
        web_gpu: App::use_signal(|| false),
    }
}

/// Enters landscape fullscreen mode for the 2D game on the active tab.
///
/// Sets the tab-specific fullscreen signal, pushes a browser history
/// entry so the system back button exits fullscreen instead of
/// navigating away, then flushes the cached safe-area insets to the
/// newly-mounted overlay container. Crucially, the canvas element is
/// *not* recreated — the active tab's `<canvas>` is re-keyed to live
/// inside `c_game_container_fullscreen` instead of its inline slot, so
/// the running game loop, ball list, FPS counter, and pause state all
/// survive the transition.
///
/// # Arguments
///
/// - `UseGame2DFullscreen` - The 2D game fullscreen state.
/// - `Signal<bool>` - The fullscreen signal for the active tab.
pub(crate) fn enter_game_2d_fullscreen(state: UseGame2DFullscreen, tab: Signal<bool>) {
    tab.set(true);
    let _ = state;
    Router::overlay_push_state();
    UseEuvLayout::apply_cached_insets();
    // Dispatch a `resize` event on the window so the existing
    // `App::use_window_event("resize", ...)` handler fires and the
    // game loop's `resize_dirty` flag is set. That causes the loop
    // to re-acquire the SSAA canvas with the new (fullscreen)
    // dimensions read from `canvas.clientWidth` / `clientHeight`,
    // so the backing buffer resizes from the inline size to the
    // fullscreen size and ball / cube physics bounds follow.
    let Some(window_value): Option<Window> = window() else {
        return;
    };
    let event: Result<Event, JsValue> = Event::new("resize");
    if let Ok(event) = event {
        let _ = window_value.dispatch_event(&event);
    }
}

/// Exits landscape fullscreen mode for the 2D game on the active tab.
///
/// Used by the in-overlay Exit button. Clears the active tab's fullscreen
/// signal and re-applies the safe-area insets to whatever overlay
/// containers are now mounted.
///
/// # Arguments
///
/// - `Signal<bool>` - The fullscreen signal for the active tab.
pub(crate) fn exit_game_2d_fullscreen(tab: Signal<bool>) {
    tab.set(false);
    UseEuvLayout::apply_cached_insets();
    // See `enter_game_2d_fullscreen` - dispatch a synthetic `resize`
    // event so the game loop's resize-debounce handler picks up the
    // canvas's now-smaller CSS box and re-acquires the SSAA canvas
    // with the inline dimensions.
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
pub(crate) fn exit_game_2d_fullscreen_from_popstate(tab: Signal<bool>) {
    tab.set(false);
    UseEuvLayout::apply_cached_insets();
    // See `enter_game_2d_fullscreen` for why we dispatch a synthetic
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
/// button while the 2D game is in landscape fullscreen mode.
///
/// Watches all three tab-specific fullscreen signals. When any one is
/// `true`, the corresponding `exit_game_2d_fullscreen_from_popstate`
/// runs and the guard returns `true` to consume the `popstate` event.
/// Otherwise returns `false` so the overlay stack or router can handle
/// the back navigation normally.
///
/// Returns the guard ID so the page can unregister it on unmount.
///
/// # Arguments
///
/// - `UseGame2DFullscreen` - The 2D game fullscreen state.
///
/// # Returns
///
/// - `usize` - The popstate guard ID.
pub(crate) fn use_game_2d_fullscreen_popstate(state: UseGame2DFullscreen) -> usize {
    Router::register_popstate_guard(Rc::new(move || {
        if state.get_canvas_2d().get() {
            exit_game_2d_fullscreen_from_popstate(state.get_canvas_2d());
            true
        } else if state.get_web_gl().get() {
            exit_game_2d_fullscreen_from_popstate(state.get_web_gl());
            true
        } else if state.get_web_gpu().get() {
            exit_game_2d_fullscreen_from_popstate(state.get_web_gpu());
            true
        } else {
            false
        }
    }))
}

/// Starts the 2D WebGL bouncing balls loop driven by `requestAnimationFrame`.
///
/// Mirrors [`start_game_2d_loop`]: the same fixed-timestep physics runs on
/// the shared ball list, but rendering goes through a GLSL ES 3.00 program
/// that draws every ball as a shader-generated quad with per-ball position,
/// radius, and color uploaded to `vec4` uniform arrays each frame. The
/// canvas is cleared to the element's computed CSS background color so the
/// WebGL output matches the transparent-cleared Canvas 2D tab exactly.
/// WebGL initialization is synchronous; the `spawn_local` wrapper only
/// defers execution past the current render pass so the canvas element
/// exists in the DOM.
///
/// # Arguments
///
/// - `UseGame2DWebGl` - The WebGL backend state for signal updates.
/// - `UseGame2D` - The shared game state (running/fps signals).
/// - `Rc<RefCell<Vec<Ball>>>` - The shared ball list.
/// - `CanvasCache` - The shared canvas element cache for event handlers.
pub(crate) fn start_game_2d_webgl_loop(
    state: UseGame2DWebGl,
    game: UseGame2D,
    balls: Rc<RefCell<Vec<Ball>>>,
    canvas_cache: CanvasCache,
) {
    let init_state: UseGame2DWebGl = state;
    let loop_state: UseGame2DWebGl = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let renderer_rc: Rc<RefCell<Option<WebGlRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
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
                GAME_2D_RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        resize_timer_for_event.set(Some(new_timer));
    });
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let renderer_for_cleanup: Rc<RefCell<Option<WebGlRenderer>>> = renderer_rc.clone();
    let resize_timer_for_cleanup: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let cancelled_for_cleanup: Rc<Cell<bool>> = cancelled.clone();
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
        draw_game_2d_loading(
            GAME_2D_WEBGL_LOADING_CANVAS_SELECTOR,
            GAME_2D_WEBGL_CANVAS_SELECTOR,
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
            GAME_2D_WEBGL_CANVAS_SELECTOR,
            GAME_2D_CANVAS_WIDTH,
            GAME_2D_CANVAS_HEIGHT,
        );
        let renderer: WebGlRenderer = match Engine::webgl_renderer(&config) {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!("[euv-engine][game_2d] webgl init failed: {error}"));
                init_state.get_init_error_code().set(error.code());
                init_state.get_loaded().set(true);
                return;
            }
        };
        let program: WebGlProgram = match renderer
            .create_program(GAME_2D_WEBGL_VERTEX_SHADER, GAME_2D_WEBGL_FRAGMENT_SHADER)
        {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!(
                    "[euv-engine][game_2d] webgl program failed: {error}"
                ));
                init_state.get_init_error_code().set("WEBGL_PROGRAM_ERROR");
                init_state.get_loaded().set(true);
                return;
            }
        };
        *canvas_cache.0.borrow_mut() = game_2d_canvas_element(GAME_2D_WEBGL_CANVAS_SELECTOR);
        let clear_color: Rc<Cell<(f64, f64, f64)>> = Rc::new(Cell::new(
            game_2d_canvas_clear_color(GAME_2D_WEBGL_CANVAS_SELECTOR),
        ));
        let accumulator: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        init_state.get_active().set(true);
        // Delay flipping `loaded` so the loading overlay stays painted for a
        // minimum visible duration even when init completes instantly.
        set_loaded_delayed(init_state.get_loaded(), GAME_2D_LOADING_MIN_MILLIS);
        *renderer_rc.borrow_mut() = Some(renderer);
        let program_rc: Rc<WebGlProgram> = Rc::new(program);
        let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
        let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        let renderer_for_loop: Rc<RefCell<Option<WebGlRenderer>>> = renderer_rc.clone();
        let program_for_loop: Rc<WebGlProgram> = program_rc.clone();
        let clear_color_for_loop: Rc<Cell<(f64, f64, f64)>> = clear_color.clone();
        let acc_clone: Rc<Cell<f64>> = accumulator.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let resize_dirty_for_loop: Rc<Cell<bool>> = resize_dirty.clone();
        let cancelled_for_loop: Rc<Cell<bool>> = cancelled.clone();
        let prev_positions: Rc<RefCell<Vec<Vector2D>>> = Rc::new(RefCell::new(Vec::new()));
        let prev_for_loop: Rc<RefCell<Vec<Vector2D>>> = prev_positions.clone();
        // Tracks the canvas dimensions the balls were last physics-stepped
        // against, so a fullscreen <-> inline transition can rescale ball
        // positions and radii in lockstep with the new backing buffer.
        let last_canvas_size: Rc<RefCell<(f64, f64)>> = Rc::new(RefCell::new((0.0, 0.0)));
        let last_canvas_size_for_loop: Rc<RefCell<(f64, f64)>> = last_canvas_size.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            // Stop on tab-switch cleanup (`cancelled`) or when the canvas
            // left the document (router navigation fires no cleanup).
            if cancelled_for_loop.get() || game_2d_canvas_detached(GAME_2D_WEBGL_CANVAS_SELECTOR) {
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
                GAME_2D_FIXED_TIMESTEP
            } else {
                (current_time - prev).min(0.25)
            };
            last_clone.set(current_time);
            // Resize-rescale must run BEFORE the physics tick so that
            // `update_balls` does not first clamp positions against the
            // new (smaller) canvas and then have the rescale try to
            // proportionally shrink those clamped values. See the
            // WebGPU loop for the full rationale; the same physics
            // and the same regression live here. The helper also runs
            // a CSS-mismatch safety net in case the debounced
            // `resize` listener was missed.
            let resize_dirty: bool = handle_rescale_dirty(
                &resize_dirty_for_loop,
                &last_canvas_size_for_loop,
                &balls,
                &prev_for_loop,
                &canvas_cache,
                GAME_2D_WEBGL_CANVAS_SELECTOR,
            );
            if game.get_running().get() {
                // Accumulate only while running: a paused accumulator would grow
                // unboundedly and burst catch-up physics steps on resume.
                acc_clone.set(acc_clone.get() + frame_time);
                while acc_clone.get() >= GAME_2D_FIXED_TIMESTEP {
                    snapshot_ball_positions(&mut prev_for_loop.borrow_mut(), &balls.borrow());
                    let (cw, ch): (f64, f64) = canvas_cache
                        .0
                        .borrow()
                        .as_ref()
                        .map(|canvas| (canvas.client_width() as f64, canvas.client_height() as f64))
                        .unwrap_or((0.0, 0.0));
                    update_balls(&mut balls.borrow_mut(), GAME_2D_FIXED_TIMESTEP, cw, ch);
                    acc_clone.set(acc_clone.get() - GAME_2D_FIXED_TIMESTEP);
                }
            }
            let alpha: f64 = (acc_clone.get() / GAME_2D_FIXED_TIMESTEP).clamp(0.0, 1.0);
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
            // Read the canvas's CSS pixel dimensions (clientWidth /
            // clientHeight) on the resize tick so the GPU backing store
            // grows with the canvas when the user enters or exits
            // fullscreen. The WebGPU / WebGL canvases have their own
            // resize path here (see `renderer.resize` below) that is
            // driven by the same `resize_dirty` debounce, so we just
            // swap the constant dimensions for runtime ones.
            let (canvas_width, canvas_height): (f64, f64) = canvas_cache
                .0
                .borrow()
                .as_ref()
                .map(|canvas| (canvas.client_width() as f64, canvas.client_height() as f64))
                .unwrap_or((0.0, 0.0));
            let new_physical_width: u32 = (canvas_width * dpr).round() as u32;
            let new_physical_height: u32 = (canvas_height * dpr).round() as u32;
            if let Some(renderer) = renderer_for_loop.borrow_mut().as_mut() {
                // Per-frame CSS-vs-backing safety net for the same
                // reason documented in `start_game_3d_webgl_loop` /
                // `start_game_3d_webgpu_loop`: the synthetic `resize`
                // event debounce fires while the canvas DOM still has
                // the previous CSS box, leaving a multi-frame window
                // where the browser paints the OLD-size backing image
                // stretched into the NEW CSS box. Apply `canvas.width`
                // BEFORE `renderer.resize(...)` so the backing store
                // matches the CSS box on the very next paint cycle.
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
                let render_balls: Vec<Ball> =
                    interpolate_balls(&balls.borrow(), &prev_for_loop.borrow(), alpha);
                let (pos_radius_data, color_data) = pack_game_2d_balls_webgl(&render_balls, dpr);
                let vertex_count: i32 = (render_balls.len() * 6) as i32;
                renderer.set_uniform_2f(
                    &program_for_loop,
                    "u_canvas_size",
                    canvas_width as f32,
                    canvas_height as f32,
                );
                renderer.set_uniform_4fv(
                    &program_for_loop,
                    "u_ball_pos_radius[0]",
                    &pos_radius_data,
                );
                renderer.set_uniform_4fv(&program_for_loop, "u_ball_color[0]", &color_data);
                // Refresh the clear color every frame so a theme toggle
                // takes effect within one paint. The computed style is
                // cached by the engine after the first read, so the only
                // per-frame cost is a small string parse and equality
                // check; the GPU clear value is only re-uploaded when the
                // tuple actually changes.
                let next_clear: (f64, f64, f64) =
                    game_2d_canvas_clear_color(GAME_2D_WEBGL_CANVAS_SELECTOR);
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
