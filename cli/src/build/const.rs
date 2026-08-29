/// Placeholder token used in HTML templates for the JS import path.
///
/// Replaced at runtime with the resolved import path relative to the www directory.
pub const IMPORT_PATH_PLACEHOLDER: &str = "__IMPORT_PATH__";

/// Placeholder token used in HTML templates for the reload endpoint URL.
///
/// Replaced at runtime with the actual reload route path.
pub const RELOAD_ROUTE_PLACEHOLDER: &str = "__RELOAD_ROUTE__";

/// The URL path for the reload endpoint.
///
/// Used by the live-reload script in the HTML template and the server route registration.
pub const RELOAD_ROUTE: &str = "/__euv_reload";

/// The wasm-pack flag indicating a release build.
pub const RELEASE_FLAG: &str = "--release";

/// The CLI command name for wasm-pack.
pub const WASM_PACK_COMMAND: &str = "wasm-pack";

/// The wasm-pack subcommand for building.
pub const WASM_PACK_BUILD_SUBCOMMAND: &str = "build";

/// The wasm-pack argument for specifying the output directory.
pub const OUT_DIR_ARG: &str = "--out-dir";

/// The wasm-pack argument for specifying the output name.
pub const OUT_NAME_ARG: &str = "--out-name";

/// The wasm-pack argument for specifying the target.
pub const TARGET_ARG: &str = "--target";

/// The default wasm-pack target for browser usage.
pub const TARGET_WEB: &str = "web";

/// The default output subdirectory name for wasm-pack artifacts.
pub const PKG_DIR_NAME: &str = "pkg";

/// The JavaScript file extension.
pub const JS_EXTENSION: &str = ".js";

/// The source directory name within a Cargo project.
pub const SRC_DIR_NAME: &str = "src";

/// The name of the gitignore file.
pub const GITIGNORE_FILE_NAME: &str = ".gitignore";

/// The name of the Cargo manifest file.
pub const CARGO_TOML_FILE_NAME: &str = "Cargo.toml";

/// The index HTML file name.
pub const INDEX_HTML_FILE_NAME: &str = "index.html";

/// The relative path prefix used for import path construction.
pub const RELATIVE_PATH_PREFIX: &str = "./";

/// The path separator used for joining path components.
pub const PATH_SEPARATOR: &str = "/";

/// The parent directory indicator used in relative path computation.
pub const PARENT_DIR: &str = "..";

/// The wasm-pack flag for development builds.
pub const DEV_FLAG: &str = "--dev";

/// The wasm-pack flag for profiling builds.
pub const PROFILING_FLAG: &str = "--profiling";

/// The euv-specific argument for specifying the crate path.
pub const CRATE_PATH_ARG: &str = "--crate-path";

/// The short form of the crate-path argument.
pub const CRATE_PATH_ARG_SHORT: &str = "-c";

/// The euv-specific argument for specifying the server port.
pub const PORT_ARG: &str = "--port";

/// The short form of the port argument.
pub const PORT_ARG_SHORT: &str = "-p";

/// The euv-specific argument for specifying the www directory.
pub const WWW_DIR_ARG: &str = "--www-dir";

/// The euv-specific argument for specifying a custom index.html template.
pub const INDEX_HTML_ARG: &str = "--index-html";

/// The euv-specific argument for removing the .gitignore file from the output directory.
pub const NO_GITIGNORE_ARG: &str = "--no-gitignore";

/// The euv-specific argument names that should not be forwarded to wasm-pack.
pub const EUV_ARGS: &[&str] = &[
    CRATE_PATH_ARG,
    CRATE_PATH_ARG_SHORT,
    PORT_ARG,
    PORT_ARG_SHORT,
    WWW_DIR_ARG,
    INDEX_HTML_ARG,
    NO_GITIGNORE_ARG,
];

/// The double-dash separator used to distinguish euv args from wasm-pack args.
pub const DOUBLE_DASH: &str = "--";

/// The `run` action name used in banner display.
pub const ACTION_RUN: &str = "run";

/// The `build` action name used in banner display.
pub const ACTION_BUILD: &str = "build";

/// The environment variable name for setting the minimum stack size of rustc threads.
pub const RUST_MIN_STACK_ENV: &str = "RUST_MIN_STACK";

/// The minimum stack size in bytes for rustc threads (16 MiB).
pub const RUST_MIN_STACK_VALUE: &str = "16777216";

/// The `index.html` template for the development profile.
///
/// Includes a live-reload `<script>` block that connects to the
/// `/__euv_reload` endpoint and parses the JSON payload sent by the server.
/// The JSON uses a tagged enum format:
/// - `{"type":"Reload"}` — the client should reload the page.
/// - `{"type":"Error","message":"..."}` — a build error occurred.
///
/// The `__IMPORT_PATH__` placeholder is replaced with the resolved JS import path at runtime.
pub const INDEX_HTML_DEV: &str = r#"<!doctype html>
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
  <script type="module">
    import init, { main } from '__IMPORT_PATH__';
    await init();
    main();
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
pub const INDEX_HTML_RELEASE: &str = r#"<!doctype html>
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
  <script type="module">
    import init, { main } from '__IMPORT_PATH__';
    await init();
    main();
  </script>
</html>
"#;
