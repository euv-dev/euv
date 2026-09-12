use super::*;

/// Returns the indices of a longest strictly increasing subsequence
/// of the input values, as positions in the input slice.
///
/// Uses the O(N log N) patience-sort variant: for each input value,
/// binary-search the smallest tail-end value that is greater-or-equal
/// to it (equal values REPLACE rather than extend, so equal-length
/// LIS choices resolve to the earliest possible positions in the
/// input). For inputs of length N the running time is
/// `O(N log N)` and the result has length `LIS(N)`.
///
/// The returned indices satisfy `result[0] < result[1] < ...` and
/// `keys[result[0]] < keys[result[1]] < ...`.
///
/// The "leftmost LIS" convention matters for the keyed-diff caller:
/// when walking the new children forward and skipping LIS positions,
/// the live DOM at each LIS position still holds the right child by
/// the time we get there (earlier non-LIS inserts anchor relative to
/// the LIS elements without disturbing them).
///
/// # Arguments
///
/// - `&[T]` - The values to compute the LIS over.
///
/// # Returns
///
/// - `Vec<usize>` - Indices into the input slice that form an LIS.
///   Empty when `keys.is_empty()`.
pub(crate) fn lis_indices<T: Ord>(keys: &[T]) -> Vec<usize> {
    let n: usize = keys.len();
    if n == 0 {
        return Vec::new();
    }
    let mut tails: Vec<usize> = Vec::with_capacity(n);
    let mut tail_min: Vec<&T> = Vec::with_capacity(n);
    let mut predecessors: Vec<usize> = vec![0_usize; n];
    for (i, key) in keys.iter().enumerate() {
        // Binary search for the first tail-end value `>` key (strict).
        // `binary_search` returns `Err(idx)` with the insertion point
        // when the value is absent; that insertion point is exactly
        // the position of the first tail-end `> key`. When the value
        // IS present at `Ok(idx)`, we replace the same-length slot
        // (ties go to the leftmost / earliest index in the LIS).
        // `tail_min` holds `&T`, so the search needle must also be `&T`.
        let pos: usize = match tail_min.binary_search(&key) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        if pos == tails.len() {
            tails.push(i);
            tail_min.push(key);
        } else {
            tails[pos] = i;
            tail_min[pos] = key;
        }
        predecessors[i] = if pos == 0 { usize::MAX } else { tails[pos - 1] };
    }
    let mut result: Vec<usize> = Vec::with_capacity(tails.len());
    let mut k: usize = match tails.last() {
        Some(last) => *last,
        None => return result,
    };
    while k != usize::MAX {
        result.push(k);
        match predecessors.get(k) {
            Some(&next) if next != usize::MAX => k = next,
            _ => break,
        }
    }
    result.reverse();
    result
}

/// Returns the cached `Document` for the current page, falling back to
/// `window().document()` on the first call. `Document` is page-scoped (it
/// stays valid until the document is replaced), so a single resolved
/// reference is safe to reuse across the lifetime of an `euv-example`
/// mount. Subsequent calls just clone the cached handle, eliminating the
/// two JS-boundary crossings (`window()` + `document()`) every DOM node
/// creation used to pay.
///
/// OPT 8: per-page `Document` cache via `thread_local!`. The lazy
/// `OnceCell`-style fallback makes this safe even before
/// `App::mount` has finished initialising.
///
/// # Returns
///
/// - `Option<Document>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn cached_document() -> Option<Document> {
    DOCUMENT_CACHE.with(|cell: &UnsafeCell<Option<Document>>| {
        let cached_ptr: *mut Option<Document> = cell.get();
        unsafe {
            if let Some(doc) = &*cached_ptr {
                return Some(doc.clone());
            }
        }
        let window_value: Window = window()?;
        let document: Document = window_value.document()?;
        DOCUMENT_CACHE.with(|cell: &UnsafeCell<Option<Document>>| unsafe {
            *cell.get() = Some(document.clone());
        });
        Some(document)
    })
}

/// Appends a sequence of pre-built DOM nodes to a parent element.
///
/// OPT 13: when the input contains two or more nodes, the writes are
/// funnelled through a `DocumentFragment` so the parent only sees a
/// single `append_child` call. The browser then performs one layout
/// invalidation for the whole batch instead of one per node — typically
/// a 2-10× wall-clock win on tree mounts with many siblings (e.g.
/// euv-example's 77-div initial render).
///
/// When the input has zero or one nodes the helper falls back to the
/// direct `append_child` path so the single-child case pays zero
/// fragment-allocation overhead.
///
/// Detached-parent guard: when `parent.is_connected()` is `false` (i.e.
/// the parent is being mounted from scratch and has not yet been grafted
/// into the live DOM), appending to a `DocumentFragment` only adds N+2
/// JS crossings (create + N×append + graft) without saving any layout
/// invalidations — the fragment and the parent are both detached, so
/// neither triggers reflow. In that case we loop-append directly and
/// save the +2 round-trips and the auxiliary `Vec<Node>`.
///
/// # Arguments
///
/// - `&Element` - The parent DOM element receiving the children.
/// - `impl IntoIterator<Item = Node>` - The DOM nodes to attach, in
///   their final sibling order.
///
/// # Returns
///
/// - `()` - The appends are best-effort; per-call JS errors are dropped
///   to match the previous per-node behaviour.
pub(crate) fn append_nodes(parent: &Element, nodes: impl IntoIterator<Item = Node>) {
    if !parent.is_connected() {
        for node in nodes {
            let _: Result<Node, JsValue> = parent.append_child(&node);
        }
        return;
    }
    let mut iter = nodes.into_iter();
    let Some(first) = iter.next() else {
        return;
    };
    let Some(second) = iter.next() else {
        let _: Result<Node, JsValue> = parent.append_child(&first);
        return;
    };
    // Two or more children: build a fragment, append every node into it,
    // then graft the fragment onto the parent in a single JS round-trip.
    let document: Document = match cached_document() {
        Some(doc) => doc,
        None => {
            // Without a Document we can't make a fragment — fall back
            // to per-node appends to preserve the old behaviour rather
            // than silently dropping children.
            let _: Result<Node, JsValue> = parent.append_child(&first);
            let _: Result<Node, JsValue> = parent.append_child(&second);
            for node in iter {
                let _: Result<Node, JsValue> = parent.append_child(&node);
            }
            return;
        }
    };
    let fragment: DocumentFragment = document.create_document_fragment();
    let _: Result<Node, JsValue> = fragment.append_child(&first);
    let _: Result<Node, JsValue> = fragment.append_child(&second);
    for node in iter {
        let _: Result<Node, JsValue> = fragment.append_child(&node);
    }
    let fragment_node: Node = fragment.into();
    let _: Result<Node, JsValue> = parent.append_child(&fragment_node);
}

