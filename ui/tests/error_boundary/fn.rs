use super::*;
#[test]
fn error_boundary_phase_default_is_healthy() {
    let phase: ErrorBoundaryPhase = ErrorBoundaryPhase::default();
    assert!(matches!(phase, ErrorBoundaryPhase::Healthy));
}

#[test]
fn error_boundary_phase_partial_eq_both_healthy() {
    let a: ErrorBoundaryPhase = ErrorBoundaryPhase::Healthy;
    let b: ErrorBoundaryPhase = ErrorBoundaryPhase::Healthy;
    assert_eq!(a, b);
}

#[test]
fn error_boundary_phase_partial_eq_same_message() {
    let a: ErrorBoundaryPhase = ErrorBoundaryPhase::Caught("oops".to_string());
    let b: ErrorBoundaryPhase = ErrorBoundaryPhase::Caught("oops".to_string());
    assert_eq!(a, b);
}

#[test]
fn error_boundary_phase_partial_eq_different_message() {
    let a: ErrorBoundaryPhase = ErrorBoundaryPhase::Caught("oops".to_string());
    let b: ErrorBoundaryPhase = ErrorBoundaryPhase::Caught("boom".to_string());
    assert_ne!(a, b);
}

#[test]
fn error_boundary_phase_partial_eq_healthy_vs_caught() {
    let a: ErrorBoundaryPhase = ErrorBoundaryPhase::Healthy;
    let b: ErrorBoundaryPhase = ErrorBoundaryPhase::Caught("x".to_string());
    assert_ne!(a, b);
    assert_ne!(b, a);
}

#[test]
fn new_is_healthy() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
    assert!(!matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Caught(_)
    ));
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
}

#[test]
fn default_is_healthy() {
    let boundary: ErrorBoundary = ErrorBoundary::default();
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
}

#[test]
fn phase_returns_signal_with_healthy_value() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let signal: &Signal<ErrorBoundaryPhase> = boundary.get_phase();
    assert!(matches!(signal.get(), ErrorBoundaryPhase::Healthy));
}

#[test]
fn debug_format_works() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let s: String = format!("{:?}", boundary);
    assert!(s.contains("ErrorBoundary"));
}

#[test]
fn display_format_works() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let s: String = format!("{}", boundary);
    assert!(s.contains("ErrorBoundary"));
    assert!(s.contains("Healthy"));
}

#[test]
fn try_with_success_returns_value() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let result: Result<i32, String> = boundary.try_with(|| 42);
    assert_eq!(result, Ok(42));
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
}

#[test]
fn try_with_success_string_value() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let result: Result<String, String> = boundary.try_with(|| "hello".to_string());
    assert_eq!(result, Ok("hello".to_string()));
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
}

#[test]
fn try_with_success_complex_value() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let result: Result<Vec<i32>, String> = boundary.try_with(|| vec![1, 2, 3]);
    assert_eq!(result, Ok(vec![1, 2, 3]));
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
}

#[test]
fn try_with_success_unit() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let result: Result<(), String> = boundary.try_with(|| {});
    assert_eq!(result, Ok(()));
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
}

#[test]
fn try_with_static_str_panic() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let result: Result<(), String> = boundary.try_with(|| {
        panic!("static str panic");
    });
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "static str panic");
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Caught(_)
    ));
}

#[test]
fn try_with_string_panic() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let result: Result<(), String> = boundary.try_with(|| {
        panic!("{}", "owned string panic");
    });
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "owned string panic");
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Caught(_)
    ));
}

#[test]
fn try_with_non_string_panic() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let result: Result<(), String> = boundary.try_with(|| {
        panic_any(42_i32);
    });
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "<unknown panic payload>",
        "non-string panic payloads must fall back to the documented placeholder"
    );
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Caught(_)
    ));
}

#[test]
fn reset_from_caught_returns_to_healthy() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let _ = boundary.try_with(|| {
        panic!("boom");
    });
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Caught(_)
    ));
    boundary.reset();
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
}

#[test]
fn reset_from_healthy_is_noop() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    boundary.reset();
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
}

