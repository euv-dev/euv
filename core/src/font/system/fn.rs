use super::*;

/// Returns `true` when `ch` belongs to wide CJK, Kana, Hangul, or fullwidth Unicode blocks.
///
/// # Arguments
///
/// - `char` - Unicode scalar value to classify.
///
/// # Returns
///
/// - `bool` - True when the character occupies a full em advance width.
#[must_use]
pub fn is_wide_unicode(ch: char) -> bool {
    let cp: u32 = ch as u32;
    (CJK_UNIFIED_START..=CJK_UNIFIED_END).contains(&cp)
        || (CJK_EXT_A_START..=CJK_EXT_A_END).contains(&cp)
        || (CJK_PUNCT_START..=CJK_PUNCT_END).contains(&cp)
        || (FULLWIDTH_START..=FULLWIDTH_END).contains(&cp)
        || (HANGUL_START..=HANGUL_END).contains(&cp)
}

/// Deterministically rasterizes non-ASCII / CJK glyphs into distinct, crisp stroke patterns.
///
/// # Arguments
///
/// - `GlyphKey` - Quantized glyph descriptor.
///
/// # Returns
///
/// - `RasterizedGlyph` - Bit-identical cross-platform coverage bitmap for `key`.
#[must_use]
pub fn rasterize_unicode_deterministic(key: GlyphKey) -> RasterizedGlyph {
    let size_px: f32 = (key.get_size_tenths() as f32) * 0.1;
    let out_h: u32 = (size_px * 0.88).round().max(8.0) as u32;
    let wide: bool = is_wide_unicode(key.get_codepoint());
    let out_w: u32 = if wide {
        out_h
    } else {
        ((out_h as f32) * 0.68).round().max(6.0) as u32
    };
    let advance: u32 = out_w + ((size_px * 0.10).round().max(1.0) as u32);
    let mut coverage: Vec<u8> = vec![0; (out_w as usize) * (out_h as usize)];
    let mut seed: u32 = (key.get_codepoint() as u32).wrapping_mul(0x9E37_79B1);
    let stroke: u32 = if key.get_bold() { 2 } else { 1 };
    let grid: u32 = 7;
    for gy in 0..grid {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let row_mask: u32 = (seed >> 20) & 0x7F;
        let y0: u32 = (gy * out_h) / grid;
        let y1: u32 = (((gy + 1) * out_h) / grid).min(out_h);
        for gx in 0..grid {
            let mirror_x: u32 = if gx < 4 { gx } else { grid - 1 - gx };
            let bit_on: bool = ((row_mask >> mirror_x) & 1) != 0 || gy == 0 || gy == grid - 1;
            if !bit_on {
                continue;
            }
            let x0: u32 = (gx * out_w) / grid;
            let x1: u32 = (x0 + stroke).min(((gx + 1) * out_w) / grid).min(out_w);
            for py in y0..y1 {
                for px in x0..x1 {
                    coverage[(py as usize) * (out_w as usize) + (px as usize)] = 255;
                }
            }
        }
    }
    RasterizedGlyph::new(out_w, out_h, advance, coverage)
}

/// Probes the active host operating system's native font rasterization capability.
///
/// # Returns
///
/// - `SystemFontInfo` - Resolved native font backend and feature flags.
#[must_use]
pub fn probe_system_font() -> SystemFontInfo {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        SystemFontInfo::new(
            FontRenderBackend::CoreText,
            FONT_FAMILY_APPLE_SYSTEM.to_string(),
            true,
            true,
        )
    }
    #[cfg(target_os = "windows")]
    {
        SystemFontInfo::new(
            FontRenderBackend::Win32Gdi,
            FONT_FAMILY_WINDOWS_SYSTEM.to_string(),
            true,
            true,
        )
    }
    #[cfg(target_os = "linux")]
    {
        let has_ft: bool = freetype_runtime_available();
        if has_ft {
            SystemFontInfo::new(
                FontRenderBackend::FreeType,
                FONT_FAMILY_LINUX_SYSTEM.to_string(),
                true,
                true,
            )
        } else {
            SystemFontInfo::new(
                FontRenderBackend::DeterministicBitmap,
                FONT_FAMILY_DETERMINISTIC.to_string(),
                true,
                false,
            )
        }
    }
    #[cfg(target_os = "android")]
    {
        let has_ft: bool = freetype_runtime_available();
        if has_ft {
            SystemFontInfo::new(
                FontRenderBackend::FreeType,
                FONT_FAMILY_ANDROID_SYSTEM.to_string(),
                true,
                true,
            )
        } else {
            SystemFontInfo::new(
                FontRenderBackend::DeterministicBitmap,
                FONT_FAMILY_DETERMINISTIC.to_string(),
                true,
                false,
            )
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        SystemFontInfo::new(
            FontRenderBackend::WebCanvas,
            FONT_FAMILY_WEB_SYSTEM.to_string(),
            true,
            true,
        )
    }
    #[cfg(not(any(target_arch = "wasm32", unix, windows)))]
    {
        SystemFontInfo::new(
            FontRenderBackend::DeterministicBitmap,
            FONT_FAMILY_DETERMINISTIC.to_string(),
            true,
            false,
        )
    }
}

