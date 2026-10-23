use super::*;

/// Inherent implementation of [`Color`].
impl Color {
    /// Returns an opaque colour from its red, green, and blue channels.
    ///
    /// # Arguments
    ///
    /// - `u8` - A `u8` parameter.
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    /// Returns a colour from its red, green, blue, and alpha channels.
    ///
    /// # Arguments
    ///
    /// - `u8` - A `u8` parameter.
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Returns a colour from a packed `0xRRGGBBAA` value.
    ///
    /// # Arguments
    ///
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    pub fn from_u32(packed: u32) -> Self {
        Self {
            red: ((packed >> 24) & 0xFF) as u8,
            green: ((packed >> 16) & 0xFF) as u8,
            blue: ((packed >> 8) & 0xFF) as u8,
            alpha: (packed & 0xFF) as u8,
        }
    }

    /// Returns the colour packed as `0xRRGGBBAA`.
    ///
    /// # Returns
    ///
    /// - `u32` - A 32-bit unsigned integer.
    pub fn to_u32(self) -> u32 {
        ((self.get_red() as u32) << 24)
            | ((self.get_green() as u32) << 16)
            | ((self.get_blue() as u32) << 8)
            | (self.get_alpha() as u32)
    }

    /// Returns the colour as a CSS `rgba()` string.
    ///
    /// # Returns
    ///
    /// - `String` - A `String` value.
    pub fn to_css(self) -> String {
        if self.get_alpha() == 255 {
            format!(
                "#{:02x}{:02x}{:02x}",
                self.get_red(),
                self.get_green(),
                self.get_blue()
            )
        } else {
            format!(
                "rgba({},{},{},{})",
                self.get_red(),
                self.get_green(),
                self.get_blue(),
                format!("{:.3}", self.get_alpha() as f64 / 255.0)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
            )
        }
    }

    /// Returns the colour that lies between two colours.
    ///
    /// The channels are mixed one by one, which is what the ramp of a
    /// visual map does: the middle of the ramp is the middle of its
    /// two ends.
    ///
    /// # Arguments
    ///
    /// - `f64` - The opposite endpoint of the interpolation.
    /// - `Self` - The opposite endpoint of the interpolation.
    pub fn lerp(self, other: Self, amount: f64) -> Self {
        let amount: f64 = amount.clamp(0.0, 1.0);
        let mix = move |from: u8, to: u8| {
            (from as f64 + (to as f64 - from as f64) * amount)
                .round()
                .clamp(0.0, 255.0) as u8
        };
        Self {
            red: mix(self.get_red(), other.get_red()),
            green: mix(self.get_green(), other.get_green()),
            blue: mix(self.get_blue(), other.get_blue()),
            alpha: mix(self.get_alpha(), other.get_alpha()),
        }
    }

    /// Parses a CSS colour string.
    ///
    /// The accepted forms are the ones ECharts accepts: `#rgb`, `#rgba`,
    /// `#rrggbb`, `#rrggbbaa`, `rgb()`, `rgba()`, `hsl()`, `hsla()`,
    /// `transparent`, and the CSS named colours. Returns `None` when the
    /// string is not a colour.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - `Some(...)` on success, `None` otherwise.
    pub fn parse(input: &str) -> Option<Self> {
        parse_css_color(input)
    }

    /// Returns the colour with its alpha replaced.
    ///
    /// # Arguments
    ///
    /// - `u8` - A `u8` parameter.
    pub fn with_alpha(self, alpha: u8) -> Self {
        Self { alpha, ..self }
    }

    /// Returns the colour mixed with another one.
    ///
    /// `ratio` is the weight of `other`: `0.0` returns this colour
    /// unchanged, `1.0` returns `other`.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    /// - `Self` - A `Self` parameter.
    pub fn mix(self, other: Self, ratio: f64) -> Self {
        let ratio: f64 = ratio.clamp(0.0, 1.0);
        let blend = move |a: u8, b: u8| (a as f64 * (1.0 - ratio) + b as f64 * ratio).round() as u8;
        Self {
            red: blend(self.get_red(), other.get_red()),
            green: blend(self.get_green(), other.get_green()),
            blue: blend(self.get_blue(), other.get_blue()),
            alpha: blend(self.get_alpha(), other.get_alpha()),
        }
    }

    /// Returns the colour brightened towards white.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn lighten(self, ratio: f64) -> Self {
        self.mix(Self::rgb(255, 255, 255), ratio)
    }

    /// Returns the colour darkened towards black.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn darken(self, ratio: f64) -> Self {
        self.mix(Self::rgb(0, 0, 0), ratio)
    }
}

/// Inherent implementation of [`LinearGradient`].
impl LinearGradient {
    /// Returns a linear gradient between the two points.
    ///
    /// # Arguments
    ///
    /// - `(f64, f64)` - A `(f64, f64)` parameter.
    pub fn new(start: (f64, f64), end: (f64, f64)) -> Self {
        Self {
            start,
            end,
            stops: Vec::new(),
        }
    }

    /// Adds a colour stop and returns the gradient.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    /// - `Color` - A `Color` parameter.
    pub fn with_stop(mut self, offset: f64, color: Color) -> Self {
        self.get_mut_stops().push(GradientStop { offset, color });
        self.get_mut_stops()
            .sort_by(|a: &GradientStop, b: &GradientStop| {
                a.offset
                    .partial_cmp(&b.offset)
                    .unwrap_or(core::cmp::Ordering::Equal)
            });
        self
    }
}

