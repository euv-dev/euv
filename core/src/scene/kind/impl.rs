use super::*;

/// Helper operations for `ImageBuffer` and `InputNodeState`.
impl ImageBuffer {
    /// Creates a solid-color RGBA8 image buffer for icons, placeholders, or tests.
    ///
    /// # Arguments
    ///
    /// - `u32` - Width in pixels.
    /// - `u32` - Height in pixels.
    /// - `Color` - Fill color.
    ///
    /// # Returns
    ///
    /// - `Self` - Populated `ImageBuffer`.
    #[must_use]
    pub fn solid(width: u32, height: u32, color: Color) -> Self {
        let count: usize = (width as usize).saturating_mul(height as usize);
        let rgba: [u8; 4] = color.to_rgba_bytes();
        let mut pixels: Vec<u8> = Vec::with_capacity(count.saturating_mul(4));
        for _ in 0..count {
            pixels.extend_from_slice(&rgba);
        }
        Self::new(width, height, pixels)
    }
}

/// Text editing and fluent construction helpers for `InputNodeState`.
impl InputNodeState {
    /// Creates an enabled single-line `InputNodeState` initialized with `value` and caret at the end.
    ///
    /// # Arguments
    ///
    /// - `S` - Initial input text value.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready `InputNodeState`.
    #[must_use]
    pub fn text<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        let s: String = value.into();
        let caret: usize = s.len();
        Self::new(s, String::new(), caret, false, false, false)
    }

    /// Sets placeholder text displayed when the input value is empty.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source input state.
    /// - `S` - Placeholder text.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated input state.
    #[must_use]
    pub fn with_placeholder<S>(mut self, placeholder: S) -> Self
    where
        S: Into<String>,
    {
        self.set_placeholder(placeholder.into());
        self
    }

    /// Toggles masked password rendering.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source input state.
    /// - `bool` - Password mode flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated input state.
    #[must_use]
    pub fn with_password(mut self, password: bool) -> Self {
        self.set_password(password);
        self
    }

    /// Toggles multi-line text editing.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source input state.
    /// - `bool` - Multiline flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated input state.
    #[must_use]
    pub fn with_multiline(mut self, multiline: bool) -> Self {
        self.set_multiline(multiline);
        self
    }

    /// Toggles disabled state preventing user edits.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source input state.
    /// - `bool` - Disabled flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated input state.
    #[must_use]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.set_disabled(disabled);
        self
    }

    /// Inserts `text` at the current caret position and advances the caret.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable input state.
    /// - `&str` - UTF-8 text to insert.
    pub fn insert_str(&mut self, text: &str) {
        if self.get_disabled() || text.is_empty() {
            return;
        }
        let pos: usize = self.get_caret().min(self.get_value().len());
        self.get_mut_value().insert_str(pos, text);
        self.set_caret(pos + text.len());
    }

    /// Removes the character immediately preceding the caret if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable input state.
    pub fn backspace(&mut self) {
        if self.get_disabled() || self.get_caret() == 0 {
            return;
        }
        let pos: usize = self.get_caret().min(self.get_value().len());
        let prev_char_start: usize = self.get_value()[..pos]
            .char_indices()
            .next_back()
            .map(|(idx, _): (usize, char)| idx)
            .unwrap_or(0);
        self.get_mut_value().replace_range(prev_char_start..pos, "");
        self.set_caret(prev_char_start);
    }
}
