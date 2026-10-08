use super::*;

#[test]

fn the_default_font_is_the_documented_size_and_family() {
    let rendered: String = CanvasRenderer::default_font();
    assert_eq!(
        rendered, "16px sans-serif",
        "every renderer falls back to this, so the size and family must be exact"
    );
}

#[test]
fn a_default_font_with_no_explicit_size_is_still_valid_css() {
    let rendered: String = CanvasRenderer::default_font();
    let parts: Vec<&str> = rendered.split(' ').collect();
    assert_eq!(
        parts.len(),
        2,
        "a shorthand needs a size and a family, got {rendered:?}"
    );
    assert!(
        parts[0].ends_with("px"),
        "a bare number would be an invalid length in css, got {rendered:?}"
    );
}
