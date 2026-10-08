/// The DOM `id` attribute value for the shared `<style>` element used by euv.
///
/// This ID is used to locate or create the `<style>` element in the document `<head>`
/// where all dynamically generated CSS rules are appended.
pub(crate) const EUV_CSS_INJECTED_ID: &str = "euv-css-injected";

/// The HTML `style` tag name used when creating a `<style>` element in the DOM.
pub(crate) const STYLE_TAG: &str = "style";

/// The callback event name used as a default for component prop event handlers.
///
/// When a closure is passed as a component prop via `IntoCallbackAttribute`,
/// it is wrapped with this generic event name. The actual DOM event type is
/// later resolved via `EventAdapter::into_attribute`.
pub(crate) const CALLBACK_EVENT_NAME: &str = "callback";

/// The CSS pseudo-rule serialization separator between selector and style block.
///
/// Used by `Css::parse_pseudo_rules` and `Css::parse_media_rules` to locate
/// the boundary between the selector and the style declarations in the
/// compact serialization format produced by the `class!` macro.
pub(crate) const CSS_RULE_OPEN: &str = " { ";

/// The CSS `@media` rule prefix used in serialized media query strings.
///
/// Used by `Css::parse_media_rules` to identify and extract media query
/// blocks from the compact serialization format produced by the `class!` macro.
pub(crate) const CSS_MEDIA_PREFIX: &str = "@media ";

/// The space string used in class/style name merging.
pub(crate) const CHAR_SPACE: &str = " ";

/// The CSS property separator string (name: value).
pub(crate) const CSS_PROP_SEPARATOR: &str = ": ";

/// The CSS declaration terminator character.
pub(crate) const CHAR_CSS_DECL_TERMINATOR: char = ';';

/// The CSS rule closing brace character.
pub(crate) const CHAR_CSS_RULE_CLOSE: char = '}';

/// The CSS class selector prefix character.
pub(crate) const CHAR_CSS_CLASS_PREFIX: char = '.';

/// The CSS rule open brace format string used when injecting style rules into the DOM.
///
/// Used by `Css::inject_style` to format CSS class rules and pseudo-class rules
/// as `.class-name { style }` or `.class-name:hover { style }`.
pub(crate) const CSS_RULE_OPEN_FORMAT: &str = " { ";

/// The CSS rule close brace format string used when injecting style rules into the DOM.
///
/// Used by `Css::inject_style` to close the declaration block of a CSS rule.
pub(crate) const CSS_RULE_CLOSE_FORMAT: &str = " }";

/// The newline character used as a separator between CSS rules in injected style text.
///
/// Each rule (class rule, pseudo rule, or media rule) is separated by a newline
/// when appended to the shared `<style>` element.
pub(crate) const CHAR_CSS_RULE_SEPARATOR: char = '\n';

/// The backslash character used for escaping special characters in CSS selectors.
pub(crate) const CHAR_CSS_ESCAPE: char = '\\';

/// The hyphen character used in CSS class names and selectors.
pub(crate) const CHAR_HYPHEN: char = '-';

/// The underscore character used in CSS class names and identifiers.
pub(crate) const CHAR_UNDERSCORE: char = '_';

/// The FNV-1a offset basis used to derive stable class-name suffixes from parameter values.
pub(crate) const CLASS_PARAM_HASH_FNV_OFFSET: u64 = 14695981039346656037;

/// The FNV-1a prime used to derive stable class-name suffixes from parameter values.
pub(crate) const CLASS_PARAM_HASH_FNV_PRIME: u64 = 1099511628211;
