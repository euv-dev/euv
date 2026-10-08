/// The default output subdirectory name for wasm-pack artifacts.
pub(crate) const PKG_DIR_NAME: &str = "pkg";

/// The index HTML file name.
pub(crate) const INDEX_HTML_FILE_NAME: &str = "index.html";

/// The hyphen character used when normalizing crate names to wasm-pack's
/// underscore output convention.
pub(crate) const STR_HYPHEN: &str = "-";

/// Placeholder token used in HTML templates for the JS import path.
///
/// Replaced at runtime with the resolved import path relative to the www directory.
pub(crate) const IMPORT_PATH_PLACEHOLDER: &str = "__IMPORT_PATH__";

/// Placeholder token used in HTML templates for the reload endpoint URL.
///
/// Replaced at runtime with the actual reload route path.
pub(crate) const RELOAD_ROUTE_PLACEHOLDER: &str = "__RELOAD_ROUTE__";

/// Placeholder token used in HTML templates for the inlined wasm-bindgen JS bridge.
///
/// Replaced at runtime with the body of `pkg/<name>.js` (with its top-level
/// `import` statements stripped and the 2 snippet helper functions inlined),
/// wrapped in a synchronous IIFE that fetches the wasm module and calls
/// `main()`. This eliminates the separate HTTP request for the JS bridge file
/// and skips ES module graph parsing on the critical path.
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

/// Environment variable to disable JS bridge inlining and fall back to the
/// classic `<script type="module">import init, { main } from '__IMPORT_PATH__'`
/// bootstrap. Set to any non-empty value to opt out (e.g. when the wasm-pack
/// output structure changes and the inline pipeline can't recover).
pub(crate) const EUV_NO_INLINE_BRIDGE_ENV: &str = "EUV_NO_INLINE_BRIDGE";

/// The URL path for the reload endpoint.
///
/// Used by the live-reload script in the HTML template and the server route registration.
pub(crate) const RELOAD_ROUTE: &str = "/__euv_reload";

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
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, interactive-widget=resizes-visual"
    />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta
      name="description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta
      name="keywords"
      content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv"
    />
    <meta property="og:title" content="euv" />
    <meta
      property="og:description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta property="og:type" content="website" />
    <title>Euv</title>
    __EUV_BASE_HREF_TAG__
    <style>
      * {
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-rendering: optimizeLegibility;
      }
      canvas {
        image-rendering: auto;
      }
    </style>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script>
__EUV_INLINE_JS__
  </script>
  <script>
    (function () {
      async function connect() {
        try {
          const res = await fetch('/__euv_reload');
          const data = await res.json();
          if (data.type === 'Reload') {
            location.reload();
          } else if (data.type === 'Error') {
            console.error('[euv] build error:', data.message);
            setTimeout(connect, 1000);
          } else {
            setTimeout(connect, 1000);
          }
        } catch (_) {
          setTimeout(connect, 2000);
        }
      }
      connect();
    })();
  </script>
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
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, interactive-widget=resizes-visual"
    />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta
      name="description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta
      name="keywords"
      content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv"
    />
    <meta property="og:title" content="euv" />
    <meta
      property="og:description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta property="og:type" content="website" />
    <title>Euv</title>
    __EUV_BASE_HREF_TAG__
    <style>
      * {
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-rendering: optimizeLegibility;
      }
      canvas {
        image-rendering: auto;
      }
    </style>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script>
__EUV_INLINE_JS__
  </script>
</html>
"#;

/// The euv-specific argument for specifying the server port.
pub(crate) const PORT_ARG: &str = "--port";
