use super::*;

/// High-level semantic category covering every supported cross-platform media format.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum MediaKind {
    /// Static raster image (PNG, JPEG, WebP, BMP, HEIC, AVIF, raw RGBA).
    #[default]
    Image,
    /// Multi-frame animated GIF image sequence.
    AnimatedGif,
    /// Apple/Android-style LIVE Photo pairing a high-res still with a motion clip and audio pulse.
    LivePhoto,
    /// Timed video stream or frame sequence with audio, captions, and seekable playback.
    Video,
    /// Digital audio track with waveform visualization and playback controls.
    Audio,
    /// Multi-page Portable Document Format (`%PDF-1.x`) document.
    Pdf,
    /// Sanitized rich HTML document rendered natively without a WebView or JavaScript.
    Html,
    /// Scalable Vector Graphics (`<svg>`) document rasterized natively.
    Svg,
}

/// Box-fitting behavior controlling how visual media scales inside its viewport.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum MediaFit {
    /// Scales uniformly so the entire media fits inside the viewport with letterboxing if needed.
    #[default]
    Contain,
    /// Scales uniformly to fill the entire viewport, cropping overflowing edges.
    Cover,
    /// Stretches media to match the exact viewport width and height.
    Fill,
    /// Renders at intrinsic pixel dimensions unless larger than the viewport.
    ScaleDown,
}

/// Transport playback state shared across GIF, LIVE Photo, Video, and Audio players.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PlaybackState {
    /// Loaded and ready at its initial position.
    #[default]
    Idle,
    /// Actively advancing timeline frames or audio samples.
    Playing,
    /// Halted at the current playback timestamp.
    Paused,
    /// Reached the end of a non-looping timeline.
    Ended,
    /// Encountered a decoding or source loading failure.
    Error,
}

/// Encoded image container format identified by magic bytes or file extension.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ImageFormat {
    /// Portable Network Graphics (`.png`).
    #[default]
    Png,
    /// Joint Photographic Experts Group (`.jpg`, `.jpeg`).
    Jpeg,
    /// Google WebP (`.webp`).
    WebP,
    /// Graphics Interchange Format (`.gif`).
    Gif,
    /// Windows Device Independent Bitmap (`.bmp`).
    Bmp,
    /// Scalable Vector Graphics (`.svg`).
    Svg,
    /// High Efficiency Image Container (`.heic`, `.heif`).
    Heic,
    /// AV1 Image File Format (`.avif`).
    Avif,
    /// Uncompressed RGBA8 pixel buffer.
    RawRgba,
}

/// Digital audio container/codec format.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AudioCodec {
    /// RIFF Waveform Audio PCM (`.wav`).
    #[default]
    Wav,
    /// MPEG-1 Audio Layer III (`.mp3`).
    Mp3,
    /// Advanced Audio Coding (`.aac`, `.m4a`).
    Aac,
    /// Opus interactive audio (`.opus`, `.ogg`).
    Opus,
    /// Free Lossless Audio Codec (`.flac`).
    Flac,
    /// Raw floating-point PCM samples.
    Pcm,
}

/// Video codec or frame transport format.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum VideoCodec {
    /// H.264 / AVC hardware-accelerated stream (`.mp4`, `.mov`).
    #[default]
    H264,
    /// H.265 / HEVC stream.
    H265,
    /// VP9 stream (`.webm`).
    Vp9,
    /// AV1 stream.
    Av1,
    /// Self-contained decoded RGBA8 frame sequence for deterministic cross-platform playback.
    RawFrames,
}

/// Semantic block type emitted by the native sanitized HTML parser.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum HtmlNodeKind {
    /// Section heading (`1..=6` matching `<h1>..<h6>`).
    Heading(u8),
    /// Standard body paragraph (`<p>`).
    #[default]
    Paragraph,
    /// Quoted callout block (`<blockquote>`).
    Blockquote,
    /// Monospace code block (`<pre>` / `<code>`).
    CodeBlock,
    /// Bulleted or numbered list entry (`<li>`).
    ListItem,
    /// Clickable hyperlink (`<a href="...">`).
    Link,
    /// Inline or block image reference (`<img src="..." alt="...">`).
    Image,
    /// Visual section divider (`<hr>`).
    HorizontalRule,
}
