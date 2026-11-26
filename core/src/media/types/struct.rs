use super::*;

/// Unified descriptor pointing to a local, remote, or in-memory media resource.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct MediaSource {
    /// Source URI, asset path, or logical identifier.
    pub(crate) uri: String,
    /// MIME type string (e.g. `"image/png"`, `"video/mp4"`, `"application/pdf"`, `"text/html"`).
    pub(crate) mime_type: String,
    /// Resolved semantic media category.
    #[get(type(copy))]
    pub(crate) kind: MediaKind,
    /// Optional inline raw binary payload for embedded or fetched media.
    pub(crate) bytes: Vec<u8>,
}

/// Normalized inspection metadata common across all media assets.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct MediaMetadata {
    /// Human-readable title or filename.
    pub(crate) title: String,
    /// Intrinsic pixel width (`0` for audio-only assets).
    #[get(type(copy))]
    pub(crate) width: u32,
    /// Intrinsic pixel height (`0` for audio-only assets).
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Total playback duration in seconds (`0.0` for static media).
    #[get(type(copy))]
    pub(crate) duration_secs: f32,
    /// Page count (`>= 1` for PDF documents).
    #[get(type(copy))]
    pub(crate) page_count: usize,
    /// Frame count (`>= 1` for GIF, LIVE Photo, or Video sequences).
    #[get(type(copy))]
    pub(crate) frame_count: usize,
    /// Audio sample rate in Hz (e.g. `44_100` or `48_000`).
    #[get(type(copy))]
    pub(crate) sample_rate_hz: u32,
    /// Audio channel count (`1` = mono, `2` = stereo).
    #[get(type(copy))]
    pub(crate) channels: u16,
    /// Short human-readable format badge (e.g. `"BMP"`, `"GIF89a"`, `"LIVE"`, `"MP4/H.264"`, `"WAV"`, `"PDF 1.7"`, `"HTML5"`, `"SVG"`).
    pub(crate) format_label: String,
}

/// Single timed RGBA8 frame belonging to an animated GIF, LIVE Photo, or Video track.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct MediaFrame {
    /// Decoded RGBA8 pixel buffer for this frame.
    pub(crate) image: ImageBuffer,
    /// Display duration of this frame in milliseconds.
    #[get(type(copy))]
    pub(crate) delay_ms: u32,
    /// Presentation timestamp in seconds from timeline start.
    #[get(type(copy))]
    pub(crate) timestamp_secs: f32,
}

/// Decoded static image asset carrying format, alt text, metadata, and RGBA8 pixel buffer.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct ImageAsset {
    /// Detected source image format.
    #[get(type(copy))]
    pub(crate) format: ImageFormat,
    /// Decoded RGBA8 pixel buffer ready for software or GPU rendering.
    pub(crate) buffer: ImageBuffer,
    /// Accessibility alternative description text.
    pub(crate) alt_text: String,
    /// Intrinsic image metadata.
    pub(crate) metadata: MediaMetadata,
}

/// Decoded multi-frame animated GIF player model supporting looping and frame-accurate stepping.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct GifAnimation {
    /// Ordered animation frames with per-frame millisecond delays.
    pub(crate) frames: Vec<MediaFrame>,
    /// Number of loops (`0` indicates infinite looping).
    #[get(type(copy))]
    pub(crate) loop_count: u32,
    /// Currently visible frame index.
    #[get(type(copy))]
    pub(crate) current_frame: usize,
    /// Accumulated milliseconds within the active frame.
    #[get(type(copy))]
    pub(crate) elapsed_ms: u32,
    /// Whether playback is actively advancing.
    #[get(type(copy))]
    pub(crate) playing: bool,
    /// GIF asset metadata.
    pub(crate) metadata: MediaMetadata,
}

/// Apple/Android-style LIVE Photo pairing a still photograph with a short motion burst and audio waveform.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct LivePhotoAsset {
    /// Primary high-resolution still photograph displayed at rest.
    pub(crate) still_image: ImageBuffer,
    /// Short burst of motion frames revealed on press-and-hold or LIVE trigger.
    pub(crate) motion_frames: Vec<MediaFrame>,
    /// Ambient sound waveform amplitudes (`0.0..=1.0`) accompanying the LIVE burst.
    pub(crate) audio_waveform: Vec<f32>,
    /// Whether LIVE motion playback is currently active.
    #[get(type(copy))]
    pub(crate) active_motion: bool,
    /// Elapsed milliseconds into the active LIVE motion burst.
    #[get(type(copy))]
    pub(crate) motion_elapsed_ms: u32,
    /// Whether the accompanying LIVE audio burst is muted.
    #[get(type(copy))]
    pub(crate) muted: bool,
    /// LIVE Photo metadata.
    pub(crate) metadata: MediaMetadata,
}

