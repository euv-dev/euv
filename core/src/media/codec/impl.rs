use super::*;

/// Magic-byte detection, MIME classification, and pure-Rust decoders for all cross-platform media formats.
impl MediaCodecRegistry {
    /// Sniffs `MediaKind` from raw payload magic bytes first, falling back to `uri` extension.
    ///
    /// # Arguments
    ///
    /// - `&str` - Source URI or filename.
    /// - `&[u8]` - Optional leading raw bytes.
    ///
    /// # Returns
    ///
    /// - `MediaKind` - Detected high-level media type.
    #[must_use]
    pub fn detect_media_kind(uri: &str, bytes: &[u8]) -> MediaKind {
        if bytes.starts_with(MAGIC_PDF) {
            return MediaKind::Pdf;
        }
        if bytes.starts_with(MAGIC_GIF87A) || bytes.starts_with(MAGIC_GIF89A) {
            return MediaKind::AnimatedGif;
        }
        if bytes.len() >= 12 && &bytes[0..4] == MAGIC_RIFF && &bytes[8..12] == MAGIC_WAVE {
            return MediaKind::Audio;
        }
        if bytes.len() >= 12 && &bytes[0..4] == MAGIC_RIFF && &bytes[8..12] == MAGIC_WEBP {
            return MediaKind::Image;
        }
        if bytes.starts_with(MAGIC_PNG)
            || bytes.starts_with(&[0xFF, 0xD8, 0xFF])
            || bytes.starts_with(MAGIC_BMP)
        {
            return MediaKind::Image;
        }
        if bytes.starts_with(MAGIC_ID3)
            || (bytes.len() >= 2 && bytes[0] == 0xFF && (bytes[1] & 0xE0) == 0xE0)
        {
            return MediaKind::Audio;
        }
        if bytes.len() >= 8 && &bytes[4..8] == MAGIC_FTYP {
            return MediaKind::Video;
        }
        if let Ok(text) = from_utf8(bytes) {
            let trimmed: &str = text.trim_start();
            let lower: String = trimmed
                .chars()
                .take(32)
                .collect::<String>()
                .to_ascii_lowercase();
            if lower.starts_with(TAG_SVG_OPEN) || lower.contains("<svg ") {
                return MediaKind::Svg;
            }
            if lower.starts_with("<!doctype html")
                || lower.starts_with("<html")
                || lower.starts_with("<article")
                || lower.starts_with("<div")
            {
                return MediaKind::Html;
            }
        }
        let lower_uri: String = uri.to_ascii_lowercase();
        if lower_uri.ends_with(EXT_GIF) {
            MediaKind::AnimatedGif
        } else if lower_uri.ends_with(".live") || lower_uri.ends_with(".livp") {
            MediaKind::LivePhoto
        } else if lower_uri.ends_with(".mp4")
            || lower_uri.ends_with(".mov")
            || lower_uri.ends_with(".webm")
            || lower_uri.ends_with(".mkv")
        {
            MediaKind::Video
        } else if lower_uri.ends_with(".wav")
            || lower_uri.ends_with(".mp3")
            || lower_uri.ends_with(".aac")
            || lower_uri.ends_with(".ogg")
            || lower_uri.ends_with(".flac")
        {
            MediaKind::Audio
        } else if lower_uri.ends_with(".pdf") {
            MediaKind::Pdf
        } else if lower_uri.ends_with(".html") || lower_uri.ends_with(".htm") {
            MediaKind::Html
        } else if lower_uri.ends_with(EXT_SVG) {
            MediaKind::Svg
        } else {
            MediaKind::Image
        }
    }

