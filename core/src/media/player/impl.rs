use super::*;

/// Built-in sample constructors and helpers for `ImageAsset`.
impl ImageAsset {
    /// Generates a crisp high-contrast architectural sample photo asset for previews and tests.
    ///
    /// # Arguments
    ///
    /// - `&str` - Alternative accessibility caption.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready `ImageAsset`.
    #[must_use]
    pub fn sample_photo(alt_text: &str) -> Self {
        let buf: ImageBuffer = MediaCodecRegistry::synthesize_frame(
            240,
            135,
            Color::hex(0x0F172A),
            Color::hex(0x38BDF8),
            2,
        );
        Self {
            format: ImageFormat::Png,
            buffer: buf,
            alt_text: alt_text.to_string(),
            metadata: MediaMetadata {
                title: alt_text.to_string(),
                width: 240,
                height: 135,
                duration_secs: 0.0,
                page_count: 1,
                frame_count: 1,
                sample_rate_hz: 0,
                channels: 0,
                format_label: "PNG · 240×135".to_string(),
            },
        }
    }
}

/// Playback stepping and sample generation for `GifAnimation`.
impl GifAnimation {
    /// Creates an 8-frame looping neon GIF animation sample.
    ///
    /// # Returns
    ///
    /// - `Self` - Populated `GifAnimation`.
    #[must_use]
    pub fn sample_neon_gif() -> Self {
        let accents: [Color; 4] = [
            Color::hex(0x38BDF8),
            Color::hex(0xA855F7),
            Color::hex(0xF43F5E),
            Color::hex(0x34D399),
        ];
        let mut frames: Vec<MediaFrame> = Vec::with_capacity(8);
        for i in 0..8 {
            frames.push(MediaFrame {
                image: MediaCodecRegistry::synthesize_frame(
                    220,
                    124,
                    Color::hex(0x111827),
                    accents[i % accents.len()],
                    i,
                ),
                delay_ms: 120,
                timestamp_secs: (i as f32) * 0.12,
            });
        }
        Self {
            frames,
            loop_count: 0,
            current_frame: 0,
            elapsed_ms: 0,
            playing: true,
            metadata: MediaMetadata {
                title: "Neon Pulse Loop".to_string(),
                width: 220,
                height: 124,
                duration_secs: 0.96,
                page_count: 1,
                frame_count: 8,
                sample_rate_hz: 0,
                channels: 0,
                format_label: "GIF89a · 8f".to_string(),
            },
        }
    }

    /// Advances the GIF frame timer by `dt_ms` milliseconds when playing.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable GIF animation.
    /// - `u32` - Elapsed time in milliseconds.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the displayed frame index changed.
    pub fn step_ms(&mut self, dt_ms: u32) -> bool {
        if !self.get_playing() || self.get_frames().is_empty() {
            return false;
        }
        let cur: usize = self.get_current_frame() % self.get_frames().len();
        let delay: u32 = self.get_frames()[cur].get_delay_ms().max(16);
        let next_elapsed: u32 = self.get_elapsed_ms().saturating_add(dt_ms);
        if next_elapsed >= delay {
            let next_idx: usize = (cur + 1) % self.get_frames().len();
            self.set_current_frame(next_idx);
            self.set_elapsed_ms(next_elapsed - delay);
            true
        } else {
            self.set_elapsed_ms(next_elapsed);
            false
        }
    }

    /// Advances immediately to the next animation frame (useful when paused).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable GIF animation.
    pub fn next_frame(&mut self) {
        if !self.get_frames().is_empty() {
            let next_idx: usize = (self.get_current_frame() + 1) % self.get_frames().len();
            self.set_current_frame(next_idx);
            self.set_elapsed_ms(0);
        }
    }

    /// Returns the currently displayed `ImageBuffer` frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - GIF animation reference.
    ///
    /// # Returns
    ///
    /// - `ImageBuffer` - Active frame image buffer.
    #[must_use]
    pub fn active_image(&self) -> ImageBuffer {
        self.get_frames()
            .get(self.get_current_frame())
            .or_else(|| self.get_frames().first())
            .map(|f: &MediaFrame| f.get_image().clone())
            .unwrap_or_else(|| ImageBuffer::solid(64, 64, Color::hex(0x0F172A)))
    }
}

