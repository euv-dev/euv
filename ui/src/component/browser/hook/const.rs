/// The default message used by console.log when the input is empty.
pub(crate) const CONSOLE_LOG_DEFAULT_MESSAGE: &str = "Hello from console.log!";

/// The default message used by console.warn when the input is empty.
pub(crate) const CONSOLE_WARN_DEFAULT_MESSAGE: &str = "Warning from console.warn!";

/// The default message used by console.error when the input is empty.
pub(crate) const CONSOLE_ERROR_DEFAULT_MESSAGE: &str = "Error from console.error!";

/// The JS property name exposing `navigator.clipboard`.
pub(crate) const BROWSER_NAVIGATOR_CLIPBOARD_KEY: &str = "clipboard";

/// The message returned when a clipboard read yields a non-string value.
pub(crate) const BROWSER_CLIPBOARD_NO_TEXT_CONTENT: &str = "No text content";

/// The message returned when a clipboard read promise rejects.
pub(crate) const BROWSER_CLIPBOARD_READ_FAILED: &str = "Failed to read clipboard";

/// The message shown when the Clipboard API is missing or blocked.
///
/// A non-secure context (or a browser without `navigator.clipboard`) fails
/// this probe, and the one remediation is the same in both cases.
pub(crate) const BROWSER_CLIPBOARD_UNAVAILABLE: &str =
    "Clipboard API not available (requires secure context)";

/// The placeholder used wherever a navigator or location read yields `None`.
pub(crate) const BROWSER_VALUE_UNKNOWN: &str = "Unknown";

/// The copy result shown when the text box is empty at copy time.
pub(crate) const BROWSER_COPY_EMPTY_TEXT: &str = "Please enter text to copy";

/// The copy result shown after a successful clipboard write.
pub(crate) const BROWSER_COPY_SUCCEEDED: &str = "Copied to clipboard!";

/// The copy result shown after a clipboard write rejects or fails.
pub(crate) const BROWSER_COPY_FAILED: &str = "Failed to copy";
