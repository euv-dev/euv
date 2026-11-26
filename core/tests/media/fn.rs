use super::*;

#[test]
fn media_codec_registry_sniffs_and_decodes_image_bmp_gif_live_video_audio_wav_pdf_html_and_svg() {
    assert_eq!(
        MediaCodecRegistry::detect_media_kind("a.bin", b"%PDF-1.7\n"),
        MediaKind::Pdf
    );
    assert_eq!(
        MediaCodecRegistry::detect_media_kind("a.bin", b"GIF89a\x01\x00"),
        MediaKind::AnimatedGif
    );
    assert_eq!(
        MediaCodecRegistry::detect_media_kind("a.bin", b"RIFF\x24\x00\x00\x00WAVE"),
        MediaKind::Audio
    );
    assert_eq!(
        MediaCodecRegistry::detect_media_kind("clip.mp4", b"\x00\x00\x00\x18ftypmp42"),
        MediaKind::Video
    );
    assert_eq!(
        MediaCodecRegistry::detect_media_kind("photo.livp", &[]),
        MediaKind::LivePhoto
    );
    assert_eq!(
        MediaCodecRegistry::detect_media_kind("doc.html", b"<!DOCTYPE html><h1>Hi</h1>"),
        MediaKind::Html
    );
    assert_eq!(
        MediaCodecRegistry::detect_media_kind("icon.svg", b"<svg viewBox=\"0 0 10 10\"></svg>"),
        MediaKind::Svg
    );
    assert_eq!(
        MediaCodecRegistry::detect_image_format("hero.webp", b"RIFF\x00\x00\x00\x00WEBP"),
        ImageFormat::WebP
    );
    let mut bmp: Vec<u8> = vec![0; 54 + 16];
    bmp[0] = b'B';
    bmp[1] = b'M';
    bmp[10] = 54;
    bmp[14] = 40;
    bmp[18] = 2;
    bmp[22] = 2;
    bmp[26] = 1;
    bmp[28] = 24;
    bmp[54] = 0x10;
    bmp[55] = 0x80;
    bmp[56] = 0xFF;
    let decoded_bmp: ImageAsset = MediaCodecRegistry::decode_bmp(&bmp, "test_bmp").unwrap();
    assert_eq!(decoded_bmp.get_format(), ImageFormat::Bmp);
    assert_eq!(decoded_bmp.get_buffer().get_width(), 2);
    assert_eq!(decoded_bmp.get_buffer().get_height(), 2);
    assert_eq!(decoded_bmp.get_buffer().get_pixels()[8], 0xFF);
    let mut gif_bytes: Vec<u8> = Vec::new();
    gif_bytes.extend_from_slice(b"GIF89a");
    gif_bytes.extend_from_slice(&32_u16.to_le_bytes());
    gif_bytes.extend_from_slice(&24_u16.to_le_bytes());
    gif_bytes.push(0x80);
    gif_bytes.push(0);
    gif_bytes.push(0);
    gif_bytes.extend_from_slice(&[15, 23, 42, 56, 189, 248]);
    gif_bytes.extend_from_slice(&[0x21, 0xF9, 0x04, 0x00, 0x08, 0x00, 0x00, 0x00]);
    gif_bytes.extend_from_slice(&[0x2C, 0, 0, 0, 0, 32, 0, 24, 0, 0, 2, 0]);
    gif_bytes.extend_from_slice(&[0x21, 0xF9, 0x04, 0x00, 0x0C, 0x00, 0x00, 0x00]);
    gif_bytes.extend_from_slice(&[0x2C, 0, 0, 0, 0, 32, 0, 24, 0, 0, 2, 0]);
    gif_bytes.push(0x3B);
    let mut gif: GifAnimation = MediaCodecRegistry::decode_gif(&gif_bytes, "test_gif").unwrap();
    assert_eq!(gif.get_frames().len(), 2);
    assert_eq!(gif.get_frames()[0].get_delay_ms(), 80);
    assert_eq!(gif.get_frames()[1].get_delay_ms(), 120);
    assert!(gif.step_ms(90));
    assert_eq!(gif.get_current_frame(), 1);
    let mut live: LivePhotoAsset = LivePhotoAsset::sample_live_photo();
    let still_px: Vec<u8> = live.active_image().get_pixels().clone();
    live.set_motion_playing(true);
    live.step_ms(120);
    let motion_px: Vec<u8> = live.active_image().get_pixels().clone();
    assert_ne!(still_px, motion_px);
    let mut video: VideoTrack = VideoTrack::sample_hdr_video();
    video.toggle_play();
    video.step(3.5);
    assert!(video.get_current_time_secs() >= 3.4);
    assert!(!video.active_caption().is_empty());
    let mut audio: AudioTrack = AudioTrack::sample_synth_wav();
    assert_eq!(audio.get_codec(), AudioCodec::Wav);
    assert_eq!(audio.get_waveform_peaks().len(), 48);
    assert!(audio.get_metadata().get_duration_secs() > 7.5);
    audio.toggle_play();
    audio.step(2.0);
    assert!((audio.get_current_time_secs() - 2.0).abs() < 0.05);
    let mut pdf: PdfDocument = PdfDocument::sample_pdf();
    assert_eq!(pdf.get_version(), "1.7");
    assert_eq!(pdf.get_pages().len(), 3);
    pdf.next_page();
    assert_eq!(pdf.get_current_page(), 1);
    pdf.adjust_zoom(0.25);
    assert!((pdf.get_zoom() - 1.25).abs() < 0.01);
    let html: HtmlDocument = HtmlDocument::sample_html();
    assert_eq!(html.get_links_count(), 1);
    assert!(html.get_blocks().len() >= 6);
    assert!(
        html.get_blocks()
            .iter()
            .any(|b: &HtmlBlock| b.get_kind() == HtmlNodeKind::Heading(1))
    );
    let svg: SvgDocument = SvgDocument::sample_svg();
    assert_eq!(svg.get_primitive_count(), 5);
    assert_eq!(svg.get_rasterized().get_width(), 480);
}
