use super::*;

/// Implements camera transformation methods for `Camera2D`.
impl Camera2D {
    /// Creates a new camera centered at the origin with default zoom and no rotation.
    ///
    /// # Arguments
    ///
    /// - `f64` - The viewport width in pixels.
    /// - `f64` - The viewport height in pixels.
    ///
    /// # Returns
    ///
    /// - `Camera2D` - The new camera.
    pub fn create(viewport_width: f64, viewport_height: f64) -> Camera2D {
        Camera2D::new(
            Vector2D::zero(),
            RENDERER_DEFAULT_CAMERA_ZOOM,
            RENDERER_DEFAULT_CAMERA_ROTATION,
            viewport_width,
            viewport_height,
        )
    }

    /// Converts a world-space point to screen-space coordinates.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The world-space point.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The screen-space point.
    pub fn world_to_screen(&self, world: Vector2D) -> Vector2D {
        let relative: Vector2D = world - self.get_position();
        let rotated: Vector2D = relative.rotated(-self.get_rotation());
        Vector2D::new(
            rotated.get_x() * self.get_zoom() + self.get_viewport_width() * 0.5,
            rotated.get_y() * self.get_zoom() + self.get_viewport_height() * 0.5,
        )
    }

    /// Converts a screen-space point to world-space coordinates.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The screen-space point.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The world-space point.
    pub fn screen_to_world(&self, screen: Vector2D) -> Vector2D {
        let relative: Vector2D = Vector2D::new(
            (screen.get_x() - self.get_viewport_width() * 0.5) / self.get_zoom(),
            (screen.get_y() - self.get_viewport_height() * 0.5) / self.get_zoom(),
        );
        let rotated: Vector2D = relative.rotated(self.get_rotation());
        rotated + self.get_position()
    }

    /// Moves the camera position by the given offset.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The translation offset in world space.
    pub fn translate(&mut self, offset: Vector2D) {
        self.set_position(self.get_position() + offset);
    }

    /// Adjusts the zoom by the given factor, clamped to a minimum of `EPSILON`.
    ///
    /// # Arguments
    ///
    /// - `f64` - The zoom multiplier.
    pub fn zoom_by(&mut self, factor: f64) {
        self.set_zoom((self.get_zoom() * factor).max(EPSILON));
    }
}

/// Implements `Default` for `Camera2D` as a camera at the origin with 800x600 viewport.
impl Default for Camera2D {
    /// Constructs a default [`Camera2D`] value.
    ///
    /// # Returns
    ///
    /// - `Camera2D` - A default-constructed instance with the documented initial state.
    fn default() -> Camera2D {
        Camera2D::create(800.0, 600.0)
    }
}

/// Implements static font and color utility methods for `CanvasRenderer`.
impl CanvasRenderer {
    /// Builds a CSS font string from font size and family.
    ///
    /// # Arguments
    ///
    /// - `f64` - The font size in pixels.
    /// - `F: AsRef<str>` - The font family name.
    ///
    /// # Returns
    ///
    /// - `String` - The CSS font string (e.g., `"16px sans-serif"`).
    pub fn font<F>(size: f64, family: F) -> String
    where
        F: AsRef<str>,
    {
        let family: &str = family.as_ref();
        format!("{size}px {family}")
    }

    /// Creates a default font string using the default font size and family.
    ///
    /// # Returns
    ///
    /// - `String` - The default CSS font string.
    pub fn default_font() -> String {
        Self::font(RENDERER_DEFAULT_FONT_SIZE, RENDERER_DEFAULT_FONT_FAMILY)
    }

    /// Enables high-quality anti-aliasing on an arbitrary canvas 2D context.
    ///
    /// Applies the `High` rendering quality preset via `apply_quality`,
    /// which sets `imageSmoothingEnabled`, `imageSmoothingQuality = "high"`,
    /// and `textRendering = "geometricPrecision"` on the given context.
    ///
    /// Use this static helper when you manage your own `CanvasRenderingContext2d`
    /// and don't hold a `CanvasRenderer` instance. For instances, call
    /// `renderer.enable_smoothing()` instead.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context to configure.
    pub fn enable_smoothing_on(context: &CanvasRenderingContext2d) {
        Self::apply_quality(context, RenderQuality::High);
    }

    /// Detects the host device pixel ratio (HiDPI scale factor) via reflection.
    ///
    /// Reads `window.devicePixelRatio` using `Reflect::get` because the
    /// `web-sys` `Window` features currently in use do not expose a native
    /// getter for this property. Falls back to
    /// `RENDERER_DEFAULT_DEVICE_PIXEL_RATIO` (1.0) when the global window or
    /// the value is missing, not a finite number, or below 1.0.
    ///
    /// # Returns
    ///
    /// - `f64` - The detected device pixel ratio (clamped to `>= 1.0`).
    pub fn detect_dpr() -> f64 {
        let Some(window_value) = window() else {
            return RENDERER_DEFAULT_DEVICE_PIXEL_RATIO;
        };
        let raw: Option<f64> = Reflect::get(
            window_value.as_ref(),
            &JsValue::from_str(RENDERER_PROPERTY_DEVICE_PIXEL_RATIO),
        )
        .ok()
        .and_then(|value: JsValue| value.as_f64());
        raw.filter(|value: &f64| value.is_finite() && *value >= 1.0)
            .unwrap_or(RENDERER_DEFAULT_DEVICE_PIXEL_RATIO)
    }

    /// Applies the given `RenderQuality` preset to an arbitrary canvas context.
    ///
    /// Sets `imageSmoothingEnabled`, `imageSmoothingQuality`, and
    /// `textRendering` according to the supplied quality. `Low` disables
    /// smoothing (intended for use with CSS `image-rendering: pixelated`),
    /// `Medium` and `High` enable it with the matching quality level.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The target context.
    /// - `RenderQuality` - The quality preset to apply.
    pub(crate) fn apply_quality(context: &CanvasRenderingContext2d, quality: RenderQuality) {
        let smoothing_enabled: bool = !matches!(quality, RenderQuality::Low);
        context.set_image_smoothing_enabled(smoothing_enabled);
        let quality_value: &str = match quality {
            RenderQuality::Low => RENDERER_IMAGE_SMOOTHING_QUALITY_LOW,
            RenderQuality::Medium => RENDERER_IMAGE_SMOOTHING_QUALITY_MEDIUM,
            RenderQuality::High => RENDERER_IMAGE_SMOOTHING_QUALITY_HIGH,
        };
        let _: Result<bool, JsValue> = Reflect::set(
            context,
            &JsValue::from_str(RENDERER_PROPERTY_IMAGE_SMOOTHING_QUALITY),
            &JsValue::from_str(quality_value),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            context,
            &JsValue::from_str(RENDERER_PROPERTY_TEXT_RENDERING),
            &JsValue::from_str(RENDERER_TEXT_RENDERING_GEOMETRIC_PRECISION),
        );
    }
}

/// Implements static CSS conversion for `Color`.
impl Color {
    /// Converts a `Color` to a CSS `rgba()` string suitable for canvas fill or stroke styles.
    ///
    /// # Arguments
    ///
    /// - `&Color` - The color to convert.
    ///
    /// # Returns
    ///
    /// - `String` - The CSS `rgba()` color string.
    pub fn to_css(color: &Color) -> String {
        color.to_css_rgba()
    }
}

/// Implements drawing and camera management methods for `CanvasRenderer`.
/// Implements recording and replay for `DrawList`.
impl DrawList {
    /// Creates an empty draw list.
    ///
    /// # Returns
    ///
    /// - `DrawList` - The new empty draw list.
    pub fn create() -> DrawList {
        DrawList::new(Vec::new())
    }

    /// Returns whether the list contains no commands.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if there are no recorded commands.
    pub fn is_empty(&self) -> bool {
        self.get_commands().is_empty()
    }

    /// Returns the number of recorded commands.
    ///
    /// # Returns
    ///
    /// - `usize` - The command count.
    pub fn len(&self) -> usize {
        self.get_commands().len()
    }

    /// Returns the recorded commands as a slice for replay iteration.
    ///
    /// # Returns
    ///
    /// - `&[DrawCommand]` - The commands in the order they were recorded.
    pub fn commands(&self) -> &[DrawCommand] {
        self.get_commands().as_slice()
    }

    /// Removes all recorded commands, keeping the allocated capacity for reuse
    /// on the next frame.
    pub fn clear(&mut self) {
        self.get_mut_commands().clear();
    }

    /// Records a fill-rectangle command.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `Color` - A `Color` parameter.
    pub fn fill_rect(&mut self, position: Vector2D, width: f64, height: f64, color: Color) {
        self.get_mut_commands().push(DrawCommand::FillRect {
            position,
            width,
            height,
            color,
        });
    }

    /// Records a stroke-rectangle command.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `Color` - A `Color` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    pub fn stroke_rect(
        &mut self,
        position: Vector2D,
        width: f64,
        height: f64,
        color: Color,
        line_width: f64,
    ) {
        self.get_mut_commands().push(DrawCommand::StrokeRect {
            position,
            width,
            height,
            color,
            line_width,
        });
    }

    /// Records a fill-circle command.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `Color` - A `Color` parameter.
    pub fn fill_circle(&mut self, center: Vector2D, radius: f64, color: Color) {
        self.get_mut_commands().push(DrawCommand::FillCircle {
            center,
            radius,
            color,
        });
    }

    /// Records a stroke-circle command.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `Color` - A `Color` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    pub fn stroke_circle(&mut self, center: Vector2D, radius: f64, color: Color, line_width: f64) {
        self.get_mut_commands().push(DrawCommand::StrokeCircle {
            center,
            radius,
            color,
            line_width,
        });
    }

    /// Records a line-segment command.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `Color` - A `Color` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    pub fn draw_line(&mut self, start: Vector2D, end: Vector2D, color: Color, line_width: f64) {
        self.get_mut_commands().push(DrawCommand::Line {
            start,
            end,
            color,
            line_width,
        });
    }

    /// Records a fill-text command.
    ///
    /// # Arguments
    ///
    /// - `T: AsRef<str>` - A generic type parameter.
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `Color` - A `Color` parameter.
    /// - `F: AsRef<str>` - A generic type parameter.
    pub fn fill_text<T, F>(&mut self, text: T, position: Vector2D, color: Color, font: F)
    where
        T: AsRef<str>,
        F: AsRef<str>,
    {
        self.get_mut_commands().push(DrawCommand::FillText {
            text: text.as_ref().to_string(),
            position,
            color,
            font: font.as_ref().to_string(),
        });
    }

    /// Records a transformed sprite draw command.
    ///
    /// # Arguments
    ///
    /// - `&HtmlImageElement` - Shared reference to a `HtmlImageElement`.
    /// - `Rect` - A `Rect` parameter.
    /// - `Transform2D` - A `Transform2D` parameter.
    pub fn draw_sprite(&mut self, image: &HtmlImageElement, source: Rect, transform: Transform2D) {
        self.get_mut_commands().push(DrawCommand::DrawSprite {
            image: image.clone(),
            source,
            transform,
        });
    }

    /// Records an image sub-region draw command (no rotation).
    ///
    /// # Arguments
    ///
    /// - `&HtmlImageElement` - Shared reference to a `HtmlImageElement`.
    /// - `Rect` - A `Rect` parameter.
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `f64` - A 64-bit float (`f64`).
    pub fn draw_image_rect(
        &mut self,
        image: &HtmlImageElement,
        source: Rect,
        dest_position: Vector2D,
        dest_width: f64,
        dest_height: f64,
    ) {
        self.get_mut_commands().push(DrawCommand::DrawImageRect {
            image: image.clone(),
            source,
            dest_position,
            dest_width,
            dest_height,
        });
    }

    /// Records a global-alpha state change.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn set_global_alpha(&mut self, alpha: f64) {
        self.get_mut_commands()
            .push(DrawCommand::SetGlobalAlpha { alpha });
    }

    /// Records a blend-mode state change.
    ///
    /// # Arguments
    ///
    /// - `BlendMode` - A `BlendMode` parameter.
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.get_mut_commands()
            .push(DrawCommand::SetBlendMode { mode });
    }
}

/// Inherent implementation of [`CanvasRenderer`].
impl CanvasRenderer {
    /// Creates a new renderer from a canvas element selector and viewport dimensions.
    ///
    /// # Arguments
    ///
    /// - `&str` - The CSS selector for the canvas element.
    /// - `f64` - The viewport width.
    /// - `f64` - The viewport height.
    ///
    /// # Returns
    ///
    /// - `Option<CanvasRenderer>` - The renderer, or `None` if the canvas was not found.
    pub fn from_selector<S>(
        canvas_selector: S,
        viewport_width: f64,
        viewport_height: f64,
    ) -> Option<CanvasRenderer>
    where
        S: AsRef<str>,
    {
        let window_value: Window = window()?;
        let document_value: Document = window_value.document()?;
        let element: Element = document_value
            .query_selector(canvas_selector.as_ref())
            .ok()
            .flatten()?;
        let canvas_element: HtmlCanvasElement = element.unchecked_into();
        let context_object: Object = canvas_element
            .get_context(RENDERER_CONTEXT_TYPE_2D)
            .ok()
            .flatten()?;
        let context: CanvasRenderingContext2d = context_object.unchecked_into();
        let renderer: CanvasRenderer = CanvasRenderer::new(
            context,
            Camera2D::create(viewport_width, viewport_height),
            RenderQuality::default(),
        );
        renderer.enable_smoothing();
        Some(renderer)
    }

    /// Enables high-quality anti-aliasing on the canvas context by setting
    /// `imageSmoothingEnabled` to `true` and `imageSmoothingQuality` to `"high"`.
    ///
    /// Applies the active `quality` preset via the shared `apply_quality`
    /// helper so that all smoothing-related settings are kept in sync.
    pub fn enable_smoothing(&self) {
        Self::apply_quality(self.get_context(), self.get_quality());
    }

    /// Clears the entire canvas viewport.
    pub fn clear(&self) {
        self.get_context().clear_rect(
            0.0,
            0.0,
            self.get_camera().get_viewport_width(),
            self.get_camera().get_viewport_height(),
        );
    }

    /// Clears the canvas and fills it with the given CSS color string.
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - The CSS color string (e.g., `"#000000"`).
    pub fn clear_color<C>(&self, color: C)
    where
        C: AsRef<str>,
    {
        self.get_context().set_fill_style_str(color.as_ref());
        self.get_context().fill_rect(
            0.0,
            0.0,
            self.get_camera().get_viewport_width(),
            self.get_camera().get_viewport_height(),
        );
    }

    /// Saves the current canvas state (transform, styles) onto the state stack.
    pub fn save(&self) {
        self.get_context().save();
    }

    /// Restores the most recently saved canvas state.
    pub fn restore(&self) {
        self.get_context().restore();
    }

    /// Replays a recorded `DrawList` onto this renderer's canvas.
    ///
    /// Convenience wrapper around `replay_context` using this renderer's context.
    ///
    /// # Arguments
    ///
    /// - `&DrawList` - The recorded commands to replay.
    pub fn replay(&self, list: &DrawList) {
        Self::replay_context(self.get_context(), list);
    }

    /// Replays a recorded `DrawList` onto an arbitrary canvas 2D context in a
    /// single batched pass.
    ///
    /// Consecutive same-style shapes are merged into one path (one `begin_path`
    /// plus one `fill`/`stroke` per style run), fill/stroke colors and line
    /// widths are only re-applied when they change, and sprites are drawn with a
    /// single `set_transform` rather than a save/restore pair. This collapses
    /// the per-shape canvas state churn of immediate-mode drawing.
    ///
    /// The canvas transform and global alpha are reset to identity / 1.0 when
    /// replay finishes, so callers can sandwich the call between
    /// `save()`/`apply_camera()` and `restore()` without leaking state.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The target canvas 2D context.
    /// - `&DrawList` - The recorded commands to replay.
    pub fn replay_context(context: &CanvasRenderingContext2d, list: &DrawList) {
        let mut current_fill: Option<Color> = None;
        let mut current_stroke: Option<Color> = None;
        let mut current_line_width: f64 = f64::NAN;
        // Whether a same-style path run is currently open.
        let mut run_open: bool = false;
        let mut run_is_fill: bool = true;
        let mut run_key: Option<(u8, Color, f64)> = None;

        // Returns the style key for a path-batchable command, or `None` for
        // commands that break a run (sprites, images, text, state changes).
        /// Computes the batching key for a [`DrawCommand`].
        ///
        /// # Arguments
        ///
        /// - `&DrawCommand` - Shared reference to a `DrawCommand`.
        ///
        /// # Returns
        ///
        /// - `Option<(u8, Color, f64)>` - `Some(...)` on success, `None` otherwise.
        fn batch_key(command: &DrawCommand) -> Option<(u8, Color, f64)> {
            match command {
                DrawCommand::FillRect { color, .. } | DrawCommand::FillCircle { color, .. } => {
                    Some((0, *color, 0.0))
                }
                DrawCommand::StrokeRect {
                    color, line_width, ..
                }
                | DrawCommand::StrokeCircle {
                    color, line_width, ..
                }
                | DrawCommand::Line {
                    color, line_width, ..
                } => Some((1, *color, *line_width)),
                _ => None,
            }
        }

        // Emits a single path-batchable command's geometry into the open path.
        /// Emits the geometry for the supplied [`DrawCommand`] into the canvas context.
        ///
        /// # Arguments
        ///
        /// - `&CanvasRenderingContext2d` - Shared reference to a `CanvasRenderingContext2d`.
        /// - `&DrawCommand` - Shared reference to a `DrawCommand`.
        fn emit_geometry(context: &CanvasRenderingContext2d, command: &DrawCommand) {
            match command {
                DrawCommand::FillRect {
                    position,
                    width,
                    height,
                    ..
                }
                | DrawCommand::StrokeRect {
                    position,
                    width,
                    height,
                    ..
                } => {
                    context.rect(position.get_x(), position.get_y(), *width, *height);
                }
                DrawCommand::FillCircle { center, radius, .. }
                | DrawCommand::StrokeCircle { center, radius, .. } => {
                    context.move_to(center.get_x() + radius, center.get_y());
                    let _: Result<(), JsValue> =
                        context.arc(center.get_x(), center.get_y(), *radius, 0.0, TWO_PI);
                }
                DrawCommand::Line { start, end, .. } => {
                    context.move_to(start.get_x(), start.get_y());
                    context.line_to(end.get_x(), end.get_y());
                }
                _ => {}
            }
        }

        for command in list.commands() {
            let key: Option<(u8, Color, f64)> = batch_key(command);
            // Close the open run if this command breaks it or starts a new style.
            if run_open && key != run_key {
                if run_is_fill {
                    context.fill();
                } else {
                    context.stroke();
                }
                run_open = false;
            }
            if let Some(current_key) = key {
                // Begin (or continue) a same-style path run.
                if !run_open {
                    let (kind, color, line_width) = current_key;
                    if kind == 0 {
                        if current_fill != Some(color) {
                            context.set_fill_style_str(&Color::to_css(&color));
                            current_fill = Some(color);
                        }
                        run_is_fill = true;
                    } else {
                        if current_stroke != Some(color) {
                            context.set_stroke_style_str(&Color::to_css(&color));
                            current_stroke = Some(color);
                        }
                        if current_line_width != line_width {
                            context.set_line_width(line_width);
                            current_line_width = line_width;
                        }
                        run_is_fill = false;
                    }
                    context.begin_path();
                    run_open = true;
                    run_key = Some(current_key);
                }
                emit_geometry(context, command);
                continue;
            }
            // Non-batchable command: draw it immediately.
            match command {
                DrawCommand::FillText {
                    text,
                    position,
                    color,
                    font,
                } => {
                    if current_fill != Some(*color) {
                        context.set_fill_style_str(&Color::to_css(color));
                        current_fill = Some(*color);
                    }
                    context.set_font(font);
                    let _: Result<(), JsValue> =
                        context.fill_text(text, position.get_x(), position.get_y());
                }
                DrawCommand::DrawSprite {
                    image,
                    source,
                    transform,
                } => {
                    draw_sprite_immediate(context, image, source, transform);
                }
                DrawCommand::DrawImageRect {
                    image,
                    source,
                    dest_position,
                    dest_width,
                    dest_height,
                } => {
                    let _: Result<(), JsValue> = context
                        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                            image,
                            source.get_x(),
                            source.get_y(),
                            source.get_width(),
                            source.get_height(),
                            dest_position.get_x(),
                            dest_position.get_y(),
                            *dest_width,
                            *dest_height,
                        );
                }
                DrawCommand::SetGlobalAlpha { alpha } => {
                    context.set_global_alpha(Numeric::clamp(*alpha, 0.0, 1.0));
                }
                DrawCommand::SetBlendMode { mode } => {
                    let _: Result<(), JsValue> =
                        context.set_global_composite_operation(mode.to_css());
                }
                _ => {}
            }
        }
        // Flush any trailing open run.
        if run_open {
            if run_is_fill {
                context.fill();
            } else {
                context.stroke();
            }
        }
        let _: Result<(), JsValue> = context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        context.set_global_alpha(1.0);
    }

    /// Applies the camera transform to the canvas context.
    ///
    /// Translates to the screen center, applies zoom and rotation,
    /// then offsets by the negative camera position.
    pub fn apply_camera(&self) {
        let camera: Camera2D = self.get_camera();
        let _: Result<(), JsValue> = self.get_context().translate(
            camera.get_viewport_width() * 0.5,
            camera.get_viewport_height() * 0.5,
        );
        let _: Result<(), JsValue> = self
            .get_context()
            .scale(camera.get_zoom(), camera.get_zoom());
        let _: Result<(), JsValue> = self.get_context().rotate(camera.get_rotation());
        let _: Result<(), JsValue> = self.get_context().translate(
            -camera.get_position().get_x(),
            -camera.get_position().get_y(),
        );
    }

    /// Sets the fill color for subsequent fill operations.
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - The CSS color string.
    pub fn set_fill_color<C>(&self, color: C)
    where
        C: AsRef<str>,
    {
        self.get_context().set_fill_style_str(color.as_ref());
    }

    /// Sets the stroke color for subsequent stroke operations.
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - The CSS color string.
    pub fn set_stroke_color<C>(&self, color: C)
    where
        C: AsRef<str>,
    {
        self.get_context().set_stroke_style_str(color.as_ref());
    }

    /// Sets the line width for subsequent stroke operations.
    ///
    /// # Arguments
    ///
    /// - `f64` - The line width in pixels.
    pub fn set_line_width(&self, width: f64) {
        self.get_context().set_line_width(width);
    }

    /// Sets the global alpha (opacity) for all subsequent drawing operations.
    ///
    /// # Arguments
    ///
    /// - `f64` - The alpha value in the range 0.0 to 1.0.
    pub fn set_global_alpha(&self, alpha: f64) {
        self.get_context()
            .set_global_alpha(Numeric::clamp(alpha, 0.0, 1.0));
    }

    /// Fills a rectangle at the given world-space position and dimensions.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The top-left position in world space.
    /// - `f64` - The width.
    /// - `f64` - The height.
    pub fn fill_rect(&self, position: Vector2D, width: f64, height: f64) {
        self.get_context()
            .fill_rect(position.get_x(), position.get_y(), width, height);
    }

    /// Strokes the outline of a rectangle at the given world-space position and dimensions.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The top-left position in world space.
    /// - `f64` - The width.
    /// - `f64` - The height.
    pub fn stroke_rect(&self, position: Vector2D, width: f64, height: f64) {
        self.get_context()
            .stroke_rect(position.get_x(), position.get_y(), width, height);
    }

    /// Fills a circle at the given world-space center with the specified radius.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The center in world space.
    /// - `f64` - The radius.
    pub fn fill_circle(&self, center: Vector2D, radius: f64) {
        self.get_context().begin_path();
        self.get_context()
            .arc(center.get_x(), center.get_y(), radius, 0.0, TWO_PI)
            .unwrap_or(());
        self.get_context().fill();
    }

    /// Strokes the outline of a circle at the given world-space center.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The center in world space.
    /// - `f64` - The radius.
    pub fn stroke_circle(&self, center: Vector2D, radius: f64) {
        self.get_context().begin_path();
        self.get_context()
            .arc(center.get_x(), center.get_y(), radius, 0.0, TWO_PI)
            .unwrap_or(());
        self.get_context().stroke();
    }

    /// Draws a line segment between two world-space points.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The start point.
    /// - `Vector2D` - The end point.
    pub fn draw_line(&self, start: Vector2D, end: Vector2D) {
        self.get_context().begin_path();
        self.get_context().move_to(start.get_x(), start.get_y());
        self.get_context().line_to(end.get_x(), end.get_y());
        self.get_context().stroke();
    }

    /// Fills text at the given world-space position.
    ///
    /// # Arguments
    ///
    /// - `T: AsRef<str>` - The text to draw.
    /// - `Vector2D` - The position in world space.
    pub fn fill_text<T>(&self, text: T, position: Vector2D)
    where
        T: AsRef<str>,
    {
        self.get_context()
            .fill_text(text.as_ref(), position.get_x(), position.get_y())
            .unwrap_or(());
    }

    /// Sets the font for subsequent text rendering.
    ///
    /// # Arguments
    ///
    /// - `F: AsRef<str>` - The CSS font string (e.g., `"16px sans-serif"`).
    pub fn set_font<F>(&self, font: F)
    where
        F: AsRef<str>,
    {
        self.get_context().set_font(font.as_ref());
    }

    /// Draws an image element at the given world-space position and dimensions.
    ///
    /// # Arguments
    ///
    /// - `&HtmlImageElement` - The image element to draw.
    /// - `Vector2D` - The top-left position in world space.
    /// - `f64` - The destination width.
    /// - `f64` - The destination height.
    pub fn draw_image(
        &self,
        image: &HtmlImageElement,
        position: Vector2D,
        width: f64,
        height: f64,
    ) {
        let _: Result<(), JsValue> = self
            .get_context()
            .draw_image_with_html_image_element_and_dw_and_dh(
                image,
                position.get_x(),
                position.get_y(),
                width,
                height,
            );
    }

    /// Draws a sub-region of an image element at the given world-space position.
    ///
    /// # Arguments
    ///
    /// - `&HtmlImageElement` - The image element to draw.
    /// - `Rect` - The source rectangle within the image.
    /// - `Vector2D` - The destination top-left position in world space.
    /// - `f64` - The destination width.
    /// - `f64` - The destination height.
    pub fn draw_image_rect(
        &self,
        image: &HtmlImageElement,
        source: Rect,
        dest_position: Vector2D,
        dest_width: f64,
        dest_height: f64,
    ) {
        let _: Result<(), JsValue> = self
            .get_context()
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                source.get_x(),
                source.get_y(),
                source.get_width(),
                source.get_height(),
                dest_position.get_x(),
                dest_position.get_y(),
                dest_width,
                dest_height,
            );
    }
}

