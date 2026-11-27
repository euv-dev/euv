use super::*;

/// Default theme tokens resolving from the active compile-time feature scheme in Light mode.
impl Default for ThemeTokens {
    /// Creates default `ThemeTokens`.
    ///
    /// # Returns
    ///
    /// - `Self` - Resolved design tokens.
    #[inline(always)]
    fn default() -> Self {
        Self::resolve(active_feature_scheme(), ThemeMode::Light)
    }
}

/// Complete 17-scheme x 2-mode deterministic token resolver backed by compile-time LUTs.
impl ThemeTokens {
    /// Maps a `DesignScheme` variant to its canonical `0..17` LUT index.
    ///
    /// # Arguments
    ///
    /// - `DesignScheme` - Target design language.
    ///
    /// # Returns
    ///
    /// - `usize` - Canonical array index in `0..17`.
    #[inline(always)]
    const fn scheme_index(scheme: DesignScheme) -> usize {
        match scheme {
            DesignScheme::Shadcn => 0,
            DesignScheme::Cupertino => 1,
            DesignScheme::Material => 2,
            DesignScheme::Fluent => 3,
            DesignScheme::AntDesign => 4,
            DesignScheme::Bootstrap => 5,
            DesignScheme::DaisyUi => 6,
            DesignScheme::Geist => 7,
            DesignScheme::HeroUi => 8,
            DesignScheme::Linear => 9,
            DesignScheme::Mantine => 10,
            DesignScheme::Monochrome => 11,
            DesignScheme::Notion => 12,
            DesignScheme::Park => 13,
            DesignScheme::Radix => 14,
            DesignScheme::Stripe => 15,
            DesignScheme::Supabase => 16,
        }
    }

    /// Maps a `ThemeMode` variant to its binary LUT offset (`0` = Light, `1` = Dark).
    ///
    /// # Arguments
    ///
    /// - `ThemeMode` - Light or Dark appearance.
    ///
    /// # Returns
    ///
    /// - `usize` - Mode bit (`0` or `1`).
    #[inline(always)]
    const fn mode_index(mode: ThemeMode) -> usize {
        match mode {
            ThemeMode::Light => 0,
            ThemeMode::Dark => 1,
        }
    }

