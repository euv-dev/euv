/// The default output subdirectory name for wasm-pack artifacts.
pub(crate) const PKG_DIR_NAME: &str = "pkg";

/// The index HTML file name.
pub(crate) const INDEX_HTML_FILE_NAME: &str = "index.html";

/// The hyphen character used when normalizing crate names to wasm-pack's
/// underscore output convention.
pub(crate) const STR_HYPHEN: &str = "-";

/// Placeholder token used by legacy HTML templates for the JS import path.
///
/// It is **detected, never substituted**: a template that still carries it
/// ships a `<script type="module">` whose `import` statement would make the
/// browser fetch a `.js` file, so [`generate_html`](crate::server::generate_html)
/// replaces the whole enclosing `<script>` element with the self-contained
/// inline bootstrap. The token itself never survives into the output.
///
/// New templates must use [`INLINE_JS_PLACEHOLDER`] instead.
pub(crate) const IMPORT_PATH_PLACEHOLDER: &str = "__IMPORT_PATH__";

/// Placeholder token used in HTML templates for the reload endpoint URL.
///
/// Replaced at runtime with the actual reload route path.
pub(crate) const RELOAD_ROUTE_PLACEHOLDER: &str = "__RELOAD_ROUTE__";

/// Placeholder token used in HTML templates for the inlined wasm-bindgen JS bridge.
///
/// Replaced at runtime with the body of the wasm-pack JS bridge (with its
/// top-level `import` statements stripped and every snippet module it
/// references inlined), wrapped in a self-contained IIFE that fetches the
/// `.wasm` module and calls `main()`. The bridge `.js` file, its `.d.ts`
/// declarations, its source maps and its `snippets/` modules are deleted
/// once the HTML carrying this body has been written, so a deployed tree
/// contains zero JavaScript files.
pub(crate) const INLINE_JS_PLACEHOLDER: &str = "__EUV_INLINE_JS__";

/// Placeholder token used in HTML templates for the `<base href="..."/>` tag.
///
/// Replaced at runtime with a server-absolute path (e.g. `/www/`) so the
/// browser resolves every relative URL — the inline IIFE's wasm fetch,
/// the module fallback's `import './pkg/<name>.js'`, and any user
/// resource the HTML references — against the euv serving root rather
/// than the page URL. This makes the dev server robust to users landing
/// on `http://host:port/` instead of `http://host:port/www/`.
pub(crate) const BASE_HREF_PLACEHOLDER: &str = "__EUV_BASE_HREF_TAG__";

/// The name of the directory wasm-pack emits snippet modules into.
///
/// Removed once every snippet it holds has been inlined into the HTML and
/// deleted, so the deployed tree carries no JavaScript.
pub(crate) const SNIPPET_DIR_NAME: &str = "snippets";

/// The file-name suffixes of every JavaScript artifact wasm-pack emits.
///
/// Matched case-insensitively against the last path segment only. A file
/// whose name ends with one of these is a build product this pipeline
/// inlined into the HTML and may therefore delete; anything else in the
/// output directory is the author's and is never touched.
pub(crate) const GENERATED_JS_SUFFIXES: &[&str] = &[
    ".js", ".mjs", ".cjs", ".js.map", ".mjs.map", ".cjs.map", ".d.ts",
];

/// The keywords an `export` line may carry before the declaration it
/// applies to. Each is stripped so the declaration survives as a bare
/// statement in the inlined bootstrap.
pub(crate) const EXPORT_DECLARATION_KEYWORDS: &[&str] = &[
    "async function ",
    "function ",
    "class ",
    "const ",
    "let ",
    "var ",
];

/// The construct marking a bridge line as un-inlinable on its own.
///
/// wasm-bindgen resolves its default `.wasm` URL through `import.meta.url`,
/// which has no meaning inside the inlined `<script>`. The bootstrap passes
/// the URL explicitly instead, so a line carrying only that expression is
/// dropped rather than kept as a syntax error.
pub(crate) const IMPORT_META_URL_MARKER: &str = "import.meta.url";

