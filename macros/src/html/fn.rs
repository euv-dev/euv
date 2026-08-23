use super::*;

/// Parses a Rust expression from the parse stream, stopping before a top-level brace.
///
/// This is used for inline `if` conditions, `match` scrutinees, and `for` iterables
/// where a plain identifier followed by `{` would otherwise be misinterpreted as a
/// struct literal expression (e.g., `if has_subtitle { ... }` would try to parse
/// `has_subtitle { ... }` as `ExprStruct`).
///
/// Tokens are collected until a top-level `Brace` delimiter is encountered, then
/// parsed as an `Expr`. Nested groups (parens, brackets) are consumed as single
/// `TokenTree` units, so braces inside them do not terminate the collection.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream positioned at the start of the expression.
///
/// # Returns
///
/// - `syn::Result<Expr>` - The parsed expression, or a syntax error.
pub(crate) fn parse_expr_until_brace(input: ParseStream) -> syn::Result<Expr> {
    let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    while !input.peek(Brace) {
        let token_tree: proc_macro2::TokenTree = input.parse()?;
        tokens.extend([token_tree]);
    }
    syn::parse2(tokens)
}

/// Determines whether the macro should automatically append `.get()` to an expression
/// inside a reactive `{}` position (e.g. `if { signal }`, `match { signal }`,
/// `for x in { signal }`, `{ signal }` DOM child).
///
/// Returns `true` only for plain single-segment identifier paths such as `signal`
/// or `value`. Chained expressions like `state.field`, `signal.iter()`, or
/// `signal == X` are NOT auto-unwrapped — the user must call `.get()` explicitly
/// in those cases (e.g., `state.field.get()`, `signal.get().iter()`,
/// `signal.get() == X`).
///
/// This is a conservative heuristic: the macro only adds `.get()` where the
/// expression is unambiguously a single identifier, which is the most common
/// case for signal references. Other expression kinds pass through unchanged
/// so that non-signal expressions (literals, function calls, etc.) keep working.
///
/// # Arguments
///
/// - `&Expr` - The expression to inspect.
///
/// # Returns
///
/// - `bool` - `true` if the expression is a single-segment path suitable for
///   automatic `.get()` unwrapping.
pub(crate) fn should_auto_get(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Path(expr_path)
            if expr_path.qself.is_none()
                && expr_path.path.leading_colon.is_none()
                && expr_path.path.segments.len() == 1
                && matches!(
                    expr_path.path.segments[0].arguments,
                    syn::PathArguments::None
                )
    )
}

/// Wraps an expression with `.get()` when it is a single-segment identifier path,
/// otherwise returns the expression unchanged.
///
/// When `enabled` is `false`, the expression is returned as-is (used for inline
/// `if`/`match`/`for` whose conditions were NOT written inside `{}`).
///
/// # Arguments
///
/// - `&Expr` - The expression to optionally unwrap.
/// - `bool` - Whether the wrapping should be applied.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The wrapped or unchanged expression tokens.
pub(crate) fn auto_get_expr_tokens(expr: &Expr, enabled: bool) -> proc_macro2::TokenStream {
    if enabled && should_auto_get(expr) {
        quote! { #expr.get() }
    } else {
        quote! { #expr }
    }
}

/// Checks whether the next tokens after the current position form a `::` path separator.
///
/// This is used to distinguish between a single `:` (attribute key-value separator)
/// and `::` (Rust path separator like `Enum::Variant`). When an `Ident` is followed
/// by `::`, it should be treated as the start of a path expression rather than an
/// attribute key.
///
/// # Arguments
///
/// - `&ParseStream` - The parse stream to check.
///
/// # Returns
///
/// - `bool` - `true` if the next two tokens after the current position are `::`.
pub(crate) fn is_double_colon(content: ParseStream) -> bool {
    let forked: ParseBuffer<'_> = content.fork();
    let _: Ident = match forked.parse() {
        Ok(ident) => ident,
        Err(_) => return false,
    };
    forked.peek(Token![::])
}

/// Sets the user-defined component registry for the current thread.
///
/// # Arguments
///
/// - `HashMap<String, ComponentInfo>` - The map of function names to component metadata.
pub(crate) fn set_user_fn_names(names: HashMap<String, ComponentInfo>) {
    unsafe {
        let pointer: *mut MaybeUninit<HashMap<String, ComponentInfo>> = &raw mut USER_FN_NAMES;
        (*pointer).write(names);
    }
}

/// Returns the already-loaded component registry without re-scanning the file system.
///
/// This is used by `HtmlDynamicTag::to_tokens` to avoid calling `load_component_registry`
/// again, since the registry has already been populated by `parse_html` before
/// token generation begins.
///
/// # Returns
///
/// - `HashMap<String, ComponentInfo>` - The loaded component registry.
pub(crate) fn get_loaded_component_registry() -> HashMap<String, ComponentInfo> {
    unsafe {
        let pointer: *const MaybeUninit<HashMap<String, ComponentInfo>> = &raw const USER_FN_NAMES;
        (*pointer).assume_init_ref().clone()
    }
}

/// Checks whether a given name corresponds to a user-defined component function.
///
/// # Arguments
///
/// - `&str` - The name to check against the stored component registry.
///
/// # Returns
///
/// - `bool` - `true` if the name exists in the component registry, `false` otherwise.
pub(crate) fn is_user_fn(name: &str) -> bool {
    unsafe {
        let pointer: *const MaybeUninit<HashMap<String, ComponentInfo>> = &raw const USER_FN_NAMES;
        (*pointer).assume_init_ref().contains_key(name)
    }
}

/// Returns the Props type name for a given component function name.
///
/// # Arguments
///
/// - `&str` - The component function name.
///
/// # Returns
///
/// - `Option<&'static str>` - The Props type name if found.
pub(crate) fn get_user_fn_props_type(name: &str) -> Option<&'static str> {
    unsafe {
        let pointer: *const MaybeUninit<HashMap<String, ComponentInfo>> = &raw const USER_FN_NAMES;
        (*pointer)
            .assume_init_ref()
            .get(name)
            .map(|info: &ComponentInfo| info.get_props_type().as_str())
    }
}

/// Returns the props field names for a given component function name.
///
/// Used to determine whether a standalone identifier inside a component body
/// should be treated as an attribute shorthand (e.g., `panel_open` → `panel_open: panel_open`).
///
/// # Arguments
///
/// - `&str` - The component function name.
///
/// # Returns
///
/// - `Option<&'static Vec<String>>` - The list of props field names if the component is found.
pub(crate) fn get_user_fn_props_fields(name: &str) -> Option<&'static Vec<String>> {
    unsafe {
        let pointer: *const MaybeUninit<HashMap<String, ComponentInfo>> = &raw const USER_FN_NAMES;
        (*pointer)
            .assume_init_ref()
            .get(name)
            .map(|info: &ComponentInfo| info.get_props_fields())
    }
}

/// Returns the props field type map for a given component function name.
///
/// Maps field name → type string (e.g., `"children"` → `"VirtualNode"`).
///
/// # Arguments
///
/// - `&str` - The component function name.
///
/// # Returns
///
/// - `Option<&'static HashMap<String, String>>` - The field type map if the component is found.
pub(crate) fn get_user_fn_props_field_types(
    name: &str,
) -> Option<&'static HashMap<String, String>> {
    unsafe {
        let pointer: *const MaybeUninit<HashMap<String, ComponentInfo>> = &raw const USER_FN_NAMES;
        (*pointer)
            .assume_init_ref()
            .get(name)
            .map(|info: &ComponentInfo| info.get_props_field_types())
    }
}

/// Parses the input tokens into a euv VNode expression.
///
/// Supports zero, one, or multiple root-level HTML nodes:
/// - `html! {}` → `VirtualNode::Empty`
/// - `html! { div { ... } }` → single `VirtualNode`
/// - `html! { div { ... } span { ... } }` → `VirtualNode::Fragment(vec![...])`
///
/// Before parsing, reads the component registry file to discover which
/// function names are marked as components via `#[component]`. This allows
/// the `html!` macro to distinguish between component function calls and
/// native HTML element tags.
///
/// # Arguments
///
/// - `TokenStream` - The raw token stream representing HTML markup.
///
/// # Returns
///
/// - `TokenStream` - The generated token stream constructing the corresponding virtual node.
pub(crate) fn parse_html(input: TokenStream) -> TokenStream {
    let fn_names: HashMap<String, ComponentInfo> = load_component_registry();
    set_user_fn_names(fn_names);
    let tokens: proc_macro2::TokenStream = match parse::<HtmlRoot>(input) {
        Ok(nodes) => nodes.into_token_stream(),
        Err(error) => return error.to_compile_error().into(),
    };
    TokenStream::from(tokens)
}

