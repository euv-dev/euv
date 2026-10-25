/// Identifies the observed Enter-event count.
#[cfg(target_arch = "wasm32")]
pub(crate) const KEYS_ID: &str = "key-count";

/// Identifies the modal-opening button wrapper.
#[cfg(target_arch = "wasm32")]
pub(crate) const OPEN_ID: &str = "dialog-open";

/// Identifies the scrollable fixture root.
#[cfg(target_arch = "wasm32")]
pub(crate) const ROOT_ID: &str = "automation-root";

/// Identifies the signal-driven input echo.
#[cfg(target_arch = "wasm32")]
pub(crate) const TEXT_ID: &str = "text-value";

/// Identifies the reactive counter text.
#[cfg(target_arch = "wasm32")]
pub(crate) const COUNT_ID: &str = "counter-value";

/// Stable identifier of the euv text input.
#[cfg(target_arch = "wasm32")]
pub(crate) const INPUT_ID: &str = "text-input";

/// Identifies the successful WASM mount marker.
#[cfg(target_arch = "wasm32")]
pub(crate) const READY_ID: &str = "automation-ready";

/// Identifies the permanently mounted modal subtree.
#[cfg(target_arch = "wasm32")]
pub(crate) const DIALOG_ID: &str = "dialog";

/// Browser keyboard-event name for Enter.
#[cfg(target_arch = "wasm32")]
pub(crate) const ENTER_KEY: &str = "Enter";

/// Identifies the reactive scroll-position text.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_ID: &str = "scroll-position";

/// Identifies the reactive checkbox text.
#[cfg(target_arch = "wasm32")]
pub(crate) const CHECKED_ID: &str = "checked-value";

/// Label of the modal-opening button.
#[cfg(target_arch = "wasm32")]
pub(crate) const OPEN_LABEL: &str = "Open dialog";

/// Text emitted only by the mounted euv fixture.
pub(crate) const READY_TEXT: &str = "ready";

/// Identifies the trusted-event indicator.
#[cfg(target_arch = "wasm32")]
pub(crate) const TRUSTED_ID: &str = "trusted-event";

/// Stable identifier of the euv checkbox.
pub(crate) const CHECKBOX_ID: &str = "checked-input";

/// Content verified after opening the modal.
pub(crate) const DIALOG_TEXT: &str = "Dialog is open";

/// Label associated with the euv text input.
#[cfg(target_arch = "wasm32")]
pub(crate) const INPUT_LABEL: &str = "Type a value";

/// Section title for editable controls.
#[cfg(target_arch = "wasm32")]
pub(crate) const INPUT_TITLE: &str = "Input and keyboard";

/// Title of the modal test section and dialog.
#[cfg(target_arch = "wasm32")]
pub(crate) const MODAL_TITLE: &str = "Modal interaction";

/// Identifies the wheel-scroll target.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLLER_ID: &str = "scroll-panel";

/// Mount selector for the browser fixture.
#[cfg(target_arch = "wasm32")]
pub(crate) const APP_SELECTOR: &str = "#app";

/// Identifies the increment-button wrapper.
#[cfg(target_arch = "wasm32")]
pub(crate) const INCREMENT_ID: &str = "increment";

/// Section title for native wheel interaction.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_TITLE: &str = "Scroll interaction";

/// Section title for counter interactions.
#[cfg(target_arch = "wasm32")]
pub(crate) const COUNTER_TITLE: &str = "Reactive counter";

/// Title of the browser fixture.
pub(crate) const FIXTURE_TITLE: &str = "euv UI automation";

/// Locates the observed Enter-event count.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const KEYS_SELECTOR: &str = "#key-count";

/// Locates the modal-opening euv button.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const OPEN_SELECTOR: &str = "#dialog-open button";

/// Locates the signal-driven input echo.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const TEXT_SELECTOR: &str = "#text-value";

/// Label associated with the euv checkbox.
#[cfg(target_arch = "wasm32")]
pub(crate) const CHECKBOX_LABEL: &str = "Enable option";

/// Locates the reactive counter text.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const COUNT_SELECTOR: &str = "#counter-value";

/// Locates the euv text input.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const INPUT_SELECTOR: &str = "#text-input";

/// Locates the successful WASM mount marker.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const READY_SELECTOR: &str = "#automation-ready";

/// Text in the oversized scrollable fixture body.
#[cfg(target_arch = "wasm32")]
pub(crate) const SCROLL_CONTENT: &str = "Scrollable component content";

/// Label of the euv increment button.
#[cfg(target_arch = "wasm32")]
pub(crate) const INCREMENT_LABEL: &str = "Increment";

/// Locates the reactive scroll-position text.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const SCROLL_SELECTOR: &str = "#scroll-position";

/// Locates the reactive checkbox text.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const CHECKED_SELECTOR: &str = "#checked-value";

/// Locates the trusted-event indicator.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const TRUSTED_SELECTOR: &str = "#trusted-event";

/// Locates the euv checkbox.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const CHECKBOX_SELECTOR: &str = "#checked-input";

/// Identifies the modal content observation point.
#[cfg(target_arch = "wasm32")]
pub(crate) const DIALOG_CONTENT_ID: &str = "dialog-content";

/// Locates the wheel-scroll target.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const SCROLLER_SELECTOR: &str = "#scroll-panel";

/// Locates the real euv increment button.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const INCREMENT_SELECTOR: &str = "#increment button";

/// Locates the real modal close button.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const DIALOG_CLOSE_SELECTOR: &str = "#dialog button";

/// Locates the modal content observation point.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const DIALOG_CONTENT_SELECTOR: &str = "#dialog-content";
