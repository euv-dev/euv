use super::*;

/// Parse the CLI arguments after `argv[0]` into an [`Args`].
///
/// Recognized flags: `-h` / `--help`, `-v` / `--version`, `--out <DIR>` /
/// `--out=<DIR>`, `--name <NAME>` / `--name=<NAME>`, `--index-html <FILE>`
/// / `--index-html=<FILE>`, `--debug`, `--release`. Positional `<SRC_DIR>`
/// is required.
///
/// # Returns
///
/// - `Result<Args, String>` - The parsed arguments, or a human-readable
///   message when the required `<SRC_DIR>` positional is missing.
pub(crate) fn parse_args() -> Result<Args, String> {
    let mut positional: Vec<PathBuf> = Vec::new();
    let mut out_dir: Option<PathBuf> = None;
    let mut name: Option<String> = None;
    let mut index_html: Option<PathBuf> = None;
    let mut locale: Option<String> = None;
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
            LOCALE_FLAG => {
                let value: String = iter
                    .next()
                    .ok_or_else(|| format!("{LOCALE_FLAG} {MSG_FLAG_REQUIRES_VALUE}"))?;
                locale = Some(value);
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
            flag if flag.starts_with(PREFIX_LOCALE) => {
                locale = Some(flag[PREFIX_LOCALE.len()..].to_string());
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
        .ok_or_else(|| MSG_MISSING_SRC_DIR.to_string())?;
    let out_dir: PathBuf = out_dir.unwrap_or_else(|| PathBuf::from(DEFAULT_OUT_DIR));
    let name: &'static str = match name {
        Some(value) => Box::leak(value.into_boxed_str()),
        None => DEFAULT_NAME,
    };
    let locale: Option<&'static str> =
        locale.map(|value: String| &*Box::leak(value.into_boxed_str()));
    Ok(Args::new(
        src_dir, out_dir, name, release, index_html, locale,
    ))
}

/// Invoke `euv build` for the supplied [`Args`], applying the env-var
/// contract that `build.rs` understands.
///
/// Per-locale bundling: with `--locale` a single bundle is compiled into
/// the output directory; without it the default locale builds first (the
/// build script writes the locale manifest into `<out>/.deploy/`), then
/// every remaining locale builds into its own sub-directory of the output
/// root (`<out>/en/`, …), one wasm bundle per locale.
///
/// # Arguments
///
/// - `&Args` - The parsed command line driving the build.
///
/// # Returns
///
/// - `Result<(), String>` - `Ok` once the site is written, or a message
///   describing which phase failed.
pub(crate) fn run(args: &Args) -> Result<(), String> {
    let user_cwd: PathBuf = env::current_dir().map_err(|e: io::Error| e.to_string())?;
    let out_dir: PathBuf = {
        let out_dir: &Path = args.get_out_dir().as_path();
        if out_dir.is_absolute() {
            out_dir.to_path_buf()
        } else {
            user_cwd.join(out_dir)
        }
    };
    if args.try_get_locale().is_some() {
        return build_one(args, &out_dir);
    }
    // Default locale first: its build writes the locale manifest the loop
    // below discovers the remaining locales from.
    build_one(args, &out_dir)?;
    for entry in read_locale_manifest(&out_dir)? {
        if entry.dir.is_empty() {
            continue;
        }
        let mut next: Args = args.clone();
        next.set_locale(Some(&*Box::leak(entry.prefix.into_boxed_str())));
        build_one(&next, &out_dir.join(entry.dir.trim_end_matches('/')))?;
    }
    Ok(())
}

/// Reads the locale manifest the first build wrote into
/// `<out>/.deploy/locales.tsv`.
///
/// # Arguments
///
/// - `&Path` - The absolute output directory of the completed first build.
///
/// # Returns
///
/// - `Result<Vec<LocaleManifestEntry>, String>` - One entry per unique
///   locale content directory, or the read/parse failure.
fn read_locale_manifest(out_dir: &Path) -> Result<Vec<LocaleManifestEntry>, String> {
    let path: PathBuf = out_dir
        .join(DEPLOY_DIR_NAME)
        .join(LOCALE_MANIFEST_FILE_NAME);
    let raw: String = fs::read_to_string(&path).map_err(|e: io::Error| {
        format!(
            "locale manifest missing at {} after the default-locale build: {e}",
            path.display()
        )
    })?;
    let mut entries: Vec<LocaleManifestEntry> = Vec::new();
    for line in raw.lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let mut columns = line.split('\t');
        let (Some(prefix), Some(dir), Some(_label)) =
            (columns.next(), columns.next(), columns.next())
        else {
            return Err(format!("malformed locale manifest row: {line}"));
        };
        entries.push(LocaleManifestEntry {
            prefix: prefix.to_string(),
            dir: dir.to_string(),
        });
    }
    Ok(entries)
}

