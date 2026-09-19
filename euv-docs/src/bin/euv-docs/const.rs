/// Binary invoked to build the WASM bundle.
pub const EUV_BIN: &str = "euv";

/// CLI flag (space form) selecting the output directory.
pub const OUT_FLAG: &str = "--out";

/// Environment variable that carries the user's PATH for child processes.
pub const PATH_ENV: &str = "PATH";

/// CLI flag (space form) selecting the wasm package name.
pub const NAME_FLAG: &str = "--name";

/// CLI flag selecting the dev (non-release) wasm profile.
pub const DEBUG_FLAG: &str = "--debug";

/// Prefix marker for `--out=<DIR>` inline form; slice delimiter for the value.
pub const PREFIX_OUT: &str = "--out=";

/// Prefix marker for `--name=<NAME>` inline form; slice delimiter for the value.
pub const PREFIX_NAME: &str = "--name=";

/// Default wasm package name (used by wasm-pack as `--out-name`).
pub const DEFAULT_NAME: &str = "euv_docs";

/// Path fragment appended to the user output dir to hold wasm-pack artifacts.
pub const PKG_DIR_NAME: &str = "pkg";

/// CLI flag selecting the release wasm profile (the default).
pub const RELEASE_FLAG: &str = "--release";

/// CLI flag selecting the unoptimized wasm profile (faster build, slower runtime).
pub const EUV_DEBUG_FLAG: &str = "--debug";

/// CLI flag (long form) for `--help`.
pub const HELP_FLAG_LONG: &str = "--help";

/// Default output directory name (relative to cwd).
pub const DEFAULT_OUT_DIR: &str = "dist";

/// CLI flag (space form) selecting the index.html template path.
pub const INDEX_HTML_FLAG: &str = "--index-html";

/// Prefix marker for `--index-html=<PATH>` inline form; slice delimiter for the value.
pub const PREFIX_INDEX_HTML: &str = "--index-html=";

/// CLI flag selecting the wasm-pack web target.
pub const EUV_TARGET_FLAG: &str = "--target";

/// CLI flag (short form) for `--help`.
pub const HELP_FLAG_SHORT: &str = "-h";

/// Web target value for wasm-pack.
pub const WASM_TARGET_WEB: &str = "web";

/// Site source directory must contain this TOML config.
///
/// **Deprecated**: euv-docs 0.2.0+ reads site config from the
/// parent README.md frontmatter (`<SRC_DIR>/../README.md`). This
/// constant is retained so the CLI can emit a helpful warning when
/// neither the legacy config.toml nor the README.md frontmatter
/// is present.
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// Sub-directory of `<SRC_DIR>` that holds static assets copied
/// verbatim into the output root. Mirrors VuePress's `public/` layout.
pub const PUBLIC_DIR_NAME: &str = "public";

/// euv subcommand that triggers the actual build.
pub const EUV_BUILD_SUBCMD: &str = "build";

/// CLI flag disabling `wasm-pack pack` artifact bundling.
pub const EUV_NO_PACK_FLAG: &str = "--no-pack";

/// CLI flag selecting the wasm-pack output directory.
pub const EUV_OUT_DIR_FLAG: &str = "--out-dir";

/// CLI flag selecting the optimized wasm profile.
pub const EUV_RELEASE_FLAG: &str = "--release";

/// Argument for unknown flag starting with `-`.
pub const MSG_UNKNOWN_FLAG: &str = "unknown flag";

/// CLI flag selecting the wasm-pack package name.
pub const EUV_OUT_NAME_FLAG: &str = "--out-name";

/// CLI flag (long form) for `--version`.
pub const VERSION_FLAG_LONG: &str = "--version";

/// HTML template copied into the output root by `euv build --index-html`.
pub const TEMPLATE_FILE_NAME: &str = "template.html";

/// CLI flag (short form) for `--version`.
pub const VERSION_FLAG_SHORT: &str = "-v";

/// CLI flag passed to `euv build` to forward `template.html`.
pub const EUV_INDEX_HTML_FLAG: &str = "--index-html";

/// Positional marker that ends `euv build` flags and starts wasm-pack flags.
pub const WASM_PACK_DELIMITER: &str = "--";

/// Environment variable the build script reads for the output directory.
pub const EUV_DOCS_OUT_DIR_ENV: &str = "EUV_DOCS_OUT_DIR";

/// Environment variable the build script reads for the source directory.
pub const EUV_DOCS_SRC_DIR_ENV: &str = "EUV_DOCS_SRC_DIR";

/// CLI flag disabling `.gitignore` generation inside the wasm output dir.
pub const EUV_NO_GITIGNORE_FLAG: &str = "--no-gitignore";

/// CLI flag disabling TypeScript generation in wasm-pack.
pub const EUV_NO_TYPESCRIPT_FLAG: &str = "--no-typescript";

/// Argument missing the value for `--out <DIR>` / `--name <NAME>`.
pub const MSG_FLAG_REQUIRES_VALUE: &str = "flag requires a value";
