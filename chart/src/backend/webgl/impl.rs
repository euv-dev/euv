use super::*;

/// Inherent implementation of [`WebGl2ChartBackend`].
impl WebGl2ChartBackend {
    /// Returns `true` when the host can give out a WebGL2 context.
    ///
    /// The check opens a throw-away canvas and asks it for a context, so
    /// it costs one context creation. It is cheap enough to run while
    /// the backend is being picked, and it never throws.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_supported() -> bool {
        is_supported()
    }

    /// Creates a backend around the given canvas.
    ///
    /// Returns `None` when the canvas cannot give out a WebGL2 context —
    /// in practice, when the canvas already has a WebGPU context, or when
    /// the browser has no WebGL2 at all — and when the shaders cannot be
    /// built. The factory then falls back to the Canvas2D backend.
    ///
    /// # Arguments
    ///
    /// - `&HtmlCanvasElement` - Shared reference to a `HtmlCanvasElement`.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    pub fn create(canvas: &HtmlCanvasElement) -> Option<Self> {
        let attributes: WebGlContextAttributes = WebGlContextAttributes::new();
        attributes.set_alpha(true);
        attributes.set_premultiplied_alpha(true);
        attributes.set_antialias(true);
        attributes.set_depth(false);
        attributes.set_stencil(false);
        attributes.set_preserve_drawing_buffer(false);
        let options: &JsValue = attributes.as_ref();

        let context: WebGl2RenderingContext = canvas
            .get_context_with_context_options("webgl2", options)
            .ok()
            .flatten()?
            .dyn_into::<WebGl2RenderingContext>()
            .ok()?;

        let mut backend: WebGl2ChartBackend = Self {
            canvas: canvas.clone(),
            context,
            program: None,
            vertex_array: None,
            vertex_buffer: None,
            index_buffer: None,
            atlas_texture: None,
            ramp_texture: None,
            parameter_texture: None,
            images: Vec::new(),
            tessellator: Tessellator::new(),
            vertices: Vec::new(),
            indices: Vec::new(),
            batches: Vec::new(),
            gradients: Vec::new(),
            fingerprint: 0,
            built: false,
            width: 0.0,
            height: 0.0,
            dpr: 1.0,
            scissor: None,
            scissor_dirty: true,
            primary_texture: None,
        };
        if !backend.ensure_resources() {
            return None;
        }
        Some(backend)
    }

    /// Returns the canvas the backend draws into.
    ///
    /// # Returns
    ///
    /// - `&HtmlCanvasElement` - A `HtmlCanvasElement` value.
    pub fn canvas(&self) -> &HtmlCanvasElement {
        self.get_canvas()
    }

    /// Returns the number of draw calls the last frame used.
    ///
    /// The number is a debugging aid: it is the number of batches the
    /// last frame was split into, which is what a chart with no images
    /// and no clip paths reports as one.
    ///
    /// # Returns
    ///
    /// - `usize` - A non-negative integer.
    pub fn draw_call_count(&self) -> usize {
        self.get_batches().len()
    }

    /// Builds the GPU resources the backend needs, if they are missing.
    ///
    /// The resources are created once and then kept: the backend only
    /// comes back here after [`ChartBackend::dispose`] has released
    /// them.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    fn ensure_resources(&mut self) -> bool {
        if self.try_get_program().is_some()
            && self.try_get_vertex_array().is_some()
            && self.try_get_vertex_buffer().is_some()
            && self.try_get_index_buffer().is_some()
        {
            return true;
        }

        let Some(program) = WebGl2Program::create(self.get_context()) else {
            return false;
        };
        let Some(vertex_array) = self.get_context().create_vertex_array() else {
            return false;
        };
        let Some(vertex_buffer) = self.get_context().create_buffer() else {
            return false;
        };
        let Some(index_buffer) = self.get_context().create_buffer() else {
            return false;
        };

        // The vertex layout is recorded in the vertex array object, so
        // it is set up once and every later frame inherits it.
        self.get_context().bind_vertex_array(Some(&vertex_array));
        self.get_context()
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        self.get_context().bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );
        for (location, size, offset) in [
            (GL_ATTRIBUTE_POSITION, 2, 0),
            (GL_ATTRIBUTE_UV, 2, 8),
            (GL_ATTRIBUTE_COLOR, 4, 16),
            (GL_ATTRIBUTE_META, 2, 24),
        ] {
            self.get_context().enable_vertex_attrib_array(location);
            self.get_context().vertex_attrib_pointer_with_i32(
                location,
                size,
                WebGl2RenderingContext::FLOAT,
                false,
                GL_VERTEX_STRIDE,
                offset,
            );
        }

        // The chart draws in the order the display list asks for, with
        // alpha blending and without depth or face culling: the
        // tessellator emits triangles of both windings, and a label has
        // to stay visible whatever the shapes around it do.
        self.get_context().enable(WebGl2RenderingContext::BLEND);
        self.get_context().blend_func_separate(
            WebGl2RenderingContext::ONE,
            WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
            WebGl2RenderingContext::ONE,
            WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
        );
        self.get_context()
            .disable(WebGl2RenderingContext::DEPTH_TEST);
        self.get_context()
            .disable(WebGl2RenderingContext::CULL_FACE);

        program.set_samplers(self.get_context());
        self.set_program(Some(program));
        self.set_vertex_array(Some(vertex_array));
        self.set_vertex_buffer(Some(vertex_buffer));
        self.set_index_buffer(Some(index_buffer));
        true
    }

    /// Rebuilds the geometry of the frame from the display list.
    ///
    /// Everything the display list holds ends up in one vertex buffer,
    /// in paint order: the fills, the strokes, the area strips, the
    /// labels, and the images. The images are the only exception — they
    /// need a texture of their own, so they end the batch they are in.
    ///
    /// # Arguments
    ///
    /// - `&DisplayList` - Shared reference to a `DisplayList`.
    /// - `&mut TextShaper` - Mutable reference to a `TextShaper` (mutated in place).
    fn rebuild(&mut self, list: &DisplayList, shaper: &mut TextShaper) {
        self.get_mut_vertices().clear();
        self.get_mut_indices().clear();
        self.get_mut_batches().clear();
        self.get_mut_gradients().clear();

        let mut clip_stack: Vec<Option<WebGl2Scissor>> = Vec::new();
        let mut clip: Option<WebGl2Scissor> = None;
        let mut batch_start: usize = 0_usize;
        let mut batch_kind: WebGl2BatchKind = WebGl2BatchKind::Atlas;
        let mut batch_image: usize = 0_usize;

        for item in list.items() {
            match &item.cmd {
                DrawCmd::Fill {
                    path,
                    paint,
                    opacity,
                } => {
                    self.append_fill(path, paint, *opacity);
                }
                DrawCmd::Stroke {
                    path,
                    paint,
                    style,
                    opacity,
                } => {
                    self.append_stroke(path, paint, style, *opacity);
                }
                DrawCmd::Strip {
                    points,
                    baseline,
                    paint,
                    opacity,
                } => {
                    self.append_strip(points, *baseline, paint, *opacity);
                }
                DrawCmd::Text { text, x, y, style } => {
                    self.append_text(text, *x, *y, style, shaper);
                }
                DrawCmd::Image {
                    image,
                    x,
                    y,
                    width,
                    height,
                } => {
                    // An image is sampled from a texture of its own, so
                    // it closes the batch it follows and forms one of
                    // its own.
                    self.flush_batch(&mut batch_start, batch_kind, batch_image, clip);
                    let image_index: usize = self.image_index(image);
                    self.append_image(*x, *y, *width, *height);
                    self.flush_batch(&mut batch_start, WebGl2BatchKind::Image, image_index, clip);
                    batch_kind = WebGl2BatchKind::Atlas;
                    batch_image = 0;
                }
                DrawCmd::Clip { path } => {
                    self.flush_batch(&mut batch_start, batch_kind, batch_image, clip);
                    clip_stack.push(clip);
                    clip = Some(intersect_scissor(clip, scissor_of(&path.bounds())));
                }
                DrawCmd::PopClip => {
                    self.flush_batch(&mut batch_start, batch_kind, batch_image, clip);
                    clip = clip_stack.pop().flatten();
                }
            }
        }
        self.flush_batch(&mut batch_start, batch_kind, batch_image, clip);
    }

    /// Appends a filled path.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Shared reference to a `Path`.
    /// - `&Fill` - Shared reference to a `Fill`.
    /// - `f64` - A 64-bit float (`f64`).
    fn append_fill(&mut self, path: &Path, paint: &Fill, opacity: f64) {
        let style: WebGl2Style = match paint {
            Fill::None => return,
            Fill::Solid(color) => WebGl2Style {
                color: scaled_color(*color, opacity),
                kind: WebGl2VertexKind::Solid.code(),
                gradient: 0.0,
            },
            Fill::Linear(gradient) => {
                let row: f32 = self.gradient_row(&WebGl2Gradient::Linear((*gradient).clone()));
                WebGl2Style {
                    color: [1.0, 1.0, 1.0, opacity.clamp(0.0, 1.0) as f32],
                    kind: WebGl2VertexKind::Gradient.code(),
                    gradient: row,
                }
            }
            Fill::Radial(gradient) => {
                let row: f32 = self.gradient_row(&WebGl2Gradient::Radial((*gradient).clone()));
                WebGl2Style {
                    color: [1.0, 1.0, 1.0, opacity.clamp(0.0, 1.0) as f32],
                    kind: WebGl2VertexKind::Gradient.code(),
                    gradient: row,
                }
            }
        };
        let mesh: SolidMesh = self.get_mut_tessellator().fill(path, 0);
        self.push_mesh(&mesh, &style);
    }

    /// Appends a stroked path.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Shared reference to a `Path`.
    /// - `&Stroke` - Shared reference to a `Stroke`.
    /// - `&LineStyle` - Shared reference to a `LineStyle`.
    /// - `f64` - A 64-bit float (`f64`).
    fn append_stroke(&mut self, path: &Path, paint: &Stroke, style: &LineStyle, opacity: f64) {
        if style.width <= 0.0 {
            return;
        }
        let alpha: f32 = (opacity * style.opacity).clamp(0.0, 1.0) as f32;
        let shading: WebGl2Style = match paint {
            Stroke::None => return,
            Stroke::Solid(color) => WebGl2Style {
                color: scaled_color(*color, opacity * style.opacity),
                kind: WebGl2VertexKind::Solid.code(),
                gradient: 0.0,
            },
            Stroke::Linear(gradient) => {
                let row: f32 = self.gradient_row(&WebGl2Gradient::Linear((*gradient).clone()));
                WebGl2Style {
                    color: [1.0, 1.0, 1.0, alpha],
                    kind: WebGl2VertexKind::Gradient.code(),
                    gradient: row,
                }
            }
            Stroke::Radial(gradient) => {
                let row: f32 = self.gradient_row(&WebGl2Gradient::Radial((*gradient).clone()));
                WebGl2Style {
                    color: [1.0, 1.0, 1.0, alpha],
                    kind: WebGl2VertexKind::Gradient.code(),
                    gradient: row,
                }
            }
        };

        let geometry: StrokeStyle = style.geometry();
        let mesh: SolidMesh = if style.dash.is_solid() {
            self.get_mut_tessellator().stroke(path, &geometry, 0)
        } else {
            // A dashed stroke is broken into its dashes first, because
            // the tessellator strokes a path as one piece of geometry.
            let contours: Vec<Contour> = self.get_mut_tessellator().flatten(path).contours.clone();
            let dashed: Vec<Contour> =
                dash_contours(&contours, &style.dash.pattern, style.dash.offset);
            self.get_mut_tessellator()
                .stroke_contours(&dashed, &geometry, 0)
        };
        self.push_mesh(&mesh, &shading);
    }

    /// Appends an area strip.
    ///
    /// The strip is already a set of triangles, so it is copied over as
    /// it is: running the tessellator over it again would be wasted
    /// work on the hottest path of the whole crate.
    ///
    /// # Arguments
    ///
    /// - `&[Point]` - Shared reference to a `[Point]`.
    /// - `f32` - A 32-bit float (`f32`).
    /// - `&Fill` - Shared reference to a `Fill`.
    /// - `f64` - A 64-bit float (`f64`).
    fn append_strip(&mut self, points: &[Point], baseline: f32, paint: &Fill, opacity: f64) {
        if points.len() < 2 {
            return;
        }
        let style: WebGl2Style = match paint {
            Fill::None => return,
            Fill::Solid(color) => WebGl2Style {
                color: scaled_color(*color, opacity),
                kind: WebGl2VertexKind::Solid.code(),
                gradient: 0.0,
            },
            Fill::Linear(gradient) => {
                let row: f32 = self.gradient_row(&WebGl2Gradient::Linear((*gradient).clone()));
                WebGl2Style {
                    color: [1.0, 1.0, 1.0, opacity.clamp(0.0, 1.0) as f32],
                    kind: WebGl2VertexKind::Gradient.code(),
                    gradient: row,
                }
            }
            Fill::Radial(gradient) => {
                let row: f32 = self.gradient_row(&WebGl2Gradient::Radial((*gradient).clone()));
                WebGl2Style {
                    color: [1.0, 1.0, 1.0, opacity.clamp(0.0, 1.0) as f32],
                    kind: WebGl2VertexKind::Gradient.code(),
                    gradient: row,
                }
            }
        };
        let mesh: SolidMesh = self.get_mut_tessellator().fill_strip(points, baseline, 0);
        self.push_mesh(&mesh, &style);
    }

    /// Appends a label.
    ///
    /// The run is shaped with the same shaper the other backends use, so
    /// a label lands on the same pixel whichever backend is in use. The
    /// background of the label is a plain rectangle, drawn before the
    /// glyphs, exactly as the Canvas2D backend draws it.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `&TextStyle` - Shared reference to a `TextStyle`.
    /// - `&mut TextShaper` - Mutable reference to a `TextShaper` (mutated in place).
    fn append_text(
        &mut self,
        text: &str,
        x: f64,
        y: f64,
        style: &TextStyle,
        shaper: &mut TextShaper,
    ) {
        let run: TextRun = shaper.shape(text, style, x, y);
        if let Some(background) = style.background {
            let shading: WebGl2Style = WebGl2Style {
                color: scaled_color(background, 1.0),
                kind: WebGl2VertexKind::Solid.code(),
                gradient: 0.0,
            };
            let rect: WebGl2TextureRect = WebGl2TextureRect {
                u: 0.0,
                v: 0.0,
                u_size: 0.0,
                v_size: 0.0,
            };
            for line in &run.lines {
                let left: f64 = line.left - style.padding.3;
                let top: f64 = line.baseline - run.ascent - style.padding.0;
                let width: f64 = line.width + style.padding.1 + style.padding.3;
                let height: f64 = run.ascent + run.descent + style.padding.0 + style.padding.2;
                self.push_quad(
                    left as f32,
                    top as f32,
                    width as f32,
                    height as f32,
                    rect,
                    &shading,
                );
            }
        }
        let shading: WebGl2Style = WebGl2Style {
            color: unpack_color(style.color.to_u32()),
            kind: WebGl2VertexKind::Text.code(),
            gradient: 0.0,
        };
        for quad in &run.quads {
            let rect: WebGl2TextureRect = WebGl2TextureRect {
                u: quad.u,
                v: quad.v,
                u_size: quad.u_size,
                v_size: quad.v_size,
            };
            self.push_quad(quad.x, quad.y, quad.width, quad.height, rect, &shading);
        }
    }

    /// Appends an image quad.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    fn append_image(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let shading: WebGl2Style = WebGl2Style {
            color: [1.0, 1.0, 1.0, 1.0],
            kind: WebGl2VertexKind::Image.code(),
            gradient: 0.0,
        };
        let rect: WebGl2TextureRect = WebGl2TextureRect {
            u: 0.0,
            v: 0.0,
            u_size: 1.0,
            v_size: 1.0,
        };
        self.push_quad(
            x as f32,
            y as f32,
            width as f32,
            height as f32,
            rect,
            &shading,
        );
    }

    /// Appends one vertex.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    /// - `&WebGl2Style` - Shared reference to a `WebGl2Style`.
    fn push_vertex(&mut self, x: f32, y: f32, u: f32, v: f32, style: &WebGl2Style) {
        self.get_mut_vertices().push(x);
        self.get_mut_vertices().push(y);
        self.get_mut_vertices().push(u);
        self.get_mut_vertices().push(v);
        self.get_mut_vertices().extend_from_slice(&style.color);
        self.get_mut_vertices().push(style.kind);
        self.get_mut_vertices().push(style.gradient);
    }

    /// Appends a tessellated mesh.
    ///
    /// # Arguments
    ///
    /// - `&SolidMesh` - Shared reference to a `SolidMesh`.
    /// - `&WebGl2Style` - Shared reference to a `WebGl2Style`.
    fn push_mesh(&mut self, mesh: &SolidMesh, style: &WebGl2Style) {
        let base: u32 = (self.get_vertices().len() / GL_VERTEX_FLOATS) as u32;
        for vertex in &mesh.vertices {
            self.push_vertex(vertex.x, vertex.y, 0.0, 0.0, style);
        }
        for index in &mesh.indices {
            self.get_mut_indices().push(base + index);
        }
    }

    /// Appends one quad.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    /// - `WebGl2TextureRect` - A `WebGl2TextureRect` parameter.
    /// - `&WebGl2Style` - Shared reference to a `WebGl2Style`.
    fn push_quad(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        rect: WebGl2TextureRect,
        style: &WebGl2Style,
    ) {
        let base: u32 = (self.get_vertices().len() / GL_VERTEX_FLOATS) as u32;
        self.push_vertex(x, y, rect.u, rect.v, style);
        self.push_vertex(x + width, y, rect.u + rect.u_size, rect.v, style);
        self.push_vertex(
            x + width,
            y + height,
            rect.u + rect.u_size,
            rect.v + rect.v_size,
            style,
        );
        self.push_vertex(x, y + height, rect.u, rect.v + rect.v_size, style);
        self.get_mut_indices().push(base);
        self.get_mut_indices().push(base + 1);
        self.get_mut_indices().push(base + 2);
        self.get_mut_indices().push(base);
        self.get_mut_indices().push(base + 2);
        self.get_mut_indices().push(base + 3);
    }

    /// Closes the batch that is being built.
    ///
    /// # Arguments
    ///
    /// - `&mut usize` - Mutable reference to a `usize` (mutated in place).
    /// - `WebGl2BatchKind` - A `WebGl2BatchKind` parameter.
    /// - `usize` - Mutable reference to a `usize` (mutated in place).
    /// - `Option<WebGl2Scissor>` - A `Option<WebGl2Scissor>` parameter.
    fn flush_batch(
        &mut self,
        start: &mut usize,
        kind: WebGl2BatchKind,
        image: usize,
        clip: Option<WebGl2Scissor>,
    ) {
        let count: usize = self.get_indices().len() - *start;
        if count == 0 {
            return;
        }
        self.get_mut_batches().push(WebGl2Batch {
            kind,
            first_index: *start as u32,
            index_count: count as u32,
            image,
            clip,
        });
        *start = self.get_indices().len();
    }

    /// Returns the row of the gradient table a gradient sits in.
    ///
    /// Gradients are shared: two shapes that use the same gradient point
    /// at the same row of the ramp texture, so the number of rows stays
    /// as small as the number of distinct gradients in the chart.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2Gradient` - Shared reference to a `WebGl2Gradient`.
    ///
    /// # Returns
    ///
    /// - `f32` - A 32-bit float.
    fn gradient_row(&mut self, gradient: &WebGl2Gradient) -> f32 {
        for (index, existing) in self.get_gradients().iter().enumerate() {
            let same: bool = match (existing, gradient) {
                (WebGl2Gradient::Linear(left), WebGl2Gradient::Linear(right)) => left == right,
                (WebGl2Gradient::Radial(left), WebGl2Gradient::Radial(right)) => left == right,
                _ => false,
            };
            if same {
                return index as f32;
            }
        }
        self.get_mut_gradients().push(gradient.clone());
        (self.get_gradients().len() - 1) as f32
    }

    /// Returns the index of the image table an image sits in.
    ///
    /// # Arguments
    ///
    /// - `&HtmlImageElement` - Shared reference to a `HtmlImageElement`.
    ///
    /// # Returns
    ///
    /// - `usize` - A non-negative integer.
    fn image_index(&mut self, image: &HtmlImageElement) -> usize {
        let incoming: &JsValue = image.as_ref();
        for (index, existing) in self.get_images().iter().enumerate() {
            let current: &JsValue = existing.element.as_ref();
            if current == incoming {
                return index;
            }
        }
        self.get_mut_images().push(WebGl2Image {
            element: image.clone(),
            texture: None,
        });
        self.get_images().len() - 1
    }

    /// Uploads the geometry of the frame.
    fn upload_geometry(&mut self) {
        let (Some(vertex_array), Some(vertex_buffer), Some(index_buffer)) = (
            self.try_get_vertex_array().as_ref(),
            self.try_get_vertex_buffer().as_ref(),
            self.try_get_index_buffer().as_ref(),
        ) else {
            return;
        };
        self.get_context().bind_vertex_array(Some(vertex_array));
        let vertices: Float32Array = js_sys::Float32Array::from(self.get_vertices().as_slice());
        self.get_context()
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(vertex_buffer));
        self.get_context().buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            vertices.as_ref(),
            WebGl2RenderingContext::STATIC_DRAW,
        );
        let indices: Uint32Array = js_sys::Uint32Array::from(self.get_indices().as_slice());
        self.get_context().bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(index_buffer),
        );
        self.get_context().buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            indices.as_ref(),
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    /// Uploads the glyph atlas.
    ///
    /// # Arguments
    ///
    /// - `&GlyphAtlas` - Shared reference to a `GlyphAtlas`.
    fn upload_atlas(&mut self, atlas: &GlyphAtlas) {
        let existing: Option<WebGlTexture> = self.try_get_atlas_texture().clone();
        let texture: WebGlTexture = match existing {
            Some(texture) => texture,
            None => {
                let Some(texture) = self.get_context().create_texture() else {
                    return;
                };
                self.get_context()
                    .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
                self.get_context().tex_parameteri(
                    WebGl2RenderingContext::TEXTURE_2D,
                    WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                    WebGl2RenderingContext::LINEAR as i32,
                );
                self.get_context().tex_parameteri(
                    WebGl2RenderingContext::TEXTURE_2D,
                    WebGl2RenderingContext::TEXTURE_MAG_FILTER,
                    WebGl2RenderingContext::LINEAR as i32,
                );
                self.get_context().tex_parameteri(
                    WebGl2RenderingContext::TEXTURE_2D,
                    WebGl2RenderingContext::TEXTURE_WRAP_S,
                    WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
                );
                self.get_context().tex_parameteri(
                    WebGl2RenderingContext::TEXTURE_2D,
                    WebGl2RenderingContext::TEXTURE_WRAP_T,
                    WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
                );
                self.set_atlas_texture(Some(texture.clone()));
                texture
            }
        };
        self.get_context()
            .active_texture(WebGl2RenderingContext::TEXTURE0 + GL_TEXTURE_UNIT_PRIMARY);
        self.get_context()
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        let _: Result<(), JsValue> = self
            .get_context()
            .tex_image_2d_with_u32_and_u32_and_html_canvas_element(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                atlas.canvas(),
            );
        // The binding on the primary unit changed, so the backend cannot
        // assume the atlas is still bound there.
        self.set_primary_texture(None);
    }

    /// Uploads the gradient ramps and their parameters.
    fn upload_gradients(&mut self) {
        let rows: usize = self.get_gradients().len().max(1);

        let mut ramp: Vec<u8> = vec![0_u8; GL_RAMP_WIDTH * rows * 4];
        for (index, gradient) in self.get_gradients().iter().enumerate() {
            let bytes: Vec<u8> = ramp_bytes(gradient);
            let start: usize = index * GL_RAMP_WIDTH * 4;
            ramp[start..start + bytes.len()].copy_from_slice(&bytes);
        }

        let mut parameters: Vec<f32> = vec![0.0_f32; GL_PARAMETER_WIDTH * rows * 4];
        for (index, gradient) in self.get_gradients().iter().enumerate() {
            let values: [f32; 8] = parameter_values(gradient);
            let start: usize = index * GL_PARAMETER_WIDTH * 4;
            parameters[start..start + values.len()].copy_from_slice(&values);
        }

        // The ramp holds the colours, so it is filtered smoothly.
        let existing: Option<WebGlTexture> = self.try_get_ramp_texture().clone();
        let ramp_texture: WebGlTexture = match existing {
            Some(texture) => texture,
            None => {
                let Some(texture) = self.get_context().create_texture() else {
                    return;
                };
                self.set_ramp_texture(Some(texture.clone()));
                texture
            }
        };
        self.get_context()
            .active_texture(WebGl2RenderingContext::TEXTURE0 + GL_TEXTURE_UNIT_RAMP);
        self.get_context()
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&ramp_texture));
        self.get_context().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        self.get_context().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        self.get_context().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.get_context().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        let _: Result<(), JsValue> = self
            .get_context()
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                GL_RAMP_WIDTH as i32,
                rows as i32,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                Some(&ramp),
            );

        // The parameters hold coordinates, so they are read exactly and
        // never filtered.
        let existing: Option<WebGlTexture> = self.try_get_parameter_texture().clone();
        let parameter_texture: WebGlTexture = match existing {
            Some(texture) => texture,
            None => {
                let Some(texture) = self.get_context().create_texture() else {
                    return;
                };
                self.set_parameter_texture(Some(texture.clone()));
                texture
            }
        };
        self.get_context()
            .active_texture(WebGl2RenderingContext::TEXTURE0 + GL_TEXTURE_UNIT_PARAMETERS);
        self.get_context()
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&parameter_texture));
        self.get_context().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );
        self.get_context().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );
        self.get_context().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.get_context().tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        let view: Float32Array = js_sys::Float32Array::from(parameters.as_slice());
        let _: Result<(), JsValue> = self
            .get_context()
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA32F as i32,
                GL_PARAMETER_WIDTH as i32,
                rows as i32,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::FLOAT,
                view.as_ref(),
                0,
            );
    }

    /// Uploads the images the frame refers to.
    fn upload_images(&mut self) {
        for index in 0..self.get_images().len() {
            let element: HtmlImageElement = self.get_images()[index].element.clone();
            let existing: Option<WebGlTexture> = self.get_images()[index].texture.clone();
            let texture: WebGlTexture = match existing {
                Some(texture) => texture,
                None => {
                    let Some(texture) = self.get_context().create_texture() else {
                        continue;
                    };
                    self.get_context()
                        .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
                    self.get_context().tex_parameteri(
                        WebGl2RenderingContext::TEXTURE_2D,
                        WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                        WebGl2RenderingContext::LINEAR as i32,
                    );
                    self.get_context().tex_parameteri(
                        WebGl2RenderingContext::TEXTURE_2D,
                        WebGl2RenderingContext::TEXTURE_MAG_FILTER,
                        WebGl2RenderingContext::LINEAR as i32,
                    );
                    self.get_context().tex_parameteri(
                        WebGl2RenderingContext::TEXTURE_2D,
                        WebGl2RenderingContext::TEXTURE_WRAP_S,
                        WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
                    );
                    self.get_context().tex_parameteri(
                        WebGl2RenderingContext::TEXTURE_2D,
                        WebGl2RenderingContext::TEXTURE_WRAP_T,
                        WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
                    );
                    self.get_mut_images()[index].texture = Some(texture.clone());
                    texture
                }
            };
            self.get_context()
                .active_texture(WebGl2RenderingContext::TEXTURE0 + GL_TEXTURE_UNIT_PRIMARY);
            self.get_context()
                .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
            let _: Result<(), JsValue> = self
                .get_context()
                .tex_image_2d_with_u32_and_u32_and_html_image_element(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    WebGl2RenderingContext::RGBA as i32,
                    WebGl2RenderingContext::RGBA,
                    WebGl2RenderingContext::UNSIGNED_BYTE,
                    &element,
                );
        }
        // The binding on the primary unit changed, so the backend cannot
        // assume the atlas is still bound there.
        self.set_primary_texture(None);
    }

    /// Draws the frame.
    ///
    /// The frame is one draw call for every batch, and an ordinary chart
    /// holds a single batch: the whole picture is one set of triangles
    /// in one buffer, drawn in the order the display list asks for.
    fn draw(&mut self) {
        let Some(program) = self.try_get_program().as_ref() else {
            return;
        };
        let drawing_width: u32 = self.get_canvas().width();
        let drawing_height: u32 = self.get_canvas().height();

        self.get_context()
            .viewport(0, 0, drawing_width as i32, drawing_height as i32);
        self.get_context().clear_color(0.0, 0.0, 0.0, 0.0);
        self.get_context()
            .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.get_context().use_program(Some(&program.program));
        program.set_resolution_uniform(
            self.get_context(),
            self.get_width() as f32,
            self.get_height() as f32,
        );
        program.set_gradient_count_uniform(self.get_context(), self.get_gradients().len() as f32);

        self.bind_ramp();
        self.bind_parameters();

        // The batches are taken out of the backend for the duration of
        // the draw, so that the texture bindings can be updated while
        // they are walked.
        let batches: Vec<WebGl2Batch> = take(self.get_mut_batches());
        for batch in &batches {
            self.apply_scissor(batch.clip, drawing_height);
            match batch.kind {
                WebGl2BatchKind::Atlas => self.bind_atlas(),
                WebGl2BatchKind::Image => self.bind_image(batch.image),
            }
            self.get_context().draw_elements_with_i32(
                WebGl2RenderingContext::TRIANGLES,
                batch.index_count as i32,
                WebGl2RenderingContext::UNSIGNED_INT,
                (batch.first_index as i32) * 4,
            );
        }
        self.set_batches(batches);
    }

    /// Applies the scissor rectangle of a batch.
    ///
    /// WebGL2 has no path clipping, so a clip path is approximated by
    /// the rectangle that bounds it. The rectangle is converted into the
    /// framebuffer's own coordinate system, whose origin is at the
    /// bottom left.
    ///
    /// # Arguments
    ///
    /// - `Option<WebGl2Scissor>` - A `Option<WebGl2Scissor>` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    fn apply_scissor(&mut self, clip: Option<WebGl2Scissor>, drawing_height: u32) {
        if !self.get_scissor_dirty() && clip == *self.try_get_scissor() {
            return;
        }
        self.set_scissor_dirty(false);
        self.set_scissor(clip);
        let Some(rect) = clip else {
            self.get_context()
                .disable(WebGl2RenderingContext::SCISSOR_TEST);
            return;
        };
        let x: i32 = (rect.x * self.get_dpr()).round().max(0.0) as i32;
        let y: i32 = (rect.y * self.get_dpr()).round().max(0.0) as i32;
        let width: i32 = (rect.width * self.get_dpr()).round().max(0.0) as i32;
        let height: i32 = (rect.height * self.get_dpr()).round().max(0.0) as i32;
        let bottom: i32 = drawing_height as i32 - (y + height);
        self.get_context()
            .enable(WebGl2RenderingContext::SCISSOR_TEST);
        self.get_context().scissor(x, bottom.max(0), width, height);
    }

    /// Binds the glyph atlas to the primary texture unit.
    fn bind_atlas(&mut self) {
        if *self.try_get_primary_texture() == Some((WebGl2BatchKind::Atlas, 0)) {
            return;
        }
        let Some(texture) = self.try_get_atlas_texture().clone() else {
            return;
        };
        self.get_context()
            .active_texture(WebGl2RenderingContext::TEXTURE0 + GL_TEXTURE_UNIT_PRIMARY);
        self.get_context()
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        self.set_primary_texture(Some((WebGl2BatchKind::Atlas, 0)));
    }

    /// Binds an image to the primary texture unit.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    fn bind_image(&mut self, image: usize) {
        if *self.try_get_primary_texture() == Some((WebGl2BatchKind::Image, image)) {
            return;
        }
        let Some(texture) = self
            .get_images()
            .get(image)
            .and_then(|entry: &WebGl2Image| entry.texture.clone())
        else {
            return;
        };
        self.get_context()
            .active_texture(WebGl2RenderingContext::TEXTURE0 + GL_TEXTURE_UNIT_PRIMARY);
        self.get_context()
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        self.set_primary_texture(Some((WebGl2BatchKind::Image, image)));
    }

    /// Binds the gradient ramps to their texture unit.
    fn bind_ramp(&mut self) {
        let Some(texture) = self.try_get_ramp_texture().clone() else {
            return;
        };
        self.get_context()
            .active_texture(WebGl2RenderingContext::TEXTURE0 + GL_TEXTURE_UNIT_RAMP);
        self.get_context()
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
    }

    /// Binds the gradient parameters to their texture unit.
    fn bind_parameters(&mut self) {
        let Some(texture) = self.try_get_parameter_texture().clone() else {
            return;
        };
        self.get_context()
            .active_texture(WebGl2RenderingContext::TEXTURE0 + GL_TEXTURE_UNIT_PARAMETERS);
        self.get_context()
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
    }
}

