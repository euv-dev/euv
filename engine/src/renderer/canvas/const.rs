/// The window property name for device pixel ratio (HiDPI scale factor).
pub(crate) const RENDERER_PROPERTY_DEVICE_PIXEL_RATIO: &str = "devicePixelRatio";

/// The fallback device pixel ratio when window detection fails.
pub(crate) const RENDERER_DEFAULT_DEVICE_PIXEL_RATIO: f64 = 1.0;

/// The high-quality text rendering value for geometric precision.
pub(crate) const RENDERER_TEXT_RENDERING_GEOMETRIC_PRECISION: &str = "geometricPrecision";

/// The default font family used for text rendering.
pub(crate) const RENDERER_DEFAULT_FONT_FAMILY: &str = "sans-serif";

/// The high-quality image smoothing value.
pub(crate) const RENDERER_IMAGE_SMOOTHING_QUALITY_HIGH: &str = "high";

/// The default render layer z-index for background elements.
pub(crate) const RENDERER_LAYER_BACKGROUND: i32 = 0;

/// The default render layer z-index for foreground game objects.
pub(crate) const RENDERER_LAYER_FOREGROUND: i32 = 100;

/// The low-quality image smoothing value (fastest, pixelated-friendly).
pub(crate) const RENDERER_IMAGE_SMOOTHING_QUALITY_LOW: &str = "low";

/// The default font size in pixels.
pub(crate) const RENDERER_DEFAULT_FONT_SIZE: f64 = 16.0;

/// The canvas context property name for text rendering.
pub(crate) const RENDERER_PROPERTY_TEXT_RENDERING: &str = "textRendering";

/// The default render layer z-index for UI overlay elements.
pub(crate) const RENDERER_LAYER_UI: i32 = 1000;

/// The default SSAA scale factor (2.0 means 4x supersampling).
pub(crate) const RENDERER_DEFAULT_SSAA_SCALE_FACTOR: f64 = 2.0;

/// The default shadow color used when no explicit color is provided.
pub(crate) const RENDERER_DEFAULT_SHADOW_COLOR: &str = "rgba(0, 0, 0, 0.5)";

/// The canvas context property name for image smoothing quality.
pub(crate) const RENDERER_PROPERTY_IMAGE_SMOOTHING_QUALITY: &str = "imageSmoothingQuality";

/// The default shadow blur radius in pixels.
pub(crate) const RENDERER_DEFAULT_SHADOW_BLUR: f64 = 4.0;

/// The default camera zoom level.
pub(crate) const RENDERER_DEFAULT_CAMERA_ZOOM: f64 = 1.0;

/// The medium-quality image smoothing value.
pub(crate) const RENDERER_IMAGE_SMOOTHING_QUALITY_MEDIUM: &str = "medium";

/// The HTML element tag name for creating a canvas element.
pub(crate) const RENDERER_ELEMENT_CANVAS: &str = "canvas";

/// The default camera rotation in radians.
pub(crate) const RENDERER_DEFAULT_CAMERA_ROTATION: f64 = 0.0;

/// The canvas 2D rendering context type identifier.
pub(crate) const RENDERER_CONTEXT_TYPE_2D: &str = "2d";