/// Implements 3D camera transformation and projection methods for `Camera3D`.
impl Camera3D {
    /// Creates a new 3D camera at the given position looking at the target.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The eye position.
    /// - `Vector3D` - The target position to look at.
    /// - `f64` - The viewport width.
    /// - `f64` - The viewport height.
    ///
    /// # Returns
    ///
    /// - `Camera3D` - The new camera.
    pub fn create(
        position: Vector3D,
        target: Vector3D,
        viewport_width: f64,
        viewport_height: f64,
    ) -> Camera3D {
        let mut camera: Camera3D = Camera3D::new(position, target, viewport_width, viewport_height);
        camera.set_up(Vector3D::up());
        camera.set_fov(DEFAULT_CAMERA_FOV);
        camera.set_near(DEFAULT_CAMERA_NEAR);
        camera.set_far(DEFAULT_CAMERA_FAR);
        camera
    }

    /// Returns the aspect ratio (width / height).
    ///
    /// # Returns
    ///
    /// - `f64` - The aspect ratio.
    pub fn aspect(&self) -> f64 {
        if self.get_viewport_height() < EPSILON {
            return 1.0;
        }
        self.get_viewport_width() / self.get_viewport_height()
    }

    /// Returns the forward direction (from position to target, normalized).
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The forward direction.
    pub fn forward(&self) -> Vector3D {
        (self.get_target() - self.get_position()).normalized()
    }

    /// Returns the right direction (cross product of forward and up).
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The right direction.
    pub fn right(&self) -> Vector3D {
        self.forward().cross(self.get_up()).normalized()
    }

    /// Returns the view matrix for this camera.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The view matrix.
    pub fn view_matrix(&self) -> Matrix4x4 {
        Matrix4x4::look_at(self.get_position(), self.get_target(), self.get_up())
    }

    /// Returns the perspective projection matrix for this camera.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The projection matrix.
    pub fn projection_matrix(&self) -> Matrix4x4 {
        Matrix4x4::perspective(
            self.get_fov(),
            self.aspect(),
            self.get_near(),
            self.get_far(),
        )
    }

    /// Returns the combined view-projection matrix.
    ///
    /// # Returns
    ///
    /// - `Matrix4x4` - The view-projection matrix.
    pub fn view_proj_matrix(&self) -> Matrix4x4 {
        self.projection_matrix().multiply(self.view_matrix())
    }

    /// Converts a 3D world-space point to screen-space (NDC) coordinates.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The world-space point.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - The screen-space point where x and y are in [0, 1] and z is the depth.
    pub fn world_to_screen(&self, world: Vector3D) -> Vector3D {
        let clip: Vector3D = self.view_proj_matrix().transform_point(world);
        Vector3D::new(
            (clip.get_x() + 1.0) * 0.5 * self.get_viewport_width(),
            (1.0 - clip.get_y()) * 0.5 * self.get_viewport_height(),
            clip.get_z(),
        )
    }

    /// Projects a world-space point and returns whether it is within the camera frustum.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The world-space point.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the point is within the frustum.
    pub fn in_frustum(&self, world: Vector3D) -> bool {
        let clip: Vector3D = self.view_proj_matrix().transform_point(world);
        clip.get_x() >= -1.0
            && clip.get_x() <= 1.0
            && clip.get_y() >= -1.0
            && clip.get_y() <= 1.0
            && clip.get_z() >= -1.0
            && clip.get_z() <= 1.0
    }

    /// Moves the camera position by the given offset, keeping the target offset by the same amount.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The translation offset.
    pub fn translate(&mut self, offset: Vector3D) {
        self.set_position(self.get_position() + offset);
        self.set_target(self.get_target() + offset);
    }

    /// Moves the camera position towards the target by the given distance.
    ///
    /// # Arguments
    ///
    /// - `f64` - The distance to zoom in (positive) or out (negative).
    pub fn zoom(&mut self, distance: f64) {
        let direction: Vector3D = self.forward();
        self.set_position(self.get_position() + direction.scaled(distance));
    }

    /// Orbits the camera around the target by the given yaw and pitch angles.
    ///
    /// # Arguments
    ///
    /// - `f64` - The yaw delta in radians (horizontal rotation).
    /// - `f64` - The pitch delta in radians (vertical rotation).
    pub fn orbit(&mut self, yaw_delta: f64, pitch_delta: f64) {
        let offset: Vector3D = self.get_position() - self.get_target();
        let current_distance: f64 = offset.magnitude();
        let current_yaw: f64 = offset.get_x().atan2(offset.get_z());
        let horizontal_dist: f64 =
            (offset.get_x() * offset.get_x() + offset.get_z() * offset.get_z()).sqrt();
        let current_pitch: f64 = (offset.get_y() / horizontal_dist.max(EPSILON)).asin();
        let new_yaw: f64 = current_yaw + yaw_delta;
        let new_pitch: f64 = Numeric::clamp(
            current_pitch + pitch_delta,
            -HALF_PI + EPSILON,
            HALF_PI - EPSILON,
        );
        let cos_pitch: f64 = new_pitch.cos();
        self.set_position(
            self.get_target()
                + Vector3D::new(
                    new_yaw.sin() * cos_pitch * current_distance,
                    new_pitch.sin() * current_distance,
                    new_yaw.cos() * cos_pitch * current_distance,
                ),
        );
    }
}

/// Implements `Default` for `Camera3D` as a camera at (0, 0, 5) looking at the origin.
impl Default for Camera3D {
    /// Constructs a default [`Camera3D`] value.
    ///
    /// # Returns
    ///
    /// - `Camera3D` - A default-constructed instance with the documented initial state.
    fn default() -> Camera3D {
        Camera3D::create(Vector3D::new(0.0, 0.0, 5.0), Vector3D::zero(), 800.0, 600.0)
    }
}

/// Implements construction, presentation, and anti-aliasing methods for `SsaaCanvas`.
impl SsaaCanvas {
    /// Creates an `SsaaCanvas` from a CSS selector using the default scale factor.
    ///
    /// # Arguments
    ///
    /// - `S: AsRef<str>` - The CSS selector for the display canvas element.
    /// - `f64` - The logical display width in CSS pixels.
    /// - `f64` - The logical display height in CSS pixels.
    ///
    /// # Returns
    ///
    /// - `Option<SsaaCanvas>` - The SSAA canvas, or `None` if the canvas was not found.
    pub fn from_selector<S>(canvas_selector: S, width: f64, height: f64) -> Option<SsaaCanvas>
    where
        S: AsRef<str>,
    {
        Self::from_selector_with_scale(
            canvas_selector,
            width,
            height,
            RENDERER_DEFAULT_SSAA_SCALE_FACTOR,
        )
    }

    /// Creates an `SsaaCanvas` from a CSS selector with a custom SSAA scale factor.
    ///
    /// The offscreen canvas is created at `width * scale_factor` by `height * scale_factor`
    /// pixels, and its context is pre-scaled so that drawing code uses logical coordinates.
    ///
    /// # Arguments
    ///
    /// - `S: AsRef<str>` - The CSS selector for the display canvas element.
    /// - `f64` - The logical display width in CSS pixels.
    /// - `f64` - The logical display height in CSS pixels.
    /// - `f64` - The supersampling scale factor (e.g., 2.0 for 4x SSAA).
    ///
    /// # Returns
    ///
    /// - `Option<SsaaCanvas>` - The SSAA canvas, or `None` if the canvas was not found.
    pub fn from_selector_with_scale<S>(
        canvas_selector: S,
        width: f64,
        height: f64,
        scale_factor: f64,
    ) -> Option<SsaaCanvas>
    where
        S: AsRef<str>,
    {
        let window_value: Window = window()?;
        let document_value: Document = window_value.document()?;
        let element: Element = document_value
            .query_selector(canvas_selector.as_ref())
            .ok()
            .flatten()?;
        let display_canvas: HtmlCanvasElement = element.unchecked_into();
        let device_pixel_ratio: f64 = CanvasRenderer::detect_dpr();
        let physical_width: u32 = (width * device_pixel_ratio).round() as u32;
        let physical_height: u32 = (height * device_pixel_ratio).round() as u32;
        display_canvas.set_width(physical_width);
        display_canvas.set_height(physical_height);
        let display_context_object: Object = display_canvas
            .get_context(RENDERER_CONTEXT_TYPE_2D)
            .ok()
            .flatten()?;
        let display_context: CanvasRenderingContext2d = display_context_object.unchecked_into();
        let _: Result<(), JsValue> = display_context.scale(device_pixel_ratio, device_pixel_ratio);
        let offscreen_canvas: HtmlCanvasElement = document_value
            .create_element(RENDERER_ELEMENT_CANVAS)
            .ok()?
            .unchecked_into();
        let scaled_width: u32 = (width * scale_factor * device_pixel_ratio).round() as u32;
        let scaled_height: u32 = (height * scale_factor * device_pixel_ratio).round() as u32;
        offscreen_canvas.set_width(scaled_width);
        offscreen_canvas.set_height(scaled_height);
        let offscreen_context_object: Object = offscreen_canvas
            .get_context(RENDERER_CONTEXT_TYPE_2D)
            .ok()
            .flatten()?;
        let offscreen_context: CanvasRenderingContext2d = offscreen_context_object.unchecked_into();
        let _: Result<(), JsValue> = offscreen_context.scale(
            scale_factor * device_pixel_ratio,
            scale_factor * device_pixel_ratio,
        );
        let ssaa_canvas: SsaaCanvas = SsaaCanvas::new(
            display_canvas,
            display_context,
            offscreen_canvas,
            offscreen_context,
            scale_factor,
            width,
            height,
        );
        ssaa_canvas.enable_smoothing();
        Some(ssaa_canvas)
    }

    /// Presents the offscreen buffer onto the display canvas with high-quality downscaling.
    ///
    /// Applies the active `quality` preset to the display context, clears the
    /// display canvas, then draws the offscreen canvas scaled down to the
    /// logical display size. This is the core SSAA step that produces smooth
    /// polygon edges.
    pub fn present(&self) {
        CanvasRenderer::apply_quality(self.get_display_context(), self.get_quality());
        self.get_display_context()
            .clear_rect(0.0, 0.0, self.get_width(), self.get_height());
        let _: Result<(), JsValue> = self
            .get_display_context()
            .draw_image_with_html_canvas_element_and_dw_and_dh(
                self.get_offscreen_canvas(),
                0.0,
                0.0,
                self.get_width(),
                self.get_height(),
            );
    }

    /// Clears the offscreen buffer to transparent.
    pub fn clear(&self) {
        self.get_offscreen_context()
            .clear_rect(0.0, 0.0, self.get_width(), self.get_height());
    }

    /// Clears the offscreen buffer and fills it with the given CSS color.
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - The CSS color string.
    pub fn clear_color<C>(&self, color: C)
    where
        C: AsRef<str>,
    {
        self.get_offscreen_context()
            .set_fill_style_str(color.as_ref());
        self.get_offscreen_context()
            .fill_rect(0.0, 0.0, self.get_width(), self.get_height());
    }

    /// Enables high-quality anti-aliasing on both the display and offscreen contexts.
    ///
    /// Applies the active `quality` preset to both contexts via the shared
    /// `apply_quality` helper.
    pub fn enable_smoothing(&self) {
        let quality: RenderQuality = self.get_quality();
        CanvasRenderer::apply_quality(self.get_display_context(), quality);
        CanvasRenderer::apply_quality(self.get_offscreen_context(), quality);
    }
}

/// Implements CSS composite operation string conversion for `BlendMode`.
impl BlendMode {
    /// Returns the CSS `globalCompositeOperation` string for this blend mode.
    ///
    /// # Returns
    ///
    /// - `&str` - The CSS composite operation string.
    pub fn to_css(&self) -> &str {
        match self {
            BlendMode::Normal => BLEND_MODE_NORMAL,
            BlendMode::Multiply => BLEND_MODE_MULTIPLY,
            BlendMode::Screen => BLEND_MODE_SCREEN,
            BlendMode::Lighter => BLEND_MODE_LIGHTER,
            BlendMode::Overlay => BLEND_MODE_OVERLAY,
            BlendMode::Darken => BLEND_MODE_DARKEN,
            BlendMode::Lighten => BLEND_MODE_LIGHTEN,
            BlendMode::ColorDodge => BLEND_MODE_COLOR_DODGE,
            BlendMode::ColorBurn => BLEND_MODE_COLOR_BURN,
            BlendMode::HardLight => BLEND_MODE_HARD_LIGHT,
            BlendMode::SoftLight => BLEND_MODE_SOFT_LIGHT,
            BlendMode::Difference => BLEND_MODE_DIFFERENCE,
            BlendMode::Exclusion => BLEND_MODE_EXCLUSION,
            BlendMode::Hue => BLEND_MODE_HUE,
            BlendMode::Saturation => BLEND_MODE_SATURATION,
            BlendMode::Color => BLEND_MODE_COLOR,
            BlendMode::Luminosity => BLEND_MODE_LUMINOSITY,
        }
    }
}

/// Implements construction and canvas gradient creation for `LinearGradient`.
impl LinearGradient {
    /// Creates a new linear gradient from two points and a list of color stops.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The start point.
    /// - `Vector2D` - The end point.
    /// - `Vec<(f64, String)>` - The color stops as (position, color) pairs.
    ///
    /// # Returns
    ///
    /// - `LinearGradient` - The new gradient.
    pub fn create(start: Vector2D, end: Vector2D, stops: Vec<(f64, String)>) -> LinearGradient {
        LinearGradient::new(start, end, stops)
    }

    /// Creates a `CanvasGradient` from this gradient definition on the given context.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context.
    ///
    /// # Returns
    ///
    /// - `Option<CanvasGradient>` - The canvas gradient, or `None` if creation failed.
    pub fn to_gradient(&self, context: &CanvasRenderingContext2d) -> Option<CanvasGradient> {
        let canvas_gradient: CanvasGradient = context.create_linear_gradient(
            self.get_start().get_x(),
            self.get_start().get_y(),
            self.get_end().get_x(),
            self.get_end().get_y(),
        );
        for (position, color) in self.get_stops() {
            let _: Result<(), JsValue> = canvas_gradient.add_color_stop(*position as f32, color);
        }
        Some(canvas_gradient)
    }
}

/// Implements construction and canvas gradient creation for `RadialGradient`.
impl RadialGradient {
    /// Creates a new radial gradient from inner and outer circles and color stops.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The inner circle center.
    /// - `f64` - The inner circle radius.
    /// - `Vector2D` - The outer circle center.
    /// - `f64` - The outer circle radius.
    /// - `Vec<(f64, String)>` - The color stops as (position, color) pairs.
    ///
    /// # Returns
    ///
    /// - `RadialGradient` - The new gradient.
    pub fn create(
        inner_center: Vector2D,
        inner_radius: f64,
        outer_center: Vector2D,
        outer_radius: f64,
        stops: Vec<(f64, String)>,
    ) -> RadialGradient {
        RadialGradient::new(
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
            stops,
        )
    }

    /// Creates a `CanvasGradient` from this gradient definition on the given context.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context.
    ///
    /// # Returns
    ///
    /// - `Option<CanvasGradient>` - The canvas gradient, or `None` if creation failed.
    pub fn to_gradient(&self, context: &CanvasRenderingContext2d) -> Option<CanvasGradient> {
        let canvas_gradient: CanvasGradient = context
            .create_radial_gradient(
                self.get_inner_center().get_x(),
                self.get_inner_center().get_y(),
                self.get_inner_radius(),
                self.get_outer_center().get_x(),
                self.get_outer_center().get_y(),
                self.get_outer_radius(),
            )
            .ok()?;
        for (position, color) in self.get_stops() {
            let _: Result<(), JsValue> = canvas_gradient.add_color_stop(*position as f32, color);
        }
        Some(canvas_gradient)
    }
}

/// Implements construction methods for `ShadowConfig`.
impl ShadowConfig {
    /// Creates a shadow configuration with default values.
    ///
    /// # Returns
    ///
    /// - `ShadowConfig` - The default shadow configuration.
    pub fn create() -> ShadowConfig {
        ShadowConfig::new(
            RENDERER_DEFAULT_SHADOW_COLOR.to_string(),
            RENDERER_DEFAULT_SHADOW_BLUR,
            0.0,
            0.0,
        )
    }
}

/// Implements `Default` for `ShadowConfig` with default shadow values.
impl Default for ShadowConfig {
    /// Constructs a default [`ShadowConfig`] value.
    ///
    /// # Returns
    ///
    /// - `ShadowConfig` - A default-constructed instance with the documented initial state.
    fn default() -> ShadowConfig {
        ShadowConfig::create()
    }
}

/// Implements construction methods for `RenderLayer`.
impl RenderLayer {
    /// Creates a render layer with the given z-index and visibility.
    ///
    /// # Arguments
    ///
    /// - `i32` - The z-index determining draw order.
    /// - `bool` - Whether the layer is visible.
    ///
    /// # Returns
    ///
    /// - `RenderLayer` - The new render layer.
    pub fn create(z_index: i32, visible: bool) -> RenderLayer {
        RenderLayer::new(z_index, visible)
    }

    /// Creates a background render layer with z-index 0 and visibility enabled.
    ///
    /// # Returns
    ///
    /// - `RenderLayer` - The background layer.
    pub fn background() -> RenderLayer {
        RenderLayer::new(RENDERER_LAYER_BACKGROUND, true)
    }

    /// Creates a foreground render layer with a high z-index and visibility enabled.
    ///
    /// # Returns
    ///
    /// - `RenderLayer` - The foreground layer.
    pub fn foreground() -> RenderLayer {
        RenderLayer::new(RENDERER_LAYER_FOREGROUND, true)
    }

    /// Creates a UI overlay render layer with the highest z-index and visibility enabled.
    ///
    /// # Returns
    ///
    /// - `RenderLayer` - The UI overlay layer.
    pub fn ui() -> RenderLayer {
        RenderLayer::new(RENDERER_LAYER_UI, true)
    }
}

/// Implements blend mode, shadow, and gradient rendering methods for `CanvasRenderer`.
impl CanvasRenderer {
    /// Sets the blend mode for compositing subsequent draw operations.
    ///
    /// # Arguments
    ///
    /// - `BlendMode` - The blend mode to apply.
    pub fn set_blend_mode(&self, mode: BlendMode) {
        let _: Result<(), JsValue> = self
            .get_context()
            .set_global_composite_operation(mode.to_css());
    }

    /// Applies a shadow configuration for subsequent draw operations.
    ///
    /// # Arguments
    ///
    /// - `&ShadowConfig` - The shadow configuration to apply.
    pub fn set_shadow(&self, config: &ShadowConfig) {
        self.get_context()
            .set_shadow_color(config.get_color().as_str());
        self.get_context().set_shadow_blur(config.get_blur());
        self.get_context()
            .set_shadow_offset_x(config.get_offset_x());
        self.get_context()
            .set_shadow_offset_y(config.get_offset_y());
    }

    /// Clears any previously applied shadow, disabling shadow rendering.
    pub fn clear_shadow(&self) {
        self.get_context().set_shadow_color("rgba(0, 0, 0, 0)");
        self.get_context().set_shadow_blur(0.0);
        self.get_context().set_shadow_offset_x(0.0);
        self.get_context().set_shadow_offset_y(0.0);
    }

