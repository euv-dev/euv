use super::*;

#[test]
fn engine_cell_exposes_the_value_it_was_built_with() {
    let cell: EngineCell<u32> = EngineCell::new(7);
    assert_eq!(*cell.get(), 7, "a fresh cell must hold its initial value");
}

#[test]
fn engine_cell_mutable_borrow_writes_through() {
    let cell: EngineCell<u32> = EngineCell::new(1);
    *cell.get_mut() = 42;
    assert_eq!(
        *cell.get(),
        42,
        "a write through the mutable borrow must be visible to later reads"
    );
}

#[test]
fn engine_cell_default_uses_the_inner_default() {
    let cell: EngineCell<u32> = EngineCell::default();
    assert_eq!(
        *cell.get(),
        0,
        "a default cell must hold the inner type default"
    );
}

#[test]
fn engine_cell_inner_exposes_the_backing_storage() {
    let cell: EngineCell<u32> = EngineCell::new(3);
    let inner: &std::cell::UnsafeCell<u32> = cell.get_inner();
    assert_eq!(
        unsafe { *inner.get() },
        3,
        "the inner accessor must expose the same storage the cell wraps"
    );
}

#[test]
fn engine_cell_is_usable_for_non_copy_payloads() {
    let cell: EngineCell<String> = EngineCell::new(String::from("hello"));
    cell.get_mut().push_str(" world");
    assert_eq!(
        cell.get().as_str(),
        "hello world",
        "a heap payload must round-trip through the cell"
    );
}

#[test]
fn a_new_maybe_cell_reports_itself_empty() {
    let cell: MaybeEngineCell<u32> = MaybeEngineCell::new();
    assert!(cell.try_get().is_none(), "a fresh maybe cell must be empty");
    assert!(
        cell.try_get_mut().is_none(),
        "a fresh maybe cell has no mutable view"
    );
    assert!(
        cell.try_take().is_none(),
        "taking from an empty cell yields nothing"
    );
}

#[test]
fn maybe_cell_try_set_fills_an_empty_cell() {
    let cell: MaybeEngineCell<u32> = MaybeEngineCell::new();
    let outcome: Result<(), u32> = cell.try_set(5);
    assert!(outcome.is_ok(), "setting an empty cell must succeed");
    assert_eq!(
        *cell.try_get().unwrap(),
        5,
        "the value must be readable after a successful set"
    );
}

#[test]
fn maybe_cell_try_set_refuses_to_overwrite_and_returns_the_value() {
    let cell: MaybeEngineCell<u32> = MaybeEngineCell::new();
    let _: Result<(), u32> = cell.try_set(5);
    let outcome: Result<(), u32> = cell.try_set(9);
    assert_eq!(
        outcome,
        Err(9),
        "a second set must hand the value back to the caller"
    );
    assert_eq!(
        *cell.try_get().unwrap(),
        5,
        "a refused set must leave the original value intact"
    );
}

#[test]
fn maybe_cell_try_take_empties_the_cell() {
    let cell: MaybeEngineCell<String> = MaybeEngineCell::new();
    let _: Result<(), String> = cell.try_set(String::from("payload"));
    let taken: Option<String> = cell.try_take();
    assert_eq!(
        taken,
        Some(String::from("payload")),
        "take must return the stored value"
    );
    assert!(cell.try_get().is_none(), "take must leave the cell empty");
    assert!(
        cell.try_take().is_none(),
        "a second take must yield nothing"
    );
}

#[test]
fn maybe_cell_try_replace_swaps_the_stored_value() {
    let cell: MaybeEngineCell<u32> = MaybeEngineCell::new();
    let empty_previous: Option<u32> = cell.try_replace(3);
    assert_eq!(
        empty_previous, None,
        "replacing an empty cell must report no previous value"
    );
    let previous: Option<u32> = cell.try_replace(4);
    assert_eq!(
        previous,
        Some(3),
        "replace must return the value it displaced"
    );
    assert_eq!(
        *cell.try_get().unwrap(),
        4,
        "the new value must be readable"
    );
}

#[test]
fn maybe_cell_try_get_mut_writes_through() {
    let cell: MaybeEngineCell<u32> = MaybeEngineCell::new();
    let _: Result<(), u32> = cell.try_set(1);
    *cell.try_get_mut().unwrap() += 10;
    assert_eq!(
        *cell.try_get().unwrap(),
        11,
        "a write through the mutable view must stick"
    );
}

#[test]
fn maybe_cell_default_is_empty() {
    let cell: MaybeEngineCell<u32> = MaybeEngineCell::default();
    assert!(
        cell.try_get().is_none(),
        "a default maybe cell must be empty"
    );
}

#[test]
fn maybe_cell_set_inner_swaps_the_whole_backing_storage() {
    let mut cell: MaybeEngineCell<u32> = MaybeEngineCell::new();
    let _: Result<(), u32> = cell.try_set(8);
    let previous: std::cell::UnsafeCell<Option<u32>> =
        cell.set_inner(std::cell::UnsafeCell::new(Some(2)));
    assert_eq!(
        unsafe { *previous.get() },
        Some(8),
        "set_inner must hand back the storage it replaced"
    );
    assert_eq!(
        *cell.try_get().unwrap(),
        2,
        "the cell must now read through the replacement storage"
    );
}

#[test]
fn maybe_cell_inner_exposes_the_backing_option_storage() {
    let cell: MaybeEngineCell<u32> = MaybeEngineCell::new();
    let inner: &std::cell::UnsafeCell<Option<u32>> = cell.get_inner();
    assert!(
        unsafe { (*inner.get()).is_none() },
        "the inner accessor must expose the same option storage the cell wraps"
    );
}
