use super::*;

/// Queries browser dark-mode preference using `window.matchMedia("(prefers-color-scheme: dark)")`.
///
/// # Returns
///
/// - `ThemeMode` - Detected browser appearance mode.
#[must_use]
pub fn web_system_theme() -> ThemeMode {
    let Some(win) = window() else {
        return ThemeMode::Light;
    };
    let matches_dark: bool = win
        .match_media(WEB_DARK_MEDIA_QUERY)
        .ok()
        .flatten()
        .map(|mql: MediaQueryList| mql.matches())
        .unwrap_or(false);
    if matches_dark {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    }
}

/// Queries the browser's `window.devicePixelRatio`.
///
/// # Returns
///
/// - `f32` - Device pixel ratio clamped to at least `0.5`.
#[must_use]
pub fn web_device_pixel_ratio() -> f32 {
    window()
        .map(|w: Window| (w.device_pixel_ratio() as f32).max(0.5))
        .unwrap_or(1.0)
}

/// Creates and attaches an `HTMLCanvasElement` and its 2D context directly via `web-sys`.
///
/// # Arguments
///
/// - `&WindowConfig` - Declarative window configuration.
/// - `f32` - Browser device pixel ratio.
///
/// # Returns
///
/// - `Result<(HtmlCanvasElement, CanvasRenderingContext2d), PlatformError>` - Canvas and 2D context.
pub(crate) fn web_create_canvas(
    config: &WindowConfig,
    dpr: f32,
) -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), PlatformError> {
    let win: Window = window()
        .ok_or_else(|| PlatformError::InitializationFailed(ERR_WEB_WINDOW_MISSING.to_string()))?;
    let doc: Document = win
        .document()
        .ok_or_else(|| PlatformError::InitializationFailed(ERR_WEB_WINDOW_MISSING.to_string()))?;
    doc.set_title(config.get_title());
    let el: Element = doc.create_element(WEB_CANVAS_TAG).map_err(|_: JsValue| {
        PlatformError::InitializationFailed(ERR_WEB_CANVAS_CREATE.to_string())
    })?;
    let canvas: HtmlCanvasElement = el.dyn_into::<HtmlCanvasElement>().map_err(|_: Element| {
        PlatformError::InitializationFailed(ERR_WEB_CANVAS_CREATE.to_string())
    })?;
    let phys_w: u32 = ((config.get_width() as f32) * dpr).round().max(1.0) as u32;
    let phys_h: u32 = ((config.get_height() as f32) * dpr).round().max(1.0) as u32;
    canvas.set_width(phys_w);
    canvas.set_height(phys_h);
    let css_style: String = format!(
        "width:{}px;height:{}px;display:block;margin:0 auto;",
        config.get_width(),
        config.get_height()
    );
    drop(canvas.set_attribute("style", &css_style));
    if let Some(body) = doc.body() {
        drop(body.append_child(&canvas));
    }
    let raw_ctx: Object = canvas
        .get_context(WEB_CONTEXT_2D)
        .map_err(|_: JsValue| PlatformError::SurfaceError(ERR_WEB_CTX2D_MISSING.to_string()))?
        .ok_or_else(|| PlatformError::SurfaceError(ERR_WEB_CTX2D_MISSING.to_string()))?;
    let ctx2d: CanvasRenderingContext2d = raw_ctx
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_: Object| PlatformError::SurfaceError(ERR_WEB_CTX2D_MISSING.to_string()))?;
    Ok((canvas, ctx2d))
}

/// Blits an RGBA8 `Framebuffer` directly onto a browser `CanvasRenderingContext2d` via `ImageData`.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - Browser 2D rendering context.
/// - `&Framebuffer` - Rendered RGBA8 framebuffer.
///
/// # Returns
///
/// - `Result<(), PlatformError>` - Presentation status.
pub(crate) fn web_present_image_data(
    ctx2d: &CanvasRenderingContext2d,
    framebuffer: &Framebuffer,
) -> Result<(), PlatformError> {
    let clamped: Clamped<&[u8]> = Clamped(framebuffer.get_pixels().as_slice());
    let img: ImageData = ImageData::new_with_u8_clamped_array_and_sh(
        clamped,
        framebuffer.get_width(),
        framebuffer.get_height(),
    )
    .map_err(|_: JsValue| PlatformError::SurfaceError(ERR_WEB_PRESENT_FAILED.to_string()))?;
    ctx2d
        .put_image_data(&img, 0.0, 0.0)
        .map_err(|_: JsValue| PlatformError::SurfaceError(ERR_WEB_PRESENT_FAILED.to_string()))
}
