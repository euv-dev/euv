use super::*;

/// Inherent implementation of [`TextShaper`].
impl TextShaper {
    /// Creates a shaper with the given measurer and atlas.
    ///
    /// # Arguments
    ///
    /// - `TextMeasurer` - A `TextMeasurer` parameter.
    /// - `GlyphAtlas` - A `GlyphAtlas` parameter.
    pub fn new(measurer: TextMeasurer, atlas: GlyphAtlas) -> Self {
        Self { measurer, atlas }
    }

    /// Returns the measurer.
    ///
    /// # Returns
    ///
    /// - `&TextMeasurer` - A `TextMeasurer` value.
    pub fn measurer(&self) -> &TextMeasurer {
        self.get_measurer()
    }

    /// Returns the atlas.
    ///
    /// # Returns
    ///
    /// - `&GlyphAtlas` - A `GlyphAtlas` value.
    pub fn atlas(&self) -> &GlyphAtlas {
        self.get_atlas()
    }

    /// Returns the atlas as mutable, so a backend can upload it.
    ///
    /// # Returns
    ///
    /// - `&mut GlyphAtlas` - A `mut GlyphAtlas` value.
    pub fn atlas_mut(&mut self) -> &mut GlyphAtlas {
        self.get_mut_atlas()
    }

    /// Shapes a run of text.
    ///
    /// `x` and `y` are the anchor of the text; the alignment and the
    /// vertical alignment of the style decide which part of the text
    /// lands on the anchor.
    ///
    /// A text that carries rich-text markup — `{name|content}` — is
    /// split into its runs, and each run is shaped with the style that
    /// the name refers to.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `&TextStyle` - Shared reference to a `TextStyle`.
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `TextRun` - A `TextRun` value.
    pub fn shape(&mut self, text: &str, style: &TextStyle, x: f64, y: f64) -> TextRun {
        if !style.rich.is_empty() && text.contains('{') {
            return self.shape_rich(text, style, x, y);
        }
        self.shape_plain(text, style, x, y)
    }

    /// Shapes a run of plain text.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `&TextStyle` - Shared reference to a `TextStyle`.
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `TextRun` - A `TextRun` value.
    fn shape_plain(&mut self, text: &str, style: &TextStyle, x: f64, y: f64) -> TextRun {
        let lines: Vec<&str> = text.split('\n').collect();
        let font: String = style.font();
        let mut quads: Vec<GlyphQuad> = Vec::new();
        let mut widths: Vec<f64> = Vec::with_capacity(lines.len());
        let mut ascent: f64 = 0.0_f64;
        let mut descent: f64 = 0.0_f64;
        let line_height: f64 = style.line_height_px();

        for line in &lines {
            let metrics: TextMetrics = self.get_measurer().measure(line, &font);
            widths.push(metrics.width);
            ascent = ascent.max(metrics.ascent);
            descent = descent.max(metrics.descent);
        }

        let block_height: f64 = line_height * lines.len() as f64;
        let top: f64 = match style.vertical_align {
            TextVerticalAlign::Top => y,
            TextVerticalAlign::Middle => y - block_height * 0.5,
            TextVerticalAlign::Bottom => y - block_height,
            TextVerticalAlign::Baseline => y - ascent,
        };

        let mut shaped_lines: Vec<TextLine> = Vec::with_capacity(lines.len());
        for (index, line) in lines.iter().enumerate() {
            let line_width: f64 = widths[index];
            let left: f64 = match style.align {
                TextAlign::Left => x,
                TextAlign::Center => x - line_width * 0.5,
                TextAlign::Right => x - line_width,
            };
            let baseline: f64 = top + line_height * index as f64 + ascent;
            self.shape_line(line, &font, left, baseline, 0, &mut quads);
            shaped_lines.push(TextLine {
                text: (*line).to_string(),
                left,
                baseline,
                width: line_width,
            });
        }

        TextRun {
            quads,
            lines: shaped_lines,
            width: widths.iter().copied().fold(0.0_f64, f64::max),
            height: block_height,
            ascent,
            descent,
            line_height,
            line_count: lines.len(),
        }
    }