/// Rasterizes a glyph through the active OS native font API, falling back to `None` if unavailable.
///
/// # Arguments
///
/// - `GlyphKey` - Quantized glyph descriptor.
///
/// # Returns
///
/// - `Option<RasterizedGlyph>` - Anti-aliased native OS glyph bitmap when available.
#[must_use]
pub fn rasterize_native_glyph(key: GlyphKey) -> Option<RasterizedGlyph> {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        rasterize_coretext_glyph(key)
    }
    #[cfg(target_os = "windows")]
    {
        rasterize_win32_gdi_glyph(key)
    }
    #[cfg(target_arch = "wasm32")]
    {
        rasterize_web_canvas_glyph(key)
    }
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let _: GlyphKey = key;
        None
    }
    #[cfg(not(any(target_arch = "wasm32", unix, windows)))]
    {
        let _: GlyphKey = key;
        None
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[link(name = "CoreText", kind = "framework")]
unsafe extern "C" {
    /// Creates an Apple system UI `CTFontRef` at `size` points.
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `f64` - A 64-bit float (`f64`).
    /// - `*const c_void` - A `*const c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn CTFontCreateUIFontForLanguage(
        ui_type: u32,
        size: f64,
        language: *const c_void,
    ) -> *mut c_void;
    /// Resolves a fallback `CTFontRef` covering the UTF-16 `range` in `string`.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `*const c_void` - A `*const c_void` parameter.
    /// - `CtRange` - A `CtRange` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn CTFontCreateForString(
        current_font: *mut c_void,
        string: *const c_void,
        range: CtRange,
    ) -> *mut c_void;
    /// Maps UTF-16 code units to CoreText glyph indices.
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - A `*const c_void` parameter.
    /// - `*const u16` - A `*const u16` parameter.
    /// - `*mut u16` - A `*mut u16` parameter.
    /// - `isize` - A `isize` parameter.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    fn CTFontGetGlyphsForCharacters(
        font: *const c_void,
        characters: *const u16,
        glyphs: *mut u16,
        count: isize,
    ) -> bool;
    /// Queries typographic advance metrics for CoreText glyph indices.
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - A `*const c_void` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `*const u16` - A `*const u16` parameter.
    /// - `*mut CtSize` - A `*mut CtSize` parameter.
    /// - `isize` - A `isize` parameter.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    fn CTFontGetAdvancesForGlyphs(
        font: *const c_void,
        orientation: u32,
        glyphs: *const u16,
        advances: *mut CtSize,
        count: isize,
    ) -> f64;
    /// Draws positioned CoreText glyphs into a CoreGraphics bitmap context.
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - A `*const c_void` parameter.
    /// - `*const u16` - A `*const u16` parameter.
    /// - `*const CtPoint` - A `*const CtPoint` parameter.
    /// - `usize` - A non-negative integer (`usize`).
    /// - `*mut c_void` - A `*mut c_void` parameter.
    fn CTFontDrawGlyphs(
        font: *const c_void,
        glyphs: *const u16,
        positions: *const CtPoint,
        count: usize,
        context: *mut c_void,
    );
    /// Returns the typographic ascent metric of a CoreText font.
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - A CoreText `CTFontRef` handle.
    ///
    /// # Returns
    ///
    /// - `f64` - Ascent distance above the baseline in points.
    fn CTFontGetAscent(font: *const c_void) -> f64;
    /// Returns the typographic descent metric of a CoreText font.
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - A CoreText `CTFontRef` handle.
    ///
    /// # Returns
    ///
    /// - `f64` - Descent distance below the baseline in points.
    fn CTFontGetDescent(font: *const c_void) -> f64;
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    /// Allocates a `CFStringRef` from a UTF-16 code unit buffer.
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - A `*const c_void` parameter.
    /// - `*const u16` - A `*const u16` parameter.
    /// - `isize` - A `isize` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn CFStringCreateWithCharacters(
        alloc: *const c_void,
        chars: *const u16,
        num_chars: isize,
    ) -> *mut c_void;
    /// Decrements the reference count of a CoreFoundation object.
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - A `*const c_void` parameter.
    fn CFRelease(cf: *const c_void);
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    /// Creates a device-dependent grayscale `CGColorSpaceRef`.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn CGColorSpaceCreateDeviceGray() -> *mut c_void;
    /// Releases a `CGColorSpaceRef`.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    fn CGColorSpaceRelease(space: *mut c_void);
    /// Creates an offscreen CoreGraphics bitmap drawing context.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `usize` - A non-negative integer (`usize`).
    /// - `usize` - A non-negative integer (`usize`).
    /// - `usize` - A non-negative integer (`usize`).
    /// - `usize` - A non-negative integer (`usize`).
    /// - `*const c_void` - A `*const c_void` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn CGBitmapContextCreate(
        data: *mut c_void,
        width: usize,
        height: usize,
        bits_per_component: usize,
        bytes_per_row: usize,
        space: *const c_void,
        bitmap_info: u32,
    ) -> *mut c_void;
    /// Sets the grayscale fill intensity and alpha for a CoreGraphics context.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `f64` - A 64-bit float (`f64`).
    fn CGContextSetGrayFillColor(ctx: *mut c_void, gray: f64, alpha: f64);
    /// Enables or disables anti-aliasing on a CoreGraphics context.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `bool` - A boolean (`bool`).
    fn CGContextSetShouldAntialias(ctx: *mut c_void, should_antialias: bool);
    /// Allows or disallows anti-aliasing on a CoreGraphics context.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - CoreGraphics context pointer.
    /// - `bool` - Whether anti-aliasing is permitted.
    fn CGContextSetAllowsAntialiasing(ctx: *mut c_void, allows_antialiasing: bool);
    /// Allows or disallows subpixel font smoothing on a CoreGraphics context.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - CoreGraphics context pointer.
    /// - `bool` - Whether font smoothing is permitted.
    fn CGContextSetAllowsFontSmoothing(ctx: *mut c_void, allows_font_smoothing: bool);
    /// Enables or disables font smoothing on a CoreGraphics context.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - CoreGraphics context pointer.
    /// - `bool` - Whether font smoothing should be active.
    fn CGContextSetShouldSmoothFonts(ctx: *mut c_void, should_smooth_fonts: bool);
    /// Releases a CoreGraphics context.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    fn CGContextRelease(ctx: *mut c_void);
}

