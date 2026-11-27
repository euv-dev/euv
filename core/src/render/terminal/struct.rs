use super::*;

/// Single character cell inside a `TerminalCellGrid` supporting wide CJK glyphs and truecolor styling.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, PartialEq)]
pub struct TerminalCell {
    /// Displayed Unicode scalar value (`' '` when empty, `'\0'` on trailing wide-char continuation cells).
    #[get(type(copy))]
    pub(crate) ch: char,
    /// Display column width: `1` for narrow, `2` for wide lead cell, `0` for wide continuation cell.
    #[get(type(copy))]
    pub(crate) width: u8,
    /// Optional 24-bit foreground text color.
    #[get(skip)]
    pub(crate) fg: Option<Color>,
    /// Optional 24-bit cell background color.
    #[get(skip)]
    pub(crate) bg: Option<Color>,
    /// Whether bold SGR emphasis is active.
    #[get(type(copy))]
    pub(crate) bold: bool,
    /// Whether reverse-video SGR highlight (used for focus indicators) is active.
    #[get(type(copy))]
    pub(crate) reverse: bool,
    /// Whether underline SGR decoration is active.
    #[get(type(copy))]
    pub(crate) underline: bool,
}

/// Dense `cols x rows` character cell surface for deterministic terminal TUI and CLI rendering.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct TerminalCellGrid {
    /// Terminal grid width in character columns.
    #[get(type(copy))]
    pub(crate) cols: u16,
    /// Terminal grid height in character rows.
    #[get(type(copy))]
    pub(crate) rows: u16,
    /// Row-major cell buffer of length `cols as usize * rows as usize`.
    pub(crate) cells: Vec<TerminalCell>,
}

/// Per-frame terminal cell-diff and bandwidth telemetry produced by `TerminalRenderer::render_tree`.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, PartialEq)]
pub struct TerminalDiffStats {
    /// Total cells in the active `cols x rows` terminal grid.
    #[get(type(copy))]
    pub(crate) total_cells: usize,
    /// Number of cells that differed from the previous frame (equals `total_cells` on full sync).
    #[get(type(copy))]
    pub(crate) changed_cells: usize,
    /// Number of UTF-8 bytes emitted in the rendered output payload.
    #[get(type(copy))]
    pub(crate) output_bytes: usize,
    /// `true` when zero cells changed in interactive mode and zero bytes were flushed.
    #[get(type(copy))]
    pub(crate) is_idle: bool,
    /// `true` when the frame performed a full screen synchronization (first frame or resize).
    #[get(type(copy))]
    pub(crate) full_sync: bool,
}

/// RAII terminal session guard restoring cursor visibility, raw mode flags, and alternate screen on drop.
#[derive(CustomDebug, Data)]
pub struct TerminalSessionGuard {
    /// Whether alternate screen mode was entered and must be exited on drop.
    #[get(type(copy))]
    pub(crate) alt_screen: bool,
    /// Whether cursor visibility was hidden and must be restored on drop.
    #[get(type(copy))]
    pub(crate) cursor_hidden: bool,
    /// Whether terminal raw/bracketed input modes were enabled and must be reset on drop.
    #[get(type(copy))]
    pub(crate) raw_mode: bool,
    /// Whether terminal restoration has already completed.
    #[get(type(copy))]
    pub(crate) restored: bool,
    /// Whether enter/restore sequences should also be flushed directly to standard output on native targets.
    #[get(type(copy))]
    pub(crate) write_to_stdout: bool,
    /// Optional capture buffer recording emitted enter and restore escape sequences for deterministic testing.
    #[debug(skip)]
    #[get(skip)]
    #[get_mut(skip)]
    #[set(skip)]
    pub(crate) sink: Option<Rc<RefCell<String>>>,
}

/// Stateful terminal renderer projecting a retained `SceneTree` into interactive ANSI diffs or static CLI plain text.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct TerminalRenderer {
    /// Snapshot of the previously rendered grid for incremental cell-diffing.
    #[get(skip)]
    pub(crate) previous: Option<TerminalCellGrid>,
    /// Working character grid populated during the current frame.
    pub(crate) current: TerminalCellGrid,
    /// Last rendered terminal column count for resize detection.
    #[get(type(copy))]
    pub(crate) last_cols: u16,
    /// Last rendered terminal row count for resize detection.
    #[get(type(copy))]
    pub(crate) last_rows: u16,
    /// Total frames projected through this renderer.
    #[get(type(copy))]
    pub(crate) frames_rendered: u64,
    /// Whether keyboard navigation has requested terminal session exit.
    #[get(type(copy))]
    pub(crate) exit_requested: bool,
}

/// Internal character-grid layout viewport passed recursively while projecting scene nodes.
#[derive(Clone, Copy, Data, Debug, Default, Eq, New, PartialEq)]
pub(crate) struct TerminalRegion {
    /// Left column bound of the active character region.
    #[get(type(copy))]
    pub(crate) x: u16,
    /// Available horizontal width in terminal columns.
    #[get(type(copy))]
    pub(crate) width: u16,
    /// Exclusive bottom row limit of the active character region.
    #[get(type(copy))]
    pub(crate) max_y: u16,
    /// Currently focused scene node handle, if any.
    #[get(type(copy))]
    pub(crate) focused: Option<NodeId>,
}
