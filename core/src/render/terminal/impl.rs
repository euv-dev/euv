use super::*;

/// Blank default cell (`' '`, width `1`, default terminal attributes).
impl Default for TerminalCell {
    /// Creates a blank space terminal cell.
    ///
    /// # Returns
    ///
    /// - `Self` - Blank cell.
    fn default() -> Self {
        Self::blank()
    }
}

/// Construction and style inspection helpers for `TerminalCell`.
impl TerminalCell {
    /// Creates an empty single-column space cell with default attributes.
    ///
    /// # Returns
    ///
    /// - `Self` - Blank cell.
    #[must_use]
    pub const fn blank() -> Self {
        Self {
            ch: ' ',
            width: 1,
            fg: None,
            bg: None,
            bold: false,
            reverse: false,
            underline: false,
        }
    }

    /// Creates a trailing continuation cell (`width == 0`) paired with a wide character lead cell.
    ///
    /// # Arguments
    ///
    /// - `Option<Color>` - Inherited foreground color.
    /// - `Option<Color>` - Inherited background color.
    /// - `bool` - Inherited bold flag.
    /// - `bool` - Inherited reverse-video flag.
    /// - `bool` - Inherited underline flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Zero-width continuation cell.
    #[must_use]
    pub const fn continuation(
        fg: Option<Color>,
        bg: Option<Color>,
        bold: bool,
        reverse: bool,
        underline: bool,
    ) -> Self {
        Self {
            ch: '\0',
            width: 0,
            fg,
            bg,
            bold,
            reverse,
            underline,
        }
    }

    /// Creates a styled character cell with explicit display width and ANSI flags.
    ///
    /// # Arguments
    ///
    /// - `char` - Character scalar value.
    /// - `u8` - Display width (`1` or `2`).
    /// - `Option<Color>` - Optional foreground color.
    /// - `Option<Color>` - Optional background color.
    /// - `bool` - Bold flag.
    /// - `bool` - Reverse-video flag.
    /// - `bool` - Underline flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed styled cell.
    #[must_use]
    pub const fn styled(
        ch: char,
        width: u8,
        fg: Option<Color>,
        bg: Option<Color>,
        bold: bool,
        reverse: bool,
        underline: bool,
    ) -> Self {
        Self {
            ch,
            width,
            fg,
            bg,
            bold,
            reverse,
            underline,
        }
    }

    /// Returns the optional 24-bit foreground color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Cell reference.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Foreground color when specified.
    #[must_use]
    pub const fn fg_color(&self) -> Option<Color> {
        let Self { fg, .. } = *self;
        fg
    }

    /// Returns the optional 24-bit background color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Cell reference.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Background color when specified.
    #[must_use]
    pub const fn bg_color(&self) -> Option<Color> {
        let Self { bg, .. } = *self;
        bg
    }

    /// Returns `true` when this cell is the trailing half of a wide character.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Cell reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` iff `width == 0`.
    #[must_use]
    pub const fn is_continuation(&self) -> bool {
        let Self { width, .. } = *self;
        width == 0
    }

    /// Returns `true` when this cell is the leading cell of a double-width character.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Cell reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` iff `width == 2`.
    #[must_use]
    pub const fn is_wide_lead(&self) -> bool {
        let Self { width, .. } = *self;
        width == 2
    }

    /// Returns `true` when two cells share identical SGR styling attributes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - First cell reference.
    /// - `&Self` - Second cell reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when foreground, background, bold, reverse, and underline match.
    #[must_use]
    pub fn same_style(&self, other: &Self) -> bool {
        self.fg_color() == other.fg_color()
            && self.bg_color() == other.bg_color()
            && self.get_bold() == other.get_bold()
            && self.get_reverse() == other.get_reverse()
            && self.get_underline() == other.get_underline()
    }
}

/// Grid allocation, wide-char-safe writing, box drawing, and plain-text serialization for `TerminalCellGrid`.
impl TerminalCellGrid {
    /// Allocates a blank `cols x rows` terminal cell grid (clamped to at least `1 x 1`).
    ///
    /// # Arguments
    ///
    /// - `u16` - Column count.
    /// - `u16` - Row count.
    ///
    /// # Returns
    ///
    /// - `Self` - Blank grid.
    #[must_use]
    pub fn allocate(cols: u16, rows: u16) -> Self {
        let clamped_cols: u16 = cols.max(TERMINAL_MIN_COLS);
        let clamped_rows: u16 = rows.max(TERMINAL_MIN_ROWS);
        let total: usize = (clamped_cols as usize).saturating_mul(clamped_rows as usize);
        Self {
            cols: clamped_cols,
            rows: clamped_rows,
            cells: vec![TerminalCell::blank(); total],
        }
    }

    /// Resizes and clears the grid to `cols x rows`, reusing existing capacity when unchanged.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable grid.
    /// - `u16` - Target column count.
    /// - `u16` - Target row count.
    pub fn reset(&mut self, cols: u16, rows: u16) {
        let clamped_cols: u16 = cols.max(TERMINAL_MIN_COLS);
        let clamped_rows: u16 = rows.max(TERMINAL_MIN_ROWS);
        let total: usize = (clamped_cols as usize).saturating_mul(clamped_rows as usize);
        self.set_cols(clamped_cols);
        self.set_rows(clamped_rows);
        let cells: &mut Vec<TerminalCell> = self.get_mut_cells();
        if cells.len() == total {
            cells.fill(TerminalCell::blank());
        } else {
            cells.clear();
            cells.resize(total, TerminalCell::blank());
        }
    }

    /// Returns total number of character cells in the grid.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Grid reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Total cell count (`cols * rows`).
    #[must_use]
    pub fn total_cells(&self) -> usize {
        self.get_cells().len()
    }

    /// Returns the flat vector index for `(col, row)` if within grid bounds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Grid reference.
    /// - `u16` - Zero-based column coordinate.
    /// - `u16` - Zero-based row coordinate.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - Linear cell index if in bounds.
    #[must_use]
    fn cell_index(&self, col: u16, row: u16) -> Option<usize> {
        let cols: u16 = self.get_cols();
        let rows: u16 = self.get_rows();
        if col >= cols || row >= rows {
            return None;
        }
        Some((row as usize) * (cols as usize) + (col as usize))
    }

