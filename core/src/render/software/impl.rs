use super::*;

/// Rasterization pipeline methods for `SoftwareRenderer`.
impl SoftwareRenderer {
    /// Creates a software renderer configured for `width x height` physical pixels.
    ///
    /// # Arguments
    ///
    /// - `u32` - Physical width.
    /// - `u32` - Physical height.
    /// - `RenderConfig` - Rendering configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready-to-draw software renderer.
    #[must_use]
    pub fn with_config(width: u32, height: u32, config: RenderConfig) -> Self {
        Self::with_font_provider(width, height, config, NativeSystemFont::deterministic())
    }

    /// Creates a software renderer configured with an explicit `NativeSystemFont` provider.
    ///
    /// # Arguments
    ///
    /// - `u32` - Physical width.
    /// - `u32` - Physical height.
    /// - `RenderConfig` - Rendering configuration.
    /// - `NativeSystemFont` - Native OS or deterministic font provider.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready-to-draw software renderer.
    #[must_use]
    pub fn with_font_provider(
        width: u32,
        height: u32,
        config: RenderConfig,
        font_provider: NativeSystemFont,
    ) -> Self {
        Self {
            framebuffer: Framebuffer::allocate(width, height),
            glyph_atlas: GlyphAtlas::default(),
            font_provider,
            measurer: DeterministicTextMeasurer,
            clip_stack: Vec::with_capacity(8),
            antialiasing: config.get_antialiasing(),
            tile_size: config.get_tile_size().max(16),
        }
    }

    /// Rasterizes `display_list` into the internal `Framebuffer` clipped to `damage`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `&DisplayList` - Backend-agnostic draw command stream.
    /// - `DamageRegion` - Optional dirty damage region for partial rasterization.
    ///
    /// # Returns
    ///
    /// - `(usize, usize)` - Tuple of `(tiles_updated, pixels_shaded)`.
    pub fn render(&mut self, display_list: &DisplayList, damage: DamageRegion) -> (usize, usize) {
        let scale: f32 = display_list.get_scale_factor().max(0.5);
        let phys_w: u32 = ((display_list.get_viewport().get_width() * scale).round() as u32).max(1);
        let phys_h: u32 =
            ((display_list.get_viewport().get_height() * scale).round() as u32).max(1);
        let resized: bool = self.get_mut_framebuffer().resize(phys_w, phys_h);
        let effective_damage: DamageRegion = if resized {
            DamageRegion::full()
        } else {
            damage
        };
        let Some(root_clip) = effective_damage.resolve(phys_w, phys_h) else {
            return (0, 0);
        };
        let tile_sz: u32 = self.get_tile_size().max(16);
        let tiles_x: usize = ((root_clip.get_width().ceil() as u32).div_ceil(tile_sz)) as usize;
        let tiles_y: usize = ((root_clip.get_height().ceil() as u32).div_ceil(tile_sz)) as usize;
        let tiles_updated: usize = tiles_x.saturating_mul(tiles_y);
        self.get_mut_clip_stack().clear();
        self.get_mut_clip_stack().push(root_clip);
        let mut pixels_shaded: usize = 0;
        for cmd in display_list.get_commands().iter() {
            pixels_shaded += self.execute_command(cmd, scale);
        }
        (tiles_updated, pixels_shaded)
    }

