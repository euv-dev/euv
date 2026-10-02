/// Debug-format label for the [`NodeRef`](crate::NodeRef)(crate::NodeRef)(crate::NodeRef) debug struct.
///
/// Used as the struct name in the derived `Debug` output, so a printed
/// `NodeRef` reads `NodeRef { is_set: true }` rather than showing the
/// generic parameter soup the automatic derive would produce.
pub(crate) const DEBUG_NAME_NODE_REF: &str = "NodeRef";

/// Debug-format field label for the `is_set` field of a [`NodeRef`](crate::NodeRef).
///
/// The field reports whether the renderer has already populated the
/// handle with a live DOM element, which is the only piece of state a
/// reader needs when debugging a `ref:` binding.
pub(crate) const DEBUG_FIELD_IS_SET: &str = "is_set";
