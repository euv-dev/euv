use super::*;

/// Decoded RGBA8 image buffer rendered uniformly by software and hardware backends.
#[derive(Clone, Debug, Default, Eq, Getter, New, PartialEq)]
pub struct ImageBuffer {
    /// Image width in pixels.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// Image height in pixels.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Packed RGBA8 pixel bytes (`width * height * 4`).
    pub(crate) pixels: Vec<u8>,
}

/// Interactive text input state stored on `NodeKind::Input`.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct InputNodeState {
    /// Current UTF-8 value entered by the user.
    pub(crate) value: String,
    /// Placeholder text displayed when `value` is empty.
    pub(crate) placeholder: String,
    /// Byte offset of the caret inside `value`.
    #[get(type(copy))]
    pub(crate) caret: usize,
    /// Whether characters should be masked as a password field.
    #[get(type(copy))]
    pub(crate) password: bool,
    /// Whether multi-line input is enabled.
    #[get(type(copy))]
    pub(crate) multiline: bool,
    /// Whether input is disabled.
    #[get(type(copy))]
    pub(crate) disabled: bool,
}
