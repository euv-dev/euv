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
        // Reused scratch for `write_css_rgba` — avoids one `Color::to_css`
        // String allocation per style change during replay.
        let mut css_buf: String = String::new();
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
                            css_buf.clear();
                            color.write_css_rgba(&mut css_buf);
                            context.set_fill_style_str(&css_buf);
                            current_fill = Some(color);
                        }
                        run_is_fill = true;
                    } else {
                        if current_stroke != Some(color) {
                            css_buf.clear();
                            color.write_css_rgba(&mut css_buf);
                            context.set_stroke_style_str(&css_buf);
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
                        css_buf.clear();
                        color.write_css_rgba(&mut css_buf);
                        context.set_fill_style_str(&css_buf);
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
    /// Clears the display canvas, then draws the offscreen canvas scaled
    /// down to the logical display size. The active `quality` preset is
    /// applied once at construction via `enable_smoothing` — there is
    /// no need to re-apply it every frame, since the preset never
    /// changes mid-render.
    ///
    /// #32: the previous version called `apply_quality` on every
    /// `present()`, costing `set_image_smoothing_enabled` + 2×Reflect
    /// + 4×from_str ≈ 7 JS crossings per frame for a value that was
    ///   invariant across the entire session.
    pub fn present(&self) {
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
