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
        self.set_portal_split_index(0);
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
                DrawCommand::TextCaret {
                    text_rect,
                    text,
                    color,
                    ..
                } => {
                    Self::mix_hash(&mut hash, 13);
                    Self::mix_hash(&mut hash, text_rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, text_rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, text.len() as u64);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(color.to_rgba_bytes()) as u64);
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
                DrawCommand::PushRoundedClip { rect, radius } => {
                    Self::mix_hash(&mut hash, 14);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, radius.max_radius().to_bits() as u64);
                }
                DrawCommand::PopClip => {
                    Self::mix_hash(&mut hash, 12);
                }
            }
        }
        hash
    }

    /// Computes a full collision-resistant 64-bit FNV-1a digest over the base-scene slice `commands[..portal_split_index]` for lossless portal backdrop caching.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Display list reference.
    ///
    /// # Returns
    ///
    /// - `u64` - Complete base-scene digest (guaranteed non-zero when commands are present).
    #[must_use]
    pub fn base_layer_hash(&self) -> u64 {
        let split: usize = self.get_portal_split_index().min(self.get_commands().len());
        let mut hash: u64 = FNV1A_OFFSET_BASIS;
        Self::mix_hash(&mut hash, split as u64);
        Self::mix_hash(&mut hash, self.get_viewport().get_width().to_bits() as u64);
        Self::mix_hash(&mut hash, self.get_viewport().get_height().to_bits() as u64);
        Self::mix_hash(&mut hash, self.get_scale_factor().to_bits() as u64);
        for cmd in self.get_commands()[..split].iter() {
            match cmd {
                DrawCommand::Clear(c) => {
                    Self::mix_hash(&mut hash, 1);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(c.to_rgba_bytes()) as u64);
                }
                DrawCommand::FillRoundedRect {
                    rect,
                    color,
                    radius,
                    border,
                } => {
                    Self::mix_hash(&mut hash, 2);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_width().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_height().to_bits() as u64);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(color.to_rgba_bytes()) as u64);
                    Self::mix_hash(&mut hash, radius.get_top_left().to_bits() as u64);
                    Self::mix_hash(&mut hash, radius.get_top_right().to_bits() as u64);
                    Self::mix_hash(&mut hash, radius.get_bottom_right().to_bits() as u64);
                    Self::mix_hash(&mut hash, radius.get_bottom_left().to_bits() as u64);
                    Self::mix_hash(&mut hash, border.get_width().to_bits() as u64);
                    Self::mix_hash(
                        &mut hash,
                        u32::from_le_bytes(border.get_color().to_rgba_bytes()) as u64,
                    );
                }
                DrawCommand::LinearGradientRect {
                    rect,
                    gradient,
                    radius,
                } => {
                    Self::mix_hash(&mut hash, 3);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_width().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_height().to_bits() as u64);
                    Self::mix_hash(&mut hash, gradient.get_angle_deg().to_bits() as u64);
                    Self::mix_hash(
                        &mut hash,
                        u32::from_le_bytes(gradient.get_start_color().to_rgba_bytes()) as u64,
                    );
                    Self::mix_hash(
                        &mut hash,
                        u32::from_le_bytes(gradient.get_end_color().to_rgba_bytes()) as u64,
                    );
                    Self::mix_hash(&mut hash, radius.max_radius().to_bits() as u64);
                }
                DrawCommand::BoxShadow {
                    rect,
                    radius,
                    shadow,
                } => {
                    Self::mix_hash(&mut hash, 4);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_width().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_height().to_bits() as u64);
                    Self::mix_hash(&mut hash, radius.max_radius().to_bits() as u64);
                    Self::mix_hash(&mut hash, shadow.get_offset_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, shadow.get_offset_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, shadow.get_blur_radius().to_bits() as u64);
                    Self::mix_hash(&mut hash, shadow.get_spread_radius().to_bits() as u64);
                    Self::mix_hash(
                        &mut hash,
                        u32::from_le_bytes(shadow.get_color().to_rgba_bytes()) as u64,
                    );
                }
                DrawCommand::TextRun { rect, text, style } => {
                    Self::mix_hash(&mut hash, 5);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_width().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_height().to_bits() as u64);
                    Self::mix_hash(&mut hash, style.get_font_size().to_bits() as u64);
                    Self::mix_hash(&mut hash, style.get_family() as u64);
                    Self::mix_hash(&mut hash, style.get_weight() as u64);
                    Self::mix_hash(&mut hash, u64::from(style.get_italic()));
                    Self::mix_hash(&mut hash, style.get_align() as u64);
                    Self::mix_hash(
                        &mut hash,
                        u32::from_le_bytes(style.get_color().to_rgba_bytes()) as u64,
                    );
                    Self::mix_hash(&mut hash, text.len() as u64);
                    for byte in text.as_bytes().iter().copied() {
                        Self::mix_hash(&mut hash, byte as u64);
                    }
                }
                DrawCommand::TextCaret {
                    text_rect,
                    content_rect,
                    text,
                    style,
                    caret_height,
                    color,
                    multiline,
                } => {
                    Self::mix_hash(&mut hash, 13);
                    Self::mix_hash(&mut hash, text_rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, text_rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, content_rect.get_width().to_bits() as u64);
                    Self::mix_hash(&mut hash, caret_height.to_bits() as u64);
                    Self::mix_hash(&mut hash, style.get_font_size().to_bits() as u64);
                    Self::mix_hash(&mut hash, u64::from(*multiline));
                    Self::mix_hash(&mut hash, u32::from_le_bytes(color.to_rgba_bytes()) as u64);
                    Self::mix_hash(&mut hash, text.len() as u64);
                    for byte in text.as_bytes().iter().copied() {
                        Self::mix_hash(&mut hash, byte as u64);
                    }
                }
                DrawCommand::Image {
                    rect,
                    buffer,
                    radius,
                } => {
                    Self::mix_hash(&mut hash, 6);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_width().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_height().to_bits() as u64);
                    Self::mix_hash(&mut hash, buffer.get_width() as u64);
                    Self::mix_hash(&mut hash, buffer.get_height() as u64);
                    Self::mix_hash(&mut hash, buffer.get_pixels().as_ptr() as usize as u64);
                    Self::mix_hash(&mut hash, radius.max_radius().to_bits() as u64);
                }
                DrawCommand::Line {
                    from,
                    to,
                    color,
                    width,
                } => {
                    Self::mix_hash(&mut hash, 7);
                    Self::mix_hash(&mut hash, from.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, from.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, to.get_x().to_bits() as u64);
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
                    Self::mix_hash(&mut hash, p0.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, p1.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, p1.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, p2.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, p2.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(color.to_rgba_bytes()) as u64);
                }
                DrawCommand::SpriteRegion {
                    rect,
                    buffer,
                    src_rect,
                    tint,
                } => {
                    Self::mix_hash(&mut hash, 10);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_width().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_height().to_bits() as u64);
                    Self::mix_hash(&mut hash, buffer.get_pixels().as_ptr() as usize as u64);
                    Self::mix_hash(&mut hash, src_rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, src_rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, u32::from_le_bytes(tint.to_rgba_bytes()) as u64);
                }
                DrawCommand::PushClip(r) => {
                    Self::mix_hash(&mut hash, 11);
                    Self::mix_hash(&mut hash, r.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, r.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, r.get_width().to_bits() as u64);
                    Self::mix_hash(&mut hash, r.get_height().to_bits() as u64);
                }
                DrawCommand::PushRoundedClip { rect, radius } => {
                    Self::mix_hash(&mut hash, 14);
                    Self::mix_hash(&mut hash, rect.get_x().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_y().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_width().to_bits() as u64);
                    Self::mix_hash(&mut hash, rect.get_height().to_bits() as u64);
                    Self::mix_hash(&mut hash, radius.get_top_left().to_bits() as u64);
                    Self::mix_hash(&mut hash, radius.get_top_right().to_bits() as u64);
                    Self::mix_hash(&mut hash, radius.get_bottom_right().to_bits() as u64);
                    Self::mix_hash(&mut hash, radius.get_bottom_left().to_bits() as u64);
                }
                DrawCommand::PopClip => {
                    Self::mix_hash(&mut hash, 12);
                }
            }
        }
        if hash == 0 { 1 } else { hash }
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
        let active_hover_target: Option<NodeId> = Self::resolve_active_hover_target(tree);
        let mut portals: Vec<(i32, usize, NodeId)> = Vec::new();
        Self::emit_node(
            tree,
            tree.get_root(),
            (Point::new(0.0, 0.0), 1.0),
            viewport_rect,
            active_hover_target,
            Some(&mut portals),
            display_list,
        );
        display_list.set_portal_split_index(display_list.len());
        portals.sort_by(|a: &(i32, usize, NodeId), b: &(i32, usize, NodeId)| {
            a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))
        });
        for (_, _, portal_id) in portals.into_iter() {
            Self::emit_node(
                tree,
                portal_id,
                (Point::new(0.0, 0.0), 1.0),
                viewport_rect,
                active_hover_target,
                None,
                display_list,
            );
        }
    }

    /// Resolves the single innermost interactive non-scrim node along the active hover chain so nested controls never stack duplicate hover fills.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Innermost interactive node eligible for hover tinting.
    #[must_use]
    fn resolve_active_hover_target(tree: &SceneTree) -> Option<NodeId> {
        let mut cursor: Option<NodeId> = tree.get_focus().hovered_node();
        while let Some(id) = cursor {
            let Some(node) = tree.get_node(id) else {
                break;
            };
            let style: Style = node.get_style();
            let paint: PaintStyle = style.get_paint();
            let is_scrim: bool = matches!(node.get_kind(), NodeKind::Portal)
                || style.get_layout().get_position() == Position::Fixed;
            let is_transparent_canvas: bool =
                matches!(node.get_kind(), NodeKind::Canvas) && paint.background_color().is_none();
            let is_interactive: bool = paint.get_cursor() == CursorIcon::Pointer
                || node
                    .get_handlers()
                    .handler_for(&UiEventKind::Click(Point::default()))
                    .is_some();
            if !is_scrim && !is_transparent_canvas && is_interactive {
                return Some(id);
            }
            cursor = node.parent_id();
        }
        None
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

    /// Recursively emits commands for `node_id` and its z-sorted children with hierarchical transform and physical-pixel lockstep.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Scene tree reference.
    /// - `NodeId` - Node to paint.
    /// - `(Point, f32)` - Accumulated ancestor physical pixel displacement `(tx_phys - scroll_phys)` and opacity.
    /// - `Rect` - Active cull clip rectangle.
    /// - `Option<NodeId>` - Innermost hovered interactive node eligible for automatic hover tinting.
    /// - `Option<&mut Vec<(i32, usize, NodeId)>>` - Optional deferred top-layer portal queue (`None` when painting top layer).
    /// - `&mut DisplayList` - Output command list.
    fn emit_node(
        tree: &SceneTree,
        node_id: NodeId,
        offset_and_opacity: (Point, f32),
        cull_rect: Rect,
        active_hover_target: Option<NodeId>,
        mut portals: Option<&mut Vec<(i32, usize, NodeId)>>,
        display_list: &mut DisplayList,
    ) {
        let (offset_phys, parent_opacity): (Point, f32) = offset_and_opacity;
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
        let is_scrim: bool =
            matches!(node.get_kind(), NodeKind::Portal) || layout.get_position() == Position::Fixed;
        let is_interactive: bool = paint.get_cursor() == CursorIcon::Pointer
            || node
                .get_handlers()
                .handler_for(&UiEventKind::Click(Point::default()))
                .is_some();
        let is_transparent_canvas: bool =
            matches!(node.get_kind(), NodeKind::Canvas) && paint.background_color().is_none();
        let hover_visual: bool = hovered && active_hover_target == Some(node_id);
        if is_interactive
            && !is_scrim
            && !is_transparent_canvas
            && node.hover_paint().is_none()
            && (hover_visual || pressed)
        {
            paint.set_background(Self::interactive_tint(
                paint.background_color(),
                hover_visual,
                pressed,
            ));
        }
        let opacity: f32 = (parent_opacity * paint.get_opacity()).clamp(0.0, 1.0);
        if opacity <= 0.0 {
            return;
        }
        let scale: f32 = display_list.get_scale_factor().max(0.5);
        let inv_scale: f32 = 1.0 / scale;
        let raw_bounds: Rect = node.get_layout_box().get_bounds();
        let transform: Transform2D = paint.get_transform();
        let tx_phys: f32 = (transform.get_translate_x() * scale).round();
        let ty_phys: f32 = (transform.get_translate_y() * scale).round();
        let total_off_x: f32 = offset_phys.get_x() + tx_phys;
        let total_off_y: f32 = offset_phys.get_y() + ty_phys;
        let raw_x0: f32 = (raw_bounds.get_x() * scale).round();
        let raw_y0: f32 = (raw_bounds.get_y() * scale).round();
        let raw_x1: f32 = (raw_bounds.right() * scale).round();
        let raw_y1: f32 = (raw_bounds.bottom() * scale).round();
        let static_w_phys: f32 = (raw_x1 - raw_x0).max(0.0);
        let static_h_phys: f32 = (raw_y1 - raw_y0).max(0.0);
        let scaled_w_phys: f32 = (static_w_phys * transform.get_scale_x()).round().max(0.0);
        let scaled_h_phys: f32 = (static_h_phys * transform.get_scale_y()).round().max(0.0);
        let scale_dx_phys: f32 = ((static_w_phys - scaled_w_phys) * 0.5).round();
        let scale_dy_phys: f32 = ((static_h_phys - scaled_h_phys) * 0.5).round();
        let bounds: Rect = Rect::new(
            (raw_x0 + total_off_x + scale_dx_phys) * inv_scale,
            (raw_y0 + total_off_y + scale_dy_phys) * inv_scale,
            scaled_w_phys * inv_scale,
            scaled_h_phys * inv_scale,
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
                    let cx0: f32 = (content_b.get_x() * scale).round();
                    let cy0: f32 = (content_b.get_y() * scale).round();
                    let shifted_content: Rect = Rect::new(
                        (cx0 + total_off_x) * inv_scale,
                        (cy0 + total_off_y) * inv_scale,
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
                    let cx0: f32 = (content_b.get_x() * scale).round();
                    let cy0: f32 = (content_b.get_y() * scale).round();
                    let shifted_content: Rect = Rect::new(
                        (cx0 + total_off_x) * inv_scale,
                        (cy0 + total_off_y) * inv_scale,
                        content_b.get_width(),
                        content_b.get_height(),
                    );
                    let line_h: f32 =
                        (text_style.get_font_size() * text_style.get_line_height()).max(1.0);
                    let text_rect: Rect = if input_state.get_multiline() {
                        shifted_content
                    } else {
                        text_style.set_overflow(TextOverflow::Clip);
                        let v_off_phys: f32 =
                            (((shifted_content.get_height() - line_h) * 0.5).max(0.0) * scale)
                                .round();
                        let text_y: f32 = (cy0 + total_off_y + v_off_phys) * inv_scale;
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
                        let caret_h: f32 = (text_style.get_font_size() * 1.15)
                            .min(shifted_content.get_height().max(12.0));
                        let caret_text: String = if input_state.get_value().is_empty() {
                            String::new()
                        } else {
                            rendered_text
                        };
                        display_list.push(DrawCommand::TextCaret {
                            text_rect,
                            content_rect: shifted_content,
                            text: caret_text,
                            style: text_style,
                            caret_height: caret_h,
                            color: style.get_text().get_color().multiply_alpha(opacity),
                            multiline: input_state.get_multiline(),
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
                        let canvas_bw: f32 = paint.get_border().get_width();
                        let canvas_clip_rect: Rect = if canvas_bw > 0.0 {
                            Rect::new(
                                bounds.get_x() + canvas_bw,
                                bounds.get_y() + canvas_bw,
                                (bounds.get_width() - canvas_bw * 2.0).max(0.0),
                                (bounds.get_height() - canvas_bw * 2.0).max(0.0),
                            )
                        } else {
                            bounds
                        };
                        let canvas_max_r: f32 = (bounds.get_width() * 0.5)
                            .min(bounds.get_height() * 0.5)
                            .max(0.0);
                        let canvas_raw_r: CornerRadius = paint.get_radius();
                        let canvas_inner_r: CornerRadius = CornerRadius::new(
                            (canvas_raw_r.get_top_left().min(canvas_max_r) - canvas_bw).max(0.0),
                            (canvas_raw_r.get_top_right().min(canvas_max_r) - canvas_bw).max(0.0),
                            (canvas_raw_r.get_bottom_right().min(canvas_max_r) - canvas_bw)
                                .max(0.0),
                            (canvas_raw_r.get_bottom_left().min(canvas_max_r) - canvas_bw).max(0.0),
                        );
                        if canvas_inner_r.max_radius() > 0.25 {
                            display_list.push(DrawCommand::PushRoundedClip {
                                rect: canvas_clip_rect,
                                radius: canvas_inner_r,
                            });
                        } else {
                            display_list.push(DrawCommand::PushClip(canvas_clip_rect));
                        }
                        func(bounds, display_list);
                        display_list.push(DrawCommand::PopClip);
                    }
                }
                NodeKind::View | NodeKind::ScrollView | NodeKind::Portal | NodeKind::Fragment => {}
            }
        }
        if clipped {
            let bw: f32 = paint.get_border().get_width();
            let clip_rect: Rect = if bw > 0.0 {
                Rect::new(
                    bounds.get_x() + bw,
                    bounds.get_y() + bw,
                    (bounds.get_width() - bw * 2.0).max(0.0),
                    (bounds.get_height() - bw * 2.0).max(0.0),
                )
            } else {
                bounds
            };
            let max_r: f32 = (bounds.get_width() * 0.5)
                .min(bounds.get_height() * 0.5)
                .max(0.0);
            let raw_r: CornerRadius = paint.get_radius();
            let inner_r: CornerRadius = CornerRadius::new(
                (raw_r.get_top_left().min(max_r) - bw).max(0.0),
                (raw_r.get_top_right().min(max_r) - bw).max(0.0),
                (raw_r.get_bottom_right().min(max_r) - bw).max(0.0),
                (raw_r.get_bottom_left().min(max_r) - bw).max(0.0),
            );
            if inner_r.max_radius() > 0.25 {
                display_list.push(DrawCommand::PushRoundedClip {
                    rect: clip_rect,
                    radius: inner_r,
                });
            } else {
                display_list.push(DrawCommand::PushClip(clip_rect));
            }
        }
        let raw_node_scroll: Point = node.get_layout_box().get_scroll_offset();
        let scroll_x_phys: f32 = (raw_node_scroll.get_x() * scale).round();
        let scroll_y_phys: f32 = (raw_node_scroll.get_y() * scale).round();
        let child_offset_phys: Point =
            Point::new(total_off_x - scroll_x_phys, total_off_y - scroll_y_phys);
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
                    (child_offset_phys, opacity),
                    next_cull,
                    active_hover_target,
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
                    (child_offset_phys, opacity),
                    next_cull,
                    active_hover_target,
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
                let track_x_snapped: f32 = (track_x * scale).round() * inv_scale;
                let track_y_snapped: f32 = (track_y * scale).round() * inv_scale;
                let track_w_snapped: f32 = (track_w * scale).round().max(1.0) * inv_scale;
                let track_h_snapped: f32 = (track_h * scale).round().max(1.0) * inv_scale;
                let thumb_y_snapped: f32 = (thumb_y * scale).round() * inv_scale;
                let thumb_h_snapped: f32 = (thumb_h * scale).round().max(1.0) * inv_scale;
                if hovered {
                    display_list.push(DrawCommand::FillRoundedRect {
                        rect: Rect::new(
                            track_x_snapped,
                            track_y_snapped,
                            track_w_snapped,
                            track_h_snapped,
                        ),
                        color: Color::rgba(113, 113, 122, 24).multiply_alpha(opacity),
                        radius: CornerRadius::all(3.0),
                        border: Border::default(),
                    });
                }
                let thumb_alpha: u8 = if hovered { 180 } else { 112 };
                display_list.push(DrawCommand::FillRoundedRect {
                    rect: Rect::new(
                        track_x_snapped,
                        thumb_y_snapped,
                        track_w_snapped,
                        thumb_h_snapped,
                    ),
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