/// The `<style>` block for the pre-render spinner.
///
/// The spinner mirrors the framework's own `c_spinner` class (40px ring,
/// 4px border, `accent` over `accent-muted`, `euv-spin` at 0.8s linear)
/// and reuses the same design tokens, so it is visually identical to the
/// in-app loading overlay and follows the OS colour scheme before the
/// wasm theme hook takes over. Tokens are declared locally because the
/// theme classes are injected by the wasm bundle, which has not run yet.
///
/// The accent arc spans three quarters of the ring (`border-right` and
/// `border-bottom` carry `accent` alongside `border-top`), leaving only
/// the trailing top-left quarter muted. A single-sided arc reads as a
/// short stub at 40px; three quarters reads as a ring in motion.
pub(crate) const LOADING_STYLE: &str = r#"<style>:root{--background: #ffffff;--accent: #000000;--accent-muted: #ffffff}@media(prefers-color-scheme:dark){:root{--background: #000000;--accent: #ffffff;--accent-muted: #000000}}#euv-loading{position:fixed;inset:0;display:flex;-webkit-user-select:none;user-select:none;align-items:center;justify-content:center;background:var(--background);z-index:9999}#euv-loading>div{-webkit-user-select:none;user-select:none;width:40px;height:40px;border:4px solid var(--accent-muted);border-top-color:var(--accent);border-right-color:var(--accent);border-bottom-color:var(--accent);border-radius:50%;box-sizing:border-box;animation:euv-spin .8s linear infinite}@keyframes euv-spin{0%{transform:rotate(0)}to{transform:rotate(360deg)}}@media(prefers-reduced-motion:reduce){#euv-loading>div{animation-duration:1.6s}}</style>"#;

/// The `<div>` element for the pre-render spinner, injected into `<body>`.
///
/// Text-free by design: the spinner alone communicates the pending state,
/// so the page shows no copy while the wasm module loads.
pub(crate) const LOADING_ELEMENT: &str = r#"<div id="euv-loading"><div></div></div>"#;

/// The script that removes the pre-render spinner.
///
/// Runs after the inline bridge so the spinner is torn down only once the
/// wasm module has booted and painted. A `MutationObserver` watches `#app`
/// for the framework's first child: `main()` mounts synchronously but the
/// renderer commits on the next frame, so removing the spinner on
/// `main()` alone would expose an empty viewport for one frame.
pub(crate) const LOADING_REMOVAL_SCRIPT: &str = r#"(function(){var e=document.getElementById("euv-loading");if(!e)return;var n=document.getElementById("app");function r(){e&&e.parentNode&&e.parentNode.removeChild(e)}if(!n){r();return}if(n.firstChild){r();return}var t=new MutationObserver(function(){n.firstChild&&(t.disconnect(),r())});t.observe(n,{childList:true})})();"#;

/// Placeholder token for the pre-render spinner `<style>` block.
pub(crate) const LOADING_STYLE_PLACEHOLDER: &str = "__EUV_LOADING_STYLE__";

/// Placeholder token for the pre-render spinner element.
pub(crate) const LOADING_ELEMENT_PLACEHOLDER: &str = "__EUV_LOADING_ELEMENT__";

/// Placeholder token for the script that tears down the pre-render spinner.
pub(crate) const LOADING_REMOVAL_PLACEHOLDER: &str = "__EUV_LOADING_REMOVAL__";

/// The URL path for the reload endpoint.
///
/// Used by the live-reload script in the HTML template and the server route registration.
pub(crate) const RELOAD_ROUTE: &str = "/__euv_reload";

/// The URL path for the generated index document.
///
/// hyperlane 21.12 registers routes through `#[route(...)]`, which feeds the
/// path to `HookType::Route(&'static str, ..)`, so the `format!` that used to
/// build this at runtime has to become a literal.
pub(crate) const INDEX_HTML_ROUTE: &str = "/index.html";

/// The wasm-pack flag indicating a release build.
pub(crate) const RELEASE_FLAG: &str = "--release";

/// The CLI command name for wasm-pack.
pub(crate) const WASM_PACK_COMMAND: &str = "wasm-pack";

/// The wasm-pack subcommand for building.
pub(crate) const WASM_PACK_BUILD_SUBCOMMAND: &str = "build";

/// The wasm-pack argument for specifying the output directory.
pub(crate) const OUT_DIR_ARG: &str = "--out-dir";

/// The wasm-pack argument for specifying the output name.
pub(crate) const OUT_NAME_ARG: &str = "--out-name";

/// The wasm-pack argument for specifying the target.
pub(crate) const TARGET_ARG: &str = "--target";

/// The default wasm-pack target for browser usage.
pub(crate) const TARGET_WEB: &str = "web";

/// The JavaScript file extension.
pub(crate) const JS_EXTENSION: &str = ".js";

/// The source directory name within a Cargo project.
pub(crate) const SRC_DIR_NAME: &str = "src";

/// The name of the gitignore file.
pub(crate) const GITIGNORE_FILE_NAME: &str = ".gitignore";

/// The name of the Cargo manifest file.
pub(crate) const CARGO_TOML_FILE_NAME: &str = "Cargo.toml";

/// The relative path prefix used for import path construction.
pub(crate) const RELATIVE_PATH_PREFIX: &str = "./";

/// The path separator used for joining path components.
pub(crate) const PATH_SEPARATOR: &str = "/";

/// The parent directory indicator used in relative path computation.
pub(crate) const PARENT_DIR: &str = "..";

