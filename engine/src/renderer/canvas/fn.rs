use super::*;

/// Draws a transformed sprite immediately with a single `set_transform`.
///
/// Mirrors the `SpriteSheet::draw_frame` fast path: the TRS matrix is composed
/// in Rust (scale signs flip) and applied once, then reset to identity.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - Shared reference to a `CanvasRenderingContext2d`.
/// - `&HtmlImageElement` - Shared reference to a `HtmlImageElement`.
/// - `&Rect` - Shared reference to a `Rect`.
/// - `&Transform2D` - Shared reference to a `Transform2D`.
pub(crate) fn draw_sprite_immediate(
    context: &CanvasRenderingContext2d,
    image: &HtmlImageElement,
    source: &Rect,
    transform: &Transform2D,
) {
    let rotation: f64 = transform.get_rotation();
    let cos: f64 = rotation.cos();
    let sin: f64 = rotation.sin();
    let scale_x: f64 = transform.get_scale().get_x();
    let scale_y: f64 = transform.get_scale().get_y();
    let _: Result<(), JsValue> = context.set_transform(
        cos * scale_x,
        sin * scale_x,
        -sin * scale_y,
        cos * scale_y,
        transform.get_position().get_x(),
        transform.get_position().get_y(),
    );
    let _: Result<(), JsValue> = context
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            image,
            source.get_x(),
            source.get_y(),
            source.get_width(),
            source.get_height(),
            -source.get_width() * 0.5,
            -source.get_height() * 0.5,
            source.get_width(),
            source.get_height(),
        );
    let _: Result<(), JsValue> = context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
}
