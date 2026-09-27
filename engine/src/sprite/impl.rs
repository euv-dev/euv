use super::*;

/// Implements frame extraction, rendering, and animation creation for `SpriteSheet`.
impl SpriteSheet {
    /// Creates a new sprite sheet from an image element and uniform frame dimensions.
    ///
    /// Computes the grid columns and rows from the image dimensions and frame size.
    ///
    /// # Arguments
    ///
    /// - `HtmlImageElement` - The sprite sheet image.
    /// - `f64` - The width of each frame.
    /// - `f64` - The height of each frame.
    ///
    /// # Returns
    ///
    /// - `SpriteSheet` - The new sprite sheet.
    pub fn from_image(image: HtmlImageElement, frame_width: f64, frame_height: f64) -> SpriteSheet {
        let total_width: f64 = image.width() as f64;
        let total_height: f64 = image.height() as f64;
        let columns: u32 = (total_width / frame_width).max(1.0) as u32;
        let rows: u32 = (total_height / frame_height).max(1.0) as u32;
        SpriteSheet::new(image, frame_width, frame_height, columns, rows)
    }

    /// Returns the source rectangle for the frame at the given grid index.
    ///
    /// # Arguments
    ///
    /// - `u32` - The frame index (left-to-right, top-to-bottom).
    ///
    /// # Returns
    ///
    /// - `Rect` - The source rectangle.
    pub fn frame_source(&self, index: u32) -> Rect {
        let column: u32 = index % self.get_columns();
        let row: u32 = index / self.get_columns();
        Rect::new(
            column as f64 * self.get_frame_width(),
            row as f64 * self.get_frame_height(),
            self.get_frame_width(),
            self.get_frame_height(),
        )
    }

    /// Creates a frame at the given grid index with a default duration.
    ///
    /// # Arguments
    ///
    /// - `u32` - The frame index.
    ///
    /// # Returns
    ///
    /// - `SpriteFrame` - The new frame.
    pub fn frame(&self, index: u32) -> SpriteFrame {
        SpriteFrame::new(self.frame_source(index), SPRITE_DEFAULT_FRAME_DURATION)
    }

    /// Creates an animation from a range of frame indices.
    ///
    /// # Arguments
    ///
    /// - `&str` - The animation name.
    /// - `u32` - The starting frame index (inclusive).
    /// - `u32` - The ending frame index (exclusive).
    /// - `AnimationMode` - The playback mode.
    ///
    /// # Returns
    ///
    /// - `SpriteAnimation` - The new animation.
    pub fn animation(
        &self,
        name: &str,
        start: u32,
        end: u32,
        mode: AnimationMode,
    ) -> SpriteAnimation {
        let frames: Vec<SpriteFrame> = (start..end).map(|index: u32| self.frame(index)).collect();
        SpriteAnimation::new(name.to_string(), frames, mode)
    }

    /// Draws a static (non-animated) sprite frame onto the canvas.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context.
    /// - `u32` - The frame index to draw.
    /// - `&Transform2D` - The world-space transform to apply.
    pub fn draw_frame(
        &self,
        context: &CanvasRenderingContext2d,
        frame_index: u32,
        transform: &Transform2D,
    ) {
        let source: Rect = self.frame_source(frame_index);
        // Compose the TRS matrix in Rust and apply it with a single `set_transform`
        // instead of save/translate/rotate/scale/restore (5 FFI calls + a canvas
        // state-stack push/pop per sprite). Scale signs handle flipping.
        let rotation: f64 = transform.get_rotation();
        let cos: f64 = rotation.cos();
        let sin: f64 = rotation.sin();
        let scale_x: f64 = transform.get_scale().get_x();
        let scale_y: f64 = transform.get_scale().get_y();
        let _: Result<(), JsValue> = context.set_transform(
            cos * scale_x,
            sin * scale_x,
            -sin * scale_y,
            cos * scale_y,
            transform.get_position().get_x(),
            transform.get_position().get_y(),
        );
        let _: Result<(), JsValue> = context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                self.get_image(),
                source.get_x(),
                source.get_y(),
                source.get_width(),
                source.get_height(),
                -source.get_width() * 0.5,
                -source.get_height() * 0.5,
                source.get_width(),
                source.get_height(),
            );
        let _: Result<(), JsValue> = context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    }
}