/// Implements [`ChartBackend`] for [`WebGl2ChartBackend`].
impl ChartBackend for WebGl2ChartBackend {
    /// Implements the `kind` operation on [`WebGl2ChartBackend`].
    ///
    /// # Returns
    ///
    /// - `BackendKind` - A `BackendKind` value.
    fn kind(&self) -> BackendKind {
        BackendKind::WebGl2
    }

    /// Implements the `resize` operation on [`WebGl2ChartBackend`].
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
        // The geometry is stored in CSS pixels, so a resize only moves
        // the viewport: nothing has to be re-uploaded.
        self.get_context()
            .viewport(0, 0, physical_width as i32, physical_height as i32);
        // The scissor rectangle is expressed in framebuffer pixels, and
        // the framebuffer just moved, so it has to be re-applied.
        self.set_scissor_dirty(true);
    }

    /// Implements the `render` operation on [`WebGl2ChartBackend`].
    ///
    /// # Arguments
    ///
    /// - `&DisplayList` - Shared reference to a `DisplayList`.
    /// - `&mut TextShaper` - Mutable reference to a `TextShaper` (mutated in place).
    fn render(&mut self, list: &DisplayList, shaper: &mut TextShaper) {
        if self.get_context().is_context_lost() {
            return;
        }
        if !self.ensure_resources() {
            return;
        }

        // The display list is retained between frames, so the geometry
        // is only rebuilt when the list actually changed. An unchanged
        // chart costs one fingerprint, one draw call, and nothing else.
        let fingerprint: u64 = fingerprint(list);
        if !self.get_built() || self.get_fingerprint() != fingerprint {
            self.rebuild(list, shaper);
            self.set_fingerprint(fingerprint);
            self.set_built(true);
            self.upload_geometry();
            self.upload_gradients();
            self.upload_images();
        }

        // The atlas grows whenever a new glyph is rasterised into it,
        // which happens while the text is shaped above.
        if self.try_get_atlas_texture().is_none() || shaper.atlas().is_dirty() {
            self.upload_atlas(shaper.atlas());
            shaper.atlas_mut().mark_clean();
        }

        self.draw();
    }

    /// Implements the `dispose` operation on [`WebGl2ChartBackend`].
    fn dispose(&mut self) {
        if let Some(program) = self.get_mut_program().take() {
            self.get_context().delete_program(Some(&program.program));
        }
        if let Some(vertex_array) = self.get_mut_vertex_array().take() {
            self.get_context().delete_vertex_array(Some(&vertex_array));
        }
        if let Some(buffer) = self.get_mut_vertex_buffer().take() {
            self.get_context().delete_buffer(Some(&buffer));
        }
        if let Some(buffer) = self.get_mut_index_buffer().take() {
            self.get_context().delete_buffer(Some(&buffer));
        }
        if let Some(texture) = self.get_mut_atlas_texture().take() {
            self.get_context().delete_texture(Some(&texture));
        }
        if let Some(texture) = self.get_mut_ramp_texture().take() {
            self.get_context().delete_texture(Some(&texture));
        }
        if let Some(texture) = self.get_mut_parameter_texture().take() {
            self.get_context().delete_texture(Some(&texture));
        }
        for index in 0..self.get_images().len() {
            let texture: Option<WebGlTexture> =
                self.get_mut_images()[index].get_mut_texture().take();
            if let Some(texture) = texture {
                self.get_context().delete_texture(Some(&texture));
            }
        }
        self.get_mut_images().clear();
        self.get_mut_vertices().clear();
        self.get_mut_indices().clear();
        self.get_mut_batches().clear();
        self.get_mut_gradients().clear();
        self.set_built(false);
        self.set_fingerprint(0);
        self.set_primary_texture(None);
        self.set_scissor(None);
        self.set_scissor_dirty(true);

        // The canvas is left empty rather than holding the last frame.
        self.get_context().viewport(
            0,
            0,
            self.get_canvas().width() as i32,
            self.get_canvas().height() as i32,
        );
        self.get_context().clear_color(0.0, 0.0, 0.0, 0.0);
        self.get_context()
            .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}

