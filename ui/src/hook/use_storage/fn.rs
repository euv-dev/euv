use super::*;

/// Obtains the [`UseStorage`] registered against the current hook context
/// slot.
///
/// Behaves like `HookContext::use_hook` - the same handle is returned on
/// every render at the same hook index, so the stored value survives
/// re-renders.
///
/// The handle is seeded with the value already in storage, and falls
/// back to `initial` when the key is not there yet. `initial` is
/// *not* written to storage: the first write belongs to the caller, so
/// a page that only reads a preference does not leave one behind.
///
/// # Arguments
///
/// - `&'static str` - The storage key.
/// - `UseStorageKind` - Which storage to use.
/// - `&str` - The storage key.
///
/// # Returns
///
/// - `UseStorage` - The handle, with `get_value()` returning the
///   reactive `Signal<String>`.
pub fn use_storage(key: &'static str, kind: UseStorageKind, initial: &str) -> UseStorage {
    HookContext::use_hook(|| UseStorage::new(key, kind, initial))
}

/// Reads a key out of the chosen storage.
///
/// # Arguments
///
/// - `UseStorageKind` - Which storage to read from.
/// - `&str` - The key to read.
///
/// # Returns
///
/// - `Option<String>` - The stored value, when the key is present and
///   the host has the storage at all.
pub(crate) fn read_storage(kind: UseStorageKind, key: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window: Window = browser_window()?;
        let storage: Storage = match kind {
            UseStorageKind::Local => window.local_storage().ok()??,
            UseStorageKind::Session => window.session_storage().ok()??,
        };
        storage.get_item(key).ok()?
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _: (UseStorageKind, &str) = (kind, key);
        None
    }
}

/// Writes a key into the chosen storage.
///
/// # Arguments
///
/// - `UseStorageKind` - Which storage to write to.
/// - `&str` - The key to write.
/// - `&str` - The value to write.
pub(crate) fn write_storage(kind: UseStorageKind, key: &str, value: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(window) = browser_window() else {
            return;
        };
        let storage: Storage = match kind {
            UseStorageKind::Local => match window.local_storage() {
                Ok(Some(storage)) => storage,
                _ => return,
            },
            UseStorageKind::Session => match window.session_storage() {
                Ok(Some(storage)) => storage,
                _ => return,
            },
        };
        let _: Result<(), JsValue> = storage.set_item(key, value);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _: (UseStorageKind, &str, &str) = (kind, key, value);
    }
}

/// Removes a key from the chosen storage.
///
/// # Arguments
///
/// - `UseStorageKind` - Which storage to remove from.
/// - `&str` - The key to remove.
pub(crate) fn remove_storage(kind: UseStorageKind, key: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(window) = browser_window() else {
            return;
        };
        let storage: Storage = match kind {
            UseStorageKind::Local => match window.local_storage() {
                Ok(Some(storage)) => storage,
                _ => return,
            },
            UseStorageKind::Session => match window.session_storage() {
                Ok(Some(storage)) => storage,
                _ => return,
            },
        };
        let _: Result<(), JsValue> = storage.remove_item(key);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _: (UseStorageKind, &str) = (kind, key);
    }
}