/// Implements playback control for `Animator`.
impl Animator {
    /// Creates a new idle animator with no animation.
    ///
    /// # Returns
    ///
    /// - `Animator` - The new animator.
    pub fn create() -> Animator {
        Animator::new(AnimationState::Paused, 1)
    }

    /// Plays the given animation from the beginning.
    ///
    /// # Arguments
    ///
    /// - `SpriteAnimation` - The animation to play.
    pub fn play(&mut self, animation: SpriteAnimation) {
        self.set_current_animation(Some(animation));
        self.set_current_frame_index(0);
        self.set_elapsed_time(0.0);
        self.set_state(AnimationState::Playing);
        self.set_direction(1);
    }

    /// Pauses the current animation.
    pub fn pause(&mut self) {
        if self.get_state() == AnimationState::Playing {
            self.set_state(AnimationState::Paused);
        }
    }

    /// Resumes the current animation from where it was paused.
    pub fn resume(&mut self) {
        if self.get_state() == AnimationState::Paused {
            self.set_state(AnimationState::Playing);
        }
    }

    /// Stops the current animation and resets to the first frame.
    pub fn stop(&mut self) {
        self.set_current_frame_index(0);
        self.set_elapsed_time(0.0);
        self.set_state(AnimationState::Paused);
        self.set_direction(1);
    }

    /// Advances the animation by the given delta time.
    ///
    /// # Arguments
    ///
    /// - `f64` - The time elapsed since the last update, in seconds.
    pub fn update(&mut self, delta_time: f64) {
        if self.get_state() != AnimationState::Playing {
            return;
        }
        let current_frame_index: usize = self.get_current_frame_index();
        let (frame_count, current_duration, mode) = {
            let Some(animation) = self.get_mut_current_animation().as_ref() else {
                return;
            };
            if animation.get_frames().is_empty() {
                return;
            }
            (
                animation.get_frames().len(),
                animation.get_frames()[current_frame_index].get_duration(),
                animation.get_mode(),
            )
        };
        *self.get_mut_elapsed_time() += delta_time;
        if self.get_elapsed_time() < current_duration {
            return;
        }
        self.set_elapsed_time(0.0);
        self.advance_frame(frame_count, mode);
    }

    /// Returns the source rectangle of the current frame, or `None` if no animation is playing.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - The current frame's source rectangle.
    pub fn current_frame_source(&self) -> Option<Rect> {
        let animation: Option<SpriteAnimation> = self.get_current_animation();
        let animation: &SpriteAnimation = animation.as_ref()?;
        let frame: &SpriteFrame = animation.get_frames().get(self.get_current_frame_index())?;
        Some(frame.get_source())
    }

    /// Advances to the next frame according to the animation mode.
    ///
    /// # Arguments
    ///
    /// - `&SpriteAnimation` - The current animation.
    fn advance_frame(&mut self, frame_count: usize, mode: AnimationMode) {
        match mode {
            AnimationMode::Loop => {
                self.set_current_frame_index((self.get_current_frame_index() + 1) % frame_count);
            }
            AnimationMode::Once => {
                if self.get_current_frame_index() + 1 < frame_count {
                    *self.get_mut_current_frame_index() += 1;
                } else {
                    self.set_state(AnimationState::Finished);
                }
            }
            AnimationMode::PingPong => {
                if self.get_direction() > 0 {
                    if self.get_current_frame_index() + 1 < frame_count {
                        *self.get_mut_current_frame_index() += 1;
                    } else {
                        self.set_direction(-1);
                        if self.get_current_frame_index() > 0 {
                            *self.get_mut_current_frame_index() -= 1;
                        }
                    }
                } else if self.get_current_frame_index() > 0 {
                    *self.get_mut_current_frame_index() -= 1;
                } else {
                    self.set_direction(1);
                    if self.get_current_frame_index() + 1 < frame_count {
                        *self.get_mut_current_frame_index() += 1;
                    }
                }
            }
        }
    }
}

/// Forwards `Animator::update` through the [`Updatable`] trait so that
/// collections of heterogeneous updateable objects (entities, animators,
/// physics worlds, scene managers) can be driven by a single scheduler loop.
///
/// The inherent [`Animator::update`] method is the canonical implementation;
/// this impl exists purely for trait dispatch. The inherent call resolves
/// first when both are in scope, so there is no recursion.
impl Updatable for Animator {
    /// Advances the simulation by `delta_time` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - Seconds elapsed since the previous update.
    fn update(&mut self, delta_time: f64) {
        Animator::update(self, delta_time);
    }
}

