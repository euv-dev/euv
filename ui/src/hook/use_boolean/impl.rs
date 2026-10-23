use super::*;

/// Inherent implementation of [`UseBoolean`].
impl UseBoolean {
    /// Sets the value to `true`.
    pub fn set_true(&self) {
        self.get_value().set(true);
    }

    /// Sets the value to `false`.
    pub fn set_false(&self) {
        self.get_value().set(false);
    }

    /// Flips the value: `true` becomes `false`, `false` becomes `true`.
    pub fn toggle(&self) {
        let current: bool = self.get_value().get();
        self.get_value().set(!current);
    }

    /// Replaces the value with `next`.
    ///
    /// # Arguments
    ///
    /// - `bool` - The new value.
    pub fn set_value(&self, next: bool) {
        self.get_value().set(next);
    }

    /// Returns the current value as a snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - The current value.
    pub fn get(&self) -> bool {
        self.get_value().get()
    }
}
