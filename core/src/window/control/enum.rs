use super::*;

/// Cross-platform imperative window command queued for execution by the platform backend.
#[derive(Clone, Debug, PartialEq)]
pub enum WindowCommand {
    /// Update window or document title.
    SetTitle(WindowId, String),
    /// Resize window to logical `(width, height)`.
    SetSize(WindowId, u32, u32),
    /// Move window to screen `(x, y)`.
    SetPosition(WindowId, i32, i32),
    /// Toggle window resizability.
    SetResizable(WindowId, bool),
    /// Toggle window OS decorations.
    SetDecorations(WindowId, bool),
    /// Toggle always-on-top z-order.
    SetAlwaysOnTop(WindowId, bool),
    /// Change fullscreen presentation mode.
    SetFullscreen(WindowId, FullscreenMode),
    /// Toggle window visibility.
    SetVisible(WindowId, bool),
    /// Update active pointer cursor icon.
    SetCursor(WindowId, CursorIcon),
    /// Toggle IME / mobile soft keyboard visibility and target rect.
    SetIme(WindowId, bool, Rect),
    /// Update mobile/iPadOS status bar style.
    SetStatusBarStyle(WindowId, StatusBarStyle),
    /// Request an immediate redraw frame.
    RequestRedraw(WindowId),
    /// Switch active render mode dynamically (`Auto`, `Software`, `Hardware`, `Hybrid`).
    SetRenderMode(WindowId, RenderMode),
    /// Close the target window.
    Close(WindowId),
}
