use super::*;

fn outside_file(name: &str, file: &str) -> PathBuf {
    let root: PathBuf = temp_dir().join(name);
    create_dir_all(&root).expect("outside dir");
    let target: PathBuf = root.join(file);
    write(&target, "outside the served root").expect("outside file");
    target
}

fn scratch_root(name: &str) -> PathBuf {
    let root: PathBuf = temp_dir().join(format!("euv-cli-server-{name}"));
    let _ = remove_dir_all(&root);
    create_dir_all(root.join("assets")).expect("scratch root");
    create_dir_all(root.join("assets/nested")).expect("scratch assets");
    write(root.join("assets/index.html"), "<html></html>").expect("scratch file");
    write(root.join("assets/nested/deep.txt"), "deep").expect("scratch file");
    root
}

#[tokio::test]
async fn a_file_inside_the_base_directory_resolves() {
    let root: PathBuf = scratch_root("inside");
    let resolved: Option<PathBuf> = resolve_file_in_base(&root, "assets/index.html").await;
    assert_eq!(resolved, Some(root.join("assets/index.html")));
}

#[tokio::test]
async fn a_nested_file_inside_the_base_directory_resolves() {
    let root: PathBuf = scratch_root("nested");
    let resolved: Option<PathBuf> = resolve_file_in_base(&root, "assets/nested/deep.txt").await;
    assert_eq!(resolved, Some(root.join("assets/nested/deep.txt")));
}

#[tokio::test]
async fn a_path_that_climbs_one_level_out_of_the_base_is_refused() {
    let victim: PathBuf = outside_file("euv-cli-server-victim-one", "secret.txt");
    let root: PathBuf = scratch_root("climb-one");
    let resolved: Option<PathBuf> = resolve_file_in_base(&root, "../euv-cli-server-victim-one/secret.txt").await;
    assert!(
        resolved.is_none(),
        "the file really exists, so only the containment check can refuse it; got {resolved:?} \
         for victim {victim:?}"
    );
}

#[tokio::test]
async fn a_path_that_climbs_two_levels_out_of_the_base_is_refused() {
    let _victim: PathBuf = outside_file("euv-cli-server-victim-two", "secret.txt");
    let root: PathBuf = scratch_root("climb-two");
    let resolved: Option<PathBuf> =
        resolve_file_in_base(&root, "assets/../../euv-cli-server-victim-two/secret.txt").await;
    assert!(
        resolved.is_none(),
        "a climb that passes back through the base's own subtree is still an escape; got {resolved:?}"
    );
}

#[tokio::test]
async fn a_sibling_directory_whose_name_starts_with_the_base_is_refused() {
    let _victim: PathBuf = outside_file("euv-cli-server-sibling-evil", "loot.txt");
    let root: PathBuf = scratch_root("sibling");
    let resolved: Option<PathBuf> =
        resolve_file_in_base(&root, "../euv-cli-server-sibling-evil/loot.txt").await;
    assert!(
        resolved.is_none(),
        "a string prefix match is not containment; got {resolved:?}"
    );
}

#[tokio::test]
async fn a_path_that_does_not_exist_is_refused() {
    let root: PathBuf = scratch_root("missing");
    let resolved: Option<PathBuf> = resolve_file_in_base(&root, "assets/nope.html").await;
    assert!(resolved.is_none(), "got {resolved:?}");
}

#[tokio::test]
async fn a_directory_itself_resolves_because_it_exists() {
    let root: PathBuf = scratch_root("dir");
    let resolved: Option<PathBuf> = resolve_file_in_base(&root, "assets").await;
    assert_eq!(
        resolved,
        Some(root.join("assets")),
        "this guard checks containment, not that the target is a regular file"
    );
}