    /// Reads the `TerminalCell` stored at `(col, row)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Grid reference.
    /// - `u16` - Zero-based column coordinate.
    /// - `u16` - Zero-based row coordinate.
    ///
    /// # Returns
    ///
    /// - `Option<TerminalCell>` - Cell copy if within bounds.
    #[must_use]
    pub fn cell(&self, col: u16, row: u16) -> Option<TerminalCell> {
        let idx: usize = self.cell_index(col, row)?;
        self.get_cells().get(idx).copied()
    }

    /// Clears any wide-character partner occupying `(col, row)` before overwriting that coordinate.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable grid.
    /// - `u16` - Zero-based column coordinate.
    /// - `u16` - Zero-based row coordinate.
    fn clear_wide_conflict(&mut self, col: u16, row: u16) {
        let Some(existing) = self.cell(col, row) else {
            return;
        };
        if existing.is_continuation() && col > 0 {
            if let Some(lead_idx) = self.cell_index(col - 1, row)
                && let Some(slot) = self.get_mut_cells().get_mut(lead_idx)
            {
                *slot = TerminalCell::blank();
            }
            if let Some(cur_idx) = self.cell_index(col, row)
                && let Some(slot) = self.get_mut_cells().get_mut(cur_idx)
            {
                *slot = TerminalCell::blank();
            }
        } else if existing.is_wide_lead() {
            if let Some(trail_idx) = self.cell_index(col.saturating_add(1), row)
                && let Some(slot) = self.get_mut_cells().get_mut(trail_idx)
            {
                *slot = TerminalCell::blank();
            }
            if let Some(cur_idx) = self.cell_index(col, row)
                && let Some(slot) = self.get_mut_cells().get_mut(cur_idx)
            {
                *slot = TerminalCell::blank();
            }
        }
    }

    /// Writes a single narrow or wide `TerminalCell` at `(col, row)`, cleanly clearing overwritten wide neighbors and clipping cleanly at the right edge.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable grid.
    /// - `u16` - Zero-based column coordinate.
    /// - `u16` - Zero-based row coordinate.
    /// - `TerminalCell` - Cell value to write.
    ///
    /// # Returns
    ///
    /// - `u16` - Number of columns consumed (`0`, `1`, or `2`).
    pub fn put_cell(&mut self, col: u16, row: u16, cell: TerminalCell) -> u16 {
        let cols: u16 = self.get_cols();
        let rows: u16 = self.get_rows();
        if col >= cols || row >= rows || cell.is_continuation() {
            return 0;
        }
        if cell.is_wide_lead() {
            if col.saturating_add(1) >= cols {
                self.clear_wide_conflict(col, row);
                if let Some(idx) = self.cell_index(col, row)
                    && let Some(slot) = self.get_mut_cells().get_mut(idx)
                {
                    *slot = TerminalCell::blank();
                }
                return 0;
            }
            self.clear_wide_conflict(col, row);
            self.clear_wide_conflict(col + 1, row);
            if let Some(lead_idx) = self.cell_index(col, row)
                && let Some(slot) = self.get_mut_cells().get_mut(lead_idx)
            {
                *slot = cell;
            }
            if let Some(trail_idx) = self.cell_index(col + 1, row)
                && let Some(slot) = self.get_mut_cells().get_mut(trail_idx)
            {
                *slot = TerminalCell::continuation(
                    cell.fg_color(),
                    cell.bg_color(),
                    cell.get_bold(),
                    cell.get_reverse(),
                    cell.get_underline(),
                );
            }
            return 2;
        }
        self.clear_wide_conflict(col, row);
        if let Some(idx) = self.cell_index(col, row)
            && let Some(slot) = self.get_mut_cells().get_mut(idx)
        {
            *slot = cell;
        }
        1
    }

