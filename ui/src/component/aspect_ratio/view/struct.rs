use super::*;

/// Props for the `euv_aspect_ratio` component.
///
/// The ratio is the width divided by the height, so `16.0 / 9.0` is the
/// usual widescreen figure.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvAspectRatioProps {
    /// The width-to-height ratio of the box.
    #[get(type(copy))]
    pub ratio: f32,
}
