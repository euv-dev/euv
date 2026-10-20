use super::*;

fn rust_sources(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            rust_sources(&path, found);
        } else if path
            .extension()
            .is_some_and(|extension: &OsStr| extension == "rs")
        {
            found.push(path);
        }
    }
}

fn anchor_scroll_hook() -> String {
    let root: PathBuf = [env!("CARGO_MANIFEST_DIR"), "src"].iter().collect();
    let mut files: Vec<PathBuf> = Vec::new();
    rust_sources(&root, &mut files);
    for path in files {
        let Ok(source) = fs::read_to_string(&path) else {
            continue;
        };
        let Some(start) = source.find("fn use_anchor_scroll") else {
            continue;
        };
        let rest: &str = &source[start..];
        let end: usize = rest.find("\nfn ").unwrap_or(rest.len());
        return rest[..end].to_string();
    }
    panic!(
        "the docs shell has no use_anchor_scroll hook under {}",
        root.display()
    );
}

#[test]
fn the_anchor_scroll_follows_the_route_and_not_the_render_cycle() {
    let hook: String = anchor_scroll_hook();
    assert!(
        hook.contains("watch!("),
        "the anchor scroll must be driven by a watch on the route signal, so \
         it runs once when the shell mounts and afterwards only when the \
         route moves:\n{hook}"
    );
    assert!(
        !hook.contains(".subscribe("),
        "a bare subscribe runs the scroll on every re-render of the shell, \
         and the shell re-renders whenever the theme changes, so switching \
         the theme threw the reader back to the top of the page:\n{hook}"
    );
}
