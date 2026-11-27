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

/// Telemetry and invalidation helpers for `PaintFragmentCache`.
impl PaintFragmentCache {
    /// Clears cached whole-list signature and all retained subtree fragments.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable fragment cache reference.
    pub fn clear(&mut self) {
        let Self {
            has_last_scene,
            last_has_canvas,
            slots,
            ..
        } = self;
        *has_last_scene = false;
        *last_has_canvas = false;
        for slot in slots.iter_mut() {
            *slot = None;
        }
    }

    /// Returns cumulative retention telemetry `(whole_list_hits, fragment_hits, fragment_misses, fragment_evictions)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Fragment cache reference.
    ///
    /// # Returns
    ///
    /// - `(u64, u64, u64, u64)` - Tuple of whole-list hits, fragment hits, misses, and evictions.
    #[must_use]
    pub fn stats(&self) -> (u64, u64, u64, u64) {
        (
            self.get_whole_list_hits(),
            self.get_fragment_hits(),
            self.get_fragment_misses(),
            self.get_fragment_evictions(),
        )
    }

    /// Ensures the bounded direct-mapped slot table is allocated with `PAINT_FRAGMENT_CACHE_SLOTS` entries.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable fragment cache reference.
    #[inline]
    fn ensure_slots(&mut self) {
        let Self { slots, .. } = self;
        if slots.len() != PAINT_FRAGMENT_CACHE_SLOTS {
            slots.resize_with(PAINT_FRAGMENT_CACHE_SLOTS, || None);
        }
    }
}

/// Scene-to-display-list compilation pass.
impl PaintPass {
    /// Computes a deterministic 64-bit signature of the active text selection state.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    ///
    /// # Returns
    ///
    /// - `u64` - Selection digest (`0` when no text selection is active).
    #[inline]
    #[must_use]
    fn text_selection_sig(tree: &SceneTree) -> u64 {
        let Some(sel) = tree.text_selection() else {
            return 0;
        };
        if sel.is_empty() {
            return 0;
        }
        let (start, end): (usize, usize) = sel.ordered_range();
        let nid: NodeId = sel.get_node_id();
        let mut h: u64 = ((nid.get_index() as u64) << 32) | (nid.get_generation() as u64);
        h = (h ^ (start as u64)).wrapping_mul(PAINT_FRAGMENT_FX_MULT);
        h = (h ^ (end as u64)).wrapping_mul(PAINT_FRAGMENT_FX_MULT);
        if h == 0 { 1 } else { h }
    }

    /// Folds one `u64` word into a running FxHash accumulator.
    ///
    /// # Arguments
    ///
    /// - `&mut u64` - Running hash accumulator.
    /// - `u64` - Word to fold.
    #[inline(always)]
    fn fx_fold(hash: &mut u64, word: u64) {
        *hash = (*hash ^ word).wrapping_mul(PAINT_FRAGMENT_FX_MULT);
    }

    /// Folds a `Rect`'s exact IEEE-754 bit pattern into `hash`.
    ///
    /// # Arguments
    ///
    /// - `&mut u64` - Running hash accumulator.
    /// - `Rect` - Rectangle to hash.
    #[inline(always)]
    fn fx_fold_rect(hash: &mut u64, rect: Rect) {
        let lo: u64 = ((rect.get_x().to_bits() as u64) << 32) | (rect.get_y().to_bits() as u64);
        let hi: u64 =
            ((rect.get_width().to_bits() as u64) << 32) | (rect.get_height().to_bits() as u64);
        Self::fx_fold(hash, lo);
        Self::fx_fold(hash, hi);
    }

    /// Folds a `PaintStyle`'s visual fields into `hash`.
    ///
    /// # Arguments
    ///
    /// - `&mut u64` - Running hash accumulator.
    /// - `&PaintStyle` - Paint style reference.
    #[inline]
    fn fx_fold_paint(hash: &mut u64, paint: &PaintStyle) {
        let bg_bits: u64 = paint
            .background_color()
            .map(|c: Color| u32::from_le_bytes(c.to_rgba_bytes()) as u64 | (1_u64 << 32))
            .unwrap_or(0);
        Self::fx_fold(hash, bg_bits);
        if let Some(grad) = paint.background_gradient() {
            let g0: u64 = u32::from_le_bytes(grad.get_start_color().to_rgba_bytes()) as u64;
            let g1: u64 = u32::from_le_bytes(grad.get_end_color().to_rgba_bytes()) as u64;
            Self::fx_fold(hash, (g0 << 32) | g1);
            Self::fx_fold(hash, grad.get_angle_deg().to_bits() as u64 | (1_u64 << 33));
        }
        let border: Border = paint.get_border();
        let b_word: u64 = ((border.get_width().to_bits() as u64) << 32)
            | (u32::from_le_bytes(border.get_color().to_rgba_bytes()) as u64);
        Self::fx_fold(hash, b_word);
        let r: CornerRadius = paint.get_radius();
        let r0: u64 =
            ((r.get_top_left().to_bits() as u64) << 32) | (r.get_top_right().to_bits() as u64);
        let r1: u64 = ((r.get_bottom_right().to_bits() as u64) << 32)
            | (r.get_bottom_left().to_bits() as u64);
        Self::fx_fold(hash, r0);
        Self::fx_fold(hash, r1);
        Self::fx_fold(hash, paint.get_opacity().to_bits() as u64);
        if let Some(sh) = paint.box_shadow() {
            let s0: u64 =
                ((sh.get_offset_x().to_bits() as u64) << 32) | (sh.get_offset_y().to_bits() as u64);
            let s1: u64 = ((sh.get_blur_radius().to_bits() as u64) << 32)
                | (sh.get_spread_radius().to_bits() as u64);
            Self::fx_fold(hash, s0);
            Self::fx_fold(hash, s1);
            Self::fx_fold(
                hash,
                u32::from_le_bytes(sh.get_color().to_rgba_bytes()) as u64 | (1_u64 << 34),
            );
        }
    }