/// Rasterizes a Unicode or ASCII glyph using Apple CoreText + CoreGraphics (`SF Pro` / `PingFang SC`).
///
/// # Arguments
///
/// - `GlyphKey` - Quantized glyph descriptor.
///
/// # Returns
///
/// - `Option<RasterizedGlyph>` - Anti-aliased coverage bitmap or `None` on FFI failure.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn rasterize_coretext_glyph(key: GlyphKey) -> Option<RasterizedGlyph> {
    let size_px: f64 = ((key.get_size_tenths() as f64) * 0.1).max(6.0);
    let mut utf16_buf: [u16; 2] = [0; 2];
    let encoded: &[u16] = key.get_codepoint().encode_utf16(&mut utf16_buf);
    let count: isize = encoded.len() as isize;
    let ui_font_kind: u32 = if key.get_bold() {
        CT_FONT_UI_EMPHASIZED_SYSTEM
    } else {
        CT_FONT_UI_SYSTEM
    };
    unsafe {
        let base_font: *mut c_void = CTFontCreateUIFontForLanguage(ui_font_kind, size_px, null());
        if base_font.is_null() {
            return None;
        }
        let base_ascent: f64 = CTFontGetAscent(base_font).max(size_px * 0.80);
        let base_descent: f64 = CTFontGetDescent(base_font).max(size_px * 0.24);
        let cf_str: *mut c_void = CFStringCreateWithCharacters(null(), encoded.as_ptr(), count);
        let active_font: *mut c_void = if !cf_str.is_null() {
            let range: CtRange = CtRange {
                location: 0,
                length: count,
            };
            let cascaded: *mut c_void = CTFontCreateForString(base_font, cf_str, range);
            CFRelease(cf_str);
            if cascaded.is_null() {
                base_font
            } else {
                CFRelease(base_font);
                cascaded
            }
        } else {
            base_font
        };
        let mut glyphs: [u16; 2] = [0; 2];
        let found: bool =
            CTFontGetGlyphsForCharacters(active_font, encoded.as_ptr(), glyphs.as_mut_ptr(), count);
        if !found || glyphs[0] == 0 {
            CFRelease(active_font);
            return None;
        }
        let mut adv_size: CtSize = CtSize {
            width: 0.0,
            height: 0.0,
        };
        let adv_f64: f64 = CTFontGetAdvancesForGlyphs(
            active_font,
            CT_FONT_ORIENTATION_DEFAULT,
            glyphs.as_ptr(),
            &mut adv_size,
            1,
        );
        let advance: u32 = adv_f64.ceil().max(2.0) as u32;
        let out_w: u32 = (adv_f64.ceil() + 3.0).max(4.0) as u32;
        let out_h: u32 = (base_ascent + base_descent).ceil().max(8.0) as u32 + 2;
        let mut coverage: Vec<u8> = vec![0; (out_w as usize) * (out_h as usize)];
        let cs: *mut c_void = CGColorSpaceCreateDeviceGray();
        if cs.is_null() {
            CFRelease(active_font);
            return None;
        }
        let cg_ctx: *mut c_void = CGBitmapContextCreate(
            coverage.as_mut_ptr().cast::<c_void>(),
            out_w as usize,
            out_h as usize,
            8,
            out_w as usize,
            cs,
            0,
        );
        CGColorSpaceRelease(cs);
        if cg_ctx.is_null() {
            CFRelease(active_font);
            return None;
        }
        CGContextSetAllowsAntialiasing(cg_ctx, true);
        CGContextSetShouldAntialias(cg_ctx, true);
        CGContextSetAllowsFontSmoothing(cg_ctx, true);
        CGContextSetShouldSmoothFonts(cg_ctx, true);
        CGContextSetGrayFillColor(cg_ctx, 1.0, 1.0);
        let pos: CtPoint = CtPoint {
            x: 1.0,
            y: base_descent.ceil() + 1.0,
        };
        CTFontDrawGlyphs(active_font, glyphs.as_ptr(), &pos, 1, cg_ctx);
        CGContextRelease(cg_ctx);
        CFRelease(active_font);
        Some(RasterizedGlyph::new(out_w, out_h, advance, coverage))
    }
}

