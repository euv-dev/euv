use super::*;

fn absent_crate_path(tag: &str) -> PathBuf {
    let candidate: PathBuf = env::temp_dir()
        .join("euv-run-mode-tests")
        .join(format!("{tag}-{}", process::id()));
    let _: io::Result<()> = fs::remove_dir_all(&candidate);
    assert!(
        !candidate.exists(),
        "the fixture path must be absent before the run, otherwise canonicalize would succeed"
    );
    candidate
}

#[tokio::test]
async fn a_crate_path_that_does_not_exist_fails_before_any_server_is_started() {
    let missing: PathBuf = absent_crate_path("absent-crate");

    let args: ModeArgs = ModeArgs::parse_from(vec![
        String::from("euv"),
        String::from("--crate-path"),
        missing.to_string_lossy().to_string(),
    ]);

    let outcome: Result<(), EuvError> = run_mode(args).await;

    match outcome {
        Err(EuvError::IoPath { path, .. }) => assert_eq!(
            path, missing,
            "the error must name the crate path that could not be resolved"
        ),
        Err(other) => panic!("a missing crate path must surface as an IoPath error, got {other:?}"),
        Ok(()) => panic!("run_mode must not report success for a crate path that does not exist"),
    }
}

#[tokio::test]
async fn a_missing_crate_path_is_rejected_without_reaching_the_build_pipeline() {
    let missing: PathBuf = absent_crate_path("absent-crate-nested");
    let nested: PathBuf = missing.join("does").join("not").join("exist");

    let args: ModeArgs = ModeArgs::parse_from(vec![
        String::from("euv"),
        String::from("--crate-path"),
        nested.to_string_lossy().to_string(),
    ]);

    let outcome: Result<(), EuvError> = run_mode(args).await;

    match outcome {
        Err(EuvError::IoPath { path, .. }) => assert_eq!(
            path, nested,
            "the reported path must be the one the caller asked for, not the deepest existing prefix"
        ),
        Err(other) => panic!("a missing crate path must surface as an IoPath error, got {other:?}"),
        Ok(()) => panic!("run_mode must not report success for a crate path that does not exist"),
    }
}

#[tokio::test]
async fn a_build_mode_crate_path_that_does_not_exist_fails_before_the_pipeline_starts() {
    let missing: PathBuf = absent_crate_path("absent-build-crate");

    let args: ModeArgs = ModeArgs::parse_from(vec![
        String::from("euv"),
        String::from("--crate-path"),
        missing.to_string_lossy().to_string(),
    ]);

    let outcome: Result<(), EuvError> = build_mode(args).await;

    match outcome {
        Err(EuvError::IoPath { path, .. }) => assert_eq!(
            path, missing,
            "build mode has to name the crate path it could not resolve, the same way run mode \
             does; a build that reports success while having built nothing is the failure this \
             error exists to prevent"
        ),
        Err(other) => panic!("a missing crate path must surface as an IoPath error, got {other:?}"),
        Ok(()) => panic!("build_mode must not report success for a crate path that does not exist"),
    }
}

#[tokio::test]
async fn a_build_mode_crate_path_nested_under_a_missing_directory_is_also_rejected() {
    let missing: PathBuf = absent_crate_path("absent-build-nested");
    let nested: PathBuf = missing.join("does").join("not").join("exist");

    let args: ModeArgs = ModeArgs::parse_from(vec![
        String::from("euv"),
        String::from("--crate-path"),
        nested.to_string_lossy().to_string(),
    ]);

    let outcome: Result<(), EuvError> = build_mode(args).await;

    assert!(
        outcome.is_err(),
        "a path whose parent does not exist cannot be canonicalized either, and it has to be \
         rejected here rather than part-way through a build that already wrote files"
    );
}
