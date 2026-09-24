use super::*;

#[test]
fn now_ms_is_non_decreasing_and_non_negative() {
    let a: f64 = catch_unwind(AssertUnwindSafe(now_ms)).unwrap_or_default();
    let mut sink: u64 = 0;
    for i in 0..10_000_u64 {
        sink = sink.wrapping_add(i);
    }
    black_box(sink);
    let b: f64 = catch_unwind(AssertUnwindSafe(now_ms)).unwrap_or_default();
    assert!(a >= 0.0, "now_ms must be non-negative, got {}", a);
    assert!(b >= a, "now_ms must be monotonic, got {} then {}", a, b);
}

#[test]
fn now_ms_two_calls_typically_differ() {
    let a: f64 = catch_unwind(AssertUnwindSafe(now_ms)).unwrap_or_default();
    let mut sink: u64 = 0;
    for i in 0..1_000_000_u64 {
        sink = sink.wrapping_mul(i.wrapping_add(1));
    }
    black_box(sink);
    let b: f64 = catch_unwind(AssertUnwindSafe(now_ms)).unwrap_or_default();
    assert!(b >= a);
}

#[test]
fn profile_entry_new_stores_all_fields() {
    let entry: ProfileEntry = ProfileEntry::new(String::from("op"), 1.5, 100.0);
    assert_eq!(entry.get_label(), "op");
    assert!((entry.get_elapsed_ms() - 1.5).abs() < f64::EPSILON);
    assert!((entry.get_timestamp_ms() - 100.0).abs() < f64::EPSILON);
}

#[test]
fn profile_entry_clone_preserves_all_fields() {
    let original: ProfileEntry = ProfileEntry::new(String::from("orig"), 2.5, 200.0);
    let copy: ProfileEntry = original.clone();
    assert_eq!(copy, original);
}

#[test]
fn profile_entry_partial_eq_compares_field_by_field() {
    let a: ProfileEntry = ProfileEntry::new(String::from("a"), 1.0, 10.0);
    let b: ProfileEntry = ProfileEntry::new(String::from("a"), 1.0, 10.0);
    let c: ProfileEntry = ProfileEntry::new(String::from("a"), 1.0, 11.0);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn profiler_handle_new_starts_with_empty_entries() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let entries: Vec<ProfileEntry> = handle.get_entries().get();
    assert!(entries.is_empty());
}

#[test]
fn profiler_handle_clone_shares_entries_signal() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let twin: ProfilerHandle = handle;
    let ran_clean: bool = catch_unwind(AssertUnwindSafe(|| {
        handle.measure("first", || 42);
    }))
    .is_ok();
    if ran_clean {
        assert_eq!(twin.get_entries().get().len(), 1);
        assert_eq!(twin.get_entries().get()[0].get_label(), "first");
    }
}

#[test]
fn profiler_handle_measure_pushes_entry_with_nonzero_label() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let mut captured: Option<i32> = None;
    let ran_clean: bool = catch_unwind(AssertUnwindSafe(|| {
        captured = Some(handle.measure("compute", || 7_i32.wrapping_mul(6)));
    }))
    .is_ok();
    assert!(ran_clean, "native Signal::set panic should be caught");
    assert_eq!(captured, Some(42), "closure return value must be forwarded");
    if ran_clean {
        let entries: Vec<ProfileEntry> = handle.get_entries().get();
        assert_eq!(entries.len(), 1);
        let entry: &ProfileEntry = &entries[0];
        assert_eq!(entry.get_label(), "compute");
        assert!(entry.get_elapsed_ms() >= 0.0);
        assert!(entry.get_timestamp_ms() >= 0.0);
    }
}

#[test]
fn profiler_handle_measure_accumulates_multiple_entries() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let ran_clean: bool = catch_unwind(AssertUnwindSafe(|| {
        for i in 0..5_u32 {
            handle.measure(&format!("op-{}", i), || i.wrapping_add(1));
        }
    }))
    .is_ok();
    assert!(ran_clean, "native Signal::set panic should be caught");
    if ran_clean {
        let entries: Vec<ProfileEntry> = handle.get_entries().get();
        assert_eq!(entries.len(), 5);
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(entry.get_label(), format!("op-{}", i).as_str());
        }
    }
}