    /// Computes a `ThemePalette` at compile time for one `(DesignScheme, ThemeMode)` pair.
    ///
    /// # Arguments
    ///
    /// - `DesignScheme` - Target design language.
    /// - `ThemeMode` - Light or Dark appearance.
    ///
    /// # Returns
    ///
    /// - `ThemePalette` - Resolved semantic color palette.
    const fn build_palette(scheme: DesignScheme, mode: ThemeMode) -> ThemePalette {
        let (accent_light_hex, accent_dark_hex): (u32, u32) = match scheme {
            DesignScheme::Shadcn => (0x18181B, 0xFAFAFA),
            DesignScheme::Cupertino => (0x007AFF, 0x0A84FF),
            DesignScheme::Material => (0x6750A4, 0xD0BCFF),
            DesignScheme::Fluent => (0x0067C0, 0x60CDFF),
            DesignScheme::AntDesign => (0x1677FF, 0x1668DC),
            DesignScheme::Bootstrap => (0x0D6EFD, 0x3D8BFD),
            DesignScheme::DaisyUi => (0x570DF8, 0x661AE6),
            DesignScheme::Geist => (0x000000, 0xFFFFFF),
            DesignScheme::HeroUi => (0x006FEE, 0x338EF7),
            DesignScheme::Linear => (0x5E6AD2, 0x828FFF),
            DesignScheme::Mantine => (0x228BE6, 0x339AF0),
            DesignScheme::Monochrome => (0x111111, 0xF5F5F5),
            DesignScheme::Notion => (0x2F3437, 0xE8E8E8),
            DesignScheme::Park => (0x334155, 0xE2E8F0),
            DesignScheme::Radix => (0x3E63DD, 0x6E8BFF),
            DesignScheme::Stripe => (0x635BFF, 0x7A73FF),
            DesignScheme::Supabase => (0x3ECF8E, 0x3ECF8E),
        };
        let is_dark: bool = matches!(mode, ThemeMode::Dark);
        let primary_hex: u32 = if is_dark {
            accent_dark_hex
        } else {
            accent_light_hex
        };
        let primary: Color = Color::hex(primary_hex);
        let primary_r: u32 = (primary_hex >> 16) & 0xFF;
        let primary_g: u32 = (primary_hex >> 8) & 0xFF;
        let primary_b: u32 = primary_hex & 0xFF;
        let light_primary_fg: bool = primary_r * 299 + primary_g * 587 + primary_b * 114 < 145_000;
        let primary_foreground: Color = if light_primary_fg {
            Color::hex(0xFFFFFF)
        } else {
            Color::hex(0x09090B)
        };
        let (
            light_bg,
            light_card,
            light_muted,
            light_border,
            dark_bg,
            dark_card,
            dark_muted,
            dark_border,
        ): (Color, Color, Color, Color, Color, Color, Color, Color) = match scheme {
            DesignScheme::Shadcn => (
                Color::hex(0xFAFAFA),
                Color::hex(0xFFFFFF),
                Color::hex(0xF4F4F5),
                Color::hex(0xE4E4E7),
                Color::hex(0x09090B),
                Color::hex(0x111113),
                Color::hex(0x18181B),
                Color::hex(0x27272A),
            ),
            DesignScheme::Cupertino => (
                Color::hex(0xF2F2F7),
                Color::hex(0xFFFFFF),
                Color::hex(0xE5E5EA),
                Color::hex(0xD1D1D6),
                Color::hex(0x000000),
                Color::hex(0x1C1C1E),
                Color::hex(0x2C2C2E),
                Color::hex(0x38383A),
            ),
            DesignScheme::Material => (
                Color::hex(0xFEF7FF),
                Color::hex(0xFFFFFF),
                Color::hex(0xF3EDF7),
                Color::hex(0xCAC4D0),
                Color::hex(0x141218),
                Color::hex(0x1D1B20),
                Color::hex(0x2B2930),
                Color::hex(0x49454F),
            ),
            DesignScheme::Fluent => (
                Color::hex(0xF3F3F3),
                Color::hex(0xFFFFFF),
                Color::hex(0xEAEAEA),
                Color::hex(0xD9D9D9),
                Color::hex(0x202020),
                Color::hex(0x2B2B2B),
                Color::hex(0x323232),
                Color::hex(0x404040),
            ),
            DesignScheme::AntDesign => (
                Color::hex(0xF5F5F5),
                Color::hex(0xFFFFFF),
                Color::hex(0xF0F0F0),
                Color::hex(0xD9D9D9),
                Color::hex(0x0A0A0A),
                Color::hex(0x141414),
                Color::hex(0x1F1F1F),
                Color::hex(0x303030),
            ),
            DesignScheme::Bootstrap => (
                Color::hex(0xF8F9FA),
                Color::hex(0xFFFFFF),
                Color::hex(0xE9ECEF),
                Color::hex(0xDEE2E6),
                Color::hex(0x121417),
                Color::hex(0x1B1F23),
                Color::hex(0x252A30),
                Color::hex(0x343A40),
            ),
            DesignScheme::DaisyUi => (
                Color::hex(0xF7F7FA),
                Color::hex(0xFFFFFF),
                Color::hex(0xEEEDF5),
                Color::hex(0xDCD9E8),
                Color::hex(0x11131A),
                Color::hex(0x191D26),
                Color::hex(0x222736),
                Color::hex(0x32394D),
            ),
            DesignScheme::Geist => (
                Color::hex(0xFAFAFA),
                Color::hex(0xFFFFFF),
                Color::hex(0xF2F2F2),
                Color::hex(0xEAEAEA),
                Color::hex(0x000000),
                Color::hex(0x0A0A0A),
                Color::hex(0x171717),
                Color::hex(0x262626),
            ),
            DesignScheme::HeroUi => (
                Color::hex(0xF8FAFC),
                Color::hex(0xFFFFFF),
                Color::hex(0xF1F5F9),
                Color::hex(0xE2E8F0),
                Color::hex(0x070B14),
                Color::hex(0x0F172A),
                Color::hex(0x1E293B),
                Color::hex(0x334155),
            ),
            DesignScheme::Linear => (
                Color::hex(0xF7F8FA),
                Color::hex(0xFFFFFF),
                Color::hex(0xEEF0F4),
                Color::hex(0xDFE3EA),
                Color::hex(0x0B0C10),
                Color::hex(0x13151C),
                Color::hex(0x1C1F29),
                Color::hex(0x2A2E3D),
            ),
            DesignScheme::Mantine => (
                Color::hex(0xF8F9FA),
                Color::hex(0xFFFFFF),
                Color::hex(0xF1F3F5),
                Color::hex(0xDEE2E6),
                Color::hex(0x141517),
                Color::hex(0x1A1B1E),
                Color::hex(0x25262B),
                Color::hex(0x373A40),
            ),
            DesignScheme::Monochrome => (
                Color::hex(0xF7F7F7),
                Color::hex(0xFFFFFF),
                Color::hex(0xEDEDED),
                Color::hex(0xD4D4D4),
                Color::hex(0x0A0A0A),
                Color::hex(0x141414),
                Color::hex(0x1F1F1F),
                Color::hex(0x333333),
            ),
            DesignScheme::Notion => (
                Color::hex(0xFBFBF9),
                Color::hex(0xFFFFFF),
                Color::hex(0xF1F1EE),
                Color::hex(0xE3E2DF),
                Color::hex(0x191919),
                Color::hex(0x202020),
                Color::hex(0x2A2A2A),
                Color::hex(0x373737),
            ),
            DesignScheme::Park => (
                Color::hex(0xF8FAFC),
                Color::hex(0xFFFFFF),
                Color::hex(0xF1F5F9),
                Color::hex(0xCBD5E1),
                Color::hex(0x0B1120),
                Color::hex(0x111827),
                Color::hex(0x1E293B),
                Color::hex(0x334155),
            ),
            DesignScheme::Radix => (
                Color::hex(0xF9FAFB),
                Color::hex(0xFFFFFF),
                Color::hex(0xF0F2F7),
                Color::hex(0xDCE1EB),
                Color::hex(0x111113),
                Color::hex(0x18191B),
                Color::hex(0x212225),
                Color::hex(0x303238),
            ),
            DesignScheme::Stripe => (
                Color::hex(0xF6F9FC),
                Color::hex(0xFFFFFF),
                Color::hex(0xEBF1F6),
                Color::hex(0xD8E2EC),
                Color::hex(0x0A1628),
                Color::hex(0x10213A),
                Color::hex(0x172D4D),
                Color::hex(0x254066),
            ),
            DesignScheme::Supabase => (
                Color::hex(0xF8FAFC),
                Color::hex(0xFFFFFF),
                Color::hex(0xF1F5F4),
                Color::hex(0xDDE5E2),
                Color::hex(0x121212),
                Color::hex(0x1C1C1C),
                Color::hex(0x262626),
                Color::hex(0x333333),
            ),
        };
        if is_dark {
            ThemePalette {
                background: dark_bg,
                foreground: Color::hex(0xFAFAFA),
                card: dark_card,
                card_foreground: Color::hex(0xFAFAFA),
                primary,
                primary_foreground,
                secondary: dark_muted,
                secondary_foreground: Color::hex(0xFAFAFA),
                muted: dark_muted,
                muted_foreground: Color::hex(0xA1A1AA),
                accent: dark_border,
                destructive: Color::hex(0xEF4444),
                destructive_foreground: Color::hex(0xFAFAFA),
                success: Color::hex(0x22C55E),
                success_foreground: Color::hex(0x052E16),
                warning: Color::hex(0xF59E0B),
                warning_foreground: Color::hex(0x09090B),
                switch_track_off: Color::hex(0x3F3F46),
                switch_thumb: Color::hex(0xFAFAFA),
                media_surface: Color::hex(0x0B1120),
                border: dark_border,
                ring: primary,
                overlay: Color::rgba(0, 0, 0, 184),
            }
        } else {
            ThemePalette {
                background: light_bg,
                foreground: Color::hex(0x09090B),
                card: light_card,
                card_foreground: Color::hex(0x09090B),
                primary,
                primary_foreground,
                secondary: light_muted,
                secondary_foreground: Color::hex(0x18181B),
                muted: light_muted,
                muted_foreground: Color::hex(0x71717A),
                accent: light_muted,
                destructive: Color::hex(0xDC2626),
                destructive_foreground: Color::hex(0xFFFFFF),
                success: Color::hex(0x16A34A),
                success_foreground: Color::hex(0xFFFFFF),
                warning: Color::hex(0xD97706),
                warning_foreground: Color::hex(0x09090B),
                switch_track_off: Color::hex(0xD4D4D8),
                switch_thumb: Color::hex(0xFFFFFF),
                media_surface: Color::hex(0x0F172A),
                border: light_border,
                ring: primary,
                overlay: Color::rgba(0, 0, 0, 128),
            }
        }
    }