/// Implements [`Drop`] for [`WebGl2ChartBackend`].
impl Drop for WebGl2ChartBackend {
    /// Releases the GPU resources when the backend is dropped.
    ///
    /// A chart that is dropped without being disposed would hold on to
    /// its buffers until the browser collects them, and a chart over a
    /// million points holds on to a lot of memory.
    fn drop(&mut self) {
        <Self as ChartBackend>::dispose(self);
    }
}

/// Inherent implementation of [`WebGl2Program`].
impl WebGl2Program {
    /// Compiles and links the chart's shader program.
    ///
    /// Returns `None` when either stage fails to compile or the program
    /// fails to link, which lets the factory fall back to the Canvas2D
    /// backend instead of drawing nothing.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - Shared reference to a `WebGl2RenderingContext`.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    fn create(context: &WebGl2RenderingContext) -> Option<Self> {
        let vertex: WebGlShader = compile_shader(
            context,
            WebGl2RenderingContext::VERTEX_SHADER,
            GL_VERTEX_SHADER,
        )?;
        let fragment: WebGlShader = compile_shader(
            context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            GL_FRAGMENT_SHADER,
        )?;

        let program: WebGlProgram = context.create_program()?;
        context.attach_shader(&program, &vertex);
        context.attach_shader(&program, &fragment);
        context.link_program(&program);
        context.delete_shader(Some(&vertex));
        context.delete_shader(Some(&fragment));

