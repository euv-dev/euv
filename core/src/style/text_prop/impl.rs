use super::*;

/// Default typography style (`14px` Sans Regular, `1.4` line height).
impl Default for TextStyle {
    /// Creates the default UI text style.
    ///
    /// # Returns
    ///
    /// - `Self` - Default `TextStyle`.
    fn default() -> Self {
        Self {
            family: FontFamily::Sans,
            font_size: 14.0,
            weight: FontWeight::Normal,
            line_height: 1.4,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            overflow: TextOverflow::Wrap,
            color: COLOR_BLACK,
            underline: false,
            strikethrough: false,
        }
    }
}

/// Numeric weight helpers for `FontWeight`.
impl FontWeight {
    /// Returns the OpenType numeric weight (`300..=700`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Font weight variant.
    ///
    /// # Returns
    ///
    /// - `u16` - OpenType numeric weight value.
    #[must_use]
    pub const fn numeric(&self) -> u16 {
        match *self {
            Self::Light => 300,
            Self::Normal => 400,
            Self::Medium => 500,
            Self::SemiBold => 600,
            Self::Bold => 700,
        }
    }

    /// Returns true when the weight renders bold strokes (`>= 600`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Font weight variant.
    ///
    /// # Returns
    ///
    /// - `bool` - True for `SemiBold` and `Bold`.
    #[must_use]
    pub const fn is_bold(&self) -> bool {
        matches!(*self, Self::SemiBold | Self::Bold)
    }
}
