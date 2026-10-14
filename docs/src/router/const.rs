pub(crate) const ANCHOR_ESCAPE: u8 = b'%';

/// The separator joining a sidebar group's display text to its parent's
/// key prefix, matching euv-ui's `euv_sidebar_item` recursion.
pub(crate) const SIDEBAR_GROUP_KEY_SEPARATOR: char = '/';

/// The key prefix a top-level sidebar group is keyed under (its parent
/// level has no name of its own).
pub(crate) const SIDEBAR_GROUP_KEY_ROOT: &str = "";