    /// Shapes a run of rich text.
    ///
    /// Every run keeps its own font and colour; the runs of one line
    /// share the line's baseline, so they sit next to each other the way
    /// they would in a paragraph.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `&TextStyle` - Shared reference to a `TextStyle`.
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `TextRun` - A `TextRun` value.
    fn shape_rich(&mut self, text: &str, style: &TextStyle, x: f64, y: f64) -> TextRun {
        let runs: Vec<(Option<String>, String)> = split_rich_text(text);
        let mut quads: Vec<GlyphQuad> = Vec::new();
        let mut shaped_lines: Vec<TextLine> = Vec::new();
        let mut total_width: f64 = 0.0_f64;
        let mut ascent: f64 = 0.0_f64;
        let mut descent: f64 = 0.0_f64;
        let mut pen_x: f64 = x;

        for (name, content) in runs {
            let mut run_style: TextStyle = style.clone();
            if let Some(name) = &name
                && let Some((_, rich)) = style
                    .rich
                    .iter()
                    .find(|(key, _): &&(String, RichTextStyle)| key == name)
            {
                if let Some(family) = &rich.family {
                    run_style.family = family.clone();
                }
                if let Some(size) = rich.size {
                    run_style.size = size;
                }
                if let Some(weight) = &rich.weight {
                    run_style.weight = weight.clone();
                }
                if let Some(value) = &rich.style {
                    run_style.style = value.clone();
                }
                if let Some(color) = rich.color {
                    run_style.color = color;
                }
            }
            let color: u32 = run_style.color.to_u32();
            let font: String = run_style.font();
            let metrics: TextMetrics = self.get_measurer().measure(&content, &font);
            ascent = ascent.max(metrics.ascent);
            descent = descent.max(metrics.descent);
            total_width += metrics.width;
            let baseline: f64 = y + metrics.ascent;
            self.shape_line(&content, &font, pen_x, baseline, color, &mut quads);
            shaped_lines.push(TextLine {
                text: content,
                left: pen_x,
                baseline,
                width: metrics.width,
            });
            pen_x += metrics.width;
        }

        let line_count: usize = shaped_lines.len();
        TextRun {
            quads,
            lines: shaped_lines,
            width: total_width,
            height: ascent + descent,
            ascent,
            descent,
            line_height: style.line_height_px(),
            line_count,
        }
    }

    /// Shapes one line of text into glyph quads.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `&mut Vec<GlyphQuad>` - Mutable reference to a `Vec<GlyphQuad>` (mutated in place).
    fn shape_line(
        &mut self,
        line: &str,
        font: &str,
        left: f64,
        baseline: f64,
        color: u32,
        quads: &mut Vec<GlyphQuad>,
    ) {
        let mut pen: f32 = left as f32;
        for character in line.chars() {
            let entry: GlyphEntry = self.get_mut_atlas().glyph(character, font);
            if entry.width > 0.0 && entry.height > 0.0 {
                quads.push(GlyphQuad {
                    x: pen + entry.bearing_x,
                    y: baseline as f32 - entry.bearing_y,
                    width: entry.width,
                    height: entry.height,
                    u: entry.u,
                    v: entry.v,
                    u_size: entry.width / self.get_atlas().width() as f32,
                    v_size: entry.height / self.get_atlas().height() as f32,
                    color,
                });
            }
            pen += entry.advance;
        }
    }
}

/// Inherent implementation of [`GlyphAtlas`].
impl GlyphAtlas {
    /// Creates an atlas of the given size, in pixels.
    ///
    /// Returns `None` when the host has no DOM.
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    pub fn new(width: u32, height: u32) -> Option<Self> {
        let document: Document = web_sys::window()?.document()?;
        let canvas: HtmlCanvasElement = document
            .create_element(CANVAS_TAG)
            .ok()?
            .dyn_into::<HtmlCanvasElement>()
            .ok()?;
        canvas.set_width(width);
        canvas.set_height(height);
        let context: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .ok()
            .flatten()?
            .dyn_into::<CanvasRenderingContext2d>()
            .ok()?;
        Some(Self {
            canvas,
            context,
            entries: HashMap::new(),
            cursor_x: 0.0,
            cursor_y: 0.0,
            shelf_height: 0.0,
            width,
            height,
            dirty: false,
        })
    }

