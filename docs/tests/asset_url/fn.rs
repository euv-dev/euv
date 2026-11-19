use super::*;

#[test]
fn absolute_https_badges_keep_their_origin_in_non_default_bundles() {
    for url in [
        "https://img.shields.io/crates/v/http-request.svg",
        "https://docs.rs/http-request/badge.svg",
        "https://github.com/ltpp-universe/http-request/actions/workflows/rust.yml/badge.svg",
    ] {
        assert_eq!(rebase_asset_url(url), url);
        let html: String = format!(r#"<a><img src="{url}" alt="badge"></a>"#);
        assert_eq!(rebase_html_asset_srcs(&html), html);
    }
}

#[test]
fn only_relative_assets_are_rebased_and_only_once() {
    for (url, expected) in [
        ("./img/a.png", "../img/a.png"),
        ("img/a.png", "../img/a.png"),
        ("../img/a.png", "../img/a.png"),
        ("/img/a.png", "/img/a.png"),
        ("http://example.com/a.png", "http://example.com/a.png"),
        ("//example.com/a.png", "//example.com/a.png"),
        ("data:image/png;base64,AQ==", "data:image/png;base64,AQ=="),
        (
            "blob:https://example.com/image",
            "blob:https://example.com/image",
        ),
        ("", ""),
    ] {
        assert_eq!(rebase_asset_url(url), expected);
        assert_eq!(rebase_asset_url(&rebase_asset_url(url)), expected);
    }
}

#[test]
fn raw_html_rebases_local_assets_without_rewriting_external_badges() {
    let external: &str = "https://docs.rs/http-request/badge.svg";
    let input: String = format!(r#"<img src="./img/a.png"><img src="{external}">"#);
    let expected: String = format!(r#"<img src="../img/a.png"><img src="{external}">"#);
    assert_eq!(rebase_html_asset_srcs(&input), expected);
}