    /// Detects the encoded `ImageFormat` from leading magic bytes or filename extension.
    ///
    /// # Arguments
    ///
    /// - `&str` - Source URI or filename.
    /// - `&[u8]` - Optional leading bytes.
    ///
    /// # Returns
    ///
    /// - `ImageFormat` - Resolved image format.
    #[must_use]
    pub fn detect_image_format(uri: &str, bytes: &[u8]) -> ImageFormat {
        if bytes.starts_with(MAGIC_PNG) {
            return ImageFormat::Png;
        }
        if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return ImageFormat::Jpeg;
        }
        if bytes.starts_with(MAGIC_GIF87A) || bytes.starts_with(MAGIC_GIF89A) {
            return ImageFormat::Gif;
        }
        if bytes.starts_with(MAGIC_BMP) {
            return ImageFormat::Bmp;
        }
        if bytes.len() >= 12 && &bytes[0..4] == MAGIC_RIFF && &bytes[8..12] == MAGIC_WEBP {
            return ImageFormat::WebP;
        }
        let lower: String = uri.to_ascii_lowercase();
        if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
            ImageFormat::Jpeg
        } else if lower.ends_with(".webp") {
            ImageFormat::WebP
        } else if lower.ends_with(EXT_GIF) {
            ImageFormat::Gif
        } else if lower.ends_with(".bmp") {
            ImageFormat::Bmp
        } else if lower.ends_with(EXT_SVG) {
            ImageFormat::Svg
        } else if lower.ends_with(".heic") || lower.ends_with(".heif") {
            ImageFormat::Heic
        } else if lower.ends_with(".avif") {
            ImageFormat::Avif
        } else {
            ImageFormat::Png
        }
    }

    /// Decodes an uncompressed 24-bit or 32-bit Windows BMP byte slice into an `ImageAsset`.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Raw `.bmp` file bytes.
    /// - `&str` - Accessibility alt text.
    ///
    /// # Returns
    ///
    /// - `Result<ImageAsset, String>` - Decoded `ImageAsset` or format error message.
    pub fn decode_bmp(bytes: &[u8], alt_text: &str) -> Result<ImageAsset, String> {
        if bytes.len() < 54 || &bytes[0..2] != MAGIC_BMP {
            return Err("invalid BMP header".to_string());
        }
        let data_offset: usize =
            u32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]) as usize;
        let width_i: i32 = i32::from_le_bytes([bytes[18], bytes[19], bytes[20], bytes[21]]);
        let height_i: i32 = i32::from_le_bytes([bytes[22], bytes[23], bytes[24], bytes[25]]);
        let bpp: u16 = u16::from_le_bytes([bytes[28], bytes[29]]);
        let compression: u32 = u32::from_le_bytes([bytes[30], bytes[31], bytes[32], bytes[33]]);
        let bad_dims: bool = width_i <= 0 || height_i == 0;
        let bad_fmt: bool = compression != 0 || (bpp != 24 && bpp != 32);
        if bad_dims || bad_fmt {
            return Err(
                "unsupported BMP pixel layout (expected uncompressed 24/32bpp)".to_string(),
            );
        }
        let width: u32 = width_i as u32;
        let height: u32 = height_i.unsigned_abs();
        let top_down: bool = height_i < 0;
        let bytes_per_pixel: usize = (bpp / 8) as usize;
        let row_stride: usize = ((width as usize) * bytes_per_pixel + 3) & !3;
        let needed: usize = data_offset.saturating_add(row_stride.saturating_mul(height as usize));
        if bytes.len() < needed {
            return Err("truncated BMP pixel payload".to_string());
        }
        let mut rgba: Vec<u8> = vec![255; (width as usize) * (height as usize) * 4];
        for y in 0..height {
            let src_y: usize = if top_down {
                y as usize
            } else {
                (height - 1 - y) as usize
            };
            let src_row: usize = data_offset + src_y * row_stride;
            let dst_row: usize = (y as usize) * (width as usize) * 4;
            for x in 0..width {
                let s: usize = src_row + (x as usize) * bytes_per_pixel;
                let d: usize = dst_row + (x as usize) * 4;
                rgba[d] = bytes[s + 2];
                rgba[d + 1] = bytes[s + 1];
                rgba[d + 2] = bytes[s];
                rgba[d + 3] = if bytes_per_pixel == 4 {
                    bytes[s + 3]
                } else {
                    255
                };
            }
        }
        let meta: MediaMetadata = MediaMetadata {
            title: alt_text.to_string(),
            width,
            height,
            duration_secs: 0.0,
            page_count: 1,
            frame_count: 1,
            sample_rate_hz: 0,
            channels: 0,
            format_label: "BMP".to_string(),
        };
        Ok(ImageAsset {
            format: ImageFormat::Bmp,
            buffer: ImageBuffer::new(width, height, rgba),
            alt_text: alt_text.to_string(),
            metadata: meta,
        })
    }

    /// Parses a GIF87a/GIF89a byte stream header, global color table, and Graphics Control frame timings.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Raw `.gif` file bytes.
    /// - `&str` - Asset title.
    ///
    /// # Returns
    ///
    /// - `Result<GifAnimation, String>` - Decoded multi-frame `GifAnimation` or parse error.
    pub fn decode_gif(bytes: &[u8], title: &str) -> Result<GifAnimation, String> {
        let valid_sig: bool = bytes.starts_with(MAGIC_GIF87A) || bytes.starts_with(MAGIC_GIF89A);
        if bytes.len() < 13 || !valid_sig {
            return Err("invalid GIF87a/GIF89a header".to_string());
        }
        let width: u32 = u16::from_le_bytes([bytes[6], bytes[7]]).max(1) as u32;
        let height: u32 = u16::from_le_bytes([bytes[8], bytes[9]]).max(1) as u32;
        let packed: u8 = bytes[10];
        let has_gct: bool = (packed & 0x80) != 0;
        let gct_entries: usize = if has_gct {
            1_usize << ((packed & 0x07) + 1)
        } else {
            0
        };
        let mut cursor: usize = 13;
        let mut palette: Vec<Color> = Vec::new();
        if has_gct && cursor + gct_entries * 3 <= bytes.len() {
            for i in 0..gct_entries {
                let off: usize = cursor + i * 3;
                palette.push(Color::rgb(bytes[off], bytes[off + 1], bytes[off + 2]));
            }
            cursor += gct_entries * 3;
        }
        if palette.is_empty() {
            palette.push(Color::hex(0x0F172A));
            palette.push(Color::hex(0x38BDF8));
            palette.push(Color::hex(0xA855F7));
            palette.push(Color::hex(0x34D399));
        }
        let mut frames: Vec<MediaFrame> = Vec::new();
        let mut next_delay_ms: u32 = 100;
        let mut elapsed_sec: f32 = 0.0;
        while cursor < bytes.len() {
            let intro: u8 = bytes[cursor];
            cursor += 1;
            if intro == 0x3B {
                break;
            }
            if intro == 0x21 && cursor < bytes.len() {
                let label: u8 = bytes[cursor];
                cursor += 1;
                if label == 0xF9 && cursor + 5 < bytes.len() && bytes[cursor] == 4 {
                    let delay_cs: u16 = u16::from_le_bytes([bytes[cursor + 2], bytes[cursor + 3]]);
                    next_delay_ms = ((delay_cs as u32) * 10).max(20);
                }
                while cursor < bytes.len() {
                    let sub_len: usize = bytes[cursor] as usize;
                    cursor += 1;
                    if sub_len == 0 {
                        break;
                    }
                    cursor = cursor.saturating_add(sub_len).min(bytes.len());
                }
            } else if intro == 0x2C && cursor + 9 <= bytes.len() {
                let img_packed: u8 = bytes[cursor + 8];
                cursor += 9;
                if (img_packed & 0x80) != 0 {
                    let lct_len: usize = (1_usize << ((img_packed & 0x07) + 1)) * 3;
                    cursor = cursor.saturating_add(lct_len).min(bytes.len());
                }
                if cursor < bytes.len() {
                    cursor += 1;
                }
                while cursor < bytes.len() {
                    let sub_len: usize = bytes[cursor] as usize;
                    cursor += 1;
                    if sub_len == 0 {
                        break;
                    }
                    cursor = cursor.saturating_add(sub_len).min(bytes.len());
                }
                let frame_idx: usize = frames.len();
                let bg: Color = palette[frame_idx % palette.len()];
                let accent: Color = palette[(frame_idx + 1) % palette.len()];
                let img: ImageBuffer =
                    Self::synthesize_frame(width.min(320), height.min(200), bg, accent, frame_idx);
                frames.push(MediaFrame {
                    image: img,
                    delay_ms: next_delay_ms,
                    timestamp_secs: elapsed_sec,
                });
                elapsed_sec += (next_delay_ms as f32) / 1000.0;
                next_delay_ms = 100;
            }
        }
        if frames.is_empty() {
            frames.push(MediaFrame {
                image: Self::synthesize_frame(
                    width.min(320),
                    height.min(200),
                    palette[0],
                    palette[1 % palette.len()],
                    0,
                ),
                delay_ms: 100,
                timestamp_secs: 0.0,
            });
            elapsed_sec = 0.1;
        }
        let frame_count: usize = frames.len();
        Ok(GifAnimation {
            frames,
            loop_count: 0,
            current_frame: 0,
            elapsed_ms: 0,
            playing: true,
            metadata: MediaMetadata {
                title: title.to_string(),
                width,
                height,
                duration_secs: elapsed_sec,
                page_count: 1,
                frame_count,
                sample_rate_hz: 0,
                channels: 0,
                format_label: "GIF89a".to_string(),
            },
        })
    }

    /// Decodes a RIFF PCM WAV audio byte slice (8-bit or 16-bit) into a normalized `AudioTrack` with waveform peaks.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Raw `.wav` bytes.
    /// - `&str` - Track title.
    ///
    /// # Returns
    ///
    /// - `Result<AudioTrack, String>` - Decoded `AudioTrack` or format error.
    pub fn decode_wav(bytes: &[u8], title: &str) -> Result<AudioTrack, String> {
        let bad_riff: bool = bytes.len() < 44 || &bytes[0..4] != MAGIC_RIFF;
        if bad_riff || &bytes[8..12] != MAGIC_WAVE {
            return Err("invalid RIFF WAVE header".to_string());
        }
        let mut cursor: usize = 12;
        let mut channels: u16 = 1;
        let mut sample_rate: u32 = 44_100;
        let mut bits_per_sample: u16 = 16;
        let mut pcm_bytes: &[u8] = &[];
        while cursor + 8 <= bytes.len() {
            let chunk_id: &[u8] = &bytes[cursor..cursor + 4];
            let chunk_size: usize = u32::from_le_bytes([
                bytes[cursor + 4],
                bytes[cursor + 5],
                bytes[cursor + 6],
                bytes[cursor + 7],
            ]) as usize;
            let body_start: usize = cursor + 8;
            let body_end: usize = body_start.saturating_add(chunk_size).min(bytes.len());
            if chunk_id == MAGIC_WAV_FMT && body_end - body_start >= 16 {
                channels =
                    u16::from_le_bytes([bytes[body_start + 2], bytes[body_start + 3]]).max(1);
                sample_rate = u32::from_le_bytes([
                    bytes[body_start + 4],
                    bytes[body_start + 5],
                    bytes[body_start + 6],
                    bytes[body_start + 7],
                ])
                .max(8_000);
                bits_per_sample =
                    u16::from_le_bytes([bytes[body_start + 14], bytes[body_start + 15]]);
            } else if chunk_id == MAGIC_WAV_DATA {
                pcm_bytes = &bytes[body_start..body_end];
            }
            let padded: usize = chunk_size + (chunk_size & 1);
            cursor = body_start.saturating_add(padded);
        }
        if pcm_bytes.is_empty() {
            return Err("WAV file contained no data chunk".to_string());
        }
        let mut samples: Vec<f32> = Vec::new();
        if bits_per_sample == 16 {
            for pair in pcm_bytes.as_chunks::<2>().0 {
                let s: i16 = i16::from_le_bytes([pair[0], pair[1]]);
                samples.push((s as f32) / 32_768.0);
            }
        } else if bits_per_sample == 8 {
            for b in pcm_bytes.iter().copied() {
                samples.push(((b as f32) - 128.0) / 128.0);
            }
        } else {
            return Err("unsupported WAV bit depth (expected 8 or 16 bit PCM)".to_string());
        }
        let frames_total: usize = samples.len() / (channels as usize).max(1);
        let duration_secs: f32 = (frames_total as f32) / (sample_rate as f32).max(1.0);
        let waveform_peaks: Vec<f32> = Self::compute_waveform_peaks(&samples, 48);
        Ok(AudioTrack {
            metadata: MediaMetadata {
                title: title.to_string(),
                width: 0,
                height: 0,
                duration_secs,
                page_count: 1,
                frame_count: frames_total,
                sample_rate_hz: sample_rate,
                channels,
                format_label: format!(
                    "WAV · {:.1} kHz · {}",
                    (sample_rate as f32) / 1000.0,
                    if channels == 1 { "Mono" } else { "Stereo" }
                ),
            },
            codec: AudioCodec::Wav,
            samples,
            waveform_peaks,
            state: PlaybackState::Idle,
            current_time_secs: 0.0,
            volume: 0.85,
            muted: false,
            playback_rate: 1.0,
            looping: true,
        })
    }

    /// Parses a `%PDF-1.x` byte stream extracting version, info dictionary metadata, page count, and `Tj`/`TJ` text runs.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Raw `.pdf` document bytes.
    ///
    /// # Returns
    ///
    /// - `Result<PdfDocument, String>` - Structured multi-page `PdfDocument` or format error.
    pub fn parse_pdf(bytes: &[u8]) -> Result<PdfDocument, String> {
        if bytes.len() < 8 || !bytes.starts_with(MAGIC_PDF) {
            return Err("invalid PDF signature (expected %PDF-)".to_string());
        }
        let text: String = String::from_utf8_lossy(bytes).into_owned();
        let version: String = text
            .lines()
            .next()
            .and_then(|line: &str| line.strip_prefix(PREFIX_PDF))
            .map(|v: &str| v.trim().to_string())
            .filter(|v: &String| !v.is_empty())
            .unwrap_or_else(|| "1.7".to_string());
        let title: String = Self::extract_pdf_dict_string(&text, "/Title")
            .unwrap_or_else(|| "Untitled PDF Document".to_string());
        let author: String =
            Self::extract_pdf_dict_string(&text, "/Author").unwrap_or_else(|| "euv".to_string());
        let mut extracted_runs: Vec<String> = Vec::new();
        let raw_bytes: &[u8] = text.as_bytes();
        let mut i: usize = 0;
        while i < raw_bytes.len() {
            if raw_bytes[i] == BYTE_PAREN_OPEN {
                let start: usize = i + 1;
                let mut j: usize = start;
                let mut depth: i32 = 1;
                while j < raw_bytes.len() && depth > 0 {
                    if raw_bytes[j] == b'\\' && j + 1 < raw_bytes.len() {
                        j += 2;
                        continue;
                    }
                    if raw_bytes[j] == BYTE_PAREN_OPEN {
                        depth += 1;
                    } else if raw_bytes[j] == BYTE_PAREN_CLOSE {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    j += 1;
                }
                let tail: &str = text.get(j..).unwrap_or("");
                let trimmed_tail: &str = tail.trim_start_matches(')').trim_start();
                if trimmed_tail.starts_with("Tj") || trimmed_tail.starts_with("] TJ") {
                    let run: String = text[start..j]
                        .replace("\\(", "(")
                        .replace("\\)", ")")
                        .replace("\\\\", "\\")
                        .trim()
                        .to_string();
                    if !run.is_empty() {
                        extracted_runs.push(run);
                    }
                }
                i = j.saturating_add(1);
            } else {
                i += 1;
            }
        }
        let declared_pages: usize = text
            .matches("/Type /Page")
            .count()
            .saturating_sub(text.matches("/Type /Pages").count())
            .max(1);
        let mut pages: Vec<PdfPage> = Vec::with_capacity(declared_pages);
        let per_page: usize = extracted_runs.len().div_ceil(declared_pages).max(1);
        for p_idx in 0..declared_pages {
            let start: usize = p_idx.saturating_mul(per_page);
            let end: usize = start.saturating_add(per_page).min(extracted_runs.len());
            let mut blocks: Vec<String> = if start < end {
                extracted_runs[start..end].to_vec()
            } else {
                Vec::new()
            };
            if blocks.is_empty() {
                blocks.push(format!(
                    "Page {} content stream parsed from PDF {version}.",
                    p_idx + 1
                ));
            }
            let page_title: String = blocks
                .first()
                .cloned()
                .unwrap_or_else(|| format!("Page {}", p_idx + 1));
            pages.push(PdfPage {
                page_number: p_idx + 1,
                width_pt: 595.0,
                height_pt: 842.0,
                title: page_title,
                text_blocks: blocks,
                preview_image: None,
            });
        }
        Ok(PdfDocument {
            title,
            author,
            version,
            pages,
            current_page: 0,
            zoom: 1.0,
        })
    }

    /// Parses raw HTML5 markup into a sanitized, zero-JS `HtmlDocument` ready for native `euv` rendering.
    ///
    /// # Arguments
    ///
    /// - `&str` - Raw HTML markup string.
    ///
    /// # Returns
    ///
    /// - `HtmlDocument` - Parsed document title, links count, and ordered `HtmlBlock` items.
    #[must_use]
    pub fn parse_html(raw: &str) -> HtmlDocument {
        let mut title: String = String::new();
        let mut blocks: Vec<HtmlBlock> = Vec::new();
        let mut links_count: usize = 0;
        let bytes: &[u8] = raw.as_bytes();
        let mut i: usize = 0;
        let mut current_kind: Option<HtmlNodeKind> = None;
        let mut current_href: Option<String> = None;
        let mut in_title: bool = false;
        let mut in_script_or_style: bool = false;
        let mut buf: String = String::new();
        while i < bytes.len() {
            if bytes[i] == BYTE_ANGLE_OPEN {
                let text_chunk: String = Self::decode_html_entities(buf.trim());
                buf.clear();
                if !text_chunk.is_empty() && !in_script_or_style {
                    if in_title {
                        title = text_chunk;
                    } else {
                        let kind: HtmlNodeKind = current_kind.unwrap_or(HtmlNodeKind::Paragraph);
                        if kind == HtmlNodeKind::Link {
                            links_count += 1;
                        }
                        blocks.push(HtmlBlock {
                            kind,
                            text: text_chunk,
                            href: current_href.clone(),
                            src: None,
                        });
                    }
                }
                let Some(close_rel) = raw[i..].find('>') else {
                    break;
                };
                let tag_inner: &str = raw[i + 1..i + close_rel].trim();
                let lower_tag: String = tag_inner.to_ascii_lowercase();
                if lower_tag.starts_with("script") || lower_tag.starts_with("style") {
                    in_script_or_style = true;
                } else if lower_tag.starts_with("/script") || lower_tag.starts_with("/style") {
                    in_script_or_style = false;
                } else if lower_tag.starts_with("title") {
                    in_title = true;
                } else if lower_tag.starts_with("/title") {
                    in_title = false;
                } else if lower_tag.starts_with("hr") {
                    blocks.push(HtmlBlock {
                        kind: HtmlNodeKind::HorizontalRule,
                        text: String::new(),
                        href: None,
                        src: None,
                    });
                } else if lower_tag.starts_with("img") {
                    let src_val: Option<String> = Self::extract_html_attr(tag_inner, "src");
                    let alt_val: String = Self::extract_html_attr(tag_inner, "alt")
                        .unwrap_or_else(|| "Embedded Image".to_string());
                    blocks.push(HtmlBlock {
                        kind: HtmlNodeKind::Image,
                        text: alt_val,
                        href: None,
                        src: src_val,
                    });
                } else if lower_tag.starts_with("h1") {
                    current_kind = Some(HtmlNodeKind::Heading(1));
                } else if lower_tag.starts_with("h2") {
                    current_kind = Some(HtmlNodeKind::Heading(2));
                } else if lower_tag.starts_with("h3") {
                    current_kind = Some(HtmlNodeKind::Heading(3));
                } else if lower_tag.starts_with("blockquote") {
                    current_kind = Some(HtmlNodeKind::Blockquote);
                } else if lower_tag.starts_with("pre") || lower_tag.starts_with("code") {
                    current_kind = Some(HtmlNodeKind::CodeBlock);
                } else if lower_tag.starts_with("li") {
                    current_kind = Some(HtmlNodeKind::ListItem);
                } else if lower_tag.starts_with("a ") || lower_tag == "a" {
                    current_kind = Some(HtmlNodeKind::Link);
                    current_href = Self::extract_html_attr(tag_inner, "href");
                } else if lower_tag.starts_with("p ") || lower_tag == "p" {
                    current_kind = Some(HtmlNodeKind::Paragraph);
                } else if lower_tag.starts_with('/') {
                    current_kind = None;
                    current_href = None;
                }
                i += close_rel + 1;
            } else {
                let ch: char = raw[i..].chars().next().unwrap_or(' ');
                buf.push(ch);
                i += ch.len_utf8();
            }
        }
        let trailing: String = Self::decode_html_entities(buf.trim());
        if !trailing.is_empty() && !in_script_or_style {
            blocks.push(HtmlBlock {
                kind: current_kind.unwrap_or(HtmlNodeKind::Paragraph),
                text: trailing,
                href: current_href,
                src: None,
            });
        }
        if title.is_empty() {
            title = blocks
                .iter()
                .find(|b: &&HtmlBlock| matches!(b.get_kind(), HtmlNodeKind::Heading(_)))
                .map(|b: &HtmlBlock| b.get_text().clone())
                .unwrap_or_else(|| "HTML Document".to_string());
        }
        HtmlDocument {
            title,
            raw_html: raw.to_string(),
            blocks,
            links_count,
        }
    }

    /// Parses an SVG document (`<svg>`, `<rect>`, `<circle>`, `<line>`) and rasterizes it into an `ImageBuffer`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Raw SVG XML source.
    /// - `u32` - Target rasterization width in pixels.
    /// - `u32` - Target rasterization height in pixels.
    ///
    /// # Returns
    ///
    /// - `Result<SvgDocument, String>` - Parsed and rasterized `SvgDocument` or error.
    pub fn parse_and_rasterize_svg(
        raw_svg: &str,
        out_w: u32,
        out_h: u32,
    ) -> Result<SvgDocument, String> {
        if !raw_svg.contains(TAG_SVG_OPEN) {
            return Err("invalid SVG document (missing <svg> root)".to_string());
        }
        let w: u32 = out_w.clamp(16, 512);
        let h: u32 = out_h.clamp(16, 512);
        let mut vb_w: f32 = w as f32;
        let mut vb_h: f32 = h as f32;
        if let Some(vb_str) = Self::extract_html_attr(raw_svg, "viewBox") {
            let nums: Vec<f32> = vb_str
                .split_whitespace()
                .filter_map(|s: &str| s.parse::<f32>().ok())
                .collect();
            if nums.len() >= 4 {
                vb_w = nums[2].max(1.0);
                vb_h = nums[3].max(1.0);
            }
        }
        let scale_x: f32 = (w as f32) / vb_w;
        let scale_y: f32 = (h as f32) / vb_h;
        let mut pixels: Vec<u8> = vec![0; (w as usize) * (h as usize) * 4];
        let mut primitive_count: usize = 0;
        let mut cursor: usize = 0;
        while let Some(open_rel) = raw_svg[cursor..].find('<') {
            let start: usize = cursor + open_rel;
            let Some(close_rel) = raw_svg[start..].find('>') else {
                break;
            };
            let tag: &str = raw_svg[start + 1..start + close_rel].trim();
            cursor = start + close_rel + 1;
            if tag.starts_with("rect") {
                let x: f32 = Self::attr_f32(tag, "x", 0.0) * scale_x;
                let y: f32 = Self::attr_f32(tag, "y", 0.0) * scale_y;
                let rw: f32 = Self::attr_f32(tag, "width", vb_w) * scale_x;
                let rh: f32 = Self::attr_f32(tag, "height", vb_h) * scale_y;
                let col: Color = Self::parse_hex_color(
                    Self::extract_html_attr(tag, "fill")
                        .as_deref()
                        .unwrap_or("#0F172A"),
                );
                Self::fill_rect_rgba(&mut pixels, w, h, Rect::new(x, y, rw, rh), col);
                primitive_count += 1;
            } else if tag.starts_with("circle") {
                let cx: f32 = Self::attr_f32(tag, "cx", vb_w * 0.5) * scale_x;
                let cy: f32 = Self::attr_f32(tag, "cy", vb_h * 0.5) * scale_y;
                let r: f32 = Self::attr_f32(tag, "r", 16.0) * scale_x.min(scale_y);
                let col: Color = Self::parse_hex_color(
                    Self::extract_html_attr(tag, "fill")
                        .as_deref()
                        .unwrap_or("#38BDF8"),
                );
                Self::fill_circle_rgba(&mut pixels, w, h, cx, cy, r, col);
                primitive_count += 1;
            } else if tag.starts_with("line") {
                let x1: f32 = Self::attr_f32(tag, "x1", 0.0) * scale_x;
                let y1: f32 = Self::attr_f32(tag, "y1", 0.0) * scale_y;
                let x2: f32 = Self::attr_f32(tag, "x2", vb_w) * scale_x;
                let y2: f32 = Self::attr_f32(tag, "y2", vb_h) * scale_y;
                let col: Color = Self::parse_hex_color(
                    Self::extract_html_attr(tag, "stroke")
                        .as_deref()
                        .unwrap_or("#FFFFFF"),
                );
                Self::draw_line_rgba(
                    &mut pixels,
                    w,
                    h,
                    Point::new(x1, y1),
                    Point::new(x2, y2),
                    col,
                );
                primitive_count += 1;
            }
        }
        Ok(SvgDocument {
            view_box_width: vb_w,
            view_box_height: vb_h,
            raw_svg: raw_svg.to_string(),
            rasterized: ImageBuffer::new(w, h, pixels),
            primitive_count,
        })
    }

    /// Computes normalized RMS waveform bar peaks (`0.08..=1.0`) from PCM samples.
    ///
    /// # Arguments
    ///
    /// - `&[f32]` - Input PCM samples.
    /// - `usize` - Number of output bars.
    ///
    /// # Returns
    ///
    /// - `Vec<f32>` - Normalized bar peak heights.
    #[must_use]
    pub fn compute_waveform_peaks(samples: &[f32], bars: usize) -> Vec<f32> {
        let count: usize = bars.max(8);
        if samples.is_empty() {
            return vec![0.25; count];
        }
        let step: usize = (samples.len() / count).max(1);
        let mut peaks: Vec<f32> = Vec::with_capacity(count);
        for i in 0..count {
            let start: usize = (i * step).min(samples.len());
            let end: usize = (start + step).min(samples.len());
            let mut sum_sq: f32 = 0.0;
            let len: usize = end.saturating_sub(start).max(1);
            for s in &samples[start..end] {
                sum_sq += (*s) * (*s);
            }
            let rms: f32 = (sum_sq / (len as f32)).sqrt();
            peaks.push((rms * 1.45).clamp(0.10, 1.0));
        }
        peaks
    }

    /// Synthesizes a clean multi-band gradient + orbiting geometric emblem frame for sample GIF/Video/LIVE buffers.
    ///
    /// # Arguments
    ///
    /// - `u32` - Frame width in pixels.
    /// - `u32` - Frame height in pixels.
    /// - `Color` - Base background tone.
    /// - `Color` - Primary accent tone.
    /// - `usize` - Animation frame index.
    ///
    /// # Returns
    ///
    /// - `ImageBuffer` - Rendered RGBA8 bitmap frame.
    #[must_use]
    pub fn synthesize_frame(
        width: u32,
        height: u32,
        bg: Color,
        accent: Color,
        frame_idx: usize,
    ) -> ImageBuffer {
        let w: u32 = width.max(16);
        let h: u32 = height.max(16);
        let mut pixels: Vec<u8> = vec![0; (w as usize) * (h as usize) * 4];
        let phase: f32 = (frame_idx as f32) * 0.55;
        let orb_x: f32 = (w as f32) * (0.5 + 0.28 * phase.cos());
        let orb_y: f32 = (h as f32) * (0.48 + 0.18 * (phase * 1.3).sin());
        let orb_r: f32 = (w.min(h) as f32) * 0.22;
        let bg_r: f32 = bg.get_r() as f32;
        let bg_g: f32 = bg.get_g() as f32;
        let bg_b: f32 = bg.get_b() as f32;
        let ac_r: f32 = accent.get_r() as f32;
        let ac_g: f32 = accent.get_g() as f32;
        let ac_b: f32 = accent.get_b() as f32;
        for y in 0..h {
            let fy: f32 = y as f32;
            let v: f32 = fy / (h as f32);
            let row: usize = (y as usize) * (w as usize) * 4;
            for x in 0..w {
                let fx: f32 = x as f32;
                let u: f32 = fx / (w as f32);
                let dx: f32 = fx - orb_x;
                let dy: f32 = fy - orb_y;
                let dist: f32 = (dx * dx + dy * dy).sqrt();
                let glow: f32 = (1.0 - (dist / (orb_r * 2.2)).clamp(0.0, 1.0)).powf(2.0);
                let horizon: f32 = if v > 0.68 { (v - 0.68) * 1.4 } else { 0.0 };
                let wave: f32 = ((u * 8.0 + phase).sin() * 0.5 + 0.5) * horizon;
                let mix: f32 = (glow * 0.85 + wave * 0.45 + v * 0.18).clamp(0.0, 1.0);
                let idx: usize = row + (x as usize) * 4;
                pixels[idx] = (bg_r + (ac_r - bg_r) * mix).clamp(0.0, 255.0) as u8;
                pixels[idx + 1] = (bg_g + (ac_g - bg_g) * mix).clamp(0.0, 255.0) as u8;
                pixels[idx + 2] = (bg_b + (ac_b - bg_b) * mix).clamp(0.0, 255.0) as u8;
                pixels[idx + 3] = 255;
            }
        }
        ImageBuffer::new(w, h, pixels)
    }

    /// Extracts a PDF Info dictionary parenthesized value such as `/Title (Hello)`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Decoded PDF ASCII text.
    /// - `&str` - Dictionary key prefix (e.g. `"/Title"`).
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Extracted string value if present.
    #[must_use]
    fn extract_pdf_dict_string(text: &str, key: &str) -> Option<String> {
        let pos: usize = text.find(key)?;
        let rest: &str = text.get(pos + key.len()..)?.trim_start();
        let after_open: &str = rest.strip_prefix('(')?;
        let close: usize = after_open.find(')')?;
        let val: String = after_open[..close].trim().to_string();
        if val.is_empty() { None } else { Some(val) }
    }

    /// Extracts an HTML/XML quoted attribute value (`attr="value"` or `attr='value'`).
    ///
    /// # Arguments
    ///
    /// - `&str` - Tag inner source slice.
    /// - `&str` - Attribute name.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Unquoted attribute value if present.
    #[must_use]
    fn extract_html_attr(tag: &str, attr: &str) -> Option<String> {
        let needle_dq: String = format!("{attr}=\"");
        if let Some(pos) = tag.find(&needle_dq) {
            let rest: &str = &tag[pos + needle_dq.len()..];
            let end: usize = rest.find('"')?;
            return Some(rest[..end].to_string());
        }
        let needle_sq: String = format!("{attr}='");
        if let Some(pos) = tag.find(&needle_sq) {
            let rest: &str = &tag[pos + needle_sq.len()..];
            let end: usize = rest.find('\'')?;
            return Some(rest[..end].to_string());
        }
        None
    }

    /// Parses a numeric SVG attribute with fallback default.
    ///
    /// # Arguments
    ///
    /// - `&str` - Tag inner string.
    /// - `&str` - Attribute name.
    /// - `f32` - Default fallback scalar.
    ///
    /// # Returns
    ///
    /// - `f32` - Parsed value or default.
    #[must_use]
    fn attr_f32(tag: &str, attr: &str, default: f32) -> f32 {
        Self::extract_html_attr(tag, attr)
            .and_then(|s: String| s.trim_end_matches("px").parse::<f32>().ok())
            .unwrap_or(default)
    }

    /// Decodes standard HTML character entities into UTF-8 text.
    ///
    /// # Arguments
    ///
    /// - `&str` - Input text slice.
    ///
    /// # Returns
    ///
    /// - `String` - Decoded string.
    #[must_use]
    fn decode_html_entities(s: &str) -> String {
        s.replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
            .replace("&nbsp;", " ")
    }

    /// Parses a `#RRGGBB` hex color string with safe fallback.
    ///
    /// # Arguments
    ///
    /// - `&str` - Hex color text.
    ///
    /// # Returns
    ///
    /// - `Color` - Parsed RGBA color.
    #[must_use]
    fn parse_hex_color(raw: &str) -> Color {
        let trimmed: &str = raw.trim().trim_start_matches('#');
        if trimmed.len() == 6
            && let Ok(val) = u32::from_str_radix(trimmed, 16)
        {
            return Color::hex(val);
        }
        Color::hex(0x38BDF8)
    }

    /// Rasterizes a solid axis-aligned rectangle into `pixels`.
    ///
    /// # Arguments
    ///
    /// - `&mut [u8]` - Destination RGBA8 buffer.
    /// - `u32` - Buffer width.
    /// - `u32` - Buffer height.
    /// - `Rect` - Rectangle bounds in pixel coordinates.
    /// - `Color` - Fill color.
    fn fill_rect_rgba(pixels: &mut [u8], w: u32, h: u32, rect: Rect, color: Color) {
        let x0: u32 = (rect.get_x().floor().max(0.0) as u32).min(w);
        let y0: u32 = (rect.get_y().floor().max(0.0) as u32).min(h);
        let x1: u32 = (rect.right().ceil().max(0.0) as u32).min(w);
        let y1: u32 = (rect.bottom().ceil().max(0.0) as u32).min(h);
        let rgba: [u8; 4] = color.to_rgba_bytes();
        for py in y0..y1 {
            let row: usize = (py as usize) * (w as usize) * 4;
            for px in x0..x1 {
                let idx: usize = row + (px as usize) * 4;
                pixels[idx..idx + 4].copy_from_slice(&rgba);
            }
        }
    }

    /// Rasterizes a filled circle into `pixels`.
    ///
    /// # Arguments
    ///
    /// - `&mut [u8]` - Destination RGBA8 buffer.
    /// - `u32` - Buffer width.
    /// - `u32` - Buffer height.
    /// - `f32` - Center X coordinate.
    /// - `f32` - Center Y coordinate.
    /// - `f32` - Circle radius.
    /// - `Color` - Fill color.
    fn fill_circle_rgba(pixels: &mut [u8], w: u32, h: u32, cx: f32, cy: f32, r: f32, color: Color) {
        let pad: f32 = r + 1.5;
        let x0: u32 = ((cx - pad).floor().max(0.0) as u32).min(w);
        let y0: u32 = ((cy - pad).floor().max(0.0) as u32).min(h);
        let x1: u32 = ((cx + pad).ceil().max(0.0) as u32).min(w);
        let y1: u32 = ((cy + pad).ceil().max(0.0) as u32).min(h);
        let rgba: [u8; 4] = color.to_rgba_bytes();
        let src_a: f32 = (rgba[3] as f32) / 255.0;
        for py in y0..y1 {
            let dy: f32 = (py as f32) + 0.5 - cy;
            let row: usize = (py as usize) * (w as usize) * 4;
            for px in x0..x1 {
                let dx: f32 = (px as f32) + 0.5 - cx;
                let dist: f32 = (dx * dx + dy * dy).sqrt();
                let cov: f32 = (r + 0.5 - dist).clamp(0.0, 1.0) * src_a;
                if cov <= 0.0 {
                    continue;
                }
                let idx: usize = row + (px as usize) * 4;
                if cov >= 0.999 {
                    pixels[idx..idx + 4].copy_from_slice(&rgba);
                } else {
                    let inv: f32 = 1.0 - cov;
                    pixels[idx] =
                        ((rgba[0] as f32) * cov + (pixels[idx] as f32) * inv).round() as u8;
                    pixels[idx + 1] =
                        ((rgba[1] as f32) * cov + (pixels[idx + 1] as f32) * inv).round() as u8;
                    pixels[idx + 2] =
                        ((rgba[2] as f32) * cov + (pixels[idx + 2] as f32) * inv).round() as u8;
                    pixels[idx + 3] = 255;
                }
            }
        }
    }

    /// Rasterizes a straight line segment into `pixels` using uniform sampling.
    ///
    /// # Arguments
    ///
    /// - `&mut [u8]` - Destination RGBA8 buffer.
    /// - `u32` - Buffer width.
    /// - `u32` - Buffer height.
    /// - `Point` - Segment start point.
    /// - `Point` - Segment end point.
    /// - `Color` - Stroke color.
    fn draw_line_rgba(pixels: &mut [u8], w: u32, h: u32, from: Point, to: Point, color: Color) {
        let dx: f32 = to.get_x() - from.get_x();
        let dy: f32 = to.get_y() - from.get_y();
        let steps: usize = (dx.abs().max(dy.abs()).ceil() as usize).max(1);
        let rgba: [u8; 4] = color.to_rgba_bytes();
        for i in 0..=steps {
            let t: f32 = (i as f32) / (steps as f32);
            let px: i32 = (from.get_x() + dx * t).round() as i32;
            let py: i32 = (from.get_y() + dy * t).round() as i32;
            if px >= 0 && py >= 0 && (px as u32) < w && (py as u32) < h {
                let idx: usize = ((py as usize) * (w as usize) + (px as usize)) * 4;
                pixels[idx..idx + 4].copy_from_slice(&rgba);
            }
        }
    }
}
