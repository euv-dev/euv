pub(crate) const ANCHOR_ESCAPE: u8 = b'%';

/// The byte length of a `%XX` escape: the escape marker and its two
/// hex digits.
pub(crate) const PERCENT_ESCAPE_LEN: usize = 3;

/// The radix the two digits of a `%XX` escape are written in.
pub(crate) const HEX_RADIX: u32 = 16;

/// How far the high digit of a `%XX` escape is shifted into the byte
/// it decodes to.
pub(crate) const HEX_NIBBLE_SHIFT: u32 = 4;

/// The separator joining a sidebar group's display text to its parent's
/// key prefix, matching euv-ui's `euv_sidebar_item` recursion.
pub(crate) const SIDEBAR_GROUP_KEY_SEPARATOR: char = '/';

/// The key prefix a top-level sidebar group is keyed under (its parent
/// level has no name of its own).
pub(crate) const SIDEBAR_GROUP_KEY_ROOT: &str = "";