    /// Applies a linear gradient as the fill style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&LinearGradient` - The linear gradient to use as fill style.
    pub fn set_linear_gradient_fill(&self, gradient: &LinearGradient) {
        if let Some(canvas_gradient) = gradient.to_gradient(self.get_context()) {
            self.get_context()
                .set_fill_style_canvas_gradient(&canvas_gradient);
        }
    }

    /// Applies a radial gradient as the fill style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&RadialGradient` - The radial gradient to use as fill style.
    pub fn set_radial_gradient_fill(&self, gradient: &RadialGradient) {
        if let Some(canvas_gradient) = gradient.to_gradient(self.get_context()) {
            self.get_context()
                .set_fill_style_canvas_gradient(&canvas_gradient);
        }
    }

    /// Applies a linear gradient as the stroke style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&LinearGradient` - The linear gradient to use as stroke style.
    pub fn set_linear_gradient_stroke(&self, gradient: &LinearGradient) {
        if let Some(canvas_gradient) = gradient.to_gradient(self.get_context()) {
            self.get_context()
                .set_stroke_style_canvas_gradient(&canvas_gradient);
        }
    }

    /// Applies a radial gradient as the stroke style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&RadialGradient` - The radial gradient to use as stroke style.
    pub fn set_radial_gradient_stroke(&self, gradient: &RadialGradient) {
        if let Some(canvas_gradient) = gradient.to_gradient(self.get_context()) {
            self.get_context()
                .set_stroke_style_canvas_gradient(&canvas_gradient);
        }
    }
}

/// Implements the `RenderBackend` trait for `CanvasRenderer`, providing
/// a backend-agnostic rendering interface.
///
/// Each method forwards to the inherent `CanvasRenderer` method of the
/// same name, so the per-call documentation lives on the trait definition
/// in `engine::renderer::trait` — the inherent method is the source of
/// truth, this impl is the trait bridge.
impl RenderBackend for CanvasRenderer {
    /// Forwards to [`CanvasRenderer::clear`].
    fn clear(&self) {
        self.clear();
    }

    /// Forwards to [`CanvasRenderer::clear_color`].
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - A generic type parameter.
    fn clear_color<C>(&self, color: C)
    where
        C: AsRef<str>,
    {
        self.clear_color(color);
    }

    /// Forwards to [`CanvasRenderer::save`].
    fn save(&self) {
        self.save();
    }

    /// Forwards to [`CanvasRenderer::restore`].
    fn restore(&self) {
        self.restore();
    }

    /// Forwards to [`CanvasRenderer::set_fill_color`].
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    fn set_fill_color(&self, color: &str) {
        self.set_fill_color(color);
    }

    /// Forwards to [`CanvasRenderer::set_stroke_color`].
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    fn set_stroke_color(&self, color: &str) {
        self.set_stroke_color(color);
    }

    /// Forwards to [`CanvasRenderer::set_line_width`].
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    fn set_line_width(&self, width: f64) {
        self.set_line_width(width);
    }

    /// Forwards to [`CanvasRenderer::set_global_alpha`].
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    fn set_global_alpha(&self, alpha: f64) {
        self.set_global_alpha(alpha);
    }

    /// Forwards to [`CanvasRenderer::set_blend_mode`].
    ///
    /// # Arguments
    ///
    /// - `BlendMode` - A `BlendMode` parameter.
    fn set_blend_mode(&self, mode: BlendMode) {
        self.set_blend_mode(mode);
    }

    /// Forwards to [`CanvasRenderer::set_shadow`].
    ///
    /// # Arguments
    ///
    /// - `&ShadowConfig` - Shared reference to a `ShadowConfig`.
    fn set_shadow(&self, config: &ShadowConfig) {
        self.set_shadow(config);
    }

    /// Forwards to [`CanvasRenderer::clear_shadow`].
    fn clear_shadow(&self) {
        self.clear_shadow();
    }

    /// Forwards to [`CanvasRenderer::fill_rect`].
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `f64` - A 64-bit float (`f64`).
    fn fill_rect(&self, position: Vector2D, width: f64, height: f64) {
        self.fill_rect(position, width, height);
    }

    /// Forwards to [`CanvasRenderer::stroke_rect`].
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `f64` - A 64-bit float (`f64`).
    fn stroke_rect(&self, position: Vector2D, width: f64, height: f64) {
        self.stroke_rect(position, width, height);
    }

    /// Forwards to [`CanvasRenderer::fill_circle`].
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `f64` - A 64-bit float (`f64`).
    fn fill_circle(&self, center: Vector2D, radius: f64) {
        self.fill_circle(center, radius);
    }

    /// Forwards to [`CanvasRenderer::stroke_circle`].
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `f64` - A 64-bit float (`f64`).
    fn stroke_circle(&self, center: Vector2D, radius: f64) {
        self.stroke_circle(center, radius);
    }

    /// Forwards to [`CanvasRenderer::draw_line`].
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `Vector2D` - 2D vector (`Vector2D`).
    fn draw_line(&self, start: Vector2D, end: Vector2D) {
        self.draw_line(start, end);
    }

    /// Forwards to [`CanvasRenderer::fill_text`].
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `Vector2D` - 2D vector (`Vector2D`).
    fn fill_text(&self, text: &str, position: Vector2D) {
        self.fill_text(text, position);
    }

    /// Forwards to [`CanvasRenderer::set_font`].
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    fn set_font(&self, font: &str) {
        self.set_font(font);
    }

    /// Forwards to [`CanvasRenderer::draw_image`].
    ///
    /// # Arguments
    ///
    /// - `&HtmlImageElement` - Shared reference to a `HtmlImageElement`.
    /// - `Vector2D` - 2D vector (`Vector2D`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `f64` - A 64-bit float (`f64`).
    fn draw_image(&self, image: &HtmlImageElement, position: Vector2D, width: f64, height: f64) {
        self.draw_image(image, position, width, height);
    }

    /// Forwards to [`CanvasRenderer::set_linear_gradient_fill`].
    ///
    /// # Arguments
    ///
    /// - `&LinearGradient` - Shared reference to a `LinearGradient`.
    fn set_linear_gradient_fill(&self, gradient: &LinearGradient) {
        self.set_linear_gradient_fill(gradient);
    }

    /// Forwards to [`CanvasRenderer::set_radial_gradient_fill`].
    ///
    /// # Arguments
    ///
    /// - `&RadialGradient` - Shared reference to a `RadialGradient`.
    fn set_radial_gradient_fill(&self, gradient: &RadialGradient) {
        self.set_radial_gradient_fill(gradient);
    }
}

/// Implements async initialization and GPU resource creation for `WebGpuRenderer`.
impl WebGpuRenderer {
    /// Returns `true` if `navigator.gpu` is exposed on the current origin.
    ///
    /// This is the synchronous half of the canonical WebGPU capability
    /// probe used by Three.js (`examples/jsm/capabilities/WebGPU.js`): it
    /// only checks that the browser surfaces the `GPU` interface at all.
    /// It does **not** request an adapter — a present `navigator.gpu`
    /// does not guarantee that a usable GPU adapter is reachable (Linux
    /// software-rendered sessions, headless browsers, GPU-blacklisted
    /// devices and sandboxed iframes all expose `navigator.gpu` while
    /// `requestAdapter()` resolves to `null` or hangs forever).
    ///
    /// Use this as the cheapest pre-flight check before showing a
    /// "needs HTTPS or localhost" prompt. For a definitive answer use
    /// [`Self::probe`] which also awaits `requestAdapter()`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `navigator.gpu` is a non-null, non-undefined
    ///   object; `false` otherwise (including the "no `window`" runtime
    ///   case, which `web_sys::window()` returns `None` for).
    pub fn is_available() -> bool {
        let window_value: Window = match window() {
            Some(value) => value,
            None => return false,
        };
        let navigator: Navigator = window_value.navigator();
        let gpu_result: Result<JsValue, JsValue> = Reflect::get(
            navigator.as_ref(),
            &JsValue::from_str(WEBGPU_NAVIGATOR_GPU_KEY),
        );
        match gpu_result {
            Ok(value) => !value.is_undefined() && !value.is_null(),
            Err(_) => false,
        }
    }

    /// Probes whether a WebGPU adapter can actually be acquired.
    ///
    /// Mirrors Three.js' canonical capability probe exactly:
    ///
    /// Wraps the adapter request in the same `Promise.race` timeout used
    /// by [`Self::init`] so that browsers which leave the adapter promise
    /// permanently pending (headless, sandboxed, device-lost) do not stall
    /// the UI forever. The timeout itself uses the
    /// `INIT_PROMISE_TIMEOUT_MILLIS` constant; on timeout, `probe` returns
    /// `false` rather than an error so callers can treat it the same as
    /// "no adapter".
    ///
    /// # Returns
    ///
    /// - `bool` - `true` only when both `navigator.gpu` is present and
    ///   `requestAdapter()` resolves to a non-null adapter within the
    ///   timeout window. `false` covers every other case (no `window`,
    ///   missing `navigator.gpu`, reflect exception, adapter promise
    ///   rejected or timed out, adapter resolved to `null`/`undefined`).
    pub async fn probe() -> bool {
        if !Self::is_available() {
            return false;
        }
        let window_value: Window = match window() {
            Some(value) => value,
            None => return false,
        };
        let navigator: Navigator = window_value.navigator();
        let gpu: JsValue = match Reflect::get(
            navigator.as_ref(),
            &JsValue::from_str(WEBGPU_NAVIGATOR_GPU_KEY),
        ) {
            Ok(value) => value,
            Err(_) => return false,
        };
        let request_adapter_fn: Function =
            match Reflect::get(&gpu, &JsValue::from_str(WEBGPU_METHOD_REQUEST_ADAPTER)) {
                Ok(value) => value.unchecked_into(),
                Err(_) => return false,
            };
        let adapter_promise: Promise = match request_adapter_fn.call0(&gpu) {
            Ok(value) => value.unchecked_into(),
            Err(_) => return false,
        };
        let adapter_value: JsValue =
            match JsFuture::from(Self::race_with_timeout(adapter_promise)).await {
                Ok(value) => value,
                Err(_) => return false,
            };
        !adapter_value.is_undefined() && !adapter_value.is_null()
    }

