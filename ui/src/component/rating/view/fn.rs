use super::*;

/// Returns the fill percentage of a rating value, clamped to `0.0..=100.0`.
///
/// A non-positive `max` yields `0.0` so a rating whose scale has not been set
/// yet renders as empty rather than dividing by zero.
///
/// # Arguments
///
/// - `f64` - The rating value to normalise.
/// - `f64` - The maximum rating value the percentage is relative to.
///
/// # Returns
///
/// - `f64` - `value / max * 100.0` clamped to the inclusive range `0.0..=100.0`.
pub fn rating_fill_percent(value: f64, max: f64) -> f64 {
    if max <= 0.0 {
        return 0.0;
    }
    (value / max * 100.0).clamp(0.0, 100.0)
}

/// A star rating supporting read-only display and half-star fill.
///
/// Renders a `div` with the base `c_euv_rating` class, carrying `role="img"`
/// and an `aria-label` so the rating is announced as a single value rather
/// than as a list of glyphs. One `c_euv_rating_star` span is rendered per
/// unit — filled `★` up to the rounded value, empty `☆` beyond it — followed
/// by a `c_euv_rating_fill` overlay whose inline `width` is the clamped
/// percentage, producing the fractional (half-star) effect. The overlay is
/// rendered for interactive ratings; a read-only rating relies on the per-star
/// glyphs alone.
///
/// # Arguments
///
/// - `VirtualNode<EuvRatingProps>` - The props node carrying the rating configuration.
///
/// # Returns
///
/// - `VirtualNode` - The component virtual DOM tree.
#[component]
pub fn euv_rating(node: VirtualNode<EuvRatingProps>) -> VirtualNode {
    let EuvRatingProps {
        value,
        max,
        readonly,
        size,
    }: EuvRatingProps = node.try_get_props().unwrap_or_default();
    let current: f64 = value.get();
    let scale: f64 = max.get();
    let percent: f64 = rating_fill_percent(current, scale);
    let units: i32 = if scale <= 0.0 { 0 } else { scale as i32 };
    let filled: i32 = ((current + 0.5).floor() as i32).clamp(0, units);
    let px: i32 = size.px();
    let star_style: String = format!("font-size: {px}px;");
    let fill_style: String = format!("width: {percent}%;");
    let aria_label: String = format!("{} of {} stars", current, scale);
    html! {
        div {
            class: c_euv_rating()
            role: ROLE_IMG
            aria-label: aria_label
            for index in 0..units {
                span {
                    key: index.to_string()
                    class: c_euv_rating_star()
                    style: star_style.clone()
                    if index < filled {
                        "★"
                    } else {
                        "☆"
                    }
                }
            }
            if { !readonly.get() } {
                div {
                    class: c_euv_rating_fill()
                    style: fill_style.clone()
                }
            }
        }
    }
}