#[cfg(target_os = "windows")]
#[link(name = "gdi32")]
unsafe extern "system" {
    /// Creates a Win32 memory device context compatible with the screen.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn CreateCompatibleDC(hdc: *mut c_void) -> *mut c_void;
    /// Deletes a Win32 device context handle.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn DeleteDC(hdc: *mut c_void) -> i32;
    /// Selects a GDI object into a device context.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn SelectObject(hdc: *mut c_void, h: *mut c_void) -> *mut c_void;
    /// Deletes a GDI object handle.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn DeleteObject(ho: *mut c_void) -> i32;
    /// Creates a logical Win32 font handle.
    ///
    /// # Arguments
    ///
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `*const u16` - A `*const u16` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn CreateFontW(
        c_height: i32,
        c_width: i32,
        c_escapement: i32,
        c_orientation: i32,
        c_weight: i32,
        b_italic: u32,
        b_underline: u32,
        b_strike_out: u32,
        i_char_set: u32,
        i_out_precision: u32,
        i_clip_precision: u32,
        i_quality: u32,
        i_pitch_and_family: u32,
        psz_face_name: *const u16,
    ) -> *mut c_void;
    /// Extracts a grayscale glyph bitmap and metrics via Win32 GDI.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `*mut WinGlyphMetrics` - A `*mut WinGlyphMetrics` parameter.
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `*const WinMat2` - A `*const WinMat2` parameter.
    ///
    /// # Returns
    ///
    /// - `u32` - A 32-bit unsigned integer.
    fn GetGlyphOutlineW(
        hdc: *mut c_void,
        u_char: u32,
        fu_format: u32,
        lpgm: *mut WinGlyphMetrics,
        cj_buffer: u32,
        pv_buffer: *mut c_void,
        lpmat2: *const WinMat2,
    ) -> u32;
}

