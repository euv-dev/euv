use super::*;

fn track_property(class: &Css, property: &str) -> Option<String> {
    class
        .get_style()
        .split(';')
        .filter_map(|part: &str| part.split_once(':'))
        .filter(|(name, _): &(&str, &str)| name.trim() == property)
        .map(|(_, value): (&str, &str)| value.trim().to_string())
        .next_back()
}

#[test]
fn horizontal_tracks_use_length_radii_instead_of_elliptical_percentage_radii() {
    for class in [
        c_euv_slider_track(),
        c_euv_slider_range(),
        c_progress_bar_track(),
    ] {
        {
            assert_eq!(
                track_property(class, "border-radius").as_deref(),
                Some("var(--radius-sm)")
            );
        }
    }
}

#[test]
fn track_and_fill_heights_remain_consistent() {
    assert_eq!(
        track_property(c_euv_slider_track(), "height").as_deref(),
        Some("var(--space-xs)")
    );
    assert_eq!(
        track_property(c_euv_slider_range(), "height").as_deref(),
        Some("100%")
    );
    assert_eq!(
        track_property(c_progress_bar_fill(), "height").as_deref(),
        Some("100%")
    );
    let expected: &str = if cfg!(feature = "theme-shadcn") {
        "var(--space-xs)"
    } else {
        "12px"
    };
    assert_eq!(
        track_property(c_progress_bar_track(), "height").as_deref(),
        Some(expected)
    );
}

#[test]
fn circular_slider_thumb_is_not_flattened_with_the_tracks() {
    assert_eq!(
        track_property(c_euv_slider_thumb(), "border-radius").as_deref(),
        Some("var(--radius-full)")
    );
    assert_eq!(
        track_property(c_euv_slider_thumb(), "width"),
        track_property(c_euv_slider_thumb(), "height")
    );
}