    /// Writes UTF-8 `text` onto `row` starting at `col` within `max_cols`, honoring CJK double-width glyphs, zero-width combining marks, and clean edge truncation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable grid.
    /// - `u16` - Starting zero-based column coordinate.
    /// - `u16` - Target zero-based row coordinate.
    /// - `u16` - Maximum display columns allowed for this write.
    /// - `&str` - UTF-8 text slice to write.
    /// - `TerminalCell` - Style template supplying foreground, background, bold, reverse, and underline flags.
    ///
    /// # Returns
    ///
    /// - `u16` - Total display columns written.
    pub fn write_styled_text(
        &mut self,
        col: u16,
        row: u16,
        max_cols: u16,
        text: &str,
        style_cell: TerminalCell,
    ) -> u16 {
        let cols: u16 = self.get_cols();
        let rows: u16 = self.get_rows();
        if col >= cols || row >= rows || max_cols == 0 {
            return 0;
        }
        let limit: u16 = max_cols.min(cols.saturating_sub(col));
        let mut used: u16 = 0;
        for ch in text.chars() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            let width: u8 = terminal_char_width(ch);
            if width == 0 {
                continue;
            }
            let w_u16: u16 = width as u16;
            if used.saturating_add(w_u16) > limit {
                break;
            }
            let cell: TerminalCell = TerminalCell::styled(
                ch,
                width,
                style_cell.fg_color(),
                style_cell.bg_color(),
                style_cell.get_bold(),
                style_cell.get_reverse(),
                style_cell.get_underline(),
            );
            let consumed: u16 = self.put_cell(col + used, row, cell);
            if consumed == 0 {
                break;
            }
            used = used.saturating_add(consumed);
        }
        used
    }

    /// Writes UTF-8 `text` onto `row` starting at `col` using default terminal style within `max_cols`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable grid.
    /// - `u16` - Starting zero-based column coordinate.
    /// - `u16` - Target zero-based row coordinate.
    /// - `u16` - Maximum display columns allowed.
    /// - `&str` - UTF-8 text slice to write.
    ///
    /// # Returns
    ///
    /// - `u16` - Total display columns written.
    pub fn write_text(&mut self, col: u16, row: u16, max_cols: u16, text: &str) -> u16 {
        self.write_styled_text(col, row, max_cols, text, TerminalCell::blank())
    }

    /// Draws a Unicode box-drawing border frame covering `[x, x + w) x [y, y + h)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable grid.
    /// - `u16` - Left column coordinate.
    /// - `u16` - Top row coordinate.
    /// - `u16` - Box width in columns.
    /// - `u16` - Box height in rows.
    /// - `TerminalCell` - Style template supplying foreground, background, bold, and reverse flags.
    pub fn draw_box(&mut self, x: u16, y: u16, w: u16, h: u16, style_cell: TerminalCell) {
        if w < 2 || h < 2 {
            return;
        }
        let right: u16 = x.saturating_add(w - 1);
        let bottom: u16 = y.saturating_add(h - 1);
        let fg: Option<Color> = style_cell.fg_color();
        let bg: Option<Color> = style_cell.bg_color();
        let bold: bool = style_cell.get_bold();
        let rev: bool = style_cell.get_reverse();
        let ul: bool = style_cell.get_underline();
        let _: u16 = self.put_cell(
            x,
            y,
            TerminalCell::styled(TERMINAL_BOX_TOP_LEFT, 1, fg, bg, bold, rev, ul),
        );
        let _: u16 = self.put_cell(
            right,
            y,
            TerminalCell::styled(TERMINAL_BOX_TOP_RIGHT, 1, fg, bg, bold, rev, ul),
        );
        let _: u16 = self.put_cell(
            x,
            bottom,
            TerminalCell::styled(TERMINAL_BOX_BOTTOM_LEFT, 1, fg, bg, bold, rev, ul),
        );
        let _: u16 = self.put_cell(
            right,
            bottom,
            TerminalCell::styled(TERMINAL_BOX_BOTTOM_RIGHT, 1, fg, bg, bold, rev, ul),
        );
        let horiz: TerminalCell =
            TerminalCell::styled(TERMINAL_BOX_HORIZONTAL, 1, fg, bg, bold, rev, ul);
        let mut cx: u16 = x + 1;
        while cx < right {
            let _: u16 = self.put_cell(cx, y, horiz);
            let _: u16 = self.put_cell(cx, bottom, horiz);
            cx += 1;
        }
        let vert: TerminalCell =
            TerminalCell::styled(TERMINAL_BOX_VERTICAL, 1, fg, bg, bold, rev, ul);
        let mut cy: u16 = y + 1;
        while cy < bottom {
            let _: u16 = self.put_cell(x, cy, vert);
            let _: u16 = self.put_cell(right, cy, vert);
            cy += 1;
        }
    }

    /// Serializes the grid into clean UTF-8 lines with zero ANSI escape sequences for piped CLI stdout.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Grid reference.
    ///
    /// # Returns
    ///
    /// - `String` - Plain-text UTF-8 representation trimmed of trailing whitespace per line and trailing blank lines.
    #[must_use]
    pub fn to_plain_text(&self) -> String {
        let cols: usize = self.get_cols() as usize;
        let rows: usize = self.get_rows() as usize;
        let cells: &[TerminalCell] = self.get_cells().as_slice();
        let mut lines: Vec<String> = Vec::with_capacity(rows);
        for row in 0..rows {
            let start: usize = row * cols;
            let end: usize = (start + cols).min(cells.len());
            let mut line: String = String::with_capacity(cols);
            for cell in &cells[start..end] {
                if cell.is_continuation() {
                    continue;
                }
                let ch: char = cell.get_ch();
                if ch != '\x1b' && ch != '\0' {
                    line.push(ch);
                }
            }
            let trimmed_len: usize = line.trim_end().len();
            line.truncate(trimmed_len);
            lines.push(line);
        }
        while let Some(last) = lines.last() {
            if last.is_empty() {
                let _: Option<String> = lines.pop();
            } else {
                break;
            }
        }
        lines.join("\n")
    }
}

/// Session lifecycle entry and idempotent ANSI state restoration for `TerminalSessionGuard`.
impl TerminalSessionGuard {
    /// Builds the escape sequence required to enter the configured interactive terminal modes.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether to enter the alternate screen buffer.
    /// - `bool` - Whether to hide the terminal cursor.
    /// - `bool` - Whether to enable raw bracketed input mode.
    ///
    /// # Returns
    ///
    /// - `String` - Concatenated ANSI enter sequence.
    #[must_use]
    pub fn build_enter_sequence(alt_screen: bool, cursor_hidden: bool, raw_mode: bool) -> String {
        let mut out: String = String::new();
        if alt_screen {
            out.push_str(ANSI_ENTER_ALT_SCREEN);
        }
        if cursor_hidden {
            out.push_str(ANSI_HIDE_CURSOR);
        }
        if raw_mode {
            out.push_str(ANSI_ENTER_RAW_MODES);
        }
        out
    }

    /// Builds the escape sequence required to cleanly restore terminal state upon exit or unwind.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether alternate screen mode was entered.
    /// - `bool` - Whether cursor visibility was hidden.
    /// - `bool` - Whether raw input modes were enabled.
    ///
    /// # Returns
    ///
    /// - `String` - Concatenated ANSI restoration sequence.
    #[must_use]
    pub fn build_restore_sequence(alt_screen: bool, cursor_hidden: bool, raw_mode: bool) -> String {
        let mut out: String = String::from(ANSI_RESET_STYLE);
        if raw_mode {
            out.push_str(ANSI_LEAVE_RAW_MODES);
        }
        if cursor_hidden {
            out.push_str(ANSI_SHOW_CURSOR);
        }
        if alt_screen {
            out.push_str(ANSI_LEAVE_ALT_SCREEN);
        }
        out
    }

    /// Enters an interactive terminal session writing enter/restore sequences to an optional capture buffer.
    ///
    /// # Arguments
    ///
    /// - `bool` - Enter alternate screen buffer.
    /// - `bool` - Hide terminal cursor.
    /// - `bool` - Enable raw input tracking modes.
    /// - `Option<Rc<RefCell<String>>>` - Optional capture buffer for deterministic testing or virtual PTYs.
    ///
    /// # Returns
    ///
    /// - `Self` - Active RAII session guard.
    #[must_use]
    pub fn enter_captured(
        alt_screen: bool,
        cursor_hidden: bool,
        raw_mode: bool,
        sink: Option<Rc<RefCell<String>>>,
    ) -> Self {
        let enter_seq: String = Self::build_enter_sequence(alt_screen, cursor_hidden, raw_mode);
        if let Some(buf) = sink.as_ref()
            && let Ok(mut guard) = buf.try_borrow_mut()
        {
            guard.push_str(&enter_seq);
        }
        Self {
            alt_screen,
            cursor_hidden,
            raw_mode,
            restored: false,
            write_to_stdout: false,
            sink,
        }
    }

