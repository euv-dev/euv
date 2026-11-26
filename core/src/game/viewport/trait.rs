use super::*;

/// Real-time game or engine scene embedded inside an `euv` viewport or paired with an `euv` HUD overlay.
pub trait GameScene {
    /// Advances deterministic physics, AI, or ECS state by one fixed simulation step `dt`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene state.
    /// - `f32` - Fixed simulation timestep in seconds.
    /// - `&InputState` - Real-time polled keyboard and pointer state.
    fn fixed_update(&mut self, dt: f32, input: &InputState);

    /// Advances variable-rate camera, animation, or visual interpolation state.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene state.
    /// - `f32` - Variable frame delta time in seconds.
    /// - `&InputState` - Real-time polled input state.
    fn update(&mut self, _dt: f32, _input: &InputState) {}

    /// Emits immediate-mode geometry or blits an external engine RGBA8 framebuffer into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable scene state.
    /// - `f32` - Interpolation alpha in `[0.0, 1.0]`.
    /// - `&mut GameRenderContext<'_>` - Viewport-local draw surface.
    fn render(&mut self, alpha: f32, ctx: &mut GameRenderContext<'_>);
}