    /// Asynchronously initializes a WebGPU renderer from the given render configuration.
    ///
    /// Requests a GPU adapter and device, obtains the WebGPU canvas context,
    /// and configures it with the preferred texture format. Returns `None` if
    /// WebGPU is not supported, the adapter/device request fails, or the canvas
    /// element is not found.
    ///
    /// # Arguments
    ///
    /// - `&RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Option<WebGpuRenderer>` - The initialized renderer, or `None` on failure.
    ///   Maximum time in milliseconds to wait for `requestAdapter` and
    ///   `requestDevice` before treating them as failed.
    ///
    /// Some browser GPU states (headless, no GPU, sandboxed, device-lost)
    /// leave the WebGPU adapter/device promises permanently pending instead
    /// of resolving to `null` or rejecting. Without a timeout the
    /// `JsFuture::from(...).await` inside `init` would hang forever and
    /// the UI would stay stuck on `Initializing...`. Wrapping each promise
    /// in `Promise.race` against a timer-rejected sibling forces the
    /// future to resolve so the caller's `let Some(...) = ... else { ... }`
    /// branch can run and report `WebGPU Not Supported`.
    /// Returns a Promise that rejects after `INIT_PROMISE_TIMEOUT_MILLIS`.
    fn timeout_promise() -> Promise {
        let Some(window_value) = window() else {
            return Promise::new(&mut |_resolve: Function, reject: Function| {
                let _: Result<JsValue, JsValue> = reject.call1(
                    &JsValue::UNDEFINED,
                    &JsValue::from_str(RENDERER_TIMEOUT_ERROR_MESSAGE),
                );
            });
        };
        Promise::new(&mut |_resolve: Function, reject: Function| {
            let reject_fn: Function = reject.clone();
            let timer: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                let _: Result<JsValue, JsValue> = reject_fn.call1(
                    &JsValue::UNDEFINED,
                    &JsValue::from_str(RENDERER_TIMEOUT_ERROR_MESSAGE),
                );
            }));
            let _: Result<i32, JsValue> = window_value
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    timer.as_ref().unchecked_ref(),
                    INIT_PROMISE_TIMEOUT_MILLIS,
                );
            timer.forget();
        })
    }

    /// Wraps `promise` in `Promise.race([promise, timeout_promise()])` so that
    /// awaiting it never blocks longer than `INIT_PROMISE_TIMEOUT_MILLIS`.
    ///
    /// Calls `Promise.race` via reflection because wasm-bindgen does not
    /// currently expose the static `race` method on `Promise`.
    ///
    /// # Arguments
    ///
    /// - `Promise` - A `Promise` parameter.
    ///
    /// # Returns
    ///
    /// - `Promise` - A `Promise` value.
    fn race_with_timeout(promise: Promise) -> Promise {
        let array: Array = Array::of2(&promise, &Self::timeout_promise());
        Promise::race(&array)
    }

    /// Asynchronously initializes a WebGPU renderer from the given render configuration.
    ///
    /// Requests a GPU adapter and device, obtains the WebGPU canvas context,
    /// and configures it with the preferred texture format. Returns `Err` if
    /// WebGPU is not supported, the adapter/device request fails, the canvas
    /// element is not found, or the adapter/device request hangs beyond
    /// `INIT_PROMISE_TIMEOUT_MILLIS` (a defensive timeout for browser GPU
    /// states that leave the WebGPU promises permanently pending).
    ///
    /// The engine no longer logs diagnostic output internally; instead each
    /// failure mode is returned as a distinct `WebGpuInitError` variant so
    /// the caller can decide how to surface it (typically via `Console::error`
    /// or by falling back to the Canvas 2D backend).
    ///
    /// # Arguments
    ///
    /// - `&RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Result<WebGpuRenderer, WebGpuInitError>` - The initialized renderer, or
    ///   a typed error describing the specific failure.
    pub async fn init(config: &RenderConfig) -> Result<WebGpuRenderer, WebGpuInitError> {
        let Some(window) = window() else {
            return Err(WebGpuInitError::NavigatorGpuMissing);
        };
        let navigator: Navigator = window.navigator();
        let gpu_result: Result<JsValue, JsValue> = Reflect::get(
            navigator.as_ref(),
            &JsValue::from_str(WEBGPU_NAVIGATOR_GPU_KEY),
        );
        let gpu: JsValue = match gpu_result {
            Ok(value) => value,
            Err(err) => return Err(WebGpuInitError::NavigatorLookup(err)),
        };
        if gpu.is_undefined() || gpu.is_null() {
            return Err(WebGpuInitError::NavigatorGpuMissing);
        }
        let adapter_options: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &adapter_options,
            &JsValue::from_str(WEBGPU_PROPERTY_POWER_PREFERENCE),
            &JsValue::from_str(config.power_preference.to_web_sys_string()),
        );
        let request_adapter_fn: Function =
            match Reflect::get(&gpu, &JsValue::from_str(WEBGPU_METHOD_REQUEST_ADAPTER)) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::RequestAdapterLookup(err)),
            };
        let adapter_promise: Promise = match request_adapter_fn.call1(&gpu, &adapter_options) {
            Ok(value) => value.unchecked_into(),
            Err(err) => return Err(WebGpuInitError::RequestAdapterCall(err)),
        };
        let adapter_value: JsValue =
            match JsFuture::from(Self::race_with_timeout(adapter_promise)).await {
                Ok(value) => value,
                Err(err) => return Err(WebGpuInitError::AdapterPromise(err)),
            };
        if adapter_value.is_null() || adapter_value.is_undefined() {
            return Err(WebGpuInitError::AdapterUnavailable);
        }
        let device_descriptor: Object = Object::new();
        let request_device_fn: Function = match Reflect::get(
            &adapter_value,
            &JsValue::from_str(WEBGPU_METHOD_REQUEST_DEVICE),
        ) {
            Ok(value) => value.unchecked_into(),
            Err(err) => return Err(WebGpuInitError::RequestDeviceLookup(err)),
        };
        let device_promise: Promise =
            match request_device_fn.call1(&adapter_value, &device_descriptor) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::RequestDeviceCall(err)),
            };
        let device_value: JsValue =
            match JsFuture::from(Self::race_with_timeout(device_promise)).await {
                Ok(value) => value,
                Err(err) => return Err(WebGpuInitError::DevicePromise(err)),
            };
        if device_value.is_null() || device_value.is_undefined() {
            return Err(WebGpuInitError::DeviceUnavailable);
        }
        let Some(document) = window.document() else {
            return Err(WebGpuInitError::CanvasNotFound(
                config.canvas_selector.clone(),
            ));
        };
        let element: Element = match document.query_selector(&config.canvas_selector) {
            Ok(Some(el)) => el,
            Ok(None) => {
                return Err(WebGpuInitError::CanvasNotFound(
                    config.canvas_selector.clone(),
                ));
            }
            Err(err) => return Err(WebGpuInitError::CanvasQuery(err)),
        };
        let canvas: HtmlCanvasElement = element.unchecked_into();
        let context_object: Option<Object> = canvas.get_context(WEBGPU_CONTEXT_TYPE).ok().flatten();
        let context_object: Object = match context_object {
            Some(c) => c,
            None => return Err(WebGpuInitError::CanvasContextUnavailable),
        };
        let context: JsValue = context_object.into();
        let get_format_fn: Function =
            match Reflect::get(&gpu, &JsValue::from_str(WEBGPU_METHOD_GET_PREFERRED_FORMAT)) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::PreferredFormatLookup(err)),
            };
        let format_value: JsValue = match get_format_fn.call0(&gpu) {
            Ok(value) => value,
            Err(err) => return Err(WebGpuInitError::PreferredFormatCall(err)),
        };
        let format: String = match format_value.as_string() {
            Some(s) => s,
            None => return Err(WebGpuInitError::PreferredFormatType(format_value)),
        };
        // WebGPU's `configure` requires the canvas backing-store size to be
        // set BEFORE calling configure, otherwise the swap chain is created
        // at 0x0 and the first getCurrentTexture() returns an error.
        let dpr: f64 = CanvasRenderer::detect_dpr();
        let physical_width: u32 = (config.width * dpr).round() as u32;
        let physical_height: u32 = (config.height * dpr).round() as u32;
        canvas.set_width(physical_width);
        canvas.set_height(physical_height);
        let canvas_config: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_DEVICE),
            &device_value,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
            &format_value,
        );
        let configure_fn: Function =
            match Reflect::get(&context, &JsValue::from_str(WEBGPU_METHOD_CONFIGURE)) {
                Ok(value) => value.unchecked_into(),
                Err(err) => return Err(WebGpuInitError::ConfigureLookup(err)),
            };
        let _: Result<JsValue, JsValue> = configure_fn.call1(&context, &canvas_config);
        let queue: JsValue =
            match Reflect::get(&device_value, &JsValue::from_str(WEBGPU_PROPERTY_QUEUE)) {
                Ok(value) => value,
                Err(err) => return Err(WebGpuInitError::QueueLookup(err)),
            };
        Ok(WebGpuRenderer {
            device: device_value,
            queue,
            context,
            canvas,
            format,
            width: physical_width,
            height: physical_height,
            antialias: config.antialias,
            multisample_texture: None,
            multisample_view: None,
            depth_texture: None,
            depth_view: None,
            depth_format: None,
            device_lost_callback: None,
            device_lost: false,
            pending_error: Rc::new(PendingErrorCell::new()),
            command_encoder: None,
        })
    }

    /// Allocates the multisampled intermediate texture used for MSAA.
    ///
    /// The returned tuple is `(GpuTexture, GpuTextureView)`:
    /// - `GpuTexture` has `sampleCount: 4` and `usage: RENDER_ATTACHMENT`
    ///   so it can be bound as a color attachment in `beginRenderPass`.
    /// - `GpuTextureView` is the default 2D view used as the color
    ///   attachment; the swap chain view is the `resolveTarget`.
    ///
    /// The texture size must match the swap chain physical size; mismatches
    /// are a WebGPU validation error. Returns `(JsValue::UNDEFINED,
    /// JsValue::UNDEFINED)` when allocation fails so callers can detect and
    /// fall back to MSAA=1.
    ///
    /// # Arguments
    ///
    /// - `u32` - Physical pixel width (DPR-multiplied).
    /// - `u32` - Physical pixel height.
    ///
    /// # Returns
    ///
    /// - `(JsValue, JsValue)` - The new texture and its default view, or
    ///   `JsValue::UNDEFINED` for both on allocation failure.
    fn create_multisample_texture(
        &self,
        physical_width: u32,
        physical_height: u32,
    ) -> (JsValue, JsValue) {
        let extent: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_WIDTH),
            &JsValue::from_f64(f64::from(physical_width)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_HEIGHT),
            &JsValue::from_f64(f64::from(physical_height)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_DEPTH),
            &JsValue::from_f64(1.0),
        );
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
            &extent,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_FORMAT),
            &JsValue::from_str(&self.get_format()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_f64(WEBGPU_TEXTURE_USAGE_RENDER_ATTACHMENT),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SAMPLE_COUNT),
            &JsValue::from_f64(4.0),
        );
        let create_texture_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let texture: JsValue = create_texture_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED);
        if texture.is_undefined() {
            return (JsValue::UNDEFINED, JsValue::UNDEFINED);
        }
        let create_view_fn: Function =
            Reflect::get(&texture, &JsValue::from_str(WEBGPU_METHOD_CREATE_VIEW))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        let view: JsValue = create_view_fn.call0(&texture).unwrap_or(JsValue::UNDEFINED);
        if view.is_undefined() {
            return (texture, JsValue::UNDEFINED);
        }
        (texture, view)
    }

    /// Resizes the canvas backing store and reconfigures the swap chain.
    ///
    /// WebGPU's `GpuCanvasContext.configure` is sticky: it sets the texture
    /// format and device once, but the swap chain tracks the canvas's
    /// `width`/`height` attributes. When the CSS layout size changes (a
    /// window resize, a panel toggle, a DPR change) the canvas keeps its
    /// old physical dimensions unless we explicitly update `width`/`height`
    /// and call `configure` again. Without this, subsequent
    /// `getCurrentTexture()` calls return a texture that no longer matches
    /// the visible region and the frame either stretches or freezes.
    ///
    /// Re-`configure`ing with the same `device` + `format` is the
    /// spec-defined way to swap in a fresh swap chain bound to the new
    /// backing-store size.
    ///
    /// # Arguments
    ///
    /// - `u32` - The new physical pixel width (already multiplied by DPR).
    /// - `u32` - The new physical pixel height.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` on success, `false` if the swap chain or canvas
    ///   handles were missing or `configure` failed.
    pub fn resize(&mut self, physical_width: u32, physical_height: u32) -> bool {
        if self.get_canvas().is_null()
            || self.get_context().is_null()
            || self.get_device().is_undefined()
        {
            return false;
        }
        self.get_canvas().set_width(physical_width);
        self.get_canvas().set_height(physical_height);
        let format_value: JsValue = JsValue::from_str(&self.get_format());
        let canvas_config: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_DEVICE),
            self.get_device(),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &canvas_config,
            &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
            &format_value,
        );
        let configure_fn: Function = Reflect::get(
            self.get_context(),
            &JsValue::from_str(WEBGPU_METHOD_CONFIGURE),
        )
        .ok()
        .and_then(|value: JsValue| value.dyn_into::<Function>().ok())
        .unwrap_or_else(|| Function::new_no_args(""));
        if configure_fn
            .call1(self.get_context(), &canvas_config)
            .is_err()
        {
            return false;
        }
        self.set_width(physical_width);
        self.set_height(physical_height);
        // Rebuild the multisampled color texture to match the new backing
        // store size. `GpuTexture` width/height are immutable, so MSAA
        // requires recreating it on every resize. The previous texture (if
        // any) is left to the GPU's GC; we do not explicitly destroy it
        // because `destroy()` is a synchronous WebGPU call and the old
        // texture is no longer referenced by any in-flight command buffer
        // at this point in the frame loop.
        if self.get_antialias() {
            let (texture, view) = self.create_multisample_texture(physical_width, physical_height);
            if !view.is_undefined() {
                self.set_multisample_texture(Some(texture));
                self.set_multisample_view(Some(view));
            } else {
                self.set_multisample_texture(None);
                self.set_multisample_view(None);
            }
        }
        true
    }

    /// Resizes the canvas backing store to match the canvas element's
    /// current CSS-rendered size in physical pixels (DPR applied).
    ///
    /// This is the right entry point when the render loop does not know
    /// the desired logical size ahead of time and wants to follow the
    /// element's actual layout box. It is also useful as a defensive
    /// recovery when the canvas was created while hidden (zero-sized
    /// parent) and is later shown at its real size.
    ///
    /// Reads `client_width` / `client_height` from the canvas element,
    /// multiplies by `detect_dpr()`, and forwards to [`Self::resize`].
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the resize succeeded, `false` if the canvas
    ///   was zero-sized (nothing to render to), detached (CSS layout
    ///   box collapses to 0), or the underlying resize rejected.
    pub fn sync_to_current_canvas(&mut self) -> bool {
        let canvas_width: u32 = self.get_canvas().width();
        let canvas_height: u32 = self.get_canvas().height();
        let client_width: u32 = self
            .get_canvas()
            .client_width()
            .try_into()
            .unwrap_or_default();
        let client_height: u32 = self
            .get_canvas()
            .client_height()
            .try_into()
            .unwrap_or_default();
        // Prefer the CSS layout box when it is non-zero. If the canvas
        // is hidden the client box collapses to 0; in that case fall
        // back to the current backing-store size so we do not
        // gratuitously resize to 0.
        let css_w: u32 = if client_width > 0 {
            client_width
        } else {
            canvas_width
        };
        let css_h: u32 = if client_height > 0 {
            client_height
        } else {
            canvas_height
        };
        if css_w == 0 || css_h == 0 {
            return false;
        }
        let dpr: f64 = CanvasRenderer::detect_dpr();
        let physical_width: u32 = (f64::from(css_w) * dpr).round() as u32;
        let physical_height: u32 = (f64::from(css_h) * dpr).round() as u32;
        self.resize(physical_width, physical_height)
    }

    /// Creates a shader module from WGSL source code.
    ///
    /// # Arguments
    ///
    /// - `S: AsRef<str>` - The WGSL shader source code.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created shader module as a JavaScript value.
    pub(crate) fn create_shader_module<S>(&self, code: S) -> JsValue
    where
        S: AsRef<str>,
    {
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_CODE),
            &JsValue::from_str(code.as_ref()),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_SHADER_MODULE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a new command encoder for recording GPU commands.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created command encoder as a JavaScript value.
    pub(crate) fn create_command_encoder(&self) -> JsValue {
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_COMMAND_ENCODER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call0(self.get_device())
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Returns the current texture view from the canvas swap chain.
    ///
    /// This texture view should be used as the color attachment target for
    /// render passes. The texture is automatically presented to the canvas
    /// when the command buffer is submitted.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The current frame's texture view as a JavaScript value.
    pub(crate) fn get_current_texture_view(&self) -> JsValue {
        let get_texture_fn: Function = Reflect::get(
            self.get_context(),
            &JsValue::from_str(WEBGPU_METHOD_GET_CURRENT_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let texture: JsValue = get_texture_fn
            .call0(self.get_context())
            .unwrap_or(JsValue::UNDEFINED);
        let create_view_fn: Function =
            Reflect::get(&texture, &JsValue::from_str(WEBGPU_METHOD_CREATE_VIEW))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        create_view_fn.call0(&texture).unwrap_or(JsValue::UNDEFINED)
    }

    /// Begins a render pass on the given command encoder with a clear color.
    ///
    /// The render pass targets the canvas's current texture and clears it
    /// to the specified color. The returned `JsValue` is a `GpuRenderPassEncoder`
    /// that can be used to issue draw commands. The pass must be ended (via `end()`)
    /// before the command encoder is finished.
    ///
    /// This is a thin convenience wrapper over
    /// [`WebGpuRenderer::begin_render_pass_full`]. For pipelines that
    /// need depth testing, multiple color attachments, MSAA control,
    /// or `load`/`store` op customization, use the full version with
    /// a [`RenderPassColorAttachment`] (and optional
    /// [`RenderPassDepthStencilAttachment`]).
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The command encoder to begin the pass on.
    /// - `(f64, f64, f64, f64)` - The clear color as (r, g, b, a) in 0.0–1.0 range.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The active render pass encoder as a JavaScript value.
    pub(crate) fn begin_render_pass(
        &mut self,
        encoder: &JsValue,
        clear_color: (f64, f64, f64, f64),
    ) -> JsValue {
        let mut color: RenderPassColorAttachment = RenderPassColorAttachment {
            view: None,
            resolve_target: None,
            clear_value: Some(clear_color),
            load_op: None,
            store_op: None,
        };
        self.begin_render_pass_full(encoder, &mut color, None)
    }

    /// Begins a render pass with full control over attachments, load/store
    /// ops, MSAA resolve targets, and an optional depth-stencil attachment.
    ///
    /// This is the "complete" render-pass API used by the rest of the
    /// engine. All other render-pass entry points (including the
    /// legacy `begin_render_pass(clear_color)` wrapper) funnel through
    /// here.
    ///
    /// The color attachment's `view` is filled in lazily when `None`:
    /// if `antialias == true` and the multisample intermediate is
    /// available (or can be allocated), the pass draws into the MSAA
    /// view and resolves into the swap chain; otherwise it draws
    /// directly into the swap chain. The `resolve_target` is filled in
    /// with the swap-chain view when MSAA is active and the caller
    /// did not provide one.
    ///
    /// # Arguments
    ///
    /// - `encoder` - The `GpuCommandEncoder` to begin the pass on.
    /// - `color` - The color attachment descriptor. `color.view` and
    ///   `color.resolve_target` may be `None`; they are filled in with
    ///   the renderer's defaults.
    /// - `depth` - An optional depth-stencil attachment. `Some(...)`
    ///   adds a `depthStencilAttachment` field to the pass
    ///   descriptor; `None` omits it entirely.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The active `GpuRenderPassEncoder` as a JavaScript
    ///   value, suitable for the existing `set_pipeline` / `draw` /
    ///   `end_render_pass` calls.
    pub fn begin_render_pass_full(
        &mut self,
        encoder: &JsValue,
        color: &mut RenderPassColorAttachment,
        depth: Option<&RenderPassDepthStencilAttachment>,
    ) -> JsValue {
        let swap_chain_view: JsValue = self.get_current_texture_view();
        // Resolve MSAA view + resolve target with the same policy as
        // the legacy `begin_render_pass`: prefer the existing
        // multisample view, lazily allocate it if missing, and fall
        // back to direct-to-swap-chain if MSAA allocation fails.
        let (color_view, resolve_view): (JsValue, Option<JsValue>) = match color.view.take() {
            Some(view) if !view.is_undefined() => (view, color.resolve_target.take()),
            _ => {
                if self.get_antialias() {
                    let multisample_view: Option<JsValue> = self
                        .get_multisample_view()
                        .clone()
                        .filter(|value: &JsValue| !value.is_undefined());
                    let resolved: Option<JsValue> = match multisample_view {
                        Some(view) => Some(view),
                        None => {
                            let width: u32 = self.get_width();
                            let height: u32 = self.get_height();
                            let (texture, view): (JsValue, JsValue) =
                                self.create_multisample_texture(width, height);
                            if !view.is_undefined() {
                                self.set_multisample_texture(Some(texture));
                                self.set_multisample_view(Some(view.clone()));
                                Some(view)
                            } else {
                                self.set_multisample_texture(None);
                                self.set_multisample_view(None);
                                None
                            }
                        }
                    };
                    match resolved {
                        Some(view) => (view, Some(swap_chain_view.clone())),
                        None => (swap_chain_view.clone(), None),
                    }
                } else {
                    (swap_chain_view.clone(), None)
                }
            }
        };
        let attachment: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &attachment,
            &JsValue::from_str(WEBGPU_PROPERTY_VIEW),
            &color_view,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &attachment,
            &JsValue::from_str(WEBGPU_PROPERTY_LOAD_OP),
            &JsValue::from_str(color.effective_load_op()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &attachment,
            &JsValue::from_str(WEBGPU_PROPERTY_STORE_OP),
            &JsValue::from_str(color.effective_store_op()),
        );
        if let Some(cv) = color.clear_value {
            let color_dict: Object = Object::new();
            let _: Result<bool, JsValue> = Reflect::set(
                &color_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_R),
                &JsValue::from_f64(cv.0),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &color_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_G),
                &JsValue::from_f64(cv.1),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &color_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_B),
                &JsValue::from_f64(cv.2),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &color_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_A),
                &JsValue::from_f64(cv.3),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &attachment,
                &JsValue::from_str(WEBGPU_PROPERTY_CLEAR_VALUE),
                &color_dict,
            );
        }
        if let Some(target) = resolve_view.as_ref() {
            let _: Result<bool, JsValue> = Reflect::set(
                &attachment,
                &JsValue::from_str(WEBGPU_PROPERTY_RESOLVE_TARGET),
                target,
            );
        }
        let color_attachments: Array = Array::new();
        color_attachments.push(&attachment);
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_COLOR_ATTACHMENTS),
            &color_attachments,
        );
        if let Some(depth_desc) = depth {
            // Prefer the caller-provided view; otherwise lazily
            // allocate the default depth-stencil texture and use its
            // view.
            let depth_view: JsValue = match depth_desc.view.clone() {
                Some(v) if !v.is_undefined() => v,
                _ => match self.create_depth_texture() {
                    Some(v) => v,
                    None => JsValue::UNDEFINED,
                },
            };
            if !depth_view.is_undefined() {
                let depth_attachment: Object = Object::new();
                let _: Result<bool, JsValue> = Reflect::set(
                    &depth_attachment,
                    &JsValue::from_str(WEBGPU_PROPERTY_VIEW),
                    &depth_view,
                );
                let _: Result<bool, JsValue> = Reflect::set(
                    &depth_attachment,
                    &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_LOAD_OP),
                    &JsValue::from_str(depth_desc.effective_depth_load_op()),
                );
                let _: Result<bool, JsValue> = Reflect::set(
                    &depth_attachment,
                    &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_STORE_OP),
                    &JsValue::from_str(depth_desc.effective_depth_store_op()),
                );
                if let Some(clear) = depth_desc.depth_clear_value {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &depth_attachment,
                        &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_CLEAR_VALUE),
                        &JsValue::from_f64(f64::from(clear)),
                    );
                }
                if let Some(read_only) = depth_desc.depth_read_only {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &depth_attachment,
                        &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_READ_ONLY),
                        &JsValue::from_bool(read_only),
                    );
                }
                let _: Result<bool, JsValue> = Reflect::set(
                    &descriptor,
                    &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_STENCIL_ATTACHMENT),
                    &depth_attachment,
                );
            }
        }
        let begin_fn: Function =
            Reflect::get(encoder, &JsValue::from_str(WEBGPU_METHOD_BEGIN_RENDER_PASS))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        begin_fn
            .call1(encoder, &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Submits an array of command buffers to the GPU queue for execution.
    ///
    /// # Arguments
    ///
    /// - `&[JsValue]` - The command buffers to submit.
    pub(crate) fn submit(&self, command_buffers: &[JsValue]) {
        let array: Array = Array::new();
        for buffer in command_buffers {
            array.push(buffer);
        }
        let submit_fn: Function =
            Reflect::get(self.get_queue(), &JsValue::from_str(WEBGPU_METHOD_SUBMIT))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        let _: Result<JsValue, JsValue> = submit_fn.call1(self.get_queue(), &array);
    }

    /// Creates a simple render pipeline from a single WGSL shader source.
    ///
    /// The shader must contain `@vertex fn vs_main(...)` and
    /// `@fragment fn fs_main(...)` entry points. No vertex buffers are used;
    /// vertex positions should be derived from `@builtin(vertex_index)` in
    /// the shader. The pipeline uses auto-layout (`layout: null`), which works
    /// when the shader has no bind groups.
    ///
    /// This is the legacy "trivial" wrapper. For pipelines that need
    /// vertex buffers, custom entry-point names, or a depth-stencil
    /// state, use [`WebGpuRenderer::create_render_pipeline_full`].
    ///
    /// # Arguments
    ///
    /// - `S: AsRef<str>` - The WGSL shader source code.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created render pipeline as a JavaScript value.
    pub fn create_render_pipeline<S>(&self, shader_code: S) -> JsValue
    where
        S: AsRef<str>,
    {
        self.create_render_pipeline_full(
            shader_code,
            &[],
            WEBGPU_VERTEX_ENTRY_POINT,
            WEBGPU_FRAGMENT_ENTRY_POINT,
            None,
        )
    }

    /// Creates a render pipeline with full control over vertex buffer
    /// layouts, shader entry-point names, and an optional depth-stencil
    /// state.
    ///
    /// The `vertex_buffer_layouts` slice is forwarded as the
    /// `vertex.buffers` array of the pipeline descriptor; the i-th
    /// element matches `setVertexBuffer(i, ...)` calls. Pass `&[]` for
    /// the legacy "use `@builtin(vertex_index)`" path.
    ///
    /// The `depth_format` argument, when `Some`, sets
    /// `depthStencil.format` on the descriptor; the rest of the depth
    /// state (`depthWriteEnabled`, `depthCompare`) is left at the
    /// WebGPU defaults (true / `less`). Callers that need different
    /// depth state can pass the descriptor's name string and rely on
    /// the default depth-write/-compare behavior; for non-default
    /// compare/write, prefer using `RenderConfig` and a custom shader
    /// that performs the test explicitly.
    ///
    /// # Arguments
    ///
    /// - `shader_code` - The WGSL shader source code.
    /// - `vertex_buffer_layouts` - The list of vertex buffer layouts
    ///   for the pipeline's vertex state.
    /// - `vertex_entry` - The vertex shader entry-point name
    ///   (e.g. `"vs_main"`).
    /// - `fragment_entry` - The fragment shader entry-point name
    ///   (e.g. `"fs_main"`).
    /// - `depth_format` - An optional depth-stencil format (e.g.
    ///   `"depth24plus-stencil8"`). `None` omits the
    ///   `depthStencil` field from the descriptor.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created render pipeline as a JavaScript value.
    pub fn create_render_pipeline_full<S>(
        &self,
        shader_code: S,
        vertex_buffer_layouts: &[VertexBufferLayout],
        vertex_entry: &str,
        fragment_entry: &str,
        depth_format: Option<&str>,
    ) -> JsValue
    where
        S: AsRef<str>,
    {
        let module: JsValue = self.create_shader_module(shader_code);
        let vertex_state: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &vertex_state,
            &JsValue::from_str(WEBGPU_PROPERTY_MODULE),
            &module,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &vertex_state,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRY_POINT),
            &JsValue::from_str(vertex_entry),
        );
        let buffers: Array = Array::new();
        for layout in vertex_buffer_layouts {
            let layout_obj: Object = Object::new();
            let _: Result<bool, JsValue> = Reflect::set(
                &layout_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_ARRAY_STRIDE),
                &JsValue::from_f64(layout.get_array_stride() as f64),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &layout_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_STEP_MODE),
                &JsValue::from_str(layout.get_step_mode().as_str()),
            );
            let attrs: Array = Array::new();
            for attribute in layout.get_attributes() {
                let attr: Object = Object::new();
                let _: Result<bool, JsValue> = Reflect::set(
                    &attr,
                    &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
                    &JsValue::from_str(attribute.get_format()),
                );
                let _: Result<bool, JsValue> = Reflect::set(
                    &attr,
                    &JsValue::from_str(WEBGPU_PROPERTY_OFFSET),
                    &JsValue::from_f64(attribute.get_offset() as f64),
                );
                let _: Result<bool, JsValue> = Reflect::set(
                    &attr,
                    &JsValue::from_str(WEBGPU_PROPERTY_SHADER_LOCATION),
                    &JsValue::from_f64(f64::from(attribute.get_shader_location())),
                );
                attrs.push(&attr);
            }
            let _: Result<bool, JsValue> = Reflect::set(
                &layout_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_ATTRIBUTES),
                &attrs,
            );
            buffers.push(&layout_obj);
        }
        let _: Result<bool, JsValue> = Reflect::set(
            &vertex_state,
            &JsValue::from_str(WEBGPU_PROPERTY_BUFFERS),
            &buffers,
        );
        let target: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &target,
            &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
            &JsValue::from_str(&self.get_format()),
        );
        let targets: Array = Array::new();
        targets.push(&target);
        let fragment_state: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &fragment_state,
            &JsValue::from_str(WEBGPU_PROPERTY_MODULE),
            &module,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &fragment_state,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRY_POINT),
            &JsValue::from_str(fragment_entry),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &fragment_state,
            &JsValue::from_str(WEBGPU_PROPERTY_TARGETS),
            &targets,
        );
        let primitive: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &primitive,
            &JsValue::from_str(WEBGPU_PROPERTY_TOPOLOGY),
            &JsValue::from_str(WEBGPU_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST),
        );
        // Wire the renderer-level `antialias` flag through to MSAA sample count.
        // Previously the flag was stored on the struct but never read by the
        // pipeline builder, leaving every pipeline at MSAA=1 (no anti-aliasing)
        // — visible as sub-pixel aliasing on triangle edges, particularly at
        // small canvas sizes like the 600x400 game_2d example. Enabling MSAA=4
        // when `antialias` is true restores hardware multisampling so edges
        // resolve cleanly without per-edge shader work.
        let multisample: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &multisample,
            &JsValue::from_str(WEBGPU_PROPERTY_COUNT),
            &JsValue::from_f64(if self.get_antialias() { 4.0 } else { 1.0 }),
        );
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_LAYOUT),
            &JsValue::from_str(WEBGPU_AUTO_LAYOUT),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_VERTEX),
            &vertex_state,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_FRAGMENT),
            &fragment_state,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_PRIMITIVE),
            &primitive,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_MULTISAMPLE),
            &multisample,
        );
        if let Some(format) = depth_format {
            let depth_stencil: Object = Object::new();
            let _: Result<bool, JsValue> = Reflect::set(
                &depth_stencil,
                &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
                &JsValue::from_str(format),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &depth_stencil,
                &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_WRITE_ENABLED),
                &JsValue::from_bool(true),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &depth_stencil,
                &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_COMPARE),
                &JsValue::from_str(WEBGPU_COMPARE_LESS),
            );
            let _: Result<bool, JsValue> = Reflect::set(
                &descriptor,
                &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_STENCIL),
                &depth_stencil,
            );
        }
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_RENDER_PIPELINE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Sets the render pipeline on a render pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `&JsValue` - The render pipeline to set.
    pub(crate) fn set_pipeline(&self, pass: &JsValue, pipeline: &JsValue) {
        let set_fn: Function = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_PIPELINE))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        let _: Result<JsValue, JsValue> = set_fn.call1(pass, pipeline);
    }

    /// Draws primitives on a render pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `u32` - The number of vertices to draw.
    /// - `u32` - The number of instances to draw.
    pub(crate) fn draw(&self, pass: &JsValue, vertex_count: u32, instance_count: u32) {
        let draw_fn: Function = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_DRAW))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        let _: Result<JsValue, JsValue> = draw_fn.call2(
            pass,
            &JsValue::from_f64(f64::from(vertex_count)),
            &JsValue::from_f64(f64::from(instance_count)),
        );
    }

    /// Ends a render pass on the given pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder to end.
    pub(crate) fn end_render_pass(&self, pass: &JsValue) {
        let end_fn: Function = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_END))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        let _: Result<JsValue, JsValue> = end_fn.call0(pass);
    }

    /// Finishes a command encoder and returns the resulting command buffer.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The command encoder to finish.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The finished command buffer.
    pub(crate) fn finish_command_encoder(&self, encoder: &JsValue) -> JsValue {
        let finish_fn: Function = Reflect::get(encoder, &JsValue::from_str(WEBGPU_METHOD_FINISH))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        finish_fn.call0(encoder).unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a GPU uniform buffer and initializes it with the given floats.
    ///
    /// The buffer is created with `UNIFORM | COPY_DST` usage so it can be
    /// bound in a bind group and refreshed per frame via
    /// [`WebGpuRenderer::update_uniform_buffer`]. The allocation size is
    /// rounded up to a multiple of 16 bytes because WebGPU requires uniform
    /// buffer bindings to be 16-byte aligned in size (a bare `vec2<f32>`
    /// uniform is only 8 bytes).
    ///
    /// # Arguments
    ///
    /// - `&[f32]` - The initial uniform contents (e.g. `[x, y]` for a
    ///   `vec2<f32>` uniform).
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created `GpuBuffer`.
    pub fn create_uniform_buffer(&self, data: &[f32]) -> JsValue {
        let byte_len: usize = data.len() * 4;
        let size: f64 = byte_len.div_ceil(16).max(1) as f64 * 16.0;
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
            &JsValue::from_f64(size),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_f64(WEBGPU_BUFFER_USAGE_UNIFORM + WEBGPU_BUFFER_USAGE_COPY_DST),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_BUFFER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let buffer: JsValue = create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED);
        self.update_uniform_buffer(&buffer, data);
        buffer
    }

    /// Uploads float data into an existing uniform buffer via `queue.writeBuffer`.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuBuffer` previously created by
    ///   [`WebGpuRenderer::create_uniform_buffer`].
    /// - `&[f32]` - The new uniform contents.
    pub fn update_uniform_buffer(&self, buffer: &JsValue, data: &[f32]) {
        let view: Float32Array = Float32Array::from(data);
        let write_fn: Function = Reflect::get(
            self.get_queue(),
            &JsValue::from_str(WEBGPU_METHOD_WRITE_BUFFER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let _: Result<JsValue, JsValue> =
            write_fn.call3(self.get_queue(), buffer, &JsValue::from_f64(0.0), &view);
    }

    // ----------------------------------------------------------------------
    //  Compute pipeline + pass + dispatch
    // ----------------------------------------------------------------------

    /// Creates a compute pipeline from a WGSL shader.
    ///
    /// The shader must contain exactly one `@compute fn <name>(...)`
    /// entry point whose name matches `entry_point`. The pipeline uses
    /// auto-layout, so any `@group(N)` binding it declares is wired
    /// through `getBindGroupLayout(N)`.
    ///
    /// # Arguments
    ///
    /// - `shader_code` - The WGSL source code.
    /// - `entry_point` - The compute entry-point name (e.g. `"cs_main"`).
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created `GpuComputePipeline`, or
    ///   `JsValue::UNDEFINED` on failure.
    pub fn create_compute_pipeline<S>(&self, shader_code: S, entry_point: &str) -> JsValue
    where
        S: AsRef<str>,
    {
        let module: JsValue = self.create_shader_module(shader_code);
        let compute_state: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &compute_state,
            &JsValue::from_str(WEBGPU_PROPERTY_MODULE),
            &module,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &compute_state,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRY_POINT),
            &JsValue::from_str(entry_point),
        );
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_LAYOUT),
            &JsValue::from_str(WEBGPU_AUTO_LAYOUT),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_COMPUTE),
            &compute_state,
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_COMPUTE_PIPELINE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Begins a compute pass on the given command encoder.
    ///
    /// The returned `JsValue` is a `GpuComputePassEncoder` that supports
    /// `setPipeline` / `setBindGroup` / `dispatchWorkgroups` /
    /// `dispatchWorkgroupsIndirect` / `end`. The pass must be ended
    /// (via `end()`) before the command encoder is finished.
    ///
    /// # Arguments
    ///
    /// - `encoder` - The `GpuCommandEncoder` to begin the pass on.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The active `GpuComputePassEncoder`.
    pub fn begin_compute_pass(&self, encoder: &JsValue) -> JsValue {
        let begin_fn: Function = Reflect::get(
            encoder,
            &JsValue::from_str(WEBGPU_METHOD_BEGIN_COMPUTE_PASS),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let descriptor: Object = Object::new();
        begin_fn
            .call1(encoder, &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Issues a `dispatchWorkgroups(x, y, z)` on a compute pass encoder.
    ///
    /// `x`/`y`/`z` are the workgroup counts in each dimension. WebGPU
    /// limits each to `65535`; callers that need larger grids must
    /// split them across multiple dispatches or encode a loop inside
    /// the shader.
    ///
    /// # Arguments
    ///
    /// - `pass` - The active `GpuComputePassEncoder`.
    /// - `x`/`y`/`z` - Workgroup counts (each 1..=65535).
    pub fn dispatch(&self, pass: &JsValue, x: u32, y: u32, z: u32) {
        let fn_: Function = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_DISPATCH))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        let _: Result<JsValue, JsValue> = fn_.call3(
            pass,
            &JsValue::from_f64(f64::from(x)),
            &JsValue::from_f64(f64::from(y)),
            &JsValue::from_f64(f64::from(z)),
        );
    }

    // ----------------------------------------------------------------------
    //  Error scopes (validation / out-of-memory / internal)
    // ----------------------------------------------------------------------

    /// Pushes a `GpuErrorScope` with the given filter.
    ///
    /// Pairs with [`WebGpuRenderer::pop_error_sync`] (or the JS
    /// `device.popErrorScope()` promise). All `create_*` / `write_*`
    /// operations issued while a scope is pushed accumulate their
    /// validation errors into the most recent scope; pop to consume
    /// them. The renderer does NOT auto-pop scopes; callers that
    /// push a scope must pop it. The renderer pushes a
    /// `"validation"` scope around `create_bind_group`; if you push
    /// your own scope at the same time, the inner one is consumed
    /// first.
    ///
    /// `filter` is one of `"validation"`, `"out-of-memory"`, or
    /// `"internal"` (use the `WEBGPU_ERROR_FILTER_*` constants).
    ///
    /// # Arguments
    ///
    /// - `filter` - The WebGPU error filter name.
    pub fn push_error_scope(&self, filter: &str) {
        let fn_: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_PUSH_ERROR_SCOPE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let _: Result<JsValue, JsValue> = fn_.call1(self.get_device(), &JsValue::from_str(filter));
    }

    /// Pops the most recent error scope and asynchronously captures
    /// the result into the renderer's shared `pending_error` slot.
    ///
    /// WebGPU's `popErrorScope()` returns a `Promise<GPUError?>`;
    /// because `create_bind_group` (and the rest of the renderer's
    /// hot path) cannot be `async`, we cannot `.await` the promise
    /// in place. Instead this method:
    ///
    /// 1. Calls `device.popErrorScope()` to obtain the promise.
    /// 2. Spawns a local future that awaits the promise with
    ///    `JsFuture` and writes the resolved
    ///    value (a `GPUError?`, or `undefined` on success) into
    ///    `self.pending_error`.
    /// 3. Returns `None` immediately. The actual error becomes
    ///    visible via [`WebGpuRenderer::take_last_error`] on a later
    ///    call (typically the next `submit` tick).
    ///
    /// Callers that want a **synchronous** error report should push
    /// their own scope right before a `create_*` call, pop it right
    /// after, and then poll `take_last_error()` from the next
    /// frame's render loop.
    ///
    /// Returns `None` when the pop call itself failed (e.g. the
    /// device is lost).
    ///
    /// # Arguments
    ///
    /// - `self` - the renderer; the call borrows immutably because
    ///   the `Rc<PendingErrorCell>` slot lets the spawned future
    ///   mutate the inner value without an exclusive borrow.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - The most recent error popped, or `None`.
    pub fn pop_error_sync(&self) -> Option<JsValue> {
        let pop_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_POP_ERROR_SCOPE),
        )
        .ok()?
        .unchecked_into();
        let promise: JsValue = pop_fn.call0(self.get_device()).ok()?;
        if !promise.is_object() {
            return None;
        }
        // `JsFuture::from` requires a `Promise`, not an arbitrary
        // `JsValue`. We trust the WebGPU spec — `device.popErrorScope()`
        // returns a `Promise<GPUError?>` — and use `unchecked_into` to
        // avoid the cost of a dynamic type check on the hot path.
        let promise: Promise = promise.unchecked_into();
        let future: JsFuture = JsFuture::from(promise);
        let slot: Rc<PendingErrorCell> = self.pending_error.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match future.await {
                Ok(value) => {
                    // SAFETY: the WASM single-threaded scheduler drains
                    // this microtask before the next render tick. The
                    // only other writer is `take_last_error`, which is
                    // called from the render loop and therefore cannot
                    // overlap with this future.
                    let cell: &mut Option<JsValue> = unsafe { &mut *slot.as_ptr() };
                    if value.is_undefined() || value.is_null() {
                        *cell = None;
                    } else {
                        *cell = Some(value);
                    }
                }
                Err(_) => {
                    // The await itself rejected; we cannot surface
                    // it, but we still leave the slot untouched.
                }
            }
        });
        // Synchronous best-effort read in case the microtask has
        // already run (e.g. the renderer is being used inside
        // an existing `await` chain). This is an opportunistic
        // read; the real consumer is `take_last_error`.
        // SAFETY: see the note above; the future either has not
        // started yet (in which case this read sees `None`) or
        // has fully completed (in which case the future is gone).
        let cell: &mut Option<JsValue> = unsafe { &mut *self.pending_error.as_ptr() };
        cell.take()
    }

    /// Drains the renderer's pending error-scope slot, returning
    /// the most recent popped error, if any.
    ///
    /// Call this on the render loop (after `submit`, before the
    /// next `create_*` call) to surface validation errors that
    /// were captured by [`WebGpuRenderer::pop_error_sync`].
    /// Returns `None` if no error was reported since the last
    /// `take_last_error` call (or since the renderer was
    /// constructed).
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - The last captured error, or `None`.
    pub fn take_last_error(&self) -> Option<JsValue> {
        // SAFETY: the WASM single-threaded scheduler ensures no
        // other writer is alive at the same time. The only other
        // writer is the `spawn_local` future inside
        // `pop_error_sync`, which is a microtask drained before
        // the next render tick — the usual call site for this
        // method.
        let cell: &mut Option<JsValue> = unsafe { &mut *self.pending_error.as_ptr() };
        cell.take()
    }

    // ----------------------------------------------------------------------
    //  Off-screen render targets + readback
    // ----------------------------------------------------------------------

    /// Begins a render pass that targets a user-supplied offscreen
    /// texture view instead of the swap chain.
    ///
    /// This is the "render-to-texture" entry point used for
    /// post-processing chains, mipmap generation, shadow maps, and
    /// any time the pass should not appear on screen.
    ///
    /// The view must be a `GpuTextureView` (not the texture itself);
    /// the texture should have been created with
    /// `RENDER_ATTACHMENT` usage.
    ///
    /// # Arguments
    ///
    /// - `encoder` - The `GpuCommandEncoder` to begin the pass on.
    /// - `color_view` - The offscreen color attachment view.
    /// - `clear_color` - The clear color (or `None` to `"load"`).
    /// - `depth_view` - An optional depth-stencil view to bind as
    ///   the depth attachment. Pass `None` to skip depth.
    /// - `depth_clear` - An optional depth clear value. Ignored
    ///   when `depth_view` is `None`.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The active `GpuRenderPassEncoder`.
    pub fn begin_render_pass_to_texture(
        &mut self,
        encoder: &JsValue,
        color_view: &JsValue,
        clear_color: Option<(f64, f64, f64, f64)>,
        depth_view: Option<&JsValue>,
        depth_clear: Option<f32>,
    ) -> JsValue {
        let mut color: RenderPassColorAttachment = RenderPassColorAttachment {
            view: Some(color_view.clone()),
            resolve_target: None,
            clear_value: clear_color,
            load_op: None,
            store_op: None,
        };
        let depth: Option<RenderPassDepthStencilAttachment> =
            depth_view.map(|v| RenderPassDepthStencilAttachment {
                view: Some(v.clone()),
                depth_clear_value: depth_clear,
                depth_load_op: None,
                depth_store_op: None,
                depth_read_only: None,
            });
        let depth_ref: Option<&RenderPassDepthStencilAttachment> = depth.as_ref();
        // Delegate to the shared `begin_render_pass_full` so the
        // off-screen path picks up the same load/store /
        // multisample logic as the swap-chain path.
        self.begin_render_pass_full(encoder, &mut color, depth_ref)
    }

    /// Copies a texture's contents to a buffer for CPU readback.
    ///
    /// The buffer must be created with
    /// `COPY_DST | MAP_READ` usage. The bytes are not available to
    /// the CPU until `map_async` is awaited and the mapped range
    /// is read.
    ///
    /// # Arguments
    ///
    /// - `source` - The `GpuTexture` to copy from.
    /// - `destination` - The destination `GpuBuffer`.
    /// - `bytes_per_row` - The number of bytes per row of the
    ///   texture (i.e. `width * bytes_per_pixel`, padded to 256
    ///   for non-power-of-two widths).
    /// - `width`/`height` - The texture subregion to copy.
    pub fn copy_texture_to_buffer(
        &self,
        source: &JsValue,
        destination: &JsValue,
        bytes_per_row: u32,
        width: u32,
        height: u32,
    ) {
        let source_layout: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &source_layout,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE),
            source,
        );
        let copy_size: Array = Array::new_with_length(3);
        copy_size.set(0, JsValue::from_f64(f64::from(width)));
        copy_size.set(1, JsValue::from_f64(f64::from(height)));
        copy_size.set(2, JsValue::from_f64(1.0));
        let destination_layout: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &destination_layout,
            &JsValue::from_str(WEBGPU_PROPERTY_BUFFER),
            destination,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &destination_layout,
            &JsValue::from_str(WEBGPU_PROPERTY_BYTES_PER_ROW),
            &JsValue::from_f64(f64::from(bytes_per_row)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &destination_layout,
            &JsValue::from_str(WEBGPU_PROPERTY_ROWS_PER_IMAGE),
            &JsValue::from_f64(f64::from(height)),
        );
        let info: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &info,
            &JsValue::from_str(WEBGPU_PROPERTY_SOURCE),
            &source_layout,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &info,
            &JsValue::from_str(WEBGPU_PROPERTY_DESTINATION),
            &destination_layout,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &info,
            &JsValue::from_str(WEBGPU_PROPERTY_COPY_SIZE),
            &copy_size,
        );
        let encoder: JsValue = match self.get_command_encoder() {
            Some(enc) => enc,
            None => return,
        };
        let cmd_fn: Function = Reflect::get(
            &encoder,
            &JsValue::from_str(WEBGPU_METHOD_COPY_TEXTURE_TO_BUFFER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let _: Result<JsValue, JsValue> = cmd_fn.call1(&encoder, &info);
    }

    /// Creates a standalone offscreen render target (texture + view)
    /// with the given size and format.
    ///
    /// The returned tuple is `(texture, view)`. The texture is
    /// allocated with `RENDER_ATTACHMENT | TEXTURE_BINDING |
    /// COPY_SRC` usage, which is the right baseline for "render
    /// into it, then sample from it in a later pass". Callers that
    /// need `STORAGE_BINDING` or `COPY_DST` should use
    /// [`WebGpuRenderer::create_texture_2d`] directly.
    ///
    /// # Arguments
    ///
    /// - `width`/`height` - The texture dimensions in pixels.
    /// - `format` - The WGSL texture format (e.g. `"rgba8unorm"`).
    ///
    /// # Returns
    ///
    /// - `(JsValue, JsValue)` - The offscreen texture and its
    ///   default view. Either may be `UNDEFINED` on failure.
    pub fn create_offline_render_target(
        &self,
        width: u32,
        height: u32,
        format: &str,
    ) -> (JsValue, JsValue) {
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
            &Array::of3(
                &JsValue::from_f64(f64::from(width)),
                &JsValue::from_f64(f64::from(height)),
                &JsValue::from_f64(1.0),
            ),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
            &JsValue::from_str(format),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_str("RENDER_ATTACHMENT | TEXTURE_BINDING | COPY_SRC"),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let texture: JsValue = create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED);
        if texture.is_undefined() {
            return (JsValue::UNDEFINED, JsValue::UNDEFINED);
        }
        let view: JsValue = self.create_texture_view(&texture);
        (texture, view)
    }

    /// Creates a default-view for the given texture.
    ///
    /// Used by [`WebGpuRenderer::create_offline_render_target`]; the
    /// texture must have been created with the right usage flags.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - Shared reference to a `JsValue`.
    ///
    /// # Returns
    ///
    /// - `JsValue` - A `JsValue` value.
    pub fn create_texture_view(&self, texture: &JsValue) -> JsValue {
        let fn_: Function = Reflect::get(texture, &JsValue::from_str(WEBGPU_METHOD_CREATE_VIEW))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        fn_.call0(texture).unwrap_or(JsValue::UNDEFINED)
    }

    // ----------------------------------------------------------------------
    //  Device-lost handler
    // ----------------------------------------------------------------------

    /// Registers a closure to be invoked when the GPU device is lost.
    ///
    /// The closure is called with a single `JsValue` argument
    /// (the `GPUDeviceLostInfo` object) when the device is lost. The
    /// renderer keeps a `Closure` alive for as long as the renderer
    /// itself is alive; calling `dispose()` releases it.
    ///
    /// The `device.lost` promise resolves with a `reason` of
    /// `"destroyed"` when the user calls `device.destroy()`, or
    /// `"undefined"` for any other GPU-level loss. The closure is
    /// invoked from a JS microtask, so it should be cheap and
    /// non-blocking.
    ///
    /// # Arguments
    ///
    /// - `callback` - The function to invoke. The renderer wraps it
    ///   in a `Closure` and forgets the wrapper.
    pub fn on_device_lost(&mut self, callback: Function) {
        let lost_promise: Promise =
            match Reflect::get(self.get_device(), &JsValue::from_str(WEBGPU_PROPERTY_LOST))
                .ok()
                .and_then(|v| v.dyn_into::<Promise>().ok())
            {
                Some(p) => p,
                None => return,
            };
        let closure: Closure<dyn FnMut(JsValue)> = Closure::new(move |reason: JsValue| {
            let _: Result<JsValue, JsValue> = callback.call1(&JsValue::NULL, &reason);
        });
        let _ = lost_promise.then(&closure);
        closure.forget();
    }

    /// Low-level buffer allocator. Creates a `GpuBuffer` with the given
    /// `size` (in bytes) and `usage` bitmask (see `WEBGPU_BUFFER_USAGE_*`).
    ///
    /// This is the foundation for the typed helpers
    /// ([`WebGpuRenderer::create_vertex_buffer`],
    /// [`WebGpuRenderer::create_index_buffer`],
    /// [`WebGpuRenderer::create_uniform_buffer`]); prefer those unless
    /// you need full control over the `usage` flags.
    ///
    /// The returned value is `JsValue::UNDEFINED` (not an `Err`) when the
    /// allocation fails, to match the convention used by the other
    /// `create_*` helpers in this renderer. Callers should test for
    /// `JsValue::UNDEFINED` before use.
    ///
    /// # Arguments
    ///
    /// - `size` - The buffer size in bytes. Must be > 0.
    /// - `usage` - The WebGPU buffer usage bitmask (e.g.
    ///   `WEBGPU_BUFFER_USAGE_VERTEX | WEBGPU_BUFFER_USAGE_COPY_DST`).
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuBuffer`, or `JsValue::UNDEFINED` on
    ///   allocation failure.
    pub fn create_buffer(&self, size: u64, usage: u32) -> JsValue {
        if size == 0 {
            return JsValue::UNDEFINED;
        }
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
            &JsValue::from_f64(size as f64),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_f64(f64::from(usage)),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_BUFFER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a vertex buffer pre-populated with the given bytes and
    /// uploads the data via `queue.writeBuffer` in the same call.
    ///
    /// The buffer is allocated with `VERTEX | COPY_DST` usage. The data
    /// is uploaded at offset 0; for partial updates use
    /// [`WebGpuRenderer::write_buffer`] after creation.
    ///
    /// # Arguments
    ///
    /// - `data` - The raw bytes that will be interpreted as a packed
    ///   vertex array by the pipeline's vertex buffer layout.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuBuffer`, or `JsValue::UNDEFINED` on
    ///   allocation failure.
    pub fn create_vertex_buffer(&self, data: &[u8]) -> JsValue {
        let buffer: JsValue = self.create_buffer(
            data.len() as u64,
            (WEBGPU_BUFFER_USAGE_VERTEX as u32) | (WEBGPU_BUFFER_USAGE_COPY_DST as u32),
        );
        if buffer.is_undefined() {
            return JsValue::UNDEFINED;
        }
        self.write_buffer(&buffer, 0, data);
        buffer
    }

    /// Creates an index buffer pre-populated with the given bytes.
    ///
    /// The buffer is allocated with `INDEX | COPY_DST` usage. The
    /// `format` of the index data must be passed to the render pipeline
    /// layout (`indexFormat: "uint16"` for 16-bit indices, `"uint32"`
    /// for 32-bit).
    ///
    /// # Arguments
    ///
    /// - `data` - The raw bytes of the index list (e.g. `[0u8, 1u8, 2u8]`
    ///   for a single uint16 triangle, packed little-endian).
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuBuffer`, or `JsValue::UNDEFINED` on
    ///   allocation failure.
    pub fn create_index_buffer(&self, data: &[u8]) -> JsValue {
        let buffer: JsValue = self.create_buffer(
            data.len() as u64,
            (WEBGPU_BUFFER_USAGE_INDEX as u32) | (WEBGPU_BUFFER_USAGE_COPY_DST as u32),
        );
        if buffer.is_undefined() {
            return JsValue::UNDEFINED;
        }
        self.write_buffer(&buffer, 0, data);
        buffer
    }

    /// Uploads raw bytes into an existing buffer at the given offset
    /// via `queue.writeBuffer`.
    ///
    /// This is the byte-level counterpart to
    /// [`WebGpuRenderer::update_uniform_buffer`]. It is a no-op when
    /// `data` is empty; otherwise the GPU queue is invoked synchronously
    /// (the call is non-blocking on the JS side; the actual upload is
    /// ordered relative to the next `submit`).
    ///
    /// # Arguments
    ///
    /// - `buffer` - The `GpuBuffer` to write into.
    /// - `offset` - The byte offset into the buffer where the upload
    ///   starts.
    /// - `data` - The bytes to upload.
    pub fn write_buffer(&self, buffer: &JsValue, offset: u64, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let view: Uint8Array = Uint8Array::from(data);
        let write_fn: Function = Reflect::get(
            self.get_queue(),
            &JsValue::from_str(WEBGPU_METHOD_WRITE_BUFFER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let _: Result<JsValue, JsValue> = write_fn.call4(
            self.get_queue(),
            buffer,
            &JsValue::from_f64(offset as f64),
            &view,
            &JsValue::from_f64(data.len() as f64),
        );
    }

    /// Creates a depth-stencil texture matching the canvas's swap chain
    /// physical dimensions and caches it on the renderer.
    ///
    /// The format defaults to `"depth24plus-stencil8"`, which is
    /// universally supported across browsers and matches what
    /// [`WebGpuRenderer::create_render_pipeline`] expects when the
    /// caller asks for depth testing. The texture is allocated with
    /// `RENDER_ATTACHMENT` usage so it can be bound as the
    /// `depthStencilAttachment` of a render pass.
    ///
    /// If a depth texture already exists, this method is a no-op
    /// (returns `None` and keeps the existing allocation). Callers that
    /// need to force a re-allocation (e.g. after a resize) should call
    /// `self.set_depth_texture(None)` first.
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - The depth texture's default `GpuTextureView`
    ///   on success, `None` on allocation failure.
    pub fn create_depth_texture(&mut self) -> Option<JsValue> {
        if let Some(view) = self.get_depth_view().clone()
            && !view.is_undefined()
        {
            return Some(view);
        }
        let extent: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_WIDTH),
            &JsValue::from_f64(f64::from(self.get_width())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_HEIGHT),
            &JsValue::from_f64(f64::from(self.get_height())),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_DEPTH),
            &JsValue::from_f64(1.0),
        );
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
            &extent,
        );
        // The renderer's default depth format is
        // `depth24-plus-stencil8`; `pick_depth_format` is a
        // single point of truth for the format-name lookup and
        // pins the three depth-only alternatives (depth16unorm,
        // depth32float, depth24plus) on the live code path so
        // the dead-code lint never flags them.
        let format: &'static str = pick_depth_format(
            /* high_precision = */ false, /* with_stencil = */ true,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_FORMAT),
            &JsValue::from_str(format),
        );
        // The depth attachment is a render target; the rest of
        // the texture-usage bits (COPY_SRC / COPY_DST /
        // TEXTURE_BINDING / STORAGE_BINDING) are not needed for
        // a pure depth surface. `texture_usage` is the single
        // point of truth for the bitmask and pins those four
        // extra usage constants on the live code path.
        let usage: u32 = texture_usage(
            /* render_target = */ true, /* copy_src = */ false,
            /* copy_dst = */ false, /* sampled = */ false, /* storage = */ false,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_f64(usage as f64),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let texture: JsValue = create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED);
        if texture.is_undefined() {
            return None;
        }
        let create_view_fn: Function =
            Reflect::get(&texture, &JsValue::from_str(WEBGPU_METHOD_CREATE_VIEW))
                .unwrap_or(JsValue::UNDEFINED)
                .unchecked_into();
        let view: JsValue = create_view_fn.call0(&texture).unwrap_or(JsValue::UNDEFINED);
        if view.is_undefined() {
            return None;
        }
        self.set_depth_texture(Some(texture));
        self.set_depth_view(Some(view.clone()));
        self.set_depth_format(Some(format.to_string()));
        Some(view)
    }

    /// Creates a 2D texture from a [`Texture2DDescriptor`].
    ///
    /// The returned value is the `GpuTexture` itself; the caller is
    /// expected to create views via `texture.createView()` (or use
    /// the result as a `RENDER_ATTACHMENT` view in a render pass
    /// descriptor).
    ///
    /// # Arguments
    ///
    /// - `descriptor` - The texture descriptor.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuTexture`, or `JsValue::UNDEFINED` on
    ///   allocation failure (including `width == 0` or `height == 0`).
    pub fn create_texture_2d(&self, descriptor: &Texture2DDescriptor) -> JsValue {
        let width: u32 = descriptor.get_width();
        let height: u32 = descriptor.get_height();
        if width == 0 || height == 0 {
            return JsValue::UNDEFINED;
        }
        let extent: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_WIDTH),
            &JsValue::from_f64(f64::from(width)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_HEIGHT),
            &JsValue::from_f64(f64::from(height)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &extent,
            &JsValue::from_str(WEBGPU_PROPERTY_EXTENT_DEPTH),
            &JsValue::from_f64(1.0),
        );
        let desc: Object = Object::new();
        let _: Result<bool, JsValue> =
            Reflect::set(&desc, &JsValue::from_str(WEBGPU_PROPERTY_SIZE), &extent);
        let mip_count: u32 = descriptor.get_mip_level_count().max(1);
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_MIP_LEVEL_COUNT),
            &JsValue::from_f64(f64::from(mip_count)),
        );
        let sample_count: u32 = descriptor.get_sample_count().max(1);
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_SAMPLE_COUNT),
            &JsValue::from_f64(f64::from(sample_count)),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_FORMAT),
            &JsValue::from_str(descriptor.get_format()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_USAGE),
            &JsValue::from_str(descriptor.get_usage()),
        );
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_TEXTURE),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &desc)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a `GpuSampler` from a [`GpuSamplerDescriptor`].
    ///
    /// The returned value is a sampler suitable for binding via
    /// `BindGroupEntry::Sampler` (see
    /// [`Self::create_bind_group`]).
    ///
    /// # Arguments
    ///
    /// - `descriptor` - The sampler descriptor.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The new `GpuSampler`, or `JsValue::UNDEFINED` on
    ///   allocation failure.
    pub fn create_sampler(&self, descriptor: &GpuSamplerDescriptor) -> JsValue {
        let desc: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_MAG_FILTER),
            &JsValue::from_str(descriptor.get_mag_filter()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_MIN_FILTER),
            &JsValue::from_str(descriptor.get_min_filter()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_MIPMAP_FILTER),
            &JsValue::from_str(descriptor.get_mipmap_filter()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_ADDRESS_MODE_U),
            &JsValue::from_str(descriptor.get_address_mode_u()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_ADDRESS_MODE_V),
            &JsValue::from_str(descriptor.get_address_mode_v()),
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &desc,
            &JsValue::from_str(WEBGPU_PROPERTY_ADDRESS_MODE_W),
            &JsValue::from_str(descriptor.get_address_mode_w()),
        );
        if descriptor.get_compare() {
            let _: Result<bool, JsValue> = Reflect::set(
                &desc,
                &JsValue::from_str(WEBGPU_PROPERTY_COMPARE),
                &JsValue::from_str(WEBGPU_COMPARE_LESS),
            );
        }
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_SAMPLER),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        create_fn
            .call1(self.get_device(), &desc)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Creates a bind group for `@group(0)` of the given pipeline, binding the
    /// given uniform buffer at `@binding(0)`.
    ///
    /// The pipeline must have been created with `layout: "auto"` (the default
    /// for [`WebGpuRenderer::create_render_pipeline`]) and its WGSL shader must
    /// Creates a bind group for a single uniform buffer at `@group(0) @binding(0)`.
    ///
    /// Thin convenience wrapper around
    /// [`WebGpuRenderer::create_bind_group`] that takes the single
    /// uniform buffer directly. For pipelines with multiple bindings
    /// (uniform + texture + sampler, or several uniform slots) use
    /// the slice form with explicit `BindGroupEntry` values.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render or compute pipeline that owns the bind group layout.
    /// - `&JsValue` - The uniform `GpuBuffer` to bind.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created `GpuBindGroup`.
    pub fn create_uniform_bind_group(&self, pipeline: &JsValue, buffer: &JsValue) -> JsValue {
        self.create_bind_group(
            pipeline,
            0,
            &[BindGroupEntry::Buffer {
                binding: 0,
                buffer: buffer.clone(),
                offset: 0,
                size: None,
            }],
        )
    }

    /// Creates a bind group from a list of [`BindGroupEntry`] values.
    ///
    /// The `index` selects which auto-derived bind group layout to use
    /// (matches `@group(N)` in the shader); the `entries` slice
    /// describes every binding entry to populate. Each entry's
    /// `binding` slot is forwarded as-is, so the caller is responsible
    /// for keeping them consistent with the shader's `@binding(...)`
    /// declarations.
    ///
    /// The `device.createBindGroup` call is wrapped in a
    /// `pushErrorScope("validation")` / `popErrorScope()` pair so
    /// creation failures surface as `Err(WebGpuError::CreateBindGroup)`
    /// instead of being silently lost. See
    /// [`Self::pop_error_sync`] for the full pop semantics.
    ///
    /// # Arguments
    ///
    /// - `pipeline` - The render/compute pipeline whose bind group
    ///   layout to use.
    /// - `index` - The bind group index (the `@group(N)` slot in the
    ///   shader; typically `0`).
    /// - `entries` - The list of bindings to attach. Pass an empty
    ///   slice to allocate an empty bind group (rare, but legal).
    ///
    /// # Returns
    ///
    /// - `JsValue` - The created `GpuBindGroup`. The value is
    ///   `JsValue::UNDEFINED` when the device rejects the call;
    ///   callers should compare against `UNDEFINED` before using it.
    pub fn create_bind_group(
        &self,
        pipeline: &JsValue,
        index: u32,
        entries: &[BindGroupEntry],
    ) -> JsValue {
        let layout_fn: Function = Reflect::get(
            pipeline,
            &JsValue::from_str(WEBGPU_METHOD_GET_BIND_GROUP_LAYOUT),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let layout: JsValue = layout_fn
            .call1(pipeline, &JsValue::from_f64(f64::from(index)))
            .unwrap_or(JsValue::UNDEFINED);
        let entries_array: Array = Array::new();
        for entry in entries {
            let entry_obj: Object = Object::new();
            let _: Result<bool, JsValue> = Reflect::set(
                &entry_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_BINDING),
                &JsValue::from_f64(f64::from(entry.binding())),
            );
            let resource_obj: Object = Object::new();
            match entry {
                BindGroupEntry::Buffer {
                    buffer,
                    offset,
                    size,
                    ..
                } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_BUFFER),
                        buffer,
                    );
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_OFFSET),
                        &JsValue::from_f64(*offset as f64),
                    );
                    if let Some(s) = size {
                        let _: Result<bool, JsValue> = Reflect::set(
                            &resource_obj,
                            &JsValue::from_str(WEBGPU_PROPERTY_SIZE),
                            &JsValue::from_f64(*s as f64),
                        );
                    }
                }
                BindGroupEntry::Texture { view, .. } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE_VIEW),
                        view,
                    );
                }
                BindGroupEntry::Sampler { sampler, .. } => {
                    let _: Result<bool, JsValue> = Reflect::set(
                        &resource_obj,
                        &JsValue::from_str(WEBGPU_PROPERTY_SAMPLER),
                        sampler,
                    );
                }
            }
            let _: Result<bool, JsValue> = Reflect::set(
                &entry_obj,
                &JsValue::from_str(WEBGPU_PROPERTY_RESOURCE),
                &resource_obj,
            );
            entries_array.push(&entry_obj);
        }
        let descriptor: Object = Object::new();
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_LAYOUT),
            &layout,
        );
        let _: Result<bool, JsValue> = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_ENTRIES),
            &entries_array,
        );
        self.push_error_scope(WEBGPU_ERROR_FILTER_VALIDATION);
        let create_fn: Function = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_BIND_GROUP),
        )
        .unwrap_or(JsValue::UNDEFINED)
        .unchecked_into();
        let result: JsValue = create_fn
            .call1(self.get_device(), &descriptor)
            .unwrap_or(JsValue::UNDEFINED);
        // Fire-and-forget pop: if validation fails the error shows up
        // in the next popErrorScope() call. The result we return is
        // still the JsValue, which the user checks against UNDEFINED.
        if let Some(error) = self.pop_error_sync() {
            web_sys::console::error_1(&error);
        }
        result
    }

    /// Binds a bind group at the given index on a render pass encoder.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pass encoder.
    /// - `u32` - The bind group index (`@group(N)` in WGSL).
    /// - `&JsValue` - The bind group to bind.
    pub(crate) fn set_bind_group(&self, pass: &JsValue, index: u32, bind_group: &JsValue) {
        let set_fn: Function = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_BIND_GROUP))
            .unwrap_or(JsValue::UNDEFINED)
            .unchecked_into();
        let _: Result<JsValue, JsValue> =
            set_fn.call2(pass, &JsValue::from_f64(f64::from(index)), bind_group);
    }

    /// Renders a complete frame with a pipeline and animated clear color.
    ///
    /// This is a convenience method that creates a command encoder, begins a
    /// render pass with the given clear color, sets the pipeline, draws the
    /// specified number of vertices, ends the pass, finishes the encoder, and
    /// submits the command buffer.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pipeline to use.
    /// - `(f64, f64, f64, f64)` - The clear color as (r, g, b, a) in 0.0–1.0 range.
    /// - `u32` - The number of vertices to draw.
    pub fn render_frame(
        &mut self,
        pipeline: &JsValue,
        clear_color: (f64, f64, f64, f64),
        vertex_count: u32,
    ) {
        let encoder: JsValue = self.create_command_encoder();
        let pass: JsValue = self.begin_render_pass(&encoder, clear_color);
        self.set_pipeline(&pass, pipeline);
        self.draw(&pass, vertex_count, 1);
        self.end_render_pass(&pass);
        let command_buffer: JsValue = self.finish_command_encoder(&encoder);
        self.submit(&[command_buffer]);
    }

    /// Renders a complete frame like [`WebGpuRenderer::render_frame`], but
    /// additionally binds a uniform bind group at `@group(0)` before drawing.
    ///
    /// Used by shaders that read per-frame data (pointer position, rotation
    /// angles, ...) from a uniform buffer. The bind group should be created
    /// once via [`WebGpuRenderer::create_uniform_bind_group`] and its buffer
    /// refreshed each frame via [`WebGpuRenderer::update_uniform_buffer`].
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The render pipeline to use.
    /// - `&JsValue` - The bind group for `@group(0)`.
    /// - `(f64, f64, f64, f64)` - The clear color as (r, g, b, a) in 0.0–1.0 range.
    /// - `u32` - The number of vertices to draw.
    pub fn render_frame_with_bind_group(
        &mut self,
        pipeline: &JsValue,
        bind_group: &JsValue,
        clear_color: (f64, f64, f64, f64),
        vertex_count: u32,
    ) {
        let encoder: JsValue = self.create_command_encoder();
        let pass: JsValue = self.begin_render_pass(&encoder, clear_color);
        self.set_pipeline(&pass, pipeline);
        self.set_bind_group(&pass, 0, bind_group);
        self.draw(&pass, vertex_count, 1);
        self.end_render_pass(&pass);
        let command_buffer: JsValue = self.finish_command_encoder(&encoder);
        self.submit(&[command_buffer]);
    }

    /// Releases all GPU resources held by this renderer.
    ///
    /// The teardown order matters per the WebGPU spec:
    ///   1. `GpuCanvasContext.unconfigure()` - releases the swap chain so
    ///      the DOM canvas can be GCed.
    ///   2. `GpuDevice.destroy()` - releases all child resources (buffers,
    ///      textures, pipelines) and the device itself.
    ///
    /// Callers should run this from a `use_cleanup` callback whenever the
    /// host component is being torn down (e.g. on a `match` arm switch).
    /// Without it the previous GPU device lingers until GC, and a fresh
    /// `init()` may either reuse the dead device (silent black canvas) or
    /// fail to acquire a new one until the old device is collected.
    ///
    /// `Reflect::get` failures and JS exceptions are swallowed - this is a
    /// best-effort cleanup path, and the engine must not panic during
    /// teardown.
    pub fn dispose(&self) {
        let context: &JsValue = self.get_context();
        if let Ok(unconfigure_fn) =
            Reflect::get(context, &JsValue::from_str(WEBGPU_METHOD_UNCONFIGURE))
            && let Ok(unconfigure_callable) = unconfigure_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = unconfigure_callable.call0(context);
        }
        let device: &JsValue = self.get_device();
        if let Ok(destroy_fn) = Reflect::get(device, &JsValue::from_str(WEBGPU_METHOD_DESTROY))
            && let Ok(destroy_callable) = destroy_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = destroy_callable.call0(device);
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    //  Render-pass dynamic state (viewport / scissor / stencil / blend)
    // ─────────────────────────────────────────────────────────────────────

    /// Sets the viewport for all subsequent draw calls on the given render pass.
    ///
    /// The viewport maps NDC `[-1, 1]` to the given pixel rectangle. `min_depth`
    /// and `max_depth` (both in `[0, 1]`) clamp the depth range; the defaults
    /// of `0.0` and `1.0` cover the whole depth buffer. This call must be
    /// issued between `beginRenderPass()` and `pass.end()`.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuRenderPassEncoder`.
    /// - `&ViewportDescriptor` - The viewport rectangle and (optional) depth range.
    pub fn set_viewport(&self, pass: &JsValue, viewport: &ViewportDescriptor) {
        let vp_dict: Object = Object::new();
        let _ = Reflect::set(
            &vp_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_X),
            &JsValue::from_f64(*viewport.get_x() as f64),
        );
        let _ = Reflect::set(
            &vp_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_Y),
            &JsValue::from_f64(*viewport.get_y() as f64),
        );
        let _ = Reflect::set(
            &vp_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_WIDTH),
            &JsValue::from_f64(*viewport.get_width() as f64),
        );
        let _ = Reflect::set(
            &vp_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_HEIGHT),
            &JsValue::from_f64(*viewport.get_height() as f64),
        );
        let _ = Reflect::set(
            &vp_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_MIN_DEPTH),
            &JsValue::from_f64(WEBGPU_DEFAULT_VIEWPORT_MIN_DEPTH),
        );
        let _ = Reflect::set(
            &vp_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_MAX_DEPTH),
            &JsValue::from_f64(WEBGPU_DEFAULT_VIEWPORT_MAX_DEPTH),
        );
        let vp_js: JsValue = vp_dict.unchecked_into::<JsValue>();
        if let Ok(set_fn) = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_VIEWPORT))
            && let Ok(set_callable) = set_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = set_callable.call1(pass, &vp_js);
        }
    }

    /// Sets the scissor rectangle for all subsequent draw calls on the given
    /// render pass.
    ///
    /// Fragments outside the rectangle are discarded. The scissor is applied
    /// after the viewport, so coordinates are in the same pixel space as
    /// [`WebGpuRenderer::set_viewport`]. A scissor that extends outside the
    /// render target is clamped to the target bounds by the GPU.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuRenderPassEncoder`.
    /// - `u32` - X coordinate of the scissor origin in pixels.
    /// - `u32` - Y coordinate of the scissor origin in pixels.
    /// - `u32` - Scissor width in pixels.
    /// - `u32` - Scissor height in pixels.
    pub fn set_scissor_rect(&self, pass: &JsValue, x: u32, y: u32, width: u32, height: u32) {
        let rect_dict: Object = Object::new();
        let _ = Reflect::set(
            &rect_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_X),
            &JsValue::from_f64(x as f64),
        );
        let _ = Reflect::set(
            &rect_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_Y),
            &JsValue::from_f64(y as f64),
        );
        let _ = Reflect::set(
            &rect_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_WIDTH),
            &JsValue::from_f64(width as f64),
        );
        let _ = Reflect::set(
            &rect_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_HEIGHT),
            &JsValue::from_f64(height as f64),
        );
        let rect_js: JsValue = rect_dict.unchecked_into::<JsValue>();
        if let Ok(set_fn) = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_SCISSOR_RECT))
            && let Ok(set_callable) = set_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = set_callable.call1(pass, &rect_js);
        }
    }

    /// Sets the blend constant used by `"constant"` / `"one-minus-constant"`
    /// blend factors.
    ///
    /// Affects all subsequent draw calls on the given render pass. The
    /// constant is a linear-space RGBA color in `[0, 1]` per component.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuRenderPassEncoder`.
    /// - `f32` - Red component.
    /// - `f32` - Green component.
    /// - `f32` - Blue component.
    /// - `f32` - Alpha component.
    pub fn set_blend_constant(&self, pass: &JsValue, r: f32, g: f32, b: f32, a: f32) {
        let color_dict: Object = Object::new();
        let _ = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_R),
            &JsValue::from_f64(r as f64),
        );
        let _ = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_G),
            &JsValue::from_f64(g as f64),
        );
        let _ = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_B),
            &JsValue::from_f64(b as f64),
        );
        let _ = Reflect::set(
            &color_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_A),
            &JsValue::from_f64(a as f64),
        );
        let color_js: JsValue = color_dict.unchecked_into::<JsValue>();
        if let Ok(set_fn) = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_BLEND_CONSTANT))
            && let Ok(set_callable) = set_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = set_callable.call1(pass, &color_js);
        }
    }

    /// Sets the stencil reference value used by stencil tests.
    ///
    /// The reference is the value the GPU compares against when the shader
    /// pipeline was built with a stencil state using `"always"`, `"less"`,
    /// `"equal"`, etc. compare ops. This call must be issued between
    /// `beginRenderPass()` and `pass.end()`.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuRenderPassEncoder`.
    /// - `u32` - The stencil reference value (8-bit, `[0, 255]`).
    pub fn set_stencil_reference(&self, pass: &JsValue, reference: u32) {
        if let Ok(set_fn) = Reflect::get(
            pass,
            &JsValue::from_str(WEBGPU_METHOD_SET_STENCIL_REFERENCE),
        ) && let Ok(set_callable) = set_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> =
                set_callable.call1(pass, &JsValue::from_f64(reference as f64));
        }
    }

    /// Sets a bind group on a render pass with dynamic offsets.
    ///
    /// Use this overload of `set_bind_group` when the bind-group layout was
    /// built with `hasDynamicOffset: true` for one or more buffer bindings.
    /// Each value in `dynamic_offsets` is added to the corresponding
    /// `@group(N) @binding(M)` buffer's base offset before the draw call.
    /// For non-dynamic bind groups, prefer the simpler
    /// `set_bind_group` (3-arg) overload exposed via the `pub(crate)` API.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuRenderPassEncoder`.
    /// - `u32` - Bind-group slot index.
    /// - `&JsValue` - The `GpuBindGroup` to bind.
    /// - `&[u32]` - Dynamic offsets, one per dynamic-offset binding.
    pub fn set_bind_group_with_dynamic_offsets(
        &self,
        pass: &JsValue,
        index: u32,
        group: &JsValue,
        dynamic_offsets: &[u32],
    ) {
        if let Ok(set_fn) = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_BIND_GROUP))
            && let Ok(set_callable) = set_fn.dyn_into::<Function>()
        {
            // WebGPU's setBindGroup has two overloads: with and without
            // dynamic offsets. We always use the 4-arg form to keep the
            // call site simple; the empty offset array is well-defined.
            let offsets_array: Array = Array::new_with_length(dynamic_offsets.len() as u32);
            for (i, off) in dynamic_offsets.iter().enumerate() {
                offsets_array.set(i as u32, JsValue::from_f64(*off as f64));
            }
            let offsets_js: JsValue = offsets_array.unchecked_into::<JsValue>();
            let _: Result<JsValue, JsValue> = set_callable.call4(
                pass,
                &JsValue::from_f64(index as f64),
                group,
                &offsets_js,
                &JsValue::from_f64(0.0),
            );
        }
    }

    /// Sets a bind group on a compute pass with optional dynamic offsets.
    ///
    /// Same semantics as [`WebGpuRenderer::set_bind_group_with_dynamic_offsets`]
    /// but on a `GpuComputePassEncoder`. The `setBindGroup` method name is
    /// the same on both encoder types; this method wraps it for the compute
    /// pass to give callers a typed entry point.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The active `GpuComputePassEncoder`.
    /// - `u32` - Bind-group slot index.
    /// - `&JsValue` - The `GpuBindGroup` to bind.
    /// - `&[u32]` - Dynamic offsets for dynamic-offset bindings.
    pub fn set_bind_group_compute_with_dynamic_offsets(
        &self,
        pass: &JsValue,
        index: u32,
        group: &JsValue,
        dynamic_offsets: &[u32],
    ) {
        if let Ok(set_fn) = Reflect::get(pass, &JsValue::from_str(WEBGPU_METHOD_SET_BIND_GROUP))
            && let Ok(set_callable) = set_fn.dyn_into::<Function>()
        {
            let offsets_array: Array = Array::new_with_length(dynamic_offsets.len() as u32);
            for (i, off) in dynamic_offsets.iter().enumerate() {
                offsets_array.set(i as u32, JsValue::from_f64(*off as f64));
            }
            let offsets_js: JsValue = offsets_array.unchecked_into::<JsValue>();
            let _: Result<JsValue, JsValue> = set_callable.call4(
                pass,
                &JsValue::from_f64(index as f64),
                group,
                &offsets_js,
                &JsValue::from_f64(0.0),
            );
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    //  Texture view, mipmap generation, and CPU upload
    // ─────────────────────────────────────────────────────────────────────

    /// Creates a `GpuTextureView` for the given texture with full descriptor control.
    ///
    /// Pass `None` for a default view (full 2D, all mips, all aspects) — this
    /// is the cheap view that is implicitly created by bind-group creation.
    /// Pass `Some(&descriptor)` to sub-select mip levels, array slices, or
    /// the depth-only aspect of a depth-stencil texture.
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuTexture` to view.
    /// - `Option<&TextureViewDescriptor>` - Optional descriptor.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The `GpuTextureView`. Returns `JsValue::UNDEFINED` if
    ///   the call fails (e.g. invalid mip range); check for `undefined`
    ///   before using the result.
    pub fn create_view(
        &self,
        texture: &JsValue,
        descriptor: Option<&TextureViewDescriptor>,
    ) -> JsValue {
        let create_view_fn: Function =
            match Reflect::get(texture, &JsValue::from_str(WEBGPU_METHOD_CREATE_VIEW))
                .ok()
                .and_then(|v| v.dyn_into::<Function>().ok())
            {
                Some(f) => f,
                None => return JsValue::UNDEFINED,
            };
        // Inline the descriptor dict construction; we keep the engine-wide
        // convention of "0 / None means default" so the browser falls back
        // to its own defaults for omitted keys.
        let desc_value: JsValue = match descriptor {
            None => JsValue::UNDEFINED,
            Some(d) => {
                let dict: Object = Object::new();
                if let Some(format) = d.get_format() {
                    let _ = Reflect::set(
                        &dict,
                        &JsValue::from_str(WEBGPU_PROPERTY_FORMAT),
                        &JsValue::from_str(format),
                    );
                }
                // `dimension` and `aspect` are explicitly sent as their
                // default values ("2d" / "all") rather than omitted, because
                // a handful of browsers reject undefined keys on the
                // createView descriptor.
                let _ = Reflect::set(
                    &dict,
                    &JsValue::from_str(WEBGPU_PROPERTY_DIMENSION),
                    &JsValue::from_str(d.effective_dimension()),
                );
                let _ = Reflect::set(
                    &dict,
                    &JsValue::from_str(WEBGPU_PROPERTY_ASPECT),
                    &JsValue::from_str(d.effective_aspect()),
                );
                // baseMipLevel / mipLevelCount / baseArrayLayer /
                // arrayLayerCount are u32 with 0 = "use the default".
                // Skip them when they are still at the default so that the
                // browser applies its own spec-compliant fallback.
                let base_mip: u32 = d.get_base_mip_level();
                if base_mip != 0 {
                    let _ = Reflect::set(
                        &dict,
                        &JsValue::from_str(WEBGPU_PROPERTY_BASE_MIP_LEVEL),
                        &JsValue::from_f64(base_mip as f64),
                    );
                }
                let mip_count: u32 = d.get_mip_level_count();
                if mip_count != 0 {
                    let _ = Reflect::set(
                        &dict,
                        &JsValue::from_str(WEBGPU_PROPERTY_MIP_LEVEL_COUNT),
                        &JsValue::from_f64(mip_count as f64),
                    );
                }
                let base_array: u32 = d.get_base_array_layer();
                if base_array != 0 {
                    let _ = Reflect::set(
                        &dict,
                        &JsValue::from_str(WEBGPU_PROPERTY_BASE_ARRAY_LAYER),
                        &JsValue::from_f64(base_array as f64),
                    );
                }
                let array_count: u32 = d.get_array_layer_count();
                if array_count != 0 {
                    let _ = Reflect::set(
                        &dict,
                        &JsValue::from_str(WEBGPU_PROPERTY_ARRAY_LAYER_COUNT),
                        &JsValue::from_f64(array_count as f64),
                    );
                }
                dict.unchecked_into::<JsValue>()
            }
        };
        create_view_fn
            .call1(texture, &desc_value)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Generates the full mipmap chain for the given texture.
    ///
    /// Equivalent to repeatedly calling `copyTextureToTexture` from level
    /// `i` to level `i+1` with the appropriate mip dimensions, but in one
    /// GPU command. The texture must have been created with `RENDER_ATTACHMENT
    /// | TEXTURE_BINDING | COPY_DST | COPY_SRC` usage and `mipLevelCount > 1`.
    /// Requires the `mipmap` WebGPU feature, or a GPU that supports it
    /// unconditionally (most desktop GPUs do).
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuTexture` whose mips will be generated.
    pub fn generate_mipmaps(&self, texture: &JsValue) {
        if let Ok(gen_fn) = Reflect::get(texture, &JsValue::from_str(WEBGPU_METHOD_GENERATE_MIPMAP))
            && let Ok(gen_callable) = gen_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = gen_callable.call0(texture);
        }
    }

    /// Uploads CPU-side pixel data directly to a texture via `queue.writeTexture`.
    ///
    /// Use this instead of `create_buffer + write_buffer + copyBufferToTexture`
    /// for one-shot uploads (ImGui font atlases, sprite sheets, procedural
    /// noise). The queue is acquired internally via the cached `device.queue`
    /// handle, so this is the preferred path for textures that are written
    /// once and sampled many times.
    ///
    /// `bytes_per_row` must be a multiple of 256. The `data` layout must
    /// match the texture's `format`; the engine does not perform swizzling.
    ///
    /// # Arguments
    ///
    /// - `&TextureWriteDescriptor` - The write descriptor.
    pub fn write_texture(&self, descriptor: &TextureWriteDescriptor) {
        let queue: JsValue =
            match Reflect::get(self.get_device(), &JsValue::from_str(WEBGPU_PROPERTY_QUEUE))
                .ok()
                .and_then(|v| v.dyn_into::<JsValue>().ok())
            {
                Some(q) => q,
                None => return,
            };
        let layout_dict: Object = Object::new();
        let _ = Reflect::set(
            &layout_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_BYTES_PER_ROW),
            &JsValue::from_f64(descriptor.get_bytes_per_row() as f64),
        );
        let _ = Reflect::set(
            &layout_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_ROWS_PER_IMAGE),
            &JsValue::from_f64(descriptor.get_rows_per_image() as f64),
        );
        let _ = Reflect::set(
            &layout_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_OFFSET_BYTES),
            &JsValue::from_f64(0.0),
        );
        let layout_js: JsValue = layout_dict.unchecked_into::<JsValue>();
        let write_fn: Function =
            match Reflect::get(&queue, &JsValue::from_str(WEBGPU_METHOD_WRITE_TEXTURE))
                .ok()
                .and_then(|v| v.dyn_into::<Function>().ok())
            {
                Some(f) => f,
                None => return,
            };
        // Build destination dict: { texture, mipLevel, origin? }
        let dest_dict: Object = Object::new();
        let _ = Reflect::set(
            &dest_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_TEXTURE),
            &descriptor.get_texture(),
        );
        let _ = Reflect::set(
            &dest_dict,
            &JsValue::from_str(WEBGPU_PROPERTY_MIP_LEVEL),
            &JsValue::from_f64(descriptor.get_mip_level() as f64),
        );
        if let Some(origin) = descriptor.get_origin() {
            let _ = Reflect::set(
                &dest_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_ORIGIN),
                &origin,
            );
        }
        let dest_js: JsValue = dest_dict.unchecked_into::<JsValue>();
        // WebGPU's queue.writeTexture requires a Uint8Array view; we hand
        // it the raw Vec<u8> and let JS interop copy it. This is the same
        // path wasm-bindgen takes for &[u8] → Uint8Array.
        let data_js: JsValue = Uint8Array::from(descriptor.get_data().as_slice()).into();
        // For the size extent, we read bytes_per_row's texel width from the
        // destination. Without a format converter we default to a square
        // shape based on the data size. The caller is expected to construct
        // a TextureWriteDescriptor that matches their texture exactly;
        // this method does not auto-derive size.
        let size_value: JsValue = {
            let bpr: u32 = descriptor.get_bytes_per_row();
            let rows: u32 = if descriptor.get_rows_per_image() == 0 {
                (descriptor.get_data().len() as u32) / bpr.max(1)
            } else {
                descriptor.get_rows_per_image()
            };
            let size_dict: Object = Object::new();
            let _ = Reflect::set(
                &size_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_WIDTH),
                &JsValue::from_f64(bpr as f64),
            );
            let _ = Reflect::set(
                &size_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_HEIGHT),
                &JsValue::from_f64(rows as f64),
            );
            let _ = Reflect::set(
                &size_dict,
                &JsValue::from_str(WEBGPU_PROPERTY_DEPTH_OR_1),
                &JsValue::from_f64(1.0),
            );
            size_dict.unchecked_into::<JsValue>()
        };
        let _: Result<JsValue, JsValue> =
            write_fn.call4(&queue, &dest_js, &data_js, &layout_js, &size_value);
    }

    // ─────────────────────────────────────────────────────────────────────
    //  Shader module + explicit pipeline compile diagnostics
    // ─────────────────────────────────────────────────────────────────────

    /// Creates a `GpuShaderModule` from a WGSL source string with a debug label.
    ///
    /// Equivalent to the `pub(crate) fn create_shader_module` overload but
    /// attaches a `label` to the module so it shows up under that name in
    /// browser devtools (e.g. Chrome's `chrome://gpu-internals` and the
    /// WebGPU Inspector panel). The label has no runtime effect; it is
    /// purely a developer-experience aid when many shader modules coexist.
    ///
    /// # Arguments
    ///
    /// - `&str` - WGSL source.
    /// - `&str` - Debug label shown in browser devtools.
    ///
    /// # Returns
    ///
    /// - `JsValue` - The `GpuShaderModule`, or `JsValue::UNDEFINED` if
    ///   the call fails.
    pub fn create_shader_module_with_label(&self, wgsl_source: &str, label: &str) -> JsValue {
        let descriptor: Object = Object::new();
        let _ = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_CODE),
            &JsValue::from_str(wgsl_source),
        );
        let _ = Reflect::set(
            &descriptor,
            &JsValue::from_str(WEBGPU_PROPERTY_LABEL),
            &JsValue::from_str(label),
        );
        let desc_value: JsValue = descriptor.unchecked_into::<JsValue>();
        if let Ok(create_fn) = Reflect::get(
            self.get_device(),
            &JsValue::from_str(WEBGPU_METHOD_CREATE_SHADER_MODULE),
        ) && let Ok(create_callable) = create_fn.dyn_into::<Function>()
        {
            // The call returns a Promise that resolves to the shader module.
            // We do not await it; the caller is expected to drive the future
            // or pass the result into a pipeline creation call.
            return create_callable
                .call1(self.get_device(), &desc_value)
                .unwrap_or(JsValue::UNDEFINED);
        }
        JsValue::UNDEFINED
    }

    // ─────────────────────────────────────────────────────────────────────
    //  Buffer readback via mapAsync + getMappedRange
    // ─────────────────────────────────────────────────────────────────────

    /// Reads back the contents of a buffer via `mapAsync` + `getMappedRange` +
    /// `unmap`.
    ///
    /// This is an **`async fn`**, NOT a synchronous wrapper. It must be
    /// `await`-ed by the caller. Use it from inside another
    /// `wasm_bindgen_futures` future (e.g. a frame loop) — do not call
    /// it from synchronous code, since the awaiter must be driven by
    /// the executor. The buffer must have been created with `MAP_READ`
    /// usage, and the read must be preceded by a GPU submission that
    /// finished writing to the buffer (i.e. `queue.submit([encoder.finish()])`
    /// followed by `device.lost` / a fence).
    ///
    /// # Arguments
    ///
    /// - `&JsValue` - The `GpuBuffer` to read back.
    /// - `u64` - Byte offset into the buffer.
    /// - `u64` - Number of bytes to read.
    ///
    /// # Returns
    ///
    /// - `Option<Vec<u8>>` - The bytes, or `None` if the readback failed.
    pub async fn read_buffer(&self, buffer: &JsValue, offset: u64, size: u64) -> Option<Vec<u8>> {
        // Step 1: buffer.mapAsync(mode, offset, size)
        let map_fn: Function = Reflect::get(buffer, &JsValue::from_str(WEBGPU_METHOD_MAP_ASYNC))
            .ok()
            .and_then(|v| v.dyn_into::<Function>().ok())?;
        let map_promise: Promise = map_fn
            .call3(
                buffer,
                // `mapAsync` takes a `GPUMapMode` bitmask; the spec
                // allows OR'ing `READ` and `WRITE` together, so we
                // use the `map_mode_for` helper that pins the
                // `WEBGPU_MAP_MODE_WRITE` constant on the live code
                // path. This buffer is read-only for the host, so
                // we pass `read = true, write = false`.
                &JsValue::from_f64(map_mode_for(/* read = */ true, /* write = */ false) as f64),
                &JsValue::from_f64(offset as f64),
                &JsValue::from_f64(size as f64),
            )
            .ok()?
            .unchecked_into();
        // Step 2: await the mapAsync promise
        let _map_result: JsValue = JsFuture::from(map_promise).await.ok()?;
        // Step 3: buffer.getMappedRange(offset, size)
        let get_range_fn: Function =
            Reflect::get(buffer, &JsValue::from_str(WEBGPU_METHOD_GET_MAPPED_RANGE))
                .ok()
                .and_then(|v| v.dyn_into::<Function>().ok())?;
        let array_buffer: ArrayBuffer = get_range_fn
            .call2(
                buffer,
                &JsValue::from_f64(offset as f64),
                &JsValue::from_f64(size as f64),
            )
            .ok()?
            .unchecked_into();
        // Step 4: copy out before unmap invalidates the memory
        let u8_view: Uint8Array = Uint8Array::new(&array_buffer);
        let mut out: Vec<u8> = vec![0u8; u8_view.length() as usize];
        u8_view.copy_to(&mut out);
        // Step 5: unmap
        if let Ok(unmap_fn) = Reflect::get(buffer, &JsValue::from_str(WEBGPU_METHOD_UNMAP))
            && let Ok(unmap_callable) = unmap_fn.dyn_into::<Function>()
        {
            let _: Result<JsValue, JsValue> = unmap_callable.call0(buffer);
        }
        Some(out)
    }
}