/// Loads the component registry by scanning the project source for `#[component]` annotations.
///
/// Uses a file-based cache in the `OUT_DIR` directory to avoid re-scanning and
/// re-parsing all source files on every `html!` macro invocation. The cache is
/// invalidated when the set of source files or their modification times change.
///
/// Recursively scans `.rs` files under `CARGO_MANIFEST_DIR/src/` and extracts
/// function names and their Props type names from annotated functions.
///
/// # Returns
///
/// - `HashMap<String, ComponentInfo>` - Map of component function names to component metadata.
pub(crate) fn load_component_registry() -> HashMap<String, ComponentInfo> {
    let Ok(manifest_dir) = env::var(CARGO_MANIFEST_DIR) else {
        return HashMap::new();
    };
    let mut rust_source_files: Vec<PathBuf> = Vec::new();
    let src_dir: PathBuf = PathBuf::from(&manifest_dir).join(SRC_DIR);
    collect_rs_files(&src_dir, &mut rust_source_files);
    let dep_src_dirs: Vec<PathBuf> = collect_local_dep_src_dirs(&manifest_dir);
    for dep_src_dir in dep_src_dirs {
        collect_rs_files(&dep_src_dir, &mut rust_source_files);
    }
    let fingerprint: String = compute_fingerprint(&rust_source_files);
    if let Ok(out_dir) = env::var(ENV_OUT_DIR) {
        let cache_path: PathBuf = PathBuf::from(out_dir).join(REGISTRY_CACHE_FILE_NAME);
        if let Some(cached) = try_load_cache(&cache_path, &fingerprint) {
            return cached;
        }
        let registry: HashMap<String, ComponentInfo> =
            build_registry_from_files(&rust_source_files);
        try_save_cache(&cache_path, &fingerprint, &registry);
        registry
    } else {
        build_registry_from_files(&rust_source_files)
    }
}

/// Computes a fingerprint string from the sorted list of source file paths
/// and their modification timestamps. Used to determine whether the cache
/// is still valid or needs to be rebuilt.
///
/// # Arguments
///
/// - `&[PathBuf]` - The sorted list of source file paths.
///
/// # Returns
///
/// - `String` - The computed fingerprint string.
fn compute_fingerprint(files: &[PathBuf]) -> String {
    let mut fingerprint: String = String::new();
    let mut sorted_files: Vec<&PathBuf> = files.iter().collect();
    sorted_files.sort();
    for path in sorted_files {
        fingerprint.push_str(&path.to_string_lossy());
        fingerprint.push(CHAR_SEMICOLON);
        if let Ok(metadata) = std::fs::metadata(path)
            && let Ok(modified) = metadata.modified()
            && let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH)
        {
            fingerprint.push_str(&duration.as_millis().to_string());
        }
        fingerprint.push(CHAR_SEMICOLON);
    }
    fingerprint
}

/// Attempts to load a cached component registry from the given cache path.
///
/// Returns `Some(registry)` if the cache exists and the stored fingerprint
/// matches the current fingerprint, indicating the cache is still valid.
/// Returns `None` if the cache does not exist, cannot be read, or is stale.
///
/// # Arguments
///
/// - `&PathBuf` - The path to the cache file.
/// - `&str` - The current fingerprint to validate against.
///
/// # Returns
///
/// - `Option<HashMap<String, ComponentInfo>>` - The cached registry if valid, or `None`.
fn try_load_cache(
    cache_path: &PathBuf,
    current_fingerprint: &str,
) -> Option<HashMap<String, ComponentInfo>> {
    let content: String = read_to_string(cache_path).ok()?;
    let (stored_fingerprint, data) = content.split_once(CHAR_NEWLINE)?;
    if stored_fingerprint != current_fingerprint {
        return None;
    }
    serde_json::from_str(data).ok()
}

/// Attempts to save the component registry to the given cache path,
/// along with the current fingerprint for future validation.
///
/// Silently ignores errors since caching is optional.
///
/// # Arguments
///
/// - `&PathBuf` - The path to the cache file.
/// - `&str` - The current fingerprint string.
/// - `&HashMap<String, ComponentInfo>` - The registry to cache.
fn try_save_cache(
    cache_path: &PathBuf,
    fingerprint: &str,
    registry: &HashMap<String, ComponentInfo>,
) {
    if let Ok(data) = serde_json::to_string(registry) {
        let content: String = format!("{fingerprint}{CHAR_NEWLINE}{data}");
        let _: std::io::Result<()> = write(cache_path, content);
    }
}

/// Collects the `src/` directories of local path dependencies from `Cargo.toml`.
///
/// Parses the `Cargo.toml` at the given manifest directory and extracts
/// all dependency entries that specify a `path` field pointing to a local
/// directory, or reference a workspace dependency. Returns the `src/` subdirectory
/// of each such dependency so that the component registry scanner can also
/// discover `#[component]` functions defined in local dependency crates.
///
/// # Arguments
///
/// - `&str` - The `CARGO_MANIFEST_DIR` path containing the `Cargo.toml`.
///
/// # Returns
///
/// - `Vec<PathBuf>` - A list of `src/` directory paths for local path dependencies.
fn collect_local_dep_src_dirs(manifest_dir: &str) -> Vec<PathBuf> {
    let cargo_toml_path: PathBuf = PathBuf::from(manifest_dir).join(CARGO_TOML);
    let Ok(content) = read_to_string(&cargo_toml_path) else {
        return Vec::new();
    };
    let Ok(manifest) = toml::from_str::<toml::Value>(&content) else {
        return Vec::new();
    };
    let mut dep_dirs: Vec<PathBuf> = Vec::new();
    let manifest_dir_path: PathBuf = PathBuf::from(manifest_dir);
    let workspace_root: PathBuf = find_workspace_root(manifest_dir);
    let workspace_toml: Option<toml::Value> = if workspace_root != manifest_dir_path {
        read_to_string(workspace_root.join(CARGO_TOML))
            .ok()
            .and_then(|toml_content: String| toml::from_str::<toml::Value>(&toml_content).ok())
    } else {
        None
    };
    for section_key in [DEPENDENCIES, WORKSPACE_DEPENDENCIES] {
        let Some(deps) = manifest
            .get(section_key)
            .and_then(|table_value: &toml::Value| table_value.as_table())
        else {
            continue;
        };
        for (name, value) in deps {
            let path_str: Option<&str> = if let Some(table) = value.as_table() {
                if table
                    .get(WORKSPACE_KEY)
                    .and_then(|workspace_flag: &toml::Value| workspace_flag.as_bool())
                    == Some(true)
                {
                    workspace_toml
                        .as_ref()
                        .and_then(|workspace_manifest: &toml::Value| {
                            workspace_manifest.get(WORKSPACE_KEY)
                        })
                        .and_then(|workspace_table: &toml::Value| workspace_table.get(DEPENDENCIES))
                        .and_then(|deps_table: &toml::Value| deps_table.get(name))
                        .and_then(|dep_entry: &toml::Value| dep_entry.as_table())
                        .and_then(|dep_table: &toml::Table| dep_table.get(PATH_KEY))
                        .and_then(|path_value: &toml::Value| path_value.as_str())
                } else {
                    table
                        .get(PATH_KEY)
                        .and_then(|path_value: &toml::Value| path_value.as_str())
                }
            } else {
                None
            };
            if let Some(path_str) = path_str {
                let path: PathBuf = PathBuf::from(path_str);
                let dep_dir: PathBuf = if path.is_absolute() {
                    path.join(SRC_DIR)
                } else if workspace_root != manifest_dir_path {
                    workspace_root.join(path_str).join(SRC_DIR)
                } else {
                    manifest_dir_path.join(path_str).join(SRC_DIR)
                };
                if dep_dir.is_dir() {
                    dep_dirs.push(dep_dir);
                }
            }
        }
    }
    dep_dirs
}

/// Finds the workspace root directory by traversing up from the manifest directory.
///
/// Searches for a `Cargo.toml` containing `[workspace]` section by walking up
/// the directory tree until the root is reached.
///
/// # Arguments
///
/// - `&str` - The starting manifest directory path.
///
/// # Returns
///
/// - `PathBuf` - The workspace root directory, or the starting directory if no workspace found.
fn find_workspace_root(manifest_dir: &str) -> PathBuf {
    let mut current: PathBuf = PathBuf::from(manifest_dir);
    loop {
        let cargo_toml: PathBuf = current.join(CARGO_TOML);
        if let Ok(content) = read_to_string(&cargo_toml)
            && content.contains(WORKSPACE_SECTION)
        {
            return current;
        }
        if !current.pop() {
            break;
        }
    }
    PathBuf::from(manifest_dir)
}

