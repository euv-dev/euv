use super::*;

#[test]
fn cargo_dependency_groups_expand_real_ui_components() -> Result<(), Box<dyn Error>> {
    let repository: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()?;
    let fixture: PathBuf =
        env::temp_dir().join(format!("euv-dependency-discovery-{}", process::id()));
    create_dir_all(fixture.join("src"))?;
    let fixture: PathBuf = fixture.canonicalize()?;
    let ui_path: PathBuf = relative_path(&fixture, &repository.join("ui"));
    let members: Vec<&str> = DEPENDENCY_CASES
        .iter()
        .map(|(name, _, _): &(&str, &str, bool)| *name)
        .collect();
    let root_manifest: String = format!(
        "[package]\nname = \"discovery_root\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[workspace]\nmembers = {members:?}\nresolver = \"3\"\n\n[workspace.dependencies]\neuv-ui = {{ path = {ui_path:?} }}\n\n[dependencies]\neuv = {{ path = {repository:?} }}\neuv-ui = {{ workspace = true }}\n"
    );
    write(fixture.join("Cargo.toml"), root_manifest)?;
    write(fixture.join("src/lib.rs"), CONSUMER_SOURCE)?;
    for (name, section, inherited) in DEPENDENCY_CASES {
        let member: PathBuf = fixture.join(name);
        create_dir_all(member.join("src"))?;
        let ui_dependency: String = if inherited {
            "{ workspace = true }".to_string()
        } else {
            format!(
                "{{ path = {:?} }}",
                relative_path(&member, &repository.join("ui"))
            )
        };
        let dependency_header: String = if section == "dependencies" {
            String::new()
        } else {
            format!("\n[{section}]\n")
        };
        let manifest: String = format!(
            "[package]\nname = \"discovery_{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\neuv = {{ path = {repository:?} }}\n{dependency_header}euv-ui = {ui_dependency}\n"
        );
        write(member.join("Cargo.toml"), manifest)?;
        write(member.join("src/lib.rs"), CONSUMER_SOURCE)?;
    }
    let output: Output = Command::new(env!("CARGO"))
        .args(["test", "--offline", "--workspace", "--lib"])
        .arg("--manifest-path")
        .arg(fixture.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(env::temp_dir().join("euv-dependency-discovery-target"))
        .output()?;
    assert!(
        output.status.success(),
        "dependency fixture failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout)
            .matches("test dependency_component_is_a_real_button ... ok")
            .count(),
        DEPENDENCY_CASES.len() + 1
    );
    remove_dir_all(&fixture)?;
    Ok(())
}

fn relative_path(base: &Path, destination: &Path) -> PathBuf {
    let base_components: Vec<Component<'_>> = base.components().collect();
    let destination_components: Vec<Component<'_>> = destination.components().collect();
    let shared: usize = base_components
        .iter()
        .zip(&destination_components)
        .take_while(|(left, right): &(&Component<'_>, &Component<'_>)| left == right)
        .count();
    let mut relative: PathBuf = PathBuf::new();
    for _ in shared..base_components.len() {
        relative.push("..");
    }
    for component in &destination_components[shared..] {
        relative.push(component.as_os_str());
    }
    relative
}
