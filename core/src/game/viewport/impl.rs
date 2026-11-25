use super::*;

/// Viewport-local drawing primitives for `GameRenderContext`.
impl<'a> GameRenderContext<'a> {
    /// Creates a `GameRenderContext` bound to `bounds` and `display_list`.
    ///
    /// # Arguments
    ///
    /// - `Rect` - Viewport bounds in root logical coordinates.
    /// - `&'a mut DisplayList` - Target display list.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed viewport render context.
    pub fn new(bounds: Rect, display_list: &'a mut DisplayList) -> Self {
        Self {
            bounds,
            display_list,
        }
    }

    /// Returns the logical width and height of the game viewport.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Render context reference.
    ///
    /// # Returns
    ///
    /// - `Size` - Viewport size in logical pixels.
    #[must_use]
    pub fn viewport_size(&self) -> Size {
        let Self { bounds, .. } = self;
        Size::new(bounds.get_width(), bounds.get_height())
    }

    /// Fills the entire game viewport with `color`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `Color` - Background clear color.
    pub fn clear(&mut self, color: Color) {
        let Self {
            bounds,
            display_list,
        } = self;
        display_list.push(DrawCommand::FillRoundedRect {
            rect: *bounds,
            color,
            radius: CornerRadius::default(),
            border: Border::default(),
        });
    }

