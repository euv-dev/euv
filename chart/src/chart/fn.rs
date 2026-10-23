use super::*;

/// Returns the size of a canvas, in CSS pixels.
///
/// The element's layout size wins over its attribute size, because the
/// attribute is what the drawing buffer is sized by and the layout size
/// is what the user sees.
///
/// # Arguments
///
/// - `&HtmlCanvasElement` - Shared reference to a `HtmlCanvasElement`.
///
/// # Returns
///
/// - `(f64, f64)` - A `(f64, f64)` value.
pub(crate) fn canvas_size(canvas: &HtmlCanvasElement) -> (f64, f64) {
    let width: f64 = canvas.client_width() as f64;
    let height: f64 = canvas.client_height() as f64;
    if width > 0.0 && height > 0.0 {
        return (width, height);
    }
    (canvas.width() as f64, canvas.height() as f64)
}

/// Returns the device pixel ratio of the host.
///
/// # Returns
///
/// - `f64` - A 64-bit float.
pub(crate) fn detect_dpr() -> f64 {
    let Some(window) = web_sys::window() else {
        return 1.0;
    };
    window.device_pixel_ratio().max(1.0)
}

/// Returns `true` when an axis draws its own line and its tick marks.
///
/// ECharts hides the line of a value axis whose other axis is a
/// category axis: the grid of the chart says it better than a line
/// would. A category axis always draws its line, and a value axis
/// draws it when the other axis is a value axis.
///
/// # Arguments
///
/// - `&AxisLayout` - Shared reference to a `AxisLayout`.
/// - `&GridLayout` - Shared reference to a `GridLayout`.
/// - `bool` - A boolean (`bool`).
///
/// # Returns
///
/// - `bool` - A boolean.
pub(crate) fn draws_own_line(axis: &AxisLayout, grid: &GridLayout, horizontal: bool) -> bool {
    match axis.kind {
        AxisType::Category => true,
        _ => {
            let other: Option<&AxisLayout> = if horizontal {
                grid.y_axes.first()
            } else {
                grid.x_axes.first()
            };
            matches!(
                other.map(|axis: &AxisLayout| { axis.kind }),
                Some(AxisType::Value) | Some(AxisType::Log)
            )
        }
    }
}
