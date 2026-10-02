/// Debug-format label for the `Element` variant of [`VirtualNode`](crate::VirtualNode)(crate::VirtualNode)(crate::VirtualNode)(crate::VirtualNode)(crate::VirtualNode)(crate::VirtualNode).
///
/// Matches the variant name so a printed tree lines up with the source
/// expression that produced it.
pub(crate) const DEBUG_NAME_ELEMENT: &str = "Element";

/// Debug-format label for the `Text` variant of [`VirtualNode`](crate::VirtualNode).
pub(crate) const DEBUG_NAME_TEXT: &str = "Text";

/// Debug-format label for the `Fragment` variant of [`VirtualNode`](crate::VirtualNode).
pub(crate) const DEBUG_NAME_FRAGMENT: &str = "Fragment";

/// Debug-format label for the `Dynamic` variant of [`VirtualNode`](crate::VirtualNode).
///
/// The dynamic node's render function is intentionally elided: it is a
/// closure, and printing one would add noise without identifying which
/// node it belongs to.
pub(crate) const DEBUG_NAME_DYNAMIC: &str = "Dynamic";

/// Debug-format label for the `Empty` variant of [`VirtualNode`](crate::VirtualNode).
pub(crate) const DEBUG_NAME_EMPTY: &str = "Empty";

/// Debug-format field label for the `tag` of an `Element` node.
pub(crate) const DEBUG_FIELD_TAG: &str = "tag";

/// Debug-format field label for the `attributes` of an `Element` node.
pub(crate) const DEBUG_FIELD_ATTRIBUTES: &str = "attributes";

/// Debug-format field label for the `children` of an `Element` node.
pub(crate) const DEBUG_FIELD_CHILDREN: &str = "children";

/// Debug-format field label for the `key` of an `Element` node.
pub(crate) const DEBUG_FIELD_KEY: &str = "key";

/// Debug-format field label for the `props` of an `Element` node.
///
/// The props payload is skipped in the debug output; this label is kept
/// alongside the other field labels so the field order documented here
/// matches the struct definition.
pub(crate) const DEBUG_FIELD_PROPS: &str = "props";