/// Implements animated sprite rendering for `Animator`.
impl Animator {
    /// Draws the current frame of this animator onto the canvas.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context.
    /// - `&SpriteSheet` - The sprite sheet containing the image.
    /// - `&Transform2D` - The world-space transform to apply.
    pub fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        sheet: &SpriteSheet,
        transform: &Transform2D,
    ) {
        let Some(source) = self.current_frame_source() else {
            return;
        };
        // Compose the TRS matrix in Rust (flip signs folded into scale) and apply
        // it with a single `set_transform` instead of save/translate/rotate/scale/restore.
        let rotation: f64 = transform.get_rotation();
        let cos: f64 = rotation.cos();
        let sin: f64 = rotation.sin();
        let scale_x: f64 = if self.get_flip_x() {
            -transform.get_scale().get_x()
        } else {
            transform.get_scale().get_x()
        };
        let scale_y: f64 = if self.get_flip_y() {
            -transform.get_scale().get_y()
        } else {
            transform.get_scale().get_y()
        };
        let _: Result<(), JsValue> = context.set_transform(
            cos * scale_x,
            sin * scale_x,
            -sin * scale_y,
            cos * scale_y,
            transform.get_position().get_x(),
            transform.get_position().get_y(),
        );
        let _: Result<(), JsValue> = context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                sheet.get_image(),
                source.get_x(),
                source.get_y(),
                source.get_width(),
                source.get_height(),
                -source.get_width() * 0.5,
                -source.get_height() * 0.5,
                source.get_width(),
                source.get_height(),
            );
        let _: Result<(), JsValue> = context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    }
}

/// Implements `Default` for `Animator` as a new idle animator.
impl Default for Animator {
    /// Constructs a default [`Animator`] value.
    ///
    /// # Returns
    ///
    /// - `Animator` - A default-constructed instance with the documented initial state.
    fn default() -> Animator {
        Animator::create()
    }
}

/// Implements named access over the three-by-three nine-slice grid.
///
/// The index constants live in this module's `const.rs`; each accessor
/// maps a grid position to a self-documenting name so call sites and
/// assert messages read as geometry rather than as `(row, column)` pairs.
impl NineSliceRects {
    /// Returns the top-left corner sub-rectangle.
    ///
    /// # Returns
    ///
    /// - `Rect` - The top-left corner patch.
    pub fn get_top_left(&self) -> Rect {
        self.get_grid()[NINE_SLICE_ROW_TOP][NINE_SLICE_COL_LEFT]
    }

    /// Returns the top edge sub-rectangle.
    ///
    /// # Returns
    ///
    /// - `Rect` - The top edge patch.
    pub fn get_top(&self) -> Rect {
        self.get_grid()[NINE_SLICE_ROW_TOP][NINE_SLICE_COL_CENTER]
    }

    /// Returns the top-right corner sub-rectangle.
    ///
    /// # Returns
    ///
    /// - `Rect` - The top-right corner patch.
    pub fn get_top_right(&self) -> Rect {
        self.get_grid()[NINE_SLICE_ROW_TOP][NINE_SLICE_COL_RIGHT]
    }

    /// Returns the left edge sub-rectangle.
    ///
    /// # Returns
    ///
    /// - `Rect` - The left edge patch.
    pub fn get_left(&self) -> Rect {
        self.get_grid()[NINE_SLICE_ROW_MIDDLE][NINE_SLICE_COL_LEFT]
    }

    /// Returns the stretchable center sub-rectangle.
    ///
    /// # Returns
    ///
    /// - `Rect` - The center patch.
    pub fn get_center(&self) -> Rect {
        self.get_grid()[NINE_SLICE_ROW_MIDDLE][NINE_SLICE_COL_CENTER]
    }

    /// Returns the right edge sub-rectangle.
    ///
    /// # Returns
    ///
    /// - `Rect` - The right edge patch.
    pub fn get_right(&self) -> Rect {
        self.get_grid()[NINE_SLICE_ROW_MIDDLE][NINE_SLICE_COL_RIGHT]
    }