        let linked: bool = context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false);
        if !linked {
            let log: String = context
                .get_program_info_log(&program)
                .unwrap_or_else(|| "the program did not link".to_string());
            web_sys::console::error_1(&format!("euv-chart: {log}").into());
            context.delete_program(Some(&program));
            return None;
        }

        Some(Self {
            resolution: context.get_uniform_location(&program, "u_resolution"),
            atlas: context.get_uniform_location(&program, "u_atlas"),
            ramp: context.get_uniform_location(&program, "u_ramp"),
            parameters: context.get_uniform_location(&program, "u_parameters"),
            gradient_count: context.get_uniform_location(&program, "u_gradient_count"),
            program,
        })
    }

    /// Points the samplers at the texture units the backend uses.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - Shared reference to a `WebGl2RenderingContext`.
    fn set_samplers(&self, context: &WebGl2RenderingContext) {
        if let Some(location) = self.try_get_atlas() {
            context.uniform1i(Some(location), GL_TEXTURE_UNIT_PRIMARY as i32);
        }
        if let Some(location) = self.try_get_ramp() {
            context.uniform1i(Some(location), GL_TEXTURE_UNIT_RAMP as i32);
        }
        if let Some(location) = self.try_get_parameters() {
            context.uniform1i(Some(location), GL_TEXTURE_UNIT_PARAMETERS as i32);
        }
    }

    /// Sets the size of the canvas, in CSS pixels.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - Shared reference to a `WebGl2RenderingContext`.
    /// - `f32` - A 32-bit float (`f32`).
    fn set_resolution_uniform(&self, context: &WebGl2RenderingContext, width: f32, height: f32) {
        if let Some(location) = self.try_get_resolution() {
            context.uniform2f(Some(location), width, height);
        }
    }

    /// Sets the number of gradient rows the ramp texture holds.
    ///
    /// # Arguments
    ///
    /// - `&WebGl2RenderingContext` - Shared reference to a `WebGl2RenderingContext`.
    /// - `f32` - A 32-bit float (`f32`).
    fn set_gradient_count_uniform(&self, context: &WebGl2RenderingContext, count: f32) {
        if let Some(location) = self.try_get_gradient_count() {
            context.uniform1f(Some(location), count);
        }
    }
}