    /// Enters an interactive terminal session flushing enter and restore sequences directly to stdout on native targets.
    ///
    /// # Arguments
    ///
    /// - `bool` - Enter alternate screen buffer.
    /// - `bool` - Hide terminal cursor.
    /// - `bool` - Enable raw input tracking modes.
    ///
    /// # Returns
    ///
    /// - `Self` - Active RAII session guard bound to stdout.
    #[must_use]
    pub fn enter_stdout(alt_screen: bool, cursor_hidden: bool, raw_mode: bool) -> Self {
        let enter_seq: String = Self::build_enter_sequence(alt_screen, cursor_hidden, raw_mode);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut stdout: io::Stdout = io::stdout();
            let _: io::Result<()> = stdout.write_all(enter_seq.as_bytes());
            let _: io::Result<()> = stdout.flush();
        }
        #[cfg(target_arch = "wasm32")]
        let _: &String = &enter_seq;
        Self {
            alt_screen,
            cursor_hidden,
            raw_mode,
            restored: false,
            write_to_stdout: true,
            sink: None,
        }
    }

    /// Restores terminal state immediately if not already restored and returns the emitted ANSI sequence.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable session guard.
    ///
    /// # Returns
    ///
    /// - `String` - Restoration escape sequence emitted (empty if already restored).
    pub fn restore_now(&mut self) -> String {
        if self.get_restored() {
            return String::new();
        }
        self.set_restored(true);
        let seq: String = Self::build_restore_sequence(
            self.get_alt_screen(),
            self.get_cursor_hidden(),
            self.get_raw_mode(),
        );
        let write_stdout: bool = self.get_write_to_stdout();
        let Self { sink, .. } = self;
        if let Some(buf) = sink.as_ref()
            && let Ok(mut guard) = buf.try_borrow_mut()
        {
            guard.push_str(&seq);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if write_stdout {
            let mut stdout: io::Stdout = io::stdout();
            let _: io::Result<()> = stdout.write_all(seq.as_bytes());
            let _: io::Result<()> = stdout.flush();
        }
        #[cfg(target_arch = "wasm32")]
        let _: bool = write_stdout;
        seq
    }
}

/// Guaranteed RAII restoration of cursor, raw mode, and alternate screen on normal exit or panic unwind.
impl Drop for TerminalSessionGuard {
    /// Restores terminal modes automatically when the guard leaves scope.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable session guard being dropped.
    fn drop(&mut self) {
        let _: String = self.restore_now();
    }
}

/// Semantic scene projection, keyboard focus navigation, and incremental ANSI cell-diffing for `TerminalRenderer`.
impl TerminalRenderer {
    /// Creates a fresh `TerminalRenderer` with empty diff history.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized terminal renderer.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears cached previous frame state, forcing the next interactive render to emit a full synchronization frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable terminal renderer.
    pub fn invalidate(&mut self) {
        let Self { previous, .. } = self;
        *previous = None;
    }

    /// Projects `tree` into a `cols x rows` character grid and emits either incremental ANSI TUI output or zero-ANSI static plain text.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable terminal renderer.
    /// - `&SceneTree` - Retained UI scene tree to project.
    /// - `u16` - Terminal viewport width in columns.
    /// - `u16` - Terminal viewport height in rows.
    /// - `TerminalRenderMode` - Interactive TUI diff mode or static plain-text CLI mode.
    ///
    /// # Returns
    ///
    /// - `(String, TerminalDiffStats)` - Rendered UTF-8 terminal payload and cell-diff telemetry.
    pub fn render_tree(
        &mut self,
        tree: &SceneTree,
        cols: u16,
        rows: u16,
        mode: TerminalRenderMode,
    ) -> (String, TerminalDiffStats) {
        let clamped_cols: u16 = cols.max(TERMINAL_MIN_COLS);
        let clamped_rows: u16 = rows.max(TERMINAL_MIN_ROWS);
        self.get_mut_current().reset(clamped_cols, clamped_rows);
        let focused: Option<NodeId> = tree.get_focus().focused_node();
        let mut cursor_y: u16 = 0;
        let root_region: TerminalRegion =
            TerminalRegion::new(0, clamped_cols, clamped_rows, focused);
        self.project_node(tree, tree.get_root(), root_region, &mut cursor_y);
        let total_cells: usize = self.get_current().total_cells();
        let prev_missing: bool = {
            let Self { previous, .. } = self;
            previous.is_none()
        };
        let resized_or_first: bool = prev_missing
            || self.get_last_cols() != clamped_cols
            || self.get_last_rows() != clamped_rows;
        let (output, stats): (String, TerminalDiffStats) = match mode {
            TerminalRenderMode::StaticPlainText => {
                let changed: usize = self.count_changed_cells(resized_or_first);
                let plain: String = self.get_current().to_plain_text();
                let bytes: usize = plain.len();
                let diff_stats: TerminalDiffStats =
                    TerminalDiffStats::new(total_cells, changed, bytes, false, resized_or_first);
                (plain, diff_stats)
            }
            TerminalRenderMode::InteractiveTui => {
                if resized_or_first {
                    let full_out: String = self.encode_full_frame();
                    let bytes: usize = full_out.len();
                    let diff_stats: TerminalDiffStats =
                        TerminalDiffStats::new(total_cells, total_cells, bytes, false, true);
                    (full_out, diff_stats)
                } else {
                    self.encode_diff_frame()
                }
            }
        };
        let snapshot: TerminalCellGrid = self.get_current().clone();
        let Self { previous, .. } = self;
        *previous = Some(snapshot);
        self.set_last_cols(clamped_cols);
        self.set_last_rows(clamped_rows);
        let next_frames: u64 = self.get_frames_rendered().wrapping_add(1);
        self.set_frames_rendered(next_frames);
        (output, stats)
    }

