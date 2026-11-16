use super::*;

#[test]
fn stale_getting_started_spy_cannot_own_home_or_previous_guide() {
    let path: &str = "/guide/getting-started.html";
    for hash in ["#/", "#/guide/", "#/guide/#overview", "#/missing.html"] {
        assert!(!route_page_matches(path, hash), "stale spy accepted {hash}");
    }
}

#[test]
fn same_page_heading_changes_remain_owned_by_the_spy() {
    let path: &str = "/guide/getting-started.html";
    for hash in [
        "#/guide/getting-started.html",
        "#/guide/getting-started.html#install",
        "#/guide/getting-started.html#hello",
        "#/guide/getting-started.html#%E4%BD%A0%E5%A5%BD",
        "#/guide/getting-started.html#你好",
        "#/guide/getting-started.html#",
    ] {
        assert!(
            route_page_matches(path, hash),
            "same-page anchor rejected {hash}"
        );
    }
}

#[test]
fn page_ownership_is_exact_and_removes_only_the_outer_hash() {
    let path: &str = "/guide/getting-started.html";
    for hash in [
        "#/guide/getting-started.html-extra#hello",
        "#/zh/guide/getting-started.html#hello",
        "##/guide/getting-started.html#hello",
        "",
    ] {
        assert!(
            !route_page_matches(path, hash),
            "foreign page accepted {hash}"
        );
    }
    assert!(route_page_matches("/", "#/"));
    assert!(route_page_matches("/", "#/#home"));
    assert!(!route_page_matches("/", "#/guide/#home"));
}
