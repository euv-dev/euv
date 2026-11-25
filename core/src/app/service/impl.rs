use super::*;

/// Key-value storage operations for `StorageStore`.
impl StorageStore {
    /// Inserts or replaces `key` with `value`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable storage store.
    /// - `String` - Entry key.
    /// - `String` - Entry value.
    pub fn set_item(&mut self, key: String, value: String) {
        self.get_mut_entries().insert(key, value);
    }

    /// Reads the stored string for `key` if present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Storage store reference.
    /// - `&str` - Lookup key.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Stored value clone if present.
    #[must_use]
    pub fn get_item(&self, key: &str) -> Option<String> {
        self.get_entries().get(key).cloned()
    }

    /// Removes `key` from storage.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable storage store.
    /// - `&str` - Key to remove.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Removed value if present.
    pub fn remove_item(&mut self, key: &str) -> Option<String> {
        self.get_mut_entries().remove(key)
    }

    /// Returns total number of stored entries.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Storage store reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Entry count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.get_entries().len()
    }

    /// Returns true when storage contains no entries.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Storage store reference.
    ///
    /// # Returns
    ///
    /// - `bool` - True when empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.get_entries().is_empty()
    }
}
