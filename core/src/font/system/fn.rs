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
#[inline(always)]
#[must_use]
pub fn is_wide_unicode(ch: char) -> bool {
    let cp: u32 = ch as u32;
    if cp < CJK_PUNCT_START {
        return false;
    }
    let in_cjk: bool = (CJK_UNIFIED_START..=CJK_UNIFIED_END).contains(&cp)
        || (CJK_EXT_A_START..=CJK_EXT_A_END).contains(&cp);
    if in_cjk {
        return true;
    }
    let in_punct: bool = (CJK_PUNCT_START..=CJK_PUNCT_END).contains(&cp)
        || (FULLWIDTH_START..=FULLWIDTH_END).contains(&cp);
    if in_punct {
        return true;
    }
    (HANGUL_START..=HANGUL_END).contains(&cp)
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
    let out_w_usize: usize = out_w as usize;
    let mut coverage: Vec<u8> = vec![0; out_w_usize * (out_h as usize)];
    let mut seed: u32 = (key.get_codepoint() as u32).wrapping_mul(0x9E37_79B1);
    let stroke: u32 = if key.get_bold() { 2 } else { 1 };
    let grid: u32 = 7;
    let mut x_spans: [(usize, usize); 7] = [(0, 0); 7];
    for gx in 0..grid {
        let x0: u32 = (gx * out_w) / grid;
        let x1: u32 = (x0 + stroke).min(((gx + 1) * out_w) / grid).min(out_w);
        x_spans[gx as usize] = (x0 as usize, x1 as usize);
    }
    for gy in 0..grid {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let row_mask: u32 = (seed >> 20) & 0x7F;
        let y0: usize = ((gy * out_h) / grid) as usize;
        let y1: usize = (((gy + 1) * out_h) / grid).min(out_h) as usize;
        if y0 >= y1 {
            continue;
        }
        let border_row: bool = gy == 0 || gy == grid - 1;
        let first_start: usize = y0 * out_w_usize;
        let first_end: usize = first_start + out_w_usize;
        let first_row: &mut [u8] = &mut coverage[first_start..first_end];
        for (gx_idx, &(x0, x1)) in x_spans.iter().enumerate() {
            let mirror_x: usize = if gx_idx < 4 { gx_idx } else { 6 - gx_idx };
            let bit_on: bool = border_row || ((row_mask >> mirror_x) & 1) != 0;
            if bit_on && x0 < x1 {
                first_row[x0..x1].fill(255);
            }
        }
        for py in (y0 + 1)..y1 {
            coverage.copy_within(first_start..first_end, py * out_w_usize);
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
        let has_native: bool = !EXTERNAL_GLYPH_RASTERIZER.load(Ordering::Acquire).is_null()
            || freetype_runtime_available();
        if has_native {
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

/// Registers an external platform-host native glyph rasterizer callback (for example Android Skia `Paint` via JNI).
///
/// # Arguments
///
/// - `ExternalGlyphRasterizer` - Anti-aliased native glyph rasterizer function pointer.
pub fn register_external_glyph_rasterizer(rasterizer: ExternalGlyphRasterizer) {
    let raw: *mut () = rasterizer as *mut ();
    EXTERNAL_GLYPH_RASTERIZER.store(raw, Ordering::Release);
}

/// Invokes the registered external native glyph rasterizer if one has been installed.
///
/// # Arguments
///
/// - `GlyphKey` - Quantized glyph descriptor.
///
/// # Returns
///
/// - `Option<RasterizedGlyph>` - Anti-aliased glyph bitmap from the host OS bridge.
#[must_use]
fn try_external_glyph_rasterizer(key: GlyphKey) -> Option<RasterizedGlyph> {
    let raw: *mut () = EXTERNAL_GLYPH_RASTERIZER.load(Ordering::Acquire);
    if raw.is_null() {
        return None;
    }
    // SAFETY: `raw` was exclusively stored by `register_external_glyph_rasterizer` from a valid `ExternalGlyphRasterizer` fn pointer.
    let func: ExternalGlyphRasterizer = unsafe { transmute(raw) };
    func(key)
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
    if let Some(ext) = try_external_glyph_rasterizer(key) {
        return Some(ext);
    }
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
        rasterize_freetype_glyph(key)
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
    /// Derives a variant `CTFontRef` with the requested symbolic traits (bold / italic).
    ///
    /// # Arguments
    ///
    /// - `*const c_void` - Base CoreText font handle.
    /// - `f64` - Point size (`0.0` preserves base size).
    /// - `*const c_void` - Optional transform matrix pointer.
    /// - `u32` - Desired symbolic trait bitmask.
    /// - `u32` - Mask of symbolic traits to update.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - Derived `CTFontRef`, or null when unavailable.
    fn CTFontCreateCopyWithSymbolicTraits(
        font: *const c_void,
        size: f64,
        matrix: *const c_void,
        sym_trait_value: u32,
        sym_trait_mask: u32,
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

/// Releases cached CoreText `CTFontRef` handles and the thread-local grayscale `CGColorSpaceRef`.
///
/// # Arguments
///
/// - `*mut c_void` - Cached grayscale `CGColorSpaceRef` (may be null).
/// - `&mut Vec<CoreTextFontCacheEntry>` - Cached CoreText font entries to drain and release.
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) fn release_coretext_cache(
    gray_space: *mut c_void,
    fonts: &mut Vec<CoreTextFontCacheEntry>,
) {
    // SAFETY: Every cached handle was created by CoreText/CoreGraphics and retained exclusively by this thread-local cache.
    unsafe {
        for entry in fonts.drain(..) {
            if !entry.font.is_null() {
                CFRelease(entry.font);
            }
        }
        if !gray_space.is_null() {
            CGColorSpaceRelease(gray_space);
        }
    }
}

/// Resolves or creates a cached Apple CoreText base font handle and singleton grayscale color space for `key`.
///
/// # Arguments
///
/// - `GlyphKey` - Quantized glyph style descriptor.
///
/// # Returns
///
/// - `Option<(*mut c_void, f64, f64, *mut c_void)>` - Tuple `(base_font, base_ascent, base_descent, gray_space)`.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn lookup_or_create_coretext_base(key: GlyphKey) -> Option<(*mut c_void, f64, f64, *mut c_void)> {
    let style_sig: u32 = GlyphAtlas::pack_style_sig(
        key.get_size_tenths(),
        key.get_bold(),
        key.get_monospace(),
        key.get_italic(),
    );
    CORETEXT_FONT_CACHE.with(|cell: &RefCell<CoreTextFontCache>| {
        let mut cache: RefMut<'_, CoreTextFontCache> = cell.borrow_mut();
        // SAFETY: CoreGraphics/CoreText handles live inside this thread-local cache and are released on eviction or `Drop`.
        unsafe {
            if cache.gray_space.is_null() {
                cache.gray_space = CGColorSpaceCreateDeviceGray();
                if cache.gray_space.is_null() {
                    return None;
                }
            }
            let gray_space: *mut c_void = cache.gray_space;
            for entry in cache.fonts.iter() {
                if entry.style_sig == style_sig {
                    return Some((entry.font, entry.ascent, entry.descent, gray_space));
                }
            }
            let size_px: f64 = ((key.get_size_tenths() as f64) * 0.1).max(6.0);
            let ui_font_kind: u32 = if key.get_monospace() {
                CT_FONT_UI_FIXED_PITCH
            } else if key.get_bold() {
                CT_FONT_UI_EMPHASIZED_SYSTEM
            } else {
                CT_FONT_UI_SYSTEM
            };
            let raw_base: *mut c_void =
                CTFontCreateUIFontForLanguage(ui_font_kind, size_px, null());
            if raw_base.is_null() {
                return None;
            }
            let mut desired_traits: u32 = 0;
            if key.get_monospace() && key.get_bold() {
                desired_traits |= CT_FONT_TRAIT_BOLD;
            }
            if key.get_italic() {
                desired_traits |= CT_FONT_TRAIT_ITALIC;
            }
            let base_font: *mut c_void = if desired_traits != 0 {
                let styled: *mut c_void = CTFontCreateCopyWithSymbolicTraits(
                    raw_base,
                    0.0,
                    null(),
                    desired_traits,
                    desired_traits,
                );
                if styled.is_null() {
                    raw_base
                } else {
                    CFRelease(raw_base);
                    styled
                }
            } else {
                raw_base
            };
            let base_ascent: f64 = CTFontGetAscent(base_font).max(size_px * 0.80);
            let base_descent: f64 = CTFontGetDescent(base_font).max(size_px * 0.24);
            if cache.fonts.len() >= CORETEXT_FONT_CACHE_MAX {
                let evicted: CoreTextFontCacheEntry = cache.fonts.remove(0);
                if !evicted.font.is_null() {
                    CFRelease(evicted.font);
                }
            }
            cache.fonts.push(CoreTextFontCacheEntry {
                style_sig,
                font: base_font,
                ascent: base_ascent,
                descent: base_descent,
            });
            Some((base_font, base_ascent, base_descent, gray_space))
        }
    })
}

/// Rasterizes a Unicode or ASCII glyph using cached Apple CoreText + CoreGraphics (`SF Pro` / `PingFang SC`).
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
    let mut utf16_buf: [u16; 2] = [0; 2];
    let encoded: &[u16] = key.get_codepoint().encode_utf16(&mut utf16_buf);
    let count: isize = encoded.len() as isize;
    let (base_font, base_ascent, base_descent, gray_space): (*mut c_void, f64, f64, *mut c_void) =
        lookup_or_create_coretext_base(key)?;
    // SAFETY: `base_font` and `gray_space` are valid thread-local cached handles; any cascaded fallback font is released locally.
    unsafe {
        let mut glyphs: [u16; 2] = [0; 2];
        let found_primary: bool =
            CTFontGetGlyphsForCharacters(base_font, encoded.as_ptr(), glyphs.as_mut_ptr(), count);
        let (active_font, owned_active): (*mut c_void, bool) = if found_primary && glyphs[0] != 0 {
            (base_font, false)
        } else {
            let cf_str: *mut c_void = CFStringCreateWithCharacters(null(), encoded.as_ptr(), count);
            if cf_str.is_null() {
                return None;
            }
            let range: CtRange = CtRange {
                location: 0,
                length: count,
            };
            let cascaded: *mut c_void = CTFontCreateForString(base_font, cf_str, range);
            CFRelease(cf_str);
            if cascaded.is_null() {
                return None;
            }
            let found_fallback: bool = CTFontGetGlyphsForCharacters(
                cascaded,
                encoded.as_ptr(),
                glyphs.as_mut_ptr(),
                count,
            );
            if !found_fallback || glyphs[0] == 0 {
                CFRelease(cascaded);
                return None;
            }
            (cascaded, true)
        };
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
        let cg_ctx: *mut c_void = CGBitmapContextCreate(
            coverage.as_mut_ptr().cast::<c_void>(),
            out_w as usize,
            out_h as usize,
            8,
            out_w as usize,
            gray_space,
            0,
        );
        if cg_ctx.is_null() {
            if owned_active {
                CFRelease(active_font);
            }
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
        if owned_active {
            CFRelease(active_font);
        }
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
    /// Queries typographic ascent and descent metrics for the selected Win32 font.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - Device context handle.
    /// - `*mut WinTextMetricW` - Output text metrics structure.
    ///
    /// # Returns
    ///
    /// - `i32` - Non-zero on success.
    fn GetTextMetricsW(hdc: *mut c_void, lptm: *mut WinTextMetricW) -> i32;
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

/// Releases cached Windows Win32 `HFONT` handles and the thread-local compatible memory `HDC`.
///
/// # Arguments
///
/// - `*mut c_void` - Cached memory `HDC` handle (may be null).
/// - `&mut Vec<Win32FontCacheEntry>` - Cached `HFONT` entries to drain and delete.
#[cfg(target_os = "windows")]
pub(crate) fn release_win32_cache(hdc: *mut c_void, fonts: &mut Vec<Win32FontCacheEntry>) {
    // SAFETY: Every cached handle was created by Win32 GDI and owned exclusively by this thread-local cache.
    unsafe {
        for entry in fonts.drain(..) {
            if !entry.hfont.is_null() {
                let _: i32 = DeleteObject(entry.hfont);
            }
        }
        if !hdc.is_null() {
            let _: i32 = DeleteDC(hdc);
        }
    }
}

/// Resolves or creates a cached Win32 `HFONT` and metrics inside `cache` for `(key, face_kind)`.
///
/// # Arguments
///
/// - `&mut Win32FontCache` - Thread-local Win32 GDI cache.
/// - `GlyphKey` - Quantized glyph descriptor.
/// - `u32` - Face slot discriminant (`0` = UI sans, `1` = mono, `2` = CJK).
/// - `*const u16` - Null-terminated UTF-16 font family name.
///
/// # Returns
///
/// - `Option<(*mut c_void, i32, i32)>` - Tuple `(hfont, ascent, descent)`.
#[cfg(target_os = "windows")]
fn lookup_or_create_win32_font(
    cache: &mut Win32FontCache,
    key: GlyphKey,
    face_kind: u32,
    face_utf16: *const u16,
) -> Option<(*mut c_void, i32, i32)> {
    let size_px: i32 = ((key.get_size_tenths() as f32) * 0.1).round().max(8.0) as i32;
    let style_sig: u32 = ((size_px as u32) & 0xFFFF)
        | (u32::from(key.get_bold()) << 16)
        | (u32::from(key.get_italic()) << 17)
        | (face_kind << 18);
    for entry in cache.fonts.iter() {
        if entry.style_sig == style_sig {
            return Some((entry.hfont, entry.ascent, entry.descent));
        }
    }
    let weight: i32 = if key.get_bold() {
        WIN_FW_BOLD
    } else {
        WIN_FW_NORMAL
    };
    // SAFETY: `cache.hdc` is a valid compatible memory DC and `face_utf16` is a static null-terminated UTF-16 string.
    unsafe {
        let hfont: *mut c_void = CreateFontW(
            -size_px,
            0,
            0,
            0,
            weight,
            u32::from(key.get_italic()),
            0,
            0,
            WIN_DEFAULT_CHARSET,
            0,
            0,
            4,
            0,
            face_utf16,
        );
        if hfont.is_null() {
            return None;
        }
        let prev_obj: *mut c_void = SelectObject(cache.hdc, hfont);
        let mut tm: WinTextMetricW = WinTextMetricW::default();
        let _: i32 = GetTextMetricsW(cache.hdc, &mut tm);
        let _: *mut c_void = SelectObject(cache.hdc, prev_obj);
        let ascent: i32 = tm.tm_ascent.max(((size_px as f32) * 0.80).ceil() as i32);
        let descent: i32 = tm.tm_descent.max(((size_px as f32) * 0.24).ceil() as i32);
        if cache.fonts.len() >= WIN32_FONT_CACHE_MAX {
            let evicted: Win32FontCacheEntry = cache.fonts.remove(0);
            if !evicted.hfont.is_null() {
                let _: i32 = DeleteObject(evicted.hfont);
            }
        }
        cache.fonts.push(Win32FontCacheEntry {
            style_sig,
            hfont,
            ascent,
            descent,
        });
        Some((hfont, ascent, descent))
    }
}

/// Rasterizes a glyph using a cached Win32 GDI font face into a baseline-aligned uniform line box.
///
/// # Arguments
///
/// - `&mut Win32FontCache` - Thread-local Win32 GDI handle and scratch buffer cache.
/// - `GlyphKey` - Quantized glyph descriptor.
/// - `u32` - Face slot discriminant (`0` = UI sans, `1` = mono, `2` = CJK).
/// - `*const u16` - Null-terminated UTF-16 font family name.
/// - `&WinMat2` - Identity transformation matrix.
///
/// # Returns
///
/// - `Option<RasterizedGlyph>` - Baseline-aligned anti-aliased glyph bitmap.
#[cfg(target_os = "windows")]
fn rasterize_win32_face(
    cache: &mut Win32FontCache,
    key: GlyphKey,
    face_kind: u32,
    face_utf16: *const u16,
    identity: &WinMat2,
) -> Option<RasterizedGlyph> {
    let (hfont, ascent, descent): (*mut c_void, i32, i32) =
        lookup_or_create_win32_font(cache, key, face_kind, face_utf16)?;
    let hdc: *mut c_void = cache.hdc;
    let out_h: u32 = ((ascent + descent).max(8) as u32) + 2;
    // SAFETY: `hdc` and `hfont` are valid handles cached on this thread.
    unsafe {
        let prev_obj: *mut c_void = SelectObject(hdc, hfont);
        let mut metrics: WinGlyphMetrics = WinGlyphMetrics::default();
        let needed: u32 = GetGlyphOutlineW(
            hdc,
            key.get_codepoint() as u32,
            WIN_GGO_GRAY8_BITMAP,
            &mut metrics,
            0,
            null_mut(),
            identity,
        );
        if needed == u32::MAX {
            let _: *mut c_void = SelectObject(hdc, prev_obj);
            return None;
        }
        let adv_i32: i32 = (metrics.cell_inc_x as i32).max(2);
        let bw: i32 = metrics.black_box_x as i32;
        let bh: i32 = metrics.black_box_y as i32;
        let dst_x0: i32 = metrics.glyph_origin_x.max(0) + 1;
        let dst_y0: i32 = ascent - metrics.glyph_origin_y + 1;
        let out_w: u32 = (adv_i32.max(dst_x0 + bw) + 3).max(4) as u32;
        let advance: u32 = adv_i32 as u32;
        if needed == 0 || bw <= 0 || bh <= 0 {
            let _: *mut c_void = SelectObject(hdc, prev_obj);
            return Some(RasterizedGlyph::new(
                out_w,
                out_h,
                advance,
                vec![0; (out_w as usize) * (out_h as usize)],
            ));
        }
        cache.raw_buf.clear();
        cache.raw_buf.resize(needed as usize, 0);
        let written: u32 = GetGlyphOutlineW(
            hdc,
            key.get_codepoint() as u32,
            WIN_GGO_GRAY8_BITMAP,
            &mut metrics,
            needed,
            cache.raw_buf.as_mut_ptr().cast::<c_void>(),
            identity,
        );
        let _: *mut c_void = SelectObject(hdc, prev_obj);
        if written == u32::MAX {
            return None;
        }
        let stride: usize = ((bw as usize) + 3) & !3;
        let mut coverage: Vec<u8> = vec![0; (out_w as usize) * (out_h as usize)];
        for py in 0..(bh as usize) {
            let ty: i32 = dst_y0 + (py as i32);
            if ty < 0 || ty >= (out_h as i32) {
                continue;
            }
            let row_off: usize = (ty as usize) * (out_w as usize);
            for px in 0..(bw as usize) {
                let tx: i32 = dst_x0 + (px as i32);
                if tx < 0 || tx >= (out_w as i32) {
                    continue;
                }
                let src_idx: usize = py * stride + px;
                if src_idx < cache.raw_buf.len() {
                    let level: u16 = (cache.raw_buf[src_idx] as u16).min(64);
                    coverage[row_off + (tx as usize)] = ((level * 255) / 64) as u8;
                }
            }
        }
        Some(RasterizedGlyph::new(out_w, out_h, advance, coverage))
    }
}

/// Rasterizes a glyph using cached Windows Win32 GDI `GetGlyphOutlineW` (`GGO_GRAY8_BITMAP`) with uniform baseline alignment and CJK fallback.
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
    let identity: WinMat2 = WinMat2 {
        em11: WinFixed { fract: 0, value: 1 },
        em12: WinFixed { fract: 0, value: 0 },
        em21: WinFixed { fract: 0, value: 0 },
        em22: WinFixed { fract: 0, value: 1 },
    };
    WIN32_FONT_CACHE.with(|cell: &RefCell<Win32FontCache>| {
        let mut cache: RefMut<'_, Win32FontCache> = cell.borrow_mut();
        // SAFETY: Persistent memory DC is created lazily once per thread and deleted on `Drop`.
        unsafe {
            if cache.hdc.is_null() {
                cache.hdc = CreateCompatibleDC(null_mut());
                if cache.hdc.is_null() {
                    return None;
                }
            }
        }
        let (primary_kind, primary_ptr): (u32, *const u16) = if is_wide_unicode(key.get_codepoint())
        {
            (2, WIN_FONT_CJK_UTF16.as_ptr())
        } else if key.get_monospace() {
            (1, WIN_FONT_MONO_UTF16.as_ptr())
        } else {
            (0, WIN_FONT_NAME_UTF16.as_ptr())
        };
        let mut result: Option<RasterizedGlyph> =
            rasterize_win32_face(&mut cache, key, primary_kind, primary_ptr, &identity);
        if result.is_none() && primary_kind != 2 {
            result =
                rasterize_win32_face(&mut cache, key, 2, WIN_FONT_CJK_UTF16.as_ptr(), &identity);
        }
        result
    })
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

/// Opens the first existing FreeType font face from `candidates`.
///
/// # Arguments
///
/// - `*mut c_void` - Initialized `FT_Library` handle.
/// - `FtNewFaceFn` - Resolved `FT_New_Face` function pointer.
/// - `&[&str]` - Ordered font file path candidates.
///
/// # Returns
///
/// - `*mut FtFaceRec` - Loaded face pointer, or null when none could be opened.
#[cfg(any(target_os = "linux", target_os = "android"))]
fn freetype_open_first_face(
    library: *mut c_void,
    new_face: FtNewFaceFn,
    candidates: &[&str],
) -> *mut FtFaceRec {
    for path in candidates.iter() {
        if !Path::new(path).exists() {
            continue;
        }
        let Ok(c_path) = CString::new(*path) else {
            continue;
        };
        let mut face: *mut FtFaceRec = null_mut();
        // SAFETY: Valid `FT_Library` handle and null-terminated path.
        let err: i32 = unsafe { new_face(library, c_path.as_ptr(), 0, &mut face) };
        if err == 0 && !face.is_null() {
            return face;
        }
    }
    null_mut()
}

/// Initializes a cached per-thread FreeType 2 runtime via `dlopen` (`libfreetype.so.6` / `libft2.so`).
///
/// # Returns
///
/// - `Option<FreeTypeRuntime>` - Initialized FreeType runtime with regular, bold, and CJK faces.
#[cfg(any(target_os = "linux", target_os = "android"))]
fn init_freetype_runtime() -> Option<FreeTypeRuntime> {
    let lib_name: CString = CString::new(LIB_FREETYPE_SONAME).ok()?;
    let s_init: CString = CString::new(SYM_FT_INIT).ok()?;
    let s_new_face: CString = CString::new(SYM_FT_NEW_FACE).ok()?;
    let s_set_px: CString = CString::new(SYM_FT_SET_PIXEL_SIZES).ok()?;
    let s_char_idx: CString = CString::new(SYM_FT_GET_CHAR_INDEX).ok()?;
    let s_load: CString = CString::new(SYM_FT_LOAD_GLYPH).ok()?;
    // SAFETY: Dynamically resolves standard FreeType 2 C ABI symbols and opens OS system font files.
    unsafe {
        let handle: *mut c_void = dlopen(lib_name.as_ptr(), RTLD_LAZY_FONT);
        if handle.is_null() {
            return None;
        }
        let p_init: *mut c_void = dlsym(handle, s_init.as_ptr());
        let p_new_face: *mut c_void = dlsym(handle, s_new_face.as_ptr());
        let p_set_px: *mut c_void = dlsym(handle, s_set_px.as_ptr());
        let p_char_idx: *mut c_void = dlsym(handle, s_char_idx.as_ptr());
        let p_load: *mut c_void = dlsym(handle, s_load.as_ptr());
        if p_init.is_null()
            || p_new_face.is_null()
            || p_set_px.is_null()
            || p_char_idx.is_null()
            || p_load.is_null()
        {
            return None;
        }
        let ft_init: unsafe extern "C" fn(*mut *mut c_void) -> i32 = transmute(p_init);
        let ft_new_face: FtNewFaceFn = transmute(p_new_face);
        let set_pixel_sizes: FtSetPixelSizesFn = transmute(p_set_px);
        let get_char_index: FtGetCharIndexFn = transmute(p_char_idx);
        let load_glyph: FtLoadGlyphFn = transmute(p_load);
        let mut library: *mut c_void = null_mut();
        if ft_init(&mut library) != 0 || library.is_null() {
            return None;
        }
        #[cfg(target_os = "linux")]
        let reg_list: &[&str] = &LINUX_FONT_CANDIDATES;
        #[cfg(target_os = "linux")]
        let bold_list: &[&str] = &LINUX_BOLD_FONT_CANDIDATES;
        #[cfg(target_os = "linux")]
        let mono_list: &[&str] = &LINUX_MONO_FONT_CANDIDATES;
        #[cfg(target_os = "linux")]
        let mono_bold_list: &[&str] = &LINUX_MONO_BOLD_FONT_CANDIDATES;
        #[cfg(target_os = "linux")]
        let cjk_list: &[&str] = &LINUX_CJK_FONT_CANDIDATES;
        #[cfg(target_os = "android")]
        let reg_list: &[&str] = &ANDROID_FONT_CANDIDATES;
        #[cfg(target_os = "android")]
        let bold_list: &[&str] = &ANDROID_BOLD_FONT_CANDIDATES;
        #[cfg(target_os = "android")]
        let mono_list: &[&str] = &ANDROID_MONO_FONT_CANDIDATES;
        #[cfg(target_os = "android")]
        let mono_bold_list: &[&str] = &ANDROID_MONO_BOLD_FONT_CANDIDATES;
        #[cfg(target_os = "android")]
        let cjk_list: &[&str] = &ANDROID_CJK_FONT_CANDIDATES;
        let regular_face: *mut FtFaceRec = freetype_open_first_face(library, ft_new_face, reg_list);
        if regular_face.is_null() {
            return None;
        }
        let bold_face: *mut FtFaceRec = freetype_open_first_face(library, ft_new_face, bold_list);
        let mono_face: *mut FtFaceRec = freetype_open_first_face(library, ft_new_face, mono_list);
        let mono_bold_face: *mut FtFaceRec =
            freetype_open_first_face(library, ft_new_face, mono_bold_list);
        let cjk_face: *mut FtFaceRec = freetype_open_first_face(library, ft_new_face, cjk_list);
        Some(FreeTypeRuntime {
            regular_face,
            bold_face,
            mono_face,
            mono_bold_face,
            cjk_face,
            set_pixel_sizes,
            get_char_index,
            load_glyph,
        })
    }
}

/// Rasterizes a Unicode or ASCII glyph on Linux / Android using dynamically loaded FreeType 2 into a uniform baseline-aligned box.
///
/// # Arguments
///
/// - `GlyphKey` - Quantized glyph descriptor.
///
/// # Returns
///
/// - `Option<RasterizedGlyph>` - Anti-aliased coverage bitmap matching CoreText baseline alignment.
#[cfg(any(target_os = "linux", target_os = "android"))]
fn rasterize_freetype_glyph(key: GlyphKey) -> Option<RasterizedGlyph> {
    FREETYPE_RUNTIME_CACHE
        .try_with(|cell: &RefCell<Option<Option<FreeTypeRuntime>>>| {
            let mut guard: RefMut<'_, Option<Option<FreeTypeRuntime>>> =
                cell.try_borrow_mut().ok()?;
            if guard.is_none() {
                *guard = Some(init_freetype_runtime());
            }
            let rt: &FreeTypeRuntime = guard.as_ref()?.as_ref()?;
            let size_px: u32 = ((key.get_size_tenths() as f32) * 0.1)
                .round()
                .clamp(6.0, 256.0) as u32;
            let primary_face: *mut FtFaceRec = if key.get_monospace() {
                if key.get_bold() && !rt.mono_bold_face.is_null() {
                    rt.mono_bold_face
                } else if !rt.mono_face.is_null() {
                    rt.mono_face
                } else if key.get_bold() && !rt.bold_face.is_null() {
                    rt.bold_face
                } else {
                    rt.regular_face
                }
            } else if key.get_bold() && !rt.bold_face.is_null() {
                rt.bold_face
            } else {
                rt.regular_face
            };
            let codepoint: usize = key.get_codepoint() as usize;
            // SAFETY: Faces and function pointers are initialized by `init_freetype_runtime` and valid for thread lifetime.
            unsafe {
                let mut active_face: *mut FtFaceRec = primary_face;
                let mut glyph_idx: u32 = (rt.get_char_index)(active_face, codepoint);
                if glyph_idx == 0 && !rt.cjk_face.is_null() {
                    let cjk_idx: u32 = (rt.get_char_index)(rt.cjk_face, codepoint);
                    if cjk_idx != 0 {
                        active_face = rt.cjk_face;
                        glyph_idx = cjk_idx;
                    }
                }
                if glyph_idx == 0 && active_face != rt.regular_face {
                    let reg_idx: u32 = (rt.get_char_index)(rt.regular_face, codepoint);
                    if reg_idx != 0 {
                        active_face = rt.regular_face;
                        glyph_idx = reg_idx;
                    }
                }
                if glyph_idx == 0 {
                    return None;
                }
                if (rt.set_pixel_sizes)(active_face, 0, size_px) != 0 {
                    return None;
                }
                if (rt.load_glyph)(active_face, glyph_idx, FT_LOAD_RENDER) != 0 {
                    return None;
                }
                let face_ref: &FtFaceRec = &*active_face;
                if face_ref.glyph.is_null() || face_ref.size.is_null() {
                    return None;
                }
                let slot: &FtGlyphSlotRec = &*face_ref.glyph;
                let size_rec: &FtSizeRec = &*face_ref.size;
                let ascent: i32 = ((size_rec.metrics.ascender + 63) >> 6)
                    .max(((size_px as f32) * 0.80).ceil() as isize)
                    as i32;
                let descent: i32 = (((-size_rec.metrics.descender) + 63) >> 6)
                    .max(((size_px as f32) * 0.24).ceil() as isize)
                    as i32;
                let out_h: u32 = ((ascent + descent).max(8) as u32) + 2;
                let adv_px: i32 = (((slot.advance.x + 63) >> 6) as i32).max(2);
                let bmp_w: i32 = slot.bitmap.width as i32;
                let bmp_h: i32 = slot.bitmap.rows as i32;
                let dst_x0: i32 = slot.bitmap_left.max(0) + 1;
                let dst_y0: i32 = ascent - slot.bitmap_top + 1;
                let out_w: u32 = (adv_px.max(dst_x0 + bmp_w) + 3).max(4) as u32;
                let advance: u32 = adv_px as u32;
                let mut coverage: Vec<u8> = vec![0; (out_w as usize) * (out_h as usize)];
                if bmp_w > 0 && bmp_h > 0 && !slot.bitmap.buffer.is_null() {
                    let pitch: i32 = slot.bitmap.pitch;
                    let abs_pitch: usize = pitch.unsigned_abs() as usize;
                    for py in 0..(bmp_h as usize) {
                        let ty: i32 = dst_y0 + (py as i32);
                        if ty < 0 || ty >= (out_h as i32) {
                            continue;
                        }
                        let src_row: usize = if pitch >= 0 {
                            py
                        } else {
                            (bmp_h as usize) - 1 - py
                        };
                        let row_ptr: *const u8 = slot.bitmap.buffer.add(src_row * abs_pitch);
                        let dst_row: usize = (ty as usize) * (out_w as usize);
                        for px in 0..(bmp_w as usize) {
                            let tx: i32 = dst_x0 + (px as i32);
                            if tx < 0 || tx >= (out_w as i32) {
                                continue;
                            }
                            let alpha: u8 = if slot.bitmap.pixel_mode == FT_PIXEL_MODE_GRAY {
                                *row_ptr.add(px)
                            } else if slot.bitmap.pixel_mode == FT_PIXEL_MODE_MONO {
                                let byte: u8 = *row_ptr.add(px >> 3);
                                if (byte & (0x80 >> (px & 7))) != 0 {
                                    255
                                } else {
                                    0
                                }
                            } else {
                                *row_ptr.add(px)
                            };
                            coverage[dst_row + (tx as usize)] = alpha;
                        }
                    }
                }
                Some(RasterizedGlyph::new(out_w, out_h, advance, coverage))
            }
        })
        .ok()
        .flatten()
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
    let style_prefix: &str = if key.get_italic() { "italic " } else { "" };
    let weight: &str = if key.get_bold() { "700" } else { "400" };
    let family_decl: &str = if key.get_monospace() {
        FONT_FAMILY_WEB_MONO
    } else {
        FONT_FAMILY_WEB_SYSTEM
    };
    let font_decl: String = format!("{style_prefix}{weight} {size_px}px {family_decl}");
    ctx2d.set_font(&font_decl);
    let mut text_buf: [u8; 4] = [0; 4];
    let text: &str = key.get_codepoint().encode_utf8(&mut text_buf);
    let measured_w: f64 = ctx2d
        .measure_text(text)
        .map(|m: TextMetrics| m.width())
        .unwrap_or((size_px * 0.62) as f64);
    let advance: u32 = if key.get_monospace() && (key.get_codepoint() as u32) <= 0x7E {
        (size_px * MONOSPACE_ADVANCE_RATIO).round().max(2.0) as u32
    } else {
        measured_w.round().max(2.0) as u32
    };
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
