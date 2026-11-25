use super::*;

#[test]
fn markdown_parser_extracts_frontmatter_toc_and_code_blocks() {
    let sample: &str = "---\ntitle: Architecture\norder: 2\ntagline: Native 7-platform runtime\n---\n\n## Core Engine\n\nUnified retained scene graph.\n\n```rust\nlet sig = Signal::new(42);\n```\n\n- Zero WebView\n- Bit-identical rendering\n";
    let page: DocPage = parse_markdown_page("/guide/architecture", "guide", sample);
    assert_eq!(page.get_title(), "Architecture");
    assert_eq!(page.get_order(), 2);
    assert_eq!(page.get_summary(), "Native 7-platform runtime");
    assert_eq!(page.get_toc().len(), 1);
    assert_eq!(page.get_toc()[0].get_slug(), "core-engine");
    assert!(page.get_blocks().len() >= 4);
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
    drop(fs::remove_dir_all(&out_dir));
}