/// Seekable cross-platform video playback track with poster frame, timed frames, volume, speed, and captions.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct VideoTrack {
    /// Video stream metadata.
    pub(crate) metadata: MediaMetadata,
    /// Encoded or raw video codec descriptor.
    #[get(type(copy))]
    pub(crate) codec: VideoCodec,
    /// Poster frame shown before playback begins.
    pub(crate) poster: ImageBuffer,
    /// Timed decoded video frames.
    pub(crate) frames: Vec<MediaFrame>,
    /// Current playback state.
    #[get(type(copy))]
    pub(crate) state: PlaybackState,
    /// Current playback position in seconds.
    #[get(type(copy))]
    pub(crate) current_time_secs: f32,
    /// Normalized audio volume (`0.0..=1.0`).
    #[get(type(copy))]
    pub(crate) volume: f32,
    /// Whether audio output is muted.
    #[get(type(copy))]
    pub(crate) muted: bool,
    /// Playback speed multiplier (`0.25..=4.0`).
    #[get(type(copy))]
    pub(crate) playback_rate: f32,
    /// Whether playback automatically loops on completion.
    #[get(type(copy))]
    pub(crate) looping: bool,
    /// Optional closed-caption lines synchronized by frame index.
    pub(crate) captions: Vec<String>,
}

/// Seekable digital audio track with PCM samples, precomputed waveform peaks, and transport state.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct AudioTrack {
    /// Audio track metadata.
    pub(crate) metadata: MediaMetadata,
    /// Source audio codec.
    #[get(type(copy))]
    pub(crate) codec: AudioCodec,
    /// Normalized mono/interleaved floating-point PCM samples in `[-1.0, 1.0]`.
    pub(crate) samples: Vec<f32>,
    /// Normalized bar heights (`0.0..=1.0`) for UI waveform visualization.
    pub(crate) waveform_peaks: Vec<f32>,
    /// Current playback state.
    #[get(type(copy))]
    pub(crate) state: PlaybackState,
    /// Current playback timestamp in seconds.
    #[get(type(copy))]
    pub(crate) current_time_secs: f32,
    /// Output gain in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) volume: f32,
    /// Whether audio output is muted.
    #[get(type(copy))]
    pub(crate) muted: bool,
    /// Playback speed multiplier (`0.5..=2.0`).
    #[get(type(copy))]
    pub(crate) playback_rate: f32,
    /// Whether playback restarts automatically at end-of-track.
    #[get(type(copy))]
    pub(crate) looping: bool,
}

/// Single parsed page of a Portable Document Format (`PDF`) document.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct PdfPage {
    /// 1-based page number.
    #[get(type(copy))]
    pub(crate) page_number: usize,
    /// Page width in PostScript points (`612.0` = US Letter, `595.0` = A4).
    #[get(type(copy))]
    pub(crate) width_pt: f32,
    /// Page height in PostScript points (`792.0` = US Letter, `842.0` = A4).
    #[get(type(copy))]
    pub(crate) height_pt: f32,
    /// Section heading or title extracted from this page.
    pub(crate) title: String,
    /// Ordered text paragraphs extracted from the page content stream.
    pub(crate) text_blocks: Vec<String>,
    /// Optional rasterized page thumbnail or diagram preview.
    #[get(skip)]
    pub(crate) preview_image: Option<ImageBuffer>,
}

/// Parsed multi-page PDF document with page navigation and zoom state.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct PdfDocument {
    /// Document title extracted from PDF info dictionary or fallback name.
    pub(crate) title: String,
    /// Document author metadata.
    pub(crate) author: String,
    /// PDF specification version (e.g. `"1.7"` or `"2.0"`).
    pub(crate) version: String,
    /// Ordered pages of the document.
    pub(crate) pages: Vec<PdfPage>,
    /// 0-based active page index.
    #[get(type(copy))]
    pub(crate) current_page: usize,
    /// Active viewport zoom factor (`0.5..=2.5`).
    #[get(type(copy))]
    pub(crate) zoom: f32,
}

/// Single sanitized block node parsed from an HTML document.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct HtmlBlock {
    /// Semantic HTML block classification.
    #[get(type(copy))]
    pub(crate) kind: HtmlNodeKind,
    /// Decoded plain text content of this block.
    pub(crate) text: String,
    /// Optional hyperlink URL for `HtmlNodeKind::Link`.
    #[get(skip)]
    pub(crate) href: Option<String>,
    /// Optional image URI for `HtmlNodeKind::Image`.
    #[get(skip)]
    pub(crate) src: Option<String>,
}

/// Sanitized native HTML document compiled into renderable `HtmlBlock` items without JS or WebView.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct HtmlDocument {
    /// Document title extracted from `<title>` or first heading.
    pub(crate) title: String,
    /// Original raw HTML markup source.
    pub(crate) raw_html: String,
    /// Ordered sanitized visual blocks ready for native `euv` rendering.
    pub(crate) blocks: Vec<HtmlBlock>,
    /// Total clickable hyperlinks parsed from the document.
    #[get(type(copy))]
    pub(crate) links_count: usize,
}

/// Parsed and natively rasterized SVG vector document.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct SvgDocument {
    /// Intrinsic SVG viewBox width.
    #[get(type(copy))]
    pub(crate) view_box_width: f32,
    /// Intrinsic SVG viewBox height.
    #[get(type(copy))]
    pub(crate) view_box_height: f32,
    /// Raw XML SVG source markup.
    pub(crate) raw_svg: String,
    /// Rasterized RGBA8 pixel representation ready for immediate cross-platform presentation.
    pub(crate) rasterized: ImageBuffer,
    /// Number of vector primitives (`rect`, `circle`, `line`) parsed from the SVG.
    #[get(type(copy))]
    pub(crate) primitive_count: usize,
}

/// Stateless cross-platform media codec registry providing magic-byte detection and native pure-Rust decoders.
#[derive(Clone, Copy, Debug, Default)]
pub struct MediaCodecRegistry;