/// Inherent implementation of [`RadialGradient`].
impl RadialGradient {
    /// Returns a radial gradient at the given centre and radius.
    ///
    /// # Arguments
    ///
    /// - `(f64, f64)` - A `(f64, f64)` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    pub fn new(center: (f64, f64), radius: f64) -> Self {
        Self {
            center,
            radius,
            stops: Vec::new(),
        }
    }

    /// Adds a colour stop and returns the gradient.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    /// - `Color` - A `Color` parameter.
    pub fn with_stop(mut self, offset: f64, color: Color) -> Self {
        self.get_mut_stops().push(GradientStop { offset, color });
        self.get_mut_stops()
            .sort_by(|a: &GradientStop, b: &GradientStop| {
                a.offset
                    .partial_cmp(&b.offset)
                    .unwrap_or(core::cmp::Ordering::Equal)
            });
        self
    }
}

/// Inherent implementation of [`TextStyle`].
impl TextStyle {
    /// Returns the default text style.
    pub fn new() -> Self {
        Self {
            family: DEFAULT_FONT_FAMILY.to_string(),
            size: 12.0,
            weight: DEFAULT_FONT_WEIGHT.to_string(),
            style: DEFAULT_FONT_STYLE.to_string(),
            align: TextAlign::default(),
            vertical_align: TextVerticalAlign::default(),
            line_height: 1.2,
            wrap: false,
            overflow: TextOverflow::default(),
            color: Color::rgb(51, 51, 51),
            background: None,
            padding: (0.0, 0.0, 0.0, 0.0),
            border_width: 0.0,
            border_color: None,
            border_radius: 0.0,
            rotate: 0.0,
            stroke_width: 0.0,
            stroke_color: None,
            rich: Vec::new(),
        }
    }

    /// Returns the CSS `font` shorthand for this style.
    ///
    /// # Returns
    ///
    /// - `String` - A `String` value.
    pub fn font(&self) -> String {
        format!(
            "{} {} {} {}",
            self.get_style(),
            self.get_weight(),
            self.get_size(),
            self.get_family()
        )
    }

    /// Returns the line height in pixels.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn line_height_px(&self) -> f64 {
        self.get_size() * self.get_line_height()
    }

    /// Adds a rich-text run and returns the style.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `RichTextStyle` - A `RichTextStyle` parameter.
    pub fn with_rich(mut self, name: &str, style: RichTextStyle) -> Self {
        self.get_mut_rich().push((name.to_string(), style));
        self
    }
}

/// Default construction for [`TextStyle`].
impl Default for TextStyle {
    /// Constructs a default [`TextStyle`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`RichTextStyle`].
impl RichTextStyle {
    /// Returns an empty rich-text run style.
    pub fn new() -> Self {
        Self {
            family: None,
            size: None,
            weight: None,
            style: None,
            color: None,
            background: None,
            padding: None,
            line_height: None,
        }
    }
}

/// Default construction for [`RichTextStyle`].
impl Default for RichTextStyle {
    /// Constructs a default [`RichTextStyle`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`LineStyle`].
impl LineStyle {
    /// Returns the default line style.
    pub fn new() -> Self {
        Self {
            width: 1.0,
            cap: LineCap::default(),
            join: LineJoin::default(),
            dash: DashPattern::solid(),
            opacity: 1.0,
            shadow: None,
            color: Color::rgb(51, 51, 51),
        }
    }

    /// Returns the geometry parameters the tessellator needs.
    ///
    /// # Returns
    ///
    /// - `StrokeStyle` - A `StrokeStyle` value.
    pub fn geometry(&self) -> StrokeStyle {
        StrokeStyle {
            width: self.get_width() as f32,
            cap: *self.get_cap(),
            join: *self.get_join(),
            dash: self.get_dash().clone(),
        }
    }

    /// Returns the dash pattern for the given dash type.
    ///
    /// # Arguments
    ///
    /// - `LineDashType` - A `LineDashType` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `DashPattern` - A `DashPattern` value.
    pub fn dash_for(dash_type: LineDashType, width: f64) -> DashPattern {
        let width: f32 = width as f32;
        match dash_type {
            LineDashType::Solid => DashPattern::solid(),
            LineDashType::Dashed => DashPattern::new(vec![width * 5.0, width * 5.0], 0.0),
            LineDashType::Dotted => DashPattern::new(vec![width, width * 2.0], 0.0),
        }
    }
}

/// Default construction for [`LineStyle`].
impl Default for LineStyle {
    /// Constructs a default [`LineStyle`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`ShapeStyle`].
impl ShapeStyle {
    /// Returns the default shape style: a transparent fill and a visible
    /// stroke.
    pub fn new() -> Self {
        Self {
            fill: Fill::None,
            stroke: Stroke::None,
            line_style: LineStyle::new(),
            shadow: None,
            opacity: 1.0,
        }
    }
}

/// Default construction for [`ShapeStyle`].
impl Default for ShapeStyle {
    /// Constructs a default [`ShapeStyle`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`Fill`].
impl Fill {
    /// Returns the fill's solid colour, when the fill is solid.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - `Some(...)` on success, `None` otherwise.
    pub fn solid_color(&self) -> Option<Color> {
        match self {
            Fill::Solid(color) => Some(*color),
            _ => None,
        }
    }
}

/// Inherent implementation of [`Stroke`].
impl Stroke {
    /// Returns the stroke's solid colour, when the stroke is solid.
    ///
    /// # Returns
    ///
    /// - `Option<Color>` - `Some(...)` on success, `None` otherwise.
    pub fn solid_color(&self) -> Option<Color> {
        match self {
            Stroke::Solid(color) => Some(*color),
            _ => None,
        }
    }
}