/// Interactive LIVE Photo motion burst playback and sample builder for `LivePhotoAsset`.
impl LivePhotoAsset {
    /// Builds a sample LIVE Photo with a sharp still keyframe, 6-frame motion burst, and ambient sound peaks.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready `LivePhotoAsset`.
    #[must_use]
    pub fn sample_live_photo() -> Self {
        let still: ImageBuffer = MediaCodecRegistry::synthesize_frame(
            240,
            135,
            Color::hex(0x1E1B4B),
            Color::hex(0xF59E0B),
            0,
        );
        let mut motion_frames: Vec<MediaFrame> = Vec::with_capacity(6);
        for i in 0..6 {
            motion_frames.push(MediaFrame {
                image: MediaCodecRegistry::synthesize_frame(
                    240,
                    135,
                    Color::hex(0x1E1B4B),
                    Color::hex(0xF59E0B),
                    i + 1,
                ),
                delay_ms: 110,
                timestamp_secs: (i as f32) * 0.11,
            });
        }
        Self {
            still_image: still,
            motion_frames,
            audio_waveform: vec![0.2, 0.45, 0.78, 0.92, 0.65, 0.50, 0.35, 0.22],
            active_motion: false,
            motion_elapsed_ms: 0,
            muted: false,
            metadata: MediaMetadata {
                title: "Golden Hour Boulevard LIVE".to_string(),
                width: 240,
                height: 135,
                duration_secs: 0.66,
                page_count: 1,
                frame_count: 6,
                sample_rate_hz: 48_000,
                channels: 2,
                format_label: "LIVE · HEIC+MOV".to_string(),
            },
        }
    }

    /// Starts or stops the LIVE motion burst (called on press-and-hold or LIVE toggle).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable LIVE Photo asset.
    /// - `bool` - `true` to begin motion playback, `false` to return to keyframe still.
    pub fn set_motion_playing(&mut self, active: bool) {
        self.set_active_motion(active);
        if !active {
            self.set_motion_elapsed_ms(0);
        }
    }

    /// Advances the LIVE motion burst by `dt_ms` milliseconds.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable LIVE Photo asset.
    /// - `u32` - Elapsed time in milliseconds.
    pub fn step_ms(&mut self, dt_ms: u32) {
        if !self.get_active_motion() || self.get_motion_frames().is_empty() {
            return;
        }
        let total_ms: u32 = (self.get_motion_frames().len() as u32) * 110;
        let next: u32 = self.get_motion_elapsed_ms().saturating_add(dt_ms);
        if next >= total_ms {
            self.set_motion_elapsed_ms(0);
            self.set_active_motion(false);
        } else {
            self.set_motion_elapsed_ms(next);
        }
    }

    /// Returns either the keyframe still photo or the active LIVE motion frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - LIVE Photo reference.
    ///
    /// # Returns
    ///
    /// - `ImageBuffer` - Current display image buffer.
    #[must_use]
    pub fn active_image(&self) -> ImageBuffer {
        if !self.get_active_motion() || self.get_motion_frames().is_empty() {
            return self.get_still_image().clone();
        }
        let idx: usize =
            ((self.get_motion_elapsed_ms() / 110) as usize) % self.get_motion_frames().len();
        self.get_motion_frames()[idx].get_image().clone()
    }
}

/// Transport playback, seeking, captions, and sample builder for `VideoTrack`.
impl VideoTrack {
    /// Generates a deterministic 12-second sample video track with poster, timed frames, and closed captions.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready `VideoTrack`.
    #[must_use]
    pub fn sample_hdr_video() -> Self {
        let poster: ImageBuffer = MediaCodecRegistry::synthesize_frame(
            280,
            158,
            Color::hex(0x090D16),
            Color::hex(0x22D3EE),
            0,
        );
        let mut frames: Vec<MediaFrame> = Vec::with_capacity(12);
        let palette: [Color; 4] = [
            Color::hex(0x22D3EE),
            Color::hex(0x6366F1),
            Color::hex(0xEC4899),
            Color::hex(0x10B981),
        ];
        for i in 0..12 {
            frames.push(MediaFrame {
                image: MediaCodecRegistry::synthesize_frame(
                    280,
                    158,
                    Color::hex(0x090D16),
                    palette[i % palette.len()],
                    i + 1,
                ),
                delay_ms: 1000,
                timestamp_secs: i as f32,
            });
        }
        Self {
            metadata: MediaMetadata {
                title: "euv Engine HDR Showcase Reel".to_string(),
                width: 280,
                height: 158,
                duration_secs: 12.0,
                page_count: 1,
                frame_count: 12,
                sample_rate_hz: 48_000,
                channels: 2,
                format_label: "MP4 · H.264 · 60fps".to_string(),
            },
            codec: VideoCodec::H264,
            poster,
            frames,
            state: PlaybackState::Paused,
            current_time_secs: 0.0,
            volume: 0.85,
            muted: false,
            playback_rate: 1.0,
            looping: true,
            captions: vec![
                "Cross-platform native raster + GPU hybrid pipeline.".to_string(),
                "Zero JavaScript WebAssembly bundle running at 60Hz.".to_string(),
                "Real-time BVH ray-traced shadows and reflections.".to_string(),
                "Immersive iOS, Android, macOS, Windows, and Linux.".to_string(),
            ],
        }
    }

