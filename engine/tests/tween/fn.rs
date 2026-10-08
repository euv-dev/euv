use super::*;

fn epsilon(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}

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
            let mut count: RefMut<'_, u32> = sink.borrow_mut();
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

fn fresh() -> Tween<f64> {
    Tween::create(0.0, 10.0, 1.0)
}

#[test]
fn a_builder_chains_every_stage_and_keeps_the_underlying_values() {
    let tween: Tween<f64> = fresh()
        .with_easing(Easing::InOutQuad)
        .with_delay(0.25)
        .with_mode(AnimationMode::Loop);
    assert_eq!(tween.get_easing(), Easing::InOutQuad, "the easing stuck");
    assert!(epsilon(tween.get_delay(), 0.25), "the delay stuck");
    assert_eq!(tween.get_mode(), AnimationMode::Loop, "the mode stuck");
    assert!(
        epsilon(tween.get_from(), 0.0),
        "the start value is untouched"
    );
    assert!(epsilon(tween.get_to(), 10.0), "as is the end value");
    assert!(epsilon(tween.get_duration(), 1.0), "and so is the duration");
}

#[test]
fn the_setters_move_the_fields_the_getters_read() {
    let mut tween: Tween<f64> = fresh();
    tween.set_easing(Easing::OutBounce);
    tween.set_delay(0.5);
    tween.set_mode(AnimationMode::PingPong);
    tween.set_elapsed(0.75);
    tween.set_direction(-1.0);
    assert_eq!(tween.get_easing(), Easing::OutBounce, "easing round-trips");
    assert!(epsilon(tween.get_delay(), 0.5), "delay round-trips");
    assert_eq!(
        tween.get_mode(),
        AnimationMode::PingPong,
        "mode round-trips"
    );
    assert!(epsilon(tween.get_elapsed(), 0.75), "elapsed round-trips");
    assert!(
        epsilon(tween.get_direction(), -1.0),
        "direction round-trips"
    );
}

#[test]
fn a_completion_callback_round_trips_through_its_option() {
    let mut tween: Tween<f64> = fresh();
    assert!(
        tween.try_get_on_complete().is_none(),
        "a tween built without a builder callback has none"
    );
    tween.set_on_complete(Some(Rc::new(|| {})));
    assert!(
        tween.try_get_on_complete().is_some(),
        "and setting one is visible through the getter"
    );
    tween.set_on_complete(None);
    assert!(
        tween.try_get_on_complete().is_none(),
        "clearing it goes back to none"
    );
}

#[test]
fn elapsed_can_be_advanced_through_its_mutable_view() {
    let mut tween: Tween<f64> = fresh();
    *tween.get_elapsed_mut() = 0.5;
    assert!(
        epsilon(tween.get_elapsed(), 0.5),
        "the mutable view writes through to the getter, got {}",
        tween.get_elapsed()
    );
}

#[test]
fn set_state_is_the_same_knob_the_update_loop_turns() {
    let mut tween: Tween<f64> = fresh();
    assert_eq!(
        tween.get_state(),
        TweenState::Running,
        "a zero-delay tween starts Running, not Delayed"
    );
    tween.set_state(TweenState::Running);
    assert_eq!(tween.get_state(), TweenState::Running, "the state stuck");
    tween.set_state(TweenState::Finished);
    assert!(
        tween.is_finished(),
        "and the finished state drives is_finished"
    );
}

const EPSILON: f64 = 1e-9;
#[test]
fn a_fresh_tween_starts_running_with_a_linear_curve() {
    let tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    assert_eq!(
        tween.get_state(),
        TweenState::Running,
        "a tween starts running"
    );
    assert_eq!(
        tween.get_easing(),
        Easing::Linear,
        "the default curve is constant velocity"
    );
    assert_eq!(
        tween.get_mode(),
        AnimationMode::Once,
        "the default mode is Once"
    );
}

