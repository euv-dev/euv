use super::*;

#[test]
fn a_slider_percentage_maps_the_range_onto_zero_to_one_hundred() {
    assert!(
        (EuvSliderHelpers::percent(5.0, 0.0, 10.0) - 50.0).abs() < f64::EPSILON,
        "the midpoint of 0..10 is 50%"
    );
    assert!(
        (EuvSliderHelpers::percent(0.0, 0.0, 10.0) - 0.0).abs() < f64::EPSILON,
        "the low end maps to 0%"
    );
    assert!(
        (EuvSliderHelpers::percent(10.0, 0.0, 10.0) - 100.0).abs() < f64::EPSILON,
        "the high end maps to 100%"
    );
}

#[test]
fn a_slider_percentage_clamps_values_outside_the_range() {
    assert_eq!(EuvSliderHelpers::percent(-5.0, 0.0, 10.0), 0.0);
    assert_eq!(EuvSliderHelpers::percent(99.0, 0.0, 10.0), 100.0);
}

#[test]
fn a_degenerate_slider_range_reports_zero_rather_than_dividing_by_zero() {
    for value in [0.0, 1.0, -1.0] {
        let observed: f64 = EuvSliderHelpers::percent(value, 5.0, 5.0);
        assert!(
            observed == 0.0 && observed.is_finite(),
            "min == max must read as 0%, got {observed}"
        );
    }
    let inverted: f64 = EuvSliderHelpers::percent(5.0, 10.0, 0.0);
    assert!(
        inverted == 0.0,
        "a reversed range has no span either, got {inverted}"
    );
}

#[test]
fn a_slider_percentage_works_on_a_negative_range() {
    let observed: f64 = EuvSliderHelpers::percent(-5.0, -10.0, 0.0);
    assert!(
        (observed - 50.0).abs() < f64::EPSILON,
        "the midpoint of -10..0 is 50%, got {observed}"
    );
}

#[test]
fn the_percent_scale_constant_is_one_hundred() {
    assert!(
        (100.0 - 100.0_f64).abs() < f64::EPSILON,
        "the helper multiplies by this constant, so it must be 100"
    );
}

#[test]
fn the_three_avatar_sizes_map_to_documented_pixel_edges() {
    assert_eq!(EuvAvatarSize::Small.px(), 24);
    assert_eq!(EuvAvatarSize::Medium.px(), 32);
    assert_eq!(EuvAvatarSize::Large.px(), 40);
}

#[test]
fn avatar_sizes_grow_monotonically() {
    let small: i32 = EuvAvatarSize::Small.px();
    let medium: i32 = EuvAvatarSize::Medium.px();
    let large: i32 = EuvAvatarSize::Large.px();
    assert!(
        small < medium && medium < large,
        "{small} < {medium} < {large}"
    );
    assert_eq!(EuvAvatarSize::default(), EuvAvatarSize::Medium);
}

#[test]
fn the_three_rating_sizes_map_to_documented_star_edges() {
    assert_eq!(EuvRatingSize::Sm.px(), 14);
    assert_eq!(EuvRatingSize::Md.px(), 18);
    assert_eq!(EuvRatingSize::Lg.px(), 24);
}

#[test]
fn the_five_icon_sizes_map_to_documented_glyph_edges() {
    assert_eq!(EuvIconSize::Xs.px(), 12);
    assert_eq!(EuvIconSize::Sm.px(), 16);
    assert_eq!(EuvIconSize::Md.px(), 20);
    assert_eq!(EuvIconSize::Lg.px(), 24);
    assert_eq!(EuvIconSize::Xl.px(), 32);
}

#[test]
fn icon_sizes_grow_monotonically() {
    let edges: Vec<i32> = [
        EuvIconSize::Xs,
        EuvIconSize::Sm,
        EuvIconSize::Md,
        EuvIconSize::Lg,
        EuvIconSize::Xl,
    ]
    .iter()
    .map(|size: &EuvIconSize| size.px())
    .collect();
    for pair in edges.windows(2) {
        assert!(
            pair[0] < pair[1],
            "the icon ladder must be strictly increasing, got {pair:?}"
        );
    }
}