    /// Processes a terminal keyboard event (`Tab`, `Shift+Tab`, arrows, `Enter`, `Escape`, `Ctrl+C`) against `tree`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable terminal renderer.
    /// - `&mut SceneTree` - Retained UI scene tree to update focus, scroll, or activation on.
    /// - `KeyEvent` - Normalized keyboard event.
    ///
    /// # Returns
    ///
    /// - `TerminalNavAction` - Resulting focus, scroll, activation, or exit action.
    pub fn handle_key(&mut self, tree: &mut SceneTree, key: KeyEvent) -> TerminalNavAction {
        let code: KeyCode = key.get_code();
        let mods: Modifiers = key.get_modifiers();
        if code == KeyCode::Escape
            || (mods.get_ctrl()
                && matches!(code, KeyCode::Character('c') | KeyCode::Character('C')))
        {
            self.set_exit_requested(true);
            return TerminalNavAction::ExitRequested;
        }
        if code == KeyCode::Tab {
            let focusables: Vec<NodeId> = Self::collect_focusables(tree);
            if focusables.is_empty() {
                return TerminalNavAction::Ignored;
            }
            let current: Option<NodeId> = tree.get_focus().focused_node();
            let next_target: NodeId = match current
                .and_then(|cur: NodeId| focusables.iter().position(|id: &NodeId| *id == cur))
            {
                Some(idx) => {
                    let next_idx: usize = if mods.get_shift() {
                        if idx == 0 {
                            focusables.len() - 1
                        } else {
                            idx - 1
                        }
                    } else {
                        (idx + 1) % focusables.len()
                    };
                    focusables[next_idx]
                }
                None => {
                    if mods.get_shift() {
                        focusables[focusables.len() - 1]
                    } else {
                        focusables[0]
                    }
                }
            };
            let callbacks: Vec<(UiEventHandler, UiEvent)> = tree.update_focus(Some(next_target));
            for (handler, ev) in callbacks.into_iter() {
                if let Ok(mut cb) = handler.try_borrow_mut() {
                    cb(&ev);
                }
            }
            return TerminalNavAction::FocusMoved(next_target);
        }
        if code == KeyCode::ArrowDown || code == KeyCode::ArrowUp {
            let delta_rows: i16 = if code == KeyCode::ArrowDown { 1 } else { -1 };
            if let Some(scroll_id) = Self::find_first_scroll_view(tree) {
                let current_y: f32 = tree
                    .get_node(scroll_id)
                    .map(|n: &SceneNode| n.get_layout_box().get_scroll_offset().get_y())
                    .unwrap_or(0.0);
                let next_y: f32 =
                    (current_y + (delta_rows as f32) * TERMINAL_SCROLL_STEP_PX).max(0.0);
                tree.set_scroll_offset(scroll_id, Point::new(0.0, next_y));
                return TerminalNavAction::Scrolled(delta_rows);
            }
            return TerminalNavAction::Ignored;
        }
        if code == KeyCode::Enter {
            if let Some(target) = tree.get_focus().focused_node() {
                let click_kind: UiEventKind = UiEventKind::Click(Point::default());
                let handler_opt: Option<UiEventHandler> = tree
                    .get_node(target)
                    .and_then(|node: &SceneNode| node.get_handlers().handler_for(&click_kind));
                if let Some(handler) = handler_opt
                    && let Ok(mut cb) = handler.try_borrow_mut()
                {
                    let click_ev: UiEvent = UiEvent::new(target, click_kind);
                    cb(&click_ev);
                }
                return TerminalNavAction::Activated(target);
            }
            return TerminalNavAction::Ignored;
        }
        TerminalNavAction::Ignored
    }

    /// Collects all visible focusable or clickable node handles in pre-order traversal.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained UI scene tree.
    ///
    /// # Returns
    ///
    /// - `Vec<NodeId>` - Ordered focus ring candidates.
    #[must_use]
    fn collect_focusables(tree: &SceneTree) -> Vec<NodeId> {
        let mut out: Vec<NodeId> = Vec::new();
        let mut stack: Vec<NodeId> = vec![tree.get_root()];
        while let Some(id) = stack.pop() {
            let Some(node) = tree.get_node(id) else {
                continue;
            };
            if node.style_ref().get_layout().get_display() == Display::None
                || node.style_ref().get_paint().get_opacity() <= 0.0
            {
                continue;
            }
            if Self::is_focusable_or_interactive(node) {
                out.push(id);
            }
            for child in node.get_children().iter().rev().copied() {
                stack.push(child);
            }
        }
        out
    }

    /// Finds the first visible `NodeKind::ScrollView` node in pre-order traversal.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained UI scene tree.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - First visible scroll container handle.
    #[must_use]
    fn find_first_scroll_view(tree: &SceneTree) -> Option<NodeId> {
        let mut stack: Vec<NodeId> = vec![tree.get_root()];
        while let Some(id) = stack.pop() {
            let Some(node) = tree.get_node(id) else {
                continue;
            };
            if node.style_ref().get_layout().get_display() == Display::None {
                continue;
            }
            if matches!(node.get_kind(), NodeKind::ScrollView) {
                return Some(id);
            }
            for child in node.get_children().iter().rev().copied() {
                stack.push(child);
            }
        }
        None
    }

    /// Returns `true` when `node` should participate in terminal focus and clickable affordances.
    ///
    /// # Arguments
    ///
    /// - `&SceneNode` - Candidate scene node.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the node is focusable or interactive.
    #[must_use]
    fn is_focusable_or_interactive(node: &SceneNode) -> bool {
        node.get_interaction().get_focusable()
            || matches!(node.get_kind(), NodeKind::Input(_))
            || Self::is_clickable_node(node)
    }

    /// Returns `true` when `node` acts as a clickable button or interactive trigger.
    ///
    /// # Arguments
    ///
    /// - `&SceneNode` - Candidate scene node.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the node has click semantics.
    #[must_use]
    fn is_clickable_node(node: &SceneNode) -> bool {
        node.get_handlers()
            .handler_for(&UiEventKind::Click(Point::default()))
            .is_some()
            || node.style_ref().get_paint().get_cursor() == CursorIcon::Pointer
            || node
                .a11y_props()
                .is_some_and(|a: A11yProps| a.get_role() == A11yRole::Button)
    }

    /// Collects direct/descendant display label text for a clickable container button.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained UI scene tree.
    /// - `NodeId` - Button container node identifier.
    ///
    /// # Returns
    ///
    /// - `String` - Resolved button caption text.
    #[must_use]
    fn collect_button_label(tree: &SceneTree, node_id: NodeId) -> String {
        let Some(node) = tree.get_node(node_id) else {
            return String::new();
        };
        let mut parts: Vec<String> = Vec::new();
        let mut stack: Vec<NodeId> = node.get_children().iter().rev().copied().collect();
        while let Some(cid) = stack.pop() {
            let Some(child) = tree.get_node(cid) else {
                continue;
            };
            if child.style_ref().get_layout().get_display() == Display::None {
                continue;
            }
            if let NodeKind::Text(text) = child.get_kind() {
                let trimmed: &str = text.trim();
                if !trimmed.is_empty() {
                    parts.push(trimmed.to_string());
                }
            }
            for grand in child.get_children().iter().rev().copied() {
                stack.push(grand);
            }
        }
        if !parts.is_empty() {
            return parts.join(" ");
        }
        node.a11y_props()
            .map(|a: A11yProps| a.get_label().clone())
            .unwrap_or_default()
    }

