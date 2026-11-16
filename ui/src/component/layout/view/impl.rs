use super::*;

/// Identity comparison for the immutable, shell-owned configuration handle.
impl PartialEq for LayoutSettings {
    /// Returns `true` when `self` and `other` are equivalent.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The other value to compare against `self`.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self.get_props(), other.get_props())
    }
}
