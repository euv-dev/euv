use super::*;

/// Decoded Win32 `WndProc` callback event queued for `WindowsPlatform::poll_events`.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Win32EventPayload {
    /// OS requested window close (`WM_CLOSE`).
    Close,
    /// Client dimensions or DPI scale changed (`WM_SIZE`, `WM_DPICHANGED`).
    Resized(u32, u32, f32),
    /// Window requires repaint (`WM_PAINT`).
    Redraw,
    /// Window focus gained or lost (`WM_SETFOCUS`, `WM_KILLFOCUS`).
    Focused(bool),
    /// Pointer moved inside client coordinates (`WM_MOUSEMOVE`).
    PointerMove(i32, i32, Modifiers),
    /// Mouse button pressed (`WM_*BUTTONDOWN`).
    PointerDown(i32, i32, PointerButton, Modifiers),
    /// Mouse button released (`WM_*BUTTONUP`).
    PointerUp(i32, i32, PointerButton, Modifiers),
    /// Mouse wheel scrolled (`WM_MOUSEWHEEL`).
    Scroll(f32),
    /// Keyboard key pressed (`WM_KEYDOWN`).
    KeyDown(KeyCode, Modifiers, bool),
    /// Keyboard key released (`WM_KEYUP`).
    KeyUp(KeyCode, Modifiers),
    /// Decoded UTF-16 character input (`WM_CHAR`).
    Text(char),
    /// Application activation state changed (`WM_ACTIVATEAPP`).
    Activated(bool),
    /// Window minimized or restored (`WM_SIZE` `SIZE_MINIMIZED`).
    Minimized(bool),
    /// System clipboard contents changed (`WM_CLIPBOARDUPDATE`).
    ClipboardUpdated,
    /// System appearance or settings changed (`WM_SETTINGCHANGE` / `WM_THEMECHANGED`).
    ThemeUpdated,
    /// System power / battery-saver status changed (`WM_POWERBROADCAST`).
    PowerStatusChanged(bool),
    /// Window destroyed (`WM_DESTROY`).
    Destroyed,
}
