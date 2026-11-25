use super::*;

/// Paint-only opacity and 2D transform transition animated per frame without re-running layout.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct NodeTransition {
    /// Target retained scene node being animated.
    #[get(type(copy))]
    pub(crate) node: NodeId,
    /// Elapsed animation time in seconds.
    #[get(type(copy))]
    pub(crate) elapsed: f32,
    /// Total transition duration in seconds.
    #[get(type(copy))]
    pub(crate) duration: f32,
    /// Timing curve evaluated over normalized progress `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) easing: Easing,
    /// Initial opacity value at `t = 0.0`.
    #[get(type(copy))]
    pub(crate) from_opacity: f32,
    /// Target opacity value at `t = 1.0`.
    #[get(type(copy))]
    pub(crate) to_opacity: f32,
    /// Initial horizontal translation offset in logical pixels.
    #[get(type(copy))]
    pub(crate) from_tx: f32,
    /// Target horizontal translation offset in logical pixels.
    #[get(type(copy))]
    pub(crate) to_tx: f32,
    /// Initial vertical translation offset in logical pixels.
    #[get(type(copy))]
    pub(crate) from_ty: f32,
    /// Target vertical translation offset in logical pixels.
    #[get(type(copy))]
    pub(crate) to_ty: f32,
    /// Initial uniform scale factor.
    #[get(type(copy))]
    pub(crate) from_scale: f32,
    /// Target uniform scale factor.
    #[get(type(copy))]
    pub(crate) to_scale: f32,
    /// When `true`, sets `Display::None` on `node` once the exit animation completes.
    #[get(type(copy))]
    pub(crate) hide_on_complete: bool,
}
