use super::*;

/// Parses a CSS colour string into a [`Color`].
///
/// Returns `None` when the string is not a colour the parser
/// understands.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<Color>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn parse_css_color(input: &str) -> Option<Color> {
    let text: &str = input.trim();
    if text.is_empty() {
        return None;
    }
    let lower: String = text.to_ascii_lowercase();
    if let Some(hex) = lower.strip_prefix('#') {
        return parse_hex(hex);
    }
    if lower == "transparent" {
        return Some(Color::rgba(0, 0, 0, 0));
    }
    if let Some(body) = strip_function(&lower, "rgba(") {
        let parts: Vec<String> = split_arguments(body);
        if parts.len() == 4 {
            let red: u8 = parse_channel(&parts[0])?;
            let green: u8 = parse_channel(&parts[1])?;
            let blue: u8 = parse_channel(&parts[2])?;
            let alpha: u8 = parse_alpha(&parts[3])?;
            return Some(Color::rgba(red, green, blue, alpha));
        }
        return None;
    }
    if let Some(body) = strip_function(&lower, "rgb(") {
        let parts: Vec<String> = split_arguments(body);
        if parts.len() == 3 {
            let red: u8 = parse_channel(&parts[0])?;
            let green: u8 = parse_channel(&parts[1])?;
            let blue: u8 = parse_channel(&parts[2])?;
            return Some(Color::rgb(red, green, blue));
        }
        return None;
    }
    if let Some(body) = strip_function(&lower, "hsla(") {
        let parts: Vec<String> = split_arguments(body);
        if parts.len() == 4 {
            let (red, green, blue) = parse_hsl(&parts[0], &parts[1], &parts[2])?;
            let alpha: u8 = parse_alpha(&parts[3])?;
            return Some(Color::rgba(red, green, blue, alpha));
        }
        return None;
    }
    if let Some(body) = strip_function(&lower, "hsl(") {
        let parts: Vec<String> = split_arguments(body);
        if parts.len() == 3 {
            let (red, green, blue) = parse_hsl(&parts[0], &parts[1], &parts[2])?;
            return Some(Color::rgb(red, green, blue));
        }
        return None;
    }
    named_color(&lower)
}

/// Strips a function name and its closing parenthesis from a string.
///
/// # Arguments
///
/// - `&'a str` - Shared reference to a `'a str`.
/// - `&str` - Shared reference to a `'a str`.
///
/// # Returns
///
/// - `Option<&'a str>` - `Some(...)` on success, `None` otherwise.
fn strip_function<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    let body: &str = text.strip_prefix(prefix)?;
    body.strip_suffix(')')
}

/// Splits the arguments of a colour function.
///
/// Both the legacy comma-separated form and the modern space-separated
/// form with an optional slash before the alpha are accepted.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Vec<String>` - A `Vec<String>` value.
fn split_arguments(body: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current: String = String::new();
    let mut depth: usize = 0usize;
    for character in body.chars() {
        match character {
            '(' => {
                depth += 1;
                current.push(character);
            }
            ')' => {
                depth = depth.saturating_sub(1);
                current.push(character);
            }
            ',' if depth == 0 => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            '/' if depth == 0 => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(character),
        }
    }
    let tail: &str = current.trim();
    if !tail.is_empty() {
        parts.push(tail.to_string());
    }
    parts
}

/// Parses a hex colour body, without the leading `#`.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<Color>` - `Some(...)` on success, `None` otherwise.
fn parse_hex(hex: &str) -> Option<Color> {
    if !hex
        .chars()
        .all(|character: char| character.is_ascii_hexdigit())
    {
        return None;
    }
    match hex.len() {
        3 => {
            let red: u8 = expand_hex(hex.as_bytes()[0])?;
            let green: u8 = expand_hex(hex.as_bytes()[1])?;
            let blue: u8 = expand_hex(hex.as_bytes()[2])?;
            Some(Color::rgb(red, green, blue))
        }
        4 => {
            let red: u8 = expand_hex(hex.as_bytes()[0])?;
            let green: u8 = expand_hex(hex.as_bytes()[1])?;
            let blue: u8 = expand_hex(hex.as_bytes()[2])?;
            let alpha: u8 = expand_hex(hex.as_bytes()[3])?;
            Some(Color::rgba(red, green, blue, alpha))
        }
        6 => {
            let red: u8 = hex_byte(&hex[0..2])?;
            let green: u8 = hex_byte(&hex[2..4])?;
            let blue: u8 = hex_byte(&hex[4..6])?;
            Some(Color::rgb(red, green, blue))
        }
        8 => {
            let red: u8 = hex_byte(&hex[0..2])?;
            let green: u8 = hex_byte(&hex[2..4])?;
            let blue: u8 = hex_byte(&hex[4..6])?;
            let alpha: u8 = hex_byte(&hex[6..8])?;
            Some(Color::rgba(red, green, blue, alpha))
        }
        _ => None,
    }
}