    /// Returns the bottom-left corner sub-rectangle.
    ///
    /// # Returns
    ///
    /// - `Rect` - The bottom-left corner patch.
    pub fn get_bottom_left(&self) -> Rect {
        self.get_grid()[NINE_SLICE_ROW_BOTTOM][NINE_SLICE_COL_LEFT]
    }

    /// Returns the bottom edge sub-rectangle.
    ///
    /// # Returns
    ///
    /// - `Rect` - The bottom edge patch.
    pub fn get_bottom(&self) -> Rect {
        self.get_grid()[NINE_SLICE_ROW_BOTTOM][NINE_SLICE_COL_CENTER]
    }

    /// Returns the bottom-right corner sub-rectangle.
    ///
    /// # Returns
    ///
    /// - `Rect` - The bottom-right corner patch.
    pub fn get_bottom_right(&self) -> Rect {
        self.get_grid()[NINE_SLICE_ROW_BOTTOM][NINE_SLICE_COL_RIGHT]
    }

    /// Returns the nine patches as a flat slice in reading order.
    ///
    /// # Returns
    ///
    /// - `Vec<Rect>` - The nine patches, top row first then middle then bottom.
    pub fn to_vec(&self) -> Vec<Rect> {
        let grid: [[Rect; 3]; 3] = self.get_grid();
        vec![
            grid[NINE_SLICE_ROW_TOP][NINE_SLICE_COL_LEFT],
            grid[NINE_SLICE_ROW_TOP][NINE_SLICE_COL_CENTER],
            grid[NINE_SLICE_ROW_TOP][NINE_SLICE_COL_RIGHT],
            grid[NINE_SLICE_ROW_MIDDLE][NINE_SLICE_COL_LEFT],
            grid[NINE_SLICE_ROW_MIDDLE][NINE_SLICE_COL_CENTER],
            grid[NINE_SLICE_ROW_MIDDLE][NINE_SLICE_COL_RIGHT],
            grid[NINE_SLICE_ROW_BOTTOM][NINE_SLICE_COL_LEFT],
            grid[NINE_SLICE_ROW_BOTTOM][NINE_SLICE_COL_CENTER],
            grid[NINE_SLICE_ROW_BOTTOM][NINE_SLICE_COL_RIGHT],
        ]
    }
}

/// Implements the nine-slice geometry math for `NineSliceInsets`.
///
/// All methods here are pure functions of the insets and the rectangle
/// being split: no canvas context, no image handle, no DOM. That is what
/// makes the split verifiable on the host, and it is the same math
/// [`NineSlice::draw_into`] and [`NineSlice::record`] reuse.
impl NineSliceInsets {
    /// Splits a source rectangle into its nine sub-rectangles.
    ///
    /// The insets are clamped against the source size so the nine results
    /// always tile the source exactly: the two horizontal insets share the
    /// available width and the two vertical insets share the available
    /// height, with the center taking whatever remains.
    ///
    /// # Arguments
    ///
    /// - `Rect` - The full source rectangle to split.
    ///
    /// # Returns
    ///
    /// - `NineSliceRects` - The nine source sub-rectangles in reading order.
    pub fn source_rects(&self, source: Rect) -> NineSliceRects {
        let width: f64 = Numeric::clamp(source.get_width(), 0.0, f64::MAX);
        let height: f64 = Numeric::clamp(source.get_height(), 0.0, f64::MAX);
        let left: f64 = Numeric::clamp(self.get_left(), 0.0, width);
        let right: f64 = Numeric::clamp(self.get_right(), 0.0, width - left);
        let top: f64 = Numeric::clamp(self.get_top(), 0.0, height);
        let bottom: f64 = Numeric::clamp(self.get_bottom(), 0.0, height - top);
        let center_x: f64 = source.get_x() + left;
        let center_y: f64 = source.get_y() + top;
        let center_width: f64 = width - left - right;
        let center_height: f64 = height - top - bottom;
        let right_x: f64 = center_x + center_width;
        let bottom_y: f64 = center_y + center_height;
        NineSliceRects::new([
            [
                Rect::new(source.get_x(), source.get_y(), left, top),
                Rect::new(center_x, source.get_y(), center_width, top),
                Rect::new(right_x, source.get_y(), right, top),
            ],
            [
                Rect::new(source.get_x(), center_y, left, center_height),
                Rect::new(center_x, center_y, center_width, center_height),
                Rect::new(right_x, center_y, right, center_height),
            ],
            [
                Rect::new(source.get_x(), bottom_y, left, bottom),
                Rect::new(center_x, bottom_y, center_width, bottom),
                Rect::new(right_x, bottom_y, right, bottom),
            ],
        ])
    }

