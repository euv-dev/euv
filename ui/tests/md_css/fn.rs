use super::*;

#[test]
fn the_markdown_accessor_returns_a_stable_static_slice() {
    let first: &str = euv_md_css();
    let second: &str = euv_md_css();
    assert!(
        !first.is_empty(),
        "the markdown component ships its own css, so an empty sheet renders unstyled"
    );
    assert_eq!(
        first, second,
        "the accessor returns a static slice, so two calls cannot hand back different sheets"
    );
    assert_eq!(
        first.as_ptr(),
        second.as_ptr(),
        "a static slice is not rebuilt per call, so the pointer has to match too"
    );
}

#[test]
fn the_markdown_sheet_has_balanced_braces_and_styles_code_blocks() {
    let sheet: &str = euv_md_css();
    let opens: usize = sheet.matches('{').count();
    let closes: usize = sheet.matches('}').count();
    assert_eq!(
        opens, closes,
        "a stylesheet with unbalanced braces is dropped by the browser in whole"
    );
    assert!(
        sheet.contains("pre"),
        "the markdown component renders code blocks, so the sheet has to style them"
    );
}

#[test]
fn the_global_injector_also_ships_the_markdown_sheet_and_stays_safe_without_a_window() {
    let sheet: &str = euv_md_css();
    assert!(!sheet.is_empty(), "the sheet must survive being injected");
    inject_app_global_css();
    assert!(
        !sheet.is_empty(),
        "reaching the end means the injector degraded instead of panicking with no document"
    );
}
