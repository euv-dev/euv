/// The CSS `@media` rule prefix used in serialized media query strings.
pub(crate) const CSS_MEDIA_PREFIX: &str = "@media ";

/// The CSS `@keyframes` rule prefix used in serialized keyframes strings.
pub(crate) const CSS_KEYFRAMES_PREFIX: &str = "@keyframes ";

/// The CSS `@supports` rule prefix used in serialized supports strings.
pub(crate) const CSS_SUPPORTS_PREFIX: &str = "@supports ";

/// The CSS `@layer` rule prefix used in serialized layer strings.
pub(crate) const CSS_LAYER_PREFIX: &str = "@layer ";

/// The CSS `@container` rule prefix used in serialized container strings.
pub(crate) const CSS_CONTAINER_PREFIX: &str = "@container ";

/// The CSS `@property` rule prefix used in serialized property strings.
pub(crate) const CSS_PROPERTY_PREFIX: &str = "@property ";

/// The CSS `@scope` rule prefix used in serialized scope strings.
pub(crate) const CSS_SCOPE_PREFIX: &str = "@scope ";

/// The CSS `@font-face` rule marker used in serialized font-face strings.
pub(crate) const CSS_FONT_FACE_PREFIX: &str = "@font-face";

/// The CSS `@charset` rule marker used in serialized charset strings.
pub(crate) const CSS_CHARSET_PREFIX: &str = "@charset ";

/// The CSS `@import` rule marker used in serialized import strings.
pub(crate) const CSS_IMPORT_PREFIX: &str = "@import ";

/// The CSS `@namespace` rule marker used in serialized namespace strings.
pub(crate) const CSS_NAMESPACE_PREFIX: &str = "@namespace ";

/// The CSS `@page` rule marker used in serialized page strings.
pub(crate) const CSS_PAGE_PREFIX: &str = "@page ";

/// The CSS `@color-profile` rule marker used in serialized color-profile strings.
pub(crate) const CSS_COLOR_PROFILE_PREFIX: &str = "@color-profile ";

/// The CSS `@counter-style` rule marker used in serialized counter-style strings.
pub(crate) const CSS_COUNTER_STYLE_PREFIX: &str = "@counter-style ";

/// The CSS `@font-feature-values` rule marker used in serialized font-feature-values strings.
pub(crate) const CSS_FONT_FEATURE_VALUES_PREFIX: &str = "@font-feature-values ";

/// The CSS `@font-palette-values` rule marker used in serialized font-palette-values strings.
pub(crate) const CSS_FONT_PALETTE_VALUES_PREFIX: &str = "@font-palette-values ";

/// The CSS `@document` rule marker used in serialized document strings.
pub(crate) const CSS_DOCUMENT_PREFIX: &str = "@document ";

/// The CSS `@starting-style` rule marker used in serialized starting-style strings.
pub(crate) const CSS_STARTING_STYLE_PREFIX: &str = "@starting-style";

/// The CSS `@view-transition` rule marker used in serialized view-transition strings.
pub(crate) const CSS_VIEW_TRANSITION_PREFIX: &str = "@view-transition ";

/// The CSS `@position-try` rule marker used in serialized position-try strings.
pub(crate) const CSS_POSITION_TRY_PREFIX: &str = "@position-try ";

/// The CSS `@custom-media` rule marker used in serialized custom-media strings.
pub(crate) const CSS_CUSTOM_MEDIA_PREFIX: &str = "@custom-media ";

/// The CSS `@function` rule marker used in serialized function strings.
pub(crate) const CSS_FUNCTION_PREFIX: &str = "@function ";

/// The CSS pseudo-rule serialization separator between selector and style block.
pub(crate) const CSS_RULE_OPEN: &str = " { ";

/// The CSS property separator between property name and value.
pub(crate) const CSS_PROP_SEPARATOR: &str = ": ";

/// The CSS declaration terminator after each property value.
pub(crate) const CSS_DECL_TERMINATOR: &str = "; ";

/// The space string used for concatenating CSS parts.
pub(crate) const STR_SPACE: &str = " ";