    /// Computes `[radius_sm, radius_md, radius_lg, radius_xl, control_sm, control_md, control_lg]` at compile time for `scheme`.
    ///
    /// # Arguments
    ///
    /// - `DesignScheme` - Target design language.
    ///
    /// # Returns
    ///
    /// - `[f32; 7]` - Precomputed radius and control height ramp.
    const fn build_geometry_metrics(scheme: DesignScheme) -> [f32; 7] {
        let (radius_md, radius_lg, control_md): (f32, f32, f32) = match scheme {
            DesignScheme::Shadcn => (6.0, 8.0, 36.0),
            DesignScheme::Cupertino => (10.0, 14.0, 38.0),
            DesignScheme::Material => (16.0, 20.0, 40.0),
            DesignScheme::Fluent => (4.0, 8.0, 34.0),
            DesignScheme::AntDesign => (6.0, 8.0, 34.0),
            DesignScheme::Bootstrap => (6.0, 8.0, 38.0),
            DesignScheme::DaisyUi => (12.0, 16.0, 40.0),
            DesignScheme::Geist => (6.0, 8.0, 36.0),
            DesignScheme::HeroUi => (12.0, 14.0, 40.0),
            DesignScheme::Linear => (6.0, 10.0, 34.0),
            DesignScheme::Mantine => (8.0, 12.0, 36.0),
            DesignScheme::Monochrome => (2.0, 4.0, 36.0),
            DesignScheme::Notion => (4.0, 6.0, 34.0),
            DesignScheme::Park => (6.0, 8.0, 36.0),
            DesignScheme::Radix => (6.0, 10.0, 36.0),
            DesignScheme::Stripe => (8.0, 12.0, 38.0),
            DesignScheme::Supabase => (6.0, 10.0, 36.0),
        };
        let raw_radius_sm: f32 = radius_md * 0.65;
        let radius_sm: f32 = if raw_radius_sm > 2.0 {
            raw_radius_sm
        } else {
            2.0
        };
        let raw_control_sm: f32 = control_md - 6.0;
        let control_sm: f32 = if raw_control_sm > 28.0 {
            raw_control_sm
        } else {
            28.0
        };
        [
            radius_sm,
            radius_md,
            radius_lg,
            radius_lg + 4.0,
            control_sm,
            control_md,
            control_md + 8.0,
        ]
    }

