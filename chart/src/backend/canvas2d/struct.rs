use super::*;

/// The Canvas2D rendering backend.
///
/// This is the universal fallback: it works in every browser, including
/// ones with neither WebGPU nor WebGL2. It replays the display list as
/// immediate-mode canvas calls, which is slower than the GPU backends but
/// produces exactly the same picture.
#[derive(Clone, Data, Debug)]
pub struct Canvas2dChartBackend {
    /// The canvas the chart draws into.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) canvas: HtmlCanvasElement,
    /// The 2D rendering context.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) context: CanvasRenderingContext2d,
    /// The device pixel ratio the context is currently scaled by.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dpr: f64,
    /// The logical width of the canvas, in CSS pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) width: f64,
    /// The logical height of the canvas, in CSS pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) height: f64,
}
