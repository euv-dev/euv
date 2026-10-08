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

const EPSILON: f64 = 1e-9;
#[test]
fn a_fresh_timer_has_not_fired_yet() {
    let timer: Timer = Timer::create(1.0);
    assert!(!timer.is_finished(), "a fresh one-shot timer is unfinished");
}

#[test]
fn a_fresh_timer_is_not_paused() {
    let timer: Timer = Timer::create(1.0);
    assert!(!timer.is_paused(), "a fresh timer runs");
}

#[test]
fn a_partial_update_does_not_fire_the_timer() {
    let mut timer: Timer = Timer::create(1.0);
    let fired: u32 = timer.update(0.5);
    assert_eq!(fired, 0, "half a second is not a whole second");
    assert!(!timer.is_finished(), "the timer keeps counting");
}

#[test]
fn a_full_update_fires_the_one_shot_timer() {
    let mut timer: Timer = Timer::create(1.0);
    let fired: u32 = timer.update(1.0);
    assert_eq!(fired, 1, "reaching the duration fires once");
    assert!(timer.is_finished(), "a one-shot timer stops after firing");
}

#[test]
fn a_one_shot_timer_never_fires_twice() {
    let mut timer: Timer = Timer::create(1.0);
    let first: u32 = timer.update(1.0);
    let second: u32 = timer.update(1.0);
    assert_eq!(first, 1, "the first update fires");
    assert_eq!(second, 0, "a finished one-shot timer is inert");
}

#[test]
fn an_oversized_update_fires_a_one_shot_timer_only_once() {
    let mut timer: Timer = Timer::create(1.0);
    let fired: u32 = timer.update(10.0);
    assert_eq!(fired, 1, "a one-shot timer stops at its first firing");
}

#[test]
fn reset_makes_a_finished_timer_fire_again() {
    let mut timer: Timer = Timer::create(1.0);
    let before: u32 = timer.update(1.0);
    timer.reset();
    let after: u32 = timer.update(1.0);
    assert_eq!(before, 1, "the timer fired once before the reset");
    assert_eq!(after, 1, "reset puts the timer back in play");
}

#[test]
fn reset_clears_the_accumulated_elapsed_time() {
    let mut timer: Timer = Timer::create(1.0);
    let _: u32 = timer.update(0.75);
    timer.reset();
    assert_eq!(timer.progress(), 0.0, "a reset timer reports zero progress");
}

#[test]
fn a_paused_timer_ignores_updates() {
    let mut timer: Timer = Timer::create(1.0);
    timer.pause();
    let fired: u32 = timer.update(5.0);
    assert_eq!(fired, 0, "a paused timer does not advance");
    assert!(timer.is_paused(), "the timer stays paused");
}

#[test]
fn a_paused_timer_preserves_its_accumulated_time() {
    let mut timer: Timer = Timer::create(1.0);
    let _: u32 = timer.update(0.5);
    timer.pause();
    let _: u32 = timer.update(5.0);
    assert_eq!(
        timer.progress(),
        0.5,
        "time accumulated before the pause survives it"
    );
}

#[test]
fn resume_lets_a_paused_timer_fire_again() {
    let mut timer: Timer = Timer::create(1.0);
    let _: u32 = timer.update(0.5);
    timer.pause();
    let _: u32 = timer.update(0.25);
    timer.resume();
    let fired: u32 = timer.update(0.5);
    assert!(!timer.is_paused(), "resume clears the paused flag");
    assert_eq!(
        fired, 1,
        "the half second banked before the pause plus the new half completes the run"
    );
}

#[test]
fn time_spent_paused_is_discarded_rather_than_banked() {
    let mut timer: Timer = Timer::create(1.0);
    timer.pause();
    let _: u32 = timer.update(5.0);
    timer.resume();
    assert_eq!(
        timer.progress(),
        0.0,
        "a paused update must not bank any elapsed time at all"
    );
}

