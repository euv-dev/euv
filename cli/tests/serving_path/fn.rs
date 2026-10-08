use super::*;

fn mode_args(argv: &[&str]) -> ModeArgs {
    let mut full: Vec<String> = vec!["euv".to_string()];
    for arg in argv {
        full.push((*arg).to_string());
    }
    ModeArgs::parse_from(full)
}

fn scratch(tag: &str) -> PathBuf {
    let dir: PathBuf = env::temp_dir()
        .join("euv-serving-path-tests")
        .join(format!("{tag}-{}", process::id()));
    let _: io::Result<()> = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        let _: io::Result<()> = fs::create_dir_all(parent);
    }
    let mut file = fs::File::create(path).expect("scratch file");
    let _: io::Result<()> = io::Write::write_all(&mut file, contents.as_bytes());
}

#[test]
fn the_pkg_dir_is_wherever_the_out_dir_resolves_to() {
    let args: ModeArgs = mode_args(&[]);
    let observed: PathBuf = resolve_pkg_dir(&args);
    assert_eq!(
        observed,
        resolve_out_dir(&args),
        "resolve_pkg_dir is a thin alias over resolve_out_dir and must not drift"
    );
}

#[test]
fn the_pkg_dir_follows_an_explicit_out_dir() {
    let args: ModeArgs = mode_args(&["--crate-path", "app", "--", "--out-dir", "dist"]);
    let observed: PathBuf = resolve_pkg_dir(&args);
    assert!(
        observed.ends_with("dist"),
        "an explicit out dir wins, got {observed:?}"
    );
}

#[test]
fn the_import_path_points_at_the_bundle_from_the_serving_root() {
    let args: ModeArgs = mode_args(&["--crate-path", "app"]);
    let observed: String = resolve_import_path(&args);
    assert!(
        observed.starts_with("./"),
        "a wasm import specifier is relative, got {observed}"
    );
    assert!(
        observed.ends_with(".js"),
        "and names the generated bundle, got {observed}"
    );
}

#[test]
fn the_import_path_of_a_sibling_bundle_never_climbs_out_of_the_serving_root() {
    let args: ModeArgs = mode_args(&["--crate-path", "app", "--", "--out-dir", "dist/pkg"]);
    let observed: String = resolve_import_path(&args);
    assert!(
        !observed.contains(".."),
        "the serving root is chosen as an ancestor of the out dir, so the relative \
         specifier never has to climb out; got {observed}"
    );
    assert!(
        observed.starts_with("./"),
        "and it is still relative, got {observed}"
    );
    assert!(
        observed.ends_with(".js"),
        "and it still names the bundle, got {observed}"
    );
}

#[test]
fn a_file_directly_inside_the_base_is_served() {
    let dir: PathBuf = scratch("inside");
    let block: PathBuf = dir.join("block.js");
    write_file(&block, "export const x = 1;");
    let base: PathBuf = dir.clone();
    let observed: Option<PathBuf> = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime")
        .block_on(resolve_file_in_base(&base, "block.js"));
    match observed {
        Some(path) => assert_eq!(path, block, "a file inside the base is served"),
        None => panic!("a file inside the base must resolve"),
    }
    let _: io::Result<()> = fs::remove_dir_all(&dir);
}

#[test]
fn a_file_in_a_subdirectory_of_the_base_is_served() {
    let dir: PathBuf = scratch("subdir");
    let block: PathBuf = dir.join("nested").join("deep.js");
    write_file(&block, "export const y = 2;");
    let base: PathBuf = dir.clone();
    let observed: Option<PathBuf> = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime")
        .block_on(resolve_file_in_base(&base, "nested/deep.js"));
    assert!(
        observed.is_some(),
        "a file below the base is still inside it"
    );
    let _: io::Result<()> = fs::remove_dir_all(&dir);
}

#[test]
fn a_relative_escape_out_of_the_base_is_refused() {
    let dir: PathBuf = scratch("escape");
    let secret: PathBuf = dir.parent().expect("parent").join("euv-secret.txt");
    write_file(&secret, "classified");
    let base: PathBuf = dir.join("public");
    let _: io::Result<()> = fs::create_dir_all(&base);
    let observed: Option<PathBuf> = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime")
        .block_on(resolve_file_in_base(&base, "../euv-secret.txt"));
    assert_eq!(
        observed, None,
        "a ../ escape must never resolve, or the dev server leaks files above its root"
    );
    let _: io::Result<()> = fs::remove_file(&secret);
    let _: io::Result<()> = fs::remove_dir_all(&dir);
}

#[test]
fn a_deep_relative_escape_is_refused() {
    let dir: PathBuf = scratch("deep-escape");
    let secret: PathBuf = dir.join("secret.txt");
    write_file(&secret, "classified");
    let base: PathBuf = dir.join("a").join("b");
    let _: io::Result<()> = fs::create_dir_all(&base);
    let observed: Option<PathBuf> = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime")
        .block_on(resolve_file_in_base(&base, "../../secret.txt"));
    assert_eq!(
        observed, None,
        "a multi-level climb out of the base is refused too"
    );
    let _: io::Result<()> = fs::remove_dir_all(&dir);
}

#[test]
fn a_missing_file_is_not_served() {
    let dir: PathBuf = scratch("missing");
    let base: PathBuf = dir.clone();
    let observed: Option<PathBuf> = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime")
        .block_on(resolve_file_in_base(&base, "nothing-here.js"));
    assert_eq!(
        observed, None,
        "a path that does not exist resolves to nothing"
    );
    let _: io::Result<()> = fs::remove_dir_all(&dir);
}

#[test]
fn an_absolute_request_path_cannot_escape_the_base() {
    let dir: PathBuf = scratch("absolute");
    let secret: PathBuf = dir.join("secret.txt");
    write_file(&secret, "classified");
    let base: PathBuf = dir.join("public");
    let _: io::Result<()> = fs::create_dir_all(&base);
    let requested: String = secret.to_string_lossy().to_string();
    let observed: Option<PathBuf> = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime")
        .block_on(resolve_file_in_base(&base, &requested));
    assert_eq!(
        observed, None,
        "an absolute path outside the base is refused even though it exists"
    );
    let _: io::Result<()> = fs::remove_dir_all(&dir);
}

#[test]
fn a_dot_segment_request_still_resolves_inside_the_base() {
    let dir: PathBuf = scratch("dot-segment");
    let block: PathBuf = dir.join("block.js");
    write_file(&block, "export const z = 3;");
    let base: PathBuf = dir.clone();
    let observed: Option<PathBuf> = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime")
        .block_on(resolve_file_in_base(&base, "./block.js"));
    assert!(
        observed.is_some(),
        "a leading ./ stays inside the base and must not be treated as an escape"
    );
    let _: io::Result<()> = fs::remove_dir_all(&dir);
}