    /// Splits a destination rectangle into its nine sub-rectangles.
    ///
    /// Corners and edges keep their natural pixel size taken from the
    /// insets, and the center absorbs all remaining destination space, so
    /// the nine results tile the destination exactly. When the
    /// destination is smaller than the combined borders the center
    /// collapses to zero width and/or height instead of going negative.
    ///
    /// # Arguments
    ///
    /// - `Rect` - The full destination rectangle to split.
    ///
    /// # Returns
    ///
    /// - `NineSliceRects` - The nine destination sub-rectangles in reading order.
    pub fn dest_rects(&self, dest: Rect) -> NineSliceRects {
        let left: f64 = Numeric::clamp(self.get_left(), 0.0, f64::MAX);
        let right: f64 = Numeric::clamp(self.get_right(), 0.0, f64::MAX);
        let top: f64 = Numeric::clamp(self.get_top(), 0.0, f64::MAX);
        let bottom: f64 = Numeric::clamp(self.get_bottom(), 0.0, f64::MAX);
        let left_edge: f64 = Numeric::clamp(left, 0.0, dest.get_width());
        let right_edge: f64 = Numeric::clamp(right, 0.0, dest.get_width() - left_edge);
        let top_edge: f64 = Numeric::clamp(top, 0.0, dest.get_height());
        let bottom_edge: f64 = Numeric::clamp(bottom, 0.0, dest.get_height() - top_edge);
        let center_x: f64 = dest.get_x() + left_edge;
        let center_y: f64 = dest.get_y() + top_edge;
        let center_width: f64 = dest.get_width() - left_edge - right_edge;
        let center_height: f64 = dest.get_height() - top_edge - bottom_edge;
        let right_x: f64 = center_x + center_width;
        let bottom_y: f64 = center_y + center_height;
        NineSliceRects::new([
            [
                Rect::new(dest.get_x(), dest.get_y(), left_edge, top_edge),
                Rect::new(center_x, dest.get_y(), center_width, top_edge),
                Rect::new(right_x, dest.get_y(), right_edge, top_edge),
            ],
            [
                Rect::new(dest.get_x(), center_y, left_edge, center_height),
                Rect::new(center_x, center_y, center_width, center_height),
                Rect::new(right_x, center_y, right_edge, center_height),
            ],
            [
                Rect::new(dest.get_x(), bottom_y, left_edge, bottom_edge),
                Rect::new(center_x, bottom_y, center_width, bottom_edge),
                Rect::new(right_x, bottom_y, right_edge, bottom_edge),
            ],
        ])
    }
}

/// Implements image-backed nine-slice drawing and deferred recording.
impl NineSlice {
    /// Creates a nine-slice from an image and uniform border insets.
    ///
    /// # Arguments
    ///
    /// - `HtmlImageElement` - The source image containing the nine patches.
    /// - `f64` - The border inset applied to all four edges, in source pixels.
    ///
    /// # Returns
    ///
    /// - `NineSlice` - The new nine-slice.
    pub fn from_image(image: HtmlImageElement, border: f64) -> NineSlice {
        NineSlice::new(image, NineSliceInsets::new(border, border, border, border))
    }

    /// Returns the nine source sub-rectangles for the whole source image.
    ///
    /// # Returns
    ///
    /// - `NineSliceRects` - The nine source sub-rectangles in reading order.
    pub fn source_rects(&self) -> NineSliceRects {
        let image: &HtmlImageElement = &self.get_image();
        let source: Rect = Rect::new(0.0, 0.0, image.width() as f64, image.height() as f64);
        self.get_insets().source_rects(source)
    }

    /// Returns the nine destination sub-rectangles for a destination rect.
    ///
    /// # Arguments
    ///
    /// - `Rect` - The destination rectangle to split.
    ///
    /// # Returns
    ///
    /// - `NineSliceRects` - The nine destination sub-rectangles in reading order.
    pub fn dest_rects(&self, dest: Rect) -> NineSliceRects {
        self.get_insets().dest_rects(dest)
    }

