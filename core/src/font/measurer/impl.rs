use super::*;

/// Fast internal typography measurement helpers for `DeterministicTextMeasurer`.
impl DeterministicTextMeasurer {
    /// Precomputes invariant typography metrics for a single `measure` or `measure_size` call.
    ///
    /// # Arguments
    ///
    /// - `TextStyle` - Active typography style.
    ///
    /// # Returns
    ///
    /// - `(bool, f32, f32, f32, f32, f32, f32, f32)` - Tuple `(is_mono, font_size, bold_factor, letter_spacing, mono_w, wide_w, non_ascii_w, space_w)`.
    #[inline(always)]
    #[must_use]
    fn style_metrics(style: TextStyle) -> (bool, f32, f32, f32, f32, f32, f32, f32) {
        let is_mono: bool = style.get_family() == FontFamily::Monospace;
        let font_size: f32 = style.get_font_size();
        let bold_factor: f32 = if is_mono {
            1.0
        } else if style.get_weight().is_bold() {
            1.04
        } else if matches!(style.get_weight(), FontWeight::Medium) {
            1.02
        } else {
            1.0
        };
        let letter_spacing: f32 = style.get_letter_spacing();
        let mono_w: f32 = font_size * MONOSPACE_ADVANCE_RATIO * 1.0 + letter_spacing;
        let wide_w: f32 = font_size * 1.04 * bold_factor + letter_spacing;
        let non_ascii_w: f32 = if is_mono {
            mono_w
        } else {
            font_size * 0.76 * bold_factor + letter_spacing
        };
        let space_w: f32 = if is_mono {
            mono_w
        } else {
            font_size * ASCII_ADVANCE_LUT[b' ' as usize] * bold_factor + letter_spacing
        };
        (
            is_mono,
            font_size,
            bold_factor,
            letter_spacing,
            mono_w,
            wide_w,
            non_ascii_w,
            space_w,
        )
    }

    /// Computes the exact advance width of an ASCII byte using hoisted style metrics.
    ///
    /// # Arguments
    ///
    /// - `u8` - ASCII byte (`0..128`).
    /// - `bool` - Whether monospace family is active.
    /// - `f32` - Font size in logical pixels.
    /// - `f32` - Bold width multiplier.
    /// - `f32` - Letter spacing in logical pixels.
    /// - `f32` - Precomputed monospace advance width.
    ///
    /// # Returns
    ///
    /// - `f32` - Exact advance width matching `measure_char_advance`.
    #[inline(always)]
    #[must_use]
    fn ascii_advance(
        byte: u8,
        is_mono: bool,
        font_size: f32,
        bold_factor: f32,
        letter_spacing: f32,
        mono_w: f32,
    ) -> f32 {
        if is_mono {
            mono_w
        } else {
            font_size * ASCII_ADVANCE_LUT[byte as usize] * bold_factor + letter_spacing
        }
    }

    /// Measures the unwrapped total width of one newline-free line using hoisted style metrics.
    ///
    /// # Arguments
    ///
    /// - `&str` - Single line slice without `\n`.
    /// - `(bool, f32, f32, f32, f32, f32, f32, f32)` - Precomputed style metrics tuple.
    ///
    /// # Returns
    ///
    /// - `(f32, usize)` - Total line advance width and character count.
    #[inline]
    #[must_use]
    fn measure_raw_line_width(
        raw_line: &str,
        metrics: (bool, f32, f32, f32, f32, f32, f32, f32),
    ) -> (f32, usize) {
        let (is_mono, font_size, bold_factor, letter_spacing, mono_w, wide_w, non_ascii_w, _): (
            bool,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
        ) = metrics;
        let mut width: f32 = 0.0;
        if raw_line.is_ascii() {
            let bytes: &[u8] = raw_line.as_bytes();
            if is_mono {
                for _ in bytes.iter() {
                    width += mono_w;
                }
            } else {
                for &b in bytes.iter() {
                    width +=
                        font_size * ASCII_ADVANCE_LUT[b as usize] * bold_factor + letter_spacing;
                }
            }
            return (width, bytes.len());
        }
        let mut char_count: usize = 0;
        for ch in raw_line.chars() {
            let code: u32 = ch as u32;
            let ch_w: f32 = if code < 128 {
                Self::ascii_advance(
                    code as u8,
                    is_mono,
                    font_size,
                    bold_factor,
                    letter_spacing,
                    mono_w,
                )
            } else if is_wide_unicode(ch) {
                wide_w
            } else {
                non_ascii_w
            };
            width += ch_w;
            char_count += 1;
        }
        (width, char_count)
    }

