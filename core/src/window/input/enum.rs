use super::*;

/// Source pointer device class unified across desktop, tablet, mobile, and web.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PointerDevice {
    /// Mouse or trackpad cursor.
    #[default]
    Mouse,
    /// Capacitive touch finger contact.
    Touch,
    /// Stylus or Apple Pencil contact with pressure/tilt metadata.
    Stylus,
}

/// Mouse / stylus button identity.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PointerButton {
    /// Primary left click / finger tap.
    #[default]
    Primary,
    /// Secondary right click / context button.
    Secondary,
    /// Middle scroll-wheel click.
    Auxiliary,
}

/// Unified physical/logical key code covering desktop, iPadOS hardware keyboard, Android, and Web.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum KeyCode {
    /// Unmapped key.
    #[default]
    Unknown,
    /// Enter / Return key.
    Enter,
    /// Escape key.
    Escape,
    /// Tab key.
    Tab,
    /// Backspace key.
    Backspace,
    /// Delete key.
    Delete,
    /// Spacebar.
    Space,
    /// Arrow Up.
    ArrowUp,
    /// Arrow Down.
    ArrowDown,
    /// Arrow Left.
    ArrowLeft,
    /// Arrow Right.
    ArrowRight,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up.
    PageUp,
    /// Page Down.
    PageDown,
    /// Printable character key.
    Character(char),
}

/// Mobile / desktop application lifecycle state transitions.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LifecycleEvent {
    /// Application resumed foreground activity.
    Resumed,
    /// Application moved to background / inactive state.
    Paused,
    /// Operating system signaled low-memory pressure.
    LowMemory,
    /// Application is terminating.
    Terminated,
}

/// Input Method Editor (IME) composition event for CJK and mobile virtual keyboards.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImeEvent {
    /// IME enabled on the active text input.
    Enabled,
    /// In-progress pre-edit text with optional selection range.
    Preedit(String, Option<(usize, usize)>),
    /// Final committed text string.
    Commit(String),
    /// IME dismissed.
    Disabled,
}

/// Complete unified platform window event dispatched to the application loop.
#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    /// Window logical and physical dimensions changed.
    Resized(WindowId, u32, u32, f32),
    /// Safe-area insets updated (rotation, Stage Manager, virtual keyboard).
    SafeAreaChanged(WindowId, SafeAreaInsets),
    /// OS light/dark appearance toggled.
    ThemeChanged(WindowId, ThemeMode),
    /// Window gained or lost focus.
    Focused(WindowId, bool),
    /// OS requested window closure.
    CloseRequested(WindowId),
    /// Pointer pressed down.
    PointerDown(WindowId, PointerEvent),
    /// Pointer moved.
    PointerMove(WindowId, PointerEvent),
    /// Pointer released.
    PointerUp(WindowId, PointerEvent),
    /// Pointer gesture cancelled by OS.
    PointerCancel(WindowId, u64),
    /// Scroll wheel / trackpad / pan delta.
    Scroll(WindowId, Point),
    /// Keyboard key pressed.
    KeyDown(WindowId, KeyEvent),
    /// Keyboard key released.
    KeyUp(WindowId, KeyEvent),
    /// Direct UTF-8 text input event.
    TextInput(WindowId, String),
    /// IME composition lifecycle event.
    Ime(WindowId, ImeEvent),
    /// Application lifecycle transition.
    Lifecycle(LifecycleEvent),
    /// Platform requests a fresh rendered frame.
    RedrawRequested(WindowId),
    /// Operating system delivered a custom-scheme or universal deep link URI.
    DeepLink(DeepLinkEvent),
}
