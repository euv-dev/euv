use super::*;

/// Fluent constructors and node-matching predicates for `UiSelector`.
impl UiSelector {
    /// Creates an exact text selector.
    ///
    /// # Arguments
    ///
    /// - `&str` - Exact text label to match.
    ///
    /// # Returns
    ///
    /// - `Self` - `UiSelector::ByText` variant.
    #[must_use]
    pub fn text(label: &str) -> Self {
        Self::ByText(label.to_string())
    }

    /// Creates a substring text selector.
    ///
    /// # Arguments
    ///
    /// - `&str` - Substring to match in text, value, or placeholder.
    ///
    /// # Returns
    ///
    /// - `Self` - `UiSelector::ByTextContains` variant.
    #[must_use]
    pub fn text_contains(substr: &str) -> Self {
        Self::ByTextContains(substr.to_string())
    }

    /// Creates an input placeholder selector.
    ///
    /// # Arguments
    ///
    /// - `&str` - Exact input placeholder text.
    ///
    /// # Returns
    ///
    /// - `Self` - `UiSelector::ByPlaceholder` variant.
    #[must_use]
    pub fn placeholder(hint: &str) -> Self {
        Self::ByPlaceholder(hint.to_string())
    }

    /// Creates an exact input value selector.
    ///
    /// # Arguments
    ///
    /// - `&str` - Exact input value text.
    ///
    /// # Returns
    ///
    /// - `Self` - `UiSelector::ByInputValue` variant.
    #[must_use]
    pub fn input_value(value: &str) -> Self {
        Self::ByInputValue(value.to_string())
    }

    /// Creates an accessibility label selector.
    ///
    /// # Arguments
    ///
    /// - `&str` - Exact accessibility label text.
    ///
    /// # Returns
    ///
    /// - `Self` - `UiSelector::ByA11yLabel` variant.
    #[must_use]
    pub fn a11y_label(label: &str) -> Self {
        Self::ByA11yLabel(label.to_string())
    }

    /// Creates an accessibility role selector.
    ///
    /// # Arguments
    ///
    /// - `&str` - Role name such as `"Button"` or `"TextField"`.
    ///
    /// # Returns
    ///
    /// - `Self` - `UiSelector::ByRole` variant.
    #[must_use]
    pub fn role(role_name: &str) -> Self {
        Self::ByRole(role_name.to_string())
    }

    /// Creates a scene node kind selector.
    ///
    /// # Arguments
    ///
    /// - `&str` - Node kind name (`"View"`, `"Text"`, `"Input"`, `"ScrollView"`, `"Canvas"`, `"Portal"`).
    ///
    /// # Returns
    ///
    /// - `Self` - `UiSelector::ByKind` variant.
    #[must_use]
    pub fn kind(kind_name: &str) -> Self {
        Self::ByKind(kind_name.to_string())
    }

    /// Wraps this selector to select only its `index`-th match.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base selector.
    /// - `usize` - Zero-based occurrence index.
    ///
    /// # Returns
    ///
    /// - `Self` - `UiSelector::Nth` selector.
    #[must_use]
    pub fn nth(self, index: usize) -> Self {
        Self::Nth {
            inner: Box::new(self),
            index,
        }
    }

    /// Evaluates whether a single `AiLayoutNode` matches this selector (ignoring `Nth` indexing).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Selector reference.
    /// - `&AiLayoutNode` - Candidate inspected layout node.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the node matches.
    #[must_use]
    pub fn matches_node(&self, node: &AiLayoutNode) -> bool {
        match self {
            Self::ByText(expected) => {
                node.text_value() == Some(expected.as_str())
                    || node.input_value() == Some(expected.as_str())
            }
            Self::ByTextContains(sub) => {
                node.text_value()
                    .map(|t: &str| t.contains(sub.as_str()))
                    .unwrap_or(false)
                    || node
                        .input_value()
                        .map(|t: &str| t.contains(sub.as_str()))
                        .unwrap_or(false)
                    || node
                        .placeholder_value()
                        .map(|t: &str| t.contains(sub.as_str()))
                        .unwrap_or(false)
                    || node
                        .a11y_label_value()
                        .map(|t: &str| t.contains(sub.as_str()))
                        .unwrap_or(false)
            }
            Self::ByPlaceholder(expected) => node.placeholder_value() == Some(expected.as_str()),
            Self::ByInputValue(expected) => node.input_value() == Some(expected.as_str()),
            Self::ByA11yLabel(expected) => node
                .a11y_label_value()
                .map(|lbl: &str| lbl.eq_ignore_ascii_case(expected))
                .unwrap_or(false),
            Self::ByRole(role_name) => node.get_role().eq_ignore_ascii_case(role_name),
            Self::ByKind(kind_name) => node.get_kind().eq_ignore_ascii_case(kind_name),
            Self::ByNodeIndex(idx) => node.get_node_index() == *idx,
            Self::Interactive => node.get_interactive(),
            Self::Nth { inner, .. } => inner.matches_node(node),
            Self::All(preds) => preds.iter().all(|p: &UiSelector| p.matches_node(node)),
        }
    }
}
