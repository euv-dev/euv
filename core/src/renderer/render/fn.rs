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
