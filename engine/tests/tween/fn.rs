use super::*;

#[test]
fn a_fresh_tween_reports_the_start_value() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    assert_eq!(
        tween.value(),
        0.0,
        "a fresh tween must sit at its start value"
    );
    let first: f64 = tween.update(0.0);
    assert_eq!(first, 0.0, "a zero delta must not move the tween");
}

#[test]
fn linear_tween_reaches_the_end_value_at_its_duration() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_easing(Easing::Linear);
    let midpoint: f64 = tween.update(0.5);
    assert!(
        (midpoint - 5.0).abs() < 1e-9,
        "halfway through a linear tween must be halfway in value, got {midpoint}"
    );
    let end: f64 = tween.update(0.5);
    assert!(
        (end - 10.0).abs() < 1e-9,
        "a completed tween must land exactly on its end value, got {end}"
    );
    assert!(
        tween.is_finished(),
        "an once-mode tween must report finished"
    );
}

#[test]
fn raw_progress_tracks_elapsed_time_while_eased_progress_applies_the_curve() {
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0).with_easing(Easing::InQuad);
    let _: f64 = tween.update(0.5);
    assert!(
        (tween.raw_progress() - 0.5).abs() < 1e-9,
        "raw progress must equal the elapsed fraction, got {}",
        tween.raw_progress()
    );
    assert!(
        (tween.eased_progress() - 0.25).abs() < 1e-9,
        "eased progress must apply InQuad to the elapsed fraction, got {}",
        tween.eased_progress()
    );
}

#[test]
fn a_delayed_tween_holds_its_start_value_until_the_delay_elapses() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0)
        .with_easing(Easing::Linear)
        .with_delay(0.5);
    let during: f64 = tween.update(0.25);
    assert!(
        (during - 0.0).abs() < 1e-9,
        "a delayed tween must not interpolate during its delay, got {during}"
    );
    assert_eq!(
        tween.get_state(),
        TweenState::Delayed,
        "the tween must report the delayed state during its delay"
    );
    let after: f64 = tween.update(0.5);
    assert!(
        (after - 2.5).abs() < 1e-9,
        "a quarter second into the interpolation must yield a quarter of the range, got {after}"
    );
}

#[test]
fn pausing_a_tween_freezes_its_value_and_resuming_continues() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_easing(Easing::Linear);
    let before: f64 = tween.update(0.5);
    tween.pause();
    assert_eq!(
        tween.get_state(),
        TweenState::Paused,
        "pause must move the tween into the paused state"
    );
    let during: f64 = tween.update(0.5);
    assert!(
        (during - before).abs() < 1e-9,
        "a paused tween must not advance, got {during} after {before}"
    );
    tween.resume();
    let after: f64 = tween.update(0.5);
    assert!(
        (after - 10.0).abs() < 1e-9,
        "a resumed tween must continue from where it paused, got {after}"
    );
}

#[test]
fn reset_returns_a_finished_tween_to_its_start() {
    let mut tween: Tween<f64> = Tween::create(5.0, 15.0, 1.0).with_easing(Easing::Linear);
    let _: f64 = tween.update(2.0);
    assert!(tween.is_finished(), "the tween must have completed first");
    tween.reset();
    assert!(!tween.is_finished(), "reset must clear the finished flag");
    assert_eq!(
        tween.value(),
        5.0,
        "reset must return the value to the start endpoint"
    );
    assert_eq!(
        tween.raw_progress(),
        0.0,
        "reset must rewind the elapsed fraction"
    );
}

#[test]
fn a_finished_tween_holds_its_end_value_across_further_updates() {
    let mut tween: Tween<f64> = Tween::create(0.0, 4.0, 1.0).with_easing(Easing::Linear);
    let _: f64 = tween.update(5.0);
    let held: f64 = tween.update(5.0);
    assert!(
        (held - 4.0).abs() < 1e-9,
        "a finished tween must stay clamped at its end value, got {held}"
    );
}

#[test]
fn a_looping_tween_wraps_back_to_the_start_instead_of_finishing() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0)
        .with_easing(Easing::Linear)
        .with_mode(AnimationMode::Loop);
    let wrapped: f64 = tween.update(1.25);
    assert!(
        (0.0..=10.0).contains(&wrapped),
        "a looping tween must stay inside its value range, got {wrapped}"
    );
    assert!(
        !tween.is_finished(),
        "a looping tween must never report itself finished"
    );
    assert!(
        wrapped < 10.0,
        "a looping tween past its duration must have wrapped below its end value, got {wrapped}"
    );
}

#[test]
fn a_ping_pong_tween_reverses_direction_at_the_endpoints() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0)
        .with_easing(Easing::Linear)
        .with_mode(AnimationMode::PingPong);
    let at_end: f64 = tween.update(1.0);
    assert!(
        (at_end - 10.0).abs() < 1e-6,
        "a ping-pong tween must reach its end value first, got {at_end}"
    );
    let returning: f64 = tween.update(0.5);
    assert!(
        returning < at_end,
        "a ping-pong tween must come back down after reaching the end, got {returning}"
    );
    assert!(
        returning >= 0.0,
        "a ping-pong tween must not go below its start value, got {returning}"
    );
}

#[test]
fn the_completion_callback_fires_once_when_the_tween_finishes() {
    let fired: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let sink: Rc<RefCell<u32>> = fired.clone();
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0)
        .with_easing(Easing::Linear)
        .with_on_complete(Rc::new(move || {
            let mut count: std::cell::RefMut<'_, u32> = sink.borrow_mut();
            *count += 1;
        }));
    let _: f64 = tween.update(2.0);
    assert_eq!(*fired.borrow(), 1, "completion must fire exactly once");
    let _: f64 = tween.update(2.0);
    assert_eq!(
        *fired.borrow(),
        1,
        "a finished tween must not fire the completion callback again"
    );
}

#[test]
fn a_tween_without_a_callback_still_completes_cleanly() {
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 0.5).with_easing(Easing::Linear);
    let value: f64 = tween.update(1.0);
    assert!(
        (value - 1.0).abs() < 1e-9,
        "a callback-free tween must complete normally, got {value}"
    );
    assert!(tween.is_finished(), "the tween must report finished");
}

#[test]
fn the_tween_exposes_the_endpoints_it_was_created_with() {
    let tween: Tween<f64> = Tween::create(-2.0, 6.0, 3.0);
    assert_eq!(tween.get_from(), -2.0, "the start endpoint must round-trip");
    assert_eq!(tween.get_to(), 6.0, "the end endpoint must round-trip");
    assert_eq!(tween.get_duration(), 3.0, "the duration must round-trip");
}

#[test]
fn a_negative_delta_time_does_not_rewind_a_running_tween() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_easing(Easing::Linear);
    let before: f64 = tween.update(0.5);
    let after: f64 = tween.update(-0.25);
    assert!(
        after >= before,
        "a negative delta must not move the tween backwards, went from {before} to {after}"
    );
}
