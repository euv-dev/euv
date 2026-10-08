use super::*;

fn recognizer() -> EuvGestureRecognizer {
    EuvGestureRecognizer::new()
}

#[test]
fn the_default_thresholds_are_tuned_for_a_fingertip() {
    let config: EuvGestureConfig = EuvGestureConfig::default();
    assert_eq!(
        config.swipe_threshold, 48.0,
        "roughly one fingertip contact patch"
    );
    assert_eq!(
        config.tap_slop, 10.0,
        "sub-slop travel is jitter, not intent"
    );
    assert_eq!(
        config.long_press_millis, 500.0,
        "the iOS/Android long-press boundary"
    );
    assert!(
        config.pinch_threshold > 0.0,
        "a zero pinch threshold would report every wobble"
    );
}

#[test]
fn a_stationary_touch_is_a_tap() {
    let observed: Option<EuvGesture> = recognizer().classify(0.0, 0.0, 50.0, 0.0);
    assert_eq!(observed, Some(EuvGesture::Tap));
}

#[test]
fn a_stationary_touch_held_past_the_threshold_is_a_long_press() {
    let observed: Option<EuvGesture> = recognizer().classify(0.0, 0.0, 600.0, 0.0);
    assert_eq!(observed, Some(EuvGesture::LongPress));
}

#[test]
fn a_large_cumulative_travel_excludes_a_tap_even_when_the_net_move_is_small() {
    let observed: Option<EuvGesture> = recognizer().classify(1.0, 1.0, 50.0, 200.0);
    assert_ne!(
        observed,
        Some(EuvGesture::Tap),
        "a finger that wandered 200px and came back did not tap"
    );
}

#[test]
fn a_movement_under_the_swipe_threshold_is_neither_tap_nor_swipe() {
    let observed: Option<EuvGesture> = recognizer().classify(20.0, 0.0, 50.0, 20.0);
    assert_eq!(
        observed, None,
        "past the tap slop but short of the swipe threshold is dead zone, not a gesture"
    );
}

#[test]
fn a_rightward_swipe_is_classified_by_its_dominant_axis() {
    assert_eq!(
        recognizer().classify(100.0, 5.0, 50.0, 100.0),
        Some(EuvGesture::Right)
    );
}

#[test]
fn a_leftward_swipe_mirrors_the_right_one() {
    assert_eq!(
        recognizer().classify(-100.0, 5.0, 50.0, 100.0),
        Some(EuvGesture::Left)
    );
}

#[test]
fn a_downward_swipe_is_classified_by_its_dominant_axis() {
    assert_eq!(
        recognizer().classify(5.0, 100.0, 50.0, 100.0),
        Some(EuvGesture::Down)
    );
}

#[test]
fn an_upward_swipe_mirrors_the_down_one() {
    assert_eq!(
        recognizer().classify(5.0, -100.0, 50.0, 100.0),
        Some(EuvGesture::Up)
    );
}

#[test]
fn a_diagonal_swipe_follows_the_larger_component() {
    assert_eq!(
        recognizer().classify(100.0, 90.0, 50.0, 150.0),
        Some(EuvGesture::Right),
        "x wins when it is the larger component"
    );
    assert_eq!(
        recognizer().classify(90.0, 100.0, 50.0, 150.0),
        Some(EuvGesture::Down),
        "y wins when it is the larger component"
    );
}

#[test]
fn a_tightened_swipe_threshold_reclassifies_a_previous_dead_zone() {
    let loose: EuvGestureRecognizer = EuvGestureRecognizer::new();
    let config: EuvGestureConfig = EuvGestureConfig {
        swipe_threshold: 10.0,
        ..EuvGestureConfig::default()
    };
    let tight: EuvGestureRecognizer = EuvGestureRecognizer::with_config(config);
    assert_eq!(loose.classify(20.0, 0.0, 50.0, 20.0), None);
    assert_eq!(
        tight.classify(20.0, 0.0, 50.0, 20.0),
        Some(EuvGesture::Right),
        "the thresholds are per-config, so a carousel can be made more eager"
    );
}

#[test]
fn a_pinch_is_significant_only_past_its_relative_threshold() {
    let r: EuvGestureRecognizer = recognizer();
    assert!(
        r.pinch_is_significant(200.0, 100.0),
        "doubling the distance is well past any sane threshold"
    );
    assert!(
        !r.pinch_is_significant(100.5, 100.0),
        "a half-percent change is sub-percent jitter between adjacent touchmove events"
    );
}

#[test]
fn the_pinch_threshold_boundary_is_inclusive() {
    let r: EuvGestureRecognizer = recognizer();
    let config: EuvGestureConfig = EuvGestureConfig::default();
    assert!(
        (config.pinch_threshold - 0.01).abs() < f64::EPSILON,
        "the default is a 1% relative change"
    );
    assert!(
        r.pinch_is_significant(101.0, 100.0),
        "the comparison is >=, so a change of exactly the threshold counts as significant — \
         only SUB-percent wobble is suppressed"
    );
    assert!(!r.pinch_is_significant(100.99, 100.0));
}

#[test]
fn a_pinch_is_significant_in_both_directions() {
    let r: EuvGestureRecognizer = recognizer();
    assert!(
        r.pinch_is_significant(50.0, 100.0),
        "pinching in is as significant as pinching out"
    );
}