/// Expands one hex digit into a full byte.
///
/// # Arguments
///
/// - `u8` - A `u8` parameter.
///
/// # Returns
///
/// - `Option<u8>` - `Some(...)` on success, `None` otherwise.
fn expand_hex(digit: u8) -> Option<u8> {
    let value: u8 = (digit as char).to_digit(16)? as u8;
    Some(value * 17)
}

/// Parses a two-digit hex byte.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<u8>` - `Some(...)` on success, `None` otherwise.
fn hex_byte(text: &str) -> Option<u8> {
    u8::from_str_radix(text, 16).ok()
}

/// Parses one channel of an `rgb()` or `rgba()` colour.
///
/// Percentages are scaled against 255; bare numbers are clamped.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<u8>` - `Some(...)` on success, `None` otherwise.
fn parse_channel(text: &str) -> Option<u8> {
    let trimmed: &str = text.trim();
    if let Some(percent) = trimmed.strip_suffix('%') {
        let value: f64 = percent.trim().parse::<f64>().ok()?;
        return Some((value / 100.0 * 255.0).round().clamp(0.0, 255.0) as u8);
    }
    let value: f64 = trimmed.parse::<f64>().ok()?;
    Some(value.round().clamp(0.0, 255.0) as u8)
}

/// Parses the alpha channel of an `rgba()` or `hsla()` colour.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<u8>` - `Some(...)` on success, `None` otherwise.
fn parse_alpha(text: &str) -> Option<u8> {
    let trimmed: &str = text.trim();
    if let Some(percent) = trimmed.strip_suffix('%') {
        let value: f64 = percent.trim().parse::<f64>().ok()?;
        return Some((value / 100.0 * 255.0).round().clamp(0.0, 255.0) as u8);
    }
    let value: f64 = trimmed.parse::<f64>().ok()?;
    Some((value.clamp(0.0, 1.0) * 255.0).round() as u8)
}

/// Parses an `hsl()` triplet into an RGB triplet.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<(u8, u8, u8)>` - `Some(...)` on success, `None` otherwise.
fn parse_hsl(hue: &str, saturation: &str, lightness: &str) -> Option<(u8, u8, u8)> {
    let hue: f64 = hue.trim().parse::<f64>().ok()?;
    let saturation: f64 = percent_value(saturation)?;
    let lightness: f64 = percent_value(lightness)?;
    let hue: f64 = ((hue % 360.0) + 360.0) % 360.0;
    let saturation: f64 = saturation.clamp(0.0, 1.0);
    let lightness: f64 = lightness.clamp(0.0, 1.0);
    let chroma: f64 = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let secondary: f64 = chroma * (1.0 - (((hue / 60.0) % 2.0) - 1.0).abs());
    let match_value: f64 = lightness - chroma / 2.0;
    let (red, green, blue) = match hue as u32 / 60 {
        0 => (chroma, secondary, 0.0),
        1 => (secondary, chroma, 0.0),
        2 => (0.0, chroma, secondary),
        3 => (0.0, secondary, chroma),
        4 => (secondary, 0.0, chroma),
        _ => (chroma, 0.0, secondary),
    };
    Some((
        ((red + match_value) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((green + match_value) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((blue + match_value) * 255.0).round().clamp(0.0, 255.0) as u8,
    ))
}

/// Parses a percentage value into the range `0.0` to `1.0`.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<f64>` - `Some(...)` on success, `None` otherwise.
fn percent_value(text: &str) -> Option<f64> {
    let trimmed: &str = text.trim();
    let value = match trimmed.strip_suffix('%') {
        Some(percent) => percent.trim().parse::<f64>().ok()? / 100.0,
        None => trimmed.parse::<f64>().ok()?,
    };
    Some(value)
}

/// Looks up a CSS colour keyword.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `Option<Color>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn named_color(name: &str) -> Option<Color> {
    NAMED_COLORS
        .binary_search_by(|(candidate, _): &(&str, u32)| (*candidate).cmp(name))
        .ok()
        // The table holds `0xRRGGBB`; the alpha byte is filled in so
        // that a named colour is fully opaque.
        .map(|index: usize| Color::from_u32((NAMED_COLORS[index].1 << 8) | 0xFF))
}