    /// Precomputes all 34 `(DesignScheme, ThemeMode)` semantic color palettes into a compile-time lookup table.
    ///
    /// # Returns
    ///
    /// - `[ThemePalette; 34]` - Compile-time palette LUT indexed by `(scheme_idx << 1) | dark_idx`.
    pub(crate) const fn build_palette_lut() -> [ThemePalette; 34] {
        let schemes: [DesignScheme; 17] = DesignScheme::all();
        let placeholder: ThemePalette = Self::build_palette(DesignScheme::Shadcn, ThemeMode::Light);
        let mut lut: [ThemePalette; 34] = [placeholder; 34];
        let mut idx: usize = 0;
        while idx < schemes.len() {
            let scheme: DesignScheme = schemes[idx];
            lut[idx << 1] = Self::build_palette(scheme, ThemeMode::Light);
            lut[(idx << 1) | 1] = Self::build_palette(scheme, ThemeMode::Dark);
            idx += 1;
        }
        lut
    }

    /// Precomputes radius and control height metrics for all 17 design schemes into a compile-time lookup table.
    ///
    /// # Returns
    ///
    /// - `[[f32; 7]; 17]` - Compile-time geometry metrics LUT indexed by `scheme_idx`.
    pub(crate) const fn build_geometry_metrics_lut() -> [[f32; 7]; 17] {
        let schemes: [DesignScheme; 17] = DesignScheme::all();
        let mut lut: [[f32; 7]; 17] = [[0.0; 7]; 17];
        let mut idx: usize = 0;
        while idx < schemes.len() {
            lut[idx] = Self::build_geometry_metrics(schemes[idx]);
            idx += 1;
        }
        lut
    }

