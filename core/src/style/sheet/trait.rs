use super::*;

/// Type-directed property application trait enabling concise enum/value shorthand inside `style! { AlignItems::Center; Overflow::Hidden; }`.
pub trait StyleApply {
    /// Applies this strongly-typed style property directly onto `style`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Typed style property value.
    /// - `Style` - Target style accumulator.
    ///
    /// # Returns
    ///
    /// - `Style` - Updated style with the property applied.
    #[must_use]
    fn apply_to(self, style: Style) -> Style;
}
