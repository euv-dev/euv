use super::*;

/// Default layout style matching standard Flexbox defaults.
impl Default for LayoutStyle {
    /// Creates default layout properties (`Flex`, `Row`, `flex_shrink: 1.0`).
    ///
    /// # Returns
    ///
    /// - `Self` - Default `LayoutStyle`.
    fn default() -> Self {
        Self {
            display: Display::Flex,
            position: Position::Relative,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            align_self: AlignSelf::Auto,
            overflow: Overflow::Visible,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Length::Auto,
            grid_columns: 1,
            width: Length::Auto,
            height: Length::Auto,
            min_width: Length::Auto,
            min_height: Length::Auto,
            max_width: Length::Auto,
            max_height: Length::Auto,
            margin: Edges::default(),
            padding: Edges::default(),
            row_gap: 0.0,
            column_gap: 0.0,
            top: None,
            right: None,
            bottom: None,
            left: None,
            z_index: 0,
            aspect_ratio: None,
        }
    }
}

/// Breakpoint classification helpers.
impl Breakpoint {
    /// Classifies a logical viewport width into its canonical responsive breakpoint.
    ///
    /// # Arguments
    ///
    /// - `f32` - Logical viewport width in pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Matching `Breakpoint`.
    #[must_use]
    pub fn from_width(width: f32) -> Self {
        if width < 768.0 {
            Self::Compact
        } else if width < 1024.0 {
            Self::Medium
        } else {
            Self::Expanded
        }
    }

    /// Returns true when the viewport is compact mobile width (`< 768px`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Breakpoint value.
    ///
    /// # Returns
    ///
    /// - `bool` - True for `Compact`.
    #[must_use]
    pub fn is_compact(&self) -> bool {
        *self == Self::Compact
    }
}

/// Non-panicking optional offset readers for `LayoutStyle`.
impl LayoutStyle {
    /// Returns the optional `top` offset.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Layout style.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Top offset if specified.
    #[must_use]
    pub fn top_offset(&self) -> Option<f32> {
        let Self { top, .. } = *self;
        top
    }

    /// Returns the optional `right` offset.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Layout style.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Right offset if specified.
    #[must_use]
    pub fn right_offset(&self) -> Option<f32> {
        let Self { right, .. } = *self;
        right
    }

    /// Returns the optional `bottom` offset.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Layout style.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Bottom offset if specified.
    #[must_use]
    pub fn bottom_offset(&self) -> Option<f32> {
        let Self { bottom, .. } = *self;
        bottom
    }

    /// Returns the optional `left` offset.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Layout style.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Left offset if specified.
    #[must_use]
    pub fn left_offset(&self) -> Option<f32> {
        let Self { left, .. } = *self;
        left
    }

    /// Returns the optional aspect ratio (`width / height`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Layout style.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Aspect ratio if specified.
    #[must_use]
    pub fn aspect_ratio_value(&self) -> Option<f32> {
        let Self { aspect_ratio, .. } = *self;
        aspect_ratio
    }
}