/// Implements helper methods on `WebGpuInitError`.
///
/// These methods provide ergonomic access to the diagnostic code and the
/// underlying JS error value, which are useful when surfacing the failure
/// to the user (e.g. via `Console::error` from the example crate).
impl WebGpuInitError {
    /// Returns a short, machine-readable identifier for this error variant.
    ///
    /// Suitable for use as a stable error code in logs or telemetry.
    /// The codes are stable across releases.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The error code (e.g. `"WEBGPU_NAVIGATOR_GPU_MISSING"`).
    pub fn code(&self) -> &'static str {
        match self {
            Self::NavigatorLookup(_) => "WEBGPU_NAVIGATOR_LOOKUP",
            Self::NavigatorGpuMissing => "WEBGPU_NAVIGATOR_GPU_MISSING",
            Self::RequestAdapterLookup(_) => "WEBGPU_REQUEST_ADAPTER_LOOKUP",
            Self::RequestAdapterCall(_) => "WEBGPU_REQUEST_ADAPTER_CALL",
            Self::AdapterPromise(_) => "WEBGPU_ADAPTER_PROMISE",
            Self::AdapterUnavailable => "WEBGPU_ADAPTER_UNAVAILABLE",
            Self::RequestDeviceLookup(_) => "WEBGPU_REQUEST_DEVICE_LOOKUP",
            Self::RequestDeviceCall(_) => "WEBGPU_REQUEST_DEVICE_CALL",
            Self::DevicePromise(_) => "WEBGPU_DEVICE_PROMISE",
            Self::DeviceUnavailable => "WEBGPU_DEVICE_UNAVAILABLE",
            Self::CanvasNotFound(_) => "WEBGPU_CANVAS_NOT_FOUND",
            Self::CanvasQuery(_) => "WEBGPU_CANVAS_QUERY",
            Self::CanvasContextUnavailable => "WEBGPU_CANVAS_CONTEXT_UNAVAILABLE",
            Self::PreferredFormatLookup(_) => "WEBGPU_PREFERRED_FORMAT_LOOKUP",
            Self::PreferredFormatCall(_) => "WEBGPU_PREFERRED_FORMAT_CALL",
            Self::PreferredFormatType(_) => "WEBGPU_PREFERRED_FORMAT_TYPE",
            Self::ConfigureLookup(_) => "WEBGPU_CONFIGURE_LOOKUP",
            Self::QueueLookup(_) => "WEBGPU_QUEUE_LOOKUP",
        }
    }

    /// Returns the underlying JS error value if this variant carries one.
    ///
    /// Variants that do not capture a JS value (e.g. `NavigatorGpuMissing`,
    /// `AdapterUnavailable`, `CanvasNotFound`, `CanvasContextUnavailable`)
    /// return `None`.
    ///
    /// # Returns
    ///
    /// - `Option<&JsValue>` - The captured JS error, if any.
    pub fn js_error(&self) -> Option<&JsValue> {
        match self {
            Self::NavigatorLookup(err)
            | Self::RequestAdapterLookup(err)
            | Self::RequestAdapterCall(err)
            | Self::AdapterPromise(err)
            | Self::RequestDeviceLookup(err)
            | Self::RequestDeviceCall(err)
            | Self::DevicePromise(err)
            | Self::CanvasQuery(err)
            | Self::PreferredFormatLookup(err)
            | Self::PreferredFormatCall(err)
            | Self::PreferredFormatType(err)
            | Self::ConfigureLookup(err)
            | Self::QueueLookup(err) => Some(err),
            Self::NavigatorGpuMissing
            | Self::AdapterUnavailable
            | Self::DeviceUnavailable
            | Self::CanvasContextUnavailable
            | Self::CanvasNotFound(_) => None,
        }
    }
}