/// Builds the component registry by parsing all source files in a single pass.
///
/// Each file is read and parsed exactly once, extracting both struct definitions
/// (for Props field information) and component function annotations simultaneously.
///
/// # Arguments
///
/// - `&[PathBuf]` - The list of source file paths to parse.
///
/// # Returns
///
/// - `HashMap<String, ComponentInfo>` - Map of component function names to component metadata.
fn build_registry_from_files(files: &[PathBuf]) -> HashMap<String, ComponentInfo> {
    let mut global_props_fields_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut global_props_field_types_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut component_entries: Vec<(String, String)> = Vec::new();
    for path in files {
        let Ok(content) = read_to_string(path) else {
            continue;
        };
        let Ok(file) = parse_file(&content) else {
            continue;
        };
        global_props_fields_map.extend(extract_props_structs(&file));
        global_props_field_types_map.extend(extract_props_struct_types(&file));
        extract_component_entries(&file, &mut component_entries);
    }
    component_entries
        .into_iter()
        .map(|(fn_name, props_type): (String, String)| {
            let props_fields: Vec<String> = global_props_fields_map
                .get(&props_type)
                .cloned()
                .unwrap_or_default();
            let props_field_types: HashMap<String, String> = global_props_field_types_map
                .get(&props_type)
                .cloned()
                .unwrap_or_default();
            (
                fn_name,
                ComponentInfo {
                    props_type,
                    props_fields,
                    props_field_types,
                },
            )
        })
        .collect()
}

/// Extracts component function entries from a parsed file.
///
/// Collects (function_name, props_type) pairs for functions annotated with `#[component]`.
///
/// # Arguments
///
/// - `&File` - The parsed Rust source file.
/// - `&mut Vec<(String, String)>` - The vector to populate with (fn_name, props_type) pairs.
fn extract_component_entries(file: &File, entries: &mut Vec<(String, String)>) {
    file.items
        .iter()
        .filter_map(|item: &Item| {
            let Item::Fn(item_fn) = item else {
                return None;
            };
            item_fn
                .attrs
                .iter()
                .any(|attr: &Attribute| attr.path().is_ident(COMPONENT_ATTR))
                .then(|| {
                    let fn_name: String = item_fn.sig.ident.to_string();
                    let props_type: String = extract_props_type_from_fn(item_fn);
                    (fn_name, props_type)
                })
        })
        .for_each(|entry: (String, String)| {
            entries.push(entry);
        });
}

/// Recursively scans a directory for `.rs` files and collects their paths.
///
/// # Arguments
///
/// - `&PathBuf` - The directory to scan.
/// - `&mut Vec<PathBuf>` - The vector to populate with discovered file paths.
fn collect_rs_files(dir: &PathBuf, files: &mut Vec<PathBuf>) {
    let Ok(entries) = read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, files);
        } else if path
            .extension()
            .is_some_and(|ext: &OsStr| ext == OsStr::new(RUST_FILE_EXTENSION))
        {
            files.push(path);
        }
    }
}

/// Extracts all struct definitions from a file and maps their names to field name lists.
///
/// # Arguments
///
/// - `&File` - The parsed Rust source file.
///
/// # Returns
///
/// - `HashMap<String, Vec<String>>` - Map of struct name → list of field names.
fn extract_props_structs(file: &File) -> HashMap<String, Vec<String>> {
    file.items
        .iter()
        .filter_map(|item: &Item| {
            let Item::Struct(item_struct) = item else {
                return None;
            };
            Some((
                item_struct.ident.to_string(),
                item_struct
                    .fields
                    .iter()
                    .filter_map(|field: &Field| {
                        field.ident.as_ref().map(|ident: &Ident| ident.to_string())
                    })
                    .collect(),
            ))
        })
        .collect()
}

/// Extracts all struct definitions from a file and maps their names to field-type maps.
///
/// Each field's type is resolved to its last path segment (e.g., `VirtualNode`, `String`).
///
/// # Arguments
///
/// - `&File` - The parsed Rust source file.
///
/// # Returns
///
/// - `HashMap<String, HashMap<String, String>>` - Map of struct name → (field name → type string).
fn extract_props_struct_types(file: &File) -> HashMap<String, HashMap<String, String>> {
    file.items
        .iter()
        .filter_map(|item: &Item| {
            let Item::Struct(item_struct) = item else {
                return None;
            };
            Some((
                item_struct.ident.to_string(),
                item_struct
                    .fields
                    .iter()
                    .filter_map(|field: &Field| {
                        field.ident.as_ref().map(|ident: &Ident| {
                            (ident.to_string(), extract_type_last_segment(&field.ty))
                        })
                    })
                    .collect(),
            ))
        })
        .collect()
}

/// Extracts the last segment identifier from a type path.
///
/// For example, `::euv::VirtualNode` → `"VirtualNode"`, `String` → `"String"`.
/// Falls back to the full type string representation if the type is not a path.
///
/// # Arguments
///
/// - `&Type` - The syn type to extract from.
///
/// # Returns
///
/// - `String` - The last segment of the type path.
fn extract_type_last_segment(param_type: &Type) -> String {
    if let Type::Path(type_path) = param_type
        && let Some(segment) = type_path.path.segments.last()
    {
        return segment.ident.to_string();
    }
    param_type
        .to_token_stream()
        .to_string()
        .replace(CHAR_SPACE, STR_EMPTY)
}

/// Extracts the Props type name from the first parameter of a component function.
///
/// Looks for the first parameter's type. If the type is `VirtualNode<T>`,
/// extracts the generic argument `T` as the Props type name. Falls back to
/// checking for a simple path type (e.g., `PrimaryButtonProps`) for backward
/// compatibility. Returns an empty string if neither pattern matches.
///
/// # Arguments
///
/// - `&syn::ItemFn` - The function item to extract from.
///
/// # Returns
///
/// - `String` - The Props type name, or empty string if not extractable.
fn extract_props_type_from_fn(item_fn: &syn::ItemFn) -> String {
    let inputs: &syn::punctuated::Punctuated<syn::FnArg, Token![,]> = &item_fn.sig.inputs;
    for input in inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            let param_type: &Type = &pat_type.ty;
            if let Type::Path(type_path) = param_type
                && let Some(segment) = type_path.path.segments.last()
                && segment.ident == VIRTUAL_NODE_TYPE
            {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments
                    && let Some(syn::GenericArgument::Type(inner_param_type)) = args.args.first()
                    && let Type::Path(inner_path) = inner_param_type
                    && let Some(inner_segment) = inner_path.path.segments.last()
                {
                    return inner_segment.ident.to_string();
                }
            } else if let Type::Path(type_path) = param_type
                && let Some(segment) = type_path.path.segments.last()
            {
                return segment.ident.to_string();
            }
        }
    }
    String::new()
}

/// Checks whether a double-brace pattern `{{ ... }}` represents a dynamic tag
/// rather than a simple braced expression.
///
/// A dynamic tag is detected when the second brace group contains:
/// - Empty content, or
/// - An identifier followed by `:` or `-` (attribute pattern), or
/// - Keywords `if`, `match`, `for`, or
/// - A string literal, or
/// - A braced expression followed by `:` (dynamic key), or
/// - Another double brace (nested dynamic tag).
///
/// # Arguments
///
/// - `&ParseBuffer` - The parse buffer of the second brace group.
/// - `&ParseStream` - The outer parse stream (for `peek2` checks).
///
/// # Returns
///
/// - `bool` - `true` if the pattern is a dynamic tag.
pub(crate) fn is_dynamic_tag_pattern(second_brace: ParseStream, outer: ParseStream) -> bool {
    second_brace.is_empty()
        || is_attr_key_pattern(second_brace)
        || second_brace.peek(Token![if])
        || second_brace.peek(Token![match])
        || second_brace.peek(Token![for])
        || second_brace.peek(LitStr)
        || (second_brace.peek(Brace) && outer.peek2(Colon))
        || (second_brace.peek(Brace) && second_brace.peek2(Brace))
}