#[test]
fn a_rewound_elapsed_counter_shows_up_as_a_fresh_start() {
    let mut tween: Tween<f64> = fresh();
    tween.set_delay(0.0);
    let _: f64 = tween.update(0.5);
    assert!(
        epsilon(tween.raw_progress(), 0.5),
        "halfway through, got {}",
        tween.raw_progress()
    );
    *tween.get_elapsed_mut() = 0.0;
    assert!(
        epsilon(tween.raw_progress(), 0.0),
        "zeroing the elapsed counter rewinds the tween"
    );
    let value: f64 = tween.value();
    assert!(
        epsilon(value, 0.0),
        "and the value follows it back to the start"
    );
}

#[test]
fn a_fresh_tween_keeps_its_endpoints_and_duration() {
    let tween: Tween<f64> = Tween::create(-5.0, 15.0, 2.5);
    assert_eq!(tween.get_from(), -5.0, "the start value is stored");
    assert_eq!(tween.get_to(), 15.0, "the end value is stored");
    assert_eq!(tween.get_duration(), 2.5, "the duration is stored");
}

#[test]
fn a_negative_duration_is_clamped_to_zero() {
    let tween: Tween<f64> = Tween::create(0.0, 1.0, -4.0);
    assert_eq!(
        tween.get_duration(),
        0.0,
        "a negative duration is clamped away"
    );
}

#[test]
fn a_fresh_tween_reports_its_start_value() {
    let tween: Tween<f64> = Tween::create(3.0, 9.0, 1.0);
    assert_eq!(
        tween.value(),
        3.0,
        "before any update the value is the start"
    );
}

#[test]
fn a_zero_duration_tween_jumps_straight_to_the_end() {
    let mut tween: Tween<f64> = Tween::create(0.0, 42.0, 0.0);
    let observed: f64 = tween.update(0.1);
    assert_eq!(observed, 42.0, "a zero-length tween completes at once");
    assert!(tween.is_finished(), "a zero-length tween is finished");
}

#[test]
fn a_half_elapsed_linear_tween_sits_at_the_midpoint() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    let observed: f64 = tween.update(0.5);
    assert!(
        (observed - 5.0).abs() < EPSILON,
        "half a second of a one-second run is halfway, got {observed}"
    );
}

#[test]
fn a_fully_elapsed_linear_tween_lands_on_the_end_value() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    let observed: f64 = tween.update(1.0);
    assert!(
        tween.is_finished(),
        "a once-mode tween ends at its duration"
    );
    assert!(
        (observed - 10.0).abs() < EPSILON,
        "the end value is reached"
    );
}

#[test]
fn an_overshoot_update_lands_on_the_end_value_not_beyond() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    let observed: f64 = tween.update(5.0);
    assert!(
        (observed - 10.0).abs() < EPSILON,
        "progress is clamped at one, got {observed}"
    );
}

#[test]
fn a_negative_update_does_not_rewind_the_tween() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    let _: f64 = tween.update(0.5);
    let observed: f64 = tween.update(-1.0);
    assert!(
        (observed - 5.0).abs() < EPSILON,
        "a negative delta must not move the tween backwards, got {observed}"
    );
}

#[test]
fn raw_progress_tracks_the_elapsed_fraction() {
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 2.0);
    let _: f64 = tween.update(0.5);
    assert_eq!(
        tween.raw_progress(),
        0.25,
        "a quarter of the duration elapsed"
    );
}

#[test]
fn raw_progress_never_exceeds_one() {
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0);
    let _: f64 = tween.update(10.0);
    assert_eq!(tween.raw_progress(), 1.0, "raw progress is clamped at one");
}

#[test]
fn raw_progress_of_a_zero_duration_tween_reads_as_complete() {
    let tween: Tween<f64> = Tween::create(0.0, 1.0, 0.0);
    assert_eq!(tween.raw_progress(), 1.0, "an empty run is trivially done");
}

