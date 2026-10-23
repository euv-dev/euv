use super::*;

/// How an image is currently doing.
///
/// The status is what lets the component show a placeholder while the
/// picture is still on its way, and a readable message when it never
/// arrives.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvImageStatus {
    /// The picture has not reported itself yet.
    #[default]
    Loading,
    /// The picture is on screen.
    Loaded,
    /// The picture could not be loaded.
    Failed,
}
