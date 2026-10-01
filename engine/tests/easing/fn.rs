use super::*;

fn every_variant() -> Vec<Easing> {
    vec![
        Easing::Linear,
        Easing::InQuad,
        Easing::OutQuad,
        Easing::InOutQuad,
        Easing::InCubic,
        Easing::OutCubic,
        Easing::InOutCubic,
        Easing::InQuart,
        Easing::OutQuart,
        Easing::InOutQuart,
        Easing::InQuint,
        Easing::OutQuint,
        Easing::InOutQuint,
        Easing::InSine,
        Easing::OutSine,
        Easing::InOutSine,
        Easing::InExpo,
        Easing::OutExpo,
        Easing::InOutExpo,
        Easing::InCirc,
        Easing::OutCirc,
        Easing::InOutCirc,
        Easing::InBack,
        Easing::OutBack,
        Easing::InOutBack,
        Easing::InElastic,
        Easing::OutElastic,
        Easing::InOutElastic,
        Easing::InBounce,
        Easing::OutBounce,
        Easing::InOutBounce,
    ]
}

#[test]
fn every_easing_starts_at_zero_and_ends_at_one() {
    for easing in every_variant() {
        let start: f64 = easing.evaluate(0.0);
        let end: f64 = easing.evaluate(1.0);
        assert!(
            start.abs() < 1e-6,
            "{easing:?} must start at zero, got {start}"
        );
        assert!(
            (end - 1.0).abs() < 1e-6,
            "{easing:?} must end at one, got {end}"
        );
    }
}

#[test]
fn every_easing_stays_finite_across_the_unit_interval() {
    for easing in every_variant() {
        for step in 0..=20 {
            let t: f64 = step as f64 / 20.0;
            let value: f64 = easing.evaluate(t);
            assert!(
                value.is_finite(),
                "{easing:?} produced a non-finite value at t={t}"
            );
        }
    }
}

#[test]
fn linear_easing_is_the_identity_curve() {
    for step in 0..=10 {
        let t: f64 = step as f64 / 10.0;
        assert!(
            (Easing::Linear.evaluate(t) - t).abs() < 1e-9,
            "linear easing must return t unchanged"
        );
    }
}

#[test]
fn quad_easing_matches_its_closed_form() {
    let t: f64 = 0.25;
    assert!(
        (Easing::InQuad.evaluate(t) - t * t).abs() < 1e-9,
        "InQuad must equal t squared"
    );
    assert!(
        (Easing::OutQuad.evaluate(t) - (1.0 - (1.0 - t) * (1.0 - t))).abs() < 1e-9,
        "OutQuad must equal the mirrored tail of t squared"
    );
}

#[test]
fn in_out_variants_are_symmetric_about_the_midpoint() {
    let lower: f64 = Easing::InOutQuad.evaluate(0.25);
    let upper: f64 = Easing::InOutQuad.evaluate(0.75);
    assert!(
        (lower + upper - 1.0).abs() < 1e-6,
        "InOutQuad must be symmetric about the midpoint"
    );
    assert!(
        (Easing::InOutCubic.evaluate(0.25) + Easing::InOutCubic.evaluate(0.75) - 1.0).abs() < 1e-6,
        "InOutCubic must be symmetric about the midpoint"
    );
}

#[test]
fn in_variants_dwell_near_the_start_and_out_variants_near_the_end() {
    let in_quad: f64 = Easing::InQuad.evaluate(0.25);
    let out_quad: f64 = Easing::OutQuad.evaluate(0.25);
    assert!(
        in_quad < out_quad,
        "InQuad must trail OutQuad early on, got {in_quad} and {out_quad}"
    );
    let in_cubic: f64 = Easing::InCubic.evaluate(0.25);
    let out_cubic: f64 = Easing::OutCubic.evaluate(0.25);
    assert!(
        in_cubic < out_cubic,
        "InCubic must trail OutCubic early on, got {in_cubic} and {out_cubic}"
    );
}

#[test]
fn back_and_elastic_variants_deliberately_overshoot_the_unit_range() {
    let mut saw_overshoot: bool = false;
    for easing in [
        Easing::InBack,
        Easing::OutBack,
        Easing::InOutBack,
        Easing::InElastic,
        Easing::OutElastic,
        Easing::InOutElastic,
    ] {
        for step in 0..=40 {
            let t: f64 = step as f64 / 40.0;
            let value: f64 = easing.evaluate(t);
            if value < -1e-6 || value > 1.0 + 1e-6 {
                saw_overshoot = true;
            }
        }
    }
    assert!(
        saw_overshoot,
        "back and elastic easings must be allowed to leave the unit range"
    );
}

#[test]
fn bounce_variants_stay_within_the_unit_range_and_reach_one_at_the_end() {
    for easing in [Easing::InBounce, Easing::OutBounce, Easing::InOutBounce] {
        for step in 0..=40 {
            let t: f64 = step as f64 / 40.0;
            let value: f64 = easing.evaluate(t);
            assert!(
                (-1e-6..=1.0 + 1e-6).contains(&value),
                "{easing:?} must stay within the unit range, got {value} at t={t}"
            );
        }
    }
}

#[test]
fn interpolate_maps_the_curve_onto_the_given_endpoints() {
    let start: f64 = 10.0;
    let end: f64 = 20.0;
    assert!(
        (Easing::Linear.interpolate(start, end, 0.0) - start).abs() < 1e-9,
        "progress zero must return the start value"
    );
    assert!(
        (Easing::Linear.interpolate(start, end, 1.0) - end).abs() < 1e-9,
        "progress one must return the end value"
    );
    let middle: f64 = Easing::Linear.interpolate(start, end, 0.5);
    assert!(
        (middle - 15.0).abs() < 1e-9,
        "linear interpolation at the midpoint must land halfway, got {middle}"
    );
}

#[test]
fn interpolate_tolerates_a_reversed_range() {
    let forward: f64 = Easing::Linear.interpolate(0.0, 1.0, 0.5);
    let backward: f64 = Easing::Linear.interpolate(1.0, 0.0, 0.5);
    assert!(
        (forward + backward - 1.0).abs() < 1e-9,
        "reversing the endpoints must mirror the result"
    );
}

#[test]
fn the_default_variant_is_linear() {
    assert_eq!(
        Easing::default(),
        Easing::Linear,
        "the engine default easing must be Linear"
    );
}
