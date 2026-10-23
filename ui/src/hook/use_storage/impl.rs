use super::*;

/// Inherent implementation of [`UseStorage`].
impl UseStorage {
    /// Creates a handle seeded with the stored value, falling back to
    /// `initial` when the key is not in storage yet.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The storage key.
    /// - `UseStorageKind` - Which storage to read from.
    /// - `&str` - The storage key.
    ///
    /// # Returns
    ///
    /// - `UseStorage` - The fresh handle.
    pub(crate) fn new(key: &'static str, kind: UseStorageKind, initial: &str) -> Self {
        let stored: Option<String> = read_storage(kind, key);
        let value: String = stored.unwrap_or_else(|| initial.to_owned());
        Self {
            key,
            value: Signal::create(value),
            kind,
        }
    }

    /// Returns the current value as a snapshot.
    ///
    /// # Returns
    ///
    /// - `String` - The stored value.
    pub fn current(&self) -> String {
        self.get_value().get()
    }

    /// Writes `value` to storage and updates the signal.
    ///
    /// # Arguments
    ///
    /// - `&str` - The value to store.
    pub fn set_value(&self, value: &str) {
        write_storage(self.get_kind(), self.get_key(), value);
        self.get_value().set(value.to_owned());
    }

    /// Removes the key from storage and clears the signal.
    pub fn clear(&self) {
        remove_storage(self.get_kind(), self.get_key());
        self.get_value().set(String::new());
    }
}
