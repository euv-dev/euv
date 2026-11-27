/// ANSI escape sequence entering the alternate screen buffer.
pub const ANSI_ENTER_ALT_SCREEN: &str = "\x1b[?1049h";
/// ANSI escape sequence leaving the alternate screen buffer.
pub const ANSI_LEAVE_ALT_SCREEN: &str = "\x1b[?1049l";
/// ANSI escape sequence hiding the terminal cursor.
pub const ANSI_HIDE_CURSOR: &str = "\x1b[?25l";
/// ANSI escape sequence restoring terminal cursor visibility.
pub const ANSI_SHOW_CURSOR: &str = "\x1b[?25h";
/// ANSI escape sequence resetting all SGR visual attributes.
pub const ANSI_RESET_STYLE: &str = "\x1b[0m";
/// ANSI escape sequence clearing the entire screen and homing the cursor.
pub const ANSI_CLEAR_AND_HOME: &str = "\x1b[2J\x1b[H";
/// ANSI escape sequence enabling raw bracketed paste tracking.
pub const ANSI_ENTER_RAW_MODES: &str = "\x1b[?2004h";
/// ANSI escape sequence disabling raw bracketed paste and mouse tracking.
pub const ANSI_LEAVE_RAW_MODES: &str = "\x1b[?2004l\x1b[?1000l";

pub(crate) const ANSI_CSI_PREFIX: &str = "\x1b[";
pub(crate) const ANSI_BOLD_CODE: &str = ";1";
pub(crate) const ANSI_REVERSE_CODE: &str = ";7";
pub(crate) const ANSI_UNDERLINE_CODE: &str = ";4";
pub(crate) const ANSI_FG_TRUECOLOR_PREFIX: &str = ";38;2;";
pub(crate) const ANSI_BG_TRUECOLOR_PREFIX: &str = ";48;2;";
pub(crate) const TERMINAL_CANVAS_FALLBACK: &str = "[Canvas/Viewport]";
pub(crate) const TERMINAL_IMAGE_FALLBACK_PREFIX: &str = "[Image: ";
pub(crate) const TERMINAL_CANVAS_FALLBACK_PREFIX: &str = "[Canvas: ";
pub(crate) const TERMINAL_IMAGE_DIM_PREFIX: &str = "[Image ";
pub(crate) const TERMINAL_BOX_TOP_LEFT: char = '┌';
pub(crate) const TERMINAL_BOX_TOP_RIGHT: char = '┐';
pub(crate) const TERMINAL_BOX_BOTTOM_LEFT: char = '└';
pub(crate) const TERMINAL_BOX_BOTTOM_RIGHT: char = '┘';
pub(crate) const TERMINAL_BOX_HORIZONTAL: char = '─';
pub(crate) const TERMINAL_BOX_VERTICAL: char = '│';
pub(crate) const TERMINAL_SCROLL_THUMB: char = '█';
pub(crate) const TERMINAL_SCROLL_TRACK: char = '│';
pub(crate) const TERMINAL_MIN_COLS: u16 = 1;
pub(crate) const TERMINAL_MIN_ROWS: u16 = 1;
pub(crate) const TERMINAL_ROW_GAP: u16 = 1;
pub(crate) const TERMINAL_SCROLL_STEP_PX: f32 = 24.0;