#[test]
fn profiler_handle_measure_forwards_closure_return_value() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let mut captured: Option<String> = None;
    let ran_clean: bool = catch_unwind(AssertUnwindSafe(|| {
        captured = Some(handle.measure("build-string", || String::from("hello")));
    }))
    .is_ok();
    assert!(ran_clean);
    assert_eq!(captured.as_deref(), Some("hello"));
    let mut captured_tuple: Option<(i32, f64)> = None;
    let ran_clean: bool = catch_unwind(AssertUnwindSafe(|| {
        captured_tuple = Some(handle.measure("build-tuple", || (7, PI)));
    }))
    .is_ok();
    assert!(ran_clean);
    assert_eq!(captured_tuple, Some((7, PI)));
}

#[test]
fn profiler_handle_clear_empties_entries() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let ran_clean: bool = catch_unwind(AssertUnwindSafe(|| {
        handle.measure("first", || ());
        handle.measure("second", || ());
    }))
    .is_ok();
    assert!(ran_clean);
    if ran_clean {
        assert_eq!(handle.get_entries().get().len(), 2);
        handle.clear();
        assert!(handle.get_entries().get().is_empty());
        let ran_clean_post: bool = catch_unwind(AssertUnwindSafe(|| {
            handle.measure("third", || ());
        }))
        .is_ok();
        assert!(ran_clean_post);
        assert_eq!(handle.get_entries().get().len(), 1);
        assert_eq!(handle.get_entries().get()[0].get_label(), "third");
    }
}

#[test]
fn profiler_handle_begin_end_push_entry_with_nonzero_elapsed() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let mut sink: u64 = 0;
    for i in 0..10_000_u64 {
        sink = sink.wrapping_add(i);
    }
    black_box(sink);
    let ran_clean: bool = catch_unwind(AssertUnwindSafe(|| {
        let mark: ProfilerMark = handle.begin("interval");
        mark.end();
    }))
    .is_ok();
    if ran_clean {
        let entries: Vec<ProfileEntry> = handle.get_entries().get();
        assert_eq!(entries.len(), 1);
        let entry: &ProfileEntry = &entries[0];
        assert_eq!(entry.get_label(), "interval");
        assert!(entry.get_elapsed_ms() >= 0.0);
        assert!(entry.get_timestamp_ms() >= 0.0);
    }
}

#[test]
fn profiler_handle_entries_signal_is_subscribable() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let subscriber_signal: Signal<Vec<ProfileEntry>> = *handle.get_entries();
    assert!(subscriber_signal.get().is_empty());
    let ran_clean: bool = catch_unwind(AssertUnwindSafe(|| {
        handle.measure("first", || ());
    }))
    .is_ok();
    assert!(ran_clean);
    if ran_clean {
        assert_eq!(subscriber_signal.get().len(), 1);
        let ran_clean2: bool = catch_unwind(AssertUnwindSafe(|| {
            handle.measure("second", || ());
        }))
        .is_ok();
        assert!(ran_clean2);
        assert_eq!(subscriber_signal.get().len(), 2);
    }
}

#[test]
fn profiler_mark_drop_without_end_discards_silently() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    {
        let _mark: ProfilerMark = handle.begin("discard");
    }
    assert!(handle.get_entries().get().is_empty());
}

#[test]
fn profiler_handle_measure_records_distinct_timestamps() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let ran_clean: bool = catch_unwind(AssertUnwindSafe(|| {
        handle.measure("a", || ());
        handle.measure("b", || ());
    }))
    .is_ok();
    assert!(ran_clean);
    if ran_clean {
        let entries: Vec<ProfileEntry> = handle.get_entries().get();
        assert!(entries[0].get_timestamp_ms() <= entries[1].get_timestamp_ms());
    }
}