#[test]
fn a_non_linear_easing_makes_eased_progress_differ_from_raw() {
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0).with_easing(Easing::InQuad);
    let _: f64 = tween.update(0.5);
    let raw: f64 = tween.raw_progress();
    let eased: f64 = tween.eased_progress();
    assert_eq!(raw, 0.5, "the raw fraction is unchanged by easing");
    assert!(
        (eased - 0.25).abs() < EPSILON,
        "InQuad squares the raw fraction, got {eased}"
    );
}

#[test]
fn easing_is_invertible_onto_the_endpoints() {
    let mut tween: Tween<f64> = Tween::create(0.0, 100.0, 1.0).with_easing(Easing::OutCubic);
    let start: f64 = tween.eased_progress();
    let _: f64 = tween.update(1.0);
    let end: f64 = tween.eased_progress();
    assert!(
        start.abs() < EPSILON,
        "every curve starts at zero, got {start}"
    );
    assert!(
        (end - 1.0).abs() < EPSILON,
        "every curve ends at one, got {end}"
    );
}

#[test]
fn a_delayed_tween_starts_in_the_delayed_state() {
    let tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_delay(0.5);
    assert_eq!(
        tween.get_state(),
        TweenState::Delayed,
        "attaching a delay parks the tween in Delayed"
    );
}

#[test]
fn a_tween_without_a_delay_starts_running() {
    let tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_delay(0.0);
    assert_eq!(
        tween.get_state(),
        TweenState::Running,
        "a zero delay leaves the tween running"
    );
}

#[test]
fn a_negative_delay_is_clamped_to_zero() {
    let tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_delay(-3.0);
    assert_eq!(tween.get_delay(), 0.0, "a negative delay is clamped away");
}

#[test]
fn a_delayed_tween_holds_its_start_value_through_the_delay() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_delay(0.5);
    let observed: f64 = tween.update(0.25);
    assert_eq!(observed, 0.0, "the delay holds the value at the start");
    assert_eq!(
        tween.get_state(),
        TweenState::Delayed,
        "the tween is still waiting out its delay"
    );
}

#[test]
fn a_delayed_tween_starts_moving_once_the_delay_elapses() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_delay(0.5);
    let held: f64 = tween.update(0.25);
    let moving: f64 = tween.update(0.5);
    assert_eq!(held, 0.0, "the first half second is still delay");
    assert_eq!(
        tween.get_state(),
        TweenState::Running,
        "the delay has expired"
    );
    assert!(
        (moving - 2.5).abs() < EPSILON,
        "half a second into a one-second run is halfway, got {moving}"
    );
}

#[test]
fn a_delayed_tween_ends_at_the_end_value() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_delay(0.5);
    let observed: f64 = tween.update(1.5);
    assert!(
        tween.is_finished(),
        "delay plus duration completes the tween"
    );
    assert!(
        (observed - 10.0).abs() < EPSILON,
        "the delay must not eat into the run itself, got {observed}"
    );
}

#[test]
fn a_paused_tween_ignores_updates() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    tween.pause();
    let observed: f64 = tween.update(0.5);
    assert_eq!(
        tween.get_state(),
        TweenState::Paused,
        "the tween stays paused"
    );
    assert_eq!(observed, 0.0, "a paused tween does not advance");
}

#[test]
fn resume_lets_a_paused_tween_continue() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    tween.pause();
    let _: f64 = tween.update(0.5);
    tween.resume();
    let observed: f64 = tween.update(0.5);
    assert_eq!(
        tween.get_state(),
        TweenState::Running,
        "resume returns to Running"
    );
    assert!(
        (observed - 5.0).abs() < EPSILON,
        "the banked half second plus the new one is halfway, got {observed}"
    );
}

#[test]
fn resuming_a_paused_delayed_tween_goes_back_to_delayed() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_delay(1.0);
    tween.pause();
    tween.resume();
    assert_eq!(
        tween.get_state(),
        TweenState::Delayed,
        "resuming before the delay expires returns to Delayed"
    );
}

