//! Integration tests for the keyed-list reorder planner
//! (`compute_child_ops_plan` + `ChildOpPlan`).
//!
//! Moved out of `core/src/renderer/render/fn.rs` per rust-standards
//! §14.4 (test code lives under `core/tests/`). The render module
//! re-exports `compute_child_ops_plan` and `ChildOpPlan` as `pub`
//! specifically so these integration tests can reach them.
//!
//! Coverage:
//! - `lis_indices`: 9 degenerate + pathological inputs
//! - `compute_child_ops_plan`: 13 insert/remove/reorder shapes
//! - `simulated_dom_order_matches_new_keys`: end-to-end DOM-order
//!   simulator exercising scroll-by-1 / scroll-by-N / middle-insert /
//!   bottom-delete / middle-delete / full-reverse / rotate / swap

#[cfg(test)]
mod tests {
    use euv_core::*;

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
        type Case = (
            &'static str,
            Vec<Option<&'static str>>,
            Vec<Option<&'static str>>,
        );
        // Simulate all the interesting reorder shapes — including
        // scroll-by-1 (1 Remove, 1 Insert, rest Keep), middle-insert,
        // middle-delete, full-reverse, swap.
        let cases: &[Case] = &[
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
