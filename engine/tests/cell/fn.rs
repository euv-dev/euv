use super::*;

#[test]
fn engine_cell_new_exposes_the_seeded_value_through_get() {
    let cell: EngineCell<i32> = EngineCell::new(7);
    let observed: &i32 = cell.get();
    assert_eq!(*observed, 7, "new stores the value verbatim");
}

#[test]
fn engine_cell_default_starts_at_the_inner_type_default() {
    let cell: EngineCell<u8> = EngineCell::default();
    let observed: &u8 = cell.get();
    assert_eq!(*observed, 0, "Default delegates to the inner type default");
}

#[test]
fn engine_cell_get_mut_writes_through_to_the_same_storage() {
    let cell: EngineCell<i32> = EngineCell::new(1);
    let writer: &mut i32 = cell.get_mut();
    *writer = 42;
    let reader: &i32 = cell.get();
    assert_eq!(*reader, 42, "get_mut must be visible to a later get");
}

#[test]
fn engine_cell_get_inner_borrows_the_backing_unsafe_cell() {
    let cell: EngineCell<i32> = EngineCell::new(5);
    let inner: &UnsafeCell<i32> = cell.get_inner();
    let observed: &i32 = unsafe { &*inner.get() };
    assert_eq!(*observed, 5, "get_inner exposes the live backing storage");
}

#[test]
fn engine_cell_get_inner_stays_the_same_slot_across_a_write() {
    let cell: EngineCell<i32> = EngineCell::new(5);
    let before: &UnsafeCell<i32> = cell.get_inner();
    let writer: &mut i32 = cell.get_mut();
    *writer = 9;
    let after: &UnsafeCell<i32> = cell.get_inner();
    let observed: &i32 = unsafe { &*after.get() };
    assert_eq!(
        before as *const UnsafeCell<i32>, after as *const UnsafeCell<i32>,
        "get_inner must not swap the backing storage"
    );
    assert_eq!(*observed, 9, "the write landed in the original slot");
}

#[test]
fn maybe_engine_cell_new_is_const_constructible_and_starts_empty() {
    let cell: MaybeEngineCell<i32> = MaybeEngineCell::new();
    let observed: Option<&i32> = cell.try_get();
    assert_eq!(observed, None, "a fresh cell holds no value");
}

#[test]
fn maybe_engine_cell_default_starts_empty() {
    let cell: MaybeEngineCell<String> = MaybeEngineCell::default();
    let observed: Option<&String> = cell.try_get();
    assert_eq!(observed, None, "Default is the empty state, not a value");
}

#[test]
fn maybe_engine_cell_try_set_succeeds_on_an_empty_cell() {
    let cell: MaybeEngineCell<i32> = MaybeEngineCell::new();
    let outcome: Result<(), i32> = cell.try_set(11);
    let observed: Option<&i32> = cell.try_get();
    assert_eq!(outcome, Ok(()), "an empty cell accepts the value");
    assert_eq!(observed, Some(&11), "the value is readable afterwards");
}

#[test]
fn maybe_engine_cell_try_set_returns_the_rejected_value_when_occupied() {
    let cell: MaybeEngineCell<i32> = MaybeEngineCell::new();
    let first: Result<(), i32> = cell.try_set(1);
    let second: Result<(), i32> = cell.try_set(2);
    let observed: Option<&i32> = cell.try_get();
    assert_eq!(first, Ok(()), "the first install wins");
    assert_eq!(
        second,
        Err(2),
        "a rejected install hands the value back to the caller"
    );
    assert_eq!(observed, Some(&1), "a rejected install must not overwrite");
}

#[test]
fn maybe_engine_cell_try_get_mut_writes_through_to_the_stored_value() {
    let cell: MaybeEngineCell<i32> = MaybeEngineCell::new();
    let installed: Result<(), i32> = cell.try_set(1);
    let writer: Option<&mut i32> = cell.try_get_mut();
    match writer {
        Some(slot) => *slot = 33,
        None => panic!("try_get_mut must expose an installed value"),
    }
    let observed: Option<&i32> = cell.try_get();
    assert_eq!(installed, Ok(()), "setup install must succeed");
    assert_eq!(observed, Some(&33), "try_get_mut writes reach the cell");
}