/// Parses a stream of tokens into a list of HTML child nodes.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream containing HTML child content.
///
/// # Returns
///
/// - `syn::Result<Vec<HtmlNode>>` - The parsed list of HTML child nodes, or a syntax error.
pub(crate) fn parse_html_children(content: ParseStream) -> syn::Result<Vec<HtmlNode>> {
    let mut children: Vec<HtmlNode> = Vec::new();
    while !content.is_empty() {
        if content.peek(Brace) && content.peek2(Brace) {
            let forked: ParseBuffer<'_> = content.fork();
            let _first_brace: ParseBuffer<'_>;
            braced!(_first_brace in forked);
            let second_brace: ParseBuffer<'_>;
            braced!(second_brace in forked);
            if is_dynamic_tag_pattern(&second_brace, content) {
                let tag_content: ParseBuffer<'_>;
                braced!(tag_content in content);
                let tag_expr: Expr = tag_content.parse()?;
                let body_content: ParseBuffer<'_>;
                braced!(body_content in content);
                let (dynamic_attrs, dynamic_children): (HtmlAttrs, Vec<HtmlNode>) =
                    parse_dynamic_component_children(&body_content)?;
                children.push(HtmlNode::DynamicTag(HtmlDynamicTag::new(
                    tag_expr,
                    dynamic_attrs,
                    dynamic_children,
                )));
            } else {
                let child_content: ParseBuffer<'_>;
                braced!(child_content in content);
                let expr: Expr = child_content.parse()?;
                children.push(HtmlNode::Dynamic(expr));
            }
        } else if content.peek(LitStr) && content.peek2(Brace) {
            let element: HtmlElement = content.parse()?;
            children.push(HtmlNode::Element(element));
        } else if content.peek(LitStr) {
            let literal_string: LitStr = content.parse()?;
            children.push(HtmlNode::Text(literal_string.value()));
        } else if (is_attr_key_pattern(content) || content.peek(LitStr) && content.peek2(Colon))
            && !is_double_colon(content)
        {
            break;
        } else if content.peek(Token![if]) {
            let html_if: HtmlIf = content.parse()?;
            children.push(HtmlNode::If(html_if));
        } else if content.peek(Token![match]) {
            let html_match: HtmlMatch = content.parse()?;
            children.push(HtmlNode::Match(html_match));
        } else if content.peek(Token![for]) {
            let html_for: HtmlFor = content.parse()?;
            children.push(HtmlNode::For(html_for));
        } else if content.peek(Brace) {
            let child_content: ParseBuffer<'_>;
            braced!(child_content in content);
            let expr: Expr = child_content.parse()?;
            children.push(HtmlNode::Dynamic(expr));
        } else if content.peek(Ident) {
            if content.peek2(Brace) {
                let element: HtmlElement = content.parse()?;
                children.push(HtmlNode::Element(element));
            } else {
                let expr: Expr = content.parse()?;
                children.push(HtmlNode::Expr(expr));
            }
        } else {
            return Err(content.error(ERR_UNEXPECTED_TOKEN_IN_HTML));
        }
    }
    Ok(children)
}

/// Parses the body of a match arm after the `=>` token.
///
/// Unlike `parse_html_children` which operates on a braced scope, this function
/// reads directly from the arms content stream and stops when it encounters a
/// top-level comma (indicating the next arm) or the end of the stream.
/// Supports all HTML node types: elements, text, expressions, if, match, for,
/// and braced dynamic expressions.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream positioned after `=>` in a match arm.
///
/// # Returns
///
/// - `syn::Result<Vec<HtmlNode>>` - The parsed list of HTML nodes for the arm body.
pub(crate) fn parse_match_arm_body(content: ParseStream) -> syn::Result<Vec<HtmlNode>> {
    if content.peek(Brace) {
        let child_content: ParseBuffer<'_>;
        braced!(child_content in content);
        parse_html_children(&child_content)
    } else {
        let node: HtmlNode = content.parse()?;
        Ok(vec![node])
    }
}

/// Parses the body of a dynamic component `@ {expr} { ... }`.
///
/// The body contains attributes (key: value) and children (HTML nodes),
/// similar to an `HtmlElement` body but without a tag name.
/// Attributes are recognized by the pattern `ident:` or `ident-...:`.
/// Everything else is treated as child content.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream containing the dynamic component body.
///
/// # Returns
///
/// - `syn::Result<(HtmlAttrs, Vec<HtmlNode>)>` - The parsed attributes and children.
pub(crate) fn parse_dynamic_component_children(
    content: ParseStream,
) -> syn::Result<(HtmlAttrs, Vec<HtmlNode>)> {
    let mut attributes: HtmlAttrs = Vec::new();
    let mut children: Vec<HtmlNode> = Vec::new();
    while !content.is_empty() {
        if content.peek(Brace) && content.peek2(Brace) {
            let forked: ParseBuffer<'_> = content.fork();
            let _first_brace: ParseBuffer<'_>;
            braced!(_first_brace in forked);
            let second_brace: ParseBuffer<'_>;
            braced!(second_brace in forked);
            if is_dynamic_tag_pattern(&second_brace, content) {
                let tag_content: ParseBuffer<'_>;
                braced!(tag_content in content);
                let tag_expr: Expr = tag_content.parse()?;
                let body_content: ParseBuffer<'_>;
                braced!(body_content in content);
                let (dynamic_attrs, dynamic_children): (HtmlAttrs, Vec<HtmlNode>) =
                    parse_dynamic_component_children(&body_content)?;
                children.push(HtmlNode::DynamicTag(HtmlDynamicTag::new(
                    tag_expr,
                    dynamic_attrs,
                    dynamic_children,
                )));
            } else {
                let child_content: ParseBuffer<'_>;
                braced!(child_content in content);
                let expr: Expr = child_content.parse()?;
                children.push(HtmlNode::Dynamic(expr));
            }
        } else if is_attr_key_pattern(content) && !is_double_colon(content) {
            let key_string: String = parse_ident_name(content)?;
            let key_literal: LitStr = LitStr::new(&key_string, content.span());
            content.parse::<Colon>()?;
            let key_str: String = key_string
                .strip_prefix(RAW_IDENT_PREFIX)
                .unwrap_or(&key_string)
                .to_string();
            let value: HtmlAttrValue = parse_attr_value(content, &key_str)?;
            attributes.push((key_literal.to_token_stream(), value));
        } else if content.peek(Token![if]) {
            let html_if: HtmlIf = content.parse()?;
            children.push(HtmlNode::If(html_if));
        } else if content.peek(Token![match]) {
            let html_match: HtmlMatch = content.parse()?;
            children.push(HtmlNode::Match(html_match));
        } else if content.peek(Token![for]) {
            let html_for: HtmlFor = content.parse()?;
            children.push(HtmlNode::For(html_for));
        } else if content.peek(Brace) && content.peek2(Colon) {
            let key_content: ParseBuffer<'_>;
            braced!(key_content in content);
            let key_expr: Expr = key_content.parse()?;
            content.parse::<Colon>()?;
            let value: HtmlAttrValue = parse_attr_value(content, STR_EMPTY)?;
            attributes.push((key_expr.to_token_stream(), value));
        } else if content.peek(Brace) {
            let child_content: ParseBuffer<'_>;
            braced!(child_content in content);
            let expr: Expr = child_content.parse()?;
            children.push(HtmlNode::Dynamic(expr));
        } else if content.peek(LitStr) && content.peek2(Brace) {
            let element: HtmlElement = content.parse()?;
            children.push(HtmlNode::Element(element));
        } else if content.peek(LitStr) && content.peek2(Colon) {
            let key_literal: LitStr = content.parse()?;
            let key_str: String = key_literal.value();
            content.parse::<Colon>()?;
            let value: HtmlAttrValue = parse_attr_value(content, &key_str)?;
            attributes.push((key_literal.to_token_stream(), value));
        } else if content.peek(LitStr) {
            let literal_string: LitStr = content.parse()?;
            children.push(HtmlNode::Text(literal_string.value()));
        } else if content.peek(Ident) {
            if content.peek2(Brace) {
                let element: HtmlElement = content.parse()?;
                children.push(HtmlNode::Element(element));
            } else {
                let expr: Expr = content.parse()?;
                children.push(HtmlNode::Expr(expr));
            }
        } else {
            return Err(content.error(ERR_UNEXPECTED_TOKEN_IN_DYNAMIC_COMPONENT));
        }
    }
    let merged_attributes: HtmlAttrs = merge_same_key_attributes(attributes);
    Ok((merged_attributes, children))
}

/// Converts a slice of `HtmlNode` children into a `Vec<proc_macro2::TokenStream>`.
///
/// Shared helper used by both `children_to_node_tokens` and `children_to_tokens`.
///
/// # Arguments
///
/// - `&[HtmlNode]` - The slice of HTML child nodes to convert.
///
/// # Returns
///
/// - `Vec<proc_macro2::TokenStream>` - The generated token stream representing a single `VirtualNode`.
pub(crate) fn nodes_to_token_vec(children: &[HtmlNode]) -> Vec<proc_macro2::TokenStream> {
    children
        .iter()
        .map(|child: &HtmlNode| {
            let mut token_stream: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            child.to_tokens(&mut token_stream);
            token_stream
        })
        .collect()
}

