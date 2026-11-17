use super::*;

#[test]
fn snapshot_inventory_includes_unlinked_themes_and_excludes_native_metadata() {
    let files: &[(&str, &[u8])] = &[
        ("index.html", b"root"),
        ("themes/theme-shadcn/index.html", b"theme"),
        ("themes/theme-shadcn/pkg/euv_example_bg.wasm", b"wasm"),
        ("docs/data.json", b"{}"),
        ("__euv_local_bundle__.json", b"manifest"),
        ("_manifest.json", b"manifest"),
        (".empty", b""),
        ("themes/theme-shadcn/pkg/.gitignore", b"*"),
        ("themes/theme-shadcn/pkg/README.md", b"package readme"),
        ("themes/theme-shadcn/pkg/package.json", b"{}"),
        ("themes/theme-shadcn/pkg/LICENSE", b"license"),
    ];
    assert_eq!(
        snapshot_paths(files),
        vec!["index.html", "themes/theme-shadcn/index.html",]
    );
}

#[test]
fn deployment_bases_do_not_escape_the_local_document_directory() {
    for prefix in ["/www/", "/pages/euv/", "https://ltpp.vip/pages/euv/"] {
        let html: String = format!(
            "<head><base href=\"{prefix}\" /></head><script>load('./pkg/app.wasm')</script>"
        );
        assert_eq!(
            local_document(&html),
            "<head><base href=\"./\" /></head><script>load('./pkg/app.wasm')</script>"
        );
    }
    let html: &str =
        "<BASE HREF='/pages/euv/themes/theme-shadcn/'><basefont size='1'><body>Theme</body>";
    assert_eq!(
        local_document(html),
        "<base href=\"./\" /><basefont size='1'><body>Theme</body>"
    );
    assert_eq!(
        local_document("<html>No base</html>"),
        "<html>No base</html>"
    );
}

#[test]
fn old_root_only_snapshots_are_repaired_but_complete_snapshots_remain_usable() {
    let runtime: Runtime = Builder::new_current_thread().enable_all().build().unwrap();
    runtime.block_on(async {
        let directory: PathBuf =
            env::temp_dir().join(format!("euv-snapshot-completeness-{}", id()));
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join("index.html"), b"old root").unwrap();
        let paths: Vec<String> = vec![
            "index.html".to_string(),
            "themes/theme-shadcn/index.html".to_string(),
        ];
        assert!(!complete_snapshot(&directory, &paths).await);
        fs::create_dir_all(directory.join("themes/theme-shadcn")).unwrap();
        fs::write(
            directory.join("themes/theme-shadcn/index.html"),
            b"new theme",
        )
        .unwrap();
        assert!(complete_snapshot(&directory, &paths).await);
        fs::write(directory.join("themes/theme-shadcn/index.html"), b"").unwrap();
        assert!(!complete_snapshot(&directory, &paths).await);
        fs::remove_dir_all(directory).unwrap();
    });
}
