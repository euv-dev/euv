use super::*;

#[test]
fn a_fresh_previous_has_nothing_recorded() {
    let previous: Previous<u32> = Previous::new();
    let observed: Option<u32> = previous.get_previous_snapshot();
    assert_eq!(observed, None, "a new Previous has no history");
}

#[test]
fn record_stores_the_value() {
    let previous: Previous<u32> = Previous::new();
    previous.record(7);
    let observed: Option<u32> = previous.get_previous_snapshot();
    assert_eq!(observed, Some(7), "the recorded value comes back");
}

#[test]
fn recording_twice_keeps_only_the_latest() {
    let previous: Previous<u32> = Previous::new();
    previous.record(1);
    previous.record(2);
    let observed: Option<u32> = previous.get_previous_snapshot();
    assert_eq!(
        observed,
        Some(2),
        "Previous is not a history, it is a single slot"
    );
}

#[test]
fn recording_the_same_value_twice_still_reads_back() {
    let previous: Previous<u32> = Previous::new();
    previous.record(5);
    previous.record(5);
    let observed: Option<u32> = previous.get_previous_snapshot();
    assert_eq!(
        observed,
        Some(5),
        "an unchanged value is still the latest value"
    );
}

#[test]
fn clear_empties_the_slot() {
    let previous: Previous<u32> = Previous::new();
    previous.record(3);
    previous.clear();
    let observed: Option<u32> = previous.get_previous_snapshot();
    assert_eq!(observed, None, "clear drops the recorded value");
}

#[test]
fn the_first_step_reports_no_previous_value() {
    let previous: Previous<u32> = Previous::new();
    let observed: Option<u32> = previous_step(previous, 10);
    assert_eq!(
        observed, None,
        "the very first call has no predecessor to report"
    );
}

#[test]
fn the_second_step_reports_the_first_value() {
    let previous: Previous<u32> = Previous::new();
    let _: Option<u32> = previous_step(previous, 10);
    let observed: Option<u32> = previous_step(previous, 20);
    assert_eq!(observed, Some(10), "the step reports the value before it");
}

#[test]
fn each_step_reports_exactly_one_step_back() {
    let previous: Previous<u32> = Previous::new();
    let _: Option<u32> = previous_step(previous, 1);
    let _: Option<u32> = previous_step(previous, 2);
    let observed: Option<u32> = previous_step(previous, 3);
    assert_eq!(observed, Some(2), "only one step of history, not a stack");
}

#[test]
fn a_step_also_records_the_value_it_was_given() {
    let previous: Previous<u32> = Previous::new();
    let _: Option<u32> = previous_step(previous, 42);
    let recorded: Option<u32> = previous.get_previous_snapshot();
    assert_eq!(
        recorded,
        Some(42),
        "stepping forwards feeds the next comparison"
    );
}

#[test]
fn stepping_with_an_unchanged_value_still_reports_the_same_number() {
    let previous: Previous<u32> = Previous::new();
    let _: Option<u32> = previous_step(previous, 8);
    let observed: Option<u32> = previous_step(previous, 8);
    assert_eq!(
        observed,
        Some(8),
        "a steady value reports itself as its own predecessor"
    );
}

#[test]
fn a_cleared_previous_restarts_the_sequence() {
    let previous: Previous<u32> = Previous::new();
    let _: Option<u32> = previous_step(previous, 1);
    previous.clear();
    let observed: Option<u32> = previous_step(previous, 2);
    assert_eq!(observed, None, "after a clear there is no history again");
}

#[test]
fn previous_works_for_string_values() {
    let previous: Previous<String> = Previous::new();
    let _: Option<String> = previous_step(previous, String::from("first"));
    let observed: Option<String> = previous_step(previous, String::from("second"));
    assert_eq!(
        observed,
        Some(String::from("first")),
        "the generic is not u32-only"
    );
}

#[test]
fn previous_works_for_non_copy_values() {
    let previous: Previous<Vec<u32>> = Previous::new();
    let _: Option<Vec<u32>> = previous_step(previous, vec![1, 2]);
    let observed: Option<Vec<u32>> = previous_step(previous, vec![3]);
    assert_eq!(
        observed,
        Some(vec![1, 2]),
        "a heap value round-trips by value"
    );
}

#[test]
fn a_copy_previous_can_be_passed_without_ownership_sharing_questions() {
    let previous: Previous<u32> = Previous::new();
    let alias: Previous<u32> = previous;
    let _: Option<u32> = previous_step(previous, 1);
    let observed: Option<u32> = previous_step(alias, 2);
    assert_eq!(
        observed,
        Some(1),
        "Previous is Copy because it is a handle over a shared signal, not a value"
    );
}
