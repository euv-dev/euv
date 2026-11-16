use super::*;

fn hover_geometry_violations(source: &str) -> Vec<String> {
    let mut violations: Vec<String> = Vec::new();
    for (offset, _) in source.match_indices(":hover") {
        let tail: &str = &source[offset..];
        let Some((_, declarations)) = tail.split_once('{') else {
            violations.push(tail.to_string());
            continue;
        };
        let Some((body, _)) = declarations.split_once('}') else {
            violations.push(tail.to_string());
            continue;
        };
        for declaration in body
            .split(';')
            .map(str::trim)
            .filter(|part: &&str| !part.is_empty())
        {
            let property: &str = declaration
                .split_once(':')
                .map_or(declaration, |(name, _): (&str, &str)| name.trim());
            if !matches!(
                property,
                "background"
                    | "background-color"
                    | "color"
                    | "border-color"
                    | "border-top-color"
                    | "border-right-color"
                    | "border-bottom-color"
                    | "border-left-color"
                    | "box-shadow"
                    | "text-shadow"
                    | "outline-color"
                    | "text-decoration-color"
                    | "fill"
                    | "stroke"
            ) {
                violations.push(declaration.to_string());
            }
        }
    }
    violations
}

fn assert_source_hover_is_paint_only(path: &Path) {
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("consumer source directory") {
            assert_source_hover_is_paint_only(&entry.expect("consumer source entry").path());
        }
        return;
    }
    if !matches!(path.extension().and_then(OsStr::to_str), Some("rs" | "css")) {
        return;
    }
    let source: String = fs::read_to_string(path).expect("consumer Rust or CSS source");
    let violations: Vec<String> = hover_geometry_violations(&source);
    assert!(
        violations.is_empty(),
        "{} contains non-paint hover declarations: {violations:?}",
        path.display()
    );
}

#[test]
fn consumer_hover_rules_are_paint_only_in_every_feature_branch() {
    let manifest: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert_source_hover_is_paint_only(&manifest.join("src"));
    assert_source_hover_is_paint_only(&manifest.join("../example/src"));
}

#[test]
fn hover_guard_rejects_geometry_metrics_and_indirect_motion() {
    for declaration in [
        "transform: translateY(-1px)",
        "translate: 0 -1px",
        "scale: 1.1",
        "rotate: 5deg",
        "padding: 1px",
        "border: 0",
        "border-width: 2px",
        "font-weight: 700",
        "font-size: 2em",
        "letter-spacing: 1px",
        "line-height: 2",
        "width: 2px",
        "top: 1px",
        "display: block",
        "animation: bounce 1s",
        "--offset: 2px",
    ] {
        let css: String = format!(".control:hover {{ {declaration}; }}");
        assert_eq!(hover_geometry_violations(&css), vec![declaration], "{css}");
    }
}

#[test]
fn hover_guard_keeps_shadows_colors_and_non_hover_transforms() {
    let source: &str = ".drawer_open { transform: translateX(0); } .drag { translate: 2px 3px; } .press:active { transform: scale(0.95); } .demo { animation: spin 1s; } .control:hover { color: var(--foreground); background: var(--muted); border-color: currentColor; box-shadow: 0 2px 4px black; }";
    assert!(hover_geometry_violations(source).is_empty());
}
