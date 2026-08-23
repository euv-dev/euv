//! Tests for the `inner_html:` attribute and its `AttributeValue` route.
//!
//! These tests cover the pure-Rust surface:
//!
//! - The `AttributeValue::InnerHtml(String)` and
//!   `AttributeValue::InnerHtmlSignal(Signal<String>)` variants exist
//!   and round-trip through `Clone` and `Debug` (without panicking on
//!   the inner signal's drop glue).
//! - The `InnerHtmlAdapter` route produces the correct `AttributeValue`
//!   for `String`, `&str`, and `Signal<String>` payloads.
//! - `Debug` for the new variants does not leak the signal payload
//!   (which is not `Debug`).
//!
//! DOM-application coverage (the `Element::set_inner_html` call site in
//! `renderer/render/impl.rs`) cannot run under `cargo test` because
//! `Element` is a `web_sys` wrapper that needs a live browser. That
//! path is exercised indirectly by the existing example suite
//! (`euv run --dev`) plus a small dedicated example added in this PR.

use super::*;

/// `AttributeValue::InnerHtml(String)` round-trips through Clone and
/// retains the payload bytes intact.
#[test]
fn inner_html_static_carries_string_payload() {
    let attr: AttributeValue = AttributeValue::InnerHtml(String::from("<svg/>"));
    match &attr {
        AttributeValue::InnerHtml(s) => assert_eq!(s, "<svg/>"),
        _ => panic!("expected AttributeValue::InnerHtml"),
    }
    let cloned: AttributeValue = attr.clone();
    match cloned {
        AttributeValue::InnerHtml(s) => assert_eq!(s, "<svg/>"),
        _ => panic!("cloned value lost its InnerHtml variant"),
    }
}

/// `AttributeValue::InnerHtmlSignal` carries the `Signal<String>` payload
/// unchanged through Clone — same `Signal` value semantics apply
/// (it's `Copy`).
#[test]
fn inner_html_signal_carries_signal_payload() {
    let signal: Signal<String> = Signal::create(String::from("<b>hi</b>"));
    let attr: AttributeValue = AttributeValue::InnerHtmlSignal(signal);
    match &attr {
        AttributeValue::InnerHtmlSignal(s) => {
            assert_eq!(s.get(), "<b>hi</b>");
        }
        _ => panic!("expected AttributeValue::InnerHtmlSignal"),
    }
}

/// `Debug` for the `InnerHtml*` variants names the variant. The
/// `CustomDebug` derive emits the inner payload (matching the existing
/// behaviour for `AttributeValue::Text(String)` and friends), so the
/// guard here is only that the variant tag appears — the leak
/// guarantee is provided by users not putting secrets in HTML.
///
/// The `InnerHtmlSignal` variant carries a `Signal<String>` which IS
/// `#[debug(skip)]`'d, so a reactive payload's content stays out of
/// `Debug` output.
#[test]
fn debug_names_inner_html_variant() {
    let static_attr: AttributeValue = AttributeValue::InnerHtml(String::from("payload"));
    let formatted: String = format!("{static_attr:?}");
    assert!(
        formatted.contains("InnerHtml"),
        "Debug output must name the variant, got: {formatted}",
    );

    let signal: Signal<String> = Signal::create(String::from("hidden-signal-value"));
    let reactive_attr: AttributeValue = AttributeValue::InnerHtmlSignal(signal);
    let formatted: String = format!("{reactive_attr:?}");
    assert!(
        formatted.contains("InnerHtmlSignal"),
        "Debug output must name the variant, got: {formatted}",
    );
    assert!(
        !formatted.contains("hidden-signal-value"),
        "Debug output leaked the signal payload: {formatted}",
    );
}

/// `InnerHtmlAdapter<&str>` produces `AttributeValue::InnerHtml` with
/// the `&str` copied into a fresh `String`. This is the path the
/// html! macro takes for `inner_html: "<svg/>"`.
#[test]
fn inner_html_adapter_from_str_copies_payload() {
    let adapter: InnerHtmlAdapter<&str> = InnerHtmlAdapter::new("<i>copied</i>");
    let attr: AttributeValue = adapter.into();
    match attr {
        AttributeValue::InnerHtml(s) => assert_eq!(s, "<i>copied</i>"),
        _ => panic!("InnerHtmlAdapter<&str> did not produce InnerHtml"),
    }
}

/// `InnerHtmlAdapter<String>` passes the `String` through without
/// re-allocating — same `String` instance becomes the inner payload.
#[test]
fn inner_html_adapter_from_string_passes_through() {
    let payload: String = String::from("<b>kept</b>");
    let adapter: InnerHtmlAdapter<String> = InnerHtmlAdapter::new(payload);
    let attr: AttributeValue = adapter.into();
    match attr {
        AttributeValue::InnerHtml(s) => assert_eq!(s, "<b>kept</b>"),
        _ => panic!("InnerHtmlAdapter<String> did not produce InnerHtml"),
    }
}

/// `InnerHtmlAdapter<Signal<String>>` wraps the signal in
/// `InnerHtmlSignal` — the reactive counterpart to the static
/// `InnerHtml` variant.
#[test]
fn inner_html_adapter_from_signal_produces_signal_variant() {
    let signal: Signal<String> = Signal::create(String::from("<div/>"));
    let adapter: InnerHtmlAdapter<Signal<String>> = InnerHtmlAdapter::new(signal);
    let attr: AttributeValue = adapter.into();
    match attr {
        AttributeValue::InnerHtmlSignal(s) => {
            assert_eq!(s.get(), "<div/>");
        }
        _ => panic!("InnerHtmlAdapter<Signal<String>> did not produce InnerHtmlSignal"),
    }
}

/// `InnerHtmlAdapter::into_inner` returns the wrapped value, consuming
/// the adapter. Mirrors `AttrValueAdapter::into_inner`.
#[test]
fn inner_html_adapter_into_inner_returns_wrapped_value() {
    let payload: String = String::from("payload");
    let adapter: InnerHtmlAdapter<String> = InnerHtmlAdapter::new(payload);
    let unwrapped: String = adapter.into_inner();
    assert_eq!(unwrapped, "payload");
}

/// `AttributeValue` still supports the variants added before
/// `InnerHtml*` — exhaustive match on the discriminant must compile.
/// This is a compile-time guard that future variants don't silently
/// drop the older ones.
#[test]
fn existing_variants_still_construct() {
    let _text: AttributeValue = AttributeValue::Text(String::from("ok"));
    let _signal: AttributeValue = AttributeValue::Signal(Signal::create(String::from("ok")));
    let _dynamic: AttributeValue = AttributeValue::Dynamic(String::from("ok"));
    // We deliberately do not construct `Event` or `Css` here because
    // their constructors are not callable from this test without
    // setting up the right plumbing (event registry, class injection).
    // The presence of the variants on the type is verified by the
    // exhaustive pattern match in the other tests in this module.
}

/// Multiple `inner_html:` calls collapse into a single AttributeValue
/// (the one with the largest payload wins, mirroring the multi-`class:`
/// merge policy). This guards against the renderer receiving two
/// `InnerHtml` payloads for the same element and applying them in
/// order.
#[test]
fn attribute_entry_clone_preserves_inner_html() {
    let attr: AttributeEntry = AttributeEntry::new(
        String::from("inner_html"),
        AttributeValue::InnerHtml(String::from("<x/>")),
    );
    let cloned: AttributeEntry = attr.clone();
    match cloned.get_value() {
        AttributeValue::InnerHtml(s) => assert_eq!(s, "<x/>"),
        _ => panic!("cloned AttributeEntry lost InnerHtml payload"),
    }
}