    /// Resolves complete `ThemeTokens` for any `(DesignScheme, ThemeMode)` pair via O(1) compile-time LUT lookup.
    ///
    /// # Arguments
    ///
    /// - `DesignScheme` - Target design language.
    /// - `ThemeMode` - Light or Dark appearance.
    ///
    /// # Returns
    ///
    /// - `Self` - Compiled zero-allocation token bundle.
    #[inline(always)]
    #[must_use]
    pub fn resolve(scheme: DesignScheme, mode: ThemeMode) -> Self {
        let scheme_idx: usize = Self::scheme_index(scheme);
        let dark_idx: usize = Self::mode_index(mode);
        let palette: ThemePalette = THEME_PALETTE_LUT[(scheme_idx << 1) | dark_idx];
        let [
            radius_sm,
            radius_md,
            radius_lg,
            radius_xl,
            control_sm,
            control_md,
            control_lg,
        ]: [f32; 7] = THEME_GEOMETRY_METRICS_LUT[scheme_idx];
        let (shadow_xs, shadow_sm, shadow_md, shadow_lg): (Shadow, Shadow, Shadow, Shadow) =
            if dark_idx != 0 {
                (
                    Shadow::new(0.0, 1.0, 2.0, 0.0, Color::rgba(0, 0, 0, 48)),
                    Shadow::new(0.0, 2.0, 6.0, 0.0, Color::rgba(0, 0, 0, 64)),
                    Shadow::new(0.0, 6.0, 16.0, 0.0, Color::rgba(0, 0, 0, 84)),
                    Shadow::new(0.0, 12.0, 26.0, 0.0, Color::rgba(0, 0, 0, 108)),
                )
            } else {
                (
                    Shadow::new(0.0, 1.0, 2.0, 0.0, Color::rgba(0, 0, 0, 12)),
                    Shadow::new(0.0, 1.0, 3.0, 0.0, Color::rgba(0, 0, 0, 18)),
                    Shadow::new(0.0, 4.0, 12.0, 0.0, Color::rgba(0, 0, 0, 24)),
                    Shadow::new(0.0, 10.0, 20.0, 0.0, Color::rgba(0, 0, 0, 32)),
                )
            };
        let geometry: ThemeGeometry = ThemeGeometry {
            radius_sm,
            radius_md,
            radius_lg,
            radius_xl,
            border_width: 1.0,
            ring_width: 2.0,
            control_sm,
            control_md,
            control_lg,
            font_body: 14.0,
            font_heading: 16.0,
            font_title: 24.0,
            shadow_xs,
            shadow_sm,
            shadow_md,
            shadow_lg,
        };
        Self {
            scheme,
            mode,
            palette,
            geometry,
        }
    }

    /// Returns a standard card surface `Style` honoring the active scheme and theme.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Active theme tokens.
    ///
    /// # Returns
    ///
    /// - `Style` - Compiled card box style.
    #[inline(always)]
    #[must_use]
    pub fn card_style(&self) -> Style {
        let p: ThemePalette = self.get_palette();
        let g: ThemeGeometry = self.get_geometry();
        let mut style: Style = Style::default();
        style
            .get_mut_layout()
            .set_flex_direction(FlexDirection::Column)
            .set_padding(Edges::all(SPACE_LG))
            .set_row_gap(SPACE_MD)
            .set_column_gap(SPACE_MD)
            .set_overflow(Overflow::Hidden);
        style
            .get_mut_paint()
            .set_background(Some(p.get_card()))
            .set_border(Border::new(g.get_border_width(), p.get_border()))
            .set_radius(CornerRadius::all(g.get_radius_lg()))
            .set_shadow(Some(g.get_shadow_sm()));
        style
            .get_mut_text()
            .set_color(p.get_card_foreground())
            .set_font_size(g.get_font_body());
        style
    }
}