    /// Draws the nine-slice into a destination rectangle immediately.
    ///
    /// Issues nine `drawImage` calls against the canvas context, pairing
    /// each source patch with its destination sub-rectangle. Uses the
    /// canvas transform already in effect, so the caller controls world
    /// positioning, rotation and scale.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context.
    /// - `Rect` - The destination rectangle in current canvas space.
    pub fn draw_into(&self, context: &CanvasRenderingContext2d, dest: Rect) {
        let sources: Vec<Rect> = self.source_rects().to_vec();
        let dests: Vec<Rect> = self.dest_rects(dest).to_vec();
        let image: &HtmlImageElement = &self.get_image();
        for index in 0..sources.len() {
            let source: Rect = sources[index];
            let target: Rect = dests[index];
            let _: Result<(), JsValue> = context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    image,
                    source.get_x(),
                    source.get_y(),
                    source.get_width(),
                    source.get_height(),
                    target.get_x(),
                    target.get_y(),
                    target.get_width(),
                    target.get_height(),
                );
        }
    }

    /// Records the nine-slice into a deferred draw list.
    ///
    /// The command carries the image plus the insets rather than the nine
    /// resolved sub-rectangles, so the split is recomputed at replay time
    /// against whatever destination size the command was recorded with.
    ///
    /// # Arguments
    ///
    /// - `&mut DrawList` - The draw list to record into.
    /// - `Vector2D` - The destination top-left position in world space.
    /// - `f64` - The destination width in pixels.
    /// - `f64` - The destination height in pixels.
    pub fn record(
        &self,
        list: &mut DrawList,
        dest_position: Vector2D,
        dest_width: f64,
        dest_height: f64,
    ) {
        list.get_mut_commands().push(DrawCommand::DrawNineSlice {
            image: self.get_image(),
            insets: self.get_insets(),
            dest_position,
            dest_width,
            dest_height,
        });
    }
}

/// Implements the pure name-to-rectangle index for `AtlasRegions`.
impl AtlasRegions {
    /// Inserts or replaces the source rectangle stored under a name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The sprite name.
    /// - `Rect` - The source rectangle in atlas pixels.
    pub fn insert(&mut self, name: &str, region: Rect) {
        self.get_mut_regions().insert(name.to_string(), region);
    }

    /// Returns the source rectangle stored under a name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The sprite name.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - The source rectangle, or `None` if the name is unknown.
    pub fn get(&self, name: &str) -> Option<Rect> {
        self.get_regions().get(name).copied()
    }

    /// Returns whether a name is present in the index.
    ///
    /// # Arguments
    ///
    /// - `&str` - The sprite name.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the name has a stored rectangle.
    pub fn contains(&self, name: &str) -> bool {
        self.get_regions().contains_key(name)
    }

    /// Returns the number of named regions in the index.
    ///
    /// # Returns
    ///
    /// - `usize` - The region count.
    pub fn len(&self) -> usize {
        self.get_regions().len()
    }

    /// Returns whether the index holds no regions.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when no region has been inserted.
    pub fn is_empty(&self) -> bool {
        self.get_regions().is_empty()
    }

    /// Returns the stored names in unspecified order.
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - Every stored sprite name.
    pub fn names(&self) -> Vec<String> {
        self.get_regions().keys().cloned().collect()
    }
}

/// Implements image-backed drawing, UV conversion, and recording for `SpriteAtlas`.
impl SpriteAtlas {
    /// Creates an empty atlas backed by the given image.
    ///
    /// # Arguments
    ///
    /// - `HtmlImageElement` - The shared image holding every packed sprite.
    ///
    /// # Returns
    ///
    /// - `SpriteAtlas` - The new empty atlas.
    pub fn create(image: HtmlImageElement) -> SpriteAtlas {
        SpriteAtlas::new(image, AtlasRegions::default())
    }

    /// Inserts or replaces the source rectangle stored under a name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The sprite name.
    /// - `Rect` - The source rectangle in atlas pixels.
    pub fn insert(&mut self, name: &str, region: Rect) {
        self.get_mut_regions().insert(name, region);
    }

    /// Returns the source rectangle stored under a name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The sprite name.
    ///
    /// # Returns
    ///
    /// - `Option<Rect>` - The source rectangle, or `None` if the name is unknown.
    pub fn get(&self, name: &str) -> Option<Rect> {
        self.get_regions().get(name)
    }