/// Inherent implementation of [`WebGl2VertexKind`].
impl WebGl2VertexKind {
    /// Returns the code the shader tells the kinds apart by.
    ///
    /// The codes are part of the shader contract: the fragment stage
    /// compares them against the constants it was written with, so they
    /// must not be renumbered without changing the shader.
    ///
    /// # Returns
    ///
    /// - `f32` - A 32-bit float.
    pub fn code(self) -> f32 {
        match self {
            WebGl2VertexKind::Solid => 0.0,
            WebGl2VertexKind::Text => 1.0,
            WebGl2VertexKind::Gradient => 2.0,
            WebGl2VertexKind::Image => 3.0,
        }
    }
}

/// Inherent implementation of [`DisplayHasher`].
impl DisplayHasher {
    /// The offset basis of the FNV-1a hash.
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

    /// The prime the FNV-1a hash multiplies by.
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    /// Returns a hasher with the empty fingerprint.
    pub(crate) fn new() -> Self {
        Self { hash: Self::OFFSET }
    }

    /// Mixes one word into the hash.
    ///
    /// # Arguments
    ///
    /// - `u64` - A `u64` parameter.
    pub(crate) fn write_u64(&mut self, value: u64) {
        self.set_hash(self.get_hash() ^ value);
        self.set_hash(self.get_hash().wrapping_mul(Self::PRIME));
    }

