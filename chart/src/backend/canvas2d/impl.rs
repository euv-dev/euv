use super::*;

/// Inherent implementation of [`Canvas2dChartBackend`].
impl Canvas2dChartBackend {
    /// Creates a backend around the given canvas.
    ///
    /// Returns `None` when the canvas cannot give out a 2D context — in
    /// practice, when the canvas already has a WebGPU or WebGL context.
    ///
    /// # Arguments
    ///
    /// - `&HtmlCanvasElement` - Shared reference to a `HtmlCanvasElement`.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    pub fn create(canvas: &HtmlCanvasElement) -> Option<Self> {
        let context: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .ok()
            .flatten()?
            .dyn_into::<CanvasRenderingContext2d>()
            .ok()?;
        Some(Self {
            canvas: canvas.clone(),
            context,
            dpr: 1.0,
            width: 0.0,
            height: 0.0,
        })
    }

    /// Returns the canvas the backend draws into.
    ///
    /// # Returns
    ///
    /// - `&HtmlCanvasElement` - A `HtmlCanvasElement` value.
    pub fn canvas(&self) -> &HtmlCanvasElement {
        self.get_canvas()
    }

    /// Returns the device pixel ratio the backend renders at.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn dpr(&self) -> f64 {
        self.get_dpr()
    }

    /// Traces a path into the 2D context.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Shared reference to a `Path`.
    pub(crate) fn trace_path(&self, path: &Path) {
        self.get_context().begin_path();
        for command in path.commands() {
            match *command {
                PathCommand::MoveTo(x, y) => self.get_context().move_to(x as f64, y as f64),
                PathCommand::LineTo(x, y) => self.get_context().line_to(x as f64, y as f64),
                PathCommand::QuadTo(cx, cy, x, y) => self
                    .get_context()
                    .quadratic_curve_to(cx as f64, cy as f64, x as f64, y as f64),
                PathCommand::CubicTo(c1x, c1y, c2x, c2y, x, y) => {
                    self.get_context().bezier_curve_to(
                        c1x as f64, c1y as f64, c2x as f64, c2y as f64, x as f64, y as f64,
                    )
                }
                PathCommand::Close => self.get_context().close_path(),
            }
        }
    }

    /// Traces a triangle strip into the 2D context.
    ///
    /// # Arguments
    ///
    /// - `&[Point]` - Shared reference to a `[Point]`.
    /// - `f32` - A 32-bit float (`f32`).
    pub(crate) fn trace_strip(&self, points: &[Point], baseline: f32) {
        self.get_context().begin_path();
        if let Some(first) = points.first() {
            self.get_context().move_to(first.x as f64, first.y as f64);
            for point in &points[1..] {
                self.get_context().line_to(point.x as f64, point.y as f64);
            }
            self.get_context()
                .line_to(points[points.len() - 1].x as f64, baseline as f64);
            self.get_context()
                .line_to(points[0].x as f64, baseline as f64);
            self.get_context().close_path();
        }
    }

    /// Applies a fill paint to the context.
    ///
    /// # Arguments
    ///
    /// - `&Fill` - Shared reference to a `Fill`.
    pub(crate) fn apply_fill(&self, paint: &Fill) {
        match paint {
            Fill::Solid(color) => {
                self.get_context().set_fill_style_str(&color.to_css());
            }
            Fill::Linear(gradient) => {
                if let Some(canvas) = self.create_gradient(gradient) {
                    self.get_context().set_fill_style_canvas_gradient(&canvas);
                }
            }
            Fill::Radial(gradient) => {
                if let Some(canvas) = self.create_radial_gradient(gradient) {
                    self.get_context().set_fill_style_canvas_gradient(&canvas);
                }
            }
            Fill::None => {}
        }
    }

    /// Applies a stroke paint to the context.
    ///
    /// # Arguments
    ///
    /// - `&Stroke` - Shared reference to a `Stroke`.
    pub(crate) fn apply_stroke(&self, paint: &Stroke) {
        match paint {
            Stroke::Solid(color) => {
                self.get_context().set_stroke_style_str(&color.to_css());
            }
            Stroke::Linear(gradient) => {
                if let Some(canvas) = self.create_gradient(gradient) {
                    self.get_context().set_stroke_style_canvas_gradient(&canvas);
                }
            }
            Stroke::Radial(gradient) => {
                if let Some(canvas) = self.create_radial_gradient(gradient) {
                    self.get_context().set_stroke_style_canvas_gradient(&canvas);
                }
            }
            Stroke::None => {}
        }
    }

    /// Builds a Canvas2D linear gradient.
    ///
    /// # Arguments
    ///
    /// - `&LinearGradient` - Shared reference to a `LinearGradient`.
    ///
    /// # Returns
    ///
    /// - `Option<CanvasGradient>` - `Some(...)` on success, `None` otherwise.
    fn create_gradient(&self, gradient: &LinearGradient) -> Option<CanvasGradient> {
        let canvas: CanvasGradient = self.get_context().create_linear_gradient(
            gradient.start.0,
            gradient.start.1,
            gradient.end.0,
            gradient.end.1,
        );
        for stop in &gradient.stops {
            let _: Result<(), JsValue> =
                canvas.add_color_stop(stop.offset as f32, &stop.color.to_css());
        }
        Some(canvas)
    }

    /// Builds a Canvas2D radial gradient.
    ///
    /// # Arguments
    ///
    /// - `&RadialGradient` - Shared reference to a `RadialGradient`.
    ///
    /// # Returns
    ///
    /// - `Option<CanvasGradient>` - `Some(...)` on success, `None` otherwise.
    fn create_radial_gradient(&self, gradient: &RadialGradient) -> Option<CanvasGradient> {
        let canvas: CanvasGradient = self
            .get_context()
            .create_radial_gradient(
                gradient.center.0,
                gradient.center.1,
                0.0,
                gradient.center.0,
                gradient.center.1,
                gradient.radius,
            )
            .ok()?;
        for stop in &gradient.stops {
            let _: Result<(), JsValue> =
                canvas.add_color_stop(stop.offset as f32, &stop.color.to_css());
        }
        Some(canvas)
    }
}

