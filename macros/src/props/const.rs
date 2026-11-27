/// Attribute namespace recognized on fields by `#[derive(Props)]`.
pub(crate) const ATTR_PROP: &str = "prop";

/// Field option inside `#[prop(custom)]` suppressing generation of a fluent setter method when a custom setter exists.
pub(crate) const PROP_OPT_SKIP: &str = "custom";

/// Field option inside `#[prop(default = expr)]` overriding the default initializer expression.
pub(crate) const PROP_OPT_DEFAULT: &str = "default";

/// Type segment name detected to accept `impl Into<String>` ergonomics in generated setters.
pub(crate) const TYPE_NAME_STRING: &str = "String";

/// Compile-time diagnostic when `#[derive(Props)]` is applied to a non-struct or non-named-field struct.
pub(crate) const ERR_PROPS_NAMED_STRUCT_ONLY: &str =
    "#[derive(Props)] can only be derived on structs with named fields";

/// Compile-time diagnostic for unknown options inside `#[prop(...)]`.
pub(crate) const ERR_PROPS_UNKNOWN_OPTION: &str =
    "unsupported #[prop(...)] option; expected `custom` or `default = <expr>`";
