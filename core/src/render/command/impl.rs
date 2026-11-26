use super::*;

/// Command emission helpers for `DisplayList`.
impl DisplayList {
    /// Clears all commands while preserving allocated backing capacity.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable display list.
    /// - `Size` - Target logical viewport size.
    /// - `f32` - Active DPI scale factor.
    pub fn reset(&mut self, viewport: Size, scale_factor: f32) {
        self.get_mut_commands().clear();
        self.set_viewport(viewport);
        self.set_scale_factor(scale_factor.max(0.5));
    }

    /// Appends a single `DrawCommand` to the list.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable display list.
    /// - `DrawCommand` - Command to push.
    pub fn push(&mut self, command: DrawCommand) {
        self.get_mut_commands().push(command);
    }

    /// Returns total emitted command count.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Display list reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of commands.
    #[must_use]
    pub fn len(&self) -> usize {
        self.get_commands().len()
    }

    /// Returns true when the display list contains zero commands.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Display list reference.
    ///
    /// # Returns
    ///
    /// - `bool` - True if empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.get_commands().is_empty()
    }

    /// Folds a 64-bit value into an FNV-1a accumulator.
    ///
    /// # Arguments
    ///
    /// - `&mut u64` - Mutable hash accumulator.
    /// - `u64` - Value to fold into the digest.
    fn mix_hash(hash: &mut u64, value: u64) {
        *hash ^= value;
        *hash = hash.wrapping_mul(FNV1A_PRIME);
    }

    /// Computes a fast O(commands) 64-bit FNV-1a digest over the display list without scanning framebuffer pixels.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Display list reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Deterministic command-stream digest.
    #[must_use]
    pub fn fast_hash(&self) -> u64 {
        let mut hash: u64 = FNV1A_OFFSET_BASIS;
        Self::mix_hash(&mut hash, self.get_commands().len() as u64);
        Self::mix_hash(&mut hash, self.get_viewport().get_width().to_bits() as u64);
        Self::mix_hash(&mut hash, self.get_viewport().get_height().to_bits() as u64);
        Self::mix_hash(&mut hash, self.get_scale_factor().to_bits() as u64);
        for cmd in self.get_commands().iter() {
            match cmd {
                DrawCommand::Clear(c) => {
                    Self::mix_hash(&mut hash, 1);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(c.to_rgba_bytes()) as u64);
                }
                DrawCommand::FillRoundedRect {
                    rect,
                    color,
                    border,
                    ..
                } => {
                    Self::mix_hash(&mut hash, 2);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_width().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_height().to_bits() as u64);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(color.to_rgba_bytes()) as u64);
                    Self::mix_hash(&mut hash, border.get_width().to_bits() as u64);
                }
                DrawCommand::LinearGradientRect { rect, gradient, .. } => {
                    Self::mix_hash(&mut hash, 3);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(
                        &mut hash,
                        u32::from_le_bytes(gradient.get_start_color().to_rgba_bytes()) as u64,
                    );
                }
                DrawCommand::BoxShadow { rect, shadow, .. } => {
                    Self::mix_hash(&mut hash, 4);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, shadow.get_blur_radius().to_bits() as u64);
                }
                DrawCommand::TextRun { rect, text, style } => {
                    Self::mix_hash(&mut hash, 5);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, text.len() as u64);
                    for byte in text.as_bytes().iter().take(16).copied() {
                        Self::mix_hash(&mut hash, byte as u64);
                    }
                    Self::mix_hash(
                        &mut hash,
                        u32::from_le_bytes(style.get_color().to_rgba_bytes()) as u64,
                    );
                }
                DrawCommand::Image { rect, buffer, .. } => {
                    Self::mix_hash(&mut hash, 6);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, buffer.get_width() as u64);
                    Self::mix_hash(&mut hash, buffer.get_height() as u64);
                }
                DrawCommand::Line {
                    from,
                    to,
                    color,
                    width,
                } => {
                    Self::mix_hash(&mut hash, 7);
                    Self::mix_hash(&mut hash, from.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, to.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(color.to_rgba_bytes()) as u64);
                    Self::mix_hash(&mut hash, width.to_bits() as u64);
                }
                DrawCommand::Circle {
                    center,
                    radius,
                    color,
                } => {
                    Self::mix_hash(&mut hash, 8);
                    Self::mix_hash(&mut hash, center.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, center.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, radius.to_bits() as u64);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(color.to_rgba_bytes()) as u64);
                }
                DrawCommand::Triangle { p0, p1, p2, color } => {
                    Self::mix_hash(&mut hash, 9);
                    Self::mix_hash(&mut hash, p0.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, p1.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, p2.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(color.to_rgba_bytes()) as u64);
                }
                DrawCommand::SpriteRegion {
                    rect,
                    src_rect,
                    tint,
                    ..
                } => {
                    Self::mix_hash(&mut hash, 10);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, src_rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(tint.to_rgba_bytes()) as u64);
                }
                DrawCommand::PushClip(r) => {
                    Self::mix_hash(&mut hash, 11);
                    Self::mix_hash(&mut hash, r.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, r.get_y().to_bits() as u64);
                }
                DrawCommand::PopClip => {
                    Self::mix_hash(&mut hash, 12);
                }
            }
        }
        hash
    }
}

