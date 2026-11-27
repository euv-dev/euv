use super::*;

/// Default typography style (`14px` Sans Regular, `1.4` line height).
impl Default for TextStyle {
    /// Creates the default UI text style.
    ///
    /// # Returns
    ///
    /// - `Self` - Default `TextStyle`.
    #[inline(always)]
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
            italic: false,
            underline: false,
            strikethrough: false,
            selectable: false,
            selectable_explicit: false,
            selection_color: None,
        }
    }
}

/// Optional selection highlight helpers for `TextStyle`.
impl TextStyle {
    /// Returns the optional custom selection highlight fill color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Text style reference.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - Selection highlight tint if explicitly configured.
    #[must_use]
    #[inline(always)]
    pub fn selection_color(&self) -> Option<Color> {
        let Self {
            selection_color, ..
        } = *self;
        selection_color
    }

    /// Returns `true` when `other` differs only in color or selection properties and has identical layout metrics.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Previous text style.
    /// - `&Self` - Candidate updated text style.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when typography layout metrics are unchanged.
    #[must_use]
    #[inline(always)]
    pub(crate) fn same_layout_metrics(&self, other: &Self) -> bool {
        self.get_family() == other.get_family()
            && self.get_font_size() == other.get_font_size()
            && self.get_weight() == other.get_weight()
            && self.get_line_height() == other.get_line_height()
            && self.get_letter_spacing() == other.get_letter_spacing()
            && self.get_align() == other.get_align()
            && self.get_overflow() == other.get_overflow()
            && self.get_italic() == other.get_italic()
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
    #[inline(always)]
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
    #[inline(always)]
    pub const fn is_bold(&self) -> bool {
        matches!(*self, Self::SemiBold | Self::Bold)
    }
}