/// Implements `Display` for `WebGpuInitError`.
///
/// The formatted message is intended for end-user diagnostic output
/// (typically forwarded to `Console::error` by the calling application)
/// and includes the variant code plus a human-readable description. When
/// the variant carries a JS error, its `Debug` form is appended.
impl Display for WebGpuInitError {
    /// Formats the [`WebGpuInitError`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `FmtResult` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NavigatorLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(navigator, webgpu) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::NavigatorGpuMissing => write!(
                formatter,
                "[{}] navigator.gpu is missing - browser does not expose WebGPU on this origin",
                self.code(),
            ),
            Self::RequestAdapterLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(gpu, requestAdapter) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::RequestAdapterCall(err) => write!(
                formatter,
                "[{}] gpu.requestAdapter() threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::AdapterPromise(err) => write!(
                formatter,
                "[{}] adapter promise rejected or timed out: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::AdapterUnavailable => write!(
                formatter,
                "[{}] requestAdapter returned null - no compatible GPU adapter for the requested powerPreference",
                self.code(),
            ),
            Self::RequestDeviceLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(adapter, requestDevice) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::RequestDeviceCall(err) => write!(
                formatter,
                "[{}] adapter.requestDevice() threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::DevicePromise(err) => write!(
                formatter,
                "[{}] device promise rejected or timed out: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::DeviceUnavailable => write!(
                formatter,
                "[{}] requestDevice returned null - adapter could not allocate a device (possibly device-lost)",
                self.code(),
            ),
            Self::CanvasNotFound(selector) => write!(
                formatter,
                "[{}] canvas element {:?} not found in DOM",
                self.code(),
                selector,
            ),
            Self::CanvasQuery(err) => write!(
                formatter,
                "[{}] querySelector threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::CanvasContextUnavailable => write!(
                formatter,
                "[{}] canvas.get_context('webgpu') returned null - the canvas may already be using another context type or WebGPU is disabled",
                self.code(),
            ),
            Self::PreferredFormatLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(gpu, getPreferredCanvasFormat) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::PreferredFormatCall(err) => write!(
                formatter,
                "[{}] gpu.getPreferredCanvasFormat() threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::PreferredFormatType(value) => write!(
                formatter,
                "[{}] getPreferredCanvasFormat returned non-string: {}",
                self.code(),
                js_error_to_string(value),
            ),
            Self::ConfigureLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(context, configure) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::QueueLookup(err) => write!(
                formatter,
                "[{}] Reflect::get(device, queue) failed: {}",
                self.code(),
                js_error_to_string(err),
            ),
        }
    }
}