    /// Computes the ellipsis-truncated display width of one unwrapped line without allocating.
    ///
    /// # Arguments
    ///
    /// - `&str` - Single unwrapped line slice.
    /// - `f32` - Full unwrapped width of `raw_line`.
    /// - `usize` - Unicode scalar count of `raw_line`.
    /// - `TextStyle` - Active typography style.
    /// - `Option<f32>` - Optional maximum width constraint.
    /// - `(bool, f32, f32, f32, f32, f32, f32, f32)` - Precomputed style metrics tuple.
    ///
    /// # Returns
    ///
    /// - `f32` - Final line width after optional ellipsis truncation.
    #[inline]
    #[must_use]
    fn ellipsis_line_width(
        raw_line: &str,
        width: f32,
        char_count: usize,
        style: TextStyle,
        max_width: Option<f32>,
        metrics: (bool, f32, f32, f32, f32, f32, f32, f32),
    ) -> f32 {
        let (is_mono, font_size, bold_factor, letter_spacing, mono_w, wide_w, non_ascii_w, _): (
            bool,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
        ) = metrics;
        if style.get_overflow() == TextOverflow::Ellipsis
            && let Some(limit) = max_width
            && width > limit + 0.5
            && limit > 0.0
            && char_count > 2
        {
            let dot_w: f32 = Self::ascii_advance(
                b'.',
                is_mono,
                font_size,
                bold_factor,
                letter_spacing,
                mono_w,
            );
            let ellipsis_w: f32 = dot_w * 3.0;
            if width <= ellipsis_w + 1.0 {
                return width;
            }
            let target_w: f32 = (limit - ellipsis_w).max(0.0);
            let mut acc: f32 = 0.0;
            for ch in raw_line.chars() {
                let code: u32 = ch as u32;
                let ch_w: f32 = if code < 128 {
                    Self::ascii_advance(
                        code as u8,
                        is_mono,
                        font_size,
                        bold_factor,
                        letter_spacing,
                        mono_w,
                    )
                } else if is_wide_unicode(ch) {
                    wide_w
                } else {
                    non_ascii_w
                };
                if acc + ch_w > target_w {
                    break;
                }
                acc += ch_w;
            }
            return acc + ellipsis_w;
        }
        width
    }

