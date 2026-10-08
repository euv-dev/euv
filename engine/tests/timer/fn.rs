use super::*;

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
