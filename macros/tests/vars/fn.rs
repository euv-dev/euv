use super::*;

vars! {
    pub c_test_bg {
        background: "white";
        foreground: "black";
    }
    pub c_test_fg {
        color: "blue";
    }
}

#[test]
fn vars_macro_emits_function_returning_static_css_ref() {
    let _emit: fn() -> &'static Css = c_test_bg;
}

#[test]
fn vars_macro_generates_one_function_per_block() {
    let _bg: fn() -> &'static Css = c_test_bg;
    let _fg: fn() -> &'static Css = c_test_fg;
}

#[test]
fn vars_macro_uses_once_lock_for_caching() {
    let outcome: thread::Result<(&Css, &Css)> = catch_unwind(AssertUnwindSafe(|| {
        let first: &Css = c_test_bg();
        let second: &Css = c_test_bg();
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
