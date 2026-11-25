use super::*;

/// Translation lookup and registration methods for `I18nCatalog`.
impl I18nCatalog {
    /// Creates a new `I18nCatalog` with initial `default_locale`.
    ///
    /// # Arguments
    ///
    /// - `String` - Initial active and fallback locale code.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized translation catalog.
    #[must_use]
    pub fn create(default_locale: String) -> Self {
        Self {
            locale: Signal::new(default_locale.clone()),
            fallback_locale: default_locale,
            tables: HashMap::new(),
        }
    }

    /// Registers a translation string for `(locale, key)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable catalog.
    /// - `String` - Locale code.
    /// - `String` - Message key.
    /// - `String` - Localized text.
    pub fn insert(&mut self, locale: String, key: String, value: String) {
        self.get_mut_tables()
            .entry(locale)
            .or_default()
            .insert(key, value);
    }

    /// Translates `key` reactively using the active locale, falling back to `fallback_locale` or `key`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Catalog reference.
    /// - `&str` - Message key.
    ///
    /// # Returns
    ///
    /// - `String` - Localized string.
    #[must_use]
    pub fn t(&self, key: &str) -> String {
        let active: String = self.get_locale().get();
        if let Some(found) = self
            .get_tables()
            .get(&active)
            .and_then(|map: &HashMap<String, String>| map.get(key))
        {
            return found.clone();
        }
        if let Some(found) = self
            .get_tables()
            .get(self.get_fallback_locale())
            .and_then(|map: &HashMap<String, String>| map.get(key))
        {
            return found.clone();
        }
        key.to_string()
    }
}
