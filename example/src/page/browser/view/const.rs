/// The HTML id for the localStorage key input element.
pub(crate) const LOCAL_STORAGE_KEY_ID: &str = "local-storage-key";

/// The HTML id for the localStorage value input element.
pub(crate) const LOCAL_STORAGE_VALUE_ID: &str = "local-storage-value";

/// The HTML id for the sessionStorage key input element.
pub(crate) const SESSION_STORAGE_KEY_ID: &str = "session-storage-key";

/// The HTML id for the sessionStorage value input element.
pub(crate) const SESSION_STORAGE_VALUE_ID: &str = "session-storage-value";

/// The HTML id for the clipboard text input element.
pub(crate) const CLIPBOARD_TEXT_ID: &str = "clipboard-text";

/// The HTML id for the console message input element.
pub(crate) const CONSOLE_MESSAGE_ID: &str = "console-message";

/// The HTML name attribute for the localStorage key input element.
pub(crate) const LOCAL_STORAGE_KEY_NAME: &str = "local_key";

/// The HTML name attribute for the localStorage value input element.
pub(crate) const LOCAL_STORAGE_VALUE_NAME: &str = "local_value";

/// The HTML name attribute for the sessionStorage key input element.
pub(crate) const SESSION_STORAGE_KEY_NAME: &str = "session_key";

/// The HTML name attribute for the sessionStorage value input element.
pub(crate) const SESSION_STORAGE_VALUE_NAME: &str = "session_value";

/// The HTML name attribute for the clipboard text input element.
pub(crate) const CLIPBOARD_TEXT_NAME: &str = "clipboard_text";

/// The HTML name attribute for the console message input element.
pub(crate) const CONSOLE_MESSAGE_NAME: &str = "console_message";

/// The HTML input type for text.
pub(crate) const BROWSER_TEXT_TYPE: &str = "text";

/// The HTML autocomplete attribute value for off.
pub(crate) const BROWSER_AUTOCOMPLETE_OFF: &str = "off";

/// The HTML placeholder for the localStorage key input element.
pub(crate) const LOCAL_STORAGE_KEY_PLACEHOLDER: &str = "Storage key...";

/// The HTML placeholder for the localStorage value input element.
pub(crate) const LOCAL_STORAGE_VALUE_PLACEHOLDER: &str = "Storage value...";

/// The HTML placeholder for the sessionStorage key input element.
pub(crate) const SESSION_STORAGE_KEY_PLACEHOLDER: &str = "Session key...";

/// The HTML placeholder for the sessionStorage value input element.
pub(crate) const SESSION_STORAGE_VALUE_PLACEHOLDER: &str = "Session value...";

/// The HTML placeholder for the clipboard text input element.
pub(crate) const CLIPBOARD_TEXT_PLACEHOLDER: &str = "Enter text to copy...";

/// The HTML placeholder for the console message input element.
pub(crate) const CONSOLE_MESSAGE_PLACEHOLDER: &str = "Type a message to log...";

/// The label of the console error button.
pub(crate) const BROWSER_ERROR_BUTTON_LABEL: &str = "Error";

/// The label of the console warn button.
pub(crate) const BROWSER_WARN_BUTTON_LABEL: &str = "Warn";

/// The label of the console message input.
pub(crate) const BROWSER_CONSOLE_MESSAGE_LABEL: &str = "Console message";

/// The description of the developer console demo.
pub(crate) const BROWSER_CONSOLE_CARD_DESC: &str = "Send log, warning, and error messages to the browser developer console. Open DevTools (F12) to see the output.";

/// The card title of the developer console demo.
pub(crate) const BROWSER_CONSOLE_CARD_TITLE: &str = "Console";

/// The label of the location pathname readout.
pub(crate) const BROWSER_PATHNAME_LABEL: &str = "Pathname";

/// The label of the location origin readout.
pub(crate) const BROWSER_ORIGIN_LABEL: &str = "Origin";