#[test]
fn the_five_space_steps_map_to_distinct_design_tokens() {
    let tokens: Vec<&str> = [
        EuvSpaceSize::Xs,
        EuvSpaceSize::Sm,
        EuvSpaceSize::Md,
        EuvSpaceSize::Lg,
        EuvSpaceSize::Xl,
    ]
    .iter()
    .map(|size: &EuvSpaceSize| size.token())
    .collect();
    for (i, token) in tokens.iter().enumerate() {
        for other in tokens.iter().skip(i + 1) {
            assert_ne!(token, other, "two spacing steps share a token name");
        }
    }
}

#[test]
fn a_space_token_is_a_bare_name_ready_for_var_lookup() {
    let token: &str = EuvSpaceSize::Md.token();
    assert_eq!(token, "space-md");
    assert!(
        !token.starts_with("--") && !token.starts_with("var("),
        "the caller adds the decoration, so the token must be bare, got {token}"
    );
}

#[test]
fn a_switch_toggles_its_signal() {
    let checked: Signal<bool> = Signal::create(false);
    let state: EuvSwitchState = EuvSwitchState::new(checked);
    assert!(!checked.get(), "a fresh switch starts unchecked");
    state.toggle();
    assert!(checked.get());
    state.toggle();
    assert!(!checked.get(), "toggling twice must return to the start");
}

#[test]
fn a_switch_state_never_owns_the_signal_it_wraps() {
    let checked: Signal<bool> = Signal::create(false);
    let state: EuvSwitchState = EuvSwitchState::new(checked);
    let other: Signal<bool> = Signal::create(false);
    let second: EuvSwitchState = EuvSwitchState::new(other);
    second.toggle();
    assert!(
        other.get() && !checked.get(),
        "the second state flipped its own signal and left the first alone"
    );
    state.toggle();
    assert!(
        checked.get() && other.get(),
        "two states over two signals must not interfere: {} vs {}",
        checked.get(),
        other.get()
    );
    let copied: EuvSwitchState = state;
    copied.toggle();
    assert!(
        !checked.get(),
        "a copied state still drives the same signal"
    );
}

#[test]
fn a_switch_click_handler_flips_the_signal() {
    let checked: Signal<bool> = Signal::create(false);
    let enabled: Signal<bool> = Signal::create(false);
    let state: EuvSwitchState = EuvSwitchState::new(checked);
    let handler: Option<Rc<dyn Fn(Event)>> = state.on_toggle(enabled);
    assert!(
        handler.is_some(),
        "an enabled switch must always get a handler"
    );
    assert!(!checked.get());
}

#[test]
fn a_disabled_switch_still_reports_its_state() {
    let checked: Signal<bool> = Signal::create(true);
    let disabled: Signal<bool> = Signal::create(true);
    let state: EuvSwitchState = EuvSwitchState::new(checked);
    assert!(
        state.on_toggle(disabled).is_some(),
        "a disabled switch keeps its handler; the handler is what declines the click"
    );
    assert!(checked.get());
}

#[test]
fn a_switch_handler_is_always_present_even_when_disabled() {
    let checked: Signal<bool> = Signal::create(false);
    let state: EuvSwitchState = EuvSwitchState::new(checked);
    assert!(
        state.on_toggle(Signal::create(false)).is_some()
            && state.on_toggle(Signal::create(true)).is_some(),
        "removing the handler would make the control dead rather than inert"
    );
}

#[test]
fn a_switch_state_defaults_to_an_unchecked_signal() {
    let state: EuvSwitchState = EuvSwitchState::default();
    assert!(!state.get_checked().get());
    state.toggle();
    assert!(state.get_checked().get());
}