    /// Flushes the active wrapped slice state into `lines` and resets it for the next line.
    ///
    /// # Arguments
    ///
    /// - `&str` - Source newline-free line slice.
    /// - `f32` - Space character advance width.
    /// - `&mut (bool, usize, usize, Option<String>, f32, usize, bool)` - Active wrapped line state tuple.
    /// - `&mut Vec<TextLine>` - Output wrapped lines buffer.
    /// - `bool` - Whether trailing spaces should be stripped before pushing.
    #[inline(always)]
    fn flush_wrapped_line(
        raw_line: &str,
        space_w: f32,
        state: &mut (bool, usize, usize, Option<String>, f32, usize, bool),
        lines: &mut Vec<TextLine>,
        trim_spaces: bool,
    ) {
        let (cur_has_chars, cur_start, cur_end, cur_overflow, cur_w, trailing_spaces, skipped): (
            &mut bool,
            &mut usize,
            &mut usize,
            &mut Option<String>,
            &mut f32,
            &mut usize,
            &mut bool,
        ) = (
            &mut state.0,
            &mut state.1,
            &mut state.2,
            &mut state.3,
            &mut state.4,
            &mut state.5,
            &mut state.6,
        );
        if trim_spaces {
            let trim_count: usize = *trailing_spaces;
            for _ in 0..trim_count {
                *cur_w = (*cur_w - space_w).max(0.0);
            }
            if let Some(mut buf) = cur_overflow.take() {
                for _ in 0..trim_count {
                    let _: Option<char> = buf.pop();
                }
                lines.push(TextLine::new(buf, *cur_w));
            } else if *cur_has_chars {
                let trimmed_end: usize = *cur_end - trim_count;
                lines.push(TextLine::new(
                    raw_line[*cur_start..trimmed_end].to_string(),
                    *cur_w,
                ));
            } else {
                lines.push(TextLine::new(String::new(), *cur_w));
            }
        } else if let Some(buf) = cur_overflow.take() {
            lines.push(TextLine::new(buf, *cur_w));
        } else if *cur_has_chars {
            lines.push(TextLine::new(
                raw_line[*cur_start..*cur_end].to_string(),
                *cur_w,
            ));
        } else {
            lines.push(TextLine::new(String::new(), *cur_w));
        }
        *cur_has_chars = false;
        *cur_start = 0;
        *cur_end = 0;
        *cur_w = 0.0;
        *trailing_spaces = 0;
        *skipped = false;
    }

    /// Appends a byte segment of `raw_line` onto the active wrapped line state.
    ///
    /// # Arguments
    ///
    /// - `&str` - Source newline-free line slice.
    /// - `&mut (bool, usize, usize, Option<String>, f32, usize, bool)` - Active wrapped line state tuple.
    /// - `(usize, usize, f32)` - Segment byte range `(seg_start, seg_end, seg_w)`.
    /// - `bool` - Whether the appended segment is a single ASCII space.
    #[inline(always)]
    fn append_wrapped_segment(
        raw_line: &str,
        state: &mut (bool, usize, usize, Option<String>, f32, usize, bool),
        segment: (usize, usize, f32),
        is_space: bool,
    ) {
        let (seg_start, seg_end, seg_w): (usize, usize, f32) = segment;
        let (cur_has_chars, cur_start, cur_end, cur_overflow, cur_w, trailing_spaces, skipped): (
            &mut bool,
            &mut usize,
            &mut usize,
            &mut Option<String>,
            &mut f32,
            &mut usize,
            &mut bool,
        ) = (
            &mut state.0,
            &mut state.1,
            &mut state.2,
            &mut state.3,
            &mut state.4,
            &mut state.5,
            &mut state.6,
        );
        if !*cur_has_chars {
            *cur_has_chars = true;
            *cur_start = seg_start;
            *cur_end = seg_end;
            *cur_w = seg_w;
            *trailing_spaces = usize::from(is_space);
            *skipped = false;
        } else if *skipped || cur_overflow.is_some() {
            if cur_overflow.is_none() {
                *cur_overflow = Some(raw_line[*cur_start..*cur_end].to_string());
            }
            if let Some(buf) = cur_overflow.as_mut() {
                buf.push_str(&raw_line[seg_start..seg_end]);
            }
            *cur_w += seg_w;
            *trailing_spaces = if is_space { *trailing_spaces + 1 } else { 0 };
        } else {
            *cur_end = seg_end;
            *cur_w += seg_w;
            *trailing_spaces = if is_space { *trailing_spaces + 1 } else { 0 };
        }
    }

    /// Flushes scalar wrap metrics for one line during zero-allocation `measure_size`.
    ///
    /// # Arguments
    ///
    /// - `f32` - Space character advance width.
    /// - `&mut (bool, f32, usize)` - Active `(cur_has_chars, cur_w, trailing_spaces)` state.
    /// - `&mut usize` - Running wrapped line counter.
    /// - `&mut f32` - Running maximum line width accumulator.
    /// - `bool` - Whether trailing spaces should be stripped from `cur_w`.
    #[inline(always)]
    fn flush_wrapped_size(
        space_w: f32,
        state: &mut (bool, f32, usize),
        line_count: &mut usize,
        max_line_w: &mut f32,
        trim_spaces: bool,
    ) {
        if trim_spaces {
            for _ in 0..state.2 {
                state.1 = (state.1 - space_w).max(0.0);
            }
        }
        if state.1 > *max_line_w {
            *max_line_w = state.1;
        }
        *line_count += 1;
        state.0 = false;
        state.1 = 0.0;
        state.2 = 0;
    }

