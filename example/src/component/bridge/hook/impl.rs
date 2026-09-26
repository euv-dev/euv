use super::*;

/// Hand-written `Deserialize` for `UpdateStatus` so the wire-tag mapping is
/// expressed as Rust control flow against the canonical constants in
/// `const.rs` — that constant table is now the only place the literal
/// `"success"` / `"failed"` strings live. `Display` reads the same constants
/// out, so both the parse path and the log-formatting path point at one
/// source of truth.
///
/// `serde_wasm_bindgen` invokes this via the inner `UpdateResultPayload`
/// struct field `result: UpdateStatus` (defined in `struct.rs`), so the
/// visitor has to drive a real `Deserializer` rather than just match a
/// `String`.
impl<'de> Deserialize<'de> for UpdateStatus {
    /// Deserializes from the supplied reader.
    ///
    /// # Arguments
    ///
    /// - `Reader` - Reader to read from.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Reader::Error>` - `Ok(Self)` on success, or the reader's error.
    fn deserialize<Reader>(reader: Reader) -> Result<Self, Reader::Error>
    where
        Reader: serde::Deserializer<'de>,
    {
        struct TagVisitor;

        impl<'de> serde::de::Visitor<'de> for TagVisitor {
            type Value = UpdateStatus;

            /// Writes the expected value description into the formatter.
            ///
            /// # Arguments
            ///
            /// - `&mut Formatter<'_>` - The formatter receiving the expected-value description.
            ///
            /// # Returns
            ///
            /// - `fmt::Result` - Result of the formatting operation.
            fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str(&format!(
                    "an `UpdateStatus` wire tag ({UPDATE_RESULT_SUCCESS:?} / {UPDATE_RESULT_FAILED:?})"
                ))
            }

            /// Deserializes a borrowed string slice into `Self::Value`.
            ///
            /// # Arguments
            ///
            /// - `&str` - Borrowed string slice to deserialise.
            ///
            /// # Returns
            ///
            /// - `Result<Self::Value, Error>` - `Result<Self::Value, Error>` from the deserialiser.
            fn visit_str<Error>(self, value: &str) -> Result<Self::Value, Error>
            where
                Error: serde::de::Error,
            {
                if value == UPDATE_RESULT_SUCCESS {
                    Ok(UpdateStatus::Success)
                } else if value == UPDATE_RESULT_FAILED {
                    Ok(UpdateStatus::Failed)
                } else {
                    Err(Error::unknown_variant(
                        value,
                        &[UPDATE_RESULT_SUCCESS, UPDATE_RESULT_FAILED],
                    ))
                }
            }
        }

        reader.deserialize_str(TagVisitor)
    }
}

/// Formatting / debug-printing for [`UpdateStatus`].
impl Display for UpdateStatus {
    /// Formats the [`UpdateStatus`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let tag: &'static str = match self {
            UpdateStatus::Success => UPDATE_RESULT_SUCCESS,
            UpdateStatus::Failed => UPDATE_RESULT_FAILED,
        };
        formatter.write_str(tag)
    }
}

/// Default implementation for `BridgeConfig`.
///
/// Uses the standard euv-app bridge keys: `window.bridge.core.invoke`.
impl Default for BridgeConfig {
    /// Creates a `BridgeConfig` with the default euv-app bridge keys.
    ///
    /// # Returns
    ///
    /// - `BridgeConfig` - The default bridge configuration.
    fn default() -> Self {
        BridgeConfig::new(
            BRIDGE_DEFAULT_GLOBAL_KEY,
            BRIDGE_DEFAULT_CORE_KEY,
            BRIDGE_DEFAULT_INVOKE_KEY,
        )
    }
}

/// Implementation of native bridge functionality.
impl UseEuvNativeBridge {
    /// Creates native bridge state for accessing platform-native features.
    ///
    /// Initializes `available` to `false` and `permissions` to an empty string.
    /// `loading` starts as `true` because the first `load_data` invocation
    /// will not return synchronously; the actual data is loaded asynchronously
    /// via `Self::load_data`.
    ///
    /// # Returns
    ///
    /// - `UseEuvNativeBridge` - The native bridge state.
    pub(crate) fn use_bridge_state() -> UseEuvNativeBridge {
        UseEuvNativeBridge::new(
            App::use_signal(|| false),
            App::use_signal(|| true),
            App::use_signal(String::new),
        )
    }

