/// Default localhost address for the dev-only AI Schema v1 HTTP/JSON inspector server.
pub const DEV_AI_INSPECTOR_ADDR: &str = "127.0.0.1:4174";

#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub(crate) const HTTP_PATH_SCHEMA: &str = "/schema";
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub(crate) const HTTP_PATH_SNAPSHOT: &str = "/snapshot";
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub(crate) const HTTP_PATH_QUERY: &str = "/query";
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub(crate) const HTTP_HEADER_SEP: &str = "\r\n\r\n";
