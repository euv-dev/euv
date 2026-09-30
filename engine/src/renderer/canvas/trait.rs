use super::*;

/// A trait for objects that can record themselves into a draw list.
pub trait Renderable {
    /// Records this object's draw commands into the given draw list.
    ///
    /// # Arguments
    ///
    /// - `&mut DrawList` - The draw list to record commands into.
    /// - `&Transform2D` - The world-space transform to apply.
    fn draw(&self, draw_list: &mut DrawList, transform: &Transform2D);
}

/// An abstract rendering backend that decouples draw calls from the concrete
/// canvas API, enabling future backends such as WebGL or WebGPU.
///
/// All methods mirror the `CanvasRenderingContext2d` interface so that
/// `CanvasRenderer` can implement this trait with zero overhead.
pub trait RenderBackend {
    /// Clears the entire viewport.
    fn clear(&self);

    /// Clears the viewport and fills it with the given CSS color string.
    ///
    /// # Arguments
    ///
    /// - `C: AsRef<str>` - The CSS color string (e.g., `"#000000"`).
    fn clear_color<C>(&self, color: C)
    where
        C: AsRef<str>;

    /// Saves the current rendering state onto the state stack.
    fn save(&self);

    /// Restores the most recently saved rendering state.
    fn restore(&self);

    /// Sets the fill color for subsequent fill operations.
    ///
    /// # Arguments
    ///
    /// - `&str` - The CSS color string.
    fn set_fill_color(&self, color: &str);

    /// Sets the stroke color for subsequent stroke operations.
    ///
    /// # Arguments
    ///
    /// - `&str` - The CSS color string.
    fn set_stroke_color(&self, color: &str);

    /// Sets the line width for subsequent stroke operations.
    ///
    /// # Arguments
    ///
    /// - `f64` - The line width in pixels.
    fn set_line_width(&self, width: f64);

    /// Sets the global alpha (opacity) for all subsequent drawing operations.
    ///
    /// # Arguments
    ///
    /// - `f64` - The alpha value in the range 0.0 to 1.0.
    fn set_global_alpha(&self, alpha: f64);

    /// Sets the blend mode for compositing subsequent draw operations.
    ///
    /// # Arguments
    ///
    /// - `BlendMode` - The blend mode to apply.
    fn set_blend_mode(&self, mode: BlendMode);

    /// Applies a shadow configuration for subsequent draw operations.
    ///
    /// # Arguments
    ///
    /// - `&ShadowConfig` - The shadow configuration to apply.
    fn set_shadow(&self, config: &ShadowConfig);

    /// Clears any previously applied shadow, disabling shadow rendering.
    fn clear_shadow(&self);

    /// Fills a rectangle at the given world-space position and dimensions.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The top-left position in world space.
    /// - `f64` - The width.
    /// - `f64` - The height.
    fn fill_rect(&self, position: Vector2D, width: f64, height: f64);

    /// Strokes the outline of a rectangle at the given world-space position and dimensions.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The top-left position in world space.
    /// - `f64` - The width.
    /// - `f64` - The height.
    fn stroke_rect(&self, position: Vector2D, width: f64, height: f64);

    /// Fills a circle at the given world-space center with the specified radius.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The center in world space.
    /// - `f64` - The radius.
    fn fill_circle(&self, center: Vector2D, radius: f64);

    /// Strokes the outline of a circle at the given world-space center.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The center in world space.
    /// - `f64` - The radius.
    fn stroke_circle(&self, center: Vector2D, radius: f64);

    /// Draws a line segment between two world-space points.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The start point.
    /// - `Vector2D` - The end point.
    fn draw_line(&self, start: Vector2D, end: Vector2D);

    /// Fills text at the given world-space position.
    ///
    /// # Arguments
    ///
    /// - `&str` - The text to draw.
    /// - `Vector2D` - The position in world space.
    fn fill_text(&self, text: &str, position: Vector2D);

    /// Sets the font for subsequent text rendering.
    ///
    /// # Arguments
    ///
    /// - `&str` - The CSS font string (e.g., `"16px sans-serif"`).
    fn set_font(&self, font: &str);

    /// Draws an image element at the given world-space position and dimensions.
    ///
    /// # Arguments
    ///
    /// - `&HtmlImageElement` - The image element to draw.
    /// - `Vector2D` - The top-left position in world space.
    /// - `f64` - The destination width.
    /// - `f64` - The destination height.
    fn draw_image(&self, image: &HtmlImageElement, position: Vector2D, width: f64, height: f64);

    /// Applies a linear gradient as the fill style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&LinearGradient` - The linear gradient to use as fill style.
    fn set_linear_gradient_fill(&self, gradient: &LinearGradient);

    /// Applies a radial gradient as the fill style for subsequent operations.
    ///
    /// # Arguments
    ///
    /// - `&RadialGradient` - The radial gradient to use as fill style.
    fn set_radial_gradient_fill(&self, gradient: &RadialGradient);
}
