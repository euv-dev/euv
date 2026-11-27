/// Sets flex main-axis direction (`row`, `column`, or `FlexDirection`).
pub const fn direction() {}

/// Sets box display mode (`Display::Flex`, `Display::Grid`, `Display::Block`, `Display::None`).
pub const fn display() {}

/// Sets positioning mode (`Position::Relative`, `Position::Absolute`, `Position::Fixed`).
pub const fn position() {}

/// Sets `(top, right, bottom, left)` offsets as `(Option<f32>, Option<f32>, Option<f32>, Option<f32>)`.
pub const fn inset() {}

/// Sets box width (`Length::Px`, `Length::Percent`, `Length::Auto`).
pub const fn width() {}

/// Sets box height (`Length::Px`, `Length::Percent`, `Length::Auto`).
pub const fn height() {}

/// Sets minimum box width constraint (`Length`).
pub const fn min_width() {}

/// Sets minimum box height constraint (`Length`).
pub const fn min_height() {}

/// Sets maximum box width constraint (`Length`).
pub const fn max_width() {}

/// Sets maximum box height constraint (`Length`).
pub const fn max_height() {}

/// Sets preferred width-to-height aspect ratio (`f32`).
pub const fn aspect_ratio() {}

/// Sets inner box padding (`Edges`).
pub const fn padding() {}

/// Sets outer box margin (`Edges`).
pub const fn margin() {}

/// Sets uniform row and column child gap (`f32`).
pub const fn gap() {}

/// Sets solid background fill color (`Color`).
pub const fn background() {}

/// Sets linear gradient background fill (`LinearGradient`).
pub const fn gradient() {}

/// Sets drop shadow elevation (`Shadow`).
pub const fn shadow() {}

/// Sets keyboard focus outline stroke (`Border`).
pub const fn focus_ring() {}

/// Sets pointer hover cursor icon (`CursorIcon`).
pub const fn cursor() {}

/// Controls pointer hit-testing participation (`PointerEvents`).
pub const fn pointer_events() {}

/// Blocks wheel/touch scroll bubbling to ancestor scroll containers (`bool`).
pub const fn stop_scroll_propagation() {}

/// Blocks pointer and click event bubbling to ancestor nodes (`bool`).
pub const fn stop_event_propagation() {}

/// Sets foreground text color (`Color`).
pub const fn color() {}

/// Sets uniform corner radius in logical pixels (`f32`).
pub const fn radius() {}

/// Sets cross-axis child alignment (`AlignItems`).
pub const fn align() {}

/// Sets main-axis content justification (`JustifyContent`).
pub const fn justify() {}

/// Sets flex grow factor (`f32`).
pub const fn flex_grow() {}

/// Sets flex shrink factor (`f32`).
pub const fn flex_shrink() {}

/// Sets flex line wrapping policy (`FlexWrap`).
pub const fn flex_wrap() {}

/// Sets child clipping or scroll behavior (`Overflow`).
pub const fn overflow() {}

/// Sets text font size in logical pixels (`f32`).
pub const fn font_size() {}

/// Sets text font weight (`FontWeight`).
pub const fn font_weight() {}

/// Sets text font family (`FontFamily::Sans`, `FontFamily::Serif`, `FontFamily::Monospace`).
pub const fn font_family() {}

/// Enables or disables italic font slant (`bool`).
pub const fn italic() {}

/// Sets horizontal text alignment (`TextAlign`).
pub const fn text_align() {}

/// Sets text overflow strategy (`TextOverflow::Wrap`, `TextOverflow::Clip`, `TextOverflow::Ellipsis`).
pub const fn text_overflow() {}

/// Sets subtree opacity multiplier in `[0.0, 1.0]` (`f32`).
pub const fn opacity() {}

/// Sets sibling/portal paint stacking order (`i32`).
pub const fn z_index() {}

/// Sets border `(width, color)` tuple (`(f32, Color)`).
pub const fn border() {}

/// Sets uniform CSS Grid column count (`u16`).
pub const fn grid() {}

/// Alias for `grid`: sets uniform CSS Grid column count (`u16`).
pub const fn columns() {}