/// The label of the location href readout.
pub(crate) const BROWSER_HREF_LABEL: &str = "Href";

/// The description of the location demo.
pub(crate) const BROWSER_LOCATION_CARD_DESC: &str = "Read the current page's full URL components: href, origin, and pathname. All values are read-only and update automatically on navigation.";

/// The card title of the location demo.
pub(crate) const BROWSER_LOCATION_CARD_TITLE: &str = "Location";

/// The label of the preferred language readout.
pub(crate) const BROWSER_LANGUAGE_LABEL: &str = "Language";

/// The label of the user agent readout.
pub(crate) const BROWSER_USER_AGENT_LABEL: &str = "User Agent";

/// The description of the navigator demo.
pub(crate) const BROWSER_NAVIGATOR_CARD_DESC: &str = "Read the browser's User-Agent string and preferred language. Useful for analytics, feature detection, and localization.";

/// The card title of the navigator demo.
pub(crate) const BROWSER_NAVIGATOR_CARD_TITLE: &str = "Navigator";

/// The label of the inner size readout.
pub(crate) const BROWSER_INNER_SIZE_LABEL: &str = "Inner Size";

/// The label of the window size refresh button.
pub(crate) const BROWSER_REFRESH_SIZE_BUTTON_LABEL: &str = "Refresh Size";

/// The description of the window metrics demo.
pub(crate) const BROWSER_WINDOW_CARD_DESC: &str = "Read the browser window's inner width and height in CSS pixels. Click Refresh Size after resizing the window.";

/// The card title of the window metrics demo.
pub(crate) const BROWSER_WINDOW_CARD_TITLE: &str = "Window";

/// The prefix shown before a browser API result value.
pub(crate) const BROWSER_RESULT_PREFIX: &str = "Result: ";

/// The label of the paste-from-clipboard button.
pub(crate) const BROWSER_PASTE_BUTTON_LABEL: &str = "Paste";

/// The label of the copy-to-clipboard button.
pub(crate) const BROWSER_COPY_BUTTON_LABEL: &str = "Copy";

/// The label of the clipboard text input.
pub(crate) const BROWSER_CLIPBOARD_TEXT_LABEL: &str = "Text to copy";

/// The description of the clipboard demo.
pub(crate) const BROWSER_CLIPBOARD_CARD_DESC: &str = "Write text to the system clipboard or read the current clipboard contents. Requires a secure context (HTTPS or localhost).";

/// The card title of the clipboard demo.
pub(crate) const BROWSER_CLIPBOARD_CARD_TITLE: &str = "Clipboard API";

/// The label of the remove entry button.
pub(crate) const BROWSER_REMOVE_BUTTON_LABEL: &str = "Remove";

/// The label of the key-value value input.
pub(crate) const BROWSER_VALUE_LABEL: &str = "Value";

/// The description of the sessionStorage demo.
pub(crate) const BROWSER_SESSION_STORAGE_CARD_DESC: &str = "Store key-value data for the duration of the page session. Data is cleared when the tab or window is closed.";

/// The card title of the sessionStorage demo.
pub(crate) const BROWSER_SESSION_STORAGE_CARD_TITLE: &str = "sessionStorage";

/// The description of the localStorage demo.
pub(crate) const BROWSER_LOCAL_STORAGE_CARD_DESC: &str = "Store, retrieve, and remove persistent key-value data. Data in localStorage survives page reloads and browser restarts.";

/// The card title of the localStorage demo.
pub(crate) const BROWSER_LOCAL_STORAGE_CARD_TITLE: &str = "localStorage";

/// The page header subtitle of the browser API demo.
pub(crate) const BROWSER_PAGE_SUBTITLE: &str = "Interact with browser storage, clipboard, window metrics, navigator info, location URL, and developer console — all through euv's typed hook APIs.";

/// The page header title of the browser API demo.
pub(crate) const BROWSER_PAGE_TITLE: &str = "Browser APIs";