    /// Toggles between `PlaybackState::Playing` and `PlaybackState::Paused`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable video track.
    pub fn toggle_play(&mut self) {
        if self.get_state() == PlaybackState::Playing {
            self.set_state(PlaybackState::Paused);
        } else {
            if self.get_state() == PlaybackState::Ended {
                self.set_current_time_secs(0.0);
            }
            self.set_state(PlaybackState::Playing);
        }
    }

    /// Seeks playback position to `secs`, clamped to `[0.0, duration_secs]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable video track.
    /// - `f32` - Target timestamp in seconds.
    pub fn seek(&mut self, secs: f32) {
        let dur: f32 = self.get_metadata().get_duration_secs().max(0.0);
        self.set_current_time_secs(secs.clamp(0.0, dur));
    }

    /// Advances video playback by `dt_secs * playback_rate`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable video track.
    /// - `f32` - Elapsed wall delta time in seconds.
    pub fn step(&mut self, dt_secs: f32) {
        if self.get_state() != PlaybackState::Playing {
            return;
        }
        let dur: f32 = self.get_metadata().get_duration_secs().max(0.1);
        let next: f32 = self.get_current_time_secs() + dt_secs * self.get_playback_rate();
        if next >= dur {
            if self.get_looping() {
                self.set_current_time_secs(next % dur);
            } else {
                self.set_current_time_secs(dur);
                self.set_state(PlaybackState::Ended);
            }
        } else {
            self.set_current_time_secs(next);
        }
    }

    /// Returns the video `ImageBuffer` corresponding to `current_time_secs`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Video track reference.
    ///
    /// # Returns
    ///
    /// - `ImageBuffer` - Active video frame or poster image.
    #[must_use]
    pub fn active_frame(&self) -> ImageBuffer {
        if self.get_frames().is_empty() {
            return self.get_poster().clone();
        }
        let dur: f32 = self.get_metadata().get_duration_secs().max(0.1);
        let frac: f32 = (self.get_current_time_secs() / dur).clamp(0.0, 0.9999);
        let idx: usize =
            ((frac * (self.get_frames().len() as f32)) as usize).min(self.get_frames().len() - 1);
        self.get_frames()[idx].get_image().clone()
    }

    /// Returns the synchronized closed-caption line for the active playback timestamp.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Video track reference.
    ///
    /// # Returns
    ///
    /// - `String` - Active closed-caption string or empty string.
    #[must_use]
    pub fn active_caption(&self) -> String {
        if self.get_captions().is_empty() {
            return String::new();
        }
        let dur: f32 = self.get_metadata().get_duration_secs().max(0.1);
        let frac: f32 = (self.get_current_time_secs() / dur).clamp(0.0, 0.9999);
        let idx: usize = ((frac * (self.get_captions().len() as f32)) as usize)
            .min(self.get_captions().len() - 1);
        self.get_captions()[idx].clone()
    }
}

