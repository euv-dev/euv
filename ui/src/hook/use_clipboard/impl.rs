use super::*;

/// Inherent implementation of [`UseClipboard`].
impl UseClipboard {
    /// Creates a handle whose status starts at
    /// [`UseClipboardStatus::Idle`].
    ///
    /// # Returns
    ///
    /// - `UseClipboard` - The fresh handle.
    pub(crate) fn new() -> Self {
        Self {
            status: Signal::create(UseClipboardStatus::Idle),
        }
    }

    /// Returns the current status as a snapshot.
    ///
    /// # Returns
    ///
    /// - `UseClipboardStatus` - The outcome of the last write.
    pub fn current_status(&self) -> UseClipboardStatus {
        self.get_status().get()
    }

    /// Copies `text` to the clipboard.
    ///
    /// The write is asynchronous: this call starts it and returns
    /// whether the browser accepted the request. The status settles
    /// once the browser has answered, so a render closure that reads
    /// `get_status()` shows the result.
    ///
    /// # Arguments
    ///
    /// - `&str` - The text to copy.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the write was started.
    pub fn copy(&self, text: &str) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(window) = browser_window() else {
                self.get_status().set(UseClipboardStatus::Unavailable);
                return false;
            };
            let navigator: Navigator = window.navigator();
            let clipboard: Clipboard = navigator.clipboard();
            let status: Signal<UseClipboardStatus> = self.get_status();
            let payload: String = text.to_owned();
            spawn_local(async move {
                match clipboard.write_text(&payload).await {
                    Ok(_) => status.set(UseClipboardStatus::Copied),
                    Err(_) => status.set(UseClipboardStatus::Failed),
                }
            });
            true
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _: &str = text;
            self.get_status().set(UseClipboardStatus::Unavailable);
            false
        }
    }
}
