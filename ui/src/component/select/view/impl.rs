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
            multiple: false,
            values: Signal::create(Vec::new()),
            size: EuvSelectSize::Default,
            options: Vec::new(),
            placeholder: "",
            label: "",
            aria_label: String::new(),
            aria_describedby: "",
            id: "",
            name: "",
            form: "",
            auto_complete: "",
            required: false,
            invalid: false,
            disabled: false,
            read_only: false,
            open: None,
            default_open: false,
            align_item_with_trigger: class_property(
                c_euv_select_panel(),
                "--euv-select-align-item",
            )
            .as_deref()
                == Some("1"),
            on_change: None,
            on_change_values: None,
            on_open_change: None,
        }
    }
}
