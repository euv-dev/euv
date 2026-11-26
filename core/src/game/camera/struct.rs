use super::*;

/// 2D game camera supporting world-to-screen projection, zoom, smooth target follow, frustum culling, and trauma screen shake.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct Camera2D {
    /// World-space focal center position.
    #[get(type(copy))]
    pub(crate) position: Point,
    /// Uniform zoom multiplier (`1.0` = 1:1 world-to-logical-pixel).
    #[get(type(copy))]
    pub(crate) zoom: f32,
    /// Current screen-shake trauma level in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) trauma: f32,
    /// Maximum screen-shake displacement in world units.
    #[get(type(copy))]
    pub(crate) max_shake: f32,
    /// Accumulated shake phase timer in seconds.
    #[get(type(copy))]
    pub(crate) shake_time: f32,
    /// Optional world clamp rectangle preventing camera overscroll past arena edges.
    #[get(skip)]
    pub(crate) world_bounds: Option<Rect>,
}