    /// Wraps one newline-free line into `lines` using zero-copy byte-slice tracking.
    ///
    /// # Arguments
    ///
    /// - `&str` - Single newline-free line slice.
    /// - `f32` - Effective wrap width limit including `WRAP_WIDTH_EPSILON`.
    /// - `(bool, f32, f32, f32, f32, f32, f32, f32)` - Precomputed style metrics tuple.
    /// - `&mut Vec<TextLine>` - Output wrapped lines buffer.
    fn wrap_raw_line_into(
        raw_line: &str,
        limit: f32,
        metrics: (bool, f32, f32, f32, f32, f32, f32, f32),
        lines: &mut Vec<TextLine>,
    ) {
        let (
            is_mono,
            font_size,
            bold_factor,
            letter_spacing,
            mono_w,
            wide_w,
            non_ascii_w,
            space_w,
        ): (bool, f32, f32, f32, f32, f32, f32, f32) = metrics;
        let mut cur_state: (bool, usize, usize, Option<String>, f32, usize, bool) =
            (false, 0, 0, None, 0.0, 0, false);
        let mut word_has_chars: bool = false;
        let mut word_start: usize = 0;
        let mut word_end: usize = 0;
        let mut word_w: f32 = 0.0;
        if raw_line.is_ascii() {
            for (byte_idx, &b) in raw_line.as_bytes().iter().enumerate() {
                let ch_w: f32 =
                    Self::ascii_advance(b, is_mono, font_size, bold_factor, letter_spacing, mono_w);
                let ch_end: usize = byte_idx + 1;
                if b == b' ' {
                    if cur_state.0 && cur_state.4 + word_w > limit {
                        Self::flush_wrapped_line(raw_line, space_w, &mut cur_state, lines, true);
                    }
                    if word_has_chars {
                        Self::append_wrapped_segment(
                            raw_line,
                            &mut cur_state,
                            (word_start, word_end, word_w),
                            false,
                        );
                        word_has_chars = false;
                        word_w = 0.0;
                    }
                    if cur_state.0 {
                        if cur_state.4 + ch_w <= limit {
                            Self::append_wrapped_segment(
                                raw_line,
                                &mut cur_state,
                                (byte_idx, ch_end, ch_w),
                                true,
                            );
                        } else {
                            cur_state.6 = true;
                        }
                    }
                } else {
                    if cur_state.0 && cur_state.4 + word_w + ch_w > limit && word_w + ch_w > limit {
                        Self::flush_wrapped_line(raw_line, space_w, &mut cur_state, lines, true);
                    }
                    if word_has_chars && !cur_state.0 && word_w + ch_w > limit {
                        lines.push(TextLine::new(
                            raw_line[word_start..word_end].to_string(),
                            word_w,
                        ));
                        word_has_chars = false;
                        word_w = 0.0;
                    }
                    if !word_has_chars {
                        word_start = byte_idx;
                        word_has_chars = true;
                    }
                    word_end = ch_end;
                    word_w += ch_w;
                }
            }
        } else {
            for (byte_idx, ch) in raw_line.char_indices() {
                let code: u32 = ch as u32;
                let wide: bool = code >= CJK_PUNCT_START && is_wide_unicode(ch);
                let ch_w: f32 = if code < 128 {
                    Self::ascii_advance(
                        code as u8,
                        is_mono,
                        font_size,
                        bold_factor,
                        letter_spacing,
                        mono_w,
                    )
                } else if wide {
                    wide_w
                } else {
                    non_ascii_w
                };
                let ch_end: usize = byte_idx + ch.len_utf8();
                if ch == ' ' {
                    if cur_state.0 && cur_state.4 + word_w > limit {
                        Self::flush_wrapped_line(raw_line, space_w, &mut cur_state, lines, true);
                    }
                    if word_has_chars {
                        Self::append_wrapped_segment(
                            raw_line,
                            &mut cur_state,
                            (word_start, word_end, word_w),
                            false,
                        );
                        word_has_chars = false;
                        word_w = 0.0;
                    }
                    if cur_state.0 {
                        if cur_state.4 + ch_w <= limit {
                            Self::append_wrapped_segment(
                                raw_line,
                                &mut cur_state,
                                (byte_idx, ch_end, ch_w),
                                true,
                            );
                        } else {
                            cur_state.6 = true;
                        }
                    }
                } else if wide {
                    if word_has_chars {
                        if cur_state.0 && cur_state.4 + word_w > limit {
                            Self::flush_wrapped_line(
                                raw_line,
                                space_w,
                                &mut cur_state,
                                lines,
                                true,
                            );
                        }
                        Self::append_wrapped_segment(
                            raw_line,
                            &mut cur_state,
                            (word_start, word_end, word_w),
                            false,
                        );
                        word_has_chars = false;
                        word_w = 0.0;
                    }
                    if cur_state.0 && cur_state.4 + ch_w > limit {
                        Self::flush_wrapped_line(raw_line, space_w, &mut cur_state, lines, false);
                    }
                    Self::append_wrapped_segment(
                        raw_line,
                        &mut cur_state,
                        (byte_idx, ch_end, ch_w),
                        false,
                    );
                } else {
                    if cur_state.0 && cur_state.4 + word_w + ch_w > limit && word_w + ch_w > limit {
                        Self::flush_wrapped_line(raw_line, space_w, &mut cur_state, lines, true);
                    }
                    if word_has_chars && !cur_state.0 && word_w + ch_w > limit {
                        lines.push(TextLine::new(
                            raw_line[word_start..word_end].to_string(),
                            word_w,
                        ));
                        word_has_chars = false;
                        word_w = 0.0;
                    }
                    if !word_has_chars {
                        word_start = byte_idx;
                        word_has_chars = true;
                    }
                    word_end = ch_end;
                    word_w += ch_w;
                }
            }
        }
        if word_has_chars {
            if cur_state.0 && cur_state.4 + word_w > limit {
                Self::flush_wrapped_line(raw_line, space_w, &mut cur_state, lines, true);
            }
            Self::append_wrapped_segment(
                raw_line,
                &mut cur_state,
                (word_start, word_end, word_w),
                false,
            );
        }
        Self::flush_wrapped_line(raw_line, space_w, &mut cur_state, lines, true);
    }