    /// Projects a single scene node subtree into the current `TerminalCellGrid` using terminal character coordinates directly.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable terminal renderer.
    /// - `&SceneTree` - Retained scene tree.
    /// - `NodeId` - Target node handle to project.
    /// - `TerminalRegion` - Active character-grid region and focus context.
    /// - `&mut u16` - Mutable current row cursor advanced as content is laid out.
    fn project_node(
        &mut self,
        tree: &SceneTree,
        node_id: NodeId,
        region: TerminalRegion,
        y: &mut u16,
    ) {
        let x: u16 = region.get_x();
        let avail_w: u16 = region.get_width();
        let max_y: u16 = region.get_max_y();
        let focused: Option<NodeId> = region.get_focused();
        if *y >= max_y || avail_w == 0 {
            return;
        }
        let Some(node) = tree.get_node(node_id) else {
            return;
        };
        let style: &Style = node.style_ref();
        let layout: LayoutStyle = style.get_layout();
        let paint: PaintStyle = style.get_paint();
        let text_style: TextStyle = style.get_text();
        if layout.get_display() == Display::None || paint.get_opacity() <= 0.0 {
            return;
        }
        let is_focused: bool = focused == Some(node_id) || node.get_interaction().get_focused();
        let is_bold: bool = matches!(
            text_style.get_weight(),
            FontWeight::SemiBold | FontWeight::Bold
        );
        let fg: Option<Color> = Some(text_style.get_color());
        let bg: Option<Color> = paint.background_color();
        let template: TerminalCell = TerminalCell::styled(
            ' ',
            1,
            fg,
            bg,
            is_bold,
            is_focused,
            text_style.get_underline(),
        );
        match node.get_kind() {
            NodeKind::Text(content) => {
                let display_str: String = if Self::is_clickable_node(node) {
                    if is_focused {
                        format!("> [ {content} ]")
                    } else {
                        format!("[ {content} ]")
                    }
                } else {
                    content.clone()
                };
                self.write_wrapped_text(region, y, &display_str, template);
            }
            NodeKind::Input(inp) => {
                let inner: String = if inp.get_value().is_empty() {
                    inp.get_placeholder().clone()
                } else if inp.get_password() {
                    let count: usize = inp.get_value().chars().count();
                    "*".repeat(count)
                } else {
                    inp.get_value().clone()
                };
                let formatted: String = if is_focused {
                    format!("> [ {inner}| ]")
                } else {
                    format!("[ {inner} ]")
                };
                let _: u16 = self
                    .get_mut_current()
                    .write_styled_text(x, *y, avail_w, &formatted, template);
                *y = y.saturating_add(1);
            }
            NodeKind::Image(img) => {
                let alt: String = node
                    .a11y_props()
                    .map(|a: A11yProps| a.get_label().clone())
                    .filter(|s: &String| !s.is_empty())
                    .map(|lbl: String| format!("{TERMINAL_IMAGE_FALLBACK_PREFIX}{lbl}]"))
                    .unwrap_or_else(|| {
                        format!(
                            "{TERMINAL_IMAGE_DIM_PREFIX}{}x{}]",
                            img.get_width(),
                            img.get_height()
                        )
                    });
                let _: u16 = self
                    .get_mut_current()
                    .write_styled_text(x, *y, avail_w, &alt, template);
                *y = y.saturating_add(1);
            }
            NodeKind::Canvas => {
                let alt: String = node
                    .a11y_props()
                    .map(|a: A11yProps| a.get_label().clone())
                    .filter(|s: &String| !s.is_empty())
                    .map(|lbl: String| format!("{TERMINAL_CANVAS_FALLBACK_PREFIX}{lbl}]"))
                    .unwrap_or_else(|| TERMINAL_CANVAS_FALLBACK.to_string());
                let _: u16 = self
                    .get_mut_current()
                    .write_styled_text(x, *y, avail_w, &alt, template);
                *y = y.saturating_add(1);
            }
            NodeKind::Fragment => {
                for child_id in node.get_children().iter().copied() {
                    if *y >= max_y {
                        break;
                    }
                    self.project_node(tree, child_id, region, y);
                }
            }
            NodeKind::ScrollView => {
                self.project_scroll_view(tree, node_id, region, y);
            }
            NodeKind::View | NodeKind::Portal => {
                let is_root: bool = node_id == tree.get_root();
                if !is_root && Self::is_clickable_node(node) {
                    let caption: String = Self::collect_button_label(tree, node_id);
                    if !caption.is_empty() {
                        let formatted: String = if is_focused {
                            format!("> [ {caption} ]")
                        } else {
                            format!("[ {caption} ]")
                        };
                        let _: u16 = self
                            .get_mut_current()
                            .write_styled_text(x, *y, avail_w, &formatted, template);
                        *y = y.saturating_add(1);
                        return;
                    }
                }
                let has_border: bool = !is_root
                    && (paint.get_border().get_width() > 0.0
                        || matches!(node.get_kind(), NodeKind::Portal))
                    && avail_w >= 4
                    && max_y.saturating_sub(*y) >= 3;
                if has_border {
                    let top_y: u16 = *y;
                    let inner_region: TerminalRegion = TerminalRegion::new(
                        x.saturating_add(1),
                        avail_w.saturating_sub(2),
                        max_y.saturating_sub(1),
                        focused,
                    );
                    let mut child_y: u16 = top_y.saturating_add(1);
                    self.project_children_flow(tree, node, inner_region, &mut child_y);
                    let box_h: u16 = child_y
                        .saturating_sub(top_y)
                        .saturating_add(1)
                        .max(3)
                        .min(max_y.saturating_sub(top_y));
                    let border_cell: TerminalCell = TerminalCell::styled(
                        ' ',
                        1,
                        Some(paint.get_border().get_color()),
                        bg,
                        false,
                        is_focused,
                        false,
                    );
                    self.get_mut_current()
                        .draw_box(x, top_y, avail_w, box_h, border_cell);
                    *y = top_y.saturating_add(box_h);
                } else {
                    self.project_children_flow(tree, node, region, y);
                }
            }
        }
    }