    /// Folds a `TextStyle` and UTF-8 string bytes into `hash`.
    ///
    /// # Arguments
    ///
    /// - `&mut u64` - Running hash accumulator.
    /// - `&TextStyle` - Typography style reference.
    /// - `&str` - Text content slice.
    #[inline]
    fn fx_fold_text(hash: &mut u64, text_style: &TextStyle, text: &str) {
        let m0: u64 = ((text_style.get_font_size().to_bits() as u64) << 32)
            | (text_style.get_line_height().to_bits() as u64);
        let m1: u64 = ((text_style.get_letter_spacing().to_bits() as u64) << 32)
            | (u32::from_le_bytes(text_style.get_color().to_rgba_bytes()) as u64);
        let flags: u64 = (text_style.get_family() as u64)
            | ((text_style.get_weight() as u64) << 8)
            | ((text_style.get_align() as u64) << 16)
            | ((text_style.get_overflow() as u64) << 24)
            | (u64::from(text_style.get_italic()) << 32);
        Self::fx_fold(hash, m0);
        Self::fx_fold(hash, m1);
        Self::fx_fold(hash, flags);
        let bytes: &[u8] = text.as_bytes();
        Self::fx_fold(hash, bytes.len() as u64);
        let (chunks, rem): (&[[u8; 8]], &[u8]) = bytes.as_chunks::<8>();
        for chunk in chunks.iter() {
            Self::fx_fold(hash, u64::from_ne_bytes(*chunk));
        }
        if !rem.is_empty() {
            let mut tail: u64 = 0;
            for (idx, &b) in rem.iter().enumerate() {
                tail |= (b as u64) << (idx * 8);
            }
            Self::fx_fold(hash, tail);
        }
    }

    /// Computes `(geom_hash, style_text_hash)` for an eligible shallow clipped container, or `None` if uncacheable.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained scene tree.
    /// - `NodeId` - Candidate container node handle.
    /// - `&SceneNode` - Candidate container node reference.
    /// - `Option<NodeId>` - Active innermost hover target.
    /// - `Option<NodeId>` - Active text selection node ID, if non-empty.
    ///
    /// # Returns
    ///
    /// - `Option<(u64, u64)>` - Tuple `(geom_hash, style_text_hash)` when the subtree is pure and cache-eligible.
    #[inline]
    #[must_use]
    fn fingerprint_shallow_container(
        tree: &SceneTree,
        node_id: NodeId,
        node: &SceneNode,
        active_hover_target: Option<NodeId>,
        selection_node: Option<NodeId>,
    ) -> Option<(u64, u64)> {
        let children: &[NodeId] = node.get_children().as_slice();
        if children.is_empty() || children.len() > PAINT_FRAGMENT_MAX_CHILDREN {
            return None;
        }
        let style: &Style = node.style_ref();
        let transform: Transform2D = style.paint.get_transform();
        let scroll: Point = node.layout_box.get_scroll_offset();
        let mut geom_h: u64 = children.len() as u64;
        Self::fx_fold_rect(&mut geom_h, node.layout_box.get_bounds());
        Self::fx_fold(
            &mut geom_h,
            ((transform.get_translate_x().to_bits() as u64) << 32)
                | (transform.get_translate_y().to_bits() as u64),
        );
        Self::fx_fold(
            &mut geom_h,
            ((transform.get_scale_x().to_bits() as u64) << 32)
                | (transform.get_scale_y().to_bits() as u64),
        );
        Self::fx_fold(
            &mut geom_h,
            ((scroll.get_x().to_bits() as u64) << 32) | (scroll.get_y().to_bits() as u64),
        );
        let mut style_h: u64 =
            ((node_id.get_index() as u64) << 32) | (node_id.get_generation() as u64);
        Self::fx_fold_paint(&mut style_h, &style.paint);
        for &cid in children.iter() {
            if active_hover_target == Some(cid) || selection_node == Some(cid) {
                return None;
            }
            let child: &SceneNode = tree.get_node(cid)?;
            if !child.get_children().is_empty() {
                return None;
            }
            let c_inter: InteractionState = child.get_interaction();
            let c_active: bool = c_inter.get_hovered() || c_inter.get_pressed();
            if c_active || c_inter.get_focused() {
                return None;
            }
            let c_style: &Style = child.style_ref();
            if c_style.layout.get_position() == Position::Fixed {
                return None;
            }
            let c_tag: u64 = match child.get_kind() {
                NodeKind::View | NodeKind::Fragment => 1,
                NodeKind::Text(text) => {
                    Self::fx_fold_text(&mut style_h, &c_style.text, text.as_str());
                    Self::fx_fold_rect(&mut geom_h, child.layout_box.get_content_bounds());
                    2
                }
                NodeKind::Image(buf) => {
                    Self::fx_fold(
                        &mut style_h,
                        ((buf.get_width() as u64) << 32) | (buf.get_height() as u64),
                    );
                    Self::fx_fold(&mut style_h, buf.get_pixels().as_ptr() as usize as u64);
                    3
                }
                NodeKind::Portal | NodeKind::Canvas | NodeKind::Input(_) | NodeKind::ScrollView => {
                    return None;
                }
            };
            let c_id_bits: u64 = ((cid.get_index() as u64) << 32) | (cid.get_generation() as u64);
            Self::fx_fold(&mut geom_h, c_id_bits);
            Self::fx_fold_rect(&mut geom_h, child.layout_box.get_bounds());
            let ct: Transform2D = c_style.paint.get_transform();
            Self::fx_fold(
                &mut geom_h,
                ((ct.get_translate_x().to_bits() as u64) << 32)
                    | (ct.get_translate_y().to_bits() as u64),
            );
            Self::fx_fold(
                &mut geom_h,
                ((ct.get_scale_x().to_bits() as u64) << 32) | (ct.get_scale_y().to_bits() as u64),
            );
            let c_lay_bits: u64 = (c_tag << 40)
                | ((c_style.layout.get_display() as u64) << 32)
                | ((c_style.layout.get_overflow() as u64) << 24)
                | ((c_style.layout.get_z_index() as u32) as u64);
            Self::fx_fold(&mut style_h, c_lay_bits);
            Self::fx_fold_paint(&mut style_h, &c_style.paint);
        }
        Some((geom_h, style_h))
    }

