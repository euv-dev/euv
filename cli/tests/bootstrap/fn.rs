use super::*;

#[test]
fn internal_bootstrap_normalization_injects_once_and_is_idempotent() {
    let source: &str = "<!doctype html><html><body><main>fixture</main></body></html>";
    let normalized: String = normalize_bootstrap(source).unwrap();
    assert_eq!(normalized.matches(INLINE_JS_PLACEHOLDER).count(), 1);
    assert!(normalized.contains("<main>fixture</main>"));
    assert_eq!(normalize_bootstrap(&normalized).unwrap(), normalized);
}

#[test]
fn internal_bootstrap_normalization_rejects_duplicate_mount_scripts() {
    let source: String = format!("<body>{INLINE_JS_PLACEHOLDER}{INLINE_JS_PLACEHOLDER}</body>");
    assert!(normalize_bootstrap(&source).is_err());
}