#[test]
fn a_repeating_timer_fires_more_than_once() {
    let mut timer: Timer = Timer::create_repeating(0.5);
    let first: u32 = timer.update(0.5);
    let second: u32 = timer.update(0.5);
    assert_eq!(first, 1, "the first interval fires once");
    assert_eq!(second, 1, "the second interval fires again");
}

#[test]
fn a_repeating_timer_never_reports_finished() {
    let mut timer: Timer = Timer::create_repeating(0.5);
    let _: u32 = timer.update(10.0);
    assert!(!timer.is_finished(), "a repeating timer has no end");
}

#[test]
fn a_repeating_timer_counts_every_interval_in_one_big_update() {
    let mut timer: Timer = Timer::create_repeating(0.5);
    let fired: u32 = timer.update(2.0);
    assert_eq!(fired, 4, "two seconds is four half-second intervals");
}

#[test]
fn a_repeating_timer_carries_the_remainder_into_the_next_update() {
    let mut timer: Timer = Timer::create_repeating(1.0);
    let first: u32 = timer.update(1.25);
    let second: u32 = timer.update(0.75);
    assert_eq!(first, 1, "the first update fires once");
    assert_eq!(
        second, 1,
        "the leftover quarter second plus three quarters completes the next one"
    );
}

#[test]
fn a_zero_duration_timer_never_fires() {
    let mut timer: Timer = Timer::create(0.0);
    let fired: u32 = timer.update(10.0);
    assert_eq!(
        fired, 0,
        "a zero-length timer is inert rather than infinite"
    );
}

#[test]
fn a_negative_duration_is_clamped_to_zero() {
    let mut timer: Timer = Timer::create(-5.0);
    let fired: u32 = timer.update(10.0);
    assert_eq!(fired, 0, "a negative duration cannot fire");
}

#[test]
fn a_negative_update_does_not_rewind_the_timer() {
    let mut timer: Timer = Timer::create(1.0);
    let _: u32 = timer.update(0.5);
    let fired: u32 = timer.update(-1.0);
    assert_eq!(fired, 0, "a negative delta never fires");
    assert_eq!(timer.progress(), 0.5, "the accumulated time is preserved");
}

#[test]
fn progress_starts_at_zero_and_ends_at_one() {
    let mut timer: Timer = Timer::create(1.0);
    let start: f64 = timer.progress();
    let _: u32 = timer.update(1.0);
    let end: f64 = timer.progress();
    assert_eq!(start, 0.0, "a fresh timer has no progress");
    assert_eq!(end, 1.0, "a fired timer is fully progressed");
}

#[test]
fn progress_tracks_the_fraction_of_the_duration_elapsed() {
    let mut timer: Timer = Timer::create(2.0);
    let _: u32 = timer.update(0.5);
    assert_eq!(
        timer.progress(),
        0.25,
        "half a second of a two-second timer is a quarter"
    );
}

#[test]
fn progress_never_exceeds_one() {
    let mut timer: Timer = Timer::create(1.0);
    let _: u32 = timer.update(5.0);
    assert_eq!(timer.progress(), 1.0, "progress is clamped at the top");
}

#[test]
fn progress_of_a_zero_duration_timer_reads_as_complete() {
    let timer: Timer = Timer::create(0.0);
    assert_eq!(
        timer.progress(),
        1.0,
        "an empty countdown is trivially done"
    );
}

#[test]
fn remaining_counts_down_towards_zero() {
    let mut timer: Timer = Timer::create(1.0);
    let start: f64 = timer.remaining();
    let _: u32 = timer.update(0.25);
    let later: f64 = timer.remaining();
    assert_eq!(start, 1.0, "a fresh timer has its full duration left");
    assert!(
        (later - 0.75).abs() < EPSILON,
        "a quarter second leaves three quarters, got {later}"
    );
}

#[test]
fn remaining_never_goes_negative() {
    let mut timer: Timer = Timer::create(1.0);
    let _: u32 = timer.update(5.0);
    assert_eq!(timer.remaining(), 0.0, "an overshot timer has nothing left");
}
