use super::*;

#[test]
fn a_fresh_form_reports_every_field_as_empty_and_untouched() {
    let state: FormState = HookContext::with(HookContext::default(), HookContext::form);
    assert_eq!(state.field("email"), "", "an unentered field has no value");
    assert_eq!(
        state.error("email"),
        "",
        "a field with no validation has no error"
    );
    assert!(
        !state.is_touched("email"),
        "a field the user has not reached must not count as touched"
    );
    assert_eq!(state.error_count(), 0);
}

#[test]
fn a_form_keeps_each_field_separate() {
    let state: FormState = HookContext::with(HookContext::default(), HookContext::form);
    state.set_field("email", "ada@example.com");
    state.set_field("name", "Ada");
    assert_eq!(state.field("email"), "ada@example.com");
    assert_eq!(
        state.field("name"),
        "Ada",
        "one field's write must not land in another"
    );
    assert_eq!(state.field("missing"), "");
}

#[test]
fn a_touched_field_is_remembered() {
    let state: FormState = HookContext::with(HookContext::default(), HookContext::form);
    state.touch("email");
    assert!(state.is_touched("email"));
    assert!(
        !state.is_touched("name"),
        "touching one field must not touch its neighbour"
    );
}

#[test]
fn resetting_a_form_forgets_every_field() {
    let state: FormState = HookContext::with(HookContext::default(), HookContext::form);
    state.set_field("email", "ada@example.com");
    state.touch("email");
    state.reset();
    assert_eq!(state.field("email"), "", "a reset must clear the values");
    assert!(!state.is_touched("email"), "and the touched flags");
}

#[test]
fn two_forms_in_different_slots_do_not_share_state() {
    let first: FormState = HookContext::with(HookContext::default(), HookContext::form);
    let second: FormState = HookContext::with(HookContext::default(), HookContext::form);
    first.set_field("email", "a@b.c");
    assert_eq!(
        second.field("email"),
        "",
        "each slot owns its own form, so a write in one is invisible to the other"
    );
}

#[test]
fn a_fresh_transition_is_not_animating() {
    let state: TransitionState = HookContext::with(HookContext::default(), || {
        HookContext::transition(TransitionConfig::default())
    });
    assert!(
        !state.is_animating(),
        "a transition starts idle by definition"
    );
}

#[test]
fn entering_a_transition_starts_it_and_ticking_finishes_it() {
    let state: TransitionState = HookContext::with(HookContext::default(), || {
        HookContext::transition(TransitionConfig::default())
    });
    state.enter();
    assert!(state.is_animating(), "entering must start the animation");
    state.tick_until_done(50);
    assert!(
        !state.is_animating(),
        "ticking to completion must land on the finished state"
    );
}

#[test]
fn a_transition_reset_returns_it_to_idle() {
    let state: TransitionState = HookContext::with(HookContext::default(), || {
        HookContext::transition(TransitionConfig::default())
    });
    state.enter();
    state.reset();
    assert!(!state.is_animating());
    assert_eq!(
        state.remaining_ms(),
        0,
        "a reset transition has no time left to run"
    );
}

#[test]
fn the_unit_loading_hint_is_empty() {
    let hint: () = <() as HasLoadingHint>::empty();
    assert_eq!(
        hint,
        (),
        "the unit type carries no metadata, so its empty hint is the unit value"
    );
}

#[test]
fn a_loading_hint_is_only_required_to_be_cloneable_and_constructible() {
    fn empty_of<T: HasLoadingHint>() -> T {
        T::empty()
    }
    let unit: () = empty_of();
    assert_eq!(unit, ());
}

#[test]
fn a_profiler_handle_can_be_built_with_no_entries() {
    let profiler: ProfilerHandle = ProfilerHandle::new_with_empty_entries();
    assert!(
        profiler.get_entries().get().is_empty(),
        "the named constructor exists so tools start from a known state"
    );
}

#[test]
fn a_built_profiler_measures_like_any_other() {
    let profiler: ProfilerHandle = ProfilerHandle::new_with_empty_entries();
    let result: u32 = profiler.measure("built", || 6 * 7);
    assert_eq!(result, 42);
    assert_eq!(profiler.get_entries().get().len(), 1);
}

#[test]
fn the_i18n_extension_trait_is_what_provides_the_hook() {
    fn locale_of_slot() -> I18n {
        HookContext::i18n()
    }
    let handle: I18n = HookContext::with(HookContext::default(), locale_of_slot);
    assert_eq!(
        handle.get_locale().get(),
        "en",
        "HookContextI18nExt::i18n is the factory App::use_i18n delegates to, \
         and it must produce a usable handle on its own"
    );
}

#[test]
fn the_form_extension_trait_is_what_provides_the_hook() {
    fn form_of_slot() -> FormState {
        HookContext::form()
    }
    let state: FormState = HookContext::with(HookContext::default(), form_of_slot);
    state.set_field("email", "ada@example.com");
    assert_eq!(
        state.field("email"),
        "ada@example.com",
        "HookContextFormExt::form must yield a working form, not a placeholder"
    );
}

#[test]
fn the_transition_extension_trait_is_what_provides_the_hook() {
    fn transition_of_slot() -> TransitionState {
        HookContext::transition(TransitionConfig::default())
    }
    let state: TransitionState = HookContext::with(HookContext::default(), transition_of_slot);
    state.enter();
    assert!(
        state.is_animating(),
        "HookContextTransitionExt::transition must yield a live transition"
    );
}

#[test]
fn the_default_loading_handle_alias_expands_to_the_unit_hint_variant() {
    let typed: DefaultLoadingHandle<u32> = DefaultLoadingHandle::default();
    let expanded: UseAsyncHandle<u32, ()> = typed;
    let _: String = format!("{expanded:?}");
}

#[test]
fn a_measured_block_lands_in_the_profiler_of_its_own_slot() {
    let entries: Vec<ProfileEntry> = HookContext::with(HookContext::default(), || {
        profiler_measure("inside", || ());
        use_profiler().get_entries().get()
    });
    let fresh: Vec<ProfileEntry> = HookContext::with(HookContext::default(), || {
        use_profiler().get_entries().get()
    });
    assert!(
        entries.is_empty(),
        "profiler_measure resolves the profiler at the call site's own hook \
         index, so it must not leak into a profiler fetched separately"
    );
    assert!(fresh.is_empty());
}

fn form_through_the_trait<C: HookContextFormExt>() -> FormState {
    C::form()
}

fn i18n_through_the_trait<C: HookContextI18nExt>() -> I18n {
    C::i18n()
}

fn transition_through_the_trait<C: HookContextTransitionExt>() -> TransitionState {
    C::transition(TransitionConfig::default())
}

#[test]
fn each_context_extension_is_reachable_through_its_trait() {
    let form: FormState = form_through_the_trait::<HookContext>();
    let i18n: I18n = i18n_through_the_trait::<HookContext>();
    let transition: TransitionState = transition_through_the_trait::<HookContext>();

    assert_eq!(
        form.field("email"),
        "",
        "the form extension hands back a fresh form, and a fresh form has no fields yet"
    );
    assert!(
        !transition.is_animating(),
        "and the transition extension a transition that has not been driven yet"
    );
    let _: String = i18n.get_locale().get();
    assert!(
        matches!(
            transition.get_phase().get(),
            TransitionPhase::Exited | TransitionPhase::Entered
        ),
        "and the transition extension a handle whose phase a caller can read and branch \
         on, rather than an opaque placeholder"
    );
}
