use std::path::Path;

use tokio::fs::{read, read_to_string};

use super::*;

/// Reads the wasm-bindgen JS bridge produced by `wasm-pack build --target web`
/// and returns a synchronous IIFE script that inlines the bridge, inlines the
/// 2 snippet helper modules, fetches the wasm module from `wasm_url`, and
/// calls `main()` once the instance is ready.
///
/// The returned script is a drop-in replacement for the old
/// `<script type="module">import init, { main } from '__IMPORT_PATH__'; ...</script>`
/// block. Because the script is plain JavaScript (not a module) and is inlined
/// directly into the HTML, the browser incurs **zero** extra HTTP roundtrips
/// to fetch the JS bridge file and skips the ES module graph parser.
///
/// The bridge file's own top-level `import` statements (tiny snippet helpers
/// from `./snippets/<crate>-<hash>/inline{0,1}.js`) are stripped and the
/// corresponding snippet bodies are inlined at the top of the bridge so the
/// result is self-contained. Namespace imports (`import * as X from ...`)
/// are rewritten to `var X = globalThis;` so the bridge can call `X.foo(...)`
/// as it did in the original module — the hoisted function declarations from
/// the snippets are already in the IIFE scope.
pub(crate) async fn build_inline_bridge(
    pkg_dir: &Path,
    js_name: &str,
    wasm_url: &str,
) -> Result<String, EuvError> {
    let js_path: std::path::PathBuf = pkg_dir.join(js_name);
    let bridge_source: String =
        read_to_string(&js_path)
            .await
            .map_err(|error: IoError| EuvError::IoPath {
                message: String::from("Failed to read wasm-bindgen JS bridge"),
                path: js_path.clone(),
                error,
            })?;
    let snippet_bodies: String = collect_snippet_bodies(&bridge_source, pkg_dir).await?;
    let stripped: String = strip_module_imports(&bridge_source);
    let js_only: String = strip_trailing_module_exports(&stripped);
    let wasm_url_json: String = serde_json_wasm_url(wasm_url);
    Ok(format!(
        "(function() {{\n\
         {snippet_bodies}\n\
         {js_only}\n\
         var __euv_wasm_url = {wasm_url_json};\n\
         if (typeof __wbg_init === 'function') {{\n\
         var __euv_base = (document.querySelector('base[href]') && document.baseURI) || location.href;\n         __wbg_init(new URL(__euv_wasm_url, __euv_base).toString()).then(function() {{\n\
         if (typeof main === 'function') main();\n\
         }}).catch(function(e) {{ console.error('[euv] inline bridge init failed:', e); }});\n\
         }} else if (typeof initSync === 'function') {{\n\
         initSync();\n\
         if (typeof main === 'function') main();\n\
         }} else {{\n\
         console.error('[euv] inline bridge: no __wbg_init or initSync exported');\n\
         }}\n\
         }})();\n"
    ))
}

/// Returns the inline `<script>` body to use when JS bridge inlining is
/// disabled (either by env var or by absence of the `pkg/<name>.js` file).
/// This restores the classic `<script type="module">` import so the browser
/// fetches the JS bridge as a normal module — useful as a fallback during
/// wasm-pack output transitions.
pub(crate) fn build_module_fallback_bridge(import_path: &str) -> String {
    format!(
        "    import init, {{ main }} from '{import_path}';\n    \
         await init();\n    \
         main();\n"
    )
}

/// Returns `true` when the inline bridge should be skipped.
///
/// Honors the `EUV_NO_INLINE_BRIDGE` env var (any non-empty value disables
/// inlining). Other future opt-out hooks (CLI flag) can short-circuit here
/// without touching the templates.
pub(crate) fn inline_bridge_disabled() -> bool {
    matches!(std::env::var(EUV_NO_INLINE_BRIDGE_ENV), Ok(value) if !value.is_empty())
}

/// A namespace import: `import * as <alias> from "<spec>"`.
struct NamespaceImport {
    alias: String,
    spec: String,
}

