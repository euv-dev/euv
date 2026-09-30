use super::*;

fn point(x: i32, y: i32) -> NativeTouchPoint {
    NativeTouchPoint {
        identifier: 0,
        client_x: x,
        client_y: y,
        screen_x: x,
        screen_y: y,
        offset_x: x,
        offset_y: y,
        page_x: x,
        page_y: y,
    }
}

#[test]
fn classify_reports_tap_for_a_stationary_short_touch() {
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    assert_eq!(
        recognizer.classify(2.0, -3.0, 100.0, 4.0),
        Some(EuvGesture::Tap)
    );
}

#[test]
fn classify_reports_long_press_past_the_duration_threshold() {
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    assert_eq!(
        recognizer.classify(0.0, 0.0, 900.0, 0.0),
        Some(EuvGesture::LongPress)
    );
}

#[test]
fn classify_picks_the_dominant_axis_of_a_swipe() {
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    assert_eq!(
        recognizer.classify(120.0, 10.0, 80.0, 121.0),
        Some(EuvGesture::Right)
    );
    assert_eq!(
        recognizer.classify(-120.0, 10.0, 80.0, 121.0),
        Some(EuvGesture::Left)
    );
    assert_eq!(
        recognizer.classify(10.0, 120.0, 80.0, 121.0),
        Some(EuvGesture::Down)
    );
    assert_eq!(
        recognizer.classify(10.0, -120.0, 80.0, 121.0),
        Some(EuvGesture::Up)
    );
}

#[test]
fn classify_ignores_movement_below_the_swipe_threshold() {
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    assert_eq!(recognizer.classify(30.0, 0.0, 80.0, 31.0), None);
}

#[test]
fn classify_uses_the_configured_thresholds() {
    let config: EuvGestureConfig = EuvGestureConfig {
        swipe_threshold: 200.0,
        tap_slop: 40.0,
        long_press_millis: 100.0,
        pinch_threshold: 0.01,
    };
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::with_config(config);
    assert_eq!(recognizer.classify(120.0, 0.0, 80.0, 121.0), None);
    assert_eq!(
        recognizer.classify(30.0, 0.0, 200.0, 31.0),
        Some(EuvGesture::LongPress)
    );
    let default_recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    assert_eq!(default_recognizer.classify(30.0, 0.0, 200.0, 31.0), None);
    assert_eq!(
        recognizer.classify(0.0, 0.0, 200.0, 0.0),
        Some(EuvGesture::LongPress)
    );
}

#[test]
fn pinch_requires_exactly_two_points() {
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    assert_eq!(recognizer.pinch_from(&[point(0, 0)], 10.0), None);
    assert_eq!(
        recognizer.pinch_from(&[point(0, 0), point(3, 4), point(9, 9)], 10.0),
        None
    );
}

#[test]
fn pinch_reports_distance_center_and_delta() {
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    let pinch: EuvPinch = recognizer
        .pinch_from(&[point(0, 0), point(6, 8)], 0.0)
        .expect("two points make a pinch");
    assert!((pinch.get_distance() - 10.0).abs() < 1e-9);
    assert!((pinch.get_center_x() - 3.0).abs() < 1e-9);
    assert!((pinch.get_center_y() - 4.0).abs() < 1e-9);
    assert!((pinch.get_delta() - 10.0).abs() < 1e-9);
}

#[test]
fn pinch_significance_is_relative_to_the_starting_separation() {
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    assert!(recognizer.pinch_is_significant(11.0, 10.0));
    assert!(!recognizer.pinch_is_significant(10.05, 10.0));
    assert!(!recognizer.pinch_is_significant(10.0, 0.0));
}

#[test]
fn drag_accumulates_path_length_beyond_the_straight_line() {
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    let live: GesturePoint = GesturePoint {
        client_x: 30.0,
        client_y: 40.0,
    };
    let drag: EuvDrag = recognizer.drag_from(&live, 0.0, 0.0, 250.0);
    assert!((drag.get_x() - 30.0).abs() < 1e-9);
    assert!((drag.get_delta_x() - 30.0).abs() < 1e-9);
    assert!((drag.get_delta_y() - 40.0).abs() < 1e-9);
    assert!((drag.get_travel() - 250.0).abs() < 1e-9);
}

#[test]
fn gesture_progress_separates_a_looping_drag_from_a_tap() {
    let config: EuvGestureConfig = EuvGestureConfig::default();
    let mut progress: GestureProgress = GestureProgress::default();
    progress.begin(&[point(0, 0)]);
    progress.advance(&[point(40, 0)]);
    progress.advance(&[point(40, 40)]);
    progress.advance(&[point(0, 40)]);
    progress.advance(&[point(0, 0)]);
    assert!(*progress.get_travel() > 100.0);
    assert_eq!(progress.finish(&config), None);
}

#[test]
fn gesture_progress_reports_a_tap() {
    let config: EuvGestureConfig = EuvGestureConfig::default();
    let mut progress: GestureProgress = GestureProgress::default();
    progress.begin(&[point(100, 100)]);
    progress.advance(&[point(103, 102)]);
    assert_eq!(progress.finish(&config), Some(EuvGesture::Tap));
}

#[test]
fn gesture_progress_is_inert_before_a_sequence_starts() {
    let config: EuvGestureConfig = EuvGestureConfig::default();
    let mut progress: GestureProgress = GestureProgress::default();
    assert!(progress.primary().is_none());
    assert_eq!(progress.finish(&config), None);
}

#[test]
fn gesture_reset_clears_an_in_flight_sequence() {
    let config: EuvGestureConfig = EuvGestureConfig::default();
    let mut progress: GestureProgress = GestureProgress::default();
    progress.begin(&[point(10, 10)]);
    progress.advance(&[point(200, 10)]);
    progress.reset();
    assert!(progress.primary().is_none());
    assert_eq!(progress.finish(&config), None);
}

#[test]
fn gesture_names_are_stable() {
    assert_eq!(EuvGesture::Left.name(), "left");
    assert_eq!(EuvGesture::Right.name(), "right");
    assert_eq!(EuvGesture::Up.name(), "up");
    assert_eq!(EuvGesture::Down.name(), "down");
    assert_eq!(EuvGesture::Tap.name(), "tap");
    assert_eq!(EuvGesture::LongPress.name(), "long-press");
}

#[test]
fn classify_does_not_mistake_a_looped_drag_for_a_tap() {
    let recognizer: EuvGestureRecognizer = EuvGestureRecognizer::new();
    assert_eq!(recognizer.classify(0.0, 0.0, 60.0, 160.0), None);
    assert_eq!(
        recognizer.classify(0.0, 0.0, 60.0, 0.0),
        Some(EuvGesture::Tap)
    );
}
