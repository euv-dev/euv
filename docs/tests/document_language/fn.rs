use super::*;

#[test]
fn every_locale_delivers_its_own_document_language() {
    for (label, tag) in [
        ("简体中文", "zh-CN"),
        ("English", "en-US"),
        ("日本語", "ja-JP"),
        ("한국어", "ko-KR"),
    ] {
        let html: String = include_str!("../../template.html").to_string();
        assert_eq!(document_language_for_label(label), tag);
        let localized: String = localize_document_html(&html, tag).unwrap();
        assert!(localized.contains(&format!(r#"<html lang="{tag}">"#)));
        assert_eq!(localize_document_html(&localized, tag), Some(localized));
    }
}

#[test]
fn document_language_replacement_preserves_custom_template_attributes() {
    for html in [
        r#"<html class="shell" lang='en' dir="ltr">"#,
        r#"<HTML class="shell" LANG = en dir="ltr">"#,
    ] {
        let localized: String = localize_document_html(html, "ja-JP").unwrap();
        assert!(localized.contains(r#"lang="ja-JP""#));
        assert!(localized.contains(r#"class="shell""#));
        assert!(localized.contains(r#"dir="ltr""#));
        assert_eq!(localize_document_html(&localized, "ja-JP"), Some(localized));
    }
    assert_eq!(
        localize_document_html("<html><body></body>", "ko-KR"),
        Some(r#"<html lang="ko-KR"><body></body>"#.to_string())
    );
    assert_eq!(
        localize_document_html(r#"<html data-note="a > b" lang="en">"#, "zh-CN"),
        Some(r#"<html data-note="a > b" lang="zh-CN">"#.to_string())
    );
    assert_eq!(localize_document_html("<body></body>", "zh-CN"), None);
}

#[test]
fn localized_index_and_fallback_are_written_for_every_locale() {
    for tag in ["zh-CN", "en-US", "ja-JP", "ko-KR"] {
        let output: PathBuf = env::temp_dir().join(format!(
            "euv-docs-document-language-{}-{tag}",
            process::id()
        ));
        fs::create_dir_all(output.join(".deploy")).unwrap();
        fs::write(
            output.join("index.html"),
            include_str!("../../template.html"),
        )
        .unwrap();
        fs::write(output.join(".deploy/document-lang.txt"), tag).unwrap();
        localize_document_output(&output).unwrap();
        let html: String = fs::read_to_string(output.join("index.html")).unwrap();
        assert!(html.contains(&format!(r#"<html lang="{tag}">"#)));
        assert_eq!(
            fs::read(output.join("index.html")).unwrap(),
            fs::read(output.join("404.html")).unwrap()
        );
        fs::remove_dir_all(output).unwrap();
    }
}
