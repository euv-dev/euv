/// Extension trait for `Element` providing DOM attribute/property manipulation methods.
///
/// Since Rust's orphan rules prevent adding inherent methods to foreign types like
/// `web_sys::Element`, this trait provides the same functionality through an extension
/// trait pattern. All methods are available on any `Element` reference via trait dispatch.
pub trait ElementExt {
    /// Removes or clears a DOM attribute/property, depending on the attribute name.
    ///
    /// For `value`, sets the DOM property to an empty string rather than calling
    /// `remove_attribute`, because `remove_attribute("value")` only removes the
    /// HTML attribute and does not clear the displayed value of input elements.
    /// For boolean properties (`checked`, `disabled`, `selected`, `readonly`),
    /// sets the DOM property to `false` rather than calling `remove_attribute`,
    /// because `remove_attribute` on a previously-set attribute may not correctly
    /// reset the property in all browsers.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name of the attribute or property to remove.
    fn remove_attribute_or_property(&self, name: &str);

    /// Sets a DOM attribute or property, depending on the attribute name.
    ///
    /// For `value`, uses the DOM property to ensure input elements update correctly.
    /// For boolean attributes (`checked`, `disabled`, `selected`, `readonly`),
    /// uses the DOM property so that the browser honors the value correctly
    /// (HTML attributes are present-or-absent, not true/false strings).
    /// For all other attributes, uses `set_attribute`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name of the attribute or property to set.
    /// - `&str` - The value to assign.
    fn set_attribute_or_property(&self, name: &str, value: &str);

    /// Returns the element's `data-euv-id`, assigning a fresh one if absent.
    ///
    /// The id keys every per-element framework registry (event handlers,
    /// binding cleanups, `NodeRef` cells), so any code path that installs
    /// per-element state calls this first to guarantee the element carries
    /// an id. Reading and assigning are both single JS crossings; the
    /// assignment happens at most once per element.
    ///
    /// # Returns
    ///
    /// - `usize` - The element's `data-euv-id` value.
    fn ensure_euv_id(&self) -> usize;
}