#[test]
fn a_pinch_from_a_zero_baseline_is_never_significant() {
    let r: EuvGestureRecognizer = recognizer();
    assert!(
        !r.pinch_is_significant(100.0, 0.0),
        "dividing by a zero baseline would yield infinity, not a decision"
    );
}

#[test]
fn a_drag_reports_its_position_delta_and_accumulated_travel() {
    let point: GesturePoint = GesturePoint {
        client_x: 120.0,
        client_y: 40.0,
    };
    let drag: EuvDrag = recognizer().drag_from(&point, 100.0, 10.0, 55.0);
    assert_eq!(drag.x, 120.0);
    assert_eq!(drag.y, 40.0);
    assert_eq!(
        drag.delta_x, 20.0,
        "dx is measured from where the drag began"
    );
    assert_eq!(drag.delta_y, 30.0);
    assert_eq!(
        drag.travel, 55.0,
        "travel is the path length, not the net offset"
    );
}

#[test]
fn a_drag_that_has_not_moved_reports_a_zero_delta() {
    let point: GesturePoint = GesturePoint {
        client_x: 50.0,
        client_y: 60.0,
    };
    let drag: EuvDrag = recognizer().drag_from(&point, 50.0, 60.0, 0.0);
    assert_eq!(drag.delta_x, 0.0);
    assert_eq!(drag.delta_y, 0.0);
}

#[test]
fn every_gesture_has_its_documented_wire_name() {
    assert_eq!(EuvGesture::Left.name(), "left");
    assert_eq!(EuvGesture::Right.name(), "right");
    assert_eq!(EuvGesture::Up.name(), "up");
    assert_eq!(EuvGesture::Down.name(), "down");
    assert_eq!(EuvGesture::Tap.name(), "tap");
    assert_eq!(
        EuvGesture::LongPress.name(),
        "long-press",
        "the telemetry and data-gesture spellings are a public contract"
    );
}

#[test]
fn the_gesture_wire_names_are_unique() {
    let names: Vec<&'static str> = [
        EuvGesture::Left,
        EuvGesture::Right,
        EuvGesture::Up,
        EuvGesture::Down,
        EuvGesture::Tap,
        EuvGesture::LongPress,
    ]
    .iter()
    .map(|gesture: &EuvGesture| gesture.name())
    .collect();
    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            assert_ne!(names[i], names[j], "two gestures share a wire name");
        }
    }
}

#[test]
fn finishing_a_progress_that_never_began_yields_nothing() {
    let mut progress: GestureProgress = GestureProgress::default();
    assert_eq!(
        progress.finish(&EuvGestureConfig::default()),
        None,
        "there is no sequence in flight, so there is no gesture to report"
    );
}

#[test]
fn a_progress_started_and_reset_reports_nothing() {
    let mut progress: GestureProgress = GestureProgress::default();
    progress.begin(&[NativeTouchPoint {
        identifier: 1,
        client_x: 10,
        client_y: 10,
        screen_x: 10,
        screen_y: 10,
        offset_x: 10,
        offset_y: 10,
        page_x: 10,
        page_y: 10,
    }]);
    progress.reset();
    assert_eq!(
        progress.finish(&EuvGestureConfig::default()),
        None,
        "reset must clear the active flag, not just the cached coordinates"
    );
}

#[test]
fn beginning_with_no_touch_points_leaves_the_progress_inactive() {
    let mut progress: GestureProgress = GestureProgress::default();
    progress.begin(&[]);
    assert_eq!(
        progress.finish(&EuvGestureConfig::default()),
        None,
        "an empty touch list must not start a sequence"
    );
}

#[test]
fn a_recogniser_starts_idle() {
    let r: EuvGestureRecognizer = EuvGestureRecognizer::default();
    assert!(
        r.pinch_is_significant(1000.0, 1.0),
        "a default recogniser still carries the default pinch threshold"
    );
}

#[test]
fn a_touch_point_carries_the_same_identifier_across_a_sequence() {
    let point: NativeTouchPoint = NativeTouchPoint::default();
    assert_eq!(
        point.identifier, 0,
        "a default point belongs to no finger yet"
    );
    let named: NativeTouchPoint = NativeTouchPoint {
        identifier: 7,
        client_x: 3,
        client_y: 4,
        screen_x: 3,
        screen_y: 4,
        offset_x: 3,
        offset_y: 4,
        page_x: 3,
        page_y: 4,
    };
    assert_eq!(named.identifier, 7);
    assert_ne!(named, point);
}

#[test]
fn the_f64_touch_point_and_the_integer_one_agree_on_their_fields() {
    let whole: NativeTouchPoint = NativeTouchPoint {
        identifier: 2,
        client_x: 11,
        client_y: 22,
        screen_x: 33,
        screen_y: 44,
        offset_x: 55,
        offset_y: 66,
        page_x: 77,
        page_y: 88,
    };
    let precise: NativeTouchPointF64 = NativeTouchPointF64 {
        identifier: whole.identifier,
        client_x: f64::from(whole.client_x),
        client_y: f64::from(whole.client_y),
        screen_x: f64::from(whole.screen_x),
        screen_y: f64::from(whole.screen_y),
        offset_x: f64::from(whole.offset_x),
        offset_y: f64::from(whole.offset_y),
        page_x: f64::from(whole.page_x),
        page_y: f64::from(whole.page_y),
    };
    assert_eq!(precise.identifier, 2);
    assert_eq!(precise.page_y, 88.0);
}