/// Implements [`ChartBackend`] for [`Canvas2dChartBackend`].
impl ChartBackend for Canvas2dChartBackend {
    /// Implements the `kind` operation on [`Canvas2dChartBackend`].
    ///
    /// # Returns
    ///
    /// - `BackendKind` - A `BackendKind` value.
    fn kind(&self) -> BackendKind {
        BackendKind::Canvas2D
    }

    /// Implements the `resize` operation on [`Canvas2dChartBackend`].
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    fn resize(&mut self, width: f64, height: f64, dpr: f64) {
        self.set_dpr(dpr);
        self.set_width(width);
        self.set_height(height);
        let physical_width: u32 = (width * dpr).round().max(1.0) as u32;
        let physical_height: u32 = (height * dpr).round().max(1.0) as u32;
        if self.get_canvas().width() != physical_width {
            self.get_canvas().set_width(physical_width);
        }
        if self.get_canvas().height() != physical_height {
            self.get_canvas().set_height(physical_height);
        }
        // Setting the width or the height of a canvas resets its context
        // state, so the transform has to be re-applied afterwards.
        let _: Result<(), JsValue> = self
            .get_context()
            .set_transform(dpr, 0.0, 0.0, dpr, 0.0, 0.0);
        self.get_context().set_line_join("miter");
    }

