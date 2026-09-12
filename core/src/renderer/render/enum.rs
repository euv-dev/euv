use super::*;

/// A single op in the keyed-patch move plan, computed by
/// [`compute_child_ops_plan`](super::compute_child_ops_plan).
///
/// The enum is consumed verbatim by `patch_children_keyed`, which
/// translates each variant into a [`crate::dom_ops::ChildOp`]. The
/// plan is generated in execution order: every `Remove` for a
/// disappearing old key is emitted before any `Move` / `Insert`, so
/// the renderer's removal pass can detach stale nodes first. After
/// the removes, the planner walks `new_keys` in order, emitting
/// `Keep` / `MoveBefore` / `InsertBefore`. Each non-`Keep` variant
/// anchors relative to the previously-emitted child's `new_index`,
/// which guarantees the `insert_before` reference is a node already
/// at its final DOM position (and therefore not about to be detached
/// by a pending remove).
///
/// `new_index` values refer to positions in the `new_keys` slice
/// passed to the planner (and to the post-patch DOM). `old_index`
/// values refer to positions in the `old_keys` slice.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChildOpPlan {
    /// `new_index` is in the LIS; its live DOM position is already
    /// `new_index`. No child-list op is needed for this child.
    Keep {
        /// Position of this child in `new_keys` (and in the post-
        /// patch DOM).
        new_index: usize,
    },
    /// `new_index` is reused from `old` (key match) but is NOT in
    /// the LIS; its live DOM position needs to be `insert_before`d
    /// relative to the `before` anchor.
    MoveBefore {
        /// Position of this child in `new_keys` (and in the post-
        /// patch DOM).
        new_index: usize,
        /// The child whose live DOM node should serve as the
        /// `insert_before` reference. `None` means append to the
        /// end of the parent. The anchor is chosen from the LIS-
        /// preserved children (or the parent's tail if no LIS child
        /// follows `new_index`).
        before: Option<usize>,
    },
    /// `new_index` is a brand-new child; the renderer must create
    /// its DOM node and `insert_before` it relative to the `before`
    /// anchor.
    InsertBefore {
        /// Position of this child in `new_keys` (and in the post-
        /// patch DOM).
        new_index: usize,
        /// Same anchor semantics as [`ChildOpPlan::MoveBefore`].
        before: Option<usize>,
    },
    /// `old_index` is in `old` but not in `new`; the renderer must
    /// detach its live DOM node. This op is emitted first in the
    /// plan so that subsequent `MoveBefore` / `InsertBefore` ops do
    /// not anchor against a soon-to-be-detached reference.
    Remove {
        /// Position of this child in `old_keys`.
        old_index: usize,
    },
}