    /// Returns the canvas the glyphs are rasterised into.
    ///
    /// The GPU backends upload this canvas straight into a texture.
    ///
    /// # Returns
    ///
    /// - `&HtmlCanvasElement` - A `HtmlCanvasElement` value.
    pub fn canvas(&self) -> &HtmlCanvasElement {
        self.get_canvas()
    }

    /// Returns the width of the atlas, in pixels.
    ///
    /// # Returns
    ///
    /// - `u32` - A 32-bit unsigned integer.
    pub fn width(&self) -> u32 {
        self.get_width()
    }

    /// Returns the height of the atlas, in pixels.
    ///
    /// # Returns
    ///
    /// - `u32` - A 32-bit unsigned integer.
    pub fn height(&self) -> u32 {
        self.get_height()
    }

    /// Returns `true` when the atlas changed since the last upload.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_dirty(&self) -> bool {
        self.get_dirty()
    }

    /// Marks the atlas as uploaded.
    pub fn mark_clean(&mut self) {
        self.set_dirty(false);
    }

    /// Returns the number of rasterised glyphs.
    ///
    /// # Returns
    ///
    /// - `usize` - A non-negative integer.
    pub fn len(&self) -> usize {
        self.get_entries().len()
    }

    /// Returns `true` when the atlas holds no glyphs.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_empty(&self) -> bool {
        self.get_entries().is_empty()
    }

    /// Returns the entry for the given character and font, rasterising it
    /// on first use.
    ///
    /// # Arguments
    ///
    /// - `char` - A `char` parameter.
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `GlyphEntry` - A `GlyphEntry` value.
    pub fn glyph(&mut self, character: char, font: &str) -> GlyphEntry {
        let key: GlyphKey = GlyphKey {
            character,
            font: font.to_string(),
        };
        if let Some(entry) = self.get_entries().get(&key) {
            return *entry;
        }
        let entry: GlyphEntry = self.rasterize(character, font);
        self.get_mut_entries().insert(key, entry);
        entry
    }

    /// Rasterises one glyph into the atlas.
    ///
    /// # Arguments
    ///
    /// - `char` - A `char` parameter.
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `GlyphEntry` - A `GlyphEntry` value.
    fn rasterize(&mut self, character: char, font: &str) -> GlyphEntry {
        self.get_context().set_font(font);
        let Ok(metrics) = self.get_context().measure_text(&character.to_string()) else {
            return GlyphEntry {
                u: 0.0,
                v: 0.0,
                width: 0.0,
                height: 0.0,
                bearing_x: 0.0,
                bearing_y: 0.0,
                advance: 0.0,
            };
        };

        let advance: f32 = metrics.width() as f32;
        let left: f32 = metrics.actual_bounding_box_left() as f32;
        let right: f32 = metrics.actual_bounding_box_right() as f32;
        let ascent: f32 = metrics.actual_bounding_box_ascent() as f32;
        let descent: f32 = metrics.actual_bounding_box_descent() as f32;

        let glyph_width: f32 = (left + right).ceil().max(0.0);
        let glyph_height: f32 = (ascent + descent).ceil().max(0.0);

        // A space has no ink, so it occupies no space in the atlas.
        if glyph_width <= 0.0 || glyph_height <= 0.0 {
            return GlyphEntry {
                u: 0.0,
                v: 0.0,
                width: 0.0,
                height: 0.0,
                bearing_x: 0.0,
                bearing_y: 0.0,
                advance,
            };
        }

        let padding: f32 = GLYPH_PADDING;
        let cell_width: f32 = glyph_width + padding * 2.0;
        let cell_height: f32 = glyph_height + padding * 2.0;

        if self.get_cursor_x() + cell_width > self.get_width() as f32 {
            self.set_cursor_x(0.0);
            self.set_cursor_y(self.get_cursor_y() + self.get_shelf_height());
            self.set_shelf_height(0.0);
        }
        if self.get_cursor_y() + cell_height > self.get_height() as f32 {
            // The atlas is full. Starting over is cheaper than a
            // compaction pass, and the atlas only fills up when a chart
            // uses more distinct glyphs than fit — at which point the
            // chart is redrawn from scratch anyway.
            self.reset();
        }

        let x: f32 = self.get_cursor_x();
        let y: f32 = self.get_cursor_y();
        self.set_cursor_x(self.get_cursor_x() + cell_width);
        self.set_shelf_height(self.get_shelf_height().max(cell_height));

        // The glyph is drawn with its pen at the cell's left padding and
        // its baseline at the cell's bottom padding.
        let pen_x: f32 = x + padding + left;
        let baseline: f32 = y + padding + ascent;
        self.get_context().set_font(font);
        self.get_context().set_fill_style_str("#ffffff");
        let _: Result<(), JsValue> =
            self.get_context()
                .fill_text(&character.to_string(), pen_x as f64, baseline as f64);

        self.set_dirty(true);

        GlyphEntry {
            u: x / self.get_width() as f32,
            v: y / self.get_height() as f32,
            width: glyph_width,
            height: glyph_height,
            bearing_x: -(left + padding),
            bearing_y: ascent + padding,
            advance,
        }
    }

    /// Empties the atlas and rewinds the packer.
    fn reset(&mut self) {
        self.get_context()
            .clear_rect(0.0, 0.0, self.get_width() as f64, self.get_height() as f64);
        self.get_mut_entries().clear();
        self.set_cursor_x(0.0);
        self.set_cursor_y(0.0);
        self.set_shelf_height(0.0);
        self.set_dirty(true);
    }
}