    /// Projects children of a container respecting row, column, and grid directions directly in character columns.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable terminal renderer.
    /// - `&SceneTree` - Retained scene tree.
    /// - `&SceneNode` - Container scene node.
    /// - `TerminalRegion` - Active character-grid region and focus context.
    /// - `&mut u16` - Mutable current row cursor.
    fn project_children_flow(
        &mut self,
        tree: &SceneTree,
        node: &SceneNode,
        region: TerminalRegion,
        y: &mut u16,
    ) {
        let x: u16 = region.get_x();
        let avail_w: u16 = region.get_width();
        let max_y: u16 = region.get_max_y();
        let focused: Option<NodeId> = region.get_focused();
        let visible_children: Vec<NodeId> = node
            .get_children()
            .iter()
            .copied()
            .filter(|cid: &NodeId| {
                tree.get_node(*cid).is_some_and(|c: &SceneNode| {
                    c.style_ref().get_layout().get_display() != Display::None
                        && c.style_ref().get_paint().get_opacity() > 0.0
                })
            })
            .collect();
        if visible_children.is_empty() {
            return;
        }
        let layout: LayoutStyle = node.style_ref().get_layout();
        let is_row: bool = matches!(
            layout.get_flex_direction(),
            FlexDirection::Row | FlexDirection::RowReverse
        ) && layout.get_display() == Display::Flex;
        if is_row && visible_children.len() > 1 && avail_w >= (visible_children.len() as u16) * 2 {
            let count: u16 = visible_children.len() as u16;
            let total_gaps: u16 = count.saturating_sub(1).saturating_mul(TERMINAL_ROW_GAP);
            let slot_w: u16 = avail_w.saturating_sub(total_gaps) / count;
            if slot_w >= 2 {
                let row_start_y: u16 = *y;
                let mut max_child_end: u16 = row_start_y;
                for (idx, child_id) in visible_children.iter().copied().enumerate() {
                    let cx: u16 = x.saturating_add((idx as u16).saturating_mul(slot_w + 1));
                    let rem_w: u16 = avail_w.saturating_sub(cx.saturating_sub(x));
                    let col_w: u16 = if idx + 1 == visible_children.len() {
                        rem_w
                    } else {
                        slot_w.min(rem_w)
                    };
                    let mut cy: u16 = row_start_y;
                    let child_region: TerminalRegion =
                        TerminalRegion::new(cx, col_w, max_y, focused);
                    self.project_node(tree, child_id, child_region, &mut cy);
                    if cy > max_child_end {
                        max_child_end = cy;
                    }
                }
                *y = max_child_end;
                return;
            }
        }
        for child_id in visible_children.into_iter() {
            if *y >= max_y {
                break;
            }
            self.project_node(tree, child_id, region, y);
        }
    }

    /// Projects a `NodeKind::ScrollView` container with row-level scroll skipping and an ASCII/Unicode scrollbar rail.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable terminal renderer.
    /// - `&SceneTree` - Retained scene tree.
    /// - `NodeId` - Scroll container node handle.
    /// - `TerminalRegion` - Active character-grid region and focus context.
    /// - `&mut u16` - Mutable current row cursor.
    fn project_scroll_view(
        &mut self,
        tree: &SceneTree,
        node_id: NodeId,
        region: TerminalRegion,
        y: &mut u16,
    ) {
        let x: u16 = region.get_x();
        let avail_w: u16 = region.get_width();
        let max_y: u16 = region.get_max_y();
        let focused: Option<NodeId> = region.get_focused();
        let Some(node) = tree.get_node(node_id) else {
            return;
        };
        let visible_children: Vec<NodeId> = node
            .get_children()
            .iter()
            .copied()
            .filter(|cid: &NodeId| {
                tree.get_node(*cid).is_some_and(|c: &SceneNode| {
                    c.style_ref().get_layout().get_display() != Display::None
                        && c.style_ref().get_paint().get_opacity() > 0.0
                })
            })
            .collect();
        let avail_h: u16 = max_y.saturating_sub(*y);
        if avail_h == 0 || visible_children.is_empty() {
            return;
        }
        let scroll_y: f32 = node.get_layout_box().get_scroll_offset().get_y().max(0.0);
        let skip_items: usize = ((scroll_y / TERMINAL_SCROLL_STEP_PX).floor() as usize)
            .min(visible_children.len().saturating_sub(1));
        let needs_rail: bool = visible_children.len() > (avail_h as usize) && avail_w >= 3;
        let inner_w: u16 = if needs_rail {
            avail_w.saturating_sub(1)
        } else {
            avail_w
        };
        let child_region: TerminalRegion = TerminalRegion::new(x, inner_w, max_y, focused);
        let start_y: u16 = *y;
        for child_id in visible_children.iter().skip(skip_items).copied() {
            if *y >= max_y {
                break;
            }
            self.project_node(tree, child_id, child_region, y);
        }
        if needs_rail {
            let rail_x: u16 = x.saturating_add(inner_w);
            let total_items: usize = visible_children.len().max(1);
            let thumb_offset: u16 = ((skip_items * (avail_h as usize)) / total_items)
                .min((avail_h - 1) as usize) as u16;
            for offset in 0..avail_h {
                let glyph: char = if offset == thumb_offset {
                    TERMINAL_SCROLL_THUMB
                } else {
                    TERMINAL_SCROLL_TRACK
                };
                let _: u16 = self.get_mut_current().put_cell(
                    rail_x,
                    start_y.saturating_add(offset),
                    TerminalCell::styled(glyph, 1, None, None, false, false, false),
                );
            }
        }
    }

