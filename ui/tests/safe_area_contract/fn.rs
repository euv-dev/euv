use super::*;

#[test]
fn only_ios_and_desktop_safari_reserve_a_safe_area() {
    let cases: [(&str, bool); 8] = [
        (
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
            true,
        ),
        (
            "Mozilla/5.0 (iPad; CPU OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
            true,
        ),
        (
            "Mozilla/5.0 (iPod touch; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1",
            true,
        ),
        (
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15",
            true,
        ),
        (
            "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Mobile Safari/537.36",
            false,
        ),
        (
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
            false,
        ),
        (
            "Mozilla/5.0 (X11; Linux x86_64; rv:126.0) Gecko/20100101 Firefox/126.0",
            false,
        ),
        (
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36 Edg/125.0.0.0",
            false,
        ),
    ];
    for (user_agent, expected) in cases {
        assert_eq!(
            needs_safe_area_insets(user_agent),
            expected,
            "user agent {user_agent:?} was classified wrongly"
        );
    }
}

#[test]
fn an_ios_user_agent_inside_another_browser_still_counts() {
    let ios_chrome: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/125.0.0.0 Mobile/15E148 Safari/604.1";
    assert!(
        needs_safe_area_insets(ios_chrome),
        "the notch is a property of the screen, not of the browser installed on it, so Chrome on iOS still needs the inset"
    );
}

#[test]
fn an_empty_or_absent_user_agent_reserves_nothing() {
    assert!(
        !needs_safe_area_insets(""),
        "no user agent means no evidence of a notched screen, and reserving space on a guess is the defect being fixed"
    );
}

#[test]
fn an_android_host_is_gated_out_even_though_it_declares_immersive() {
    let android: &str = "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Mobile Safari/537.36";
    assert!(
        !needs_safe_area_insets(android),
        "Android is excluded from the side-inset gate, so right/bottom/left collapse to 0px there even when the host declares edge-to-edge"
    );
    assert!(
        is_edge_to_edge_viewport(true, Some(0.0)),
        "the injected top inset is a separate decision: an immersive Android WebView still resolves it, which is what --euv-mobile-safe-top exists for"
    );
}

#[test]
fn an_immersive_declaration_is_trusted_even_when_the_measurement_is_missing() {
    assert!(
        is_edge_to_edge_viewport(true, None),
        "a host that declares edge-to-edge knows its own layout, so an unavailable measurement must not revoke the trust it asserted"
    );
}

#[test]
fn an_immersive_declaration_outranks_a_letterbox_looking_measurement() {
    assert!(
        is_edge_to_edge_viewport(true, Some(360.0)),
        "the declaration is the explicit escape hatch for hosts whose viewport does not track the physical screen"
    );
}

#[test]
fn a_letterboxed_browser_viewport_is_never_trusted() {
    assert!(
        !is_edge_to_edge_viewport(false, Some(120.0)),
        "a 120px gap between screen and viewport is a letterbox under a system bar, which is exactly the host that lies about env()"
    );
    assert!(
        !is_edge_to_edge_viewport(false, Some(41.0)),
        "the reported VivoBrowser-class gap must fall on the same side of the line as any other letterbox"
    );
}

#[test]
fn an_unmeasurable_undeclared_viewport_defaults_to_untrusted() {
    assert!(
        !is_edge_to_edge_viewport(false, None),
        "with no declaration and no measurement there is no evidence of edge-to-edge, so the safe default is zero"
    );
}

#[test]
fn a_flush_immersive_webview_viewport_is_trusted() {
    assert!(
        is_edge_to_edge_viewport(false, Some(0.0)),
        "a viewport whose height matches the screen really does reach the screen edges, so env() is meaningful"
    );
    assert!(
        is_edge_to_edge_viewport(false, Some(12.0)),
        "a small gap is sub-pixel and browser chrome noise rather than a status bar band"
    );
}

#[test]
fn the_letterbox_tolerance_boundary_is_exclusive() {
    assert!(
        !is_edge_to_edge_viewport(false, Some(24.0)),
        "at the tolerance the gap is wide enough to be a real system bar, so it must not be trusted"
    );
    assert!(
        is_edge_to_edge_viewport(false, Some(23.0)),
        "just under the tolerance is still an edge-to-edge viewport"
    );
}

#[test]
fn an_untrusted_viewport_resolves_every_side_to_zero() {
    assert_eq!(
        safe_area_contract_value(false, "34px"),
        "0px",
        "the whole defect is a browser reporting 34px of inset for a page that is not under any bar, so an untrusted host must collapse to zero"
    );
    assert_eq!(
        safe_area_contract_value(false, "41px"),
        "0px",
        "the lie does not get smaller by being different"
    );
}

#[test]
fn a_trusted_viewport_keeps_the_measured_inset() {
    assert_eq!(
        safe_area_contract_value(true, "34px"),
        "34px",
        "a genuinely edge-to-edge host must still get the inset it really has, or its content slides under the gesture bar"
    );
    assert_eq!(
        safe_area_contract_value(true, " 24px "),
        "24px",
        "getComputedStyle can hand back padded values, and the padding is not part of the length"
    );
}

#[test]
fn a_trusted_viewport_still_collapses_a_missing_or_malformed_measurement() {
    assert_eq!(
        safe_area_contract_value(true, ""),
        "0px",
        "no measurement means no inset, and an empty value would otherwise emit invalid css"
    );
    assert_eq!(
        safe_area_contract_value(true, "0px"),
        "0px",
        "a real zero is already the default, so normalising it changes nothing"
    );
    assert_eq!(
        safe_area_contract_value(true, "auto"),
        "0px",
        "a non-length token is not a safe-area inset and must never reach a padding declaration"
    );
    assert_eq!(
        safe_area_contract_value(true, "34"),
        "0px",
        "a unitless number cannot resolve inside calc(), so it is not a usable inset"
    );
}

#[test]
fn every_shell_edge_varies_only_with_a_trusted_measurement() {
    for measured in ["0px", "1px", "34px", "48px", "128px"] {
        assert_eq!(
            safe_area_contract_value(false, measured),
            "0px",
            "an untrusted host must never produce a non-zero edge, whatever the browser reported"
        );
    }
}