    /// Accumulates wrapped line count and maximum line width for one newline-free line with zero heap allocations.
    ///
    /// # Arguments
    ///
    /// - `&str` - Single newline-free line slice.
    /// - `f32` - Effective wrap width limit including `WRAP_WIDTH_EPSILON`.
    /// - `(bool, f32, f32, f32, f32, f32, f32, f32)` - Precomputed style metrics tuple.
    /// - `&mut usize` - Running wrapped line counter.
    /// - `&mut f32` - Running maximum line width accumulator.
    fn measure_wrapped_line_size(
        raw_line: &str,
        limit: f32,
        metrics: (bool, f32, f32, f32, f32, f32, f32, f32),
        line_count: &mut usize,
        max_line_w: &mut f32,
    ) {
        let (
            is_mono,
            font_size,
            bold_factor,
            letter_spacing,
            mono_w,
            wide_w,
            non_ascii_w,
            space_w,
        ): (bool, f32, f32, f32, f32, f32, f32, f32) = metrics;
        let mut cur_state: (bool, f32, usize) = (false, 0.0, 0);
        let mut word_has_chars: bool = false;
        let mut word_w: f32 = 0.0;
        if raw_line.is_ascii() {
            for &b in raw_line.as_bytes().iter() {
                let ch_w: f32 =
                    Self::ascii_advance(b, is_mono, font_size, bold_factor, letter_spacing, mono_w);
                if b == b' ' {
                    if cur_state.0 && cur_state.1 + word_w > limit {
                        Self::flush_wrapped_size(
                            space_w,
                            &mut cur_state,
                            line_count,
                            max_line_w,
                            true,
                        );
                    }
                    if word_has_chars {
                        cur_state.0 = true;
                        cur_state.1 += word_w;
                        cur_state.2 = 0;
                        word_has_chars = false;
                        word_w = 0.0;
                    }
                    if cur_state.0 && cur_state.1 + ch_w <= limit {
                        cur_state.1 += ch_w;
                        cur_state.2 += 1;
                    }
                } else {
                    if cur_state.0 && cur_state.1 + word_w + ch_w > limit && word_w + ch_w > limit {
                        Self::flush_wrapped_size(
                            space_w,
                            &mut cur_state,
                            line_count,
                            max_line_w,
                            true,
                        );
                    }
                    if word_has_chars && !cur_state.0 && word_w + ch_w > limit {
                        if word_w > *max_line_w {
                            *max_line_w = word_w;
                        }
                        *line_count += 1;
                        word_w = 0.0;
                    }
                    word_has_chars = true;
                    word_w += ch_w;
                }
            }
        } else {
            for ch in raw_line.chars() {
                let code: u32 = ch as u32;
                let wide: bool = code >= CJK_PUNCT_START && is_wide_unicode(ch);
                let ch_w: f32 = if code < 128 {
                    Self::ascii_advance(
                        code as u8,
                        is_mono,
                        font_size,
                        bold_factor,
                        letter_spacing,
                        mono_w,
                    )
                } else if wide {
                    wide_w
                } else {
                    non_ascii_w
                };
                if ch == ' ' {
                    if cur_state.0 && cur_state.1 + word_w > limit {
                        Self::flush_wrapped_size(
                            space_w,
                            &mut cur_state,
                            line_count,
                            max_line_w,
                            true,
                        );
                    }
                    if word_has_chars {
                        cur_state.0 = true;
                        cur_state.1 += word_w;
                        cur_state.2 = 0;
                        word_has_chars = false;
                        word_w = 0.0;
                    }
                    if cur_state.0 && cur_state.1 + ch_w <= limit {
                        cur_state.1 += ch_w;
                        cur_state.2 += 1;
                    }
                } else if wide {
                    if word_has_chars {
                        if cur_state.0 && cur_state.1 + word_w > limit {
                            Self::flush_wrapped_size(
                                space_w,
                                &mut cur_state,
                                line_count,
                                max_line_w,
                                true,
                            );
                        }
                        cur_state.0 = true;
                        cur_state.1 += word_w;
                        cur_state.2 = 0;
                        word_has_chars = false;
                        word_w = 0.0;
                    }
                    if cur_state.0 && cur_state.1 + ch_w > limit {
                        Self::flush_wrapped_size(
                            space_w,
                            &mut cur_state,
                            line_count,
                            max_line_w,
                            false,
                        );
                    }
                    cur_state.0 = true;
                    cur_state.1 += ch_w;
                    cur_state.2 = 0;
                } else {
                    if cur_state.0 && cur_state.1 + word_w + ch_w > limit && word_w + ch_w > limit {
                        Self::flush_wrapped_size(
                            space_w,
                            &mut cur_state,
                            line_count,
                            max_line_w,
                            true,
                        );
                    }
                    if word_has_chars && !cur_state.0 && word_w + ch_w > limit {
                        if word_w > *max_line_w {
                            *max_line_w = word_w;
                        }
                        *line_count += 1;
                        word_w = 0.0;
                    }
                    word_has_chars = true;
                    word_w += ch_w;
                }
            }
        }
        if word_has_chars {
            if cur_state.0 && cur_state.1 + word_w > limit {
                Self::flush_wrapped_size(space_w, &mut cur_state, line_count, max_line_w, true);
            }
            cur_state.0 = true;
            cur_state.1 += word_w;
            cur_state.2 = 0;
        }
        Self::flush_wrapped_size(space_w, &mut cur_state, line_count, max_line_w, true);
    }