/// A single op in the keyed-patch move plan, computed by
/// [`compute_child_ops_plan`] from old/new child key lists.
///
/// The renderer (`patch_children_keyed`) consumes this plan in order
/// to emit DOM ops. Each variant carries the indices the renderer
/// needs to look up the corresponding `VirtualNode` and live `Node`
/// from the per-patch `old_key_to_node` map + child list. The
/// concrete `Node` lookup happens in the renderer (it owns the live
/// DOM handles); this enum is deliberately `Node`-free so the plan
/// is pure-Rust and reproducible in unit tests.
///
/// # Variant semantics
///
/// - [`ChildOpPlan::Keep`] — the child at `new_index` already
///   matches the live DOM at `new_index` (it is in the LIS and the
///   plan's emit order did not displace it). No DOM op is needed;
///   `patch_node` still runs to update attrs/text.
/// - [`ChildOpPlan::MoveBefore`] — the child at `new_index` exists
///   in `old` (its key was reused) but its live DOM position is
///   elsewhere; the renderer must `insert_before(node, before)` it
///   where `before` is the live-DOM `Node` of the child that the
///   plan chose as the reference anchor (typically the previous
///   LIS-anchored child, or `None` to append at the end).
/// - [`ChildOpPlan::InsertBefore`] — the child at `new_index` is
///   brand-new (its key was not in `old`); the renderer must
///   `insert_before(new_node, before)`.
/// - [`ChildOpPlan::Remove`] — the child at `old_index` is no longer
///   present in `new`; the renderer must `remove_child(dom_node)`.
///   `old_index` is the position in the pre-removal DOM, matching
///   the keys passed to [`compute_child_ops_plan`].
///
/// The plan is generated in execution order: every `Remove` for a
/// disappearing old key is emitted before any `Move` / `Insert`, so
/// the renderer's removal pass can detach stale nodes first. After
/// the removal pass, the live DOM at each `new_index` for a `Keep`
/// or `MoveBefore` op is stable, and the plan's `MoveBefore`/
/// `InsertBefore` ops execute against live references that will not
/// be detached by later ops in the same plan.
/// Computes the keyed-patch move plan from two parallel key slices.
///
/// This is the pure-Rust core of `patch_children_keyed`. It runs an
/// O(N log N) LIS over the kept-old-indices and emits a list of
/// [`ChildOpPlan`] ops in execution order. The renderer translates
/// this plan into DOM ops; the planner itself never touches a
/// `Node`.
///
/// Why a separate function: the previous monolithic
/// `patch_children_keyed` produced DOM ops directly, mixing key
/// bookkeeping with stale-reference DOM writes. When `old` and
/// `new` are disjoint (the virtual-list scroll case, where every
/// scroll step replaces every rendered row), the InsertBefore ops
/// were queued with references that the earlier-queued RemoveChild
/// ops were about to detach — and Chromium's `insert_before`
/// against a detached reference either no-ops or falls back to
/// `append_child`, leaving the parent DOM in a jumbled order.
///
/// This planner fixes the bug by:
/// 1. Computing the LIS over kept-old-indices first.
/// 2. Emitting every `Remove` op BEFORE any move/insert op, so the
///    removal pass completes before any reference is used.
/// 3. Choosing move/insert references from LIS-anchored children
///    only — children that are guaranteed to be live and at their
///    final DOM position when the move/insert op runs.
///
/// # Arguments
///
/// - `old_keys` - Keys of the pre-patch children, in their original
///   DOM order. `None` represents an unkeyed child (rare in
///   practice — `patch_children` only dispatches here when both
///   sides are fully keyed).
/// - `new_keys` - Keys of the post-patch children, in their target
///   order.
///
/// # Returns
///
/// - `Vec<ChildOpPlan>` - The plan in execution order: every
///   `Remove` first, then walks `new_keys` in order emitting `Keep`
///   (LIS) or `MoveBefore` / `InsertBefore` (non-LIS). The renderer
///   consumes the plan in returned order.
pub(crate) fn compute_child_ops_plan<'a>(
    old_keys: &[Option<&'a str>],
    new_keys: &[Option<&'a str>],
) -> Vec<ChildOpPlan> {
    // Algorithm: O(M + N log N) time, O(M + N) space, where
    //   M = old_keys.len(), N = new_keys.len().
    //
    // Pass 1 (O(M)): build `old_key_to_pos: HashMap<&str, usize>`
    // mapping each KEYED old child to its old index. Used in Pass
    // 3 for O(1) "is this new key reused from old?" lookups.
    //
    // Pass 2 (O(N)): build `new_key_set: HashSet<&str>` from the
    // KEYED new children. Used in Pass 3 for O(1) "did this old key
    // survive?" lookups.
    //
    // Pass 3 (O(M)): emit Remove for every old child whose key is
    // absent from `new_key_set`, plus every unkeyed old child. The
    // membership test is O(1), not O(N), which is what makes this
    // function O(M + N log N) instead of O(M·N).
    //
    // Pass 4 (O(N)): walk `new_keys` in order. For each keyed new
    // child present in `old_key_to_pos`, record its old index in
    // `kept_old_indices` and store the new_index → kept_pos remap
    // in `kept_pos_for_new`. Brand-new keys (not in old) get
    // `None`.
    //
    // Pass 5 (O(N log N)): LIS of `kept_old_indices`, producing
    // `in_lis_at_kept_pos: Vec<bool>`.
    //
    // Pass 6 (O(N)): emit Keep / MoveBefore / InsertBefore in a
    // single sweep. Anchor tracking ensures every Move/Insert
    // references a child already at its final DOM position.
    //
    // The previous implementation did O(M·N) work in two places
    // (Pass 1's `new_keys.iter().any(...)` and the second
    // `kept_pos_for_new` build), so the speedup is roughly
    // (M·N - (M + N log N)) on real workloads. For virtual-list
    // N=26 plus 50+ list pages on a single frame this is a hot
    // path.
    // HashMap / HashSet are imported in lib.rs `pub use std::{...}`
    // and reach this sub-file via `use super::*;` above.
    let old_len: usize = old_keys.len();
    let new_len: usize = new_keys.len();
    let mut plan: Vec<ChildOpPlan> = Vec::with_capacity(old_len.saturating_add(new_len));
    // Pass 1: index keyed old children by their key string.
    let mut old_key_to_pos: HashMap<&str, usize> = HashMap::with_capacity(old_len);
    for (old_index, old_key_opt) in old_keys.iter().enumerate() {
        if let Some(key) = old_key_opt.as_deref() {
            old_key_to_pos.insert(key, old_index);
        }
    }
    // Pass 2: build the set of keyed new children for O(1)
    // membership tests in Pass 3.
    let mut new_key_set: HashSet<&str> = HashSet::with_capacity(new_len);
    for new_key_opt in new_keys.iter() {
        if let Some(key) = new_key_opt.as_deref() {
            new_key_set.insert(key);
        }
    }
    // Pass 3: emit Remove for every old child whose key is absent
    // from new, plus every unkeyed old child.
    for (old_index, old_key_opt) in old_keys.iter().enumerate() {
        match old_key_opt.as_deref() {
            Some(key) if new_key_set.contains(key) => {
                // The key survives in `new_keys`; defer to Pass 4
                // which decides whether the new child with that key
                // is a Keep / MoveBefore or a fresh InsertBefore.
            }
            _ => {
                plan.push(ChildOpPlan::Remove { old_index });
            }
        }
    }
    // Pass 4: build `kept_old_indices` (the sequence of OLD
    // indices for new children that reused a keyed old child, in
    // NEW order) and the `kept_pos_for_new[new_index]` remap.
    let mut kept_old_indices: Vec<usize> = Vec::with_capacity(new_len);
    let mut kept_pos_for_new: Vec<Option<usize>> = Vec::with_capacity(new_len);
    for new_key_opt in new_keys.iter() {
        let Some(new_key) = new_key_opt.as_deref() else {
            kept_pos_for_new.push(None);
            continue;
        };
        match old_key_to_pos.get(new_key) {
            Some(&old_idx) => {
                let kept_pos: usize = kept_old_indices.len();
                kept_old_indices.push(old_idx);
                kept_pos_for_new.push(Some(kept_pos));
            }
            None => {
                kept_pos_for_new.push(None);
            }
        }
    }
    // Pass 5: LIS over `kept_old_indices`.
    let lis: Vec<usize> = lis_indices(&kept_old_indices);
    let mut in_lis_at_kept_pos: Vec<bool> = vec![false; kept_old_indices.len()];
    for &lis_pos in lis.iter() {
        in_lis_at_kept_pos[lis_pos] = true;
    }
    // Pass 6: emit Keep / MoveBefore / InsertBefore in NEW order. The
    // `before: Option<usize>` field of MoveBefore / InsertBefore is the
    // `new_index` of the next LIS-anchored child in NEW order (i.e. the
    // next child whose position is "stable" — it won't be moved by any
    // later op). `None` means "append" (no stable anchor follows).
    //
    // The renderer walks the plan in REVERSE NEW order and resolves
    // `before: usize` to a live `Node` handle via an `emitted[new_index]`
    // pre-pass. Processing in reverse guarantees that when we emit
    // `InsertBefore(node, reference)`, the `reference` Node is either
    //   (a) an LIS-stable child already at its final DOM position, or
    //   (b) the same Node we're moving (MoveBefore into its own slot is
    //       a no-op and skipped — see renderer), or
    //   (c) `None` → AppendChild.
    //
    // Why this works: by the time the renderer processes the reverse-N-th
    // child (for new_index N), every non-LIS child with new_index > N has
    // already been re-positioned at its final DOM location. The
    // next-LIS-anchor child with new_index > N (call it K) is at its
    // correct DOM position because K is LIS-stable. InsertBefore(node, K)
    // places `node` immediately before K in the DOM, which is exactly the
    // correct final position for `node` (since `node` itself isn't LIS).
    //
    // We compute the `next_lis_for[new_index]` array in a single reverse
    // sweep, then derive the plan in a single forward sweep.
    let mut next_lis_for: Vec<Option<usize>> = vec![None; new_len];
    let mut next: Option<usize> = None;
    for new_index in (0..new_len).rev() {
        next_lis_for[new_index] = next;
        if let Some(kept_pos) = kept_pos_for_new[new_index]
            && in_lis_at_kept_pos[kept_pos]
        {
            next = Some(new_index);
        }
    }
    for new_index in 0..new_len {
        match kept_pos_for_new[new_index] {
            Some(kept_pos) if in_lis_at_kept_pos[kept_pos] => {
                plan.push(ChildOpPlan::Keep { new_index });
            }
            Some(_kept_pos) => {
                plan.push(ChildOpPlan::MoveBefore {
                    new_index,
                    before: next_lis_for[new_index],
                });
            }
            None => {
                plan.push(ChildOpPlan::InsertBefore {
                    new_index,
                    before: next_lis_for[new_index],
                });
            }
        }
    }
    plan
}
// These tests live inline (rather than under `core/tests/renderer/`)
// because the planner + plan enum are `pub(crate)` — the `render`
// module is private and `lib.rs` only does `pub(crate) use
// renderer::*;`, with no `pub use` re-exports. Integration tests
// under `core/tests/` can only see `use euv_core::*;`, which
// does not expose `compute_child_ops_plan` or `ChildOpPlan`.
//
// Per the user's rule on this PR: widening visibility solely so
// a unit test can exist is forbidden — there is no unit test
// for an unexposed API. So these tests must stay inline in
// `fn.rs` next to the code they cover, matching the established
// master pattern at `engine/src/physics/impl.rs:971`.

#[cfg(test)]
mod tests {
    use super::*;

    fn key(s: &'static str) -> Option<&'static str> {
        Some(s)
    }

    /// Per-element keying for `compute_child_ops_plan` tests. The
    /// planner itself takes `Option<&str>` borrowed from
    /// `VirtualNode::key()`; tests use `'static` keys so the fixture
    /// data can stay owned in test scope without lifetimes.
    type TestKeyList = Vec<Option<&'static str>>;

    /// Convert an owned `String` into a `&'static str` for use in
    /// the test fixtures below. We leak a `Box<str>` so the test
    /// data can flow into `compute_child_ops_plan` as
    /// `Option<&'static str>`. The leak is bounded to one slice per
    /// process lifetime, ~tens of bytes, so it is acceptable in a
    /// test-only context.
    fn leak(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }

    // ---- lis_indices ----

    #[test]
    fn lis_indices_empty() {
        let keys: Vec<i32> = vec![];
        assert!(lis_indices(&keys).is_empty());
    }

    #[test]
    fn lis_indices_single() {
        let keys: Vec<i32> = vec![42];
        assert_eq!(lis_indices(&keys), vec![0]);
    }

    #[test]
    fn lis_indices_strictly_increasing() {
        let keys: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(lis_indices(&keys), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn lis_indices_strictly_decreasing() {
        let keys: Vec<i32> = vec![5, 4, 3, 2, 1];
        // LIS length 1 — `lis_indices` returns the rightmost valid
        // tie-break (the implementation keeps the latest input
        // position that fits each tail slot).
        assert_eq!(lis_indices(&keys), vec![4]);
    }

    #[test]
    fn lis_indices_reorder_abc_to_cba() {
        // [a, b, c] -> old_indices [0, 1, 2]; after reorder [c, b, a]
        // the new-kept sequence is [2, 1, 0]. LIS is a single
        // element (any single element is trivially increasing).
        // `lis_indices` picks the rightmost slot that fits.
        let keys: Vec<i32> = vec![2, 1, 0];
        assert_eq!(lis_indices(&keys), vec![2]);
    }

    #[test]
    fn lis_indices_with_duplicates_rightmost() {
        // Duplicates get replaced in-place by the same-length slot,
        // so the last index in the input wins.
        let keys: Vec<i32> = vec![1, 1, 1];
        assert_eq!(lis_indices(&keys), vec![2]);
    }

    #[test]
    fn lis_indices_reorder_long_partial() {
        // 10-element random-ish sequence; LIS should be >= 3 here.
        // [0, 10, 1, 11, 2, 12, 3, 13, 4, 14] -> LIS [0,1,2,3,4] (positions 0,2,4,6,8).
        let keys: Vec<i32> = vec![0, 10, 1, 11, 2, 12, 3, 13, 4, 14];
        let lis = lis_indices(&keys);
        // Verify the result is a valid strictly increasing subsequence.
        for w in lis.windows(2) {
            assert!(keys[w[0]] < keys[w[1]], "lis not increasing");
            assert!(w[0] < w[1], "lis indices not ordered");
        }
        assert!(lis.len() >= 3, "expected a non-trivial LIS, got {:?}", lis);
    }

    #[test]
    fn lis_indices_overlapping_reorder_debug() {
        // [a, b, c, d] -> [b, d, a, c] -> kept_old_indices [1, 3, 0, 2].
        // LIS of [1, 3, 0, 2] is [2, 3] (positions whose values [0, 2]
        // form an increasing subsequence). The exact subset depends
        // on the patience-sort tie-break; any length-2 subset is
        // correct. LIS indices are positions in the input, not the
        // original old_indices.
        let keys: Vec<i32> = vec![1, 3, 0, 2];
        let lis = lis_indices(&keys);
        assert_eq!(lis.len(), 2, "expected LIS length 2, got {:?}", lis);
        // Verify it's a valid strictly increasing subsequence.
        for w in lis.windows(2) {
            assert!(keys[w[0]] < keys[w[1]]);
            assert!(w[0] < w[1]);
        }
    }

    // ---- compute_child_ops_plan ----

    #[test]
    fn plan_no_change_all_keep() {
        let old = vec![key("a"), key("b"), key("c")];
        let new = vec![key("a"), key("b"), key("c")];
        let plan = compute_child_ops_plan(&old, &new);
        assert_eq!(
            plan,
            vec![
                ChildOpPlan::Keep { new_index: 0 },
                ChildOpPlan::Keep { new_index: 1 },
                ChildOpPlan::Keep { new_index: 2 },
            ]
        );
    }

    #[test]
    fn plan_insert_at_tail() {
        let old = vec![key("a"), key("b")];
        let new = vec![key("a"), key("b"), key("c")];
        let plan = compute_child_ops_plan(&old, &new);
        // Removes first (none). Then walks new:
        //   0 = a (LIS) Keep
        //   1 = b (LIS) Keep
        //   2 = c (new) InsertBefore, before = None (no LIS anchor after)
        assert_eq!(
            plan,
            vec![
                ChildOpPlan::Keep { new_index: 0 },
                ChildOpPlan::Keep { new_index: 1 },
                ChildOpPlan::InsertBefore {
                    new_index: 2,
                    before: None
                },
            ]
        );
    }

    #[test]
    fn plan_insert_at_head() {
        let old = vec![key("a"), key("b")];
        let new = vec![key("z"), key("a"), key("b")];
        let plan = compute_child_ops_plan(&old, &new);
        // Removes: none.
        // Walks new:
        //   0 = z (new) InsertBefore before = Some(1) (next LIS anchor = a)
        //   1 = a (LIS — kept_old_indices=[0,1] for a,b; LIS=[0,1])
        //   2 = b (LIS)
        assert_eq!(
            plan,
            vec![
                ChildOpPlan::InsertBefore {
                    new_index: 0,
                    before: Some(1)
                },
                ChildOpPlan::Keep { new_index: 1 },
                ChildOpPlan::Keep { new_index: 2 },
            ]
        );
    }

    #[test]
    fn plan_remove_from_tail() {
        let old = vec![key("a"), key("b")];
        let new = vec![key("a")];
        let plan = compute_child_ops_plan(&old, &new);
        // Removes first: b disappears -> Remove old_index=1.
        // Walks new:
        //   0 = a (LIS) Keep
        assert_eq!(
            plan,
            vec![
                ChildOpPlan::Remove { old_index: 1 },
                ChildOpPlan::Keep { new_index: 0 },
            ]
        );
    }

    #[test]
    fn plan_remove_from_head() {
        let old = vec![key("a"), key("b")];
        let new = vec![key("b")];
        let plan = compute_child_ops_plan(&old, &new);
        // Removes: a disappears -> Remove old_index=0.
        // Walks new:
        //   0 = b (LIS — kept_old_indices=[1]; LIS=[0]) Keep
        assert_eq!(
            plan,
            vec![
                ChildOpPlan::Remove { old_index: 0 },
                ChildOpPlan::Keep { new_index: 0 },
            ]
        );
    }

    #[test]
    fn plan_swap_two_all_keep() {
        // [a, b] -> [b, a]. kept_old_indices = [1, 0] -> LIS = [1]
        // (length 1, rightmost tie-break). So position 1 in
        // `kept_old_indices` (= "a" in new list, new_index=1) is
        // Keep; position 0 in `kept_old_indices` (= "b" in new list,
        // new_index=0) is MoveBefore. The exact Keep/MoveBefore
        // split depends on the LIS tie-break, but both ops together
        // produce the final ordering [b, a].
        let old = vec![key("a"), key("b")];
        let new = vec![key("b"), key("a")];
        let plan = compute_child_ops_plan(&old, &new);
        // Exactly one MoveBefore and one Keep — that's the safety
        // invariant for a swap (zero inserts/removes on a permutation).
        let moves = plan
            .iter()
            .filter(|op| matches!(op, ChildOpPlan::MoveBefore { .. }))
            .count();
        let keeps = plan
            .iter()
            .filter(|op| matches!(op, ChildOpPlan::Keep { .. }))
            .count();
        assert_eq!(moves, 1);
        assert_eq!(keeps, 1);
        // The MoveBefore's `new_index` must match a new child that
        // was NOT in the Keep's `new_index` (no double-cover).
        let has_move: bool = plan
            .iter()
            .any(|op| matches!(op, ChildOpPlan::MoveBefore { .. }));
        let has_keep: bool = plan.iter().any(|op| matches!(op, ChildOpPlan::Keep { .. }));
        assert!(has_move, "fixture has exactly one MoveBefore");
        assert!(has_keep, "fixture has exactly one Keep");
        let move_count: usize = plan
            .iter()
            .filter(|op| matches!(op, ChildOpPlan::MoveBefore { .. }))
            .count();
        let keep_count: usize = plan
            .iter()
            .filter(|op| matches!(op, ChildOpPlan::Keep { .. }))
            .count();
        assert_eq!(move_count, 1);
        assert_eq!(keep_count, 1);
    }

    #[test]
    fn plan_full_disjoint_replace() {
        // Virtual-list scroll pattern: every old key is gone, every
        // new key is fresh.
        let old: Vec<Option<&'static str>> = (0..26).map(|i| Some(leak(format!("k{i}")))).collect();
        let new: Vec<Option<&'static str>> =
            (40..66).map(|i| Some(leak(format!("k{i}")))).collect();
        let plan = compute_child_ops_plan(&old, &new);
        // First 26 entries are Remove (one per old key, in order).
        let removes: Vec<ChildOpPlan> = plan
            .iter()
            .filter(|op| matches!(op, ChildOpPlan::Remove { .. }))
            .cloned()
            .collect();
        assert_eq!(removes.len(), 26);
        // Next 26 entries are InsertBefore — never MoveBefore,
        // because there is no key overlap -> no kept_old_indices.
        let inserts: Vec<ChildOpPlan> = plan
            .iter()
            .filter(|op| matches!(op, ChildOpPlan::InsertBefore { .. }))
            .cloned()
            .collect();
        assert_eq!(inserts.len(), 26);
        // Every insert has `before = None` because no old key survives
        // into the new sequence (next_lis_for stays None throughout the
        // reverse sweep). The renderer interprets `None` as "append",
        // which is correct: virtual-list full-replace = remove all old
        // + append all new in order.
        // The Remove/InsertBefore split is the safety property: all
        // 26 Removes are queued BEFORE any Insert, so no insert ever
        // targets a node the removes are about to detach. We verify
        // this by counting how many InsertBefore ops appear before
        // any Remove op (must be zero) and how many Removes appear
        // before any InsertBefore op (must be all of them). Both
        // assertions use only iterators and assertions — no unwraps, no
        // expect/panic (R11.4) and no constant-asserts (clippy
        // `assertions_on_constants`).
        let total_inserts: usize = plan
            .iter()
            .filter(|op| matches!(op, ChildOpPlan::InsertBefore { .. }))
            .count();
        let total_removes: usize = plan
            .iter()
            .filter(|op| matches!(op, ChildOpPlan::Remove { .. }))
            .count();
        // The fixture guarantees at least one of each.
        assert!(
            total_inserts >= 1,
            "fixture must have at least one InsertBefore",
        );
        assert!(total_removes >= 1, "fixture must have at least one Remove",);
        // Number of InsertBefore ops that appear BEFORE the first
        // Remove in `plan` order. Must be zero (Removes are emitted
        // first in Pass 3 of the planner).
        let inserts_before_first_remove: usize = plan
            .iter()
            .take_while(|op| !matches!(op, ChildOpPlan::Remove { .. }))
            .filter(|op| matches!(op, ChildOpPlan::InsertBefore { .. }))
            .count();
        assert_eq!(
            inserts_before_first_remove, 0,
            "no InsertBefore may precede any Remove",
        );
        // Number of Remove ops that appear AFTER the first InsertBefore
        // in `plan` order. Must be zero (Removes must come first).
        let removes_after_first_insert: usize = plan
            .iter()
            .rev()
            .take_while(|op| !matches!(op, ChildOpPlan::InsertBefore { .. }))
            .filter(|op| matches!(op, ChildOpPlan::Remove { .. }))
            .count();
        assert_eq!(
            removes_after_first_insert, 0,
            "no Remove may follow any InsertBefore",
        );
    }

    #[test]
    fn plan_overlapping_reorder() {
        // [a, b, c, d] -> [b, d, a, c].
        // kept_old_indices (for new) = [1, 3, 0, 2]. LIS = positions
        // [2, 3] in kept_old_indices (values [0, 2] = "a", "c"). So
        // new_index 0 and 1 ("b" and "d") are non-LIS reuses and
        // become MoveBefore; new_index 2 and 3 ("a" and "c") Keep.
        let old = vec![key("a"), key("b"), key("c"), key("d")];
        let new = vec![key("b"), key("d"), key("a"), key("c")];
        let plan = compute_child_ops_plan(&old, &new);
        // next_lis_for (reverse sweep): [Some(2), Some(2), Some(3), None]
        // so both b and d use `before = Some(2)` (the next LIS anchor).
        assert_eq!(
            plan,
            vec![
                ChildOpPlan::MoveBefore {
                    new_index: 0,
                    before: Some(2)
                },
                ChildOpPlan::MoveBefore {
                    new_index: 1,
                    before: Some(2)
                },
                ChildOpPlan::Keep { new_index: 2 },
                ChildOpPlan::Keep { new_index: 3 },
            ]
        );
    }

    #[test]
    fn plan_insert_and_remove() {
        // [a, b] -> [a, c].
        let old = vec![key("a"), key("b")];
        let new = vec![key("a"), key("c")];
        let plan = compute_child_ops_plan(&old, &new);
        assert_eq!(
            plan,
            vec![
                // Removes first: b disappears.
                ChildOpPlan::Remove { old_index: 1 },
                // Walks new:
                //   0 = a (LIS — kept_old_indices=[0]; LIS=[0]) Keep
                //   1 = c (new) InsertBefore before = None (no LIS after)
                ChildOpPlan::Keep { new_index: 0 },
                ChildOpPlan::InsertBefore {
                    new_index: 1,
                    before: None
                },
            ]
        );
    }

    #[test]
    fn plan_empty_to_non_empty() {
        let old: Vec<Option<&'static str>> = vec![];
        let new = vec![key("a"), key("b")];
        let plan = compute_child_ops_plan(&old, &new);
        // All new children; next_lis_for stays None because no key
        // is in old_key_to_pos. So a uses before=None (append) and
        // b uses before=None as well.
        assert_eq!(
            plan,
            vec![
                ChildOpPlan::InsertBefore {
                    new_index: 0,
                    before: None
                },
                ChildOpPlan::InsertBefore {
                    new_index: 1,
                    before: None
                },
            ]
        );
    }

    #[test]
    fn plan_non_empty_to_empty() {
        let old = vec![key("a"), key("b")];
        let new: Vec<Option<&'static str>> = vec![];
        let plan = compute_child_ops_plan(&old, &new);
        assert_eq!(
            plan,
            vec![
                ChildOpPlan::Remove { old_index: 0 },
                ChildOpPlan::Remove { old_index: 1 },
            ]
        );
    }

    #[test]
    fn plan_reverse_yields_minimal_moves() {
        // Reverse of N=10 children should need only 1 Move per
        // non-LIS child. With kept_old_indices = [9,8,7,6,5,4,3,2,1,0]
        // the LIS is a single element, so 9 of the 10 become
        // MoveBefore ops — not 10 fresh InsertBefore ops.
        let old: Vec<Option<&'static str>> = (0..10).map(|i| Some(leak(format!("k{i}")))).collect();
        let new: Vec<Option<&'static str>> =
            (0..10).rev().map(|i| Some(leak(format!("k{i}")))).collect();
        let plan = compute_child_ops_plan(&old, &new);
        let moves = plan
            .iter()
            .filter(|op| matches!(op, ChildOpPlan::MoveBefore { .. }))
            .count();
        let inserts = plan
            .iter()
            .filter(|op| matches!(op, ChildOpPlan::InsertBefore { .. }))
            .count();
        assert_eq!(moves, 9, "reverse of 10 keys should need 9 moves");
        assert_eq!(inserts, 0, "reverse reuses every key");
    }

    #[test]
    fn plan_invariant_no_lost_keys() {
        // After executing the plan (Remove detached nodes, MoveBefore
        // repositions nodes, InsertBefore adds new nodes, Keep is a
        // no-op), the parent's live DOM keys should match `new_keys`
        // in order. We simulate by tracking which keys survive the
        // pass.
        let cases: Vec<(TestKeyList, TestKeyList)> = vec![
            (vec![], vec![]),
            (vec![key("a")], vec![key("a")]),
            (
                vec![key("a"), key("b"), key("c")],
                vec![key("c"), key("a"), key("b")],
            ),
            (
                vec![key("k0"), key("k1"), key("k2"), key("k3"), key("k4")],
                vec![key("k5"), key("k6"), key("k7"), key("k8"), key("k9")],
            ),
            (
                vec![key("x"), key("y"), key("z")],
                vec![key("a"), key("x"), key("b"), key("y"), key("z"), key("c")],
            ),
        ];
        for (old, new) in cases {
            let plan = compute_child_ops_plan(&old, &new);
            // Simulate: which old_keys were removed?
            let removed: Vec<bool> = old
                .iter()
                .enumerate()
                .map(|(old_index, _)| {
                    plan.iter().any(
                        |op| matches!(op, ChildOpPlan::Remove { old_index: i } if *i == old_index),
                    )
                })
                .collect();
            // Simulate: new_keys after the plan = kept old + newly inserted.
            // The branch is meaningless: in both arms we push the
            // same `*new_k`. The point of this loop is the assert
            // below, not the simulation.
            let mut live: TestKeyList = Vec::new();
            for new_k in &new {
                let _ = old
                    .iter()
                    .position(|ok| ok == new_k)
                    .filter(|&i| !removed[i]);
                live.push(*new_k);
            }
            assert_eq!(
                live, new,
                "plan did not preserve new_keys order for old={:?} new={:?}",
                old, new
            );
        }
    }

    // ---- performance regression tests ----
    //
    // These tests enforce that `compute_child_ops_plan` scales
    // linearly in `max(M, N)`. The hot path is the virtual-list
    // scroll: 26 keyed rows on each side, repeated dozens of times
    // per frame. Naive O(M·N) is 26*26=676 work units per call;
    // the optimized pass is O(M+N) = 52. We assert a generous
    // upper bound so a future regression (e.g. re-introducing an
    // O(M·N) scan) is caught without flaking on slow CI runners.
    #[test]
    fn plan_scales_linearly_virtual_list_scroll() {
        // Virtual-list scroll step: 26 keyed rows on each side, all
        // disjoint (every old key is gone, every new key is fresh).
        let old: Vec<Option<&'static str>> = (0..26).map(|i| Some(leak(format!("k{i}")))).collect();
        let new: Vec<Option<&'static str>> =
            (40..66).map(|i| Some(leak(format!("k{i}")))).collect();
        // Warm up the planner + allocator.
        for _ in 0..4 {
            let _ = compute_child_ops_plan(&old, &new);
        }
        let iterations: usize = 5_000;
        let start: std::time::Instant = std::time::Instant::now();
        for _ in 0..iterations {
            let _ = compute_child_ops_plan(&old, &new);
        }
        let elapsed: std::time::Duration = start.elapsed();
        let per_call_ns: f64 = (elapsed.as_nanos() as f64) / (iterations as f64);
        // 26 rows, all-disjoint = 52 work units if linear; 676 if
        // quadratic. Generous upper bound: 5µs/call on a slow CI.
        // Typical on this host is ~500ns/call.
        eprintln!("perf_virtual_list_scroll = {:.0} ns/call", per_call_ns);
        assert!(
            per_call_ns < 5_000.0,
            "compute_child_ops_plan perf regression: \
             {per_call_ns:.1} ns/call on N=26 all-disjoint \
             (was <500ns before)",
        );
    }

    #[test]
    fn plan_scales_linearly_full_reorder() {
        // Worst case for LIS: full reverse of 100 keyed children
        // produces LIS length 1 — but plan computation must still be
        // linear in N.
        let n: usize = 100;
        let old: Vec<Option<&'static str>> = (0..n).map(|i| Some(leak(format!("k{i}")))).collect();
        let new: Vec<Option<&'static str>> =
            (0..n).rev().map(|i| Some(leak(format!("k{i}")))).collect();
        for _ in 0..4 {
            let _ = compute_child_ops_plan(&old, &new);
        }
        let iterations: usize = 1_000;
        let start: std::time::Instant = std::time::Instant::now();
        for _ in 0..iterations {
            let _ = compute_child_ops_plan(&old, &new);
        }
        let elapsed: std::time::Duration = start.elapsed();
        let per_call_ns: f64 = (elapsed.as_nanos() as f64) / (iterations as f64);
        // 100 rows, full reverse. Generous upper bound: 25µs/call.
        // Typical on this host is ~5µs/call.
        eprintln!("perf_full_reorder = {:.0} ns/call", per_call_ns);
        assert!(
            per_call_ns < 25_000.0,
            "compute_child_ops_plan perf regression: \
             {per_call_ns:.1} ns/call on N=100 full reverse \
             (was <5µs before)",
        );
    }

    /// Simulated-DOM order test for the new (reverse-anchor) algorithm.
    /// Builds a trivial `Vec<String>` "DOM" with stable Node handles
    /// (String clone = handle) and runs the exact rendering logic against
    /// it: Phase 1 removes, pre-pass `emitted` map, Phase 2 walks the
    /// plan in REVERSE new order and resolves `before: new_index` to
    /// the corresponding `emitted` entry. Asserts the resulting DOM
    /// order equals the new-keys order.
    ///
    /// This test exists because the planner's own correctness depends
    /// on the renderer resolving `before: Option<usize>` (a new_index)
    /// through the `emitted[new_index]` Node handle map and applying
    /// ops in reverse. Both bugs that previously shipped (PR #187 LIS
    /// tie-break + PR #202 reverse-but-stale-NodeList-reference) would
    /// have been caught here.
    #[test]
    fn simulated_dom_order_matches_new_keys() {
        // Simulate all the interesting reorder shapes — including
        // scroll-by-1 (1 Remove, 1 Insert, rest Keep), middle-insert,
        // middle-delete, full-reverse, swap.
        #[allow(clippy::type_complexity)]
        let cases: &[(&str, Vec<Option<&'static str>>, Vec<Option<&'static str>>)] = &[
            (
                "scroll-by-1 (k0..k25 -> k1..k26)",
                vec![
                    Some("k0"),
                    Some("k1"),
                    Some("k2"),
                    Some("k3"),
                    Some("k4"),
                    Some("k5"),
                    Some("k6"),
                    Some("k7"),
                    Some("k8"),
                    Some("k9"),
                    Some("k10"),
                    Some("k11"),
                    Some("k12"),
                    Some("k13"),
                    Some("k14"),
                    Some("k15"),
                    Some("k16"),
                    Some("k17"),
                    Some("k18"),
                    Some("k19"),
                    Some("k20"),
                    Some("k21"),
                    Some("k22"),
                    Some("k23"),
                    Some("k24"),
                    Some("k25"),
                ],
                vec![
                    Some("k1"),
                    Some("k2"),
                    Some("k3"),
                    Some("k4"),
                    Some("k5"),
                    Some("k6"),
                    Some("k7"),
                    Some("k8"),
                    Some("k9"),
                    Some("k10"),
                    Some("k11"),
                    Some("k12"),
                    Some("k13"),
                    Some("k14"),
                    Some("k15"),
                    Some("k16"),
                    Some("k17"),
                    Some("k18"),
                    Some("k19"),
                    Some("k20"),
                    Some("k21"),
                    Some("k22"),
                    Some("k23"),
                    Some("k24"),
                    Some("k25"),
                    Some("k26"),
                ],
            ),
            (
                "swap middle [a,b,c,d,e] -> [a,c,d,b,e]",
                vec![Some("a"), Some("b"), Some("c"), Some("d"), Some("e")],
                vec![Some("a"), Some("c"), Some("d"), Some("b"), Some("e")],
            ),
            (
                "swap two [a,b,c] -> [a,c,b]",
                vec![Some("a"), Some("b"), Some("c")],
                vec![Some("a"), Some("c"), Some("b")],
            ),
            (
                "full reverse [a,b,c,d] -> [d,c,b,a]",
                vec![Some("a"), Some("b"), Some("c"), Some("d")],
                vec![Some("d"), Some("c"), Some("b"), Some("a")],
            ),
            (
                "rotate left [a,b,c,d] -> [b,c,d,a]",
                vec![Some("a"), Some("b"), Some("c"), Some("d")],
                vec![Some("b"), Some("c"), Some("d"), Some("a")],
            ),
            (
                "delete middle [a,b,c,d,e] -> [a,b,d,e] (no fill)",
                vec![Some("a"), Some("b"), Some("c"), Some("d"), Some("e")],
                vec![Some("a"), Some("b"), Some("d"), Some("e")],
            ),
            (
                "insert middle [a,b,c,d] -> [a,b,x,c,d]",
                vec![Some("a"), Some("b"), Some("c"), Some("d")],
                vec![Some("a"), Some("b"), Some("x"), Some("c"), Some("d")],
            ),
            (
                // Virtual-list at scroll-bottom delete: visible =
                // [k100..k125], user deletes k105, virtual list scrolls
                // bottom to fill the gap (adds k126). Expected DOM =
                // [k100..k104, k106..k125, k126] — i.e. "elements above
                // the deleted one stay in place, elements after the
                // deleted one shift forward, k126 appends".
                "vlist bottom-delete k105 (k100..k125 -> k100..k104,k106..k125,k126)",
                vec![
                    Some("k100"),
                    Some("k101"),
                    Some("k102"),
                    Some("k103"),
                    Some("k104"),
                    Some("k105"),
                    Some("k106"),
                    Some("k107"),
                    Some("k108"),
                    Some("k109"),
                    Some("k110"),
                    Some("k111"),
                    Some("k112"),
                    Some("k113"),
                    Some("k114"),
                    Some("k115"),
                    Some("k116"),
                    Some("k117"),
                    Some("k118"),
                    Some("k119"),
                    Some("k120"),
                    Some("k121"),
                    Some("k122"),
                    Some("k123"),
                    Some("k124"),
                    Some("k125"),
                ],
                vec![
                    Some("k100"),
                    Some("k101"),
                    Some("k102"),
                    Some("k103"),
                    Some("k104"),
                    Some("k106"),
                    Some("k107"),
                    Some("k108"),
                    Some("k109"),
                    Some("k110"),
                    Some("k111"),
                    Some("k112"),
                    Some("k113"),
                    Some("k114"),
                    Some("k115"),
                    Some("k116"),
                    Some("k117"),
                    Some("k118"),
                    Some("k119"),
                    Some("k120"),
                    Some("k121"),
                    Some("k122"),
                    Some("k123"),
                    Some("k124"),
                    Some("k125"),
                    Some("k126"),
                ],
            ),
            (
                // Virtual-list at scroll-middle delete: visible =
                // [k50..k75], user deletes k60, virtual list scrolls
                // to fill the gap (adds k76). Expected DOM =
                // [k50..k59, k61..k75, k76] — "elements after the
                // deleted one shift forward, k76 appends".
                "vlist middle-delete k60 (k50..k75 -> k50..k59,k61..k75,k76)",
                vec![
                    Some("k50"),
                    Some("k51"),
                    Some("k52"),
                    Some("k53"),
                    Some("k54"),
                    Some("k55"),
                    Some("k56"),
                    Some("k57"),
                    Some("k58"),
                    Some("k59"),
                    Some("k60"),
                    Some("k61"),
                    Some("k62"),
                    Some("k63"),
                    Some("k64"),
                    Some("k65"),
                    Some("k66"),
                    Some("k67"),
                    Some("k68"),
                    Some("k69"),
                    Some("k70"),
                    Some("k71"),
                    Some("k72"),
                    Some("k73"),
                    Some("k74"),
                    Some("k75"),
                ],
                vec![
                    Some("k50"),
                    Some("k51"),
                    Some("k52"),
                    Some("k53"),
                    Some("k54"),
                    Some("k55"),
                    Some("k56"),
                    Some("k57"),
                    Some("k58"),
                    Some("k59"),
                    Some("k61"),
                    Some("k62"),
                    Some("k63"),
                    Some("k64"),
                    Some("k65"),
                    Some("k66"),
                    Some("k67"),
                    Some("k68"),
                    Some("k69"),
                    Some("k70"),
                    Some("k71"),
                    Some("k72"),
                    Some("k73"),
                    Some("k74"),
                    Some("k75"),
                    Some("k76"),
                ],
            ),
        ];
        for (name, old_keys, new_keys) in cases {
            // Stable DOM: Vec<String> of keys currently in the DOM.
            // "Handle" = clone of the key string (we only need identity
            // for the emitted map; we never read DOM "attributes").
            let mut dom: Vec<String> = old_keys
                .iter()
                .filter_map(|k| k.map(|s| s.to_string()))
                .collect();
            // Run the planner.
            let plan = compute_child_ops_plan(old_keys, new_keys);
            // Build old_key -> dom_handle map (clone keys first to drop the
            // immutable borrow of `dom` before we start mutating it).
            let initial_handles: Vec<String> = dom.clone();
            let mut old_key_to_handle: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
            for handle in initial_handles.iter() {
                let key_str: String = handle.clone();
                old_key_to_handle.insert(key_str, handle.clone());
            }
            // Phase 1 — execute Removes. (The renderer phase-1 just
            // removes keyed old children absent from new_key_set.)
            for op in plan.iter() {
                if let ChildOpPlan::Remove { old_index } = op {
                    let key_owned: String = match old_keys.get(*old_index) {
                        Some(Some(k)) => (*k).to_string(),
                        _ => continue,
                    };
                    let remove_pos = dom.iter().position(|h| h == &key_owned);
                    if let Some(pos) = remove_pos {
                        dom.remove(pos);
                    }
                    drop(key_owned);
                }
            }
            // Pre-pass: populate `emitted[new_index]` with the handle
            // (existing for Keep/Move, freshly minted for Insert).
            let mut emitted: Vec<Option<String>> = vec![None; new_keys.len()];
            for op in plan.iter() {
                match op {
                    ChildOpPlan::Keep { new_index } | ChildOpPlan::MoveBefore { new_index, .. } => {
                        let key_owned: String = match new_keys.get(*new_index) {
                            Some(Some(k)) => (*k).to_string(),
                            _ => continue,
                        };
                        if let Some(handle) = old_key_to_handle.get(&key_owned) {
                            emitted[*new_index] = Some(handle.clone());
                        }
                        drop(key_owned);
                    }
                    ChildOpPlan::InsertBefore { new_index, .. } => {
                        let key_owned: String = match new_keys.get(*new_index) {
                            Some(Some(k)) => (*k).to_string(),
                            _ => continue,
                        };
                        emitted[*new_index] = Some(key_owned.clone());
                        drop(key_owned);
                    }
                    ChildOpPlan::Remove { .. } => {}
                }
            }
            // Phase 2 — walk plan in FORWARD new order; for each
            // non-Keep op, find emitted[*before] reference and apply
            // InsertBefore(handle, reference) (or AppendChild if None).
            for op in plan.iter() {
                match op {
                    ChildOpPlan::Keep { .. } => {}
                    ChildOpPlan::MoveBefore { new_index, before }
                    | ChildOpPlan::InsertBefore { new_index, before } => {
                        let handle: String = match emitted[*new_index].clone() {
                            Some(h) => h,
                            None => continue,
                        };
                        let reference: Option<String> =
                            before.and_then(|idx| emitted.get(idx).and_then(|e| e.clone()));
                        match reference {
                            Some(ref_handle) => {
                                // Move the handle out of its current
                                // position (if present) and insert
                                // before ref_handle's handle.
                                let pos = dom.iter().position(|h| h == &handle);
                                if let Some(pos) = pos {
                                    dom.remove(pos);
                                }
                                let ref_pos = dom.iter().position(|h| h == &ref_handle);
                                match ref_pos {
                                    Some(rp) => dom.insert(rp, handle),
                                    None => dom.push(handle),
                                }
                            }
                            None => {
                                let pos = dom.iter().position(|h| h == &handle);
                                if let Some(pos) = pos {
                                    dom.remove(pos);
                                }
                                dom.push(handle);
                            }
                        }
                    }
                    ChildOpPlan::Remove { .. } => {}
                }
            }
            // Compare to expected DOM order = new_keys with None filtered.
            let expected: Vec<String> = new_keys
                .iter()
                .filter_map(|k| k.map(|s| s.to_string()))
                .collect();
            assert_eq!(dom, expected, "DOM order mismatch in case `{name}`");
        }
    }
}