/// Implements `impl std::fmt::Debug for TextMeasurer`.
impl Debug for TextMeasurer {
    /// Formats the [`TextMeasurer`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut std::fmt::Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result` - A `std::fmt::Result` value.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TextMeasurer")
            .field("cached", &self.get_cache().borrow().len())
            .finish()
    }
}

/// Inherent implementation of [`TextMeasurer`].
impl TextMeasurer {
    /// Creates a measurer backed by a fresh offscreen canvas.
    ///
    /// Returns `None` when the host has no DOM, which is the case for a
    /// native build.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    pub fn new() -> Option<Self> {
        let document: Document = web_sys::window()?.document()?;
        let canvas: HtmlCanvasElement = document
            .create_element(CANVAS_TAG)
            .ok()?
            .dyn_into::<HtmlCanvasElement>()
            .ok()?;
        canvas.set_width(1);
        canvas.set_height(1);
        let context: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .ok()
            .flatten()?
            .dyn_into::<CanvasRenderingContext2d>()
            .ok()?;
        Some(Self {
            context: Some(context),
            cache: Rc::new(RefCell::new(HashMap::new())),
        })
    }

    /// Creates a measurer that estimates the text instead of measuring
    /// it.
    ///
    /// The estimate is the average advance of a Latin character, which is
    /// close enough for a layout that is not going to be drawn — a
    /// server-side render, or a native build.
    pub fn approximate() -> Self {
        Self {
            context: None,
            cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Measures a run of text with the given CSS `font` shorthand.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `TextMetrics` - A `TextMetrics` value.
    pub fn measure(&self, text: &str, font: &str) -> TextMetrics {
        if text.is_empty() {
            return TextMetrics::default();
        }
        let key: (String, String) = (text.to_string(), font.to_string());
        if let Some(metrics) = self.get_cache().borrow().get(&key) {
            return *metrics;
        }
        let metrics: TextMetrics = self.measure_uncached(text, font);
        self.get_cache().borrow_mut().insert(key, metrics);
        metrics
    }

    /// Measures a run of text, bypassing the cache.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `TextMetrics` - A `TextMetrics` value.
    fn measure_uncached(&self, text: &str, font: &str) -> TextMetrics {
        let size: f64 = font_size_of(font);
        let Some(context) = self.try_get_context() else {
            return approximate_metrics(text, size);
        };
        context.set_font(font);
        let Ok(metrics) = context.measure_text(text) else {
            return approximate_metrics(text, size);
        };
        let ascent: f64 = metrics.actual_bounding_box_ascent();
        let descent: f64 = metrics.actual_bounding_box_descent();
        TextMetrics {
            width: metrics.width(),
            height: ascent + descent,
            ascent,
            descent,
            line_height: size * 1.2,
        }
    }

    /// Returns the number of cached measurements.
    ///
    /// # Returns
    ///
    /// - `usize` - A non-negative integer.
    pub fn cache_size(&self) -> usize {
        self.get_cache().borrow().len()
    }

    /// Empties the measurement cache.
    pub fn clear_cache(&self) {
        self.get_cache().borrow_mut().clear();
    }
}