    /// Mixes one 32-bit word into the hash.
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    pub(crate) fn write_u32(&mut self, value: u32) {
        self.write_u64(value as u64);
    }

    /// Mixes one floating-point value into the hash.
    ///
    /// The bits of the value are mixed rather than the value itself, so
    /// that two shapes that differ in the last decimal place are told
    /// apart.
    ///
    /// # Arguments
    ///
    /// - `f32` - A 32-bit float (`f32`).
    pub(crate) fn write_f32(&mut self, value: f32) {
        self.write_u32(value.to_bits());
    }

    /// Mixes one 64-bit floating-point value into the hash.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub(crate) fn write_f64(&mut self, value: f64) {
        self.write_u64(value.to_bits());
    }

    /// Mixes a string into the hash.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    pub(crate) fn write_str(&mut self, value: &str) {
        self.write_u64(value.len() as u64);
        for chunk in value.as_bytes().chunks(8) {
            let mut word: [u8; 8] = [0_u8; 8];
            word[..chunk.len()].copy_from_slice(chunk);
            self.write_u64(u64::from_le_bytes(word));
        }
    }

    /// Returns the fingerprint of everything written so far.
    ///
    /// # Returns
    ///
    /// - `u64` - A `u64` value.
    pub(crate) fn finish(&self) -> u64 {
        self.get_hash()
    }
}
