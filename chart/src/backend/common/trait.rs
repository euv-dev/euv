use super::*;

/// A rendering backend for the chart.
///
/// The three backends — WebGPU, WebGL2, and Canvas2D — all replay the
/// same display list, so the chart model never has to know which one is
/// in use. That is what keeps the public API identical across backends.
pub trait ChartBackend {
    /// Returns the kind of the backend.
    ///
    /// # Returns
    ///
    /// - `BackendKind` - A `BackendKind` value.
    fn kind(&self) -> BackendKind;

    /// Prepares the backend for a canvas of the given CSS size, at the
    /// given device pixel ratio.
    ///
    /// The backends size their drawing buffer to `width * dpr` by
    /// `height * dpr` physical pixels and scale their coordinate system
    /// accordingly, so the model always works in CSS pixels.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    fn resize(&mut self, width: f64, height: f64, dpr: f64);

    /// Replays the display list onto the canvas.
    ///
    /// The backend is free to cache whatever it can: the display list is
    /// retained between frames, so a chart that has not changed should
    /// cost close to nothing to draw.
    ///
    /// # Arguments
    ///
    /// - `&DisplayList` - Shared reference to a `DisplayList`.
    /// - `&mut TextShaper` - Mutable reference to a `TextShaper` (mutated in place).
    fn render(&mut self, list: &DisplayList, shaper: &mut TextShaper);

    /// Releases the resources the backend holds.
    ///
    /// Called when the chart is disposed. The backend must stay usable
    /// afterwards — it will simply re-acquire its resources on the next
    /// render.
    fn dispose(&mut self);
}
