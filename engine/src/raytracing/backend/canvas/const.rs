/// The blit origin: a frame always covers the whole surface, so the
/// image is written from the canvas' top-left corner with no offset.
pub(crate) const CANVAS_BLIT_ORIGIN: f64 = 0.0;

/// The error reported when a traced frame does not cover exactly one
/// surface.
///
/// A frame that is too short is a truncation and a frame that is too
/// long is a stale buffer; neither is ever clipped to fit, because a
/// silently clipped frame still produces a picture.
pub(crate) const ERROR_CANVAS_PIXEL_LENGTH: &str =
    "the traced pixel buffer does not cover exactly one surface";

/// The error reported when a released backend is driven again.
pub(crate) const ERROR_CANVAS_RELEASED: &str = "the Canvas 2D backend has already been released";