/// Builds one locale bundle into `out_dir`.
///
/// # Arguments
///
/// - `&Args` - The parsed command line; `locale` selects the bundle.
/// - `&Path` - The absolute output directory for this bundle.
///
/// # Returns
///
/// - `Result<(), String>` - `Ok` once the bundle is written, or a message
///   describing which phase failed.
fn build_one(args: &Args, out_dir: &Path) -> Result<(), String> {
    let manifest_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src_dir: &Path = args.get_src_dir().as_path();
    if !src_dir.is_dir() {
        return Err(format!(
            "source directory does not exist: {}",
            src_dir.display()
        ));
    }
    if !src_dir.join(CONFIG_FILE_NAME).is_file() {
        // euv-docs 0.2.0+ reads site config from <SRC_DIR>/../README.md
        // frontmatter; the legacy config.toml is optional. Fall through
        // here so the build script can still try to extract config from
        // the parent README.md when no config.toml is present.
        eprintln!(
            "euv-docs: note: {} not found at {}, falling back to parent README.md frontmatter",
            CONFIG_FILE_NAME,
            src_dir.display()
        );
    }
    // Phase 1: prepare output directory and template path.
    fs::create_dir_all(out_dir).map_err(|e: io::Error| {
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
    if let Some(locale) = args.try_get_locale() {
        command.env(EUV_DOCS_LOCALE_ENV, locale);
    }
    let pkg_dir: PathBuf = out_dir.join(PKG_DIR_NAME);
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
    let status: ExitStatus = command
        .status()
        .map_err(|e: io::Error| format!("failed to invoke `{EUV_BIN}` build: {e}"))?;
    if !status.success() {
        return Err(format!("{EUV_BIN} build exited with status {status}"));
    }
    // Assets are NOT copied here: `build.rs` copies `public/` and per-doc
    // assets during the wasm build, and only for the default (site-root)
    // locale bundle — non-default bundles reference the root copies via
    // rebased `../` URLs, so every asset byte is hosted exactly once.
    // Generate a 404.html fallback (copy of index.html) so static hosts
    // like GitHub Pages serve the SPA shell for unknown paths instead
    // of returning a plain 404 — the wasm router will then resolve the
    // locale-specific route client-side.
    let index_html: PathBuf = out_dir.join(INDEX_HTML_FILE_NAME);
    let not_found_html: PathBuf = out_dir.join(NOT_FOUND_HTML_FILE_NAME);
    if index_html.is_file() && !not_found_html.exists() {
        fs::copy(&index_html, &not_found_html)
            .map_err(|e: io::Error| format!("copy 404.html: {e}"))?;
    }
    println!("euv-docs: build complete -> {}", out_dir.display());
    Ok(())
}

/// Print the CLI usage banner to stdout.
pub(crate) fn print_usage() {
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
             --locale <LOCALE>   Build a single locale bundle (prefix `/en/`, dir `en`,\n  \
                                or label); default builds every locale, one bundle\n  \
                                per locale under <OUT_DIR>/<locale>/\n  \
             --release           Use release profile (default)\n  \
             --debug             Use dev profile (faster build, slower runtime)\n  \
             -h, --help          Print this help\n  \
             -v, --version       Print version\n",
        env!("CARGO_PKG_VERSION")
    );
}