    /// Shifts one `DrawCommand`'s logical coordinates by `(dx, dy)` for dyadic-scale scroll fragment reuse.
    ///
    /// # Arguments
    ///
    /// - `&DrawCommand` - Cached draw command.
    /// - `f32` - Logical X translation delta.
    /// - `f32` - Logical Y translation delta.
    ///
    /// # Returns
    ///
    /// - `DrawCommand` - Translated command copy.
    #[inline]
    #[must_use]
    fn translate_command(cmd: &DrawCommand, dx: f32, dy: f32) -> DrawCommand {
        match cmd {
            DrawCommand::Clear(c) => DrawCommand::Clear(*c),
            DrawCommand::FillRoundedRect {
                rect,
                color,
                radius,
                border,
            } => DrawCommand::FillRoundedRect {
                rect: Rect::new(
                    rect.get_x() + dx,
                    rect.get_y() + dy,
                    rect.get_width(),
                    rect.get_height(),
                ),
                color: *color,
                radius: *radius,
                border: *border,
            },
            DrawCommand::LinearGradientRect {
                rect,
                gradient,
                radius,
            } => DrawCommand::LinearGradientRect {
                rect: Rect::new(
                    rect.get_x() + dx,
                    rect.get_y() + dy,
                    rect.get_width(),
                    rect.get_height(),
                ),
                gradient: *gradient,
                radius: *radius,
            },
            DrawCommand::BoxShadow {
                rect,
                radius,
                shadow,
            } => DrawCommand::BoxShadow {
                rect: Rect::new(
                    rect.get_x() + dx,
                    rect.get_y() + dy,
                    rect.get_width(),
                    rect.get_height(),
                ),
                radius: *radius,
                shadow: *shadow,
            },
            DrawCommand::TextRun { rect, text, style } => DrawCommand::TextRun {
                rect: Rect::new(
                    rect.get_x() + dx,
                    rect.get_y() + dy,
                    rect.get_width(),
                    rect.get_height(),
                ),
                text: text.clone(),
                style: *style,
            },
            DrawCommand::TextCaret {
                text_rect,
                content_rect,
                text,
                style,
                caret_height,
                color,
                multiline,
            } => DrawCommand::TextCaret {
                text_rect: Rect::new(
                    text_rect.get_x() + dx,
                    text_rect.get_y() + dy,
                    text_rect.get_width(),
                    text_rect.get_height(),
                ),
                content_rect: Rect::new(
                    content_rect.get_x() + dx,
                    content_rect.get_y() + dy,
                    content_rect.get_width(),
                    content_rect.get_height(),
                ),
                text: text.clone(),
                style: *style,
                caret_height: *caret_height,
                color: *color,
                multiline: *multiline,
            },
            DrawCommand::Image {
                rect,
                buffer,
                radius,
            } => DrawCommand::Image {
                rect: Rect::new(
                    rect.get_x() + dx,
                    rect.get_y() + dy,
                    rect.get_width(),
                    rect.get_height(),
                ),
                buffer: buffer.clone(),
                radius: *radius,
            },
            DrawCommand::Line {
                from,
                to,
                color,
                width,
            } => DrawCommand::Line {
                from: Point::new(from.get_x() + dx, from.get_y() + dy),
                to: Point::new(to.get_x() + dx, to.get_y() + dy),
                color: *color,
                width: *width,
            },
            DrawCommand::Circle {
                center,
                radius,
                color,
            } => DrawCommand::Circle {
                center: Point::new(center.get_x() + dx, center.get_y() + dy),
                radius: *radius,
                color: *color,
            },
            DrawCommand::Triangle { p0, p1, p2, color } => DrawCommand::Triangle {
                p0: Point::new(p0.get_x() + dx, p0.get_y() + dy),
                p1: Point::new(p1.get_x() + dx, p1.get_y() + dy),
                p2: Point::new(p2.get_x() + dx, p2.get_y() + dy),
                color: *color,
            },
            DrawCommand::SpriteRegion {
                rect,
                buffer,
                src_rect,
                tint,
            } => DrawCommand::SpriteRegion {
                rect: Rect::new(
                    rect.get_x() + dx,
                    rect.get_y() + dy,
                    rect.get_width(),
                    rect.get_height(),
                ),
                buffer: buffer.clone(),
                src_rect: *src_rect,
                tint: *tint,
            },
            DrawCommand::PushClip(rect) => DrawCommand::PushClip(Rect::new(
                rect.get_x() + dx,
                rect.get_y() + dy,
                rect.get_width(),
                rect.get_height(),
            )),
            DrawCommand::PushRoundedClip { rect, radius } => DrawCommand::PushRoundedClip {
                rect: Rect::new(
                    rect.get_x() + dx,
                    rect.get_y() + dy,
                    rect.get_width(),
                    rect.get_height(),
                ),
                radius: *radius,
            },
            DrawCommand::PopClip => DrawCommand::PopClip,
        }
    }

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
        let mut saw_canvas: bool = false;
        Self::build_inner(
            tree,
            viewport,
            scale_factor,
            clear_color,
            display_list,
            None,
            &mut saw_canvas,
        );
    }

    /// Packs an optional `NodeId` into a single `u64` (`u64::MAX` when `None`).
    ///
    /// # Arguments
    ///
    /// - `Option<NodeId>` - Optional node handle.
    ///
    /// # Returns
    ///
    /// - `u64` - Packed `(index << 32) | generation` or `u64::MAX`.
    #[inline(always)]
    #[must_use]
    fn pack_opt_node_id(node: Option<NodeId>) -> u64 {
        match node {
            Some(id) => ((id.get_index() as u64) << 32) | (id.get_generation() as u64),
            None => u64::MAX,
        }
    }

    /// Compiles `tree` into `display_list` with cross-frame whole-list retention and subtree `DrawCommand` fragment caching.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Laid-out scene tree.
    /// - `Size` - Logical viewport size.
    /// - `f32` - DPI scale factor.
    /// - `Color` - Surface clear background color.
    /// - `bool` - Whether layout recomputed any node geometry on this frame.
    /// - `&mut DisplayList` - Output reusable display list.
    /// - `&mut PaintFragmentCache` - Persistent cross-frame fragment cache.
    pub fn build_retained(
        tree: &SceneTree,
        viewport: Size,
        scale_factor: f32,
        clear_color: Color,
        layout_changed: bool,
        display_list: &mut DisplayList,
        cache: &mut PaintFragmentCache,
    ) {
        let clamped_scale: f32 = scale_factor.max(0.5);
        let focus: FocusManager = tree.get_focus();
        let sig: PaintSceneSignature = PaintSceneSignature {
            tree_ptr: (tree as *const SceneTree) as usize,
            tree_revision: tree.get_revision(),
            viewport_w_bits: viewport.get_width().to_bits(),
            viewport_h_bits: viewport.get_height().to_bits(),
            scale_bits: clamped_scale.to_bits(),
            clear_rgba: u32::from_le_bytes(clear_color.to_rgba_bytes()),
            focused_packed: Self::pack_opt_node_id(focus.focused_node()),
            hovered_packed: Self::pack_opt_node_id(focus.hovered_node()),
            pressed_packed: Self::pack_opt_node_id(focus.pressed_node()),
            caret_visible: tree.get_caret_visible(),
            text_sel_sig: Self::text_selection_sig(tree),
        };
        if !layout_changed
            && !cache.last_has_canvas
            && !display_list.is_empty()
            && cache.has_last_scene
            && cache.last_scene_sig == sig
        {
            cache.whole_list_hits = cache.whole_list_hits.wrapping_add(1);
            return;
        }
        cache.ensure_slots();
        let mut saw_canvas: bool = false;
        Self::build_inner(
            tree,
            viewport,
            clamped_scale,
            clear_color,
            display_list,
            Some(cache),
            &mut saw_canvas,
        );
        cache.last_has_canvas = saw_canvas;
        cache.has_last_scene = true;
        cache.last_scene_sig = sig;
    }

    /// Shared internal scene-to-display-list builder supporting optional retained fragment caching.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Laid-out scene tree.
    /// - `Size` - Logical viewport size.
    /// - `f32` - DPI scale factor.
    /// - `Color` - Surface clear background color.
    /// - `&mut DisplayList` - Output reusable display list.
    /// - `Option<&mut PaintFragmentCache>` - Optional cross-frame fragment cache.
    /// - `&mut bool` - Output flag set when any `NodeKind::Canvas` is visited.
    fn build_inner(
        tree: &SceneTree,
        viewport: Size,
        scale_factor: f32,
        clear_color: Color,
        display_list: &mut DisplayList,
        mut fragments: Option<&mut PaintFragmentCache>,
        saw_canvas: &mut bool,
    ) {
        display_list.reset(viewport, scale_factor);
        display_list.push(DrawCommand::Clear(clear_color));
        let viewport_rect: Rect = Rect::new(0.0, 0.0, viewport.get_width(), viewport.get_height());
        let active_hover_target: Option<NodeId> = Self::resolve_active_hover_target(tree);
        let mut portals: Vec<(i32, usize, NodeId)> = take(&mut display_list.portals_scratch);
        let mut z_sort: Vec<(i32, usize, NodeId)> = take(&mut display_list.z_sort_scratch);
        portals.clear();
        z_sort.clear();
        {
            let mut scratch: PaintScratch<'_> = PaintScratch {
                portals: Some(&mut portals),
                z_sort: &mut z_sort,
                fragments: fragments.as_deref_mut(),
                saw_canvas,
            };
            Self::emit_node(
                tree,
                tree.get_root(),
                (Point::new(0.0, 0.0), 1.0),
                viewport_rect,
                active_hover_target,
                &mut scratch,
                display_list,
            );
        }
        display_list.set_portal_split_index(display_list.len());
        if portals.len() > 1 {
            portals.sort_by(|a: &(i32, usize, NodeId), b: &(i32, usize, NodeId)| {
                a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))
            });
        }
        {
            let mut portal_scratch: PaintScratch<'_> = PaintScratch {
                portals: None,
                z_sort: &mut z_sort,
                fragments,
                saw_canvas,
            };
            for &(_, _, portal_id) in portals.iter() {
                Self::emit_node(
                    tree,
                    portal_id,
                    (Point::new(0.0, 0.0), 1.0),
                    viewport_rect,
                    active_hover_target,
                    &mut portal_scratch,
                    display_list,
                );
            }
        }
        portals.clear();
        z_sort.clear();
        display_list.portals_scratch = portals;
        display_list.z_sort_scratch = z_sort;
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
            let style: &Style = node.style_ref();
            let paint: &PaintStyle = &style.paint;
            let is_scrim: bool = matches!(node.get_kind(), NodeKind::Portal)
                || style.layout.get_position() == Position::Fixed;
            let is_transparent_canvas: bool =
                matches!(node.get_kind(), NodeKind::Canvas) && paint.background_color().is_none();
            if !is_scrim && !is_transparent_canvas {
                let is_interactive: bool = paint.get_cursor() == CursorIcon::Pointer
                    || node
                        .get_handlers()
                        .handler_for(&UiEventKind::Click(Point::default()))
                        .is_some();
                if is_interactive {
                    return Some(id);
                }
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
    /// - `&mut PaintScratch<'_>` - Reusable portal queue and nested z-sort scratch buffers.
    /// - `&mut DisplayList` - Output command list.
    fn emit_node(
        tree: &SceneTree,
        node_id: NodeId,
        offset_and_opacity: (Point, f32),
        cull_rect: Rect,
        active_hover_target: Option<NodeId>,
        scratch: &mut PaintScratch<'_>,
        display_list: &mut DisplayList,
    ) {
        let (offset_phys, parent_opacity): (Point, f32) = offset_and_opacity;
        let Some(node) = tree.get_node(node_id) else {
            return;
        };
        let style: &Style = node.style_ref();
        let layout: &LayoutStyle = &style.layout;
        if layout.get_display() == Display::None {
            return;
        }
        if let Some(portal_queue) = scratch.portals.as_deref_mut()
            && (matches!(node.get_kind(), NodeKind::Portal)
                || layout.get_position() == Position::Fixed)
        {
            portal_queue.push((layout.get_z_index(), portal_queue.len(), node_id));
            return;
        }
        let interaction: InteractionState = node.get_interaction();
        let hovered: bool = interaction.get_hovered();
        let pressed: bool = interaction.get_pressed();
        let hover_visual: bool = hovered && active_hover_target == Some(node_id);
        let mut paint_owned: Option<PaintStyle> = if hovered { node.hover_paint() } else { None };
        if (hover_visual || pressed)
            && paint_owned.is_none()
            && (hovered || node.hover_paint().is_none())
        {
            let base_paint: &PaintStyle = &style.paint;
            let is_scrim: bool = matches!(node.get_kind(), NodeKind::Portal)
                || layout.get_position() == Position::Fixed;
            let is_transparent_canvas: bool = matches!(node.get_kind(), NodeKind::Canvas)
                && base_paint.background_color().is_none();
            if !is_scrim && !is_transparent_canvas {
                let is_interactive: bool = base_paint.get_cursor() == CursorIcon::Pointer
                    || node
                        .get_handlers()
                        .handler_for(&UiEventKind::Click(Point::default()))
                        .is_some();
                if is_interactive {
                    let mut tinted: PaintStyle = *base_paint;
                    tinted.set_background(Self::interactive_tint(
                        tinted.background_color(),
                        hover_visual,
                        pressed,
                    ));
                    paint_owned = Some(tinted);
                }
            }
        }
        let paint: &PaintStyle = paint_owned.as_ref().unwrap_or(&style.paint);
        let opacity: f32 = (parent_opacity * paint.get_opacity()).clamp(0.0, 1.0);
        if opacity <= 0.0 {
            return;
        }
        let scale: f32 = display_list.get_scale_factor().max(0.5);
        let inv_scale: f32 = 1.0 / scale;
        let raw_bounds: Rect = node.layout_box.get_bounds();
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
        let mut frag_capture: Option<(usize, usize, u64, u64)> = None;
        if self_visible
            && scratch.fragments.is_some()
            && scratch.portals.is_some()
            && matches!(node.get_kind(), NodeKind::View)
            && layout.get_overflow() == Overflow::Hidden
            && !hovered
            && !pressed
            && !interaction.get_focused()
            && active_hover_target != Some(node_id)
        {
            let vis_x_in: bool = self_vis_rect.get_x() >= cull_rect.get_x()
                && self_vis_rect.right() <= cull_rect.right();
            let vis_y_in: bool = self_vis_rect.get_y() >= cull_rect.get_y()
                && self_vis_rect.bottom() <= cull_rect.bottom();
            if vis_x_in && vis_y_in {
                let sel_node: Option<NodeId> = tree
                    .text_selection()
                    .filter(|s: &SelectionState| !s.is_empty())
                    .map(|s: SelectionState| s.get_node_id());
                if let Some((geom_hash, style_text_hash)) = Self::fingerprint_shallow_container(
                    tree,
                    node_id,
                    node,
                    active_hover_target,
                    sel_node,
                ) {
                    let slot_idx: usize =
                        (node_id.get_index() as usize) & (PAINT_FRAGMENT_CACHE_SLOTS - 1);
                    let scale_bits: u32 = scale.to_bits();
                    let opacity_bits: u32 = parent_opacity.to_bits();
                    let dyadic_12: bool = scale == 1.0 || scale == 2.0;
                    let dyadic_scale: bool = dyadic_12 || scale == 4.0;
                    if let Some(cache) = scratch.fragments.as_deref_mut()
                        && let Some(cached) = cache.slots.get(slot_idx).and_then(Option::as_ref)
                        && cached.node_id == node_id
                        && cached.scale_bits == scale_bits
                        && cached.opacity_bits == opacity_bits
                        && cached.geom_hash == geom_hash
                        && cached.style_text_hash == style_text_hash
                    {
                        if cached.offset_phys == offset_phys {
                            display_list
                                .get_mut_commands()
                                .extend_from_slice(cached.commands.as_slice());
                            cache.fragment_hits = cache.fragment_hits.wrapping_add(1);
                            return;
                        }
                        if dyadic_scale {
                            let dx: f32 =
                                (offset_phys.get_x() - cached.offset_phys.get_x()) * inv_scale;
                            let dy: f32 =
                                (offset_phys.get_y() - cached.offset_phys.get_y()) * inv_scale;
                            let dst: &mut Vec<DrawCommand> = display_list.get_mut_commands();
                            dst.reserve(cached.commands.len());
                            for cmd in cached.commands.iter() {
                                dst.push(Self::translate_command(cmd, dx, dy));
                            }
                            cache.fragment_hits = cache.fragment_hits.wrapping_add(1);
                            return;
                        }
                    }
                    frag_capture = Some((display_list.len(), slot_idx, geom_hash, style_text_hash));
                }
            }
        }
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
            if interaction.get_focused()
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
                    let mut text_style: TextStyle = style.text;
                    text_style.set_color(text_style.get_color().multiply_alpha(opacity));
                    let content_b: Rect = node.layout_box.get_content_bounds();
                    let cx0: f32 = (content_b.get_x() * scale).round();
                    let cy0: f32 = (content_b.get_y() * scale).round();
                    let shifted_content: Rect = Rect::new(
                        (cx0 + total_off_x) * inv_scale,
                        (cy0 + total_off_y) * inv_scale,
                        content_b.get_width(),
                        content_b.get_height(),
                    );
                    if let Some(sel) = tree.text_selection()
                        && sel.get_node_id() == node_id
                        && !sel.is_empty()
                    {
                        let sel_color: Color = tree
                            .effective_selection_color(node_id)
                            .multiply_alpha(opacity);
                        let sel_rects: Vec<Rect> = compute_text_selection_rects(
                            text.as_str(),
                            text_style,
                            shifted_content,
                            sel.ordered_range(),
                        );
                        for sel_rect in sel_rects.into_iter() {
                            display_list.push(DrawCommand::FillRoundedRect {
                                rect: sel_rect,
                                color: sel_color,
                                radius: CornerRadius::all(2.0),
                                border: Border::default(),
                            });
                        }
                    }
                    display_list.push(DrawCommand::TextRun {
                        rect: shifted_content,
                        text: text.clone(),
                        style: text_style,
                    });
                }
                NodeKind::Input(input_state) => {
                    let mut text_style: TextStyle = style.text;
                    let is_empty_value: bool = input_state.get_value().is_empty();
                    let is_focused: bool = interaction.get_focused();
                    let rendered_text: String = if is_empty_value {
                        text_style.set_color(text_style.get_color().multiply_alpha(opacity * 0.55));
                        input_state.get_placeholder().clone()
                    } else if input_state.get_password() {
                        text_style.set_color(text_style.get_color().multiply_alpha(opacity));
                        "*".repeat(input_state.get_value().chars().count())
                    } else {
                        text_style.set_color(text_style.get_color().multiply_alpha(opacity));
                        input_state.get_value().clone()
                    };
                    let content_b: Rect = node.layout_box.get_content_bounds();
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
                    let run_rect: Rect =
                        if is_empty_value && is_focused && !rendered_text.is_empty() {
                            let clearance_phys: f32 =
                                (PLACEHOLDER_FOCUSED_CARET_CLEARANCE_PX * scale).round();
                            let clearance_logical: f32 = clearance_phys * inv_scale;
                            Rect::new(
                                text_rect.get_x() + clearance_logical,
                                text_rect.get_y(),
                                (text_rect.get_width() - clearance_logical).max(0.0),
                                text_rect.get_height(),
                            )
                        } else {
                            text_rect
                        };
                    let show_caret: bool = is_focused && tree.get_caret_visible();
                    let caret_text: String = if show_caret && !is_empty_value {
                        rendered_text.clone()
                    } else {
                        String::new()
                    };
                    display_list.push(DrawCommand::PushClip(shifted_content));
                    display_list.push(DrawCommand::TextRun {
                        rect: run_rect,
                        text: rendered_text,
                        style: text_style,
                    });
                    if show_caret {
                        let caret_h: f32 = (text_style.get_font_size() * 1.15)
                            .min(shifted_content.get_height().max(12.0));
                        display_list.push(DrawCommand::TextCaret {
                            text_rect,
                            content_rect: shifted_content,
                            text: caret_text,
                            style: text_style,
                            caret_height: caret_h,
                            color: style.text.get_color().multiply_alpha(opacity),
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
                    *scratch.saw_canvas = true;
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
        let raw_node_scroll: Point = node.layout_box.get_scroll_offset();
        let scroll_x_phys: f32 = (raw_node_scroll.get_x() * scale).round();
        let scroll_y_phys: f32 = (raw_node_scroll.get_y() * scale).round();
        let child_offset_phys: Point =
            Point::new(total_off_x - scroll_x_phys, total_off_y - scroll_y_phys);
        let raw_children: &[NodeId] = node.get_children().as_slice();
        let needs_z_sort: bool = raw_children.len() > 1
            && raw_children.iter().copied().any(|cid: NodeId| {
                tree.get_node(cid)
                    .map(|c: &SceneNode| c.style_ref().layout.get_z_index() != 0)
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
                    scratch,
                    display_list,
                );
            }
        } else {
            let sort_start: usize = scratch.z_sort.len();
            for (idx, cid) in raw_children.iter().copied().enumerate() {
                if let Some(c) = tree.get_node(cid) {
                    scratch
                        .z_sort
                        .push((c.style_ref().layout.get_z_index(), idx, cid));
                }
            }
            let sort_end: usize = scratch.z_sort.len();
            scratch.z_sort[sort_start..sort_end].sort_by(
                |a: &(i32, usize, NodeId), b: &(i32, usize, NodeId)| {
                    a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))
                },
            );
            for slot_idx in sort_start..sort_end {
                let child_id: NodeId = scratch.z_sort[slot_idx].2;
                Self::emit_node(
                    tree,
                    child_id,
                    (child_offset_phys, opacity),
                    next_cull,
                    active_hover_target,
                    scratch,
                    display_list,
                );
            }
            scratch.z_sort.truncate(sort_start);
        }
        let vp: Size = display_list.get_viewport();
        let show_scrollbar: bool = vp.get_width().min(vp.get_height()) >= 540.0;
        if show_scrollbar && layout.get_overflow() == Overflow::Scroll && bounds.get_height() > 24.0
        {
            let lbox: &LayoutBox = &node.layout_box;
            let border_w: f32 = paint.get_border().get_width();
            let inner_x: f32 = bounds.get_x() + border_w;
            let inner_y: f32 = bounds.get_y() + border_w;
            let inner_w: f32 = (bounds.get_width() - border_w * 2.0).max(1.0);
            let inner_h: f32 = (bounds.get_height() - border_w * 2.0).max(1.0);
            let view_w: f32 = lbox.get_content_bounds().get_width().max(1.0);
            let view_h: f32 = lbox.get_content_bounds().get_height().max(1.0);
            let (scroll_y, total_h): (f32, f32) = match node.virtual_scroll_metrics() {
                Some((vy, vh)) => (vy, vh),
                None => (
                    lbox.get_scroll_offset().get_y(),
                    lbox.get_content_size().get_height(),
                ),
            };
            let scroll_x: f32 = lbox.get_scroll_offset().get_x();
            let total_w: f32 = lbox.get_content_size().get_width();
            let max_scroll_y: f32 = (total_h - view_h).max(0.0);
            let max_scroll_x: f32 = (total_w - view_w).max(0.0);
            let has_v_bar: bool = max_scroll_y > 1.0;
            let has_h_bar: bool = max_scroll_x > 1.0 && bounds.get_width() > 24.0;
            let corner_inset: f32 = if has_v_bar && has_h_bar { 8.0 } else { 0.0 };
            if has_v_bar {
                let track_margin: f32 = 3.0;
                let track_w: f32 = if hovered { 6.0 } else { 4.0 };
                let track_x: f32 = (inner_x + inner_w - track_w - 2.0).max(inner_x);
                let track_y: f32 = inner_y + track_margin;
                let track_h: f32 = (inner_h - track_margin * 2.0 - corner_inset).max(16.0);
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
            if has_h_bar {
                let track_margin: f32 = 3.0;
                let track_h: f32 = if hovered { 6.0 } else { 4.0 };
                let track_y: f32 = (inner_y + inner_h - track_h - 2.0).max(inner_y);
                let track_x: f32 = inner_x + track_margin;
                let track_w: f32 = (inner_w - track_margin * 2.0 - corner_inset).max(16.0);
                let ratio: f32 = (view_w / total_w.max(view_w)).clamp(0.0, 1.0);
                let min_thumb_w: f32 = 20.0_f32.min(track_w);
                let thumb_w: f32 = (track_w * ratio).clamp(min_thumb_w, track_w);
                let scroll_frac: f32 = (scroll_x / max_scroll_x).clamp(0.0, 1.0);
                let thumb_x: f32 = track_x + (track_w - thumb_w) * scroll_frac;
                let track_x_snapped: f32 = (track_x * scale).round() * inv_scale;
                let track_y_snapped: f32 = (track_y * scale).round() * inv_scale;
                let track_w_snapped: f32 = (track_w * scale).round().max(1.0) * inv_scale;
                let track_h_snapped: f32 = (track_h * scale).round().max(1.0) * inv_scale;
                let thumb_x_snapped: f32 = (thumb_x * scale).round() * inv_scale;
                let thumb_w_snapped: f32 = (thumb_w * scale).round().max(1.0) * inv_scale;
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
                        thumb_x_snapped,
                        track_y_snapped,
                        thumb_w_snapped,
                        track_h_snapped,
                    ),
                    color: Color::rgba(113, 113, 122, thumb_alpha).multiply_alpha(opacity),
                    radius: CornerRadius::all(track_h * 0.5),
                    border: Border::default(),
                });
            }
        }
        if clipped {
            display_list.push(DrawCommand::PopClip);
        }
        if let Some((cmd_start, slot_idx, geom_hash, style_text_hash)) = frag_capture
            && let Some(cache) = scratch.fragments.as_deref_mut()
            && let Some(slot_ref) = cache.slots.get_mut(slot_idx)
        {
            let cmd_slice: &[DrawCommand] = &display_list.get_commands()[cmd_start..];
            cache.fragment_misses = cache.fragment_misses.wrapping_add(1);
            if let Some(existing) = slot_ref.as_mut() {
                if existing.node_id != node_id {
                    cache.fragment_evictions = cache.fragment_evictions.wrapping_add(1);
                }
                existing.node_id = node_id;
                existing.scale_bits = scale.to_bits();
                existing.opacity_bits = parent_opacity.to_bits();
                existing.offset_phys = offset_phys;
                existing.geom_hash = geom_hash;
                existing.style_text_hash = style_text_hash;
                existing.commands.clear();
                existing.commands.extend_from_slice(cmd_slice);
            } else {
                *slot_ref = Some(RetainedPaintFragment {
                    node_id,
                    scale_bits: scale.to_bits(),
                    opacity_bits: parent_opacity.to_bits(),
                    offset_phys,
                    geom_hash,
                    style_text_hash,
                    commands: cmd_slice.to_vec(),
                });
            }
        }
    }
}