async fn collect_snippet_bodies(bridge_source: &str, pkg_dir: &Path) -> Result<String, EuvError> {
    let mut bodies: String = String::new();
    let mut namespace_imports: Vec<NamespaceImport> = Vec::new();
    for line in bridge_source.lines() {
        let trimmed: &str = line.trim();
        if !trimmed.starts_with("import ") {
            continue;
        }
        let rest: &str = match trimmed.strip_prefix("import ") {
            Some(value) => value,
            None => continue,
        };
        let spec: &str = match extract_import_spec(rest) {
            Some(value) => value,
            None => continue,
        };
        if is_namespace_import(rest)
            && let Some(alias) = extract_namespace_alias(rest)
        {
            namespace_imports.push(NamespaceImport {
                alias: alias.to_string(),
                spec: spec.to_string(),
            });
        }
        let body: Option<String> = read_snippet_module(pkg_dir, spec).await?;
        if let Some(body) = body {
            if !bodies.is_empty() {
                bodies.push('\n');
            }
            bodies.push_str(&body);
        }
    }
    for ns in &namespace_imports {
        let snippet_path: std::path::PathBuf = resolve_snippet_path(pkg_dir, &ns.spec)?;
        let exports: Vec<String> = if snippet_path.exists() {
            let raw: String = read_to_string(&snippet_path)
                .await
                .map_err(|error: IoError| EuvError::IoPath {
                    message: String::from(
                        "Failed to read wasm-pack snippet module for namespace export scan",
                    ),
                    path: snippet_path.clone(),
                    error,
                })?;
            extract_exported_function_names(&raw)
        } else {
            Vec::new()
        };
        if !bodies.is_empty() {
            bodies.push('\n');
        }
        bodies.push_str(&format!("var {} = {{", ns.alias));
        for (i, name) in exports.iter().enumerate() {
            if i > 0 {
                bodies.push(',');
            }
            bodies.push_str(name);
            bodies.push(':');
            bodies.push_str(name);
        }
        bodies.push_str("};\n");
    }
    Ok(bodies)
}

/// Scans a snippet module's source for `export function NAME(`
/// declarations and returns the list of exported names. The IIFE relies on
/// function declarations being hoisted, so the names referenced in the
/// generated namespace object always resolve to the same hoisted bindings
/// that wasm-bindgen's `__wbg_get_imports` expects.
pub fn extract_exported_function_names(source: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for line in source.lines() {
        let trimmed: &str = line.trim();
        if let Some(rest) = trimmed.strip_prefix("export function ") {
            let name_end: usize =
                match rest.find(|c: char| !c.is_alphanumeric() && c != '_' && c != '$') {
                    Some(value) => value,
                    None => rest.len(),
                };
            let name: &str = &rest[..name_end];
            if !name.is_empty() {
                names.push(name.to_string());
            }
        }
    }
    names
}

pub fn is_namespace_import(rest: &str) -> bool {
    rest.trim_start().starts_with('*')
}

pub fn extract_namespace_alias(rest: &str) -> Option<&str> {
    let rest: &str = rest.trim_end_matches(';').trim();
    if !rest.starts_with('*') {
        return None;
    }
    let rest: &str = rest.strip_prefix('*')?.trim();
    let rest: &str = rest.strip_prefix("as")?.trim();
    let end: usize = match rest.find(|c: char| !c.is_alphanumeric() && c != '_' && c != '$') {
        Some(value) => value,
        None => rest.len(),
    };
    let alias: &str = &rest[..end];
    if alias.is_empty() { None } else { Some(alias) }
}

pub fn extract_import_spec(rest: &str) -> Option<&str> {
    let rest: &str = rest.trim_end_matches(';').trim();
    let from_idx: usize = rest.find(" from ")?;
    let spec_part: &str = rest[from_idx + " from ".len()..].trim();
    let spec: &str = spec_part
        .trim_start_matches('\'')
        .trim_start_matches('"')
        .trim_end_matches('\'')
        .trim_end_matches('"');
    if spec.starts_with("./") || spec.starts_with("../") {
        Some(spec)
    } else {
        None
    }
}

async fn read_snippet_module(pkg_dir: &Path, spec: &str) -> Result<Option<String>, EuvError> {
    let snippet_path: std::path::PathBuf = resolve_snippet_path(pkg_dir, spec)?;
    if !snippet_path.exists() {
        return Ok(None);
    }
    let bytes: Vec<u8> = read(&snippet_path)
        .await
        .map_err(|error: IoError| EuvError::IoPath {
            message: String::from("Failed to read wasm-pack snippet module"),
            path: snippet_path.clone(),
            error,
        })?;
    let raw: String = String::from_utf8(bytes).map_err(|error: FromUtf8Error| EuvError::Utf8 {
        message: String::from("Snippet module is not valid UTF-8"),
        error,
    })?;
    Ok(Some(strip_snippet_export(&raw)))
}