/// Transport playback, seeking, and procedural WAV synthesis for `AudioTrack`.
impl AudioTrack {
    /// Synthesizes a valid RIFF PCM 16-bit mono WAV byte stream and decodes it through `MediaCodecRegistry::decode_wav`.
    ///
    /// # Returns
    ///
    /// - `Self` - Decoded `AudioTrack` with real PCM samples and 48 waveform peaks.
    #[must_use]
    pub fn sample_synth_wav() -> Self {
        let sample_rate: u32 = 8_000;
        let duration_secs: usize = 8;
        let sample_count: usize = (sample_rate as usize) * duration_secs;
        let data_bytes: u32 = (sample_count * 2) as u32;
        let mut wav: Vec<u8> = Vec::with_capacity(44 + data_bytes as usize);
        wav.extend_from_slice(MAGIC_RIFF);
        wav.extend_from_slice(&(36_u32 + data_bytes).to_le_bytes());
        wav.extend_from_slice(MAGIC_WAVE);
        wav.extend_from_slice(MAGIC_WAV_FMT);
        wav.extend_from_slice(&16_u32.to_le_bytes());
        wav.extend_from_slice(&1_u16.to_le_bytes());
        wav.extend_from_slice(&1_u16.to_le_bytes());
        wav.extend_from_slice(&sample_rate.to_le_bytes());
        wav.extend_from_slice(&(sample_rate * 2).to_le_bytes());
        wav.extend_from_slice(&2_u16.to_le_bytes());
        wav.extend_from_slice(&16_u16.to_le_bytes());
        wav.extend_from_slice(MAGIC_WAV_DATA);
        wav.extend_from_slice(&data_bytes.to_le_bytes());
        for i in 0..sample_count {
            let t: f32 = (i as f32) / (sample_rate as f32);
            let env: f32 = 0.35 + 0.65 * (t * 1.4).sin().abs();
            let sig: f32 = ((t * 440.0 * TAU).sin() * 0.55 + (t * 659.25 * TAU).sin() * 0.35) * env;
            let s16: i16 = (sig.clamp(-1.0, 1.0) * 28_000.0) as i16;
            wav.extend_from_slice(&s16.to_le_bytes());
        }
        MediaCodecRegistry::decode_wav(&wav, "Cyber Boulevard Synth.wav").unwrap_or_default()
    }

    /// Toggles audio playback between `Playing` and `Paused`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable audio track.
    pub fn toggle_play(&mut self) {
        if self.get_state() == PlaybackState::Playing {
            self.set_state(PlaybackState::Paused);
        } else {
            if self.get_state() == PlaybackState::Ended {
                self.set_current_time_secs(0.0);
            }
            self.set_state(PlaybackState::Playing);
        }
    }

    /// Seeks the audio playhead to `secs`, clamped to `[0.0, duration_secs]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable audio track.
    /// - `f32` - Target timestamp in seconds.
    pub fn seek(&mut self, secs: f32) {
        let dur: f32 = self.get_metadata().get_duration_secs().max(0.0);
        self.set_current_time_secs(secs.clamp(0.0, dur));
    }

    /// Advances audio timeline playback by `dt_secs * playback_rate`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable audio track.
    /// - `f32` - Elapsed delta time in seconds.
    pub fn step(&mut self, dt_secs: f32) {
        if self.get_state() != PlaybackState::Playing {
            return;
        }
        let dur: f32 = self.get_metadata().get_duration_secs().max(0.1);
        let next: f32 = self.get_current_time_secs() + dt_secs * self.get_playback_rate();
        if next >= dur {
            if self.get_looping() {
                self.set_current_time_secs(next % dur);
            } else {
                self.set_current_time_secs(dur);
                self.set_state(PlaybackState::Ended);
            }
        } else {
            self.set_current_time_secs(next);
        }
    }
}

/// Optional preview helper for `PdfPage`.
impl PdfPage {
    /// Returns a cloned optional page preview image.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Page reference.
    ///
    /// # Returns
    ///
    /// - `Option<ImageBuffer>` - Optional page thumbnail image.
    #[must_use]
    pub fn preview_image(&self) -> Option<ImageBuffer> {
        let Self { preview_image, .. } = self;
        preview_image.clone()
    }
}

/// Page navigation, zoom clamping, and sample PDF generation for `PdfDocument`.
impl PdfDocument {
    /// Generates and parses a valid multi-page `%PDF-1.7` document through `MediaCodecRegistry::parse_pdf`.
    ///
    /// # Returns
    ///
    /// - `Self` - Parsed 3-page `PdfDocument`.
    #[must_use]
    pub fn sample_pdf() -> Self {
        let raw_pdf: &str = "%PDF-1.7\n\
1 0 obj << /Title (euv Cross-Platform Architecture Spec) /Author (euv Engineering) >> endobj\n\
2 0 obj << /Type /Pages /Count 3 >> endobj\n\
3 0 obj << /Type /Page >> stream\n\
(1. Executive Summary: Zero-JS Cross-Platform Native Engine) Tj\n\
(euv compiles reactive Rust UI, 2D physics, and 3D BVH ray tracing directly to Web, iOS, Android, macOS, Windows, and Linux.) Tj\n\
endstream endobj\n\
4 0 obj << /Type /Page >> stream\n\
(2. Unified Design System & Full Media Pipeline) Tj\n\
(Native decoders handle BMP/PNG images, animated GIF89a, LIVE Photos, H.264 video frames, RIFF WAV waveforms, PDF documents, and sanitized HTML5.) Tj\n\
endstream endobj\n\
5 0 obj << /Type /Page >> stream\n\
(3. Production Telemetry & Release Stripping) Tj\n\
(AI Schema introspection and UI automation hooks compile strictly under debug_assertions for zero Release overhead.) Tj\n\
endstream endobj\n\
%%EOF\n";
        MediaCodecRegistry::parse_pdf(raw_pdf.as_bytes()).unwrap_or_default()
    }