/// Scene-to-display-list compilation pass.
impl PaintPass {
    /// Compiles `tree` into `display_list`, culling offscreen nodes and honoring z-index stacking.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Laid-out scene tree.
    /// - `Size` - Logical viewport size.
    /// - `f32` - DPI scale factor.
    /// - `Color` - Surface clear background color.
    /// - `&mut DisplayList` - Output reusable display list.
    pub fn build(
        tree: &SceneTree,
        viewport: Size,
        scale_factor: f32,
        clear_color: Color,
        display_list: &mut DisplayList,
    ) {
        display_list.reset(viewport, scale_factor);
        display_list.push(DrawCommand::Clear(clear_color));
        let viewport_rect: Rect = Rect::new(0.0, 0.0, viewport.get_width(), viewport.get_height());
        let mut portals: Vec<(i32, usize, NodeId)> = Vec::new();
        Self::emit_node(
            tree,
            tree.get_root(),
            Point::new(0.0, 0.0),
            1.0,
            viewport_rect,
            Some(&mut portals),
            display_list,
        );
        portals.sort_by(|a: &(i32, usize, NodeId), b: &(i32, usize, NodeId)| {
            a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))
        });
        for (_, _, portal_id) in portals.into_iter() {
            Self::emit_node(
                tree,
                portal_id,
                Point::new(0.0, 0.0),
                1.0,
                viewport_rect,
                None,
                display_list,
            );
        }
    }

    /// Computes subtle shadcn-aligned hover/pressed color tinting for interactive controls.
    ///
    /// # Arguments
    ///
    /// - `Option<Color>` - Base background color, if any.
    /// - `bool` - Whether the pointer hovers the control.
    /// - `bool` - Whether the pointer actively presses the control.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Tinted background color.
    fn interactive_tint(base: Option<Color>, hovered: bool, pressed: bool) -> Option<Color> {
        if !hovered && !pressed {
            return base;
        }
        let delta: i32 = if pressed { 24 } else { 14 };
        match base {
            Some(c) if c.get_a() > 0 => {
                let luma: u32 =
                    (c.get_r() as u32) * 299 + (c.get_g() as u32) * 587 + (c.get_b() as u32) * 114;
                if luma < 120_000 {
                    let lift: i32 = if pressed { 30 } else { 20 };
                    Some(Color::new(
                        (c.get_r() as i32 + lift).clamp(0, 255) as u8,
                        (c.get_g() as i32 + lift).clamp(0, 255) as u8,
                        (c.get_b() as i32 + lift).clamp(0, 255) as u8,
                        c.get_a(),
                    ))
                } else {
                    Some(Color::new(
                        (c.get_r() as i32 - delta).clamp(0, 255) as u8,
                        (c.get_g() as i32 - delta).clamp(0, 255) as u8,
                        (c.get_b() as i32 - delta).clamp(0, 255) as u8,
                        c.get_a(),
                    ))
                }
            }
            _ => Some(Color::rgba(113, 113, 122, if pressed { 42 } else { 26 })),
        }
    }

    /// Recursively emits commands for `node_id` and its z-sorted children.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Scene tree reference.
    /// - `NodeId` - Node to paint.
    /// - `Point` - Accumulated ancestor scroll offset.
    /// - `f32` - Accumulated ancestor opacity.
    /// - `Rect` - Active cull clip rectangle.
    /// - `Option<&mut Vec<(i32, usize, NodeId)>>` - Optional deferred top-layer portal queue (`None` when painting top layer).
    /// - `&mut DisplayList` - Output command list.
    fn emit_node(
        tree: &SceneTree,
        node_id: NodeId,
        scroll_offset: Point,
        parent_opacity: f32,
        cull_rect: Rect,
        mut portals: Option<&mut Vec<(i32, usize, NodeId)>>,
        display_list: &mut DisplayList,
    ) {
        let Some(node) = tree.get_node(node_id) else {
            return;
        };
        let style: Style = node.get_style();
        let layout: LayoutStyle = style.get_layout();
        if layout.get_display() == Display::None {
            return;
        }
        if let Some(portal_queue) = portals.as_deref_mut()
            && (matches!(node.get_kind(), NodeKind::Portal)
                || layout.get_position() == Position::Fixed)
        {
            portal_queue.push((layout.get_z_index(), portal_queue.len(), node_id));
            return;
        }
        let hovered: bool = node.get_interaction().get_hovered();
        let pressed: bool = node.get_interaction().get_pressed();
        let mut paint: PaintStyle = if hovered {
            node.hover_paint().unwrap_or(style.get_paint())
        } else {
            style.get_paint()
        };
        let is_interactive: bool = paint.get_cursor() == CursorIcon::Pointer
            || node
                .get_handlers()
                .handler_for(&UiEventKind::Click(Point::default()))
                .is_some();
        let is_transparent_canvas: bool =
            matches!(node.get_kind(), NodeKind::Canvas) && paint.background_color().is_none();
        if is_interactive
            && !is_transparent_canvas
            && node.hover_paint().is_none()
            && (hovered || pressed)
        {
            paint.set_background(Self::interactive_tint(
                paint.background_color(),
                hovered,
                pressed,
            ));
        }
        let opacity: f32 = (parent_opacity * paint.get_opacity()).clamp(0.0, 1.0);
        if opacity <= 0.0 {
            return;
        }
        let raw_bounds: Rect = node.get_layout_box().get_bounds();
        let transform: Transform2D = paint.get_transform();
        let bounds: Rect = Rect::new(
            raw_bounds.get_x() - scroll_offset.get_x() + transform.get_translate_x(),
            raw_bounds.get_y() - scroll_offset.get_y() + transform.get_translate_y(),
            raw_bounds.get_width() * transform.get_scale_x(),
            raw_bounds.get_height() * transform.get_scale_y(),
        );
        let clipped: bool = matches!(layout.get_overflow(), Overflow::Hidden | Overflow::Scroll);
        let next_cull: Rect = if clipped {
            match cull_rect.intersect(bounds) {
                Some(r) => r,
                None => return,
            }
        } else {
            cull_rect
        };
        let halo: f32 = if paint.box_shadow().is_some() {
            36.0
        } else if paint.focus_ring_border().is_some() {
            8.0
        } else {
            2.0
        };
        let self_vis_rect: Rect = Rect::new(
            bounds.get_x() - halo,
            bounds.get_y() - halo,
            bounds.get_width() + halo * 2.0,
            bounds.get_height() + halo * 2.0,
        );
        let self_visible: bool = bounds.get_width() > 0.0
            && bounds.get_height() > 0.0
            && self_vis_rect.intersect(cull_rect).is_some();
        if self_visible {
            if let Some(mut shadow) = paint.box_shadow() {
                shadow.set_color(shadow.get_color().multiply_alpha(opacity));
                display_list.push(DrawCommand::BoxShadow {
                    rect: bounds,
                    radius: paint.get_radius(),
                    shadow,
                });
            }
            if let Some(mut gradient) = paint.background_gradient() {
                gradient.set_start_color(gradient.get_start_color().multiply_alpha(opacity));
                gradient.set_end_color(gradient.get_end_color().multiply_alpha(opacity));
                display_list.push(DrawCommand::LinearGradientRect {
                    rect: bounds,
                    gradient,
                    radius: paint.get_radius(),
                });
            } else if let Some(bg) = paint.background_color() {
                let mut border: Border = paint.get_border();
                border.set_color(border.get_color().multiply_alpha(opacity));
                display_list.push(DrawCommand::FillRoundedRect {
                    rect: bounds,
                    color: bg.multiply_alpha(opacity),
                    radius: paint.get_radius(),
                    border,
                });
            } else if paint.get_border().get_width() > 0.0 {
                let mut border: Border = paint.get_border();
                border.set_color(border.get_color().multiply_alpha(opacity));
                display_list.push(DrawCommand::FillRoundedRect {
                    rect: bounds,
                    color: COLOR_TRANSPARENT,
                    radius: paint.get_radius(),
                    border,
                });
            }
            if node.get_interaction().get_focused()
                && let Some(mut ring) = paint.focus_ring_border()
            {
                ring.set_color(ring.get_color().multiply_alpha(opacity));
                let ring_w: f32 = ring.get_width();
                let ring_rect: Rect = Rect::new(
                    bounds.get_x() - ring_w,
                    bounds.get_y() - ring_w,
                    bounds.get_width() + ring_w * 2.0,
                    bounds.get_height() + ring_w * 2.0,
                );
                display_list.push(DrawCommand::FillRoundedRect {
                    rect: ring_rect,
                    color: COLOR_TRANSPARENT,
                    radius: CornerRadius::all(paint.get_radius().max_radius() + ring_w),
                    border: ring,
                });
            }
            match node.get_kind() {
                NodeKind::Text(text) => {
                    let mut text_style: TextStyle = style.get_text();
                    text_style.set_color(text_style.get_color().multiply_alpha(opacity));
                    let content_b: Rect = node.get_layout_box().get_content_bounds();
                    let shifted_content: Rect = Rect::new(
                        content_b.get_x() - scroll_offset.get_x() + transform.get_translate_x(),
                        content_b.get_y() - scroll_offset.get_y() + transform.get_translate_y(),
                        content_b.get_width(),
                        content_b.get_height(),
                    );
                    display_list.push(DrawCommand::TextRun {
                        rect: shifted_content,
                        text: text.clone(),
                        style: text_style,
                    });
                }
                NodeKind::Input(input_state) => {
                    let mut text_style: TextStyle = style.get_text();
                    let rendered_text: String = if input_state.get_value().is_empty() {
                        text_style.set_color(text_style.get_color().multiply_alpha(opacity * 0.55));
                        input_state.get_placeholder().clone()
                    } else if input_state.get_password() {
                        text_style.set_color(text_style.get_color().multiply_alpha(opacity));
                        "*".repeat(input_state.get_value().chars().count())
                    } else {
                        text_style.set_color(text_style.get_color().multiply_alpha(opacity));
                        input_state.get_value().clone()
                    };
                    let content_b: Rect = node.get_layout_box().get_content_bounds();
                    let shifted_content: Rect = Rect::new(
                        content_b.get_x() - scroll_offset.get_x() + transform.get_translate_x(),
                        content_b.get_y() - scroll_offset.get_y() + transform.get_translate_y(),
                        content_b.get_width(),
                        content_b.get_height(),
                    );
                    let line_h: f32 =
                        (text_style.get_font_size() * text_style.get_line_height()).max(1.0);
                    let text_rect: Rect = if input_state.get_multiline() {
                        shifted_content
                    } else {
                        text_style.set_overflow(TextOverflow::Clip);
                        let text_y: f32 = shifted_content.get_y()
                            + ((shifted_content.get_height() - line_h) * 0.5).max(0.0);
                        Rect::new(
                            shifted_content.get_x(),
                            text_y,
                            shifted_content.get_width(),
                            line_h,
                        )
                    };
                    display_list.push(DrawCommand::PushClip(shifted_content));
                    display_list.push(DrawCommand::TextRun {
                        rect: text_rect,
                        text: rendered_text.clone(),
                        style: text_style,
                    });
                    if node.get_interaction().get_focused() && tree.get_caret_visible() {
                        let meas: TextLayoutResult = DeterministicTextMeasurer.measure(
                            if input_state.get_value().is_empty() {
                                ""
                            } else {
                                rendered_text.as_str()
                            },
                            text_style,
                            if input_state.get_multiline() {
                                Some(shifted_content.get_width().max(1.0))
                            } else {
                                None
                            },
                        );
                        let caret_h: f32 = (text_style.get_font_size() * 1.15)
                            .min(shifted_content.get_height().max(12.0));
                        let (caret_x, caret_y): (f32, f32) = if input_state.get_multiline() {
                            let last_w: f32 = meas
                                .get_lines()
                                .last()
                                .map(TextLine::get_width)
                                .unwrap_or(0.0);
                            let line_idx: usize = meas.get_lines().len().saturating_sub(1);
                            let cy: f32 = shifted_content.get_y()
                                + (line_idx as f32) * meas.get_line_height_px()
                                + ((meas.get_line_height_px() - caret_h) * 0.5).max(0.0);
                            (
                                (shifted_content.get_x() + last_w + 1.0)
                                    .min(shifted_content.right() - 2.0),
                                cy.min(shifted_content.bottom() - caret_h)
                                    .max(shifted_content.get_y()),
                            )
                        } else {
                            (
                                (shifted_content.get_x() + meas.get_size().get_width() + 1.0)
                                    .min(shifted_content.right() - 2.0),
                                shifted_content.get_y()
                                    + ((shifted_content.get_height() - caret_h) * 0.5).max(0.0),
                            )
                        };
                        display_list.push(DrawCommand::FillRoundedRect {
                            rect: Rect::new(caret_x, caret_y, 1.5, caret_h),
                            color: style.get_text().get_color().multiply_alpha(opacity),
                            radius: CornerRadius::all(0.5),
                            border: Border::default(),
                        });
                    }
                    display_list.push(DrawCommand::PopClip);
                }
                NodeKind::Image(buffer) => {
                    display_list.push(DrawCommand::Image {
                        rect: bounds,
                        buffer: buffer.clone(),
                        radius: paint.get_radius(),
                    });
                }
                NodeKind::Canvas => {
                    if let Some(callback) = node.canvas_callback()
                        && let Ok(mut func) = callback.try_borrow_mut()
                    {
                        display_list.push(DrawCommand::PushClip(bounds));
                        func(bounds, display_list);
                        display_list.push(DrawCommand::PopClip);
                    }
                }
                NodeKind::View | NodeKind::ScrollView | NodeKind::Portal | NodeKind::Fragment => {}
            }
        }
        if clipped {
            display_list.push(DrawCommand::PushClip(bounds));
        }
        let child_scroll: Point = Point::new(
            scroll_offset.get_x() + node.get_layout_box().get_scroll_offset().get_x(),
            scroll_offset.get_y() + node.get_layout_box().get_scroll_offset().get_y(),
        );
        let raw_children: &[NodeId] = node.get_children().as_slice();
        let needs_z_sort: bool = raw_children.len() > 1
            && raw_children.iter().copied().any(|cid: NodeId| {
                tree.get_node(cid)
                    .map(|c: &SceneNode| c.get_style().get_layout().get_z_index() != 0)
                    .unwrap_or(false)
            });
        if !needs_z_sort {
            for child_id in raw_children.iter().copied() {
                Self::emit_node(
                    tree,
                    child_id,
                    child_scroll,
                    opacity,
                    next_cull,
                    portals.as_deref_mut(),
                    display_list,
                );
            }
        } else {
            let mut sorted_children: Vec<(i32, usize, NodeId)> = raw_children
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(idx, cid): (usize, NodeId)| {
                    tree.get_node(cid)
                        .map(|c: &SceneNode| (c.get_style().get_layout().get_z_index(), idx, cid))
                })
                .collect();
            sorted_children.sort_by(|a: &(i32, usize, NodeId), b: &(i32, usize, NodeId)| {
                a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))
            });
            for (_, _, child_id) in sorted_children.into_iter() {
                Self::emit_node(
                    tree,
                    child_id,
                    child_scroll,
                    opacity,
                    next_cull,
                    portals.as_deref_mut(),
                    display_list,
                );
            }
        }
        let vp: Size = display_list.get_viewport();
        let show_scrollbar: bool = vp.get_width().min(vp.get_height()) >= 540.0;
        if show_scrollbar && layout.get_overflow() == Overflow::Scroll && bounds.get_height() > 24.0
        {
            let lbox: LayoutBox = node.get_layout_box();
            let border_w: f32 = paint.get_border().get_width();
            let inner_x: f32 = bounds.get_x() + border_w;
            let inner_y: f32 = bounds.get_y() + border_w;
            let inner_w: f32 = (bounds.get_width() - border_w * 2.0).max(1.0);
            let inner_h: f32 = (bounds.get_height() - border_w * 2.0).max(1.0);
            let view_h: f32 = lbox.get_content_bounds().get_height().max(1.0);
            let (scroll_y, total_h): (f32, f32) = match node.virtual_scroll_metrics() {
                Some((vy, vh)) => (vy, vh),
                None => (
                    lbox.get_scroll_offset().get_y(),
                    lbox.get_content_size().get_height(),
                ),
            };
            let max_scroll_y: f32 = (total_h - view_h).max(0.0);
            if max_scroll_y > 1.0 {
                let track_margin: f32 = 3.0;
                let track_w: f32 = if hovered { 6.0 } else { 4.0 };
                let track_x: f32 = (inner_x + inner_w - track_w - 2.0).max(inner_x);
                let track_y: f32 = inner_y + track_margin;
                let track_h: f32 = (inner_h - track_margin * 2.0).max(16.0);
                let ratio: f32 = (view_h / total_h.max(view_h)).clamp(0.0, 1.0);
                let min_thumb_h: f32 = 20.0_f32.min(track_h);
                let thumb_h: f32 = (track_h * ratio).clamp(min_thumb_h, track_h);
                let scroll_frac: f32 = (scroll_y / max_scroll_y).clamp(0.0, 1.0);
                let thumb_y: f32 = track_y + (track_h - thumb_h) * scroll_frac;
                if hovered {
                    display_list.push(DrawCommand::FillRoundedRect {
                        rect: Rect::new(track_x, track_y, track_w, track_h),
                        color: Color::rgba(113, 113, 122, 24).multiply_alpha(opacity),
                        radius: CornerRadius::all(3.0),
                        border: Border::default(),
                    });
                }
                let thumb_alpha: u8 = if hovered { 180 } else { 112 };
                display_list.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(track_x, thumb_y, track_w, thumb_h),
                    color: Color::rgba(113, 113, 122, thumb_alpha).multiply_alpha(opacity),
                    radius: CornerRadius::all(track_w * 0.5),
                    border: Border::default(),
                });
            }
        }
        if clipped {
            display_list.push(DrawCommand::PopClip);
        }
    }
}
