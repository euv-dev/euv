/// Binary invoked to build the WASM bundle.
pub(crate) const EUV_BIN: &str = "euv";

/// CLI flag (space form) selecting the output directory.
pub(crate) const OUT_FLAG: &str = "--out";

/// Environment variable that carries the user's PATH for child processes.
pub(crate) const PATH_ENV: &str = "PATH";

/// CLI flag (space form) selecting the wasm package name.
pub(crate) const NAME_FLAG: &str = "--name";

/// CLI flag selecting the dev (non-release) wasm profile.
pub(crate) const DEBUG_FLAG: &str = "--debug";

/// Prefix marker for `--out=<DIR>` inline form; slice delimiter for the value.
pub(crate) const PREFIX_OUT: &str = "--out=";

/// Prefix marker for `--name=<NAME>` inline form; slice delimiter for the value.
pub(crate) const PREFIX_NAME: &str = "--name=";

/// Default wasm package name (used by wasm-pack as `--out-name`).
pub(crate) const DEFAULT_NAME: &str = "euv_docs";

/// Path fragment appended to the user output dir to hold wasm-pack artifacts.
pub(crate) const PKG_DIR_NAME: &str = "pkg";

/// CLI flag selecting the release wasm profile (the default).
pub(crate) const RELEASE_FLAG: &str = "--release";

/// CLI flag selecting the unoptimized wasm profile (faster build, slower runtime).
pub(crate) const EUV_DEBUG_FLAG: &str = "--debug";

/// CLI flag (long form) for `--help`.
pub(crate) const HELP_FLAG_LONG: &str = "--help";

/// Default output directory name (relative to cwd).
pub(crate) const DEFAULT_OUT_DIR: &str = "dist";

/// CLI flag (space form) selecting the index.html template path.
pub(crate) const INDEX_HTML_FLAG: &str = "--index-html";

/// Prefix marker for `--index-html=<PATH>` inline form; slice delimiter for the value.
pub(crate) const PREFIX_INDEX_HTML: &str = "--index-html=";

/// CLI flag selecting the wasm-pack web target.
pub(crate) const EUV_TARGET_FLAG: &str = "--target";

/// CLI flag (short form) for `--help`.
pub(crate) const HELP_FLAG_SHORT: &str = "-h";

/// Web target value for wasm-pack.
pub(crate) const WASM_TARGET_WEB: &str = "web";

/// Site source directory must contain this TOML config.
///
/// **Deprecated**: euv-docs 0.2.0+ reads site config from the
/// parent README.md frontmatter (`<SRC_DIR>/../README.md`). This
/// constant is retained so the CLI can emit a helpful warning when
/// neither the legacy config.toml nor the README.md frontmatter
/// is present.
pub(crate) const CONFIG_FILE_NAME: &str = "config.toml";

/// euv subcommand that triggers the actual build.
pub(crate) const EUV_BUILD_SUBCMD: &str = "build";

/// CLI flag disabling `wasm-pack pack` artifact bundling.
pub(crate) const EUV_NO_PACK_FLAG: &str = "--no-pack";

/// CLI flag selecting the wasm-pack output directory.
pub(crate) const EUV_OUT_DIR_FLAG: &str = "--out-dir";

/// CLI flag selecting the optimized wasm profile.
pub(crate) const EUV_RELEASE_FLAG: &str = "--release";

/// Argument for unknown flag starting with `-`.
pub(crate) const MSG_UNKNOWN_FLAG: &str = "unknown flag";

/// CLI flag selecting the wasm-pack package name.
pub(crate) const EUV_OUT_NAME_FLAG: &str = "--out-name";

/// CLI flag (long form) for `--version`.
pub(crate) const VERSION_FLAG_LONG: &str = "--version";

/// HTML template copied into the output root by `euv build --index-html`.
pub(crate) const TEMPLATE_FILE_NAME: &str = "template.html";

/// CLI flag (short form) for `--version`.
pub(crate) const VERSION_FLAG_SHORT: &str = "-v";

/// CLI flag passed to `euv build` to forward `template.html`.
pub(crate) const EUV_INDEX_HTML_FLAG: &str = "--index-html";

/// Positional marker that ends `euv build` flags and starts wasm-pack flags.
pub(crate) const WASM_PACK_DELIMITER: &str = "--";

/// Environment variable the build script reads for the output directory.
pub(crate) const EUV_DOCS_OUT_DIR_ENV: &str = "EUV_DOCS_OUT_DIR";

/// Environment variable the build script reads for the source directory.
pub(crate) const EUV_DOCS_SRC_DIR_ENV: &str = "EUV_DOCS_SRC_DIR";

/// CLI flag disabling `.gitignore` generation inside the wasm output dir.
pub(crate) const EUV_NO_GITIGNORE_FLAG: &str = "--no-gitignore";

/// CLI flag disabling TypeScript generation in wasm-pack.
pub(crate) const EUV_NO_TYPESCRIPT_FLAG: &str = "--no-typescript";

/// CLI flag (space form) building a single locale bundle.
pub(crate) const LOCALE_FLAG: &str = "--locale";

/// Prefix marker for `--locale=<NAME>` inline form; slice delimiter for the value.
pub(crate) const PREFIX_LOCALE: &str = "--locale=";

/// Environment variable the build script reads for the pinned locale.
pub(crate) const EUV_DOCS_LOCALE_ENV: &str = "EUV_DOCS_LOCALE";

/// Hidden deployment-metadata directory inside the site output where the
/// build script writes the locale manifest.
pub(crate) const DEPLOY_DIR_NAME: &str = ".deploy";

/// Locale manifest file the first build writes into `.deploy/`; the CLI
/// reads it to discover the remaining locales to build.
pub(crate) const LOCALE_MANIFEST_FILE_NAME: &str = "locales.tsv";

/// Argument missing the value for `--out <DIR>` / `--name <NAME>`.
pub(crate) const MSG_FLAG_REQUIRES_VALUE: &str = "flag requires a value";

/// Error when the required `<SRC_DIR>` positional argument is absent.
pub(crate) const MSG_MISSING_SRC_DIR: &str = "missing required <SRC_DIR> argument";

/// The SPA shell `euv build` writes into the output root; also copied
/// to `404.html` below so static hosts serve the app for unknown paths.
pub(crate) const INDEX_HTML_FILE_NAME: &str = "index.html";

/// The static-host 404 fallback, a copy of the SPA shell.
pub(crate) const NOT_FOUND_HTML_FILE_NAME: &str = "404.html";