    /// Executes a single `DrawCommand` against the current clip stack and framebuffer.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `&DrawCommand` - Command to rasterize.
    /// - `f32` - Active DPI scale factor.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of pixels shaded by this command.
    fn execute_command(&mut self, cmd: &DrawCommand, scale: f32) -> usize {
        let active_clip: Rect = self.current_clip();
        match cmd {
            DrawCommand::Clear(color) => {
                if active_clip.get_x() <= 0.0
                    && active_clip.get_y() <= 0.0
                    && active_clip.get_width() >= (self.get_framebuffer().get_width() as f32)
                    && active_clip.get_height() >= (self.get_framebuffer().get_height() as f32)
                {
                    self.get_mut_framebuffer().clear(*color);
                    (self.get_framebuffer().get_width() as usize)
                        * (self.get_framebuffer().get_height() as usize)
                } else {
                    self.fill_rect_clipped(active_clip, *color, active_clip)
                }
            }
            DrawCommand::FillRoundedRect {
                rect,
                color,
                radius,
                border,
            } => {
                let phys_rect: Rect = Self::scale_rect(*rect, scale);
                let phys_radius: CornerRadius = Self::scale_radius(*radius, scale);
                let phys_border: Border =
                    Border::new(border.get_width() * scale, border.get_color());
                self.raster_rounded_rect(phys_rect, *color, phys_radius, phys_border, active_clip)
            }
            DrawCommand::LinearGradientRect {
                rect,
                gradient,
                radius,
            } => {
                let phys_rect: Rect = Self::scale_rect(*rect, scale);
                let phys_radius: CornerRadius = Self::scale_radius(*radius, scale);
                self.raster_gradient_rect(phys_rect, *gradient, phys_radius, active_clip)
            }
            DrawCommand::BoxShadow {
                rect,
                radius,
                shadow,
            } => {
                let spread: f32 = shadow.get_spread_radius() * scale;
                let blur: f32 = shadow.get_blur_radius().max(1.0) * scale;
                let phys_rect: Rect = Self::scale_rect(*rect, scale);
                let shadow_rect: Rect = Rect::new(
                    phys_rect.get_x() + shadow.get_offset_x() * scale - spread - blur,
                    phys_rect.get_y() + shadow.get_offset_y() * scale - spread - blur,
                    phys_rect.get_width() + (spread + blur) * 2.0,
                    phys_rect.get_height() + (spread + blur) * 2.0,
                );
                let phys_radius: CornerRadius = Self::scale_radius(*radius, scale);
                self.raster_shadow(
                    shadow_rect,
                    phys_radius,
                    blur,
                    shadow.get_color(),
                    active_clip,
                )
            }
            DrawCommand::TextRun { rect, text, style } => {
                self.raster_text_run(*rect, text, *style, scale, active_clip)
            }
            DrawCommand::Image {
                rect,
                buffer,
                radius,
            } => {
                let phys_rect: Rect = Self::scale_rect(*rect, scale);
                let phys_radius: CornerRadius = Self::scale_radius(*radius, scale);
                self.raster_image(phys_rect, buffer, phys_radius, active_clip)
            }
            DrawCommand::Line {
                from,
                to,
                color,
                width,
            } => {
                let p0: Point = Point::new(from.get_x() * scale, from.get_y() * scale);
                let p1: Point = Point::new(to.get_x() * scale, to.get_y() * scale);
                self.raster_line(p0, p1, *color, (width * scale).max(1.0), active_clip)
            }
            DrawCommand::Circle {
                center,
                radius,
                color,
            } => {
                let c: Point = Point::new(center.get_x() * scale, center.get_y() * scale);
                self.raster_circle(c, radius * scale, *color, active_clip)
            }
            DrawCommand::PushClip(rect) => {
                let phys: Rect = Self::scale_rect(*rect, scale);
                let next: Rect = active_clip.intersect(phys).unwrap_or_default();
                self.get_mut_clip_stack().push(next);
                0
            }
            DrawCommand::PopClip => {
                if self.get_clip_stack().len() > 1 {
                    self.get_mut_clip_stack().pop();
                }
                0
            }
        }
    }

    /// Returns the active top physical clip rectangle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Renderer reference.
    ///
    /// # Returns
    ///
    /// - `Rect` - Active clip rectangle.
    fn current_clip(&self) -> Rect {
        self.get_clip_stack().last().copied().unwrap_or_else(|| {
            Rect::new(
                0.0,
                0.0,
                self.get_framebuffer().get_width() as f32,
                self.get_framebuffer().get_height() as f32,
            )
        })
    }

    /// Scales a logical rectangle into physical pixels.
    ///
    /// # Arguments
    ///
    /// - `Rect` - Logical rectangle.
    /// - `f32` - Scale factor.
    ///
    /// # Returns
    ///
    /// - `Rect` - Physical pixel rectangle.
    fn scale_rect(rect: Rect, scale: f32) -> Rect {
        Rect::new(
            rect.get_x() * scale,
            rect.get_y() * scale,
            rect.get_width() * scale,
            rect.get_height() * scale,
        )
    }

    /// Scales corner radii into physical pixels.
    ///
    /// # Arguments
    ///
    /// - `CornerRadius` - Logical radii.
    /// - `f32` - Scale factor.
    ///
    /// # Returns
    ///
    /// - `CornerRadius` - Physical pixel radii.
    fn scale_radius(radius: CornerRadius, scale: f32) -> CornerRadius {
        CornerRadius::new(
            radius.get_top_left() * scale,
            radius.get_top_right() * scale,
            radius.get_bottom_right() * scale,
            radius.get_bottom_left() * scale,
        )
    }

