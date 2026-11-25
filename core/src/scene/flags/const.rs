/// Empty dirty bitmask.
pub const DIRTY_NONE: u16 = 0;

/// Layout geometry needs recomputation for this node.
pub const DIRTY_LAYOUT: u16 = 1 << 0;

/// Visual paint commands need rebuilding for this node.
pub const DIRTY_PAINT: u16 = 1 << 1;

/// Text measurement and shaping need recomputation.
pub const DIRTY_TEXT: u16 = 1 << 2;

/// Children list mutated.
pub const DIRTY_CHILDREN: u16 = 1 << 3;

/// Scroll offset mutated.
pub const DIRTY_SCROLL: u16 = 1 << 4;

/// A descendant requires layout recomputation.
pub const DIRTY_SUBTREE_LAYOUT: u16 = 1 << 5;

/// A descendant requires repainting.
pub const DIRTY_SUBTREE_PAINT: u16 = 1 << 6;

/// All dirty flags combined for freshly mounted nodes.
pub const DIRTY_ALL: u16 = DIRTY_LAYOUT
    | DIRTY_PAINT
    | DIRTY_TEXT
    | DIRTY_CHILDREN
    | DIRTY_SCROLL
    | DIRTY_SUBTREE_LAYOUT
    | DIRTY_SUBTREE_PAINT;

pub(crate) const DIRTY_LAYOUT_PASS_MASK: u16 =
    DIRTY_LAYOUT + DIRTY_SUBTREE_LAYOUT + DIRTY_TEXT + DIRTY_CHILDREN;

pub(crate) const DIRTY_CHILDREN_CHANGE_MASK: u16 = DIRTY_LAYOUT + DIRTY_PAINT + DIRTY_CHILDREN;

pub(crate) const DIRTY_TEXT_CHANGE_MASK: u16 = DIRTY_LAYOUT + DIRTY_PAINT + DIRTY_TEXT;

pub(crate) const DIRTY_ANCESTOR_LAYOUT_TRIGGER: u16 = DIRTY_LAYOUT + DIRTY_TEXT + DIRTY_CHILDREN;

pub(crate) const DIRTY_ANCESTOR_PAINT_TRIGGER: u16 =
    DIRTY_PAINT + DIRTY_SCROLL + DIRTY_LAYOUT + DIRTY_CHILDREN;