/// The hyphen string used for replacing underscores in kebab-case conversion.
pub(crate) const STR_HYPHEN: &str = "-";

/// The keyword name for the `var` function in class macro syntax.
pub(crate) const VAR: &str = "var";

/// The CSS `var()` function prefix for custom property references.
pub(crate) const CSS_VAR_PREFIX: &str = "var(--";

/// The CSS `var()` function suffix for custom property references.
pub(crate) const CSS_VAR_SUFFIX: &str = ")";

/// The `format` macro name used to detect format! invocations in class property values.
pub(crate) const FORMAT_MACRO: &str = "format";

/// The at-rule keyword name for `media`.
pub(crate) const AT_MEDIA: &str = "media";

/// The at-rule keyword name for `keyframes`.
pub(crate) const AT_KEYFRAMES: &str = "keyframes";

/// The at-rule keyword name for `supports`.
pub(crate) const AT_SUPPORTS: &str = "supports";

/// The at-rule keyword name for `layer`.
pub(crate) const AT_LAYER: &str = "layer";

/// The at-rule keyword name for `container`.
pub(crate) const AT_CONTAINER: &str = "container";

/// The at-rule keyword name for `property`.
pub(crate) const AT_PROPERTY: &str = "property";

/// The at-rule keyword name for `scope`.
pub(crate) const AT_SCOPE: &str = "scope";

/// The at-rule keyword name for `font-face`.
pub(crate) const AT_FONT_FACE: &str = "font-face";

/// The at-rule keyword name for `charset`.
pub(crate) const AT_CHARSET: &str = "charset";

/// The at-rule keyword name for `import`.
pub(crate) const AT_IMPORT: &str = "import";

/// The at-rule keyword name for `namespace`.
pub(crate) const AT_NAMESPACE: &str = "namespace";

/// The at-rule keyword name for `page`.
pub(crate) const AT_PAGE: &str = "page";

/// The at-rule keyword name for `color-profile`.
pub(crate) const AT_COLOR_PROFILE: &str = "color-profile";

/// The at-rule keyword name for `counter-style`.
pub(crate) const AT_COUNTER_STYLE: &str = "counter-style";

/// The at-rule keyword name for `font-feature-values`.
pub(crate) const AT_FONT_FEATURE_VALUES: &str = "font-feature-values";

/// The at-rule keyword name for `font-palette-values`.
pub(crate) const AT_FONT_PALETTE_VALUES: &str = "font-palette-values";

/// The at-rule keyword name for `document`.
pub(crate) const AT_DOCUMENT: &str = "document";

/// The at-rule keyword name for `starting-style`.
pub(crate) const AT_STARTING_STYLE: &str = "starting-style";

/// The at-rule keyword name for `view-transition`.
pub(crate) const AT_VIEW_TRANSITION: &str = "view-transition";

/// The at-rule keyword name for `position-try`.
pub(crate) const AT_POSITION_TRY: &str = "position-try";

/// The at-rule keyword name for `custom-media`.
pub(crate) const AT_CUSTOM_MEDIA: &str = "custom-media";

/// The at-rule keyword name for `function`.
pub(crate) const AT_FUNCTION: &str = "function";

/// The colon character used in CSS pseudo-class/at-rule syntax.
pub(crate) const CHAR_COLON: char = ':';

/// The comma character used in CSS media query lists.
pub(crate) const CHAR_COMMA: char = ',';

/// The left parenthesis character used in CSS functional notation.
pub(crate) const CHAR_LEFT_PAREN: char = '(';

/// The right parenthesis character used in CSS functional notation.
pub(crate) const CHAR_RIGHT_PAREN: char = ')';

/// The left bracket character used in CSS attribute selectors.
pub(crate) const CHAR_LEFT_BRACKET: char = '[';

/// The right bracket character used in CSS attribute selectors.
pub(crate) const CHAR_RIGHT_BRACKET: char = ']';

/// The closing brace character used to terminate CSS rule blocks.
pub(crate) const CHAR_CSS_RULE_CLOSE: char = '}';

/// The at-sign character used to prefix CSS at-rules.
pub(crate) const CHAR_AT: char = '@';
