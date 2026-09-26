use super::*;

/// Inherent implementation of [`Toggle`].
impl Toggle {
    /// Sets the value to `true`.
    pub fn set_true(&self) {
        self.get_value().set(true);
    }

    /// Sets the value to `false`.
    pub fn set_false(&self) {
        self.get_value().set(false);
    }

    /// Flips the value: `true` becomes `false`,
    /// `false` becomes `true`.
    pub fn toggle(&self) {
        let current: bool = self.get_value().get();
        self.get_value().set(!current);
    }

    /// Replaces the value with `next`.
    ///
    /// # Arguments
    ///
    /// - `bool` - A boolean (`bool`).
    pub fn set(&self, next: bool) {
        self.get_value().set(next);
    }

    /// Returns the current value as a snapshot.
    ///
    /// # Returns
    ///
    /// - `bool` - The current value (or a snapshot thereof).
    pub fn get(&self) -> bool {
        self.get_value().get()
    }
}

/// Formatting / debug-printing for [`Toggle`].
impl Display for Toggle {
    /// Formats the [`Toggle`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "Toggle({})", self.get_value().get())
    }
}

/// Equality comparison for [`Toggle`].
impl PartialEq for Toggle {
    /// Returns `true` when `self` and `other` are equivalent by the [`PartialEq`] contract.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The other value to compare against `self`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when `self` and `other` are equivalent by the trait contract.
    fn eq(&self, other: &Self) -> bool {
        self.get_value().get() == other.get_value().get()
    }
}
