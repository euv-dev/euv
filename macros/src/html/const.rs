/// Environment variable name for the project manifest directory.
pub(crate) const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

/// Source directory name within a Cargo project.
pub(crate) const SRC_DIR: &str = "src";

/// Rust source file extension.
pub(crate) const RUST_FILE_EXTENSION: &str = "rs";

/// Attribute name for marking component functions.
pub(crate) const COMPONENT_ATTR: &str = "component";

/// File name for the component registry cache stored in the target directory.
pub(crate) const REGISTRY_CACHE_FILE_NAME: &str = "euv_component_registry_cache";

/// Environment variable name for the Cargo output directory.
pub(crate) const ENV_OUT_DIR: &str = "OUT_DIR";

/// The semicolon character used as a fingerprint separator.
pub(crate) const CHAR_SEMICOLON: char = ';';

/// The newline character used as the fingerprint/data separator in the cache file.
pub(crate) const CHAR_NEWLINE: char = '\n';

/// Attribute key name for CSS class bindings.
pub(crate) const ATTR_KEY_CLASS: &str = "class";

/// Attribute key name for inline style bindings.
pub(crate) const ATTR_KEY_STYLE: &str = "style";

/// Attribute key name for the raw HTML fragment binding
/// (`html! { div { inner_html: "<svg/>" } }`). Triggers
/// `AttributeValue::InnerHtml(...)` instead of `Text` so the renderer
/// routes the value through `Element::set_inner_html` rather than
/// `set_attribute`, which is what makes `<script>` tags inside the
/// payload actually execute (matching React's
/// `dangerouslySetInnerHTML` semantics).
pub(crate) const ATTR_KEY_INNER_HTML: &str = "inner_html";

/// Attribute key name for children slot bindings.
pub(crate) const ATTR_KEY_CHILDREN: &str = "children";

/// Prefix for event handler attribute keys (e.g., `onclick`, `onchange`).
pub(crate) const EVENT_ATTR_PREFIX: &str = "on";

/// The Rust raw identifier prefix.
pub(crate) const RAW_IDENT_PREFIX: &str = "r#";

/// The hyphen character used in kebab-case tag names.
pub(crate) const CHAR_HYPHEN: char = '-';

/// The space character used in CSS string formatting.
pub(crate) const CHAR_SPACE: char = ' ';

/// The CSS declaration terminator character.
pub(crate) const CHAR_CSS_DECL_TERMINATOR: char = ';';

/// The double quote character used for string literal token detection.
pub(crate) const CHAR_DOUBLE_QUOTE: char = '"';

/// Error message when HTML root content is not a valid element or expression.
pub(crate) const ERR_EXPECTED_ELEMENT: &str =
    "expected an element, string literal, if, match, for, or expression";

/// Error message when an unexpected token is encountered inside an HTML element.
pub(crate) const ERR_UNEXPECTED_TOKEN_IN_ELEMENT: &str = "unexpected token in HTML element";

/// Error message when an unexpected token is encountered in HTML children.
pub(crate) const ERR_UNEXPECTED_TOKEN_IN_HTML: &str = "unexpected token in HTML";

/// Error message when an unexpected token is encountered inside a dynamic component.
pub(crate) const ERR_UNEXPECTED_TOKEN_IN_DYNAMIC_COMPONENT: &str =
    "unexpected token in dynamic component";

/// Type name for `VirtualNode`, used to determine the else default value in component props if-chains.
pub(crate) const TYPE_VIRTUAL_NODE: &str = "VirtualNode";

/// Type name identifier for `VirtualNode`, used to detect `VirtualNode<T>` parameter types.
pub(crate) const VIRTUAL_NODE_TYPE: &str = "VirtualNode";

/// Attribute key name for the `key` binding used for element identity in keyed updates.
pub(crate) const ATTR_KEY_KEY: &str = "key";

/// Empty string constant used as default placeholder for unset attribute values.
pub(crate) const STR_EMPTY: &str = "";

/// Cargo manifest file name.
pub(crate) const CARGO_TOML: &str = "Cargo.toml";

/// Workspace section header in Cargo.toml.
pub(crate) const WORKSPACE_SECTION: &str = "[workspace]";

/// Dependencies section key in Cargo.toml.
pub(crate) const DEPENDENCIES: &str = "dependencies";

/// Workspace dependencies section key in Cargo.toml.
pub(crate) const WORKSPACE_DEPENDENCIES: &str = "workspace.dependencies";

/// Path field key in dependency table.
pub(crate) const PATH_KEY: &str = "path";

/// Workspace field key in dependency table.
pub(crate) const WORKSPACE_KEY: &str = "workspace";
