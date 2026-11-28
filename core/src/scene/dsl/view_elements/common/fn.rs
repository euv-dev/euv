/// Applies a base `Style` preset before inline property overrides.
pub const fn style() {}

/// Binds a reactive `move || -> Style` closure that updates this node's style.
pub const fn bind_style() {}

/// Binds a reactive `move || -> Vec<NodeId>` closure managing dynamic child nodes.
pub const fn bind_children() {}

/// Captures the created `NodeId` into a mutable local binding or `NodeRefTarget`.
pub const fn node_ref() {}

/// Controls whether this node can receive keyboard focus (`bool`).
pub const fn focusable() {}

/// Sets box display mode (`Display`).
pub const fn display() {}

/// Sets positioning mode (`Position`).
pub const fn position() {}

/// Sets `(top, right, bottom, left)` offsets (`(Option<f32>, Option<f32>, Option<f32>, Option<f32>)`).
pub const fn inset() {}

/// Sets box width (`Length`).
pub const fn width() {}

/// Sets box height (`Length`).
pub const fn height() {}

/// Sets minimum width constraint (`Length`).
pub const fn min_width() {}

/// Sets minimum height constraint (`Length`).
pub const fn min_height() {}

/// Sets maximum width constraint (`Length`).
pub const fn max_width() {}

/// Sets maximum height constraint (`Length`).
pub const fn max_height() {}

/// Sets preferred width-to-height aspect ratio (`f32`).
pub const fn aspect_ratio() {}

/// Sets inner padding (`Edges`).
pub const fn padding() {}

/// Sets outer margin (`Edges`).
pub const fn margin() {}

/// Sets uniform child gap in logical pixels (`f32`).
pub const fn gap() {}

/// Sets solid background fill (`Color`).
pub const fn background() {}

/// Sets linear gradient background fill (`LinearGradient`).
pub const fn gradient() {}

/// Sets drop shadow elevation (`Shadow`).
pub const fn shadow() {}

/// Sets focus outline border (`Border`).
pub const fn focus_ring() {}

/// Sets pointer hover cursor icon (`CursorIcon`).
pub const fn cursor() {}

/// Sets pointer hit-test participation policy (`PointerEvents`).
pub const fn pointer_events() {}

/// Blocks scroll propagation to ancestor scroll views (`bool`).
pub const fn stop_scroll_propagation() {}

/// Blocks pointer/click event bubbling to ancestors (`bool`).
pub const fn stop_event_propagation() {}

/// Sets foreground text color (`Color`).
pub const fn color() {}

/// Sets uniform corner radius in logical pixels (`f32`).
pub const fn radius() {}

/// Sets `(width, color)` border stroke (`(f32, Color)`).
pub const fn border() {}

/// Sets cross-axis item alignment (`AlignItems`).
pub const fn align() {}

/// Sets main-axis content justification (`JustifyContent`).
pub const fn justify() {}

/// Sets flex grow weight (`f32`).
pub const fn flex_grow() {}

/// Sets flex shrink weight (`f32`).
pub const fn flex_shrink() {}

/// Sets flex wrap policy (`FlexWrap`).
pub const fn flex_wrap() {}

/// Sets overflow clipping or scrolling (`Overflow`).
pub const fn overflow() {}

/// Sets font size in logical pixels (`f32`).
pub const fn font_size() {}

/// Sets font weight (`FontWeight`).
pub const fn font_weight() {}

/// Sets font family (`FontFamily`).
pub const fn font_family() {}

/// Enables or disables italic font slant (`bool`).
pub const fn italic() {}

/// Sets horizontal text alignment (`TextAlign`).
pub const fn text_align() {}

/// Sets text overflow strategy (`TextOverflow`).
pub const fn text_overflow() {}

/// Enables or disables native text selection (`bool`, inherited by descendant text nodes unless overridden).
pub const fn selectable() {}

/// Sets the highlight fill color used for selected text ranges (`Color`).
pub const fn selection_color() {}

/// Sets stacking order (`i32`).
pub const fn z_index() {}

/// Sets node opacity multiplier (`f32`).
pub const fn opacity() {}

/// Sets grid column count (`u16`).
pub const fn columns() {}

/// Registers a click handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_click() {}

/// Registers a value change handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_change() {}

/// Registers a scroll handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_scroll() {}

/// Registers a pointer-down handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_pointer_down() {}

/// Registers a pointer-move handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_pointer_move() {}

/// Registers a pointer-up handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_pointer_up() {}

/// Registers a pointer-cancel handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_pointer_cancel() {}

/// Registers a pointer-enter handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_pointer_enter() {}

/// Registers a pointer-leave handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_pointer_leave() {}

/// Registers a pointer hover enter/leave handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_hover() {}

/// Registers a keyboard key-down handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_key() {}

/// Registers a focus-gained handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_focus() {}

/// Registers a focus-lost handler (`FnMut(&UiEvent) + 'static`).
pub const fn on_blur() {}

/// Sets complete `A11yProps` metadata (mutually exclusive with individual a11y fields).
pub const fn a11y() {}

/// Sets semantic accessibility role (`A11yRole`).
pub const fn role() {}

/// Sets accessible screen-reader label string.
pub const fn a11y_label() {}

/// Sets accessible screen-reader usage hint string.
pub const fn a11y_hint() {}

/// Sets accessible current value string.
pub const fn a11y_value() {}

/// Marks node disabled in accessibility tree (`bool`).
pub const fn a11y_disabled() {}

/// Sets accessible checked state (`Option<bool>`).
pub const fn a11y_checked() {}
