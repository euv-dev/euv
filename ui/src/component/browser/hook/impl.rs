use super::*;

/// Implementation of browser API functionality.
impl UseEuvBrowser {
    /// Creates browser API state for localStorage, sessionStorage, clipboard, and navigator access.
    ///
    /// # Returns
    ///
    /// - `UseEuvBrowser` - The browser API state.
    pub fn use_browser_state() -> UseEuvBrowser {
        UseEuvBrowser::default()
    }

    /// Reads a value from the browser localStorage.
    ///
    /// # Arguments
    ///
    /// - `K` - The key to look up.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The stored value if found, or None.
    pub fn local_storage_get<K>(key: K) -> Option<String>
    where
        K: AsRef<str>,
    {
        let window: Window = window()?;
        let storage: Storage = window.local_storage().ok()??;
        storage.get_item(key.as_ref()).ok()?
    }

    /// Writes a key-value pair to the browser localStorage.
    ///
    /// # Arguments
    ///
    /// - `K` - The key to store.
    /// - `V` - The value to store.
    pub fn local_storage_set<K, V>(key: K, value: V)
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let Some(window) = window() else {
            return;
        };
        let storage: Storage = match window.local_storage() {
            Ok(Some(local_storage)) => local_storage,
            _ => return,
        };
        let _: Result<(), JsValue> = storage.set_item(key.as_ref(), value.as_ref());
    }

    /// Removes a key from the browser localStorage.
    ///
    /// # Arguments
    ///
    /// - `K` - The key to remove.
    pub(crate) fn local_storage_remove<K>(key: K)
    where
        K: AsRef<str>,
    {
        let Some(window) = window() else {
            return;
        };
        let storage: Storage = match window.local_storage() {
            Ok(Some(local_storage)) => local_storage,
            _ => return,
        };
        let _: Result<(), JsValue> = storage.remove_item(key.as_ref());
    }

    /// Reads a value from the browser sessionStorage.
    ///
    /// # Arguments
    ///
    /// - `K` - The key to look up.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The stored value if found, or None.
    pub(crate) fn session_storage_get<K>(key: K) -> Option<String>
    where
        K: AsRef<str>,
    {
        let window: Window = window()?;
        let storage: Storage = window.session_storage().ok()??;
        storage.get_item(key.as_ref()).ok()?
    }

    /// Writes a key-value pair to the browser sessionStorage.
    ///
    /// # Arguments
    ///
    /// - `K` - The key to store.
    /// - `V` - The value to store.
    pub(crate) fn session_storage_set<K, V>(key: K, value: V)
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let Some(window) = window() else {
            return;
        };
        let storage: Storage = match window.session_storage() {
            Ok(Some(session_storage)) => session_storage,
            _ => return,
        };
        let _: Result<(), JsValue> = storage.set_item(key.as_ref(), value.as_ref());
    }

    /// Removes a key from the browser sessionStorage.
    ///
    /// # Arguments
    ///
    /// - `K` - The key to remove.
    pub(crate) fn session_storage_remove<K>(key: K)
    where
        K: AsRef<str>,
    {
        let Some(window) = window() else {
            return;
        };
        let storage: Storage = match window.session_storage() {
            Ok(Some(session_storage)) => session_storage,
            _ => return,
        };
        let _: Result<(), JsValue> = storage.remove_item(key.as_ref());
    }

    /// Reads text from the system clipboard asynchronously.
    ///
    /// # Returns
    ///
    /// - `String` - The clipboard text content, or an error message.
    pub(crate) async fn clipboard_read_text() -> String {
        let Some(window) = window() else {
            return String::new();
        };
        let navigator: Navigator = window.navigator();
        match Reflect::get(
            &navigator,
            &JsValue::from_str(BROWSER_NAVIGATOR_CLIPBOARD_KEY),
        ) {
            Ok(clipboard_obj) if !clipboard_obj.is_undefined() => {
                let clipboard: Clipboard = navigator.clipboard();
                let promise: Promise = clipboard.read_text();
                let future: JsFuture = JsFuture::from(promise);
                match future.await {
                    Ok(value) => value
                        .as_string()
                        .unwrap_or_else(|| BROWSER_CLIPBOARD_NO_TEXT_CONTENT.to_string()),
                    Err(_) => BROWSER_CLIPBOARD_READ_FAILED.to_string(),
                }
            }
            _ => BROWSER_CLIPBOARD_UNAVAILABLE.to_string(),
        }
    }

    /// Writes text to the system clipboard asynchronously.
    ///
    /// # Arguments
    ///
    /// - `T` - The text to write.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the write succeeded.
    pub(crate) async fn clipboard_write_text<T>(text: T) -> bool
    where
        T: AsRef<str>,
    {
        let Some(window) = window() else {
            return false;
        };
        let navigator: Navigator = window.navigator();
        match js_sys::Reflect::get(
            &navigator,
            &JsValue::from_str(BROWSER_NAVIGATOR_CLIPBOARD_KEY),
        ) {
            Ok(clipboard_obj) if !clipboard_obj.is_undefined() => {
                let clipboard: Clipboard = navigator.clipboard();
                let promise: Promise = clipboard.write_text(text.as_ref());
                let future: JsFuture = JsFuture::from(promise);
                future.await.is_ok()
            }
            _ => false,
        }
    }

    /// Reads the browser window inner dimensions.
    ///
    /// # Returns
    ///
    /// - `(i32, i32)` - The viewport's inner width and height in CSS pixels.
    pub(crate) fn window_inner_size() -> (i32, i32) {
        let Some(window) = window() else {
            return (0, 0);
        };
        let width: i32 = window
            .inner_width()
            .ok()
            .map(|value: JsValue| Number::from(value).value_of() as i32)
            .unwrap_or_default();
        let height: i32 = window
            .inner_height()
            .ok()
            .map(|value: JsValue| Number::from(value).value_of() as i32)
            .unwrap_or_default();
        (width, height)
    }

    /// Reads the browser navigator user agent string.
    ///
    /// # Returns
    ///
    /// - `String` - The user agent string.
    pub(crate) fn navigator_user_agent() -> String {
        let Some(window) = window() else {
            return String::new();
        };
        window
            .navigator()
            .user_agent()
            .unwrap_or_else(|_: JsValue| BROWSER_VALUE_UNKNOWN.to_string())
    }

    /// Reads the browser navigator language.
    ///
    /// # Returns
    ///
    /// - `String` - The preferred language string.
    pub(crate) fn navigator_language() -> String {
        let Some(window) = window() else {
            return String::new();
        };
        window
            .navigator()
            .language()
            .unwrap_or_else(|| BROWSER_VALUE_UNKNOWN.to_string())
    }

    /// Reads the current browser location href.
    ///
    /// # Returns
    ///
    /// - `String` - The current full URL.
    pub(crate) fn location_href() -> String {
        let Some(window) = window() else {
            return String::new();
        };
        window
            .location()
            .href()
            .unwrap_or_else(|_error: JsValue| BROWSER_VALUE_UNKNOWN.to_string())
    }

    /// Reads the current browser location origin.
    ///
    /// # Returns
    ///
    /// - `String` - The origin portion of the URL.
    pub(crate) fn location_origin() -> String {
        let Some(window) = window() else {
            return String::new();
        };
        window
            .location()
            .origin()
            .unwrap_or_else(|_error: JsValue| BROWSER_VALUE_UNKNOWN.to_string())
    }

    /// Reads the current browser location pathname.
    ///
    /// # Returns
    ///
    /// - `String` - The pathname portion of the URL.
    pub(crate) fn location_pathname() -> String {
        let Some(window) = window() else {
            return String::new();
        };
        window
            .location()
            .pathname()
            .unwrap_or_else(|_error: JsValue| BROWSER_VALUE_UNKNOWN.to_string())
    }

    /// Creates a click event handler that sets a localStorage item.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler to set the localStorage item.
    pub fn on_local_storage_set(self) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let key: String = self.get_local_key().get();
            let value: String = self.get_local_value().get();
            if !key.is_empty() {
                Self::local_storage_set(&key, &value);
                self.get_local_result().set(format!("Set: {key} = {value}"));
            }
        }))
    }

    /// Creates a click event handler that gets a localStorage item.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler to get the localStorage item.
    pub fn on_local_storage_get(self) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let key: String = self.get_local_key().get();
            let value: Option<String> = Self::local_storage_get(&key);
            match value {
                Some(v) => self.get_local_result().set(format!("Get: {key} = {v}")),
                None => self
                    .get_local_result()
                    .set(format!("Key '{key}' not found")),
            }
        }))
    }

    /// Creates a click event handler that removes a localStorage item.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler to remove the localStorage item.
    pub fn on_local_storage_remove(self) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let key: String = self.get_local_key().get();
            Self::local_storage_remove(&key);
            self.get_local_result().set(format!("Removed key: {key}"));
        }))
    }

    /// Creates a click event handler that sets a sessionStorage item.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler to set the sessionStorage item.
    pub fn on_session_storage_set(self) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let key: String = self.get_session_key().get();
            let value: String = self.get_session_value().get();
            if !key.is_empty() {
                Self::session_storage_set(&key, &value);
                self.get_session_result()
                    .set(format!("Set: {key} = {value}"));
            }
        }))
    }

    /// Creates a click event handler that gets a sessionStorage item.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler to get the sessionStorage item.
    pub fn on_session_storage_get(self) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let key: String = self.get_session_key().get();
            let value: Option<String> = Self::session_storage_get(&key);
            match value {
                Some(v) => self.get_session_result().set(format!("Get: {key} = {v}")),
                None => self
                    .get_session_result()
                    .set(format!("Key '{key}' not found")),
            }
        }))
    }

    /// Creates a click event handler that removes a sessionStorage item.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler to remove the sessionStorage item.
    pub fn on_session_storage_remove(self) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let key: String = self.get_session_key().get();
            Self::session_storage_remove(&key);
            self.get_session_result().set(format!("Removed key: {key}"));
        }))
    }

    /// Creates a click event handler that copies text to clipboard.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler to copy text to clipboard.
    pub fn on_clipboard_copy(self) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let text: String = self.get_clipboard_text().get();
            let text_clone: String = text.clone();
            let result: Signal<String> = self.get_clipboard_result();
            if text.is_empty() {
                result.set(BROWSER_COPY_EMPTY_TEXT.to_string());
                return;
            }
            let Some(window) = window() else {
                return;
            };
            let navigator: Navigator = window.navigator();
            match js_sys::Reflect::get(
                &navigator,
                &JsValue::from_str(BROWSER_NAVIGATOR_CLIPBOARD_KEY),
            ) {
                Ok(clipboard_obj) if !clipboard_obj.is_undefined() => {
                    spawn_local(async move {
                        let success: bool = Self::clipboard_write_text(&text_clone).await;
                        if success {
                            result.set(BROWSER_COPY_SUCCEEDED.to_string());
                        } else {
                            result.set(BROWSER_COPY_FAILED.to_string());
                        }
                    });
                }
                _ => {
                    result.set(BROWSER_CLIPBOARD_UNAVAILABLE.to_string());
                }
            }
        }))
    }

    /// Creates a click event handler that reads text from clipboard.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler to read text from clipboard.
    pub fn on_clipboard_paste(self) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let result: Signal<String> = self.get_clipboard_result();
            let Some(window) = window() else {
                return;
            };
            let navigator: Navigator = window.navigator();
            match js_sys::Reflect::get(
                &navigator,
                &JsValue::from_str(BROWSER_NAVIGATOR_CLIPBOARD_KEY),
            ) {
                Ok(clipboard_obj) if !clipboard_obj.is_undefined() => {
                    spawn_local(async move {
                        let text: String = Self::clipboard_read_text().await;
                        result.set(format!("Pasted: {text}"));
                    });
                }
                _ => {
                    result.set(BROWSER_CLIPBOARD_UNAVAILABLE.to_string());
                }
            }
        }))
    }

    /// Creates a click event handler that refreshes the window size display.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler to refresh the window size.
    pub fn on_window_refresh_size(self) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let (width, height): (i32, i32) = Self::window_inner_size();
            self.get_window_size().set(format!("{width} x {height}"));
        }))
    }

    /// Creates a click event handler that logs a console message.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The console input signal.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler for console.log.
    pub fn on_console_log(console_input: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let raw: String = console_input.get();
            let message: &str = if raw.is_empty() {
                CONSOLE_LOG_DEFAULT_MESSAGE
            } else {
                &raw
            };
            Console::log(message);
        }))
    }

    /// Creates a click event handler that warns a console message.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The console input signal.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler for console.warn.
    pub fn on_console_warn(console_input: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let raw: String = console_input.get();
            let message: &str = if raw.is_empty() {
                CONSOLE_WARN_DEFAULT_MESSAGE
            } else {
                &raw
            };
            Console::warn(message);
        }))
    }

    /// Creates a click event handler that errors a console message.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The console input signal.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click handler for console.error.
    pub fn on_console_error(console_input: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let raw: String = console_input.get();
            let message: &str = if raw.is_empty() {
                CONSOLE_ERROR_DEFAULT_MESSAGE
            } else {
                &raw
            };
            Console::error(message);
        }))
    }
}

/// Default implementation for `UseEuvBrowser`.
impl Default for UseEuvBrowser {
    /// Constructs a default [`UseEuvBrowser`] value.
    fn default() -> Self {
        let window_size_val: String = {
            let (width, height): (i32, i32) = UseEuvBrowser::window_inner_size();
            format!("{width} x {height}")
        };
        UseEuvBrowser {
            local_key: App::use_signal(String::new),
            local_value: App::use_signal(String::new),
            local_result: App::use_signal(String::new),
            session_key: App::use_signal(String::new),
            session_value: App::use_signal(String::new),
            session_result: App::use_signal(String::new),
            clipboard_text: App::use_signal(String::new),
            clipboard_result: App::use_signal(String::new),
            window_size: App::use_signal(move || window_size_val.clone()),
            user_agent: App::use_signal(UseEuvBrowser::navigator_user_agent),
            language: App::use_signal(UseEuvBrowser::navigator_language),
            location_url: App::use_signal(UseEuvBrowser::location_href),
            location_origin_val: App::use_signal(UseEuvBrowser::location_origin),
            location_pathname_val: App::use_signal(UseEuvBrowser::location_pathname),
            console_input: App::use_signal(String::new),
        }
    }
}
