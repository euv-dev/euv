/// Standard mobile/desktop app convention prompt displayed at the bottom when pressing Back at the root route.
pub const DOUBLE_BACK_EXIT_PROMPT: &str = "Press back again to exit";
/// Standard confirmation window in milliseconds for a consecutive second Back press to exit the app.
pub(crate) const DOUBLE_BACK_EXIT_WINDOW_MS: u64 = 2000;
/// Hold duration in seconds before the bottom exit toast begins its smooth fade-out.
pub(crate) const DOUBLE_BACK_TOAST_HOLD_SECS: f32 = 1.72;
/// Fade-out duration in seconds for the bottom exit toast.
pub(crate) const DOUBLE_BACK_TOAST_FADE_SECS: f32 = 0.28;
/// Top-layer z-index ensuring the bottom exit prompt renders cleanly above all page content.
pub(crate) const Z_INDEX_EXIT_TOAST: i32 = 950;
/// Bottom margin above safe area for the floating double-back exit snackbar pill.
pub(crate) const EXIT_TOAST_BOTTOM_OFFSET: f32 = 36.0;
