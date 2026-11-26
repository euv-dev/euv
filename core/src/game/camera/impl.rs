use super::*;

/// Default camera centered at `(0, 0)` with `1.0x` zoom.
impl Default for Camera2D {
    /// Creates a default `Camera2D`.
    ///
    /// # Returns
    ///
    /// - `Self` - Default 2D camera.
    fn default() -> Self {
        Self {
            position: Point::new(0.0, 0.0),
            zoom: 1.0,
            trauma: 0.0,
            max_shake: 14.0,
            shake_time: 0.0,
            world_bounds: None,
        }
    }
}

/// Camera projection, smooth follow, frustum culling, and trauma shake operations.
impl Camera2D {
    /// Creates a `Camera2D` positioned at `center` with `zoom`.
    ///
    /// # Arguments
    ///
    /// - `Point` - Initial world center.
    /// - `f32` - Initial zoom multiplier clamped to `[0.25, 4.0]`.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured 2D camera.
    #[must_use]
    pub fn at(center: Point, zoom: f32) -> Self {
        Self {
            position: center,
            zoom: zoom.clamp(0.25, 4.0),
            ..Self::default()
        }
    }

    /// Adds impact trauma in `[0.0, 1.0]` to drive damped screen shake.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable camera.
    /// - `f32` - Trauma impulse amount.
    pub fn add_trauma(&mut self, amount: f32) {
        let next: f32 = (self.get_trauma() + amount.max(0.0)).clamp(0.0, 1.0);
        self.set_trauma(next);
    }

    /// Smoothly follows `target` using frame-rate-independent exponential damping and decays screen shake.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable camera.
    /// - `Point` - Desired world target point.
    /// - `f32` - Responsiveness factor (`6.0..=16.0` recommended).
    /// - `f32` - Elapsed frame delta in seconds.
    /// - `Size` - Viewport logical size for optional world-bounds clamping.
    pub fn follow(&mut self, target: Point, responsiveness: f32, dt_secs: f32, viewport: Size) {
        let alpha: f32 =
            (1.0 - (-responsiveness.max(0.1) * dt_secs.max(0.0)).exp()).clamp(0.0, 1.0);
        let cur: Point = self.get_position();
        let mut nx: f32 = cur.get_x() + (target.get_x() - cur.get_x()) * alpha;
        let mut ny: f32 = cur.get_y() + (target.get_y() - cur.get_y()) * alpha;
        let Self { world_bounds, .. } = *self;
        if let Some(bounds) = world_bounds {
            let half_w: f32 = (viewport.get_width() * 0.5) / self.get_zoom().max(0.25);
            let half_h: f32 = (viewport.get_height() * 0.5) / self.get_zoom().max(0.25);
            let min_x: f32 = bounds.get_x() + half_w;
            let max_x: f32 = bounds.right() - half_w;
            let min_y: f32 = bounds.get_y() + half_h;
            let max_y: f32 = bounds.bottom() - half_h;
            nx = if min_x <= max_x {
                nx.clamp(min_x, max_x)
            } else {
                bounds.get_x() + bounds.get_width() * 0.5
            };
            ny = if min_y <= max_y {
                ny.clamp(min_y, max_y)
            } else {
                bounds.get_y() + bounds.get_height() * 0.5
            };
        }
        self.set_position(Point::new(nx, ny));
        let next_trauma: f32 = (self.get_trauma() - dt_secs * 1.8).max(0.0);
        self.set_trauma(next_trauma);
        self.set_shake_time(self.get_shake_time() + dt_secs * 38.0);
    }

    /// Computes the current screen-shake offset in world coordinates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Camera reference.
    ///
    /// # Returns
    ///
    /// - `Point` - Shake offset `(dx, dy)`.
    #[must_use]
    pub fn shake_offset(&self) -> Point {
        let t: f32 = self.get_trauma();
        if t <= 0.001 {
            return Point::new(0.0, 0.0);
        }
        let mag: f32 = t * t * self.get_max_shake();
        let phase: f32 = self.get_shake_time();
        Point::new((phase * 1.7).sin() * mag, (phase * 2.3).cos() * mag)
    }

    /// Projects a world-space point into viewport-local screen coordinates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Camera reference.
    /// - `Point` - World-space point.
    /// - `Size` - Viewport logical size.
    ///
    /// # Returns
    ///
    /// - `Point` - Viewport-local logical pixel coordinates.
    #[must_use]
    pub fn world_to_screen(&self, world: Point, viewport: Size) -> Point {
        let zoom: f32 = self.get_zoom().max(0.25);
        let shake: Point = self.shake_offset();
        let cam: Point = self.get_position();
        Point::new(
            (world.get_x() - (cam.get_x() + shake.get_x())) * zoom + viewport.get_width() * 0.5,
            (world.get_y() - (cam.get_y() + shake.get_y())) * zoom + viewport.get_height() * 0.5,
        )
    }

    /// Unprojects a viewport-local screen point back into world coordinates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Camera reference.
    /// - `Point` - Viewport-local logical pixel point.
    /// - `Size` - Viewport logical size.
    ///
    /// # Returns
    ///
    /// - `Point` - Corresponding world-space point.
    #[must_use]
    pub fn screen_to_world(&self, screen: Point, viewport: Size) -> Point {
        let zoom: f32 = self.get_zoom().max(0.25);
        let cam: Point = self.get_position();
        Point::new(
            (screen.get_x() - viewport.get_width() * 0.5) / zoom + cam.get_x(),
            (screen.get_y() - viewport.get_height() * 0.5) / zoom + cam.get_y(),
        )
    }

    /// Computes the world-space visible frustum rectangle for culling offscreen sprites and tiles.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Camera reference.
    /// - `Size` - Viewport logical size.
    ///
    /// # Returns
    ///
    /// - `Rect` - Visible world-space frustum bounds.
    #[must_use]
    pub fn visible_world_rect(&self, viewport: Size) -> Rect {
        let zoom: f32 = self.get_zoom().max(0.25);
        let w: f32 = viewport.get_width() / zoom;
        let h: f32 = viewport.get_height() / zoom;
        let cam: Point = self.get_position();
        let pad: f32 = self.get_max_shake();
        Rect::new(
            cam.get_x() - w * 0.5 - pad,
            cam.get_y() - h * 0.5 - pad,
            w + pad * 2.0,
            h + pad * 2.0,
        )
    }
}