/// The underscore character used when normalizing crate names to wasm-pack's
/// output convention.
pub(crate) const STR_UNDERSCORE: &str = "_";

/// The wasm-pack flag for development builds.
pub(crate) const DEV_FLAG: &str = "--dev";

/// The wasm-pack flag for profiling builds.
pub(crate) const PROFILING_FLAG: &str = "--profiling";

/// The euv-specific argument for specifying the crate path.
pub(crate) const CRATE_PATH_ARG: &str = "--crate-path";

/// The short form of the crate-path argument.
pub(crate) const CRATE_PATH_ARG_SHORT: &str = "-c";

/// The short form of the port argument.
pub(crate) const PORT_ARG_SHORT: &str = "-p";

/// The euv-specific argument for specifying the www directory.
pub(crate) const WWW_DIR_ARG: &str = "--www-dir";

/// The euv-specific argument for specifying a custom index.html template.
pub(crate) const INDEX_HTML_ARG: &str = "--index-html";

/// The euv-specific argument for removing the .gitignore file from the output directory.
pub(crate) const NO_GITIGNORE_ARG: &str = "--no-gitignore";

/// The euv-specific argument names that should not be forwarded to wasm-pack.
pub(crate) const EUV_ARGS: &[&str] = &[
    CRATE_PATH_ARG,
    CRATE_PATH_ARG_SHORT,
    PORT_ARG,
    PORT_ARG_SHORT,
    WWW_DIR_ARG,
    INDEX_HTML_ARG,
    NO_GITIGNORE_ARG,
];

/// The double-dash separator used to distinguish euv args from wasm-pack args.
pub(crate) const DOUBLE_DASH: &str = "--";

/// The `run` action name used in banner display.
pub(crate) const ACTION_RUN: &str = "run";

/// The `build` action name used in banner display.
pub(crate) const ACTION_BUILD: &str = "build";

/// The environment variable name for setting the minimum stack size of rustc threads.
pub(crate) const RUST_MIN_STACK_ENV: &str = "RUST_MIN_STACK";

/// The minimum stack size in bytes for rustc threads (16 MiB).
pub(crate) const RUST_MIN_STACK_VALUE: &str = "16777216";

/// The `index.html` template for the development profile.
///
/// Includes a live-reload `<script>` block that connects to the
/// `/__euv_reload` endpoint and parses the JSON payload sent by the server.
/// The JSON uses a tagged enum format:
/// - `{"type":"Reload"}` — the client should reload the page.
/// - `{"type":"Error","message":"..."}` — a build error occurred.
///
/// The `__IMPORT_PATH__` placeholder is replaced with the resolved JS import path at runtime.
pub(crate) const INDEX_HTML_DEV: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, interactive-widget=resizes-visual" />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta name="description" content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly." />
    <meta name="keywords" content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv" />
    <meta property="og:title" content="euv" />
    <meta property="og:description" content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly." />
    <meta property="og:type" content="website" />
    <title>Euv</title>__EUV_BASE_HREF_TAG__<style>* { -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale; text-rendering: optimizeLegibility; } canvas { image-rendering: auto; }</style>__EUV_LOADING_STYLE__</head>
  <body>__EUV_LOADING_ELEMENT__<div id="app"></div>
  </body>
  <script>__EUV_INLINE_JS__</script>
  <script>__EUV_LOADING_REMOVAL__</script>
  <script>(function(){async function e(){try{const t=await(await fetch("/__euv_reload")).json();t.type==="Reload"?location.reload():(t.type==="Error"&&console.error("[euv] build error:",t.message),setTimeout(e,1e3))}catch{setTimeout(e,2e3)}}e()})();</script>
</html>
"#;

/// The `index.html` template for the release profile.
///
/// A minimal `index.html` without any live-reload instrumentation.
/// Used when building for release to produce a clean, static entry point.
/// The `__IMPORT_PATH__` placeholder is replaced with the resolved JS import path at runtime.
pub(crate) const INDEX_HTML_RELEASE: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, interactive-widget=resizes-visual" />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta name="description" content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly." />
    <meta name="keywords" content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv" />
    <meta property="og:title" content="euv" />
    <meta property="og:description" content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly." />
    <meta property="og:type" content="website" />
    <title>Euv</title>__EUV_BASE_HREF_TAG__<style>* { -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale; text-rendering: optimizeLegibility; } canvas { image-rendering: auto; }</style>__EUV_LOADING_STYLE__</head>
  <body>__EUV_LOADING_ELEMENT__<div id="app"></div>
  </body>
  <script>__EUV_INLINE_JS__</script>
  <script>__EUV_LOADING_REMOVAL__</script>
</html>
"#;

/// The euv-specific argument for specifying the server port.
pub(crate) const PORT_ARG: &str = "--port";
