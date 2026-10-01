use super::*;

#[test]
fn one_shot_timer_fires_once_at_its_duration() {
    let mut timer: Timer = Timer::create(1.0);
    assert_eq!(
        timer.update(0.5),
        0,
        "a half second must not fire a one second timer"
    );
    assert!(!timer.is_finished(), "the timer is still counting down");
    assert_eq!(
        timer.update(0.5),
        1,
        "reaching the duration must fire exactly once"
    );
    assert!(
        timer.is_finished(),
        "a one shot timer must be finished after firing"
    );
}

#[test]
fn one_shot_timer_does_not_fire_again_after_the_first_time() {
    let mut timer: Timer = Timer::create(0.5);
    let first: u32 = timer.update(1.0);
    let second: u32 = timer.update(1.0);
    let third: u32 = timer.update(1.0);
    assert_eq!(first, 1, "the first update must fire the timer");
    assert_eq!(second, 0, "a finished one shot timer must not fire again");
    assert_eq!(third, 0, "a finished one shot timer must stay finished");
}

#[test]
fn one_shot_timer_reports_every_firing_when_a_single_update_spans_several_periods() {
    let mut timer: Timer = Timer::create(0.5);
    let fired: u32 = timer.update(2.0);
    assert_eq!(fired, 1, "a one shot timer reports at most one firing");
}

#[test]
fn repeating_timer_fires_once_per_elapsed_period() {
    let mut timer: Timer = Timer::create_repeating(0.5);
    let first: u32 = timer.update(1.0);
    assert_eq!(first, 2, "two half second periods must produce two firings");
    assert!(
        !timer.is_finished(),
        "a repeating timer must never report itself finished"
    );
    let second: u32 = timer.update(1.0);
    assert_eq!(second, 2, "the cadence must repeat on the next update");
}

#[test]
fn repeating_timer_does_not_fire_before_its_period_elapses() {
    let mut timer: Timer = Timer::create_repeating(2.0);
    assert_eq!(timer.update(0.5), 0, "half a period must not fire");
    assert_eq!(
        timer.update(0.5),
        0,
        "a full period minus epsilon must not fire"
    );
    assert_eq!(timer.update(1.0), 1, "crossing the period must fire once");
}

#[test]
fn paused_timer_ignores_updates_until_it_resumes() {
    let mut timer: Timer = Timer::create(1.0);
    timer.pause();
    assert!(timer.is_paused(), "pause must be observable");
    assert_eq!(
        timer.update(5.0),
        0,
        "a paused timer must not accumulate time"
    );
    timer.resume();
    assert!(!timer.is_paused(), "resume must clear the paused flag");
    assert_eq!(
        timer.update(1.0),
        1,
        "the elapsed time must have been preserved across the pause"
    );
}

#[test]
fn reset_returns_a_finished_timer_to_its_counting_state() {
    let mut timer: Timer = Timer::create(1.0);
    let _: u32 = timer.update(2.0);
    assert!(timer.is_finished(), "the timer must have fired first");
    timer.reset();
    assert!(!timer.is_finished(), "reset must clear the finished flag");
    assert_eq!(
        timer.update(0.5),
        0,
        "reset must rewind the elapsed time to zero"
    );
    assert_eq!(
        timer.update(0.5),
        1,
        "the timer must fire again after a reset"
    );
}

#[test]
fn progress_reports_the_fraction_of_the_duration_elapsed() {
    let mut timer: Timer = Timer::create(2.0);
    assert_eq!(timer.progress(), 0.0, "a fresh timer is at zero progress");
    let _: u32 = timer.update(0.5);
    assert!(
        (timer.progress() - 0.25).abs() < 1e-9,
        "a quarter of a two second timer is a quarter done, got {}",
        timer.progress()
    );
    let _: u32 = timer.update(5.0);
    assert!(
        (timer.progress() - 1.0).abs() < 1e-9,
        "progress must saturate at one once the timer has fired"
    );
}

#[test]
fn remaining_reports_the_time_left_without_going_negative() {
    let mut timer: Timer = Timer::create(2.0);
    assert_eq!(
        timer.remaining(),
        2.0,
        "a fresh timer has its full duration left"
    );
    let _: u32 = timer.update(0.5);
    assert!(
        (timer.remaining() - 1.5).abs() < 1e-9,
        "half a second must leave one and a half seconds, got {}",
        timer.remaining()
    );
    let _: u32 = timer.update(10.0);
    assert_eq!(
        timer.remaining(),
        0.0,
        "remaining must clamp at zero rather than going negative"
    );
}

#[test]
fn a_non_positive_duration_timer_never_fires() {
    let mut timer: Timer = Timer::create(0.0);
    let fired: u32 = timer.update(5.0);
    assert_eq!(fired, 0, "a zero duration timer must not fire");
    assert!(
        !timer.is_finished(),
        "a zero duration timer must stay unfinished"
    );
}

#[test]
fn a_negative_duration_is_clamped_to_zero_at_construction() {
    let timer: Timer = Timer::create(-5.0);
    assert!(
        timer.get_duration() >= 0.0,
        "a negative duration must be clamped, got {}",
        timer.get_duration()
    );
}

#[test]
fn a_negative_delta_time_is_treated_as_no_elapsed_time() {
    let mut timer: Timer = Timer::create(1.0);
    let fired: u32 = timer.update(-10.0);
    assert_eq!(fired, 0, "a negative delta must not fire the timer");
    assert_eq!(
        timer.progress(),
        0.0,
        "a negative delta must not move the elapsed time backwards or forwards"
    );
}