    /// Asynchronously loads native bridge data and updates the provided state signals.
    ///
    /// First checks platform availability via `BridgeConfig::is_available`. If unavailable,
    /// sets `available` to `false` and `loading` to `false` (no further fetches will be
    /// attempted for this hook instance) and returns. Otherwise, invokes the
    /// `resolve_bridge_group_permissions` command, then populates the corresponding
    /// signal from the result. If the invoke fails, sets `available` to `false`
    /// so the card is hidden; in either success or failure path, `loading` is
    /// flipped to `false` once the work is done.
    ///
    /// # Arguments
    ///
    /// - `Option<BridgeConfig>` - Optional custom bridge configuration.
    pub(crate) fn load_data(self, config: Option<BridgeConfig>) {
        if !BridgeConfig::is_available(config.as_ref()) {
            self.get_available().set(false);
            self.get_loading().set(false);
            return;
        }
        let permissions_state: UseEuvNativeBridge = self;
        spawn_local(async move {
            let args_obj: Object = Object::new();
            Reflect::set(
                &args_obj,
                &JsValue::from_str(BRIDGE_GROUP_KEY),
                &JsValue::from_str(BRIDGE_GROUP_ALL),
            )
            .unwrap_or_default();
            let permissions_result: Result<JsValue, String> = match BridgeConfig::invoke(
                INVOKE_RESOLVE_BRIDGE_GROUP_PERMISSIONS,
                Some(&args_obj),
                config.as_ref(),
            ) {
                Ok(promise) => {
                    let future: JsFuture = JsFuture::from(promise);
                    match future.await {
                        Ok(value) => Ok(value),
                        Err(error) => Err(format!("{error:?}")),
                    }
                }
                Err(error) => Err(error),
            };
            match permissions_result {
                Ok(value) => {
                    let permissions_array: Vec<String> = value
                        .dyn_into::<Array>()
                        .map(|array: Array| {
                            array
                                .iter()
                                .filter_map(|item: JsValue| item.as_string())
                                .collect::<Vec<String>>()
                        })
                        .unwrap_or_default();
                    permissions_state
                        .get_permissions()
                        .set(permissions_array.join(", "));
                    permissions_state.get_available().set(true);
                }
                Err(_) => {
                    permissions_state.get_available().set(false);
                }
            }
            permissions_state.get_loading().set(false);
        });
    }
}

/// Implementation of cache update functionality.
impl UseCacheUpdate {
    /// Creates cache update state for tracking documentation build status.
    ///
    /// Initializes `doc_status` to `false`, `version` to an empty string,
    /// `updating` to `false`, and `data` / `message` to empty strings.
    /// The actual data is loaded asynchronously via `Self::load`.
    ///
    /// # Returns
    ///
    /// - `UseCacheUpdate` - The cache update state.
    pub(crate) fn use_cache_state() -> UseCacheUpdate {
        UseCacheUpdate::new(
            App::use_signal(|| false),
            App::use_signal(String::new),
            App::use_signal(|| false),
            App::use_signal(String::new),
            App::use_signal(String::new),
        )
    }

    /// Runs the provided updater closure and applies its result to the
    /// internal state signals.
    ///
    /// The closure is called asynchronously via `spawn_local`. It receives
    /// no arguments and must return a `Future<Output = UpdateResult>`.
    /// Once it resolves, every field on the returned `UpdateResult`
    /// (`doc_status`, `version`, `updating`, `data`, `message`) is
    /// pushed into the matching signal — the closure owns the entire
    /// shape of what the UI sees, so this loop stays a one-to-one mirror.
    ///
    /// The UI layer is completely unaware of how the update check is
    /// performed (fetch, version comparison, bridge invocation, etc.) —
    /// it only sees the result.
    ///
    /// # Arguments
    ///
    /// - `F: FnOnce() -> Fut + 'static` - An async closure that returns `UpdateResult`.
    pub(crate) fn load<F, Fut>(self, updater: F)
    where
        F: FnOnce() -> Fut + 'static,
        Fut: Future<Output = UpdateResult>,
    {
        spawn_local(async move {
            let result: UpdateResult = updater().await;
            self.get_doc_status().set(result.get_doc_status());
            self.get_version().set(result.get_version().clone());
            self.get_updating().set(result.get_updating());
            self.get_data().set(result.get_data().clone());
            self.get_message().set(result.get_message().clone());
        });
    }
}

