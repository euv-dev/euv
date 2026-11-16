use super::*;

/// Responsive mode selected from the caller-owned breakpoint signal.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvLayoutMode {
    /// Wide-screen header, sidebar and main content.
    #[default]
    Desktop,
    /// Header, main content and a persistently mounted navigation drawer.
    Mobile,
}