/// Rasterizes a glyph using Windows Win32 GDI `GetGlyphOutlineW` (`GGO_GRAY8_BITMAP`).
///
/// # Arguments
///
/// - `GlyphKey` - Quantized glyph descriptor.
///
/// # Returns
///
/// - `Option<RasterizedGlyph>` - Anti-aliased coverage bitmap or `None` when unavailable.
#[cfg(target_os = "windows")]
fn rasterize_win32_gdi_glyph(key: GlyphKey) -> Option<RasterizedGlyph> {
    let size_px: i32 = ((key.get_size_tenths() as f32) * 0.1).round().max(8.0) as i32;
    let weight: i32 = if key.get_bold() {
        WIN_FW_BOLD
    } else {
        WIN_FW_NORMAL
    };
    let identity: WinMat2 = WinMat2 {
        em11: WinFixed { fract: 0, value: 1 },
        em12: WinFixed { fract: 0, value: 0 },
        em21: WinFixed { fract: 0, value: 0 },
        em22: WinFixed { fract: 0, value: 1 },
    };
    unsafe {
        let hdc: *mut c_void = CreateCompatibleDC(null_mut());
        if hdc.is_null() {
            return None;
        }
        let hfont: *mut c_void = CreateFontW(
            -size_px,
            0,
            0,
            0,
            weight,
            0,
            0,
            0,
            WIN_DEFAULT_CHARSET,
            0,
            0,
            4,
            0,
            WIN_FONT_NAME_UTF16.as_ptr(),
        );
        if hfont.is_null() {
            let _: i32 = DeleteDC(hdc);
            return None;
        }
        let prev_obj: *mut c_void = SelectObject(hdc, hfont);
        let mut metrics: WinGlyphMetrics = WinGlyphMetrics::default();
        let needed: u32 = GetGlyphOutlineW(
            hdc,
            key.get_codepoint() as u32,
            WIN_GGO_GRAY8_BITMAP,
            &mut metrics,
            0,
            null_mut(),
            &identity,
        );
        if needed == u32::MAX || needed == 0 || metrics.black_box_x == 0 || metrics.black_box_y == 0
        {
            let _: *mut c_void = SelectObject(hdc, prev_obj);
            let _: i32 = DeleteObject(hfont);
            let _: i32 = DeleteDC(hdc);
            return None;
        }
        let mut raw_buf: Vec<u8> = vec![0; needed as usize];
        let written: u32 = GetGlyphOutlineW(
            hdc,
            key.get_codepoint() as u32,
            WIN_GGO_GRAY8_BITMAP,
            &mut metrics,
            needed,
            raw_buf.as_mut_ptr().cast::<c_void>(),
            &identity,
        );
        let _: *mut c_void = SelectObject(hdc, prev_obj);
        let _: i32 = DeleteObject(hfont);
        let _: i32 = DeleteDC(hdc);
        if written == u32::MAX {
            return None;
        }
        let w: u32 = metrics.black_box_x;
        let h: u32 = metrics.black_box_y;
        let stride: usize = ((w as usize) + 3) & !3;
        let mut coverage: Vec<u8> = vec![0; (w as usize) * (h as usize)];
        for py in 0..(h as usize) {
            for px in 0..(w as usize) {
                let src_idx: usize = py * stride + px;
                if src_idx < raw_buf.len() {
                    let level: u16 = (raw_buf[src_idx] as u16).min(64);
                    coverage[py * (w as usize) + px] = ((level * 255) / 64) as u8;
                }
            }
        }
        let advance: u32 = (metrics.cell_inc_x as i32).max(w as i32) as u32;
        Some(RasterizedGlyph::new(w, h, advance, coverage))
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
unsafe extern "C" {
    /// Dynamically opens a shared library object by name.
    ///
    /// # Arguments
    ///
    /// - `*const c_char` - A `*const c_char` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn dlopen(filename: *const c_char, flags: i32) -> *mut c_void;
    /// Resolves a symbol pointer from an open shared library handle.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `*const c_char` - A `*const c_char` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    /// Closes an open shared library handle.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn dlclose(handle: *mut c_void) -> i32;
}

/// Checks at runtime whether FreeType (`libfreetype.so.6` / `libft2.so`) and a system font exist.
///
/// # Returns
///
/// - `bool` - True when FreeType symbols and at least one system font file are reachable.
#[cfg(any(target_os = "linux", target_os = "android"))]
fn freetype_runtime_available() -> bool {
    let Ok(lib_name) = CString::new(LIB_FREETYPE_SONAME) else {
        return false;
    };
    let Ok(sym_init) = CString::new(SYM_FT_INIT) else {
        return false;
    };
    let Ok(sym_done) = CString::new(SYM_FT_DONE) else {
        return false;
    };
    unsafe {
        let handle: *mut c_void = dlopen(lib_name.as_ptr(), RTLD_LAZY_FONT);
        if handle.is_null() {
            return false;
        }
        let init_ptr: *mut c_void = dlsym(handle, sym_init.as_ptr());
        let done_ptr: *mut c_void = dlsym(handle, sym_done.as_ptr());
        let _: i32 = dlclose(handle);
        if init_ptr.is_null() || done_ptr.is_null() {
            return false;
        }
    }
    #[cfg(target_os = "linux")]
    {
        LINUX_FONT_CANDIDATES
            .iter()
            .any(|p: &&str| Path::new(p).exists())
    }
    #[cfg(target_os = "android")]
    {
        ANDROID_FONT_CANDIDATES
            .iter()
            .any(|p: &&str| Path::new(p).exists())
    }
}

/// Rasterizes a glyph on Web using an offscreen `HtmlCanvasElement` + `CanvasRenderingContext2d`.
///
/// # Arguments
///
/// - `GlyphKey` - Quantized glyph descriptor.
///
/// # Returns
///
/// - `Option<RasterizedGlyph>` - Anti-aliased coverage bitmap extracted from Canvas alpha channel.
#[cfg(target_arch = "wasm32")]
fn rasterize_web_canvas_glyph(key: GlyphKey) -> Option<RasterizedGlyph> {
    let win: Window = window()?;
    let doc: Document = win.document()?;
    let el: Element = doc.create_element(WEB_FONT_CANVAS_TAG).ok()?;
    let canvas: HtmlCanvasElement = el.dyn_into::<HtmlCanvasElement>().ok()?;
    let size_px: f32 = ((key.get_size_tenths() as f32) * 0.1).max(8.0);
    let h: u32 = (size_px * 1.32).ceil().max(10.0) as u32;
    let initial_w: u32 = (size_px * 1.6).ceil().max(12.0) as u32;
    canvas.set_width(initial_w);
    canvas.set_height(h);
    let raw: Object = canvas.get_context(WEB_FONT_CTX_2D).ok()??;
    let ctx2d: CanvasRenderingContext2d = raw.dyn_into::<CanvasRenderingContext2d>().ok()?;
    let weight: &str = if key.get_bold() { "600" } else { "400" };
    let font_decl: String = format!("{weight} {size_px}px {FONT_FAMILY_WEB_SYSTEM}");
    ctx2d.set_font(&font_decl);
    let mut text_buf: [u8; 4] = [0; 4];
    let text: &str = key.get_codepoint().encode_utf8(&mut text_buf);
    let measured_w: f64 = ctx2d
        .measure_text(text)
        .map(|m: TextMetrics| m.width())
        .unwrap_or((size_px * 0.62) as f64);
    let advance: u32 = measured_w.ceil().max(2.0) as u32;
    let w: u32 = (advance + 3).max(4);
    canvas.set_width(w);
    canvas.set_height(h);
    ctx2d.set_font(&font_decl);
    ctx2d.set_fill_style_str(WEB_FONT_WHITE);
    drop(ctx2d.fill_text(text, 1.0, (size_px * 0.98) as f64));
    let img: ImageData = ctx2d.get_image_data(0.0, 0.0, w as f64, h as f64).ok()?;
    let rgba: Clamped<Vec<u8>> = img.data();
    let mut coverage: Vec<u8> = Vec::with_capacity((w as usize) * (h as usize));
    for px in rgba.0.as_chunks::<4>().0.iter() {
        coverage.push(px[3]);
    }
    Some(RasterizedGlyph::new(w, h, advance, coverage))
}