#[test]
fn clone_shares_state() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let cloned: ErrorBoundary = boundary;
    assert!(matches!(
        cloned.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
    let _ = catch_unwind(AssertUnwindSafe(|| {
        boundary.reset();
    }));
    let _ = cloned.get_phase().get();
}

#[test]
fn phase_signal_is_reactive() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let signal: &Signal<ErrorBoundaryPhase> = boundary.get_phase();
    assert!(matches!(signal.get(), ErrorBoundaryPhase::Healthy));
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let _: Result<(), String> = boundary.try_with(|| {
            panic!("reactive test");
        });
    }));
    let value: ErrorBoundaryPhase = signal.get();
    assert!(matches!(
        value,
        ErrorBoundaryPhase::Healthy | ErrorBoundaryPhase::Caught(_)
    ));
}

#[test]
fn multiple_panics_overwrite_message() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let _ = boundary.try_with(|| {
        panic!("first");
    });
    let _ = boundary.try_with(|| {
        panic!("second");
    });
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Caught(_)
    ));
    if let ErrorBoundaryPhase::Caught(message) = boundary.get_phase().get() {
        assert_eq!(message, "second");
    } else {
        panic!("expected Caught");
    }
}

#[test]
fn panic_then_reset_then_success() {
    let boundary: ErrorBoundary = ErrorBoundary::new();
    let _ = boundary.try_with(|| {
        panic!("boom");
    });
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Caught(_)
    ));
    boundary.reset();
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
    let result: Result<i32, String> = boundary.try_with(|| 42);
    assert_eq!(result, Ok(42));
    assert!(matches!(
        boundary.get_phase().get(),
        ErrorBoundaryPhase::Healthy
    ));
}

#[test]
fn outside_a_render_the_hook_still_hands_back_a_usable_boundary() {
    let boundary: ErrorBoundary = use_error_boundary();

    assert!(
        matches!(boundary.get_phase().get(), ErrorBoundaryPhase::Healthy),
        "with no hook context active the factory result comes straight back, and a fresh \
         boundary is Healthy — not already Caught, and not an error"
    );
}

#[test]
fn the_hook_returns_the_same_boundary_on_every_render() {
    let mut context: HookContext = HookContext::default();

    let first: ErrorBoundary = HookContext::with(context.clone(), use_error_boundary);
    context.reset_index();
    let second: ErrorBoundary = HookContext::with(context, use_error_boundary);

    first.report_error("child render exploded");

    assert!(
        matches!(second.get_phase().get(), ErrorBoundaryPhase::Caught(_)),
        "a hook has to hand back the same slot on every render. If each render allocated a \
         fresh boundary, the fallback a parent showed would be forgotten on the next render \
         and a failing child would flash back to healthy instead of staying caught"
    );
}

#[test]
fn two_components_get_boundaries_of_their_own() {
    let left: HookContext = HookContext::default();
    let right: HookContext = HookContext::default();

    let first: ErrorBoundary = HookContext::with(left, use_error_boundary);
    let second: ErrorBoundary = HookContext::with(right, use_error_boundary);

    first.report_error("only the left one broke");

    assert!(
        matches!(second.get_phase().get(), ErrorBoundaryPhase::Healthy),
        "the slot belongs to the hook context, not to the hook name: one component's failure \
         must not put every other component's boundary into the fallback state"
    );
}

#[test]
fn the_hook_boundary_is_usable_rather_than_a_placeholder() {
    let boundary: ErrorBoundary = use_error_boundary();

    let value: Result<u32, String> = boundary.try_with(|| 7_u32);

    assert_eq!(
        value,
        Ok(7),
        "the handle this hook returns has to be a working boundary, because it is the only \
         thing a render ever gets"
    );
    assert!(
        matches!(boundary.get_phase().get(), ErrorBoundaryPhase::Healthy),
        "and a run that did not panic leaves the phase alone"
    );
}

#[test]
fn the_hook_boundary_catches_a_panic_and_reports_it() {
    let boundary: ErrorBoundary = use_error_boundary();

    let value: Result<u32, String> =
        boundary.try_with(|| -> u32 { panic!("child render exploded") });

    assert!(
        value.is_err(),
        "the panic is the boundary's whole purpose; a hook that returned an inert handle \
         would let every child render take the page down"
    );
    let phase: ErrorBoundaryPhase = boundary.get_phase().get();
    assert!(
        matches!(phase, ErrorBoundaryPhase::Caught(_)),
        "and the phase has to flip to Caught, because that is what the parent renders a \
         fallback from"
    );
}
