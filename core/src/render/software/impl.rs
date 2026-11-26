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
                let phys_rect: Rect = snap_physical_rect(*rect, scale);
                let phys_radius: CornerRadius = Self::scale_radius(*radius, scale);
                let bw: f32 = if border.get_width() > 0.0 {
                    (border.get_width() * scale).round().max(1.0)
                } else {
                    0.0
                };
                let phys_border: Border = Border::new(bw, border.get_color());
                self.raster_rounded_rect(phys_rect, *color, phys_radius, phys_border, active_clip)
            }
            DrawCommand::LinearGradientRect {
                rect,
                gradient,
                radius,
            } => {
                let phys_rect: Rect = snap_physical_rect(*rect, scale);
                let phys_radius: CornerRadius = Self::scale_radius(*radius, scale);
                self.raster_gradient_rect(phys_rect, *gradient, phys_radius, active_clip)
            }
            DrawCommand::BoxShadow {
                rect,
                radius,
                shadow,
            } => {
                let spread: f32 = shadow.get_spread_radius() * scale;
                let blur: f32 = (shadow.get_blur_radius().max(1.0) * scale).min(24.0);
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
            DrawCommand::Triangle { p0, p1, p2, color } => {
                let v0: Point = Point::new(p0.get_x() * scale, p0.get_y() * scale);
                let v1: Point = Point::new(p1.get_x() * scale, p1.get_y() * scale);
                let v2: Point = Point::new(p2.get_x() * scale, p2.get_y() * scale);
                self.raster_triangle(v0, v1, v2, *color, active_clip)
            }
            DrawCommand::SpriteRegion {
                rect,
                buffer,
                src_rect,
                tint,
            } => {
                let phys_rect: Rect = Self::scale_rect(*rect, scale);
                self.raster_sprite_region(phys_rect, buffer, *src_rect, *tint, active_clip)
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
        let cx0: u32 = ((rect.get_x() + safe_inset).ceil().max(0.0) as u32).clamp(x0, x1);
        let cx1: u32 = ((rect.right() - safe_inset).floor().max(0.0) as u32).clamp(x0, x1);
        let has_interior: bool = iy0 < iy1 && ix0 < ix1;
        let has_center_band: bool = cx0 < cx1;
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
            } else if has_center_band {
                for x in x0..cx0 {
                    shaded +=
                        Self::shade_rounded_pixel(fb, (x, y), box_geom, radius, color, border, aa);
                }
                let dist_y: f32 = py.abs() - half_h;
                let outer_cov: f32 = sdf_coverage(dist_y, aa);
                if outer_cov > 0.0 {
                    if has_border {
                        let inner_cov: f32 = sdf_coverage(dist_y + border_w, aa);
                        let mixed: Color = composite_fill_and_border(
                            color,
                            border.get_color(),
                            outer_cov,
                            inner_cov,
                        );
                        if mixed.get_a() > 0 {
                            fb.fill_span(cx0, cx1, y, mixed);
                        }
                        shaded += (cx1 - cx0) as usize;
                    } else if color.get_a() > 0 {
                        fb.fill_span(cx0, cx1, y, color.multiply_alpha(outer_cov));
                        shaded += (cx1 - cx0) as usize;
                    }
                }
                for x in cx1..x1 {
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
            let mixed: Color =
                composite_fill_and_border(color, border.get_color(), outer_cov, inner_cov);
            if mixed.get_a() > 0 {
                fb.blend_pixel(x, y, mixed);
                return 1;
            }
            return 0;
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
        let r_max: f32 = radius.max_radius();
        let inner_inset: f32 = blur + r_max;
        let sy0: u32 = ((rect.get_y() + inner_inset).ceil().max(0.0) as u32).clamp(y0, y1);
        let sy1: u32 = ((rect.bottom() - inner_inset).floor().max(0.0) as u32).clamp(y0, y1);
        let sx0: u32 = ((rect.get_x() + inner_inset).ceil().max(0.0) as u32).clamp(x0, x1);
        let sx1: u32 = ((rect.right() - inner_inset).floor().max(0.0) as u32).clamp(x0, x1);
        let has_mid_band: bool = sx0 < sx1;
        let mut shaded: usize = 0;
        let fb: &mut Framebuffer = self.get_mut_framebuffer();
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5 - cy;
            if has_mid_band && y >= sy0 && y < sy1 {
                for x in x0..sx0 {
                    let dist: f32 = ((x as f32) + 0.5 - cx).abs() - inner_w;
                    if dist < blur {
                        let falloff: f32 = if dist <= 0.0 {
                            1.0
                        } else {
                            let norm: f32 = 1.0 - (dist * inv_blur).clamp(0.0, 1.0);
                            norm * norm * (3.0 - 2.0 * norm)
                        };
                        if falloff > 0.0 {
                            fb.blend_pixel(x, y, color.multiply_alpha(falloff));
                            shaded += 1;
                        }
                    }
                }
                for x in sx1..x1 {
                    let dist: f32 = ((x as f32) + 0.5 - cx).abs() - inner_w;
                    if dist < blur {
                        let falloff: f32 = if dist <= 0.0 {
                            1.0
                        } else {
                            let norm: f32 = 1.0 - (dist * inv_blur).clamp(0.0, 1.0);
                            norm * norm * (3.0 - 2.0 * norm)
                        };
                        if falloff > 0.0 {
                            fb.blend_pixel(x, y, color.multiply_alpha(falloff));
                            shaded += 1;
                        }
                    }
                }
            } else if has_mid_band {
                for x in x0..sx0 {
                    let px: f32 = (x as f32) + 0.5 - cx;
                    let dist: f32 = rounded_box_sdf(px, py, inner_w, inner_h, r_max);
                    if dist < blur {
                        let falloff: f32 = if dist <= 0.0 {
                            1.0
                        } else {
                            let norm: f32 = 1.0 - (dist * inv_blur).clamp(0.0, 1.0);
                            norm * norm * (3.0 - 2.0 * norm)
                        };
                        if falloff > 0.0 {
                            fb.blend_pixel(x, y, color.multiply_alpha(falloff));
                            shaded += 1;
                        }
                    }
                }
                let dist_y: f32 = py.abs() - inner_h;
                if dist_y < blur {
                    let falloff: f32 = if dist_y <= 0.0 {
                        1.0
                    } else {
                        let norm: f32 = 1.0 - (dist_y * inv_blur).clamp(0.0, 1.0);
                        norm * norm * (3.0 - 2.0 * norm)
                    };
                    if falloff > 0.0 {
                        fb.fill_span(sx0, sx1, y, color.multiply_alpha(falloff));
                        shaded += (sx1 - sx0) as usize;
                    }
                }
                for x in sx1..x1 {
                    let px: f32 = (x as f32) + 0.5 - cx;
                    let dist: f32 = rounded_box_sdf(px, py, inner_w, inner_h, r_max);
                    if dist < blur {
                        let falloff: f32 = if dist <= 0.0 {
                            1.0
                        } else {
                            let norm: f32 = 1.0 - (dist * inv_blur).clamp(0.0, 1.0);
                            norm * norm * (3.0 - 2.0 * norm)
                        };
                        if falloff > 0.0 {
                            fb.blend_pixel(x, y, color.multiply_alpha(falloff));
                            shaded += 1;
                        }
                    }
                }
            } else {
                for x in x0..x1 {
                    let px: f32 = (x as f32) + 0.5 - cx;
                    let dist: f32 = rounded_box_sdf(px, py, inner_w, inner_h, r_max);
                    if dist >= blur {
                        continue;
                    }
                    let falloff: f32 = if dist <= 0.0 {
                        1.0
                    } else {
                        let norm: f32 = 1.0 - (dist * inv_blur).clamp(0.0, 1.0);
                        norm * norm * (3.0 - 2.0 * norm)
                    };
                    if falloff > 0.0 {
                        fb.blend_pixel(x, y, color.multiply_alpha(falloff));
                        shaded += 1;
                    }
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
        if text.is_empty() || style.get_color().get_a() == 0 {
            return 0;
        }
        let phys_rect: Rect = Self::scale_rect(rect, scale);
        let pad: f32 = style.get_font_size() * scale * 0.5;
        let expanded_bounds: Rect = Rect::new(
            phys_rect.get_x() - pad,
            phys_rect.get_y() - pad,
            phys_rect.get_width() + pad * 2.0,
            phys_rect.get_height() + pad * 2.0,
        );
        if expanded_bounds.intersect(clip).is_none() {
            return 0;
        }
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
        if clip_x0 >= clip_x1 || clip_y0 >= clip_y1 {
            return 0;
        }
        let letter_sp: f32 = style.get_letter_spacing() * scale;
        let space_adv: f32 = measure_char_advance(' ', style) * scale;
        let mut shaded: usize = 0;
        let Self {
            framebuffer,
            glyph_atlas,
            font_provider,
            ..
        } = self;
        for (line_idx, line) in layout.get_lines().iter().enumerate() {
            let pen_y: f32 = rect.get_y() * scale + (line_idx as f32) * line_h_phys;
            if (pen_y + line_h_phys) < (clip_y0 as f32) || pen_y > (clip_y1 as f32) {
                continue;
            }
            let align_x: f32 = match style.get_align() {
                TextAlign::Left => 0.0,
                TextAlign::Center | TextAlign::Right => {
                    let mut line_w_phys: f32 = 0.0;
                    for ch in line.get_text().chars() {
                        if ch == ' ' {
                            line_w_phys += space_adv + letter_sp;
                        } else {
                            let key: GlyphKey =
                                GlyphKey::from_style(ch, phys_font_size, style.get_weight());
                            let glyph: &RasterizedGlyph =
                                glyph_atlas.get_or_rasterize_with(key, font_provider);
                            line_w_phys += (glyph.get_advance() as f32).max(1.0) + letter_sp;
                        }
                    }
                    if style.get_align() == TextAlign::Center {
                        ((box_w_phys - line_w_phys) * 0.5).max(0.0)
                    } else {
                        (box_w_phys - line_w_phys).max(0.0)
                    }
                }
            };
            let mut pen_x: f32 = rect.get_x() * scale + align_x;
            for ch in line.get_text().chars() {
                if ch == ' ' {
                    pen_x += space_adv + letter_sp;
                    continue;
                }
                if pen_x > (clip_x1 as f32) {
                    break;
                }
                let key: GlyphKey = GlyphKey::from_style(ch, phys_font_size, style.get_weight());
                let glyph: &RasterizedGlyph = glyph_atlas.get_or_rasterize_with(key, font_provider);
                let adv_phys: f32 = (glyph.get_advance() as f32).max(1.0) + letter_sp;
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
                    let dx: u32 = (gx0 + px_start) as u32;
                    let start_u: usize = px_start as usize;
                    let end_u: usize = px_end as usize;
                    for py in py_start..py_end {
                        let dy: u32 = (gy0 + py) as u32;
                        let row_off: usize = (py as usize) * gw_usize;
                        let row_slice: &[u8] = &cov_bytes[row_off + start_u..row_off + end_u];
                        shaded += framebuffer.blend_glyph_row(dx, dy, row_slice, color);
                    }
                }
                pen_x += adv_phys;
            }
        }
        shaded
    }

    /// Blits a single bilinear-filtered image scanline with fast opaque 4-tap bypass.
    ///
    /// # Arguments
    ///
    /// - `(usize, usize, f32, f32, f32)` - `(y, x0, rect_y, inv_dst_h, src_h_f)`.
    /// - `(usize, usize)` - `(src_w_usize, max_sy)`.
    /// - `&[(usize, usize, u32)]` - Precomputed horizontal bilinear LUT.
    /// - `&[u8]` - Source RGBA8 pixels.
    /// - `&mut [u8]` - Destination scanline RGBA8 slice.
    fn blit_bilinear_row(
        row_meta: (usize, usize, f32, f32, f32),
        src_meta: (usize, usize),
        x_lut: &[(usize, usize, u32)],
        src_bytes: &[u8],
        dst_row: &mut [u8],
    ) {
        let (y, x0_usize, rect_y, inv_dst_h, src_h_f): (usize, usize, f32, f32, f32) = row_meta;
        let (src_w_usize, max_sy): (usize, usize) = src_meta;
        let v: f32 = (((y as f32) + 0.5 - rect_y) * inv_dst_h).clamp(0.0, 1.0);
        let fy: f32 = (v * src_h_f - 0.5).clamp(0.0, max_sy as f32);
        let sy0: usize = (fy.floor() as usize).min(max_sy);
        let sy1: usize = (sy0 + 1).min(max_sy);
        let wy: u32 = ((fy - (sy0 as f32)) * 256.0).round().clamp(0.0, 256.0) as u32;
        let iwy: u32 = 256 - wy;
        let row0: usize = sy0 * src_w_usize;
        let row1: usize = sy1 * src_w_usize;
        for (dx, &(sx0, sx1, wx)) in x_lut.iter().enumerate() {
            let iwx: u32 = 256 - wx;
            let w00: u32 = iwx * iwy;
            let w10: u32 = wx * iwy;
            let w01: u32 = iwx * wy;
            let w11: u32 = wx * wy;
            let i00: usize = (row0 + sx0) * 4;
            let i10: usize = (row0 + sx1) * 4;
            let i01: usize = (row1 + sx0) * 4;
            let i11: usize = (row1 + sx1) * 4;
            let a00: u32 = src_bytes[i00 + 3] as u32;
            let a10: u32 = src_bytes[i10 + 3] as u32;
            let a01: u32 = src_bytes[i01 + 3] as u32;
            let a11: u32 = src_bytes[i11 + 3] as u32;
            let didx: usize = (x0_usize + dx) * 4;
            if a00 == 255 && a10 == 255 && a01 == 255 && a11 == 255 {
                dst_row[didx] = (((src_bytes[i00] as u32) * w00
                    + (src_bytes[i10] as u32) * w10
                    + (src_bytes[i01] as u32) * w01
                    + (src_bytes[i11] as u32) * w11
                    + 32768)
                    >> 16) as u8;
                dst_row[didx + 1] = (((src_bytes[i00 + 1] as u32) * w00
                    + (src_bytes[i10 + 1] as u32) * w10
                    + (src_bytes[i01 + 1] as u32) * w01
                    + (src_bytes[i11 + 1] as u32) * w11
                    + 32768)
                    >> 16) as u8;
                dst_row[didx + 2] = (((src_bytes[i00 + 2] as u32) * w00
                    + (src_bytes[i10 + 2] as u32) * w10
                    + (src_bytes[i01 + 2] as u32) * w01
                    + (src_bytes[i11 + 2] as u32) * w11
                    + 32768)
                    >> 16) as u8;
                dst_row[didx + 3] = 255;
                continue;
            }
            let a_val: u32 = (a00 * w00 + a10 * w10 + a01 * w01 + a11 * w11 + 32768) >> 16;
            if a_val == 0 {
                continue;
            }
            let r: u32 = ((src_bytes[i00] as u32) * w00
                + (src_bytes[i10] as u32) * w10
                + (src_bytes[i01] as u32) * w01
                + (src_bytes[i11] as u32) * w11
                + 32768)
                >> 16;
            let g: u32 = ((src_bytes[i00 + 1] as u32) * w00
                + (src_bytes[i10 + 1] as u32) * w10
                + (src_bytes[i01 + 1] as u32) * w01
                + (src_bytes[i11 + 1] as u32) * w11
                + 32768)
                >> 16;
            let b: u32 = ((src_bytes[i00 + 2] as u32) * w00
                + (src_bytes[i10 + 2] as u32) * w10
                + (src_bytes[i01 + 2] as u32) * w01
                + (src_bytes[i11 + 2] as u32) * w11
                + 32768)
                >> 16;
            let inv_a: u32 = 255 - a_val;
            dst_row[didx] = ((r * a_val + (dst_row[didx] as u32) * inv_a + 127) / 255) as u8;
            dst_row[didx + 1] =
                ((g * a_val + (dst_row[didx + 1] as u32) * inv_a + 127) / 255) as u8;
            dst_row[didx + 2] =
                ((b * a_val + (dst_row[didx + 2] as u32) * inv_a + 127) / 255) as u8;
            dst_row[didx + 3] = 255;
        }
    }

    /// Blits a decoded `ImageBuffer` into `rect` with bilinear anti-aliased filtering and SDF rounded clip.
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
        let fb_w: u32 = self.get_framebuffer().get_width();
        let fb_h: u32 = self.get_framebuffer().get_height();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(fb_h);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(fb_h);
        let src_w: u32 = buffer.get_width();
        let src_h: u32 = buffer.get_height();
        let src_bytes: &[u8] = buffer.get_pixels().as_slice();
        let inv_dst_w: f32 = 1.0 / rect.get_width().max(1.0);
        let inv_dst_h: f32 = 1.0 / rect.get_height().max(1.0);
        let use_bilinear: bool = aa != AntialiasingMode::None
            && ((rect.get_width() - (src_w as f32)).abs() > 0.5
                || (rect.get_height() - (src_h as f32)).abs() > 0.5);
        let has_radius: bool = radius.max_radius() > 0.0;
        let max_r: f32 = radius.max_radius();
        let max_sx: usize = (src_w as usize).saturating_sub(1);
        let max_sy: usize = (src_h as usize).saturating_sub(1);
        let src_w_usize: usize = src_w as usize;
        let mut shaded: usize = 0;
        let fb: &mut Framebuffer = self.get_mut_framebuffer();
        if !has_radius && use_bilinear && x0 < x1 && y0 < y1 {
            let span_w: usize = (x1 - x0) as usize;
            let mut x_lut: Vec<(usize, usize, u32)> = Vec::with_capacity(span_w);
            for x in x0..x1 {
                let u: f32 = (((x as f32) + 0.5 - rect.get_x()) * inv_dst_w).clamp(0.0, 1.0);
                let fx: f32 = (u * (src_w as f32) - 0.5).clamp(0.0, max_sx as f32);
                let sx0: usize = (fx.floor() as usize).min(max_sx);
                let sx1: usize = (sx0 + 1).min(max_sx);
                let wx: u32 = ((fx - (sx0 as f32)) * 256.0).round().clamp(0.0, 256.0) as u32;
                x_lut.push((sx0, sx1, wx));
            }
            let fb_w_usize: usize = fb_w as usize;
            let dst_pixels: &mut [u8] = fb.get_mut_pixels().as_mut_slice();
            let span_h: usize = (y1 - y0) as usize;
            let x_lut_ref: &[(usize, usize, u32)] = x_lut.as_slice();
            let rect_y: f32 = rect.get_y();
            let src_h_f: f32 = src_h as f32;
            let x0_usize: usize = x0 as usize;
            let y0_usize: usize = y0 as usize;
            let row_stride: usize = fb_w_usize * 4;
            let band_slice: &mut [u8] =
                &mut dst_pixels[(y0_usize * row_stride)..((y1 as usize) * row_stride)];
            #[cfg(not(target_arch = "wasm32"))]
            if span_w * span_h >= 65_536 && span_h >= 32 {
                let workers: usize = available_parallelism()
                    .map(usize::from)
                    .unwrap_or(4)
                    .clamp(1, 8)
                    .min(span_h / 16);
                let rows_per: usize = span_h.div_ceil(workers);
                scope(|s: &thread::Scope<'_, '_>| {
                    for (idx, chunk) in band_slice.chunks_mut(rows_per * row_stride).enumerate() {
                        let start_y: usize = y0_usize + idx * rows_per;
                        let _: ScopedJoinHandle<'_, ()> = s.spawn(move || {
                            for (ly, row_buf) in chunk.chunks_mut(row_stride).enumerate() {
                                Self::blit_bilinear_row(
                                    (start_y + ly, x0_usize, rect_y, inv_dst_h, src_h_f),
                                    (src_w_usize, max_sy),
                                    x_lut_ref,
                                    src_bytes,
                                    row_buf,
                                );
                            }
                        });
                    }
                });
            } else {
                for (ly, row_buf) in band_slice.chunks_mut(row_stride).enumerate() {
                    Self::blit_bilinear_row(
                        (y0_usize + ly, x0_usize, rect_y, inv_dst_h, src_h_f),
                        (src_w_usize, max_sy),
                        x_lut_ref,
                        src_bytes,
                        row_buf,
                    );
                }
            }
            #[cfg(target_arch = "wasm32")]
            for (ly, row_buf) in band_slice.chunks_mut(row_stride).enumerate() {
                Self::blit_bilinear_row(
                    (y0_usize + ly, x0_usize, rect_y, inv_dst_h, src_h_f),
                    (src_w_usize, max_sy),
                    x_lut_ref,
                    src_bytes,
                    row_buf,
                );
            }
            shaded = span_w * span_h;
            return shaded;
        }
        for y in y0..y1 {
            let v: f32 = (((y as f32) + 0.5 - rect.get_y()) * inv_dst_h).clamp(0.0, 1.0);
            let py: f32 = (y as f32) + 0.5 - cy;
            if use_bilinear {
                let fy: f32 = (v * (src_h as f32) - 0.5).clamp(0.0, max_sy as f32);
                let sy0: usize = (fy.floor() as usize).min(max_sy);
                let sy1: usize = (sy0 + 1).min(max_sy);
                let wy: f32 = fy - (sy0 as f32);
                let iwy: f32 = 1.0 - wy;
                let row0: usize = sy0 * src_w_usize;
                let row1: usize = sy1 * src_w_usize;
                for x in x0..x1 {
                    let cov: f32 = if has_radius {
                        let px: f32 = (x as f32) + 0.5 - cx;
                        sdf_coverage(rounded_box_sdf(px, py, half_w, half_h, max_r), aa)
                    } else {
                        1.0
                    };
                    if cov <= 0.0 {
                        continue;
                    }
                    let u: f32 = (((x as f32) + 0.5 - rect.get_x()) * inv_dst_w).clamp(0.0, 1.0);
                    let fx: f32 = (u * (src_w as f32) - 0.5).clamp(0.0, max_sx as f32);
                    let sx0: usize = (fx.floor() as usize).min(max_sx);
                    let sx1: usize = (sx0 + 1).min(max_sx);
                    let wx: f32 = fx - (sx0 as f32);
                    let iwx: f32 = 1.0 - wx;
                    let i00: usize = (row0 + sx0) * 4;
                    let i10: usize = (row0 + sx1) * 4;
                    let i01: usize = (row1 + sx0) * 4;
                    let i11: usize = (row1 + sx1) * 4;
                    if i11 + 4 <= src_bytes.len() {
                        let w00: f32 = iwx * iwy;
                        let w10: f32 = wx * iwy;
                        let w01: f32 = iwx * wy;
                        let w11: f32 = wx * wy;
                        let r: u8 = ((src_bytes[i00] as f32) * w00
                            + (src_bytes[i10] as f32) * w10
                            + (src_bytes[i01] as f32) * w01
                            + (src_bytes[i11] as f32) * w11)
                            .round() as u8;
                        let g: u8 = ((src_bytes[i00 + 1] as f32) * w00
                            + (src_bytes[i10 + 1] as f32) * w10
                            + (src_bytes[i01 + 1] as f32) * w01
                            + (src_bytes[i11 + 1] as f32) * w11)
                            .round() as u8;
                        let b: u8 = ((src_bytes[i00 + 2] as f32) * w00
                            + (src_bytes[i10 + 2] as f32) * w10
                            + (src_bytes[i01 + 2] as f32) * w01
                            + (src_bytes[i11 + 2] as f32) * w11)
                            .round() as u8;
                        let a: u8 = ((src_bytes[i00 + 3] as f32) * w00
                            + (src_bytes[i10 + 3] as f32) * w10
                            + (src_bytes[i01 + 3] as f32) * w01
                            + (src_bytes[i11 + 3] as f32) * w11)
                            .round() as u8;
                        let sample: Color = Color::new(r, g, b, a).multiply_alpha(cov);
                        fb.blend_pixel(x, y, sample);
                        shaded += 1;
                    }
                }
            } else {
                let sy: usize = ((v * (src_h as f32)).floor() as usize).min(max_sy);
                let row: usize = sy * src_w_usize;
                for x in x0..x1 {
                    let cov: f32 = if has_radius {
                        let px: f32 = (x as f32) + 0.5 - cx;
                        sdf_coverage(rounded_box_sdf(px, py, half_w, half_h, max_r), aa)
                    } else {
                        1.0
                    };
                    if cov <= 0.0 {
                        continue;
                    }
                    let u: f32 = (((x as f32) + 0.5 - rect.get_x()) * inv_dst_w).clamp(0.0, 1.0);
                    let sx: usize = ((u * (src_w as f32)).floor() as usize).min(max_sx);
                    let idx: usize = (row + sx) * 4;
                    if idx + 4 <= src_bytes.len() {
                        let sample: Color = Color::new(
                            src_bytes[idx],
                            src_bytes[idx + 1],
                            src_bytes[idx + 2],
                            src_bytes[idx + 3],
                        )
                        .multiply_alpha(cov);
                        fb.blend_pixel(x, y, sample);
                        shaded += 1;
                    }
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
        if radius <= 0.0 || color.get_a() == 0 {
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
        let fb_w: u32 = self.get_framebuffer().get_width();
        let fb_h: u32 = self.get_framebuffer().get_height();
        let x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(fb_h);
        let x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(fb_h);
        let outer_r: f32 = radius + 1.0;
        let inner_r: f32 = (radius - 1.0).max(0.0);
        let outer_r2: f32 = outer_r * outer_r;
        let inner_r2: f32 = inner_r * inner_r;
        let cx: f32 = center.get_x();
        let cy: f32 = center.get_y();
        let mut shaded: usize = 0;
        let fb: &mut Framebuffer = self.get_mut_framebuffer();
        for y in y0..y1 {
            let dy: f32 = (y as f32) + 0.5 - cy;
            let dy2: f32 = dy * dy;
            if dy2 >= outer_r2 {
                continue;
            }
            let out_dx: f32 = (outer_r2 - dy2).sqrt();
            let row_x0: u32 = ((cx - out_dx).floor().max(0.0) as u32).clamp(x0, x1);
            let row_x1: u32 = ((cx + out_dx).ceil().max(0.0) as u32).clamp(x0, x1);
            if dy2 < inner_r2 && inner_r > 1.0 {
                let in_dx: f32 = (inner_r2 - dy2).sqrt();
                let in_x0: u32 = ((cx - in_dx).ceil().max(0.0) as u32).clamp(row_x0, row_x1);
                let in_x1: u32 = ((cx + in_dx).floor().max(0.0) as u32).clamp(row_x0, row_x1);
                for x in row_x0..in_x0 {
                    let dx: f32 = (x as f32) + 0.5 - cx;
                    let cov: f32 = sdf_coverage((dx * dx + dy2).sqrt() - radius, aa);
                    if cov > 0.0 {
                        fb.blend_pixel(x, y, color.multiply_alpha(cov));
                        shaded += 1;
                    }
                }
                if in_x0 < in_x1 {
                    fb.fill_span(in_x0, in_x1, y, color);
                    shaded += (in_x1 - in_x0) as usize;
                }
                for x in in_x1..row_x1 {
                    let dx: f32 = (x as f32) + 0.5 - cx;
                    let cov: f32 = sdf_coverage((dx * dx + dy2).sqrt() - radius, aa);
                    if cov > 0.0 {
                        fb.blend_pixel(x, y, color.multiply_alpha(cov));
                        shaded += 1;
                    }
                }
            } else {
                for x in row_x0..row_x1 {
                    let dx: f32 = (x as f32) + 0.5 - cx;
                    let cov: f32 = sdf_coverage((dx * dx + dy2).sqrt() - radius, aa);
                    if cov > 0.0 {
                        fb.blend_pixel(x, y, color.multiply_alpha(cov));
                        shaded += 1;
                    }
                }
            }
        }
        shaded
    }

    /// Rasterizes a solid filled triangle using fast scanline edge-intersection spans.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `Point` - First physical vertex.
    /// - `Point` - Second physical vertex.
    /// - `Point` - Third physical vertex.
    /// - `Color` - Fill color.
    /// - `Rect` - Active clip bounds.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_triangle(
        &mut self,
        p0: Point,
        p1: Point,
        p2: Point,
        color: Color,
        clip: Rect,
    ) -> usize {
        if color.get_a() == 0 {
            return 0;
        }
        let min_x: f32 = p0.get_x().min(p1.get_x()).min(p2.get_x());
        let min_y: f32 = p0.get_y().min(p1.get_y()).min(p2.get_y());
        let max_x: f32 = p0.get_x().max(p1.get_x()).max(p2.get_x());
        let max_y: f32 = p0.get_y().max(p1.get_y()).max(p2.get_y());
        let Some(vis) = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y).intersect(clip)
        else {
            return 0;
        };
        let fb_w: u32 = self.get_framebuffer().get_width();
        let fb_h: u32 = self.get_framebuffer().get_height();
        let clip_x0: u32 = (vis.get_x().floor().max(0.0) as u32).min(fb_w);
        let clip_x1: u32 = (vis.right().ceil().max(0.0) as u32).min(fb_w);
        let y0: u32 = (vis.get_y().floor().max(0.0) as u32).min(fb_h);
        let y1: u32 = (vis.bottom().ceil().max(0.0) as u32).min(fb_h);
        let edges: [(Point, Point); 3] = [(p0, p1), (p1, p2), (p2, p0)];
        let aa: AntialiasingMode = self.get_antialiasing();
        let mut shaded: usize = 0;
        let fb: &mut Framebuffer = self.get_mut_framebuffer();
        for y in y0..y1 {
            let py: f32 = (y as f32) + 0.5;
            let mut min_ix: f32 = f32::INFINITY;
            let mut max_ix: f32 = f32::NEG_INFINITY;
            for (a, b) in edges.iter() {
                let ay: f32 = a.get_y();
                let by: f32 = b.get_y();
                if (ay <= py && by > py) || (by <= py && ay > py) {
                    let t: f32 = (py - ay) / (by - ay);
                    let ix: f32 = a.get_x() + t * (b.get_x() - a.get_x());
                    min_ix = min_ix.min(ix);
                    max_ix = max_ix.max(ix);
                }
            }
            if min_ix <= max_ix {
                if aa == AntialiasingMode::None {
                    let sx0: u32 = (min_ix.floor().max(0.0) as u32).clamp(clip_x0, clip_x1);
                    let sx1: u32 = (max_ix.ceil().max(0.0) as u32).clamp(clip_x0, clip_x1);
                    if sx0 < sx1 {
                        fb.fill_span(sx0, sx1, y, color);
                        shaded += (sx1 - sx0) as usize;
                    }
                } else {
                    let left_px: u32 = (min_ix.floor().max(0.0) as u32).clamp(clip_x0, clip_x1);
                    let right_px: u32 = (max_ix.floor().max(0.0) as u32).clamp(clip_x0, clip_x1);
                    if left_px == right_px {
                        if left_px < clip_x1 {
                            let cov: f32 = (max_ix - min_ix).clamp(0.0, 1.0);
                            if cov > 0.01 {
                                fb.blend_pixel(left_px, y, color.multiply_alpha(cov));
                                shaded += 1;
                            }
                        }
                    } else {
                        if left_px >= clip_x0 && left_px < clip_x1 {
                            let cov: f32 = ((left_px as f32) + 1.0 - min_ix).clamp(0.0, 1.0);
                            if cov > 0.01 {
                                fb.blend_pixel(left_px, y, color.multiply_alpha(cov));
                                shaded += 1;
                            }
                        }
                        let inner_start: u32 = (left_px + 1).clamp(clip_x0, clip_x1);
                        let inner_end: u32 = right_px.clamp(clip_x0, clip_x1);
                        if inner_start < inner_end {
                            fb.fill_span(inner_start, inner_end, y, color);
                            shaded += (inner_end - inner_start) as usize;
                        }
                        if right_px >= clip_x0 && right_px < clip_x1 {
                            let cov: f32 = (max_ix - (right_px as f32)).clamp(0.0, 1.0);
                            if cov > 0.01 {
                                fb.blend_pixel(right_px, y, color.multiply_alpha(cov));
                                shaded += 1;
                            }
                        }
                    }
                }
            }
        }
        shaded
    }

    /// Blits a cropped sub-rectangle of an `ImageBuffer` sprite sheet with optional color tint.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable software renderer.
    /// - `Rect` - Physical destination bounds.
    /// - `&ImageBuffer` - Source sprite-sheet RGBA8 buffer.
    /// - `Rect` - Source pixel sub-rectangle.
    /// - `Color` - Multiplicative tint color.
    /// - `Rect` - Active clip bounds.
    ///
    /// # Returns
    ///
    /// - `usize` - Shaded pixel count.
    fn raster_sprite_region(
        &mut self,
        rect: Rect,
        buffer: &ImageBuffer,
        src_rect: Rect,
        tint: Color,
        clip: Rect,
    ) -> usize {
        if buffer.get_width() == 0
            || buffer.get_height() == 0
            || tint.get_a() == 0
            || src_rect.get_width() <= 0.0
            || src_rect.get_height() <= 0.0
        {
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
        let buf_w: u32 = buffer.get_width();
        let buf_h: u32 = buffer.get_height();
        let src_bytes: &[u8] = buffer.get_pixels().as_slice();
        let inv_w: f32 = 1.0 / rect.get_width().max(1.0);
        let inv_h: f32 = 1.0 / rect.get_height().max(1.0);
        let untinted: bool = tint == COLOR_WHITE;
        let tr: u32 = tint.get_r() as u32;
        let tg: u32 = tint.get_g() as u32;
        let tb: u32 = tint.get_b() as u32;
        let ta: u32 = tint.get_a() as u32;
        let mut shaded: usize = 0;
        let fb: &mut Framebuffer = self.get_mut_framebuffer();
        for y in y0..y1 {
            let v: f32 = (((y as f32) - rect.get_y()) * inv_h).clamp(0.0, 0.9999);
            let sy: u32 = ((src_rect.get_y() + v * src_rect.get_height())
                .floor()
                .max(0.0) as u32)
                .min(buf_h - 1);
            let sy_row: usize = (sy as usize) * (buf_w as usize);
            for x in x0..x1 {
                let u: f32 = (((x as f32) - rect.get_x()) * inv_w).clamp(0.0, 0.9999);
                let sx: u32 = ((src_rect.get_x() + u * src_rect.get_width())
                    .floor()
                    .max(0.0) as u32)
                    .min(buf_w - 1);
                let idx: usize = (sy_row + (sx as usize)) * 4;
                if idx + 4 <= src_bytes.len() {
                    let a: u8 = src_bytes[idx + 3];
                    if a == 0 {
                        continue;
                    }
                    let sample: Color = if untinted {
                        Color::new(src_bytes[idx], src_bytes[idx + 1], src_bytes[idx + 2], a)
                    } else {
                        let rr: u32 = (src_bytes[idx] as u32) * tr + 128;
                        let gg: u32 = (src_bytes[idx + 1] as u32) * tg + 128;
                        let bb: u32 = (src_bytes[idx + 2] as u32) * tb + 128;
                        let aa_v: u32 = (a as u32) * ta + 128;
                        Color::new(
                            ((rr + (rr >> 8)) >> 8) as u8,
                            ((gg + (gg >> 8)) >> 8) as u8,
                            ((bb + (bb >> 8)) >> 8) as u8,
                            ((aa_v + (aa_v >> 8)) >> 8) as u8,
                        )
                    };
                    fb.blend_pixel(x, y, sample);
                    shaded += 1;
                }
            }
        }
        shaded
    }
}
