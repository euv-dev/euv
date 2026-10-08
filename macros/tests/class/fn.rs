use super::*;

class! {
    c_test_red {
        color: "red";
    }
    c_test_blue {
        color: "blue";
        background: "white";
    }
}

#[test]
fn class_macro_emits_function_with_static_css_return_type() {
    let _emit: fn() -> &'static Css = c_test_red;
}

#[test]
fn class_macro_generates_one_function_per_definition() {
    let _red: fn() -> &'static Css = c_test_red;
    let _blue: fn() -> &'static Css = c_test_blue;
}

#[test]
fn class_macro_uses_once_lock_for_caching() {
    let outcome: thread::Result<(&Css, &Css)> = catch_unwind(AssertUnwindSafe(|| {
        let first: &Css = c_test_red();
        let second: &Css = c_test_red();
        (first, second)
    }));
    let (first, second): (&Css, &Css) = outcome.expect(
        "inject_style returns early on a non-wasm target instead of reaching window(), \
         so a styled class function must be callable from a native cargo test process",
    );
    assert!(
        ptr::eq(first, second),
        "the OnceLock must hand back the same Css on the second call"
    );
}
