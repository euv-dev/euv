use super::*;

/// Output mode controlling whether `TerminalRenderer` emits ANSI TUI diffs or plain UTF-8 CLI text.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TerminalRenderMode {
    /// Interactive terminal user interface mode emitting ANSI cursor and style cell-diffs.
    #[default]
    InteractiveTui,
    /// Static CLI plain-text mode emitting clean UTF-8 lines with zero ANSI escape sequences.
    StaticPlainText,
}

/// High-level navigation outcome returned by `TerminalRenderer::handle_key`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TerminalNavAction {
    /// Keyboard event did not trigger a terminal navigation change.
    #[default]
    Ignored,
    /// Keyboard focus moved to a new focusable scene node.
    FocusMoved(NodeId),
    /// Active scroll container offset changed via arrow navigation.
    Scrolled(i16),
    /// Currently focused interactive control was activated via Enter.
    Activated(NodeId),
    /// User requested graceful exit via Escape or Ctrl+C.
    ExitRequested,
}