    /// Returns whether a name is present in the atlas.
    ///
    /// # Arguments
    ///
    /// - `&str` - The sprite name.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the name has a stored rectangle.
    pub fn contains(&self, name: &str) -> bool {
        self.get_regions().contains(name)
    }

    /// Returns the number of named regions in the atlas.
    ///
    /// # Returns
    ///
    /// - `usize` - The region count.
    pub fn len(&self) -> usize {
        self.get_regions().len()
    }

    /// Returns whether the atlas holds no regions.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when no region has been inserted.
    pub fn is_empty(&self) -> bool {
        self.get_regions().is_empty()
    }

    /// Returns the normalized texture coordinates for one atlas region.
    ///
    /// Normalizes against the image's intrinsic (`naturalWidth` /
    /// `naturalHeight`) size, which is the atlas's true texture size
    /// regardless of any CSS size applied to the element. A zero-sized
    /// atlas — not yet loaded, or decoded with no intrinsic size — yields
    /// an all-zero `UvRect` rather than dividing by zero, so the result is
    /// never `NaN` or infinite.
    ///
    /// # Arguments
    ///
    /// - `Rect` - The source rectangle in atlas pixels.
    ///
    /// # Returns
    ///
    /// - `UvRect` - The normalized `(u0, v0, u1, v1)` coordinates.
    pub fn uv(&self, region: Rect) -> UvRect {
        let image: &HtmlImageElement = &self.get_image();
        let width: f64 = image.natural_width() as f64;
        let height: f64 = image.natural_height() as f64;
        Self::normalize_uv(region, width, height)
    }

    /// Normalizes a pixel rectangle against an explicit atlas size.
    ///
    /// The pure half of [`SpriteAtlas::uv`], separated so the arithmetic
    /// can be exercised without an image element. A non-positive width
    /// or height yields an all-zero `UvRect` instead of dividing by zero.
    ///
    /// # Arguments
    ///
    /// - `Rect` - The source rectangle in atlas pixels.
    /// - `f64` - The atlas width in pixels.
    /// - `f64` - The atlas height in pixels.
    ///
    /// # Returns
    ///
    /// - `UvRect` - The normalized `(u0, v0, u1, v1)` coordinates.
    pub fn normalize_uv(region: Rect, width: f64, height: f64) -> UvRect {
        if width <= 0.0 || height <= 0.0 {
            return UvRect::new(0.0, 0.0, 0.0, 0.0);
        }
        UvRect::new(
            region.get_x() / width,
            region.get_y() / height,
            (region.get_x() + region.get_width()) / width,
            (region.get_y() + region.get_height()) / height,
        )
    }

    /// Blits one named atlas region into a destination rectangle.
    ///
    /// Unknown names are a no-op. Uses the canvas transform already in
    /// effect, so the caller controls world positioning.
    ///
    /// # Arguments
    ///
    /// - `&CanvasRenderingContext2d` - The canvas context.
    /// - `&str` - The sprite name to blit.
    /// - `Rect` - The destination rectangle in current canvas space.
    pub fn draw(&self, context: &CanvasRenderingContext2d, name: &str, dest: Rect) {
        let Some(source) = self.get(name) else {
            return;
        };
        let _: Result<(), JsValue> = context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.get_image(),
                source.get_x(),
                source.get_y(),
                source.get_width(),
                source.get_height(),
                dest.get_x(),
                dest.get_y(),
                dest.get_width(),
                dest.get_height(),
            );
    }

    /// Records one named atlas region into a deferred draw list.
    ///
    /// Unknown names are a no-op, so callers do not need to probe with
    /// [`SpriteAtlas::get`] first.
    ///
    /// # Arguments
    ///
    /// - `&mut DrawList` - The draw list to record into.
    /// - `&str` - The sprite name to blit.
    /// - `Vector2D` - The destination top-left position in world space.
    /// - `f64` - The destination width in pixels.
    /// - `f64` - The destination height in pixels.
    pub fn record(
        &self,
        list: &mut DrawList,
        name: &str,
        dest_position: Vector2D,
        dest_width: f64,
        dest_height: f64,
    ) {
        let Some(source) = self.get(name) else {
            return;
        };
        list.get_mut_commands().push(DrawCommand::DrawAtlasRegion {
            image: self.get_image(),
            source,
            dest_position,
            dest_width,
            dest_height,
        });
    }
}