    /// Advances to the next PDF page if available.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable PDF document.
    pub fn next_page(&mut self) {
        if self.get_current_page() + 1 < self.get_pages().len() {
            let next: usize = self.get_current_page() + 1;
            self.set_current_page(next);
        }
    }

    /// Navigates to the previous PDF page if available.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable PDF document.
    pub fn prev_page(&mut self) {
        if self.get_current_page() > 0 {
            let prev: usize = self.get_current_page() - 1;
            self.set_current_page(prev);
        }
    }

    /// Adjusts document zoom level clamped to `[0.75, 2.0]`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable PDF document.
    /// - `f32` - Zoom delta (e.g. `0.25` or `-0.25`).
    pub fn adjust_zoom(&mut self, delta: f32) {
        let next: f32 = (self.get_zoom() + delta).clamp(0.75, 2.0);
        self.set_zoom(next);
    }

    /// Returns the active `PdfPage` if present.
    ///
    /// # Arguments
    ///
    /// - `&Self` - PDF document reference.
    ///
    /// # Returns
    ///
    /// - `Option<&PdfPage>` - Reference to active page.
    #[must_use]
    pub fn active_page(&self) -> Option<&PdfPage> {
        self.get_pages().get(self.get_current_page())
    }
}

/// Optional URL accessors for `HtmlBlock`.
impl HtmlBlock {
    /// Returns the optional hyperlink URL for `HtmlNodeKind::Link`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - HTML block reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Hyperlink target URL if present.
    #[must_use]
    pub fn href_url(&self) -> Option<String> {
        let Self { href, .. } = self;
        href.clone()
    }

    /// Returns the optional image source URI for `HtmlNodeKind::Image`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - HTML block reference.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Image URI if present.
    #[must_use]
    pub fn image_src(&self) -> Option<String> {
        let Self { src, .. } = self;
        src.clone()
    }
}

/// Sample rich HTML5 document builder for `HtmlDocument`.
impl HtmlDocument {
    /// Builds and parses a rich sanitized HTML5 article demonstrating headings, quotes, code, lists, and links.
    ///
    /// # Returns
    ///
    /// - `Self` - Parsed `HtmlDocument`.
    #[must_use]
    pub fn sample_html() -> Self {
        let src: &str = "<!DOCTYPE html><html><head><title>Native HTML5 Renderer</title></head><body>\
<h1>Zero-JS Native HTML5 Document</h1>\
<p>Rendered directly into euv scene nodes across Web, iOS, Android, and Desktop with zero WebView overhead.</p>\
<blockquote>Sanitized DOM parsing automatically strips &lt;script&gt; tags while preserving semantic typography &amp; links.</blockquote>\
<pre>let doc = MediaCodecRegistry::parse_html(markup);</pre>\
<li>Full support for headings, paragraphs, code blocks, lists, images, and dividers</li>\
<li>Click links below to dispatch native Rust callbacks without JavaScript</li>\
<hr/>\
<a href=\"euv://docs/media-engine\">Open euv Media Engine Spec</a>\
</body></html>";
        MediaCodecRegistry::parse_html(src)
    }
}

/// Sample SVG vector builder for `SvgDocument`.
impl SvgDocument {
    /// Builds and rasterizes a clean multi-primitive geometric SVG icon badge.
    ///
    /// # Returns
    ///
    /// - `Self` - Rasterized `SvgDocument`.
    #[must_use]
    pub fn sample_svg() -> Self {
        let raw: &str = "<svg viewBox=\"0 0 160 100\">\
<rect x=\"0\" y=\"0\" width=\"160\" height=\"100\" fill=\"#0F172A\"/>\
<rect x=\"12\" y=\"12\" width=\"136\" height=\"76\" fill=\"#1E293B\"/>\
<circle cx=\"52\" cy=\"50\" r=\"24\" fill=\"#38BDF8\"/>\
<circle cx=\"92\" cy=\"50\" r=\"20\" fill=\"#A855F7\"/>\
<line x1=\"24\" y1=\"80\" x2=\"136\" y2=\"80\" stroke=\"#34D399\"/>\
</svg>";
        MediaCodecRegistry::parse_and_rasterize_svg(raw, 480, 300).unwrap_or_default()
    }
}