#[test]
fn maybe_engine_cell_try_get_mut_is_none_on_an_empty_cell() {
    let cell: MaybeEngineCell<i32> = MaybeEngineCell::new();
    let observed: Option<&mut i32> = cell.try_get_mut();
    assert_eq!(observed, None, "an empty cell has nothing to hand out");
}

#[test]
fn maybe_engine_cell_try_take_empties_the_cell_and_reports_none_afterwards() {
    let cell: MaybeEngineCell<i32> = MaybeEngineCell::new();
    let installed: Result<(), i32> = cell.try_set(4);
    let taken: Option<i32> = cell.try_take();
    let again: Option<i32> = cell.try_take();
    let observed: Option<&i32> = cell.try_get();
    assert_eq!(installed, Ok(()), "setup install must succeed");
    assert_eq!(taken, Some(4), "try_take hands the value back");
    assert_eq!(again, None, "a second take finds an empty cell");
    assert_eq!(observed, None, "try_take leaves the cell empty");
}

#[test]
fn maybe_engine_cell_try_take_on_a_never_used_cell_is_none() {
    let cell: MaybeEngineCell<String> = MaybeEngineCell::new();
    let taken: Option<String> = cell.try_take();
    assert_eq!(taken, None, "taking from a fresh cell yields nothing");
}

#[test]
fn maybe_engine_cell_try_replace_returns_the_previous_value() {
    let cell: MaybeEngineCell<i32> = MaybeEngineCell::new();
    let installed: Result<(), i32> = cell.try_set(1);
    let previous: Option<i32> = cell.try_replace(2);
    let observed: Option<&i32> = cell.try_get();
    assert_eq!(installed, Ok(()), "setup install must succeed");
    assert_eq!(previous, Some(1), "try_replace reports the old value");
    assert_eq!(observed, Some(&2), "the new value is in place");
}

#[test]
fn maybe_engine_cell_try_replace_on_an_empty_cell_reports_none() {
    let cell: MaybeEngineCell<i32> = MaybeEngineCell::new();
    let previous: Option<i32> = cell.try_replace(3);
    let observed: Option<&i32> = cell.try_get();
    assert_eq!(previous, None, "nothing was stored before the replace");
    assert_eq!(observed, Some(&3), "the replaced value is now stored");
}

#[test]
fn maybe_engine_cell_set_inner_swaps_the_backing_storage() {
    let mut cell: MaybeEngineCell<i32> = MaybeEngineCell::new();
    let previous: UnsafeCell<Option<i32>> = cell.set_inner(UnsafeCell::new(Some(8)));
    let observed: Option<&i32> = cell.try_get();
    let from_previous: Option<&i32> = unsafe { (*previous.get()).as_ref() };
    assert_eq!(observed, Some(&8), "the new storage is what reads see");
    assert_eq!(
        from_previous, None,
        "set_inner hands the old storage back to the caller"
    );
}

#[test]
fn maybe_engine_cell_get_inner_reflects_try_set_and_try_take() {
    let cell: MaybeEngineCell<i32> = MaybeEngineCell::new();
    let before: &UnsafeCell<Option<i32>> = cell.get_inner();
    let installed: Result<(), i32> = cell.try_set(6);
    let after: &UnsafeCell<Option<i32>> = cell.get_inner();
    let taken: Option<i32> = cell.try_take();
    let drained: &Option<i32> = unsafe { &*after.get() };
    assert_eq!(installed, Ok(()), "setup install must succeed");
    assert_eq!(
        before as *const UnsafeCell<Option<i32>>, after as *const UnsafeCell<Option<i32>>,
        "get_inner must return the same slot the try_* methods write"
    );
    assert_eq!(taken, Some(6), "the value round-trips through take");
    assert_eq!(drained, &None, "the drained slot really is empty");
}