    /// Fast axis-aligned rectangle fill within `clip`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `Rect` - Target rectangle.
    /// - `Color` - Fill color.
    /// - `Rect` - Clipping bounds.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn fill_rect_clipped(&mut self, rect: Rect, color: Color, clip: Rect) -> usize {
        if color.get_a() == 0 {
            return 0;
        }
        let Some(vis) = rect.intersect(clip) else {
            return 0;
        };
        let fb_w: u32 = self.get_framebuffer().get_width();
        let fb_h: u32 = self.get_framebuffer().get_height();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(fb_h);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(fb_h);
        if x0 >= x1 || y0 >= y1 {
            return 0;
        }
        let span_w: usize = (x1 - x0) as usize;
        let fb: &mut Framebuffer = self.get_mut_framebuffer();
        for y in y0..y1 {
            fb.fill_span(x0, x1, y, color);
        }
        span_w.saturating_mul((y1 - y0) as usize)
    }

    /// Analytical SDF rounded rectangle and border rasterizer with 3-band interior span fast-path.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `Rect` - Physical bounds.
    /// - `Color` - Fill color.
    /// - `CornerRadius` - Physical corner radii.
    /// - `Border` - Physical border width and color.
    /// - `Rect` - Active clip bounds.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_rounded_rect(
        &mut self,
        rect: Rect,
        color: Color,
        radius: CornerRadius,
        border: Border,
        clip: Rect,
    ) -> usize {
        let Some(vis) = rect.intersect(clip) else {
            return 0;
        };
        let has_border: bool = border.get_width() > 0.0 && border.get_color().get_a() > 0;
        if radius.max_radius() <= 0.0 && !has_border {
            return self.fill_rect_clipped(rect, color, clip);
        }
        let aa: AntialiasingMode = self.get_antialiasing();
        let half_w: f32 = rect.get_width() * 0.5;
        let half_h: f32 = rect.get_height() * 0.5;
        let cx: f32 = rect.get_x() + half_w;
        let cy: f32 = rect.get_y() + half_h;
        let border_w: f32 = border.get_width();
        let fb_w: u32 = self.get_framebuffer().get_width();
        let fb_h: u32 = self.get_framebuffer().get_height();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(fb_h);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(fb_h);
        let safe_inset: f32 = radius.max_radius().max(border_w) + 1.5;
        let edge_inset: f32 = border_w + 1.5;
        let iy0: u32 = ((rect.get_y() + safe_inset).ceil().max(0.0) as u32).clamp(y0, y1);
        let iy1: u32 = ((rect.bottom() - safe_inset).floor().max(0.0) as u32).clamp(y0, y1);
        let ix0: u32 = ((rect.get_x() + edge_inset).ceil().max(0.0) as u32).clamp(x0, x1);
        let ix1: u32 = ((rect.right() - edge_inset).floor().max(0.0) as u32).clamp(x0, x1);
        let has_interior: bool = iy0 < iy1 && ix0 < ix1;
        let mut shaded: usize = 0;
        let fb: &mut Framebuffer = self.get_mut_framebuffer();
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5 - cy;
            let box_geom: (f32, f32, f32, f32) = (cx, py, half_w, half_h);
            if has_interior && y >= iy0 && y < iy1 {
                for x in x0..ix0 {
                    shaded +=
                        Self::shade_rounded_pixel(fb, (x, y), box_geom, radius, color, border, aa);
                }
                if color.get_a() > 0 {
                    fb.fill_span(ix0, ix1, y, color);
                    shaded += (ix1 - ix0) as usize;
                }
                for x in ix1..x1 {
                    shaded +=
                        Self::shade_rounded_pixel(fb, (x, y), box_geom, radius, color, border, aa);
                }
            } else {
                for x in x0..x1 {
                    shaded +=
                        Self::shade_rounded_pixel(fb, (x, y), box_geom, radius, color, border, aa);
                }
            }
        }
        shaded
    }

    /// Evaluates analytical SDF fill and border coverage for a single edge/corner pixel.
    ///
    /// # Arguments
    ///
    /// - `&mut Framebuffer` - Target framebuffer.
    /// - `(u32, u32)` - Pixel `(x, y)` coordinates.
    /// - `(f32, f32, f32, f32)` - Box geometry `(center_x, rel_py, half_w, half_h)`.
    /// - `CornerRadius` - Physical corner radii.
    /// - `Color` - Fill color.
    /// - `Border` - Border width and color.
    /// - `AntialiasingMode` - Active AA policy.
    ///
    /// # Returns
    ///
    /// - `usize` - `1` if the pixel was shaded, `0` otherwise.
    fn shade_rounded_pixel(
        fb: &mut Framebuffer,
        pos: (u32, u32),
        geom: (f32, f32, f32, f32),
        radius: CornerRadius,
        color: Color,
        border: Border,
        aa: AntialiasingMode,
    ) -> usize {
        let (x, y): (u32, u32) = pos;
        let (cx, py, half_w, half_h): (f32, f32, f32, f32) = geom;
        let px: f32 = (x as f32) + 0.5 - cx;
        let corner_r: f32 = if px < 0.0 {
            if py < 0.0 {
                radius.get_top_left()
            } else {
                radius.get_bottom_left()
            }
        } else if py < 0.0 {
            radius.get_top_right()
        } else {
            radius.get_bottom_right()
        };
        let dist: f32 = rounded_box_sdf(px, py, half_w, half_h, corner_r);
        let outer_cov: f32 = sdf_coverage(dist, aa);
        if outer_cov <= 0.0 {
            return 0;
        }
        if border.get_width() > 0.0 && border.get_color().get_a() > 0 {
            let inner_cov: f32 = sdf_coverage(dist + border.get_width(), aa);
            let border_cov: f32 = (outer_cov - inner_cov).clamp(0.0, 1.0);
            if inner_cov > 0.0 && color.get_a() > 0 {
                fb.blend_pixel(x, y, color.multiply_alpha(inner_cov));
            }
            if border_cov > 0.0 {
                fb.blend_pixel(x, y, border.get_color().multiply_alpha(border_cov));
            }
            return 1;
        }
        if color.get_a() > 0 {
            fb.blend_pixel(x, y, color.multiply_alpha(outer_cov));
            return 1;
        }
        0
    }

    /// Rasterizes a two-stop linear gradient inside an SDF rounded rectangle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `Rect` - Physical bounds.
    /// - `LinearGradient` - Gradient angle and stops.
    /// - `CornerRadius` - Physical corner radii.
    /// - `Rect` - Active clip bounds.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_gradient_rect(
        &mut self,
        rect: Rect,
        gradient: LinearGradient,
        radius: CornerRadius,
        clip: Rect,
    ) -> usize {
        let Some(vis) = rect.intersect(clip) else {
            return 0;
        };
        let aa: AntialiasingMode = self.get_antialiasing();
        let rad: f32 = gradient.get_angle_deg().to_radians();
        let dir_x: f32 = rad.sin();
        let dir_y: f32 = rad.cos();
        let half_w: f32 = rect.get_width() * 0.5;
        let half_h: f32 = rect.get_height() * 0.5;
        let cx: f32 = rect.get_x() + half_w;
        let cy: f32 = rect.get_y() + half_h;
        let inv_w: f32 = if rect.get_width() > 0.0 {
            1.0 / rect.get_width()
        } else {
            0.0
        };
        let inv_h: f32 = if rect.get_height() > 0.0 {
            1.0 / rect.get_height()
        } else {
            0.0
        };
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(self.get_framebuffer().get_width());
        let y0: u32 =
            (vis.get_y().floor().max(0.0) as u32).min(self.get_framebuffer().get_height());
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(self.get_framebuffer().get_width());
        let y1: u32 =
            (vis.bottom().ceil().max(0.0) as u32).min(self.get_framebuffer().get_height());
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5 - cy;
            let ny: f32 = ((y as f32) - rect.get_y()) * inv_h;
            for x in x0..x1 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let dist: f32 = rounded_box_sdf(px, py, half_w, half_h, radius.max_radius());
                let cov: f32 = sdf_coverage(dist, aa);
                if cov <= 0.0 {
                    continue;
                }
                let nx: f32 = ((x as f32) - rect.get_x()) * inv_w;
                let t: f32 = ((nx - 0.5) * dir_x + (ny - 0.5) * dir_y + 0.5).clamp(0.0, 1.0);
                let sample: Color = gradient
                    .get_start_color()
                    .lerp(gradient.get_end_color(), t)
                    .multiply_alpha(cov);
                self.get_mut_framebuffer().blend_pixel(x, y, sample);
                shaded += 1;
            }
        }
        shaded
    }

    /// Rasterizes a soft falloff drop shadow using an analytical SDF blur approximation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `Rect` - Expanded shadow bounds.
    /// - `CornerRadius` - Physical corner radii.
    /// - `f32` - Blur radius in physical pixels.
    /// - `Color` - Shadow tint color.
    /// - `Rect` - Active clip bounds.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_shadow(
        &mut self,
        rect: Rect,
        radius: CornerRadius,
        blur: f32,
        color: Color,
        clip: Rect,
    ) -> usize {
        let Some(vis) = rect.intersect(clip) else {
            return 0;
        };
        let inner_w: f32 = (rect.get_width() * 0.5 - blur).max(1.0);
        let inner_h: f32 = (rect.get_height() * 0.5 - blur).max(1.0);
        let cx: f32 = rect.get_x() + rect.get_width() * 0.5;
        let cy: f32 = rect.get_y() + rect.get_height() * 0.5;
        let inv_blur: f32 = 1.0 / blur.max(1.0);
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(self.get_framebuffer().get_width());
        let y0: u32 =
            (vis.get_y().floor().max(0.0) as u32).min(self.get_framebuffer().get_height());
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(self.get_framebuffer().get_width());
        let y1: u32 =
            (vis.bottom().ceil().max(0.0) as u32).min(self.get_framebuffer().get_height());
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5 - cy;
            for x in x0..x1 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let dist: f32 = rounded_box_sdf(px, py, inner_w, inner_h, radius.max_radius());
                if dist >= blur {
                    continue;
                }
                let falloff: f32 = if dist <= 0.0 {
                    1.0
                } else {
                    let norm: f32 = 1.0 - (dist * inv_blur).clamp(0.0, 1.0);
                    norm * norm
                };
                if falloff > 0.0 {
                    self.get_mut_framebuffer()
                        .blend_pixel(x, y, color.multiply_alpha(falloff));
                    shaded += 1;
                }
            }
        }
        shaded
    }

    /// Rasterizes a laid-out UTF-8 text run using the cached `GlyphAtlas`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `Rect` - Logical text bounding box.
    /// - `&str` - Text content.
    /// - `TextStyle` - Typography style.
    /// - `f32` - Active DPI scale factor.
    /// - `Rect` - Active physical clip bounds.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_text_run(
        &mut self,
        rect: Rect,
        text: &str,
        style: TextStyle,
        scale: f32,
        clip: Rect,
    ) -> usize {
        let layout: TextLayoutResult =
            self.get_measurer()
                .measure(text, style, Some(rect.get_width().max(1.0)));
        let phys_font_size: f32 = style.get_font_size() * scale;
        let line_h_phys: f32 = layout.get_line_height_px() * scale;
        let box_w_phys: f32 = rect.get_width() * scale;
        let color: Color = style.get_color();
        let clip_x0: i32 = clip.get_x().ceil().max(0.0) as i32;
        let clip_y0: i32 = clip.get_y().ceil().max(0.0) as i32;
        let clip_x1: i32 =
            (clip.right().floor().max(0.0) as i32).min(self.get_framebuffer().get_width() as i32);
        let clip_y1: i32 =
            (clip.bottom().floor().max(0.0) as i32).min(self.get_framebuffer().get_height() as i32);
        let mut shaded: usize = 0;
        let Self {
            framebuffer,
            glyph_atlas,
            font_provider,
            ..
        } = self;
        for (line_idx, line) in layout.get_lines().iter().enumerate() {
            let line_w_phys: f32 = line.get_width() * scale;
            let align_x: f32 = match style.get_align() {
                TextAlign::Left => 0.0,
                TextAlign::Center => ((box_w_phys - line_w_phys) * 0.5).max(0.0),
                TextAlign::Right => (box_w_phys - line_w_phys).max(0.0),
            };
            let mut pen_x: f32 = rect.get_x() * scale + align_x;
            let pen_y: f32 = rect.get_y() * scale + (line_idx as f32) * line_h_phys;
            for ch in line.get_text().chars() {
                let adv_phys: f32 = measure_char_advance(ch, style) * scale;
                if ch != ' ' {
                    let key: GlyphKey =
                        GlyphKey::from_style(ch, phys_font_size, style.get_weight());
                    let glyph: &RasterizedGlyph =
                        glyph_atlas.get_or_rasterize_with(key, font_provider);
                    let gx0: i32 = pen_x.round() as i32;
                    let gy0: i32 =
                        (pen_y + (line_h_phys - (glyph.get_height() as f32)) * 0.5).round() as i32;
                    let gw: i32 = glyph.get_width() as i32;
                    let gh: i32 = glyph.get_height() as i32;
                    let py_start: i32 = (clip_y0 - gy0).max(0);
                    let py_end: i32 = (clip_y1 - gy0).min(gh);
                    let px_start: i32 = (clip_x0 - gx0).max(0);
                    let px_end: i32 = (clip_x1 - gx0).min(gw);
                    if py_start < py_end && px_start < px_end {
                        let cov_bytes: &[u8] = glyph.get_coverage().as_slice();
                        let gw_usize: usize = gw as usize;
                        for py in py_start..py_end {
                            let dy: u32 = (gy0 + py) as u32;
                            let row_off: usize = (py as usize) * gw_usize;
                            for px in px_start..px_end {
                                let alpha_byte: u8 = cov_bytes[row_off + (px as usize)];
                                if alpha_byte > 0 {
                                    let eff_a: u8 = (((color.get_a() as u32) * (alpha_byte as u32)
                                        + 127)
                                        / 255)
                                        as u8;
                                    framebuffer.blend_pixel(
                                        (gx0 + px) as u32,
                                        dy,
                                        Color::new(
                                            color.get_r(),
                                            color.get_g(),
                                            color.get_b(),
                                            eff_a,
                                        ),
                                    );
                                    shaded += 1;
                                }
                            }
                        }
                    }
                }
                pen_x += adv_phys;
            }
        }
        shaded
    }

    /// Blits a decoded `ImageBuffer` into `rect` with nearest-neighbor scaling and rounded clip.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `Rect` - Physical destination bounds.
    /// - `&ImageBuffer` - Source RGBA8 image.
    /// - `CornerRadius` - Physical corner radii.
    /// - `Rect` - Active clip bounds.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_image(
        &mut self,
        rect: Rect,
        buffer: &ImageBuffer,
        radius: CornerRadius,
        clip: Rect,
    ) -> usize {
        if buffer.get_width() == 0 || buffer.get_height() == 0 {
            return 0;
        }
        let Some(vis) = rect.intersect(clip) else {
            return 0;
        };
        let aa: AntialiasingMode = self.get_antialiasing();
        let half_w: f32 = rect.get_width() * 0.5;
        let half_h: f32 = rect.get_height() * 0.5;
        let cx: f32 = rect.get_x() + half_w;
        let cy: f32 = rect.get_y() + half_h;
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(self.get_framebuffer().get_width());
        let y0: u32 =
            (vis.get_y().floor().max(0.0) as u32).min(self.get_framebuffer().get_height());
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(self.get_framebuffer().get_width());
        let y1: u32 =
            (vis.bottom().ceil().max(0.0) as u32).min(self.get_framebuffer().get_height());
        let src_w: u32 = buffer.get_width();
        let src_h: u32 = buffer.get_height();
        let src_bytes: &[u8] = buffer.get_pixels().as_slice();
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let v: f32 = ((y as f32) - rect.get_y()) / rect.get_height().max(1.0);
            let sy: u32 = ((v * (src_h as f32)).floor() as u32).min(src_h - 1);
            let py: f32 = (y as f32) + 0.5 - cy;
            for x in x0..x1 {
                let px: f32 = (x as f32) + 0.5 - cx;
                let cov: f32 = if radius.max_radius() > 0.0 {
                    sdf_coverage(
                        rounded_box_sdf(px, py, half_w, half_h, radius.max_radius()),
                        aa,
                    )
                } else {
                    1.0
                };
                if cov <= 0.0 {
                    continue;
                }
                let u: f32 = ((x as f32) - rect.get_x()) / rect.get_width().max(1.0);
                let sx: u32 = ((u * (src_w as f32)).floor() as u32).min(src_w - 1);
                let idx: usize = ((sy as usize) * (src_w as usize) + (sx as usize)) * 4;
                if idx + 4 <= src_bytes.len() {
                    let sample: Color = Color::new(
                        src_bytes[idx],
                        src_bytes[idx + 1],
                        src_bytes[idx + 2],
                        src_bytes[idx + 3],
                    )
                    .multiply_alpha(cov);
                    self.get_mut_framebuffer().blend_pixel(x, y, sample);
                    shaded += 1;
                }
            }
        }
        shaded
    }

    /// Rasterizes an anti-aliased line segment between `from` and `to`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `Point` - Physical start point.
    /// - `Point` - Physical end point.
    /// - `Color` - Stroke color.
    /// - `f32` - Physical stroke width.
    /// - `Rect` - Active clip bounds.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_line(
        &mut self,
        from: Point,
        to: Point,
        color: Color,
        width: f32,
        clip: Rect,
    ) -> usize {
        let half_w: f32 = width * 0.5;
        let min_x: f32 = from.get_x().min(to.get_x()) - half_w - 1.0;
        let min_y: f32 = from.get_y().min(to.get_y()) - half_w - 1.0;
        let max_x: f32 = from.get_x().max(to.get_x()) + half_w + 1.0;
        let max_y: f32 = from.get_y().max(to.get_y()) + half_w + 1.0;
        let Some(vis) = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y).intersect(clip)
        else {
            return 0;
        };
        let dx: f32 = to.get_x() - from.get_x();
        let dy: f32 = to.get_y() - from.get_y();
        let len_sq: f32 = (dx * dx + dy * dy).max(1e-6);
        let aa: AntialiasingMode = self.get_antialiasing();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(self.get_framebuffer().get_width());
        let y0: u32 =
            (vis.get_y().floor().max(0.0) as u32).min(self.get_framebuffer().get_height());
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(self.get_framebuffer().get_width());
        let y1: u32 =
            (vis.bottom().ceil().max(0.0) as u32).min(self.get_framebuffer().get_height());
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5;
            for x in x0..x1 {
                let px: f32 = (x as f32) + 0.5;
                let t: f32 = (((px - from.get_x()) * dx + (py - from.get_y()) * dy) / len_sq)
                    .clamp(0.0, 1.0);
                let proj_x: f32 = from.get_x() + t * dx;
                let proj_y: f32 = from.get_y() + t * dy;
                let dist: f32 =
                    ((px - proj_x) * (px - proj_x) + (py - proj_y) * (py - proj_y)).sqrt() - half_w;
                let cov: f32 = sdf_coverage(dist, aa);
                if cov > 0.0 {
                    self.get_mut_framebuffer()
                        .blend_pixel(x, y, color.multiply_alpha(cov));
                    shaded += 1;
                }
            }
        }
        shaded
    }

    /// Rasterizes an analytical SDF anti-aliased filled circle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `Point` - Physical center point.
    /// - `f32` - Physical radius.
    /// - `Color` - Fill color.
    /// - `Rect` - Active clip bounds.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_circle(&mut self, center: Point, radius: f32, color: Color, clip: Rect) -> usize {
        if radius <= 0.0 {
            return 0;
        }
        let bounds: Rect = Rect::new(
            center.get_x() - radius - 1.0,
            center.get_y() - radius - 1.0,
            (radius + 1.0) * 2.0,
            (radius + 1.0) * 2.0,
        );
        let Some(vis) = bounds.intersect(clip) else {
            return 0;
        };
        let aa: AntialiasingMode = self.get_antialiasing();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(self.get_framebuffer().get_width());
        let y0: u32 =
            (vis.get_y().floor().max(0.0) as u32).min(self.get_framebuffer().get_height());
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(self.get_framebuffer().get_width());
        let y1: u32 =
            (vis.bottom().ceil().max(0.0) as u32).min(self.get_framebuffer().get_height());
        let mut shaded: usize = 0;
        for y in y0..y1 {
            let dy: f32 = (y as f32) + 0.5 - center.get_y();
            for x in x0..x1 {
                let dx: f32 = (x as f32) + 0.5 - center.get_x();
                let dist: f32 = (dx * dx + dy * dy).sqrt() - radius;
                let cov: f32 = sdf_coverage(dist, aa);
                if cov > 0.0 {
                    self.get_mut_framebuffer()
                        .blend_pixel(x, y, color.multiply_alpha(cov));
                    shaded += 1;
                }
            }
        }
        shaded
    }
}