#[test]
fn pausing_a_finished_tween_leaves_it_finished() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    let _: f64 = tween.update(1.0);
    tween.pause();
    assert_eq!(
        tween.get_state(),
        TweenState::Finished,
        "a finished tween cannot be paused back into play"
    );
}

#[test]
fn a_finished_tween_ignores_further_updates() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    let _: f64 = tween.update(1.0);
    let observed: f64 = tween.update(5.0);
    assert!(
        (observed - 10.0).abs() < EPSILON,
        "a finished tween keeps reporting its end value, got {observed}"
    );
}

#[test]
fn reset_puts_a_tween_back_at_the_start() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    let _: f64 = tween.update(1.0);
    tween.reset();
    assert!(!tween.is_finished(), "reset clears the finished flag");
    assert_eq!(tween.get_elapsed(), 0.0, "reset zeroes the elapsed time");
    assert_eq!(
        tween.get_state(),
        TweenState::Running,
        "reset restarts running"
    );
    assert_eq!(tween.value(), 0.0, "the value returns to the start");
}

#[test]
fn a_reset_tween_can_be_replayed() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    let _: f64 = tween.update(1.0);
    tween.reset();
    let observed: f64 = tween.update(1.0);
    assert!(
        (observed - 10.0).abs() < EPSILON,
        "the replayed run reaches the end again, got {observed}"
    );
}

#[test]
fn reset_returns_a_delayed_tween_to_the_delayed_state() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_delay(0.5);
    let _: f64 = tween.update(1.0);
    tween.reset();
    assert_eq!(
        tween.get_state(),
        TweenState::Delayed,
        "a tween with a delay resets into Delayed"
    );
}

#[test]
fn a_looping_tween_never_finishes() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_mode(AnimationMode::Loop);
    let _: f64 = tween.update(5.0);
    assert!(!tween.is_finished(), "a looping tween has no end state");
}

#[test]
fn a_looping_tween_wraps_back_to_the_start() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_mode(AnimationMode::Loop);
    let _: f64 = tween.update(1.5);
    assert!(
        (tween.raw_progress() - 0.5).abs() < EPSILON,
        "half a second past the boundary is halfway round the loop, got {}",
        tween.raw_progress()
    );
}

#[test]
fn a_ping_pong_tween_flips_its_direction_at_the_boundary() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_mode(AnimationMode::PingPong);
    let forward: f64 = tween.get_direction();
    let _: f64 = tween.update(1.5);
    assert_eq!(
        tween.get_direction(),
        -forward,
        "a ping-pong tween reverses at the far end"
    );
}

#[test]
fn a_ping_pong_tween_reverses_the_value_it_reports() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_mode(AnimationMode::PingPong);
    let _: f64 = tween.update(1.5);
    let observed: f64 = tween.value();
    assert!(
        (observed - 5.0).abs() < EPSILON,
        "halfway back down is still halfway, got {observed}"
    );
}

#[test]
fn the_completion_callback_fires_once_for_a_once_mode_tween() {
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let handle: Rc<Cell<u32>> = Rc::clone(&calls);
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0).with_on_complete(Rc::new(move || {
        handle.set(handle.get() + 1);
    }));
    let _: f64 = tween.update(1.0);
    assert_eq!(calls.get(), 1, "completing the run fires the callback");
}

#[test]
fn the_completion_callback_does_not_fire_before_the_run_ends() {
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let handle: Rc<Cell<u32>> = Rc::clone(&calls);
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0).with_on_complete(Rc::new(move || {
        handle.set(handle.get() + 1);
    }));
    let _: f64 = tween.update(0.5);
    assert_eq!(
        calls.get(),
        0,
        "a half-finished run must not fire the callback"
    );
}

