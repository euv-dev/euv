use super::*;

/// Parse the CLI arguments after `argv[0]` into an [`Args`].
///
/// Recognized flags: `-h` / `--help`, `-v` / `--version`, `--out <DIR>` /
/// `--out=<DIR>`, `--name <NAME>` / `--name=<NAME>`, `--index-html <FILE>`
/// / `--index-html=<FILE>`, `--debug`, `--release`. Positional `<SRC_DIR>`
/// is required.
pub fn parse_args() -> Result<Args, String> {
    let mut positional: Vec<PathBuf> = Vec::new();
    let mut out_dir: Option<PathBuf> = None;
    let mut name: Option<String> = None;
    let mut index_html: Option<PathBuf> = None;
    let mut release: bool = true;
    let mut iter: Skip<env::Args> = env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            HELP_FLAG_SHORT | HELP_FLAG_LONG => {
                print_usage();
                exit(0);
            }
            VERSION_FLAG_SHORT | VERSION_FLAG_LONG => {
                println!("euv-docs {}", env!("CARGO_PKG_VERSION"));
                exit(0);
            }
            OUT_FLAG => {
                let value: String = iter
                    .next()
                    .ok_or_else(|| format!("{OUT_FLAG} {MSG_FLAG_REQUIRES_VALUE}"))?;
                out_dir = Some(PathBuf::from(value));
            }
            NAME_FLAG => {
                let value: String = iter
                    .next()
                    .ok_or_else(|| format!("{NAME_FLAG} {MSG_FLAG_REQUIRES_VALUE}"))?;
                name = Some(value);
            }
            INDEX_HTML_FLAG => {
                let value: String = iter
                    .next()
                    .ok_or_else(|| format!("{INDEX_HTML_FLAG} {MSG_FLAG_REQUIRES_VALUE}"))?;
                index_html = Some(PathBuf::from(value));
            }
            DEBUG_FLAG => {
                release = false;
            }
            RELEASE_FLAG => {
                release = true;
            }
            flag if flag.starts_with(PREFIX_OUT) => {
                out_dir = Some(PathBuf::from(&flag[PREFIX_OUT.len()..]));
            }
            flag if flag.starts_with(PREFIX_NAME) => {
                name = Some(flag[PREFIX_NAME.len()..].to_string());
            }
            flag if flag.starts_with(PREFIX_INDEX_HTML) => {
                index_html = Some(PathBuf::from(&flag[PREFIX_INDEX_HTML.len()..]));
            }
            flag if flag.starts_with('-') => {
                return Err(format!("{MSG_UNKNOWN_FLAG}: {flag}"));
            }
            _ => {
                positional.push(PathBuf::from(arg));
            }
        }
    }
    let src_dir: PathBuf = positional
        .into_iter()
        .next()
        .ok_or_else(|| "missing required <SRC_DIR> argument".to_string())?;
    let out_dir: PathBuf = out_dir.unwrap_or_else(|| PathBuf::from(DEFAULT_OUT_DIR));
    let name: &'static str = match name {
        Some(value) => Box::leak(value.into_boxed_str()),
        None => DEFAULT_NAME,
    };
    Ok(Args::new(src_dir, out_dir, name, release, index_html))
}

