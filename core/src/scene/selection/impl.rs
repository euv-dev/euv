use super::*;

/// Character-range queries and text extraction helpers for `SelectionState`.
impl SelectionState {
    /// Returns the normalized `(start_char, end_char)` range with `start_char <= end_char`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Selection state reference.
    ///
    /// # Returns
    ///
    /// - `(usize, usize)` - Ordered character index bounds.
    #[must_use]
    #[inline(always)]
    pub const fn ordered_range(&self) -> (usize, usize) {
        let Self {
            anchor_char,
            focus_char,
            ..
        } = *self;
        if anchor_char <= focus_char {
            (anchor_char, focus_char)
        } else {
            (focus_char, anchor_char)
        }
    }

    /// Returns `true` when the selection is collapsed (zero selected characters).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Selection state reference.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether `anchor_char == focus_char`.
    #[must_use]
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        let Self {
            anchor_char,
            focus_char,
            ..
        } = *self;
        anchor_char == focus_char
    }

    /// Extracts the selected UTF-8 slice from `text`, safely clamping to valid character boundaries.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Selection state reference.
    /// - `&'a str` - Source UTF-8 text string of the selected node.
    ///
    /// # Returns
    ///
    /// - `&'a str` - Selected substring slice (empty when collapsed).
    #[must_use]
    pub fn extract_from<'a>(&self, text: &'a str) -> &'a str {
        let (start_char, end_char): (usize, usize) = self.ordered_range();
        if start_char >= end_char || text.is_empty() {
            return "";
        }
        let mut start_byte: usize = text.len();
        let mut end_byte: usize = text.len();
        for (char_idx, (byte_idx, _)) in text.char_indices().enumerate() {
            if char_idx == start_char {
                start_byte = byte_idx;
            }
            if char_idx == end_char {
                end_byte = byte_idx;
                break;
            }
        }
        if start_byte >= end_byte {
            ""
        } else {
            &text[start_byte..end_byte]
        }
    }
}
