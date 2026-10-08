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
fn every_easing_curve_evaluates_to_zero_at_time_zero() {
    let all: [Easing; 31] = [
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
    ];
    for easing in all {
        let observed: f64 = easing.evaluate(0.0);
        assert!(
            observed.abs() < 1e-9,
            "{:?} must start at zero, got {observed}",
            "easing"
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
fn every_easing_curve_evaluates_to_one_at_time_one() {
    let all: [Easing; 31] = [
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
    ];
    for easing in all {
        let observed: f64 = easing.evaluate(1.0);
        assert!(
            (observed - 1.0).abs() < 1e-9,
            "{:?} must land on one, got {observed}",
            "easing"
        );
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

#[test]
fn every_easing_curve_clamps_a_negative_time_onto_the_zero_sample() {
    let all: [Easing; 31] = [
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
    ];
    for easing in all {
        let clamped: f64 = easing.evaluate(-0.75);
        let baseline: f64 = easing.evaluate(0.0);
        assert_eq!(
            clamped, baseline,
            "{:?} must clamp a negative time to its zero sample",
            "easing"
        );
    }
}

#[test]
fn every_easing_curve_clamps_a_time_above_one_onto_the_one_sample() {
    let all: [Easing; 31] = [
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
    ];
    for easing in all {
        let clamped: f64 = easing.evaluate(2.5);
        let baseline: f64 = easing.evaluate(1.0);
        assert_eq!(
            clamped, baseline,
            "{:?} must clamp an out-of-range high time to its one sample",
            "easing"
        );
    }
}

#[test]
fn every_easing_curve_lands_on_the_start_value_at_time_zero() {
    let all: [Easing; 31] = [
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
    ];
    for easing in all {
        let observed: f64 = easing.interpolate(-4.0, 12.0, 0.0);
        assert!(
            (observed - -4.0).abs() < 1e-9,
            "{:?} must interpolate onto the start value, got {observed}",
            "easing"
        );
    }
}

#[test]
fn every_easing_curve_lands_on_the_end_value_at_time_one() {
    let all: [Easing; 31] = [
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
    ];
    for easing in all {
        let observed: f64 = easing.interpolate(-4.0, 12.0, 1.0);
        assert!(
            (observed - 12.0).abs() < 1e-9,
            "{:?} must interpolate onto the end value, got {observed}",
            "easing"
        );
    }
}

#[test]
fn interpolate_is_the_lerp_of_the_evaluated_progress() {
    let all: [Easing; 31] = [
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
    ];
    for easing in all {
        let progress: f64 = easing.evaluate(0.37);
        let expected: f64 = 2.0 + (20.0 - 2.0) * progress;
        let observed: f64 = easing.interpolate(2.0, 20.0, 0.37);
        assert!(
            (observed - expected).abs() < 1e-9,
            "{:?} must lerp by its own evaluated progress, expected {expected} got {observed}",
            "easing"
        );
    }
}

#[test]
fn linear_is_the_identity_curve_across_the_whole_range() {
    let samples: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];
    for t in samples {
        let observed: f64 = Easing::Linear.evaluate(t);
        assert_eq!(observed, t, "Linear must pass {t} through unchanged");
    }
}

#[test]
fn linear_interpolates_as_a_plain_lerp() {
    let observed: f64 = Easing::Linear.interpolate(10.0, 20.0, 0.25);
    assert_eq!(observed, 12.5, "a quarter of the way from 10 to 20 is 12.5");
}

#[test]
fn default_variant_is_linear() {
    let observed: Easing = Easing::default();
    assert_eq!(
        observed,
        Easing::Linear,
        "the derive default must be the constant-velocity curve"
    );
}

#[test]
fn in_quad_squares_the_progress_and_out_quad_mirrors_it() {
    let eased_in: f64 = Easing::InQuad.evaluate(0.25);
    let eased_out: f64 = Easing::OutQuad.evaluate(0.25);
    assert_eq!(eased_in, 0.0625, "InQuad is t squared");
    assert_eq!(eased_out, 0.4375, "OutQuad is 1 - (1 - t) squared");
}

#[test]
fn in_out_quad_passes_through_the_halfway_point() {
    let observed: f64 = Easing::InOutQuad.evaluate(0.5);
    assert_eq!(observed, 0.5, "the InOut family is symmetric at t = 0.5");
}

#[test]
fn in_out_cubic_passes_through_the_halfway_point() {
    let observed: f64 = Easing::InOutCubic.evaluate(0.5);
    assert_eq!(observed, 0.5, "the InOut family is symmetric at t = 0.5");
}

#[test]
fn expo_curves_hold_their_endpoints_instead_of_the_pure_power() {
    let in_mid: f64 = Easing::InExpo.evaluate(0.5);
    let out_mid: f64 = Easing::OutExpo.evaluate(0.5);
    assert!(
        in_mid > 0.0 && in_mid < 1.0,
        "InExpo at the midpoint must stay inside the unit range, got {in_mid}"
    );
    assert_eq!(1.0 - in_mid, out_mid, "InExpo and OutExpo must mirror");
}

#[test]
fn in_bounce_is_the_time_reversed_out_bounce() {
    for t in [0.1_f64, 0.3, 0.5, 0.7, 0.9] {
        let reversed: f64 = Easing::InBounce.evaluate(1.0 - t);
        let expected: f64 = 1.0 - Easing::OutBounce.evaluate(t);
        assert!(
            (reversed - expected).abs() < 1e-9,
            "InBounce(1 - {t}) must equal 1 - OutBounce({t}), expected {expected} got {reversed}"
        );
    }
}

#[test]
fn in_out_bounce_passes_through_the_halfway_point() {
    let observed: f64 = Easing::InOutBounce.evaluate(0.5);
    assert_eq!(observed, 0.5, "the InOut family is symmetric at t = 0.5");
}
