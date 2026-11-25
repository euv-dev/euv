use super::*;

/// Responsive cross-platform static image component with format badge and optional caption.
#[derive(Clone, Data, Debug)]
pub struct EuvImage {
    /// Decoded static image asset.
    pub(crate) asset: ImageAsset,
    /// Visual box-fitting strategy (`Contain`, `Cover`, `Fill`, `ScaleDown`).
    #[get(type(copy))]
    pub(crate) fit: MediaFit,
    /// Fixed viewport height in logical pixels.
    #[get(type(copy))]
    pub(crate) height_px: f32,
    /// Optional caption displayed below the image.
    pub(crate) caption: String,
    /// Resolved theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Native vector SVG viewer displaying a parsed and rasterized `SvgDocument`.
#[derive(Clone, Data, Debug)]
pub struct EuvSvgView {
    /// Parsed and rasterized SVG document.
    pub(crate) document: SvgDocument,
    /// Display height in logical pixels.
    #[get(type(copy))]
    pub(crate) height_px: f32,
    /// Display label.
    pub(crate) label: String,
    /// Resolved theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive multi-frame animated GIF player with Play/Pause and single-frame stepping.
#[derive(Clone, Data, Debug)]
pub struct EuvGifPlayer {
    /// Reactive GIF animation state signal.
    #[get(type(copy))]
    pub(crate) state: Signal<GifAnimation>,
    /// Display height in logical pixels.
    #[get(type(copy))]
    pub(crate) height_px: f32,
    /// Resolved theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Interactive Apple/Android-style LIVE Photo viewer supporting press-and-hold and button-triggered motion bursts.
#[derive(Clone, Data, Debug)]
pub struct EuvLivePhoto {
    /// Reactive LIVE Photo asset signal.
    #[get(type(copy))]
    pub(crate) state: Signal<LivePhotoAsset>,
    /// Display height in logical pixels.
    #[get(type(copy))]
    pub(crate) height_px: f32,
    /// Resolved theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Cross-platform interactive video player with timeline scrubbing, speed, mute, captions, and transport controls.
#[derive(Clone, Data, Debug)]
pub struct EuvVideoPlayer {
    /// Reactive video track signal.
    #[get(type(copy))]
    pub(crate) state: Signal<VideoTrack>,
    /// Display height in logical pixels.
    #[get(type(copy))]
    pub(crate) height_px: f32,
    /// Resolved theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Studio audio player with a 48-bar reactive waveform visualizer, scrub slider, mute, and timecode readout.
#[derive(Clone, Data, Debug)]
pub struct EuvAudioPlayer {
    /// Reactive audio track signal.
    #[get(type(copy))]
    pub(crate) state: Signal<AudioTrack>,
    /// Resolved theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Multi-page native PDF document viewer with page navigation and zoom controls.
#[derive(Clone, Data, Debug)]
pub struct EuvPdfViewer {
    /// Reactive PDF document signal.
    #[get(type(copy))]
    pub(crate) state: Signal<PdfDocument>,
    /// Resolved theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}

/// Sanitized native HTML5 renderer compiling parsed `HtmlDocument` blocks directly into `euv` scene nodes.
#[derive(Clone, CustomDebug, Data)]
pub struct EuvHtmlView {
    /// Parsed sanitized HTML document.
    pub(crate) document: HtmlDocument,
    /// Optional callback invoked with the clicked `href` URL when a hyperlink is activated.
    #[debug(skip)]
    #[get(skip)]
    pub(crate) on_link: Option<HtmlLinkCallback>,
    /// Resolved theme tokens.
    #[get(type(copy))]
    pub(crate) tokens: ThemeTokens,
}
