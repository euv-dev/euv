use super::*;

/// Default theme tokens resolving from the active compile-time feature scheme in Light mode.
impl Default for ThemeTokens {
    /// Creates default `ThemeTokens`.
    ///
    /// # Returns
    ///
    /// - `Self` - Resolved design tokens.
    fn default() -> Self {
        Self::resolve(active_feature_scheme(), ThemeMode::Light)
    }
}

/// Complete 17-scheme x 2-mode deterministic token resolver.
impl ThemeTokens {
    /// Resolves complete `ThemeTokens` for any `(DesignScheme, ThemeMode)` pair.
    ///
    /// # Arguments
    ///
    /// - `DesignScheme` - Target design language.
    /// - `ThemeMode` - Light or Dark appearance.
    ///
    /// # Returns
    ///
    /// - `Self` - Compiled zero-allocation token bundle.
    #[must_use]
    pub fn resolve(scheme: DesignScheme, mode: ThemeMode) -> Self {
        let (accent_light, accent_dark, radius_md, radius_lg, control_md): (
            Color,
            Color,
            f32,
            f32,
            f32,
        ) = match scheme {
            DesignScheme::Shadcn => (Color::hex(0x18181B), Color::hex(0xFAFAFA), 6.0, 8.0, 36.0),
            DesignScheme::Cupertino => {
                (Color::hex(0x007AFF), Color::hex(0x0A84FF), 10.0, 14.0, 38.0)
            }
            DesignScheme::Material => {
                (Color::hex(0x6750A4), Color::hex(0xD0BCFF), 16.0, 20.0, 40.0)
            }
            DesignScheme::Fluent => (Color::hex(0x0067C0), Color::hex(0x60CDFF), 4.0, 8.0, 34.0),
            DesignScheme::AntDesign => (Color::hex(0x1677FF), Color::hex(0x1668DC), 6.0, 8.0, 34.0),
            DesignScheme::Bootstrap => (Color::hex(0x0D6EFD), Color::hex(0x3D8BFD), 6.0, 8.0, 38.0),
            DesignScheme::DaisyUi => (Color::hex(0x570DF8), Color::hex(0x661AE6), 12.0, 16.0, 40.0),
            DesignScheme::Geist => (Color::hex(0x000000), Color::hex(0xFFFFFF), 6.0, 8.0, 36.0),
            DesignScheme::HeroUi => (Color::hex(0x006FEE), Color::hex(0x338EF7), 12.0, 14.0, 40.0),
            DesignScheme::Linear => (Color::hex(0x5E6AD2), Color::hex(0x828FFF), 6.0, 10.0, 34.0),
            DesignScheme::Mantine => (Color::hex(0x228BE6), Color::hex(0x339AF0), 8.0, 12.0, 36.0),
            DesignScheme::Monochrome => {
                (Color::hex(0x111111), Color::hex(0xF5F5F5), 2.0, 4.0, 36.0)
            }
            DesignScheme::Notion => (Color::hex(0x2F3437), Color::hex(0xE8E8E8), 4.0, 6.0, 34.0),
            DesignScheme::Park => (Color::hex(0x334155), Color::hex(0xE2E8F0), 6.0, 8.0, 36.0),
            DesignScheme::Radix => (Color::hex(0x3E63DD), Color::hex(0x6E8BFF), 6.0, 10.0, 36.0),
            DesignScheme::Stripe => (Color::hex(0x635BFF), Color::hex(0x7A73FF), 8.0, 12.0, 38.0),
            DesignScheme::Supabase => (Color::hex(0x3ECF8E), Color::hex(0x3ECF8E), 6.0, 10.0, 36.0),
        };
        let is_dark: bool = mode == ThemeMode::Dark;
        let primary: Color = if is_dark { accent_dark } else { accent_light };
        let light_primary_fg: bool = (primary.get_r() as u32) * 299
            + (primary.get_g() as u32) * 587
            + (primary.get_b() as u32) * 114
            < 145_000;
        let primary_foreground: Color = if light_primary_fg {
            Color::hex(0xFFFFFF)
        } else {
            Color::hex(0x09090B)
        };
        let palette: ThemePalette = if is_dark {
            ThemePalette {
                background: Color::hex(0x09090B),
                foreground: Color::hex(0xFAFAFA),
                card: Color::hex(0x09090B),
                card_foreground: Color::hex(0xFAFAFA),
                primary,
                primary_foreground,
                secondary: Color::hex(0x27272A),
                secondary_foreground: Color::hex(0xFAFAFA),
                muted: Color::hex(0x18181B),
                muted_foreground: Color::hex(0xA1A1AA),
                accent: Color::hex(0x27272A),
                destructive: Color::hex(0xEF4444),
                destructive_foreground: Color::hex(0xFAFAFA),
                success: Color::hex(0x22C55E),
                warning: Color::hex(0xF59E0B),
                border: Color::hex(0x27272A),
                ring: Color::hex(0xD4D4D8),
                overlay: Color::rgba(0, 0, 0, 180),
            }
        } else {
            ThemePalette {
                background: Color::hex(0xFFFFFF),
                foreground: Color::hex(0x09090B),
                card: Color::hex(0xFFFFFF),
                card_foreground: Color::hex(0x09090B),
                primary,
                primary_foreground,
                secondary: Color::hex(0xF4F4F5),
                secondary_foreground: Color::hex(0x18181B),
                muted: Color::hex(0xF4F4F5),
                muted_foreground: Color::hex(0x71717A),
                accent: Color::hex(0xF4F4F5),
                destructive: Color::hex(0xDC2626),
                destructive_foreground: Color::hex(0xFFFFFF),
                success: Color::hex(0x16A34A),
                warning: Color::hex(0xD97706),
                border: Color::hex(0xE4E4E7),
                ring: primary,
                overlay: Color::rgba(0, 0, 0, 128),
            }
        };
        let geometry: ThemeGeometry = ThemeGeometry {
            radius_sm: (radius_md * 0.65).max(2.0),
            radius_md,
            radius_lg,
            radius_xl: radius_lg + 4.0,
            border_width: 1.0,
            ring_width: 2.0,
            control_sm: (control_md - 6.0).max(28.0),
            control_md,
            control_lg: control_md + 8.0,
            font_body: 14.0,
            font_heading: 16.0,
            font_title: 24.0,
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
    #[must_use]
    pub fn card_style(&self) -> Style {
        let p: ThemePalette = self.get_palette();
        let g: ThemeGeometry = self.get_geometry();
        Style::empty()
            .column()
            .padding(Edges::all(SPACE_LG))
            .gap(SPACE_MD)
            .background(p.get_card())
            .border(g.get_border_width(), p.get_border())
            .radius(g.get_radius_lg())
            .shadow(Shadow::new(0.0, 1.0, 2.0, 0.0, Color::rgba(0, 0, 0, 14)))
            .overflow(Overflow::Hidden)
            .color(p.get_card_foreground())
            .font_size(g.get_font_body())
    }
}
