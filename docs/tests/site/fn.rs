use super::*;

#[test]
fn markdown_parser_extracts_frontmatter_toc_and_code_blocks() {
    let sample: &str = "---\ntitle: Architecture\norder: 2\ntagline: Native 7-platform runtime\n---\n\n## Core Engine\n\nUnified retained scene graph.\n\n```rust\nlet sig = Signal::new(42);\n```\n\n- Zero WebView\n  - Nested child item\n- Bit-identical rendering\n";
    let page: DocPage = parse_markdown_page("/guide/architecture", "guide", sample);
    assert_eq!(page.get_title(), "Architecture");
    assert_eq!(page.get_order(), 2);
    assert_eq!(page.get_summary(), "Native 7-platform runtime");
    assert_eq!(page.get_toc().len(), 1);
    assert_eq!(page.get_toc()[0].get_slug(), "core-engine");
    assert!(page.get_blocks().len() >= 5);
    assert!(
        page.get_blocks()
            .iter()
            .any(|b: &DocBlock| matches!(b, DocBlock::ListItem(s) if s.contains("Zero WebView")))
    );
    assert!(
        page.get_blocks().iter().any(
            |b: &DocBlock| matches!(b, DocBlock::ListItem(s) if s.contains("Nested child item"))
        )
    );
}

#[test]
fn markdown_html_preserves_inline_rich_syntax_unicode_slugs_and_blocks_xss() {
    let base: PathBuf = env::temp_dir().join("euv_docs_rich_markdown_test");
    drop(fs::remove_dir_all(&base));
    let src: PathBuf = base.join("src");
    let out: PathBuf = src.join("custom-site-out");
    fs::create_dir_all(src.join("guide")).unwrap();
    fs::write(src.join("guide/diagram.svg"), "<svg></svg>").unwrap();
    fs::write(
        src.join("README.md"),
        "---\nsite:\n  title: Hyper Docs <Test>\n---\n\n# Hyper Docs <Test>\n\nFirst intro paragraph with **bold**, *italic*, ~~strike~~, `inline_code()`, [Guide](./guide/features.md), and [Web](https://example.com).\n",
    )
    .unwrap();
    fs::write(
        src.join("guide/features.md"),
        "# Features\n\n## 标题导航\n\n## Overview {#custom-id}\n\n## Overview\n\n1. Ordered one\n2. Ordered two\n\n- [x] Done item\n- [ ] Todo item\n\n| Col A | Col B | Col C |\n| :--- | :---: | ---: |\n| left | center | right |\n\n::: tip Pro Tip\nUse **clean URLs** and `euv docs`.\n:::\n\n::: details Expand Code\n```rust\nfn main() {}\n```\n:::\n\n![Arch](./diagram.svg)\n\n[Bad](javascript:alert(1))\n\n<script>alert('xss')</script>\n",
    )
    .unwrap();
    let mut opts: DocsBuildOptions = DocsBuildOptions::default();
    opts.set_base_url("/docs".to_string());
    let report: GeneratedSiteReport = build_docs_bundle(&src, &out, &opts).unwrap();
    assert_eq!(report.get_page_count(), 2);
    let home_html: String = fs::read_to_string(out.join("index.html")).unwrap();
    assert!(home_html.contains("<strong>bold</strong>"));
    assert!(home_html.contains("<em>italic</em>"));
    assert!(home_html.contains("<del>strike</del>"));
    assert!(home_html.contains("<code>inline_code()</code>"));
    assert!(home_html.contains("href=\"/docs/guide/features/\""));
    assert!(home_html.contains("target=\"_blank\" rel=\"noopener noreferrer\""));
    assert!(home_html.contains("<span class=\"brand-mark\">H</span>"));
    assert!(!home_html.contains("<p class=\"lead\">"));
    let feat_html: String = fs::read_to_string(out.join("guide/features/index.html")).unwrap();
    assert!(feat_html.contains("id=\"标题导航\""));
    assert!(feat_html.contains("id=\"custom-id\""));
    assert!(feat_html.contains("id=\"overview\""));
    assert!(feat_html.contains("<ol>"));
    assert!(feat_html.contains("type=\"checkbox\" disabled checked"));
    assert!(feat_html.contains("style=\"text-align:center\""));
    assert!(feat_html.contains("style=\"text-align:right\""));
    assert!(feat_html.contains("class=\"callout callout-tip\""));
    assert!(feat_html.contains("<details class=\"callout callout-details\">"));
    assert!(feat_html.contains("src=\"/docs/guide/diagram.svg\""));
    assert!(out.join("guide/diagram.svg").exists());
    let readme_h2: DocPage = parse_markdown_page(
        "/",
        "General",
        "<center>\n\n## euv\n\n</center>\n\n[Api Docs](https://docs.rs/euv)\n\n## Highlights\n\nFast native UI.\n",
    );
    assert_eq!(readme_h2.get_title(), "euv");
    assert_eq!(readme_h2.get_toc().len(), 1);
    assert_eq!(readme_h2.get_toc()[0].get_title(), "Highlights");
    let readme_site: DocSite = DocSite::new("euv".to_string(), "desc".to_string(), vec![readme_h2]);
    let readme_html: String = render_page_html(&readme_site, &readme_site.get_pages()[0], "now");
    assert!(!readme_html.contains("<h2 id=\"euv\">"));
    assert!(!readme_html.contains("<p class=\"lead\">"));
    assert!(readme_html.contains("<h2 id=\"highlights\">"));
    assert!(!feat_html.contains("href=\"javascript:"));
    assert!(!feat_html.contains("<script>alert('xss')</script>"));
    assert!(feat_html.contains("&lt;script&gt;alert('xss')&lt;/script&gt;"));
    assert!(!feat_html.contains("res.innerHTML="));
    for entry_res in fs::read_dir(&out).unwrap() {
        let entry: DirEntry = entry_res.unwrap();
        let name: String = entry.file_name().to_string_lossy().into_owned();
        assert!(!name.ends_with(".js"));
    }
    drop(fs::remove_dir_all(&base));
}

#[test]
fn docs_loader_viewer_and_static_generator_work_end_to_end() {
    reset_reactive_runtime();
    let docs_root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/en");
    let site: DocSite = load_doc_site(&docs_root).unwrap();
    assert!(!site.get_pages().is_empty());
    let hits: Vec<&DocPage> = site.search("Getting Started");
    assert!(!hits.is_empty());
    let stats: FrameStats = render_docs_smoke(&site, true).unwrap();
    assert_ne!(stats.get_frame_hash(), 0);
    assert!(stats.get_draw_commands() >= 4);
    let out_dir: PathBuf = env::temp_dir().join("euv_docs_test_bundle");
    let report: GeneratedSiteReport = generate_static_docs(&site, &out_dir).unwrap();
    assert_eq!(report.get_page_count(), site.get_pages().len());
    assert!(Path::new(report.get_index_path()).exists());
    assert!(out_dir.join("index.html").exists());
    assert!(out_dir.join("sitemap.xml").exists());
    let home_html: String = fs::read_to_string(out_dir.join("index.html")).unwrap();
    assert!(home_html.contains("euv-search"));
    assert!(home_html.contains("On this page"));
    drop(fs::remove_dir_all(&out_dir));
}