    /// Formats one unwrapped line with optional ellipsis truncation and appends it to `lines`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Single newline-free line slice.
    /// - `TextStyle` - Active typography style.
    /// - `Option<f32>` - Optional maximum width constraint.
    /// - `(bool, f32, f32, f32, f32, f32, f32, f32)` - Precomputed style metrics tuple.
    /// - `&mut Vec<TextLine>` - Output lines buffer.
    fn push_unwrapped_line(
        raw_line: &str,
        style: TextStyle,
        max_width: Option<f32>,
        metrics: (bool, f32, f32, f32, f32, f32, f32, f32),
        lines: &mut Vec<TextLine>,
    ) {
        let (width, char_count): (f32, usize) = Self::measure_raw_line_width(raw_line, metrics);
        if style.get_overflow() == TextOverflow::Ellipsis
            && let Some(limit) = max_width
            && width > limit + 0.5
            && limit > 0.0
            && char_count > 2
        {
            let (is_mono, font_size, bold_factor, letter_spacing, mono_w, wide_w, non_ascii_w, _): (
                bool,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
            ) = metrics;
            let dot_w: f32 = Self::ascii_advance(
                b'.',
                is_mono,
                font_size,
                bold_factor,
                letter_spacing,
                mono_w,
            );
            let ellipsis_w: f32 = dot_w * 3.0;
            if width <= ellipsis_w + 1.0 {
                lines.push(TextLine::new(raw_line.to_string(), width));
            } else {
                let target_w: f32 = (limit - ellipsis_w).max(0.0);
                let mut acc: f32 = 0.0;
                let mut cut_bytes: usize = 0;
                for (byte_idx, ch) in raw_line.char_indices() {
                    let code: u32 = ch as u32;
                    let ch_w: f32 = if code < 128 {
                        Self::ascii_advance(
                            code as u8,
                            is_mono,
                            font_size,
                            bold_factor,
                            letter_spacing,
                            mono_w,
                        )
                    } else if is_wide_unicode(ch) {
                        wide_w
                    } else {
                        non_ascii_w
                    };
                    if acc + ch_w > target_w {
                        break;
                    }
                    acc += ch_w;
                    cut_bytes = byte_idx + ch.len_utf8();
                }
                let mut truncated: String = String::with_capacity(cut_bytes + 3);
                truncated.push_str(&raw_line[..cut_bytes]);
                truncated.push_str("...");
                lines.push(TextLine::new(truncated, acc + ellipsis_w));
            }
        } else {
            lines.push(TextLine::new(raw_line.to_string(), width));
        }
    }
}

