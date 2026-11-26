/// Cursor icon hint unified across desktop and iPadOS pointer platforms.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CursorIcon {
    /// Standard platform arrow cursor.
    #[default]
    Default,
    /// Interactive hand pointer.
    Pointer,
    /// Text selection I-beam.
    Text,
    /// Crosshair precision cursor.
    Crosshair,
    /// Move / drag cursor.
    Move,
    /// Forbidden / disabled cursor.
    NotAllowed,
    /// Horizontal resize cursor.
    EwResize,
    /// Vertical resize cursor.
    NsResize,
    /// Grab hand cursor.
    Grab,
    /// Active grabbing hand cursor.
    Grabbing,
}

/// Pointer hit-testing participation policy.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PointerEvents {
    /// Node receives pointer events normally.
    #[default]
    Auto,
    /// Node is transparent to pointer hit-testing.
    None,
}