    /// Writes `text` wrapped across consecutive terminal rows within `region` without breaking CJK double-width cells.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable terminal renderer.
    /// - `TerminalRegion` - Active character-grid region.
    /// - `&mut u16` - Mutable current row cursor.
    /// - `&str` - UTF-8 text to wrap.
    /// - `TerminalCell` - Style template cell.
    fn write_wrapped_text(
        &mut self,
        region: TerminalRegion,
        y: &mut u16,
        text: &str,
        template: TerminalCell,
    ) {
        let x: u16 = region.get_x();
        let avail_w: u16 = region.get_width();
        let max_y: u16 = region.get_max_y();
        if *y >= max_y || avail_w == 0 {
            return;
        }
        let mut wrote_any_line: bool = false;
        for raw_line in text.split('\n') {
            if *y >= max_y {
                break;
            }
            if raw_line.is_empty() {
                *y = y.saturating_add(1);
                wrote_any_line = true;
                continue;
            }
            let mut cur_col: u16 = 0;
            for ch in raw_line.chars() {
                if ch == '\r' {
                    continue;
                }
                let width: u8 = terminal_char_width(ch);
                if width == 0 {
                    continue;
                }
                let w_u16: u16 = width as u16;
                if w_u16 > avail_w {
                    continue;
                }
                if cur_col.saturating_add(w_u16) > avail_w {
                    *y = y.saturating_add(1);
                    if *y >= max_y {
                        return;
                    }
                    cur_col = 0;
                }
                let cell: TerminalCell = TerminalCell::styled(
                    ch,
                    width,
                    template.fg_color(),
                    template.bg_color(),
                    template.get_bold(),
                    template.get_reverse(),
                    template.get_underline(),
                );
                let consumed: u16 = self.get_mut_current().put_cell(x + cur_col, *y, cell);
                cur_col = cur_col.saturating_add(consumed);
            }
            *y = y.saturating_add(1);
            wrote_any_line = true;
        }
        if !wrote_any_line {
            *y = y.saturating_add(1);
        }
    }

    /// Counts how many cells differ between `current` and `previous`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Terminal renderer reference.
    /// - `bool` - Whether the entire frame is treated as changed (first frame or resize).
    ///
    /// # Returns
    ///
    /// - `usize` - Number of differing cells.
    #[must_use]
    fn count_changed_cells(&self, full_sync: bool) -> usize {
        if full_sync {
            return self.get_current().total_cells();
        }
        let Self {
            previous, current, ..
        } = self;
        let Some(prev_grid) = previous.as_ref() else {
            return current.total_cells();
        };
        let cur_cells: &[TerminalCell] = current.get_cells().as_slice();
        let prev_cells: &[TerminalCell] = prev_grid.get_cells().as_slice();
        if cur_cells.len() != prev_cells.len() {
            return cur_cells.len();
        }
        cur_cells
            .iter()
            .zip(prev_cells.iter())
            .filter(|(a, b): &(&TerminalCell, &TerminalCell)| a != b)
            .count()
    }

    /// Encodes the full current grid into a single ANSI clear-and-draw payload.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Terminal renderer reference.
    ///
    /// # Returns
    ///
    /// - `String` - Full-sync ANSI output payload.
    #[must_use]
    fn encode_full_frame(&self) -> String {
        let grid: &TerminalCellGrid = self.get_current();
        let cols: u16 = grid.get_cols();
        let rows: u16 = grid.get_rows();
        let mut out: String = String::with_capacity(grid.total_cells().saturating_mul(2));
        out.push_str(ANSI_CLEAR_AND_HOME);
        let mut active_style: Option<TerminalCell> = None;
        for row in 0..rows {
            append_cursor_move(&mut out, 0, row);
            for col in 0..cols {
                let Some(cell) = grid.cell(col, row) else {
                    continue;
                };
                if cell.is_continuation() {
                    continue;
                }
                let style_changed: bool = match active_style {
                    Some(prev) => !prev.same_style(&cell),
                    None => true,
                };
                if style_changed {
                    append_sgr_style(
                        &mut out,
                        cell.fg_color(),
                        cell.bg_color(),
                        cell.get_bold(),
                        cell.get_reverse(),
                        cell.get_underline(),
                    );
                    active_style = Some(cell);
                }
                out.push(cell.get_ch());
            }
        }
        out.push_str(ANSI_RESET_STYLE);
        out
    }

    /// Encodes only cells changed relative to `previous`, returning an empty string (`0` bytes) when idle.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Terminal renderer reference.
    ///
    /// # Returns
    ///
    /// - `(String, TerminalDiffStats)` - Incremental ANSI diff payload and diff telemetry.
    #[must_use]
    fn encode_diff_frame(&self) -> (String, TerminalDiffStats) {
        let Self {
            previous, current, ..
        } = self;
        let total_cells: usize = current.total_cells();
        let Some(prev_grid) = previous.as_ref() else {
            let full: String = self.encode_full_frame();
            let len: usize = full.len();
            return (
                full,
                TerminalDiffStats::new(total_cells, total_cells, len, false, true),
            );
        };
        let cols: u16 = current.get_cols();
        let rows: u16 = current.get_rows();
        let mut out: String = String::new();
        let mut changed_cells: usize = 0;
        let mut active_style: Option<TerminalCell> = None;
        let mut cursor_pos: Option<(u16, u16)> = None;
        for row in 0..rows {
            let mut col: u16 = 0;
            while col < cols {
                let Some(cur_cell) = current.cell(col, row) else {
                    col += 1;
                    continue;
                };
                let Some(prev_cell) = prev_grid.cell(col, row) else {
                    col += 1;
                    continue;
                };
                if cur_cell.is_continuation() {
                    col += 1;
                    continue;
                }
                let mut cell_dirty: bool = cur_cell != prev_cell;
                if cur_cell.is_wide_lead() && col + 1 < cols {
                    let cur_trail: Option<TerminalCell> = current.cell(col + 1, row);
                    let prev_trail: Option<TerminalCell> = prev_grid.cell(col + 1, row);
                    if cur_trail != prev_trail {
                        cell_dirty = true;
                    }
                }
                if !cell_dirty {
                    col += cur_cell.get_width().max(1) as u16;
                    continue;
                }
                let step: u16 = cur_cell.get_width().max(1) as u16;
                changed_cells = changed_cells.saturating_add(step as usize);
                if cursor_pos != Some((col, row)) {
                    append_cursor_move(&mut out, col, row);
                }
                let style_changed: bool = match active_style {
                    Some(prev) => !prev.same_style(&cur_cell),
                    None => true,
                };
                if style_changed {
                    append_sgr_style(
                        &mut out,
                        cur_cell.fg_color(),
                        cur_cell.bg_color(),
                        cur_cell.get_bold(),
                        cur_cell.get_reverse(),
                        cur_cell.get_underline(),
                    );
                    active_style = Some(cur_cell);
                }
                out.push(cur_cell.get_ch());
                cursor_pos = Some((col.saturating_add(step), row));
                col = col.saturating_add(step);
            }
        }
        if changed_cells == 0 {
            return (
                String::new(),
                TerminalDiffStats::new(total_cells, 0, 0, true, false),
            );
        }
        out.push_str(ANSI_RESET_STYLE);
        let output_bytes: usize = out.len();
        (
            out,
            TerminalDiffStats::new(total_cells, changed_cells, output_bytes, false, false),
        )
    }
}
