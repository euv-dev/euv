use super::*;

/// The direction the panels of a resizable group are laid out in.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvResizableDirection {
    /// The panels sit beside each other.
    #[default]
    Horizontal,
    /// The panels sit above each other.
    Vertical,
}
