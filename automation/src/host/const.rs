/// ES-module MIME type for generated bindings.
pub(crate) const JS_MIME: &str = "text/javascript; charset=utf-8";

/// Builds the fixture library without the native binary.
pub(crate) const LIB_FLAG: &str = "--lib";

/// MIME type for the generated fixture document.
pub(crate) const HTML_MIME: &str = "text/html; charset=utf-8";

/// Browser module emitted by wasm-bindgen.
pub(crate) const WASM_FILE: &str = "euv_automation_bg.wasm";

/// Required browser streaming-compilation MIME type.
pub(crate) const WASM_MIME: &str = "application/wasm";

/// Owned browser-driver process log.
pub(crate) const DRIVER_LOG: &str = "chromedriver.log";

/// Static fixture asset route.
pub(crate) const FILE_ROUTE: &str = "/{path:.*}";

/// Fixture document served at the root URL.
pub(crate) const INDEX_FILE: &str = "index.html";

/// Bootstraps only the real compiled fixture and records browser errors.
pub(crate) const INDEX_HTML: &str = "<!doctype html>\n<html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>euv UI automation</title></head><body><div id=\"app\"></div><script type=\"module\">\nimport init, { mount_fixture } from './pkg/euv_automation.js';\nwindow.__EUV_AUTOMATION_ERRORS__ = [];\nwindow.addEventListener('error', event => window.__EUV_AUTOMATION_ERRORS__.push(String(event.message)));\nwindow.addEventListener('unhandledrejection', event => window.__EUV_AUTOMATION_ERRORS__.push(String(event.reason)));\ntry { await init(); mount_fixture(); } catch (error) { document.body.textContent = String(error); throw error; }\n</script></body></html>";

/// Fixture root route.
pub(crate) const ROOT_ROUTE: &str = "/";

/// Owned static-server process log.
pub(crate) const SERVER_LOG: &str = "server.log";

/// Internal mode serving only the generated fixture.
pub(crate) const SERVE_FLAG: &str = "--serve";

/// Generates browser-native ES-module bindings.
pub(crate) const WEB_TARGET: &str = "web";

/// Browser bridge generated from the actual fixture module.
pub(crate) const BRIDGE_FILE: &str = "euv_automation.js";

/// Stable browser bridge basename.
pub(crate) const BRIDGE_NAME: &str = "euv_automation";

/// Selects the Cargo metadata format.
pub(crate) const FORMAT_FLAG: &str = "--format-version";

/// Machine-readable browser run report.
pub(crate) const RESULT_FILE: &str = "report.json";

/// Selects a compiler or bridge output target.
pub(crate) const TARGET_FLAG: &str = "--target";

/// Target used for the actual browser fixture.
pub(crate) const WASM_TARGET: &str = "wasm32-unknown-unknown";

/// Header that prevents stale fixture assets.
pub(crate) const CACHE_HEADER: &str = "cache-control";

/// Suffix identifying JavaScript bridge assets.
pub(crate) const JS_EXTENSION: &str = ".js";

/// Avoids unrelated dependency detail during artifact lookup.
pub(crate) const NO_DEPS_FLAG: &str = "--no-deps";

/// Selects the generated bridge directory.
pub(crate) const OUT_DIR_FLAG: &str = "--out-dir";

/// Selects the automation workspace package.
pub(crate) const PACKAGE_FLAG: &str = "-p";

/// Workspace package containing fixture and runner.
pub(crate) const PACKAGE_NAME: &str = "euv-automation";

/// Uses the workspace release profile for fixture assets.
pub(crate) const RELEASE_FLAG: &str = "--release";

/// Checks that a prerequisite executable really runs.
pub(crate) const VERSION_FLAG: &str = "--version";

/// Cargo command that produces the actual WASM fixture.
pub(crate) const BUILD_COMMAND: &str = "build";

/// Selects the generated bridge basename.
pub(crate) const OUT_NAME_FLAG: &str = "--out-name";

/// WASM bridge and module directory.
pub(crate) const PKG_DIRECTORY: &str = "pkg";

/// Cargo-produced module before bridge generation.
pub(crate) const RAW_WASM_FILE: &str = "euv_automation.wasm";

/// Disables caching for deterministic fixture runs.
pub(crate) const CACHE_DISABLED: &str = "no-store";

/// Supported Cargo metadata format revision.
pub(crate) const FORMAT_VERSION: &str = "1";

/// Nonempty body of a missing fixture asset response.
pub(crate) const NOT_FOUND_BODY: &str = "Fixture file not found";

/// Generated static fixture directory.
pub(crate) const SITE_DIRECTORY: &str = "site";

/// Suffix identifying WebAssembly assets.
pub(crate) const WASM_EXTENSION: &str = ".wasm";

/// Captured static-file route parameter.
pub(crate) const ROUTE_PARAMETER: &str = "path";

/// Resolves the real Cargo target directory.
pub(crate) const METADATA_COMMAND: &str = "metadata";

/// Cargo release artifact directory.
pub(crate) const RELEASE_DIRECTORY: &str = "release";

/// Asks the operating system for an available loopback port.
pub(crate) const LOOPBACK_EPHEMERAL: &str = "127.0.0.1:0";

/// Response header identifying fixture content.
pub(crate) const CONTENT_TYPE_HEADER: &str = "content-type";

/// Cargo metadata member locating build artifacts.
pub(crate) const TARGET_DIRECTORY_KEY: &str = "target_directory";

/// Failure when Cargo cannot identify its artifact directory.
pub(crate) const ERROR_TARGET_DIRECTORY: &str = "Cargo metadata contains no target directory";

/// Failure when a server process is initialized more than once.
pub(crate) const ERROR_SERVER_REINITIALIZED: &str = "fixture root already initialized";