    /// Implements the `render` operation on [`Canvas2dChartBackend`].
    ///
    /// # Arguments
    ///
    /// - `&DisplayList` - Shared reference to a `DisplayList`.
    /// - `&mut TextShaper` - Mutable reference to a `TextShaper` (mutated in place).
    fn render(&mut self, list: &DisplayList, shaper: &mut TextShaper) {
        let _: Result<(), JsValue> =
            self.get_context()
                .set_transform(self.get_dpr(), 0.0, 0.0, self.get_dpr(), 0.0, 0.0);
        self.get_context()
            .clear_rect(0.0, 0.0, self.get_width(), self.get_height());
        for item in list.items() {
            match &item.cmd {
                DrawCmd::Fill {
                    path,
                    paint,
                    opacity,
                } => {
                    if matches!(paint, Fill::None) {
                        continue;
                    }
                    self.get_context().save();
                    self.get_context().set_global_alpha(*opacity);
                    self.apply_fill(paint);
                    self.trace_path(path);
                    self.get_context().fill();
                    self.get_context().restore();
                }
                DrawCmd::Stroke {
                    path,
                    paint,
                    style,
                    opacity,
                } => {
                    if matches!(paint, Stroke::None) || style.width <= 0.0 {
                        continue;
                    }
                    self.get_context().save();
                    self.get_context()
                        .set_global_alpha(*opacity * style.opacity);
                    self.apply_stroke(paint);
                    self.get_context().set_line_width(style.width);
                    self.get_context().set_line_cap(line_cap_name(style.cap));
                    self.get_context().set_line_join(line_join_name(style.join));
                    if !style.dash.is_solid() {
                        let dash: Array = Array::new();
                        for length in &style.dash.pattern {
                            dash.push(&JsValue::from_f64(*length as f64));
                        }
                        let _: Result<(), JsValue> =
                            self.get_context().set_line_dash(dash.as_ref());
                        self.get_context()
                            .set_line_dash_offset(style.dash.offset as f64);
                    } else {
                        let _: Result<(), JsValue> =
                            self.get_context().set_line_dash(Array::new().as_ref());
                    }
                    self.trace_path(path);
                    self.get_context().stroke();
                    self.get_context().restore();
                }
                DrawCmd::Strip {
                    points,
                    baseline,
                    paint,
                    opacity,
                } => {
                    if matches!(paint, Fill::None) || points.len() < 2 {
                        continue;
                    }
                    self.get_context().save();
                    self.get_context().set_global_alpha(*opacity);
                    self.apply_fill(paint);
                    self.trace_strip(points, *baseline);
                    self.get_context().fill();
                    self.get_context().restore();
                }
                DrawCmd::Text { text, x, y, style } => {
                    // The run is shaped with the same shaper the GPU
                    // backends use, so the baseline lands on the same
                    // pixel whichever backend is in use.
                    let run: TextRun = shaper.shape(text, style, *x, *y);
                    self.get_context().save();
                    self.get_context().set_font(&style.font());
                    self.get_context().set_text_align("left");
                    self.get_context().set_text_baseline("alphabetic");
                    for line in &run.lines {
                        if let Some(background) = style.background {
                            self.get_context().set_fill_style_str(&background.to_css());
                            self.get_context().fill_rect(
                                line.left - style.padding.3,
                                line.baseline - run.ascent - style.padding.0,
                                line.width + style.padding.1 + style.padding.3,
                                run.ascent + run.descent + style.padding.0 + style.padding.2,
                            );
                        }
                        self.get_context().set_fill_style_str(&style.color.to_css());
                        let _: Result<(), JsValue> =
                            self.get_context()
                                .fill_text(&line.text, line.left, line.baseline);
                    }
                    self.get_context().restore();
                }
                DrawCmd::Image {
                    image,
                    x,
                    y,
                    width,
                    height,
                } => {
                    let _: Result<(), JsValue> = self
                        .get_context()
                        .draw_image_with_html_image_element_and_dw_and_dh(
                            image, *x, *y, *width, *height,
                        );
                }
                DrawCmd::Clip { path } => {
                    self.get_context().save();
                    self.trace_path(path);
                    self.get_context().clip();
                }
                DrawCmd::PopClip => {
                    self.get_context().restore();
                }
            }
        }
    }

    /// Implements the `dispose` operation on [`Canvas2dChartBackend`].
    fn dispose(&mut self) {
        let _: Result<(), JsValue> = self
            .get_context()
            .set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        self.get_context()
            .clear_rect(0.0, 0.0, self.get_width(), self.get_height());
    }
}