/// Builds a Rust `if/else if/else` chain token stream from `HtmlIf` branches,
/// where each branch body produces a single `VirtualNode`.
///
/// Used for inline (non-reactive) conditionals at the top level or inside
/// other conditionals/match arms where a single `VirtualNode` is expected.
///
/// # Arguments
///
/// - `&[(Option<Expr>, Vec<HtmlNode>)]` - The branches from an `HtmlIf`.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated if-chain token stream producing a `VirtualNode`.
pub(crate) fn build_html_if_chain(
    branches: &[(Option<Expr>, Vec<HtmlNode>, bool)],
) -> proc_macro2::TokenStream {
    let mut if_chain: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    let has_else: bool = branches
        .last()
        .is_some_and(|(condition, _, _): &(Option<Expr>, Vec<HtmlNode>, bool)| condition.is_none());
    for (branch_index, (condition, body, is_reactive)) in branches.iter().enumerate() {
        let body_expr: proc_macro2::TokenStream = children_to_node_tokens(body);
        match (branch_index, condition) {
            (0, Some(cond)) => {
                let cond_tokens: proc_macro2::TokenStream =
                    auto_get_expr_tokens(cond, *is_reactive);
                if_chain.extend(quote! { if #cond_tokens { #body_expr } });
            }
            (_, Some(cond)) => {
                let cond_tokens: proc_macro2::TokenStream =
                    auto_get_expr_tokens(cond, *is_reactive);
                if_chain.extend(quote! { else if #cond_tokens { #body_expr } });
            }
            (_, None) => {
                if_chain.extend(quote! { else { #body_expr } });
            }
        }
    }
    if !has_else {
        if_chain.extend(quote! { else { ::euv::VirtualNode::Empty } });
    }
    if_chain
}

/// Converts a list of `HtmlNode` children into a single `VirtualNode` token stream.
///
/// - 0 children → `VirtualNode::Empty`
/// - 1 child → the child's token stream directly (no Fragment wrapper)
/// - N children → `VirtualNode::Fragment(vec![...])`
///
/// Inline (non-reactive) `if` conditionals are expanded as Rust `if` expressions
/// that produce a `VirtualNode`.
///
/// # Arguments
///
/// - `&[HtmlNode]` - The slice of HTML child nodes to convert.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated token stream representing a single `VirtualNode`.
pub(crate) fn children_to_node_tokens(children: &[HtmlNode]) -> proc_macro2::TokenStream {
    let has_inline_if: bool = children.iter().any(
        |child: &HtmlNode| matches!(child, HtmlNode::If(html_if) if !html_if.get_is_reactive()),
    );
    if has_inline_if {
        let vec_tokens: proc_macro2::TokenStream = children_to_tokens(children);
        return quote! { ::euv::VirtualNode::Fragment(#vec_tokens) };
    }
    match children.len() {
        0 => quote! { ::euv::VirtualNode::Empty },
        1 => {
            let mut token_stream: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            children[0].to_tokens(&mut token_stream);
            token_stream
        }
        _ => {
            let child_tokens: Vec<proc_macro2::TokenStream> = nodes_to_token_vec(children);
            quote! { ::euv::VirtualNode::Fragment(vec![#(#child_tokens), *]) }
        }
    }
}

/// Builds a Rust `if/else if/else` chain token stream from `HtmlIf` branches,
/// where each branch body produces a `Vec<VirtualNode>`.
///
/// Used for inline (non-reactive) conditionals inside `for` loops and flattened
/// element children, where each branch result is collected via `.extend()`.
///
/// # Arguments
///
/// - `&[(Option<Expr>, Vec<HtmlNode>)]` - The branches from an `HtmlIf`.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated if-chain token stream producing `Vec<VirtualNode>`.
pub(crate) fn build_html_if_chain_to_vec(
    branches: &[(Option<Expr>, Vec<HtmlNode>, bool)],
) -> proc_macro2::TokenStream {
    let mut if_chain: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    let has_else: bool = branches
        .last()
        .is_some_and(|(condition, _, _): &(Option<Expr>, Vec<HtmlNode>, bool)| condition.is_none());
    for (branch_index, (condition, body, is_reactive)) in branches.iter().enumerate() {
        let body_expr: proc_macro2::TokenStream = children_to_tokens(body);
        match (branch_index, condition) {
            (0, Some(cond)) => {
                let cond_tokens: proc_macro2::TokenStream =
                    auto_get_expr_tokens(cond, *is_reactive);
                if_chain.extend(quote! { if #cond_tokens { #body_expr } });
            }
            (_, Some(cond)) => {
                let cond_tokens: proc_macro2::TokenStream =
                    auto_get_expr_tokens(cond, *is_reactive);
                if_chain.extend(quote! { else if #cond_tokens { #body_expr } });
            }
            (_, None) => {
                if_chain.extend(quote! { else { #body_expr } });
            }
        }
    }
    if !has_else {
        if_chain.extend(quote! { else { Vec::new() } });
    }
    if_chain
}

/// Converts a list of `HtmlNode` children into a `Vec<VirtualNode>` token stream.
///
/// Always produces `vec![...]` format when no inline conditionals are present.
/// When inline (non-reactive) `if` conditionals exist, generates a block that
/// builds the `Vec<VirtualNode>` incrementally using `.push()` and `.extend()`.
///
/// # Arguments
///
/// - `&[HtmlNode]` - The slice of HTML child nodes to convert.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated token stream representing a `Vec<VirtualNode>`.
pub(crate) fn children_to_tokens(children: &[HtmlNode]) -> proc_macro2::TokenStream {
    let has_inline_if: bool = children.iter().any(
        |child: &HtmlNode| matches!(child, HtmlNode::If(html_if) if !html_if.get_is_reactive()),
    );
    if !has_inline_if {
        let child_tokens: Vec<proc_macro2::TokenStream> = nodes_to_token_vec(children);
        return quote! { vec![#(#child_tokens), *] };
    }
    let mut parts: Vec<proc_macro2::TokenStream> = Vec::new();
    for child in children {
        match child {
            HtmlNode::If(html_if) if !html_if.get_is_reactive() => {
                let if_chain: proc_macro2::TokenStream =
                    build_html_if_chain_to_vec(html_if.get_branches());
                parts.push(quote! {
                    __euv_nodes.extend(#if_chain);
                });
            }
            _ => {
                let mut token_stream: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                child.to_tokens(&mut token_stream);
                parts.push(quote! {
                    __euv_nodes.push(#token_stream);
                });
            }
        }
    }
    quote! {
        {
            let mut __euv_nodes: Vec<::euv::VirtualNode> = Vec::new();
            #(#parts)*
            __euv_nodes
        }
    }
}

/// Generates a token stream that builds a `Vec<VirtualNode>` with `For` loops
/// and inline `if` conditionals expanded inline via `.extend()` instead of being
/// wrapped in `VirtualNode::Fragment`.
///
/// This is critical for elements like `<select>` where intermediate wrapper elements
/// (such as `<slot>` used by `VirtualNode::Fragment`) are invalid HTML and cause
/// browser rendering issues. By flattening `For` loop outputs directly into the
/// parent's children list, option elements appear as direct children of `<select>`.
///
/// # Arguments
///
/// - `&[HtmlNode]` - The slice of HTML child nodes to convert.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated token stream representing a `Vec<VirtualNode>`.
pub(crate) fn children_to_flattened_tokens(children: &[HtmlNode]) -> proc_macro2::TokenStream {
    let needs_flatten: bool = children.iter().any(|child: &HtmlNode| {
        matches!(child, HtmlNode::For(_))
            || matches!(child, HtmlNode::If(html_if) if !html_if.get_is_reactive())
    });
    if !needs_flatten {
        let child_tokens: Vec<proc_macro2::TokenStream> = nodes_to_token_vec(children);
        return quote! { vec![#(#child_tokens), *] };
    }
    let mut parts: Vec<proc_macro2::TokenStream> = Vec::new();
    for child in children {
        match child {
            HtmlNode::For(html_for) => {
                let pattern: &proc_macro2::TokenStream = html_for.get_pattern();
                let iterable: &Expr = html_for.get_iterable();
                let iterable_tokens: proc_macro2::TokenStream =
                    auto_get_expr_tokens(iterable, html_for.get_is_reactive());
                let body_tokens: proc_macro2::TokenStream = children_to_tokens(html_for.get_body());
                if html_for.get_is_reactive() {
                    // A braced reactive iterable (`for x in { signal }`) must
                    // re-run whenever the signal changes. The inline snapshot
                    // loop below would run exactly once at mount outside any
                    // tracking scope, registering no dependent, so the list
                    // would never update. Wrap the loop in a dynamic node
                    // instead: its render closure runs with an active tracking
                    // id, so `Signal::get` subscribes this node, and `Signal::set`
                    // later marks it dirty and re-renders the `Fragment`.
                    parts.push(quote! {
                        __euv_nodes.push(::euv::VirtualNode::create_dynamic(
                            move |_: &mut ::euv::HookContext| {
                                let mut __euv_inner: Vec<::euv::VirtualNode> = Vec::new();
                                for #pattern in #iterable_tokens {
                                    __euv_inner.extend(#body_tokens);
                                }
                                ::euv::VirtualNode::Fragment(__euv_inner)
                            }
                        ));
                    });
                } else {
                    parts.push(quote! {
                        for #pattern in #iterable_tokens {
                            __euv_nodes.extend(#body_tokens);
                        }
                    });
                }
            }
            HtmlNode::If(html_if) if !html_if.get_is_reactive() => {
                let if_chain: proc_macro2::TokenStream =
                    build_html_if_chain_to_vec(html_if.get_branches());
                parts.push(quote! {
                    __euv_nodes.extend(#if_chain);
                });
            }
            _ => {
                let mut token_stream: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                child.to_tokens(&mut token_stream);
                parts.push(quote! {
                    __euv_nodes.push(#token_stream);
                });
            }
        }
    }
    quote! {
        {
            let mut __euv_nodes: Vec<::euv::VirtualNode> = Vec::new();
            #(#parts)*
            __euv_nodes
        }
    }
}

/// Parses a reactive or inline `if` conditional in attribute value position.
///
/// Each branch condition is independently parsed as either reactive (braced)
/// or inline (plain expression). The overall `is_inline` flag is set to
/// `false` if any branch has a braced condition, causing the entire if-chain
/// to be wrapped in a reactive `AttributeValue`.
///
/// Supported syntaxes per branch:
/// - Reactive: `{expr}` — the braced expression is treated as a signal.
/// - Inline: `condition` — a plain Rust boolean expression.
///
/// Any combination is valid, e.g.:
/// - `if {a} { v } else if {b} { v }` — all reactive
/// - `if a { v } else if b { v }` — all inline
/// - `if {a} { v } else if b { v }` — mixed (first reactive, second inline)
/// - `if a { v } else if {b} { v }` — mixed (first inline, second reactive)
///
/// When no explicit `else` branch is provided, an empty string is used as the default.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream positioned at the `if` keyword.
///
/// # Returns
///
/// - `syn::Result<HtmlAttrIf>` - The parsed attribute-level reactive or inline conditional.
pub(crate) fn parse_attr_if(content: ParseStream) -> syn::Result<HtmlAttrIf> {
    let mut branches: Vec<(Option<Expr>, Expr, bool)> = Vec::new();
    let mut is_inline: bool = true;
    content.parse::<Token![if]>()?;
    let branch_reactive: bool = content.peek(Brace);
    is_inline = is_inline && !branch_reactive;
    let condition: Expr = if branch_reactive {
        let cond_content: ParseBuffer<'_>;
        braced!(cond_content in content);
        cond_content.parse()?
    } else {
        parse_expr_until_brace(content)?
    };
    let body_content: ParseBuffer<'_>;
    braced!(body_content in content);
    let body: Expr = body_content.parse()?;
    branches.push((Some(condition), body, branch_reactive));
    while content.peek(Token![else]) {
        content.parse::<Token![else]>()?;
        if content.peek(Token![if]) {
            content.parse::<Token![if]>()?;
            let branch_reactive: bool = content.peek(Brace);
            is_inline = is_inline && !branch_reactive;
            let condition: Expr = if branch_reactive {
                let cond_content: ParseBuffer<'_>;
                braced!(cond_content in content);
                cond_content.parse()?
            } else {
                parse_expr_until_brace(content)?
            };
            let body_content: ParseBuffer<'_>;
            braced!(body_content in content);
            let body: Expr = body_content.parse()?;
            branches.push((Some(condition), body, branch_reactive));
        } else {
            let body_content: ParseBuffer<'_>;
            braced!(body_content in content);
            let body: Expr = body_content.parse()?;
            branches.push((None, body, false));
            break;
        }
    }
    let else_default: proc_macro2::TokenStream = quote! { #STR_EMPTY };
    Ok(HtmlAttrIf {
        is_inline,
        branches,
        else_default,
    })
}

/// Parses a reactive or inline `match` expression in attribute value position.
///
/// Supports two syntaxes:
/// - Reactive: `match {expr} { pattern => value, ... }`
///   Detected when `match` is immediately followed by `{`.
/// - Inline: `match expr { pattern => value, ... }`
///   Detected when `match` is followed by a non-`{` token.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream positioned at the `match` keyword.
///
/// # Returns
///
/// - `syn::Result<HtmlAttrMatch>` - The parsed attribute-level reactive or inline match expression.
pub(crate) fn parse_attr_match(content: ParseStream) -> syn::Result<HtmlAttrMatch> {
    let is_inline: bool = !content.peek2(Brace);
    content.parse::<Token![match]>()?;
    let scrutinee: Expr = if is_inline {
        parse_expr_until_brace(content)?
    } else {
        let scrutinee_content: ParseBuffer<'_>;
        braced!(scrutinee_content in content);
        scrutinee_content.parse()?
    };
    let arms_content: ParseBuffer<'_>;
    braced!(arms_content in content);
    let mut arms: Vec<(proc_macro2::TokenStream, Expr)> = Vec::new();
    while !arms_content.is_empty() {
        let mut pattern_tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
        while !arms_content.peek(Token![=>]) {
            let token_tree: proc_macro2::TokenTree = arms_content.parse()?;
            pattern_tokens.extend([token_tree]);
        }
        arms_content.parse::<Token![=>]>()?;
        let body: Expr = if arms_content.peek(Brace) {
            let body_content: ParseBuffer<'_>;
            braced!(body_content in arms_content);
            body_content.parse()?
        } else {
            arms_content.parse()?
        };
        arms.push((pattern_tokens, body));
        if arms_content.peek(Token![,]) {
            arms_content.parse::<Token![,]>()?;
        }
    }
    Ok(HtmlAttrMatch {
        is_inline,
        scrutinee,
        arms,
    })
}

/// Strips outer braces from an `Expr` if it is an `Expr::Block` with a single expression,
/// avoiding Rust `unused_braces` warnings in generated `if` conditions.
///
/// # Arguments
///
/// - `&Expr` - The expression to potentially strip.
///
/// # Returns
///
/// - `&Expr` - The inner expression if the input was a braced single-expression block, otherwise the original.
pub(crate) fn strip_braces_from_expr(expr: &Expr) -> &Expr {
    if let Expr::Block(expr_block) = expr {
        let stmts: &Vec<Stmt> = &expr_block.block.stmts;
        if stmts.len() == 1
            && let Stmt::Expr(inner, None) = &stmts[0]
        {
            return inner;
        }
    }
    expr
}

/// Generates a token stream for an `HtmlAttrIf` as a Rust `if` expression.
///
/// The generated code is used inside a reactive closure so that when signals
/// change, the conditional is re-evaluated.
///
/// The `mode` parameter controls how branch bodies are emitted:
/// - `AttrIfMode::Reactive` - Each branch body is wrapped in
///   `::euv::IntoReactiveString::into_reactive_string(...)` so that all branches
///   produce a `String` regardless of their original type (e.g., `Css`, `&str`, `String`).
///   This ensures type compatibility when the `if` and implicit `else` branches
///   return different types.
/// - `AttrIfMode::Raw` - Branch bodies are emitted as-is without wrapping.
///   Used for component props where branch types are already consistent.
///
/// # Arguments
///
/// - `&HtmlAttrIf` - The parsed attribute-level reactive conditional.
/// - `proc_macro2::TokenStream` - The default else branch token stream, used when no explicit else branch exists.
/// - `AttrIfMode` - The code generation mode for branch body wrapping.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated `if ... { ... } else if ... { ... } else { ... }` token stream.
pub(crate) fn attr_if_to_tokens(ctx: &AttrIfContext<'_>) -> proc_macro2::TokenStream {
    let html_attr_if: &HtmlAttrIf = ctx.get_html_attr_if();
    let else_default: &proc_macro2::TokenStream = ctx.get_else_default();
    let mode: AttrIfMode = ctx.get_mode();
    let mut if_chain: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    let has_else: bool = html_attr_if
        .branches
        .last()
        .is_some_and(|(condition, _, _): &(Option<Expr>, Expr, bool)| condition.is_none());
    for (branch_index, (condition, body, is_reactive)) in html_attr_if.branches.iter().enumerate() {
        let body_tokens: proc_macro2::TokenStream = match mode {
            AttrIfMode::Reactive => {
                quote! { (#body).to_string() }
            }
            AttrIfMode::Raw => quote! { #body },
        };
        match (branch_index, condition) {
            (0, Some(cond)) => {
                let cond_tokens: proc_macro2::TokenStream =
                    auto_get_expr_tokens(cond, *is_reactive);
                if_chain.extend(quote! { if #cond_tokens { #body_tokens } });
            }
            (_, Some(cond)) => {
                let cond_tokens: proc_macro2::TokenStream =
                    auto_get_expr_tokens(cond, *is_reactive);
                if_chain.extend(quote! { else if #cond_tokens { #body_tokens } });
            }
            (_, None) => {
                if_chain.extend(quote! { else { #body_tokens } });
            }
        }
    }
    if !has_else {
        let else_tokens: proc_macro2::TokenStream = match mode {
            AttrIfMode::Reactive => {
                quote! { (#else_default).to_string() }
            }
            AttrIfMode::Raw => quote! { #else_default },
        };
        if_chain.extend(quote! { else { #else_tokens } });
    }
    if_chain
}

/// Generates a token stream for an `HtmlAttrMatch` as a Rust `match` expression.
///
/// The `mode` parameter controls how arm bodies are emitted:
/// - `AttrIfMode::Reactive` - Each arm body is wrapped with `.to_string()`.
/// - `AttrIfMode::Raw` - Arm bodies are emitted as-is without wrapping.
///
/// # Arguments
///
/// - `&HtmlAttrMatch` - The parsed attribute-level match expression.
/// - `AttrIfMode` - The code generation mode for arm body wrapping.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - The generated `match ... { ... }` token stream.
pub(crate) fn attr_match_to_tokens(
    html_attr_match: &HtmlAttrMatch,
    mode: AttrIfMode,
) -> proc_macro2::TokenStream {
    let scrutinee: &Expr = html_attr_match.get_scrutinee();
    let scrutinee_tokens: proc_macro2::TokenStream =
        auto_get_expr_tokens(scrutinee, !html_attr_match.get_is_inline());
    let arm_tokens: Vec<proc_macro2::TokenStream> = html_attr_match
        .get_arms()
        .iter()
        .map(|(pattern, body): &(proc_macro2::TokenStream, Expr)| {
            let body_tokens: proc_macro2::TokenStream = match mode {
                AttrIfMode::Reactive => {
                    quote! { (#body).to_string() }
                }
                AttrIfMode::Raw => quote! { #body },
            };
            quote! { #pattern => #body_tokens, }
        })
        .collect();
    quote! { match #scrutinee_tokens { #(#arm_tokens)* } }
}

/// Checks whether an `HtmlAttrValue` contains any inline (non-reactive) conditional logic.
///
/// Returns `true` if the value is an inline `If` or inline `Match`, or if it contains
/// inline conditionals in `Style` properties.
///
/// # Arguments
///
/// - `&HtmlAttrValue` - The attribute value to check.
///
/// # Returns
///
/// - `bool` - `true` if the value contains inline conditional logic.
pub(crate) fn is_attr_value_inline(value: &HtmlAttrValue) -> bool {
    match value {
        HtmlAttrValue::If(html_attr_if) => html_attr_if.get_is_inline(),
        HtmlAttrValue::Match(html_attr_match) => html_attr_match.get_is_inline(),
        HtmlAttrValue::Style(props) => props.iter().any(
            |(_, style_value): &(String, HtmlStylePropValue)| {
                matches!(style_value, HtmlStylePropValue::If(html_attr_if) if html_attr_if.get_is_inline())
                    || matches!(style_value, HtmlStylePropValue::Match(html_attr_match) if html_attr_match.get_is_inline())
            },
        ),
        _ => false,
    }
}

/// Checks whether style properties contain any conditional logic.
///
/// # Arguments
///
/// - `&[(String, HtmlStylePropValue)]` - The style properties to check.
///
/// # Returns
///
/// - `bool` - `true` if any style property contains a conditional.
pub(crate) fn is_style_props_conditional(props: &[(String, HtmlStylePropValue)]) -> bool {
    props
        .iter()
        .any(|(_, value): &(String, HtmlStylePropValue)| {
            matches!(
                value,
                HtmlStylePropValue::If(_) | HtmlStylePropValue::Match(_)
            )
        })
}

/// Parses the value side of an attribute, handling the special `style:` attribute.
///
/// If the key is `"style"` and the value is a braced expression that looks like
/// a style object (key-value pairs separated by `;`), it is parsed as
/// `HtmlAttrValue::Style`. Otherwise, the value is parsed as a normal expression
/// or a reactive `if` conditional.
///
/// # Arguments
///
/// - `ParseStream` - The parse stream positioned after the ` -` token.
/// - `&str` - The attribute key string (e.g., `"style"`, `"class"`).
///
/// # Returns
///
/// - `syn::Result<HtmlAttrValue>` - The parsed attribute value.
pub(crate) fn parse_attr_value(content: ParseStream, key_str: &str) -> syn::Result<HtmlAttrValue> {
    if content.peek(Token![if]) {
        return Ok(HtmlAttrValue::If(parse_attr_if(content)?));
    }
    if content.peek(Token![match]) {
        return Ok(HtmlAttrValue::Match(parse_attr_match(content)?));
    }
    if key_str == ATTR_KEY_STYLE && content.peek(Brace) {
        let style_content: ParseBuffer<'_>;
        braced!(style_content in content);
        let is_style_object: bool = style_content.peek(LitStr) || style_content.peek(Ident);
        if is_style_object {
            let mut style_props: Vec<(String, HtmlStylePropValue)> = Vec::new();
            while !style_content.is_empty() {
                let css_key: String = parse_ident_name(&style_content)?;
                style_content.parse::<Colon>()?;
                let prop_value: HtmlStylePropValue = if style_content.peek(Token![if]) {
                    let html_attr_if: HtmlAttrIf = parse_attr_if(&style_content)?;
                    HtmlStylePropValue::If(html_attr_if)
                } else if style_content.peek(Token![match]) {
                    let html_attr_match: HtmlAttrMatch = parse_attr_match(&style_content)?;
                    HtmlStylePropValue::Match(html_attr_match)
                } else if style_content.peek(LitStr) {
                    let literal_string: LitStr = style_content.parse()?;
                    HtmlStylePropValue::Literal(literal_string.value())
                } else if style_content.peek(Brace) {
                    let expr_content: ParseBuffer<'_>;
                    braced!(expr_content in style_content);
                    if expr_content.peek(Token![if]) {
                        let html_attr_if: HtmlAttrIf = parse_attr_if(&expr_content)?;
                        HtmlStylePropValue::If(html_attr_if)
                    } else if expr_content.peek(Token![match]) {
                        let html_attr_match: HtmlAttrMatch = parse_attr_match(&expr_content)?;
                        HtmlStylePropValue::Match(html_attr_match)
                    } else {
                        let expr: Expr = expr_content.parse()?;
                        HtmlStylePropValue::Expr(expr)
                    }
                } else {
                    let expr: Expr = style_content.parse()?;
                    HtmlStylePropValue::Expr(expr)
                };
                style_props.push((css_key, prop_value));
                if style_content.peek(Semi) {
                    style_content.parse::<Semi>()?;
                }
            }
            Ok(HtmlAttrValue::Style(style_props))
        } else {
            Ok(HtmlAttrValue::Expr(style_content.parse()?))
        }
    } else {
        Ok(HtmlAttrValue::Expr(content.parse()?))
    }
}

/// Merges attributes with the same key name for `class` and `style`.
///
/// When multiple `class:` or `style:` attributes are declared on the same
/// element, they are combined into a single `HtmlAttrValue::Classes` or
/// `HtmlAttrValue::Styles` entry so that the renderer can merge their
/// values at runtime rather than overwriting.
///
/// Non-mergeable attribute keys keep only the last occurrence.
///
/// # Arguments
///
/// - `Vec<(Ident, HtmlAttrValue)>` - The raw parsed attributes (may contain duplicate keys).
///
/// # Returns
///
/// - `Vec<(Ident, HtmlAttrValue)>` - The merged attributes with at most one `class` and one `style` entry.
pub(crate) fn merge_same_key_attributes(attributes: HtmlAttrs) -> HtmlAttrs {
    let mut class_values: Vec<HtmlAttrValue> = Vec::new();
    let mut style_values: Vec<HtmlAttrValue> = Vec::new();
    let mut result: HtmlAttrs = Vec::new();
    for (key, value) in attributes {
        let key_string: String = extract_attr_key_string(&key);
        if key_string == ATTR_KEY_CLASS {
            class_values.push(value);
        } else if key_string == ATTR_KEY_STYLE {
            style_values.push(value);
        } else {
            result.push((key, value));
        }
    }
    let push_merged = |result: &mut HtmlAttrs,
                       key_str: &str,
                       mut values: Vec<HtmlAttrValue>,
                       wrap: fn(Vec<HtmlAttrValue>) -> HtmlAttrValue|
     -> () {
        match values.len() {
            0 => {}
            1 => result.push((
                LitStr::new(key_str, proc_macro2::Span::call_site()).to_token_stream(),
                values.remove(0),
            )),
            _ => result.push((
                LitStr::new(key_str, proc_macro2::Span::call_site()).to_token_stream(),
                wrap(values),
            )),
        }
    };
    push_merged(
        &mut result,
        ATTR_KEY_CLASS,
        class_values,
        HtmlAttrValue::Classes,
    );
    push_merged(
        &mut result,
        ATTR_KEY_STYLE,
        style_values,
        HtmlAttrValue::Styles,
    );
    result
}

/// Converts an `HtmlAttrValue` into a token stream that produces an `AttributeValue`.
///
/// This function mirrors the logic in `HtmlElement::ToTokens` for converting
/// attribute values, but always wraps the result as an `AttributeValue` variant
/// suitable for passing to `AttributeValue::merge_class`.
///
/// # Arguments
///
/// - `&HtmlAttrValue` - The attribute value to convert.
/// - `&str` - The attribute key name (used for event detection).
/// - `bool` - Whether this is a component attribute.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Token stream that evaluates to an `AttributeValue`.
pub(crate) fn attr_value_to_attribute_value_tokens(
    ctx: &AttrValueContext<'_>,
) -> proc_macro2::TokenStream {
    let value: &HtmlAttrValue = ctx.get_value();
    let key_str: &str = ctx.get_key_str();
    let is_component: bool = ctx.get_is_component();
    match value {
        HtmlAttrValue::Expr(expr) => {
            if let Some(event_name_str) = key_str.strip_prefix(EVENT_ATTR_PREFIX) {
                if is_component {
                    quote! {
                        ::euv::CallbackNamedAdapter::new(#expr, #key_str).into()
                    }
                } else {
                    quote! {
                        ::euv::EventNamedAdapter::new(#expr, #event_name_str).into()
                    }
                }
            } else if key_str == ATTR_KEY_CHILDREN {
                quote! { ::euv::AttributeValue::Dynamic(Box::new(#expr)) }
            } else if key_str == ATTR_KEY_INNER_HTML {
                // `inner_html:` accepts either a `String` / `&str` for a
                // static payload or a `Signal<String>` for a reactive
                // one. The adapter's `From` impls disambiguate at compile
                // time. We pass the raw expression through so the user
                // gets the same Rust type inference they'd see from
                // `let _: AttributeValue = my_html.into()`.
                quote! { ::euv::InnerHtmlAdapter::new(#expr).into() }
            } else {
                quote! {
                    ::euv::AttrValueAdapter::new(#expr).into()
                }
            }
        }
        HtmlAttrValue::If(_) | HtmlAttrValue::Match(_) => {
            quote! { #value }
        }
        HtmlAttrValue::Style(props) => {
            let has_conditional: bool =
                props
                    .iter()
                    .any(|(_, style_value): &(String, HtmlStylePropValue)| {
                        matches!(
                            style_value,
                            HtmlStylePropValue::If(_) | HtmlStylePropValue::Match(_)
                        )
                    });
            if has_conditional {
                quote! { #value }
            } else {
                quote! { ::euv::AttributeValue::Text(#value) }
            }
        }
        HtmlAttrValue::Classes(_) | HtmlAttrValue::Styles(_) => {
            quote! { #value }
        }
    }
}

/// Converts a style-related `HtmlAttrValue` into a token stream that produces
/// an `AttributeValue`.
///
/// Style values are wrapped in `AttributeValue::Text(...)` for static strings,
/// or kept as `AttributeValue::Signal(...)` for reactive style attributes.
///
/// # Arguments
///
/// - `&HtmlAttrValue` - The style attribute value to convert.
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Token stream that evaluates to an `AttributeValue`.
pub(crate) fn style_value_to_attribute_value_tokens(
    value: &HtmlAttrValue,
) -> proc_macro2::TokenStream {
    match value {
        HtmlAttrValue::Style(props) => {
            let has_conditional: bool =
                props
                    .iter()
                    .any(|(_, style_value): &(String, HtmlStylePropValue)| {
                        matches!(
                            style_value,
                            HtmlStylePropValue::If(_) | HtmlStylePropValue::Match(_)
                        )
                    });
            if has_conditional {
                quote! { #value }
            } else {
                quote! { ::euv::AttributeValue::Text(#value) }
            }
        }
        HtmlAttrValue::If(_) | HtmlAttrValue::Match(_) => {
            quote! { #value }
        }
        HtmlAttrValue::Expr(expr) => {
            quote! { ::euv::AttributeValue::Text(#expr.to_string()) }
        }
        HtmlAttrValue::Classes(_) | HtmlAttrValue::Styles(_) => {
            quote! { #value }
        }
    }
}

/// Extracts the clean attribute key string from a token stream.
///
/// Handles two token formats:
/// - `Ident` tokens: `key.to_string()` may include `r#` prefix which is stripped.
/// - `LitStr` tokens: `key.to_string()` includes surrounding quotes which are stripped.
///
/// # Arguments
///
/// - `&proc_macro2::TokenStream` - The token stream representing an attribute key.
///
/// # Returns
///
/// - `String` - The clean attribute key string.
pub(crate) fn extract_attr_key_string(key: &proc_macro2::TokenStream) -> String {
    let raw: String = key.to_string().replace(CHAR_SPACE, STR_EMPTY);
    if raw.starts_with(CHAR_DOUBLE_QUOTE) && raw.ends_with(CHAR_DOUBLE_QUOTE) {
        raw[1..raw.len() - 1].to_string()
    } else {
        raw.strip_prefix(RAW_IDENT_PREFIX)
            .unwrap_or(&raw)
            .to_string()
    }
}

/// Converts an `HtmlAttrValue` into a token stream that produces an `AttributeValue`
/// for use inside an `AttributeEntry::new()` call.
///
/// This is the shared conversion logic used by both `HtmlElement::to_tokens` and
/// `HtmlDynamicTag::to_tokens` to avoid duplicating the attribute value dispatch.
///
/// # Arguments
///
/// - `&HtmlAttrValue` - The attribute value to convert.
/// - `&str` - The attribute key name (used for event and special key detection).
///
/// # Returns
///
/// - `proc_macro2::TokenStream` - Token stream that evaluates to an `AttributeValue`.
pub(crate) fn attr_value_to_entry_value_tokens(
    ctx: &AttrEntryContext<'_>,
) -> proc_macro2::TokenStream {
    let value: &HtmlAttrValue = ctx.get_value();
    let key_str: &str = ctx.get_key_str();
    match value {
        HtmlAttrValue::Style(props) => {
            let has_conditional: bool = is_style_props_conditional(props);
            if has_conditional {
                quote! { #value }
            } else {
                quote! { ::euv::AttributeValue::Text(#value) }
            }
        }
        HtmlAttrValue::If(_) | HtmlAttrValue::Match(_) => {
            quote! { #value }
        }
        HtmlAttrValue::Classes(_) | HtmlAttrValue::Styles(_) => {
            quote! { #value }
        }
        HtmlAttrValue::Expr(expr) => {
            if let Some(event_name_str) = key_str.strip_prefix(EVENT_ATTR_PREFIX) {
                quote! {
                    ::euv::EventNamedAdapter::new(#expr, #event_name_str).into()
                }
            } else if key_str == ATTR_KEY_CHILDREN {
                quote! { ::euv::AttributeValue::Dynamic(Box::new(#expr)) }
            } else if key_str == ATTR_KEY_INNER_HTML {
                // `inner_html:` accepts either a `String` / `&str` for a
                // static payload or a `Signal<String>` for a reactive
                // one. The adapter's `From` impls disambiguate at compile
                // time. We pass the raw expression through so the user
                // gets the same Rust type inference they'd see from
                // `let _: AttributeValue = my_html.into()`.
                quote! { ::euv::InnerHtmlAdapter::new(#expr).into() }
            } else {
                quote! {
                    ::euv::AttrValueAdapter::new(#expr).into()
                }
            }
        }
    }
}