    /// Draws an axis-aligned solid rectangle in viewport-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `Rect` - Local rectangle bounds.
    /// - `Color` - Fill color.
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.fill_rounded_rect(rect, color, 0.0);
    }

    /// Draws a rounded solid rectangle in viewport-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `Rect` - Local rectangle bounds.
    /// - `Color` - Fill color.
    /// - `f32` - Corner radius in logical pixels.
    pub fn fill_rounded_rect(&mut self, rect: Rect, color: Color, radius: f32) {
        let Self {
            bounds,
            display_list,
        } = self;
        let world: Rect = Rect::new(
            bounds.get_x() + rect.get_x(),
            bounds.get_y() + rect.get_y(),
            rect.get_width(),
            rect.get_height(),
        );
        display_list.push(DrawCommand::FillRoundedRect {
            rect: world,
            color,
            radius: CornerRadius::all(radius),
            border: Border::default(),
        });
    }

    /// Draws an outlined rectangle in viewport-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `Rect` - Local rectangle bounds.
    /// - `f32` - Stroke thickness in logical pixels.
    /// - `Color` - Stroke color.
    pub fn stroke_rect(&mut self, rect: Rect, stroke_width: f32, color: Color) {
        let Self {
            bounds,
            display_list,
        } = self;
        let world: Rect = Rect::new(
            bounds.get_x() + rect.get_x(),
            bounds.get_y() + rect.get_y(),
            rect.get_width(),
            rect.get_height(),
        );
        display_list.push(DrawCommand::FillRoundedRect {
            rect: world,
            color: COLOR_TRANSPARENT,
            radius: CornerRadius::default(),
            border: Border::new(stroke_width, color),
        });
    }

    /// Draws a filled circle in viewport-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `Point` - Circle center in viewport-local coordinates.
    /// - `f32` - Circle radius in logical pixels.
    /// - `Color` - Fill color.
    pub fn fill_circle(&mut self, center: Point, radius: f32, color: Color) {
        let Self {
            bounds,
            display_list,
        } = self;
        let world: Point = Point::new(
            bounds.get_x() + center.get_x(),
            bounds.get_y() + center.get_y(),
        );
        display_list.push(DrawCommand::Circle {
            center: world,
            radius,
            color,
        });
    }

    /// Draws an anti-aliased line segment in viewport-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `Point` - Segment start point.
    /// - `Point` - Segment end point.
    /// - `f32` - Line width in logical pixels.
    /// - `Color` - Line color.
    pub fn draw_line(&mut self, from: Point, to: Point, width: f32, color: Color) {
        let Self {
            bounds,
            display_list,
        } = self;
        display_list.push(DrawCommand::Line {
            from: Point::new(bounds.get_x() + from.get_x(), bounds.get_y() + from.get_y()),
            to: Point::new(bounds.get_x() + to.get_x(), bounds.get_y() + to.get_y()),
            color,
            width,
        });
    }

    /// Draws HUD or debug text inside the game viewport.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `Point` - Top-left local position.
    /// - `String` - Text string.
    /// - `f32` - Font size in logical pixels.
    /// - `Color` - Text color.
    pub fn draw_text(&mut self, position: Point, text: String, font_size: f32, color: Color) {
        let Self {
            bounds,
            display_list,
        } = self;
        let mut style: TextStyle = TextStyle::default();
        style.set_font_size(font_size);
        style.set_color(color);
        display_list.push(DrawCommand::TextRun {
            rect: Rect::new(
                bounds.get_x() + position.get_x(),
                bounds.get_y() + position.get_y(),
                (bounds.get_width() - position.get_x()).max(0.0),
                font_size * 1.6,
            ),
            text,
            style,
        });
    }

    /// Blits an external 2D/3D engine RGBA8 `ImageBuffer` directly into the viewport.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `Rect` - Local destination rectangle.
    /// - `ImageBuffer` - External engine rendered frame or sprite sheet.
    pub fn blit_framebuffer(&mut self, rect: Rect, buffer: ImageBuffer) {
        let Self {
            bounds,
            display_list,
        } = self;
        let world: Rect = Rect::new(
            bounds.get_x() + rect.get_x(),
            bounds.get_y() + rect.get_y(),
            rect.get_width(),
            rect.get_height(),
        );
        display_list.push(DrawCommand::Image {
            rect: world,
            buffer,
            radius: CornerRadius::default(),
        });
    }

    /// Draws a filled triangle in viewport-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `Point` - First vertex.
    /// - `Point` - Second vertex.
    /// - `Point` - Third vertex.
    /// - `Color` - Fill color.
    pub fn fill_triangle(&mut self, p0: Point, p1: Point, p2: Point, color: Color) {
        let Self {
            bounds,
            display_list,
        } = self;
        let ox: f32 = bounds.get_x();
        let oy: f32 = bounds.get_y();
        display_list.push(DrawCommand::Triangle {
            p0: Point::new(ox + p0.get_x(), oy + p0.get_y()),
            p1: Point::new(ox + p1.get_x(), oy + p1.get_y()),
            p2: Point::new(ox + p2.get_x(), oy + p2.get_y()),
            color,
        });
    }

    /// Draws a cropped frame from a `SpriteSheet` into `rect` with multiplicative `tint`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `&SpriteSheet` - Source sprite sheet.
    /// - `u32` - Zero-based animation frame index.
    /// - `Rect` - Destination viewport-local rectangle.
    /// - `Color` - Multiplicative tint color.
    pub fn draw_sprite_frame(
        &mut self,
        sheet: &SpriteSheet,
        frame_index: u32,
        rect: Rect,
        tint: Color,
    ) {
        let src_rect: Rect = sheet.frame_rect(frame_index);
        let Self {
            bounds,
            display_list,
        } = self;
        display_list.push(DrawCommand::SpriteRegion {
            rect: Rect::new(
                bounds.get_x() + rect.get_x(),
                bounds.get_y() + rect.get_y(),
                rect.get_width(),
                rect.get_height(),
            ),
            buffer: sheet.get_texture().clone(),
            src_rect,
            tint,
        });
    }

    /// Renders all visible tiles of `tilemap` through `camera` using O(visible) frustum culling.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `&TileMap` - Grid tilemap to draw.
    /// - `&Camera2D` - Active 2D world camera.
    pub fn render_tilemap(&mut self, tilemap: &TileMap, camera: &Camera2D) {
        let vp: Size = self.viewport_size();
        let vis: Rect = camera.visible_world_rect(vp);
        let ts: f32 = tilemap.get_tile_size();
        let zoom: f32 = camera.get_zoom().max(0.25);
        let min_c: usize = (vis.get_x() / ts).floor().max(0.0) as usize;
        let max_c: usize = ((vis.right() / ts).ceil().max(0.0) as usize).min(tilemap.get_columns());
        let min_r: usize = (vis.get_y() / ts).floor().max(0.0) as usize;
        let max_r: usize = ((vis.bottom() / ts).ceil().max(0.0) as usize).min(tilemap.get_rows());
        let tile_px: f32 = (ts * zoom).max(1.0);
        for row in min_r..max_r {
            for col in min_c..max_c {
                let tid: u16 = tilemap.tile_at(col, row);
                if tid == 0 {
                    continue;
                }
                let Some(entry) = tilemap.get_palette().get(tid as usize) else {
                    continue;
                };
                let world_pt: Point = Point::new((col as f32) * ts, (row as f32) * ts);
                let screen_pt: Point = camera.world_to_screen(world_pt, vp);
                let tile_rect: Rect = Rect::new(
                    screen_pt.get_x(),
                    screen_pt.get_y(),
                    tile_px + 0.5,
                    tile_px + 0.5,
                );
                self.fill_rect(tile_rect, entry.get_color());
                if entry.get_border_color().get_a() > 0 {
                    self.stroke_rect(tile_rect, 1.0, entry.get_border_color());
                }
            }
        }
    }

    /// Renders all visible sprite entities in `world` through `camera`, sorted by `z_order` with frustum culling.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `&GameWorld` - Active ECS world.
    /// - `&Camera2D` - Active 2D world camera.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of visible entities drawn after frustum culling.
    pub fn render_world(&mut self, world: &GameWorld, camera: &Camera2D) -> usize {
        let vp: Size = self.viewport_size();
        let vis: Rect = camera.visible_world_rect(vp);
        let zoom: f32 = camera.get_zoom().max(0.25);
        let mut draw_order: Vec<(i32, usize)> = Vec::with_capacity(world.get_live_count());
        for (idx, ent) in world.get_entities().iter().enumerate() {
            if !ent.get_alive() {
                continue;
            }
            let Some(sp) = ent.sprite_component() else {
                continue;
            };
            if !sp.get_visible() {
                continue;
            }
            if ent.world_bounds().intersect(vis).is_none() {
                continue;
            }
            draw_order.push((sp.get_z_order(), idx));
        }
        draw_order.sort_by(|a: &(i32, usize), b: &(i32, usize)| {
            a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))
        });
        let rendered: usize = draw_order.len();
        for (_, idx) in draw_order.into_iter() {
            let ent: &EntityRecord = &world.get_entities()[idx];
            let Some(sp) = ent.sprite_component() else {
                continue;
            };
            let tr: EntityTransform = ent.get_transform();
            let center: Point = camera.world_to_screen(tr.get_position(), vp);
            let w: f32 = sp.get_size().get_width() * tr.get_scale().get_width() * zoom;
            let h: f32 = sp.get_size().get_height() * tr.get_scale().get_height() * zoom;
            match sp.get_shape() {
                EntityVisualShape::Rect => {
                    let rect: Rect =
                        Rect::new(center.get_x() - w * 0.5, center.get_y() - h * 0.5, w, h);
                    self.fill_rounded_rect(rect, sp.get_color(), sp.get_corner_radius() * zoom);
                    if sp.get_accent().get_a() > 0 {
                        self.stroke_rect(rect, 1.5, sp.get_accent());
                    }
                }
                EntityVisualShape::Circle => {
                    let r: f32 = (w.min(h) * 0.5).max(1.0);
                    self.fill_circle(center, r, sp.get_color());
                    if sp.get_accent().get_a() > 0 && r > 3.0 {
                        self.fill_circle(center, r * 0.45, sp.get_accent());
                    }
                }
                EntityVisualShape::Triangle => {
                    let ang: f32 = tr.get_rotation_rad();
                    let cos_a: f32 = ang.cos();
                    let sin_a: f32 = ang.sin();
                    let rx: f32 = w * 0.55;
                    let ry: f32 = h * 0.48;
                    let nose: Point =
                        Point::new(center.get_x() + cos_a * rx, center.get_y() + sin_a * rx);
                    let left: Point = Point::new(
                        center.get_x() - cos_a * (rx * 0.72) - sin_a * ry,
                        center.get_y() - sin_a * (rx * 0.72) + cos_a * ry,
                    );
                    let right: Point = Point::new(
                        center.get_x() - cos_a * (rx * 0.72) + sin_a * ry,
                        center.get_y() - sin_a * (rx * 0.72) - cos_a * ry,
                    );
                    self.fill_triangle(nose, left, right, sp.get_color());
                    if sp.get_accent().get_a() > 0 {
                        self.fill_circle(center, (w.min(h) * 0.16).max(2.0), sp.get_accent());
                    }
                }
                EntityVisualShape::Diamond => {
                    let hw: f32 = w * 0.5;
                    let hh: f32 = h * 0.5;
                    let top: Point = Point::new(center.get_x(), center.get_y() - hh);
                    let right: Point = Point::new(center.get_x() + hw, center.get_y());
                    let bottom: Point = Point::new(center.get_x(), center.get_y() + hh);
                    let left: Point = Point::new(center.get_x() - hw, center.get_y());
                    self.fill_triangle(top, right, bottom, sp.get_color());
                    self.fill_triangle(top, bottom, left, sp.get_color());
                    if sp.get_accent().get_a() > 0 {
                        self.fill_circle(center, (w.min(h) * 0.18).max(2.0), sp.get_accent());
                    }
                }
            }
        }
        rendered
    }

    /// Renders all active pooled particles in `particles` through `camera` with frustum culling and lifetime interpolation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `&ParticleSystem` - Active particle system.
    /// - `&Camera2D` - Active 2D world camera.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of visible particles rendered.
    pub fn render_particles(&mut self, particles: &ParticleSystem, camera: &Camera2D) -> usize {
        let vp: Size = self.viewport_size();
        let vis: Rect = camera.visible_world_rect(vp);
        let zoom: f32 = camera.get_zoom().max(0.25);
        let mut count: usize = 0;
        for p in particles.get_pool().iter() {
            if !p.get_active() {
                continue;
            }
            let pos: Point = p.get_position();
            if !vis.contains(pos) {
                continue;
            }
            let t: f32 = (p.get_age() / p.get_lifetime().max(0.001)).clamp(0.0, 1.0);
            let r: f32 = ((p.get_start_size() + (p.get_end_size() - p.get_start_size()) * t)
                * zoom)
                .max(0.8);
            let col: Color = p.get_start_color().lerp(p.get_end_color(), t);
            if col.get_a() == 0 {
                continue;
            }
            let screen: Point = camera.world_to_screen(pos, vp);
            self.fill_circle(screen, r, col);
            count += 1;
        }
        count
    }

    /// Draws wireframe collider bounds for debugging physics shapes inside `world`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable render context.
    /// - `&GameWorld` - Active ECS world.
    /// - `&Camera2D` - Active 2D world camera.
    pub fn draw_debug_colliders(&mut self, world: &GameWorld, camera: &Camera2D) {
        let vp: Size = self.viewport_size();
        let vis: Rect = camera.visible_world_rect(vp);
        let zoom: f32 = camera.get_zoom().max(0.25);
        for ent in world.get_entities().iter() {
            if !ent.get_alive() {
                continue;
            }
            let Some(col) = ent.collider_component() else {
                continue;
            };
            let aabb: Rect = col.world_aabb(ent.get_transform().get_position());
            if aabb.intersect(vis).is_none() {
                continue;
            }
            let tl: Point = camera.world_to_screen(Point::new(aabb.get_x(), aabb.get_y()), vp);
            let color: Color = if col.get_is_trigger() {
                Color::rgba(250, 204, 21, 180)
            } else {
                Color::rgba(34, 197, 94, 180)
            };
            self.stroke_rect(
                Rect::new(
                    tl.get_x(),
                    tl.get_y(),
                    aabb.get_width() * zoom,
                    aabb.get_height() * zoom,
                ),
                1.0,
                color,
            );
        }
    }
}

