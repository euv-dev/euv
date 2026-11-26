use super::*;

/// Complete semantic color palette resolved for a `(DesignScheme, ThemeMode)` pair.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, PartialEq)]
pub struct ThemePalette {
    /// Application canvas background color.
    #[get(type(copy))]
    pub(crate) background: Color,
    /// Primary foreground ink color.
    #[get(type(copy))]
    pub(crate) foreground: Color,
    /// Elevated card and panel surface color.
    #[get(type(copy))]
    pub(crate) card: Color,
    /// Card foreground ink color.
    #[get(type(copy))]
    pub(crate) card_foreground: Color,
    /// Brand primary interactive fill color.
    #[get(type(copy))]
    pub(crate) primary: Color,
    /// Foreground text color atop `primary`.
    #[get(type(copy))]
    pub(crate) primary_foreground: Color,
    /// Subtle secondary control fill color.
    #[get(type(copy))]
    pub(crate) secondary: Color,
    /// Foreground text color atop `secondary`.
    #[get(type(copy))]
    pub(crate) secondary_foreground: Color,
    /// Muted container background color.
    #[get(type(copy))]
    pub(crate) muted: Color,
    /// Muted label and helper text color.
    #[get(type(copy))]
    pub(crate) muted_foreground: Color,
    /// Accent hover/selection highlight color.
    #[get(type(copy))]
    pub(crate) accent: Color,
    /// Destructive action fill color.
    #[get(type(copy))]
    pub(crate) destructive: Color,
    /// Foreground text color atop `destructive`.
    #[get(type(copy))]
    pub(crate) destructive_foreground: Color,
    /// Positive / success semantic color.
    #[get(type(copy))]
    pub(crate) success: Color,
    /// Foreground text color atop `success`.
    #[get(type(copy))]
    pub(crate) success_foreground: Color,
    /// Warning semantic color.
    #[get(type(copy))]
    pub(crate) warning: Color,
    /// Foreground text color atop `warning`.
    #[get(type(copy))]
    pub(crate) warning_foreground: Color,
    /// Inactive toggle switch track color.
    #[get(type(copy))]
    pub(crate) switch_track_off: Color,
    /// Toggle switch thumb fill color when inactive.
    #[get(type(copy))]
    pub(crate) switch_thumb: Color,
    /// Dark cinema surface color used behind media, video, and canvas frames.
    #[get(type(copy))]
    pub(crate) media_surface: Color,
    /// Standard 1px structural hairline border color.
    #[get(type(copy))]
    pub(crate) border: Color,
    /// Keyboard focus ring outline color.
    #[get(type(copy))]
    pub(crate) ring: Color,
    /// Modal / drawer scrim backdrop overlay color.
    #[get(type(copy))]
    pub(crate) overlay: Color,
}

/// Geometry, radius, control height, shadow ramp, and typography scale owned by a design scheme.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct ThemeGeometry {
    /// Small corner radius (badges, kbd).
    #[get(type(copy))]
    pub(crate) radius_sm: f32,
    /// Medium corner radius (buttons, inputs).
    #[get(type(copy))]
    pub(crate) radius_md: f32,
    /// Large corner radius (cards, popovers).
    #[get(type(copy))]
    pub(crate) radius_lg: f32,
    /// Extra-large corner radius (modals, hero containers).
    #[get(type(copy))]
    pub(crate) radius_xl: f32,
    /// Default structural border width.
    #[get(type(copy))]
    pub(crate) border_width: f32,
    /// Focus ring stroke width.
    #[get(type(copy))]
    pub(crate) ring_width: f32,
    /// Small control height.
    #[get(type(copy))]
    pub(crate) control_sm: f32,
    /// Default control height.
    #[get(type(copy))]
    pub(crate) control_md: f32,
    /// Large control height.
    #[get(type(copy))]
    pub(crate) control_lg: f32,
    /// Body font size in logical pixels.
    #[get(type(copy))]
    pub(crate) font_body: f32,
    /// Section heading font size.
    #[get(type(copy))]
    pub(crate) font_heading: f32,
    /// Page title font size.
    #[get(type(copy))]
    pub(crate) font_title: f32,
    /// Subtle control elevation shadow (buttons, switches, active pills).
    #[get(type(copy))]
    pub(crate) shadow_xs: Shadow,
    /// Standard surface elevation shadow (cards, panels).
    #[get(type(copy))]
    pub(crate) shadow_sm: Shadow,
    /// Floating surface elevation shadow (popovers, dropdowns, tooltips, toasts).
    #[get(type(copy))]
    pub(crate) shadow_md: Shadow,
    /// High-elevation overlay shadow (modals, dialogs, drawers, hover cards).
    #[get(type(copy))]
    pub(crate) shadow_lg: Shadow,
}

/// Complete compiled cross-platform design token bundle passed across all UI components.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct ThemeTokens {
    /// Active design scheme.
    #[get(type(copy))]
    pub(crate) scheme: DesignScheme,
    /// Active appearance mode (`Light` / `Dark`).
    #[get(type(copy))]
    pub(crate) mode: ThemeMode,
    /// Resolved semantic color palette.
    #[get(type(copy))]
    pub(crate) palette: ThemePalette,
    /// Resolved control geometry and typography scale.
    #[get(type(copy))]
    pub(crate) geometry: ThemeGeometry,
}
