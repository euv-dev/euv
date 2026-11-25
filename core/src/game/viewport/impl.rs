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
