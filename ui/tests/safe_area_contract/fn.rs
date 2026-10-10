use super::*;

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