/// Host construction, fixed-step execution, and scene mounting for `GameEngineHost`.
impl GameEngineHost {
    /// Creates a new `GameEngineHost` wrapping `scene` with a `fixed_dt` simulation cadence.
    ///
    /// # Arguments
    ///
    /// - `S` - User game scene implementing `GameScene`.
    /// - `f32` - Fixed physics timestep in seconds (e.g. `1.0 / 60.0`).
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed engine host.
    #[must_use]
    pub fn new<S>(scene: S, fixed_dt: f32) -> Self
    where
        S: GameScene + 'static,
    {
        Self {
            clock: Rc::new(RefCell::new(FrameClock::new(fixed_dt, 8))),
            input: Rc::new(RefCell::new(InputState::new())),
            scene: Rc::new(RefCell::new(scene)),
        }
    }

    /// Feeds a `WindowEvent` into the hosted game `InputState`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Engine host reference.
    /// - `&WindowEvent` - Event received from the OS window.
    pub fn ingest_event(&self, event: &WindowEvent) {
        if let Ok(mut input) = self.get_input().try_borrow_mut() {
            input.ingest(event);
        }
    }

    /// Advances the hosted game scene by `dt_secs` using fixed-timestep accumulation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Engine host reference.
    /// - `f32` - Elapsed wall seconds since the previous tick.
    ///
    /// # Returns
    ///
    /// - `u32` - Number of fixed physics steps executed.
    pub fn step(&self, dt_secs: f32) -> u32 {
        let (steps, fixed_dt): (u32, f32) = match self.get_clock().try_borrow_mut() {
            Ok(mut clock) => (clock.tick(dt_secs), clock.get_fixed_dt()),
            Err(_) => (0, 1.0 / 60.0),
        };
        let input_snapshot: InputState = self
            .get_input()
            .try_borrow()
            .map(|guard: Ref<'_, InputState>| guard.clone())
            .unwrap_or_default();
        let Self { scene, .. } = self;
        if let Ok(mut game) = scene.try_borrow_mut() {
            for _ in 0..steps {
                game.fixed_update(fixed_dt, &input_snapshot);
            }
            game.update(dt_secs, &input_snapshot);
        }
        if let Ok(mut input) = self.get_input().try_borrow_mut() {
            input.begin_frame();
        }
        steps
    }

    /// Mounts a live `Canvas` viewport into the active ambient `BuildContext` backed by this `GameScene`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Engine host reference.
    /// - `Style` - Layout and visual style of the game viewport.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted game viewport scene node.
    #[must_use]
    pub fn mount_viewport(&self, style: Style) -> NodeId {
        let Self { clock, scene, .. } = self;
        let clock_c: Rc<RefCell<FrameClock>> = clock.clone();
        let scene_c: Rc<RefCell<dyn GameScene>> = scene.clone();
        let mut ctx: BuildContext = current_build_context();
        ctx.canvas(
            style,
            move |bounds: Rect, display_list: &mut DisplayList| {
                let alpha: f32 = clock_c
                    .try_borrow()
                    .map(|c: Ref<'_, FrameClock>| c.get_alpha())
                    .unwrap_or(1.0);
                if let Ok(mut game) = scene_c.try_borrow_mut() {
                    let mut render_ctx: GameRenderContext<'_> =
                        GameRenderContext::new(bounds, display_list);
                    game.render(alpha, &mut render_ctx);
                }
            },
        )
    }
}