fn resolve_snippet_path(pkg_dir: &Path, spec: &str) -> Result<std::path::PathBuf, EuvError> {
    let base: &str = spec.trim_start_matches('.').trim_start_matches('/');
    let mut path: std::path::PathBuf = pkg_dir.to_path_buf();
    for segment in base.split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." {
            return Err(EuvError::Message(String::from(
                "Snippet import spec escapes pkg directory",
            )));
        }
        path.push(segment);
    }
    Ok(path)
}

fn strip_snippet_export(raw: &str) -> String {
    let mut out: String = String::with_capacity(raw.len());
    for line in raw.lines() {
        let trimmed: &str = line.trim();
        if trimmed.starts_with("export ") {
            if let Some(rest) = trimmed.strip_prefix("export ") {
                out.push_str(rest);
                out.push('\n');
            }
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn strip_module_imports(source: &str) -> String {
    let mut out: String = String::with_capacity(source.len());
    for line in source.lines() {
        let trimmed: &str = line.trim();
        if trimmed.starts_with("import ") {
            continue;
        }
        if trimmed.contains("import.meta.url") {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn strip_trailing_module_exports(source: &str) -> String {
    let mut out: String = String::with_capacity(source.len());
    for line in source.lines() {
        let trimmed: &str = line.trim();
        if trimmed.starts_with("export ") && trimmed.contains('{') && trimmed.contains('}') {
            let inner: &str = &trimmed["export ".len()..];
            let stripped: &str = match inner.strip_prefix('{') {
                Some(value) => value,
                None => {
                    out.push_str(line);
                    out.push('\n');
                    continue;
                }
            };
            let close: usize = match stripped.rfind('}') {
                Some(value) => value,
                None => {
                    out.push_str(line);
                    out.push('\n');
                    continue;
                }
            };
            let names_block: &str = &stripped[..close];
            let inlined: String = inline_export_aliases(names_block.trim());
            if !inlined.is_empty() {
                out.push_str(&inlined);
                out.push('\n');
            }
            continue;
        }
        if trimmed.starts_with("export function ") {
            let rest: &str = match trimmed.strip_prefix("export function ") {
                Some(value) => value,
                None => {
                    out.push_str(line);
                    out.push('\n');
                    continue;
                }
            };
            let name_end: usize = match rest.find(|c: char| !c.is_alphanumeric() && c != '_') {
                Some(value) => value,
                None => rest.len(),
            };
            let name: &str = &rest[..name_end];
            if !name.is_empty() {
                out.push_str("function ");
                out.push_str(name);
                out.push_str(&rest[name_end..]);
                out.push('\n');
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Converts the body of an `export { a, b as c, d as default }` clause into
/// the corresponding runtime side effects: each alias becomes
/// `<name> = <original>;` so the local function stays reachable through the
/// `as` name. Pure renames where original and exported name match are kept
/// as a no-op to keep the diff readable.
///
/// Exports of `wasm` (the `instance.exports` binding set by `__wbg_load`)
/// are silently dropped: the IIFE inlines the bridge inside a regular
/// `<script>` tag (not a module), so there is no `wasm` binding in scope
/// to expose — the `main` call in the IIFE reaches the wasm exports via
/// the `main` symbol that the user-defined Rust `#[wasm_bindgen] pub fn main`
/// exposes.
fn inline_export_aliases(block: &str) -> String {
    let mut out: String = String::new();
    for entry in block.split(',') {
        let entry: &str = entry.trim();
        if entry.is_empty() {
            continue;
        }
        if let Some((original, exported)) = entry.split_once(" as ") {
            let original: &str = original.trim();
            let exported: &str = exported.trim();
            if original == exported || exported == "default" || original == "wasm" {
                continue;
            }
            out.push_str(exported);
            out.push_str(" = ");
            out.push_str(original);
            out.push_str(";\n");
        }
    }
    out
}

fn serde_json_wasm_url(url: &str) -> String {
    let mut out: String = String::with_capacity(url.len() + 2);
    out.push('"');
    for ch in url.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
