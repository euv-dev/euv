use super::*;

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
