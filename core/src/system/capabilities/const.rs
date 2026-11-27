/// Native action kind for tactile haptic feedback.
pub const NATIVE_ACTION_HAPTIC: &str = "haptic";
/// Native action kind for OS notifications.
pub const NATIVE_ACTION_NOTIFY: &str = "notify";
/// Native action kind for Dynamic Island / Live Activity updates.
pub const NATIVE_ACTION_ISLAND: &str = "island";
/// Native action kind for OS runtime permission prompts.
pub const NATIVE_ACTION_PERMISSION: &str = "permission";
/// Native action kind for immersive fullscreen toggling.
pub const NATIVE_ACTION_FULLSCREEN: &str = "fullscreen";
/// Native action kind for custom URI protocol registration confirmation.
pub const NATIVE_ACTION_PROTOCOL: &str = "protocol";
/// Native action kind for OS soft-keyboard / IME visibility synchronization.
pub const NATIVE_ACTION_IME: &str = "ime";
/// Native action kind for displaying a brief bottom system toast / snackbar prompt.
pub const NATIVE_ACTION_TOAST: &str = "toast";
/// Native action kind for requesting graceful OS application termination (e.g. Android `Activity.finish()`).
pub const NATIVE_ACTION_EXIT: &str = "exit";
/// Native action kind for reading or writing the host OS system clipboard.
pub const NATIVE_ACTION_CLIPBOARD: &str = "clipboard";

/// Native clipboard action verb for writing UTF-8 text to the host system clipboard.
pub(crate) const CLIPBOARD_ACTION_WRITE: &str = "write";
