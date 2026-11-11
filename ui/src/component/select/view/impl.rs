use super::*;

/// Internal behavior shared by Select option keyboard navigation.
impl EuvSelectOption {
    /// Checks whether the option label begins with a keyboard typeahead prefix.
    ///
    /// # Arguments
    ///
    /// - `&str` - The case-insensitive prefix to match.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether this enabled option matches the prefix.
    pub(crate) fn matches_typeahead(&self, query: &str) -> bool {
        !self.get_disabled()
            && self
                .get_label()
                .to_lowercase()
                .starts_with(&query.to_lowercase())
    }
}

/// Implementation of default props for Select.
impl Default for EuvSelectProps {
    /// Constructs a default [`EuvSelectProps`] value.
    fn default() -> Self {
        Self {
            value: Signal::create(String::new()),
            options: Vec::new(),
            placeholder: "",
            label: "",
            id: "",
            name: "",
            required: false,
            invalid: false,
            disabled: false,
            align_item_with_trigger: cfg!(feature = "theme-shadcn"),
            on_change: None,
        }
    }
}