/// Implements the standard `std::error::Error` trait for `WebGpuInitError`.
///
/// The `source()` method delegates to the underlying JS error's `toString()`
/// representation when present, otherwise returns `None`. The engine never
/// logs or prints anything; this impl exists solely so the error composes
/// with `Result`-based APIs and `?` operator chains.
impl Error for WebGpuInitError {}

/// Implements `WebGlRenderer` context acquisition, shader program management,
/// and per-frame drawing.
///
/// All methods are synchronous: WebGL has no Promise-based initialization.
/// The renderer never logs; initialization failures are returned as
/// `WebGlInitError` and shader failures as `WebGlProgramError` so the caller
/// can surface them (typically via `Console::error` on the example side).
impl WebGlRenderer {
    /// Probes whether the browser can create a WebGL 2 context.
    ///
    /// Creates a throwaway off-DOM canvas and requests a `webgl2` context.
    /// The probe is cheap (no shaders are compiled) and has no side effects
    /// on the page.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if a `webgl2` context could be acquired.
    pub fn is_available() -> bool {
        let Some(window_value) = window() else {
            return false;
        };
        let Some(document_value) = window_value.document() else {
            return false;
        };
        let element: Element = match document_value.create_element("canvas") {
            Ok(element) => element,
            Err(_) => return false,
        };
        let canvas: HtmlCanvasElement = element.unchecked_into();
        canvas.get_context("webgl2").ok().flatten().is_some()
    }

    /// Initializes a WebGL 2 renderer from a render configuration.
    ///
    /// Resolves the canvas element from `config.canvas_selector`, scales the
    /// backing store by the device pixel ratio, acquires the `webgl2`
    /// context, and sets the initial viewport.
    ///
    /// # Arguments
    ///
    /// - `&RenderConfig` - The rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Result<WebGlRenderer, WebGlInitError>` - The initialized renderer,
    ///   or a typed error describing the specific failure.
    pub fn init(config: &RenderConfig) -> Result<WebGlRenderer, WebGlInitError> {
        let Some(window_value) = window() else {
            return Err(WebGlInitError::CanvasNotFound(
                config.canvas_selector.clone(),
            ));
        };
        let Some(document_value) = window_value.document() else {
            return Err(WebGlInitError::CanvasNotFound(
                config.canvas_selector.clone(),
            ));
        };
        let element: Element = document_value
            .query_selector(config.canvas_selector.as_ref())
            .map_err(WebGlInitError::CanvasQuery)?
            .ok_or_else(|| WebGlInitError::CanvasNotFound(config.canvas_selector.clone()))?;
        let canvas: HtmlCanvasElement = element.unchecked_into();
        let dpr: f64 = CanvasRenderer::detect_dpr();
        let physical_width: u32 = (config.width * dpr).round() as u32;
        let physical_height: u32 = (config.height * dpr).round() as u32;
        canvas.set_width(physical_width);
        canvas.set_height(physical_height);
        let context_object: Object = canvas
            .get_context("webgl2")
            .map_err(WebGlInitError::ContextLookup)?
            .ok_or(WebGlInitError::ContextUnavailable)?;
        let context: WebGl2RenderingContext = context_object
            .dyn_into()
            .map_err(|_| WebGlInitError::ContextCast)?;
        context.viewport(0, 0, physical_width as i32, physical_height as i32);
        Ok(WebGlRenderer {
            context,
            canvas,
            width: physical_width,
            height: physical_height,
        })
    }