/// Deterministic cross-platform `TextMeasurer` implementation.
impl TextMeasurer for DeterministicTextMeasurer {
    /// Measures and lays out `text` identically on every operating system and rendering backend.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Measurer instance.
    /// - `&str` - Input UTF-8 string.
    /// - `TextStyle` - Active typography style.
    /// - `Option<f32>` - Optional maximum available width.
    ///
    /// # Returns
    ///
    /// - `TextLayoutResult` - Measured lines and bounding box.
    fn measure(&self, text: &str, style: TextStyle, max_width: Option<f32>) -> TextLayoutResult {
        let line_height_px: f32 = (style.get_font_size() * style.get_line_height()).max(1.0);
        if text.is_empty() {
            return TextLayoutResult::new(
                Vec::new(),
                Size::new(0.0, line_height_px),
                line_height_px,
            );
        }
        let metrics: (bool, f32, f32, f32, f32, f32, f32, f32) = Self::style_metrics(style);
        let wrap_limit: Option<f32> = match style.get_overflow() {
            TextOverflow::Wrap => max_width.filter(|w: &f32| *w > 0.0),
            TextOverflow::Clip | TextOverflow::Ellipsis => None,
        };
        let single_raw_line: bool = !text.as_bytes().contains(&b'\n');
        if single_raw_line && wrap_limit.is_none() {
            let mut lines: Vec<TextLine> = Vec::with_capacity(1);
            Self::push_unwrapped_line(text, style, max_width, metrics, &mut lines);
            let line_w: f32 = lines[0].get_width();
            return TextLayoutResult::new(lines, Size::new(line_w, line_height_px), line_height_px);
        }
        let mut lines: Vec<TextLine> = Vec::with_capacity(if single_raw_line { 1 } else { 4 });
        if let Some(raw_limit) = wrap_limit {
            let limit: f32 = raw_limit.max(1.0) + WRAP_WIDTH_EPSILON;
            if single_raw_line {
                Self::wrap_raw_line_into(text, limit, metrics, &mut lines);
            } else {
                for raw_line in text.split('\n') {
                    Self::wrap_raw_line_into(raw_line, limit, metrics, &mut lines);
                }
            }
        } else {
            for raw_line in text.split('\n') {
                Self::push_unwrapped_line(raw_line, style, max_width, metrics, &mut lines);
            }
        }
        let max_line_w: f32 = lines.iter().fold(0.0_f32, |acc: f32, line: &TextLine| {
            acc.max(line.get_width())
        });
        let total_h: f32 = (lines.len().max(1) as f32) * line_height_px;
        TextLayoutResult::new(lines, Size::new(max_line_w, total_h), line_height_px)
    }