#[test]
fn a_tween_without_a_callback_still_completes() {
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0);
    let observed: f64 = tween.update(1.0);
    assert!(tween.is_finished(), "the callback is optional");
    assert!(
        (observed - 1.0).abs() < EPSILON,
        "the run still reaches its end"
    );
}

#[test]
fn set_easing_replaces_the_curve() {
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0);
    tween.set_easing(Easing::InQuad);
    assert_eq!(
        tween.get_easing(),
        Easing::InQuad,
        "the setter takes effect"
    );
}

#[test]
fn set_elapsed_seeks_the_tween_directly() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    tween.set_elapsed(0.5);
    assert_eq!(tween.get_elapsed(), 0.5, "the setter stores the value");
    assert!(
        (tween.value() - 5.0).abs() < EPSILON,
        "seeking half way reports the halfway value"
    );
}

#[test]
fn get_elapsed_mut_writes_through_to_the_tween() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    let slot: &mut f64 = tween.get_elapsed_mut();
    *slot = 0.25;
    assert_eq!(
        tween.get_elapsed(),
        0.25,
        "the mutable accessor is write-through"
    );
}

#[test]
fn set_state_overrides_the_playback_state() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    tween.set_state(TweenState::Paused);
    assert_eq!(
        tween.get_state(),
        TweenState::Paused,
        "the setter takes effect"
    );
    assert!(!tween.is_finished(), "paused is not finished");
}

#[test]
fn set_mode_replaces_the_completion_mode() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    tween.set_mode(AnimationMode::Loop);
    assert_eq!(
        tween.get_mode(),
        AnimationMode::Loop,
        "the setter takes effect"
    );
}

#[test]
fn set_direction_reverses_the_reported_value() {
    let mut tween: Tween<f64> = Tween::create(0.0, 10.0, 1.0);
    let forward: f64 = tween.value();
    tween.set_direction(-1.0);
    let backward: f64 = tween.value();
    assert_eq!(forward, 0.0, "a fresh tween sits at the start");
    assert!(
        (backward - 10.0).abs() < EPSILON,
        "reversing the direction mirrors the progress, got {backward}"
    );
}

#[test]
fn set_on_complete_installs_a_callback_after_construction() {
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let handle: Rc<Cell<u32>> = Rc::clone(&calls);
    let mut tween: Tween<f64> = Tween::create(0.0, 1.0, 1.0);
    tween.set_on_complete(Some(Rc::new(move || {
        handle.set(handle.get() + 1);
    })));
    let _: f64 = tween.update(1.0);
    assert_eq!(calls.get(), 1, "a late-installed callback still fires");
}

#[test]
fn try_get_on_complete_reports_whether_one_is_attached() {
    let without: Tween<f64> = Tween::create(0.0, 1.0, 1.0);
    let with: Tween<f64> = Tween::create(0.0, 1.0, 1.0).with_on_complete(Rc::new(|| {}));
    assert!(
        without.try_get_on_complete().is_none(),
        "a bare tween has no callback"
    );
    assert!(
        with.try_get_on_complete().is_some(),
        "a tween built with one reports it"
    );
}

#[test]
fn a_cloned_tween_replays_the_same_sequence() {
    let mut original: Tween<f64> = Tween::create(0.0, 10.0, 1.0).with_easing(Easing::InQuad);
    let _: f64 = original.update(0.5);
    let mut copy: Tween<f64> = original.clone();
    let left: f64 = original.update(0.25);
    let right: f64 = copy.update(0.25);
    assert_eq!(left, right, "a clone advances identically to its source");
}

#[test]
fn a_vector_tween_interpolates_componentwise() {
    let mut tween: Tween<Vector2D> =
        Tween::create(Vector2D::new(0.0, 0.0), Vector2D::new(10.0, 20.0), 1.0);
    let observed: Vector2D = tween.update(0.5);
    assert_eq!(
        observed,
        Vector2D::new(5.0, 10.0),
        "a vector tween lerps both components"
    );
}