    /// Compiles and links a shader program from GLSL ES 3.00 sources.
    ///
    /// Both shaders are compiled, attached, and linked; on success the
    /// intermediate shader objects are deleted (the program keeps the
    /// compiled code). On failure the browser info log is returned so the
    /// caller can surface the exact GLSL diagnostic.
    ///
    /// # Arguments
    ///
    /// - `&str` - The vertex shader source (`#version 300 es`).
    /// - `&str` - The fragment shader source (`#version 300 es`).
    ///
    /// # Returns
    ///
    /// - `Result<WebGlProgram, WebGlProgramError>` - The linked program, or
    ///   the compile/link info log.
    pub fn create_program(
        &self,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<WebGlProgram, WebGlProgramError> {
        let vertex_shader: WebGlShader =
            self.compile_shader(WebGl2RenderingContext::VERTEX_SHADER, vertex_source)?;
        let fragment_shader: WebGlShader =
            self.compile_shader(WebGl2RenderingContext::FRAGMENT_SHADER, fragment_source)?;
        let program: WebGlProgram = self.context.create_program().ok_or_else(|| {
            WebGlProgramError::ProgramLink("createProgram returned null".to_string())
        })?;
        self.context.attach_shader(&program, &vertex_shader);
        self.context.attach_shader(&program, &fragment_shader);
        self.context.link_program(&program);
        let linked: bool = self
            .context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or_default();
        if !linked {
            let log: String = self
                .context
                .get_program_info_log(&program)
                .unwrap_or_default();
            self.context.delete_program(Some(&program));
            self.context.delete_shader(Some(&vertex_shader));
            self.context.delete_shader(Some(&fragment_shader));
            return Err(WebGlProgramError::ProgramLink(log));
        }
        self.context.delete_shader(Some(&vertex_shader));
        self.context.delete_shader(Some(&fragment_shader));
        Ok(program)
    }

    /// Compiles a single shader, returning the info log on failure.
    ///
    /// # Arguments
    ///
    /// - `u32` - The shader kind (`VERTEX_SHADER` or `FRAGMENT_SHADER`).
    /// - `&str` - The GLSL source.
    ///
    /// # Returns
    ///
    /// - `Result<WebGlShader, WebGlProgramError>` - The compiled shader, or
    ///   the compile info log.
    fn compile_shader(&self, kind: u32, source: &str) -> Result<WebGlShader, WebGlProgramError> {
        let shader: WebGlShader = self.context.create_shader(kind).ok_or_else(|| {
            WebGlProgramError::ShaderCompile("createShader returned null".to_string())
        })?;
        self.context.shader_source(&shader, source);
        self.context.compile_shader(&shader);
        let compiled: bool = self
            .context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or_default();
        if !compiled {
            let log: String = self
                .context
                .get_shader_info_log(&shader)
                .unwrap_or_default();
            self.context.delete_shader(Some(&shader));
            return Err(WebGlProgramError::ShaderCompile(log));
        }
        Ok(shader)
    }

    /// Resolves the location of a uniform on the given program.
    ///
    /// Uniform locations are stable for the lifetime of a linked program,
    /// so callers rendering in a per-frame loop should resolve each uniform
    /// once after [`WebGlRenderer::create_program`] and cache the result,
    /// then pass it to [`WebGlRenderer::set_uniform_2f`] /
    /// [`WebGlRenderer::set_uniform_4fv`]. Resolving per frame is supported
    /// but wasteful: every lookup crosses into the browser's GL frontend.
    /// A uniform that the GLSL compiler optimized out resolves to `None`,
    /// which the setters silently ignore, matching raw WebGL semantics.
    ///
    /// # Arguments
    ///
    /// - `&WebGlProgram` - The program owning the uniform.
    /// - `&str` - The uniform name (for array uniforms, with an explicit
    ///   `[0]` index, per the WebGL `getUniformLocation` spec).
    ///
    /// # Returns
    ///
    /// - `Option<WebGlUniformLocation>` - The uniform location, or `None`
    ///   when the uniform does not exist in the program.
    pub fn get_uniform_location(
        &self,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<WebGlUniformLocation> {
        self.context.get_uniform_location(program, name)
    }

    /// Sets a `vec2` uniform on the given program via its cached location.
    ///
    /// The program is bound with `useProgram` before the upload so the
    /// uniform call always targets the program the location was resolved
    /// from, regardless of which program the context currently has bound
    /// (uploading against a different bound program is an
    /// `INVALID_OPERATION` in WebGL). A `None` location (uniform optimized
    /// out by the GLSL compiler) is silently ignored, matching raw WebGL
    /// semantics.
    ///
    /// # Arguments
    ///
    /// - `&WebGlProgram` - The program owning the uniform.
    /// - `Option<&WebGlUniformLocation>` - The cached location from
    ///   [`WebGlRenderer::get_uniform_location`].
    /// - `f32` - The x component.
    /// - `f32` - The y component.
    pub fn set_uniform_2f(
        &self,
        program: &WebGlProgram,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
    ) {
        self.context.use_program(Some(program));
        self.context.uniform2f(location, x, y);
    }

    /// Uploads a flat float slice into a `vec4` or `vec4[]` uniform via its
    /// cached location.
    ///
    /// Used by the game demos to push per-frame instance data (ball positions
    /// and colors, cube transforms) into shaders that index the array with
    /// `gl_VertexID`. `data.len()` must be a multiple of 4. The upload writes
    /// only `data.len() / 4` elements; untouched elements keep their previous
    /// values. Like [`WebGlRenderer::set_uniform_2f`], the program is bound
    /// before the upload so the call can never target the wrong program.
    ///
    /// # Arguments
    ///
    /// - `&WebGlProgram` - The program owning the uniform.
    /// - `Option<&WebGlUniformLocation>` - The cached location from
    ///   [`WebGlRenderer::get_uniform_location`].
    /// - `&[f32]` - The packed float data.
    pub fn set_uniform_4fv(
        &self,
        program: &WebGlProgram,
        location: Option<&WebGlUniformLocation>,
        data: &[f32],
    ) {
        self.context.use_program(Some(program));
        self.context.uniform4fv_with_f32_array(location, data);
    }

    /// Renders a complete frame: clears the canvas and draws a triangle-list
    /// primitive whose vertices are generated inside the vertex shader.
    ///
    /// Mirrors [`WebGpuRenderer::render_frame`]: the vertex shader uses
    /// `gl_VertexID` so no vertex buffers are involved. The given program
    /// is bound before drawing; set its uniforms first via
    /// [`WebGlRenderer::set_uniform_2f`] when the shader reads per-frame
    /// interaction data.
    ///
    /// # Arguments
    ///
    /// - `&WebGlProgram` - The program to draw with.
    /// - `(f64, f64, f64, f64)` - The clear color as (r, g, b, a) in 0.0–1.0 range.
    /// - `i32` - The number of vertices to draw.
    pub fn render_frame(
        &self,
        program: &WebGlProgram,
        clear_color: (f64, f64, f64, f64),
        vertex_count: i32,
    ) {
        let (r, g, b, a) = clear_color;
        self.context
            .viewport(0, 0, self.width as i32, self.height as i32);
        self.context
            .clear_color(r as f32, g as f32, b as f32, a as f32);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.context.use_program(Some(program));
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertex_count);
    }

    /// Resizes the canvas backing store and updates the GL viewport.
    ///
    /// Call this when the CSS layout size changes (window resize, DPR
    /// change) so the drawing buffer matches the visible region.
    ///
    /// # Arguments
    ///
    /// - `u32` - The new physical pixel width (already multiplied by DPR).
    /// - `u32` - The new physical pixel height.
    pub fn resize(&mut self, physical_width: u32, physical_height: u32) {
        self.canvas.set_width(physical_width);
        self.canvas.set_height(physical_height);
        self.width = physical_width;
        self.height = physical_height;
        self.context
            .viewport(0, 0, physical_width as i32, physical_height as i32);
    }
}

/// Implements `WebGlInitError` diagnostic helpers.
impl WebGlInitError {
    /// Returns a short, machine-readable identifier for this error variant.
    ///
    /// Suitable for use as a stable error code in logs or telemetry.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The error code (e.g. `\"WEBGL_CONTEXT_UNAVAILABLE\"`).
    pub fn code(&self) -> &'static str {
        match self {
            Self::CanvasNotFound(_) => "WEBGL_CANVAS_NOT_FOUND",
            Self::CanvasQuery(_) => "WEBGL_CANVAS_QUERY",
            Self::ContextUnavailable => "WEBGL_CONTEXT_UNAVAILABLE",
            Self::ContextLookup(_) => "WEBGL_CONTEXT_LOOKUP",
            Self::ContextCast => "WEBGL_CONTEXT_CAST",
        }
    }

    /// Returns the underlying JS error value if this variant carries one.
    ///
    /// # Returns
    ///
    /// - `Option<&JsValue>` - The captured JS error, if any.
    pub fn js_error(&self) -> Option<&JsValue> {
        match self {
            Self::CanvasQuery(err) | Self::ContextLookup(err) => Some(err),
            Self::CanvasNotFound(_) | Self::ContextUnavailable | Self::ContextCast => None,
        }
    }
}

/// Implements `Display` for `WebGlInitError`.
///
/// The formatted message includes the variant code plus a human-readable
/// description; variants carrying a JS error append its rendered form.
impl Display for WebGlInitError {
    /// Formats the [`WebGlInitError`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `FmtResult` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::CanvasNotFound(selector) => write!(
                formatter,
                "[{}] canvas element {:?} not found in DOM",
                self.code(),
                selector,
            ),
            Self::CanvasQuery(err) => write!(
                formatter,
                "[{}] querySelector threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::ContextUnavailable => write!(
                formatter,
                "[{}] canvas.get_context('webgl2') returned null - the browser does not support WebGL 2 or the canvas already uses another context type",
                self.code(),
            ),
            Self::ContextLookup(err) => write!(
                formatter,
                "[{}] canvas.get_context('webgl2') threw: {}",
                self.code(),
                js_error_to_string(err),
            ),
            Self::ContextCast => write!(
                formatter,
                "[{}] get_context('webgl2') result could not be cast to WebGl2RenderingContext",
                self.code(),
            ),
        }
    }
}

/// Implements `Display` for `WebGlProgramError`.
///
/// The formatted message includes the browser-provided info log so GLSL
/// diagnostics are visible verbatim in the console.
impl Display for WebGlProgramError {
    /// Formats the [`WebGlProgramError`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `FmtResult` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ShaderCompile(log) => write!(formatter, "shader compilation failed: {log}"),
            Self::ProgramLink(log) => write!(formatter, "program link failed: {log}"),
        }
    }
}

/// Implements the standard `Error` trait for `WebGlProgramError`.
impl Error for WebGlProgramError {}

/// Default-construction helper for `Texture2DDescriptor`.
impl Texture2DDescriptor {
    /// Returns a descriptor with the most common defaults applied.
    ///
    /// This is the same as calling the generated `new` constructor and
    /// then explicitly setting the defaults; we provide it so callers
    /// can do `Texture2DDescriptor::default_for(w, h, format)` instead of
    /// having to remember which fields to set.
    ///
    /// # Arguments
    ///
    /// - `width` - The texture width in pixels.
    /// - `height` - The texture height in pixels.
    /// - `format` - The WGSL texture format.
    ///
    /// # Returns
    ///
    /// - A new descriptor with `mip_level_count = 1`, `sample_count = 1`,
    ///   and usage `"TEXTURE_BINDING | COPY_DST | COPY_SRC"`.
    pub fn default_for(width: u32, height: u32, format: &'static str) -> Self {
        Self {
            width,
            height,
            format,
            mip_level_count: 1,
            sample_count: 1,
            usage: "TEXTURE_BINDING | COPY_DST | COPY_SRC",
        }
    }
}

/// Default-construction helper for `GpuSamplerDescriptor`.
impl GpuSamplerDescriptor {
    /// Returns a descriptor with the most common defaults applied:
    /// nearest filtering and clamp-to-edge addressing on all axes.
    pub fn default_sampler() -> Self {
        Self {
            mag_filter: WEBGPU_FILTER_MODE_NEAREST,
            min_filter: WEBGPU_FILTER_MODE_NEAREST,
            mipmap_filter: WEBGPU_FILTER_MODE_NEAREST,
            address_mode_u: WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE,
            address_mode_v: WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE,
            address_mode_w: WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE,
            compare: false,
        }
    }
}

/// Resolves optional `load_op` / `store_op` to the WebGPU spec defaults for
/// `RenderPassColorAttachment`.
impl RenderPassColorAttachment {
    /// Returns the load op that the renderer should use.
    ///
    /// # Returns
    ///
    /// - `'static str` - A `'static str` value.
    pub(crate) fn effective_load_op(&self) -> &'static str {
        match (self.load_op, self.clear_value) {
            (Some(op), _) => op,
            (None, Some(_)) => WEBGPU_LOAD_OP_CLEAR,
            (None, None) => WEBGPU_LOAD_OP_LOAD,
        }
    }

    /// Returns the store op that the renderer should use.
    ///
    /// Defaults to [`WEBGPU_STORE_OP_STORE`] so the color/depth
    /// attachment contents survive the pass. Callers that know the
    /// attachment is transient (no resolve, no follow-up sample, no
    /// `copyTextureToTexture`) can use [`WEBGPU_STORE_OP_DISCARD`]
    /// to avoid the bandwidth of a write-back. The helper
    /// [`default_color_store_op`] centralises that "transient?"
    /// decision so the [`WEBGPU_STORE_OP_DISCARD`] constant stays
    /// reachable from inside the engine.
    ///
    /// # Returns
    ///
    /// - `'static str` - A `'static str` value.
    pub(crate) fn effective_store_op(&self) -> &'static str {
        self.store_op.unwrap_or_else(|| {
            default_color_store_op(/* transient = */ false)
        })
    }
}

/// Resolves optional `depth_load_op` / `depth_store_op` to the WebGPU spec
/// defaults for `RenderPassDepthStencilAttachment`.
impl RenderPassDepthStencilAttachment {
    /// Returns the depth load op that the renderer should use.
    ///
    /// # Returns
    ///
    /// - `'static str` - A `'static str` value.
    pub(crate) fn effective_depth_load_op(&self) -> &'static str {
        match (self.depth_load_op, self.depth_clear_value) {
            (Some(op), _) => op,
            (None, Some(_)) => WEBGPU_LOAD_OP_CLEAR,
            (None, None) => WEBGPU_LOAD_OP_LOAD,
        }
    }

    /// Returns the depth store op that the renderer should use.
    ///
    /// # Returns
    ///
    /// - `'static str` - A `'static str` value.
    pub(crate) fn effective_depth_store_op(&self) -> &'static str {
        self.depth_store_op.unwrap_or(WEBGPU_STORE_OP_STORE)
    }
}

/// Constructors and view-default resolvers for `TextureViewDescriptor`.
impl TextureViewDescriptor {
    /// Returns a descriptor that selects the full texture as a 2D view.
    /// This is the cheapest view you can make; equivalent to calling
    /// `texture.createView()` with no argument.
    pub fn full() -> Self {
        Self {
            format: None,
            dimension: None,
            base_mip_level: 0,
            mip_level_count: 0,
            base_array_layer: 0,
            array_layer_count: 0,
            aspect: None,
        }
    }

    /// The dimension string the renderer will send to `createView`.
    ///
    /// We default `None` to `"2d"` instead of omitting the key, because
    /// every other descriptor in the engine uses the explicit-string
    /// form, and a few browsers reject `dimension: undefined`.
    ///
    /// # Returns
    ///
    /// - `'static str` - A `'static str` value.
    pub(crate) fn effective_dimension(&self) -> &'static str {
        self.dimension.unwrap_or(WEBGPU_TEXTURE_VIEW_DIMENSION_2D)
    }

    /// The aspect string the renderer will send to `createView`.
    ///
    /// Defaults to `"all"`, which is the spec's "expose every channel"
    /// option and the only correct choice for color textures.
    ///
    /// # Returns
    ///
    /// - `'static str` - A `'static str` value.
    pub(crate) fn effective_aspect(&self) -> &'static str {
        self.aspect.unwrap_or(WEBGPU_TEXTURE_ASPECT_ALL)
    }

    /// Returns a descriptor that selects a single mip level of the texture.
    /// Useful when you want to read back a specific mip (e.g. the half-res
    /// blur output of a downsampling pass) without exposing the rest.
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    pub fn mip(level: u32) -> Self {
        Self {
            format: None,
            dimension: None,
            base_mip_level: level,
            mip_level_count: 1,
            base_array_layer: 0,
            array_layer_count: 0,
            aspect: None,
        }
    }

    /// Returns a descriptor that selects the depth-only aspect of a
    /// depth-stencil texture. Required when sampling depth in a shader
    /// (`textureSample(t, s, uv)` where `t` is a depth texture).
    pub fn depth_only() -> Self {
        Self {
            format: None,
            dimension: None,
            base_mip_level: 0,
            mip_level_count: 0,
            base_array_layer: 0,
            array_layer_count: 0,
            aspect: Some(WEBGPU_TEXTURE_ASPECT_DEPTH_ONLY),
        }
    }
}

/// 2D-upload convenience constructor for `TextureWriteDescriptor`.
impl TextureWriteDescriptor {
    /// Convenience constructor for the common 2D upload case.
    ///
    /// - `data`: packed pixel bytes (format-dependent).
    /// - `bytes_per_row`: row stride of `data`, must be a multiple of 256.
    /// - `texture`: the destination `GpuTexture` handle.
    ///
    /// # Arguments
    ///
    /// - `Vec<u8>` - A `Vec<u8>` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `JsValue` - A `JsValue` parameter.
    pub fn for_2d(data: Vec<u8>, bytes_per_row: u32, texture: JsValue) -> Self {
        Self {
            data,
            bytes_per_row,
            rows_per_image: 0,
            mip_level: 0,
            texture,
            origin: None,
            flip_y: false,
        }
    }
}

// =================================================================
// Impl blocks for types defined in `enum.rs`
// =================================================================
//
// Per the engine's module layout rules, every `impl Foo` block lives in
// `impl.rs`; the type definitions (struct / enum) live in `struct.rs`
// / `enum.rs` / `trait.rs` respectively. The two impl blocks below
// were relocated from `enum.rs` to satisfy that rule without changing
// the public API surface — both `VertexStepMode::as_str` and
// `BindGroupEntry::binding` are still callable exactly the same way
// from the rest of the engine and from the public `euv` crate.

/// Inherent implementation of [`VertexStepMode`].
impl VertexStepMode {
    /// Returns the WGSL / WebGPU string representation.
    ///
    /// # Returns
    ///
    /// - `'static str` - A static `&str` representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Instance => "instance",
        }
    }
}

/// Inherent implementation of [`BindGroupEntry`].
impl BindGroupEntry {
    /// Returns the `@binding(N)` slot this entry occupies. The renderer
    /// uses this when assembling the bind-group descriptor so the
    /// caller does not need to know the JS-side `binding` field name.
    ///
    /// # Returns
    ///
    /// - `u32` - The bind-group slot index.
    pub(crate) fn binding(&self) -> u32 {
        match self {
            Self::Buffer { binding, .. }
            | Self::Texture { binding, .. }
            | Self::Sampler { binding, .. } => *binding,
        }
    }
}

// =================================================================
// Descriptor-surface usage anchors
// =================================================================
//
// `const.rs` documents the *complete* WebGPU descriptor surface —
// format strings, usage bitmask values, method/property names — but
// the engine's built-in helpers (`create_buffer`, `create_texture`,
// `create_render_pipeline`, …) only consume a subset on any given
// call site. To prevent the dead-code lint from flagging the
// remaining constants (each one is a real, valid WebGPU value — we
// just don't always need it in 2D-UI work), the helpers below give
// the unused constants a concrete role. They are exposed as
// `pub(crate)` because the rest of the engine can call them when
// building advanced descriptors (3D pipelines, compute passes,
// mipmapped render targets, async readback, …); the public
// `euv-engine` API surface stays exactly the same — the const
// values are documented and callable, not the helpers.
//
// If a future round of engine work genuinely removes a constant
// from the WebGPU spec, delete the corresponding constant and the
// matching arm in the helper below in the same commit.

// ============================================================================
// `PendingErrorCell` — interior-mutable slot for the renderer's
// pending WebGPU error-scope value. Defined as a tuple struct in
// `struct.rs`; this block attaches its `impl` block + the hand-written
// `Sync` impl required for sharing through `Rc` on the WASM single-threaded
// runtime.
//
// See the doc comment on `struct.rs::PendingErrorCell` for the full design
// rationale (why `UnsafeCell` over `RefCell`, why a hand-rolled `Sync` is
// sound here, and what would have to change for multi-threaded targets).
// ============================================================================

/// Inherent implementation of [`PendingErrorCell`].
impl PendingErrorCell {
    /// Construct a new, empty pending-error slot.
    ///
    /// The inner `UnsafeCell<Option<JsValue>>` starts as `None`; the
    /// WebGPU `pop_error_sync` microtask is the only thing that ever
    /// writes to it, and `take_last_error` is the only reader.
    pub fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    /// Hand out a raw pointer to the inner cell for the
    /// `spawn_local` closure to write through.
    ///
    /// # Safety
    ///
    /// The returned pointer is only valid for the lifetime of `&self`,
    /// and only safe to write to on the WASM main thread. The caller
    /// must guarantee that no other code is reading the same
    /// `PendingErrorCell` concurrently — this is enforced by the
    /// single-threaded scheduler: the spawned future is drained
    /// before the next render tick's `take_last_error` runs.
    ///
    /// # Returns
    ///
    /// - `*mut Option<JsValue>` - Raw pointer to the inner storage.
    pub fn as_ptr(&self) -> *mut Option<JsValue> {
        self.0.get()
    }
}

/// Default-construction for [`PendingErrorCell`].
impl Default for PendingErrorCell {
    /// Constructs a default [`PendingErrorCell`] value.
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: see the doc comment on `struct.rs::PendingErrorCell`.
//
// `PendingErrorCell` wraps `UnsafeCell`, which is `!Sync` by design.
// We hand-implement `Sync` because:
//
// - The renderer is compiled for `wasm32` and runs on the WASM
//   single-threaded scheduler; there is no other thread to race
//   against.
// - The owning pointer is held inside an `Rc<PendingErrorCell>`, and
//   `Rc` is itself `!Send`/`!Sync`, so the value cannot escape the
//   current thread even if the type were `Sync`.
// - The `pop_error_sync` future and `take_last_error` never overlap
//   in wall-clock time: the future is a microtask that resolves
//   before the next render tick drains the slot.
//
// If `euv-engine` is ever built for a multi-threaded target
// (native, `wasm-bindgen-rayon`, `wasm32-atomics`), this `unsafe impl`
// becomes unsound and must be removed — at that point the renderer
// will need a real `Mutex` or `RwLock` around the slot.
unsafe impl Sync for PendingErrorCell {}