    /// Measures only the aggregate bounding box of `text` under `style` with zero heap allocations.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Measurer instance.
    /// - `&str` - Input UTF-8 string.
    /// - `TextStyle` - Active typography style.
    /// - `Option<f32>` - Optional maximum available width.
    ///
    /// # Returns
    ///
    /// - `Size` - Exact aggregate bounding box matching `measure(text, style, max_width).get_size()`.
    fn measure_size(&self, text: &str, style: TextStyle, max_width: Option<f32>) -> Size {
        let line_height_px: f32 = (style.get_font_size() * style.get_line_height()).max(1.0);
        if text.is_empty() {
            return Size::new(0.0, line_height_px);
        }
        let metrics: (bool, f32, f32, f32, f32, f32, f32, f32) = Self::style_metrics(style);
        let wrap_limit: Option<f32> = match style.get_overflow() {
            TextOverflow::Wrap => max_width.filter(|w: &f32| *w > 0.0),
            TextOverflow::Clip | TextOverflow::Ellipsis => None,
        };
        let single_raw_line: bool = !text.as_bytes().contains(&b'\n');
        if single_raw_line && wrap_limit.is_none() {
            let (width, char_count): (f32, usize) = Self::measure_raw_line_width(text, metrics);
            let final_w: f32 =
                Self::ellipsis_line_width(text, width, char_count, style, max_width, metrics);
            return Size::new(final_w, line_height_px);
        }
        let mut line_count: usize = 0;
        let mut max_line_w: f32 = 0.0;
        if let Some(raw_limit) = wrap_limit {
            let limit: f32 = raw_limit.max(1.0) + WRAP_WIDTH_EPSILON;
            if single_raw_line {
                Self::measure_wrapped_line_size(
                    text,
                    limit,
                    metrics,
                    &mut line_count,
                    &mut max_line_w,
                );
            } else {
                for raw_line in text.split('\n') {
                    Self::measure_wrapped_line_size(
                        raw_line,
                        limit,
                        metrics,
                        &mut line_count,
                        &mut max_line_w,
                    );
                }
            }
        } else {
            for raw_line in text.split('\n') {
                let (width, char_count): (f32, usize) =
                    Self::measure_raw_line_width(raw_line, metrics);
                let final_w: f32 = Self::ellipsis_line_width(
                    raw_line, width, char_count, style, max_width, metrics,
                );
                if final_w > max_line_w {
                    max_line_w = final_w;
                }
                line_count += 1;
            }
        }
        let total_h: f32 = (line_count.max(1) as f32) * line_height_px;
        Size::new(max_line_w, total_h)
    }
}