/// Invoke `euv build` for the supplied [`Args`], applying the env-var
/// contract that `build.rs` understands.
pub fn run(args: &Args) -> Result<(), String> {
    let manifest_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src_dir: &Path = args.get_src_dir().as_path();
    if !src_dir.is_dir() {
        return Err(format!(
            "source directory does not exist: {}",
            src_dir.display()
        ));
    }
    if !src_dir.join(CONFIG_FILE_NAME).is_file() {
        return Err(format!(
            "source directory is missing {}: {}",
            CONFIG_FILE_NAME,
            src_dir.display()
        ));
    }
    // Phase 1: prepare output directory and template path.
    let out_dir: &Path = args.get_out_dir().as_path();
    std::fs::create_dir_all(out_dir).map_err(|e| {
        format!(
            "failed to create output directory {}: {e}",
            out_dir.display()
        )
    })?;
    let user_template: Option<&PathBuf> = args.try_get_index_html().as_ref();
    let user_cwd: PathBuf = env::current_dir().unwrap_or_else(|_| manifest_dir.clone());
    let template_path: PathBuf = match user_template {
        Some(path) if path.is_absolute() => path.clone(),
        Some(path) => user_cwd.join(path),
        None => manifest_dir.join(TEMPLATE_FILE_NAME),
    };
    // Phase 2: build the `euv` invocation (euv build ... -- --target web ...).
    let mut command: Command = Command::new(EUV_BIN);
    if let Some(path) = env::var_os(PATH_ENV) {
        command.env(PATH_ENV, path);
    }
    command.current_dir(&manifest_dir);
    command
        .arg(EUV_BUILD_SUBCMD)
        .arg(if *args.get_release() {
            EUV_RELEASE_FLAG
        } else {
            EUV_DEBUG_FLAG
        })
        .arg(EUV_INDEX_HTML_FLAG)
        .arg(&template_path);
    command.env(EUV_DOCS_SRC_DIR_ENV, src_dir);
    command.env(EUV_DOCS_OUT_DIR_ENV, out_dir);
    let user_cwd: PathBuf = env::current_dir().unwrap_or_else(|_| manifest_dir.clone());
    let resolved_out_dir: PathBuf = if out_dir.is_absolute() {
        out_dir.to_path_buf()
    } else {
        user_cwd.join(out_dir)
    };
    let pkg_dir: PathBuf = resolved_out_dir.join(PKG_DIR_NAME);
    command.arg(WASM_PACK_DELIMITER);
    command
        .arg(EUV_TARGET_FLAG)
        .arg(WASM_TARGET_WEB)
        .arg(EUV_OUT_DIR_FLAG)
        .arg(&pkg_dir)
        .arg(EUV_OUT_NAME_FLAG)
        .arg(*args.get_name())
        .arg(EUV_NO_TYPESCRIPT_FLAG)
        .arg(EUV_NO_PACK_FLAG)
        .arg(EUV_NO_GITIGNORE_FLAG);
    // Phase 3: invoke and stream progress.
    println!(
        "euv-docs: building {} -> {}",
        src_dir.display(),
        out_dir.display()
    );
    let status: std::process::ExitStatus = command
        .status()
        .map_err(|e| format!("failed to invoke `{EUV_BIN}` build: {e}"))?;
    if !status.success() {
        return Err(format!("{EUV_BIN} build exited with status {status}"));
    }
    // Phase 4: copy `<src_dir>/public/` into the output root. `euv build`
    // only generates `index.html` + `pkg/*`; static assets that the site
    // references under `/foo.png` etc. live in the user's source tree
    // under `public/` and need to be copied into `<out_dir>/` ourselves.
    copy_public_assets(src_dir, out_dir)?;
    println!("euv-docs: build complete -> {}", out_dir.display());
    Ok(())
}

/// Recursively copy `<src_dir>/public/` into `<out_dir>/`. Missing
/// `public/` is treated as success (the source may have no static
/// assets); per-file copy errors are returned verbatim so the user
/// sees them.
fn copy_public_assets(src_dir: &Path, out_dir: &Path) -> Result<(), String> {
    let public_dir: PathBuf = src_dir.join(PUBLIC_DIR_NAME);
    if !public_dir.is_dir() {
        return Ok(());
    }
    copy_dir_recursive(&public_dir, out_dir)
}

/// Walk `src` and copy every entry under it to the matching relative
/// path under `dst`, creating intermediate directories as needed.
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    let entries: Vec<std::fs::DirEntry> = std::fs::read_dir(src)
        .map_err(|e| format!("read_dir({}): {e}", src.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("read_dir({}): {e}", src.display()))?;
    for entry in entries {
        let entry_path: PathBuf = entry.path();
        let file_name: std::ffi::OsString = entry.file_name();
        let target_path: PathBuf = dst.join(&file_name);
        let file_type: std::fs::FileType = entry
            .file_type()
            .map_err(|e| format!("file_type({}): {e}", entry_path.display()))?;
        if file_type.is_dir() {
            std::fs::create_dir_all(&target_path)
                .map_err(|e| format!("create_dir_all({}): {e}", target_path.display()))?;
            copy_dir_recursive(&entry_path, &target_path)?;
        } else if file_type.is_file() {
            std::fs::copy(&entry_path, &target_path).map_err(|e| {
                format!(
                    "copy {} -> {}: {e}",
                    entry_path.display(),
                    target_path.display()
                )
            })?;
        }
    }
    Ok(())
}

/// Print the CLI usage banner to stdout.
pub fn print_usage() {
    println!(
        "euv-docs {} — build a markdown directory into a static site.\n\n\
         USAGE:\n  \
             euv-docs <SRC_DIR> [--out <OUT_DIR>] [--name <NAME>] [--index-html <FILE>] [--debug]\n\n\
         ARGS:\n  \
             <SRC_DIR>    Directory containing *.md files (site config\n                          is read from the parent README.md frontmatter)\n\n\
         OPTIONS:\n  \
             --out <DIR>         Output directory (default: ./dist)\n  \
             --name <NAME>       Wasm package name (default: euv_docs)\n  \
             --index-html <FILE> Custom index.html template (default: CLI-bundled)\n  \
             --release           Use release profile (default)\n  \
             --debug             Use dev profile (faster build, slower runtime)\n  \
             -h, --help          Print this help\n  \
             -v, --version       Print version\n",
        env!("CARGO_PKG_VERSION")
    );
}