/// Implementation of bridge configuration and invocation.
impl BridgeConfig {
    /// Checks whether the bridge native bridge is available on the current platform.
    ///
    /// Looks up `window.___.core` via `Reflect` to determine if the
    /// bridge runtime is present. Returns `false` if the property chain does not exist
    /// or if any reflection error occurs.
    ///
    /// # Arguments
    ///
    /// - `Option<&BridgeConfig>` - Optional custom bridge configuration.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the bridge core module is available.
    pub(crate) fn is_available(config: Option<&BridgeConfig>) -> bool {
        let config: BridgeConfig = config.cloned().unwrap_or_default();
        let Some(window_value): Option<Window> = window() else {
            return false;
        };
        let bridge_key: JsValue = JsValue::from_str(config.get_global_key());
        let bridge_obj: JsValue = match Reflect::get(&window_value, &bridge_key) {
            Ok(value) => value,
            Err(_) => return false,
        };
        if bridge_obj.is_undefined() || bridge_obj.is_null() {
            return false;
        }
        let core_key: JsValue = JsValue::from_str(config.get_core_key());
        let core_obj: JsValue = match Reflect::get(&bridge_obj, &core_key) {
            Ok(value) => value,
            Err(_) => return false,
        };
        !core_obj.is_undefined() && !core_obj.is_null()
    }

    /// Invokes a bridge core command by name via `window.___.core.invoke`.
    ///
    /// Resolves the `___` → `core` → `invoke` property chain on the global
    /// `window` object, then calls `invoke` with the given command name and
    /// optional arguments object. Returns the resulting `Promise`, or an
    /// error string if any step in the reflection chain fails.
    ///
    /// # Arguments
    ///
    /// - `&str` - The bridge command name to invoke.
    /// - `Option<&JsValue>` - Optional arguments object to pass to the command.
    /// - `Option<&BridgeConfig>` - Optional custom bridge configuration.
    ///
    /// # Returns
    ///
    /// - `Result<Promise, String>` - The promise returned by the invoke call, or an error message.
    pub(crate) fn invoke(
        command: &str,
        args: Option<&JsValue>,
        config: Option<&BridgeConfig>,
    ) -> Result<Promise, String> {
        let config: BridgeConfig = config.cloned().unwrap_or_default();
        let Some(window_value): Option<Window> = window() else {
            return Err("no global window exists".to_string());
        };
        let bridge_key: JsValue = JsValue::from_str(config.get_global_key());
        let bridge_obj: JsValue = Reflect::get(&window_value, &bridge_key)
            .map_err(|error: JsValue| format!("{error:?}"))?;
        let core_key: JsValue = JsValue::from_str(config.get_core_key());
        let core_obj: JsValue =
            Reflect::get(&bridge_obj, &core_key).map_err(|error: JsValue| format!("{error:?}"))?;
        let invoke_key: JsValue = JsValue::from_str(config.get_invoke_key());
        let invoke_fn: JsValue =
            Reflect::get(&core_obj, &invoke_key).map_err(|error: JsValue| format!("{error:?}"))?;
        let invoke_function: Function = invoke_fn
            .dyn_into::<Function>()
            .map_err(|error: JsValue| format!("{error:?}"))?;
        let command_value: JsValue = JsValue::from_str(command);
        let result: JsValue = match args {
            Some(arguments) => invoke_function
                .call2(&core_obj, &command_value, arguments)
                .map_err(|error: JsValue| format!("{error:?}"))?,
            None => invoke_function
                .call1(&core_obj, &command_value)
                .map_err(|error: JsValue| format!("{error:?}"))?,
        };
        result
            .dyn_into::<Promise>()
            .map_err(|error: JsValue| format!("{error:?}"))
    }
}
