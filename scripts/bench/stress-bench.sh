#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${REPO_ROOT}"

echo "=== Running euv Release Benchmark & Stress Suite ==="
cargo run --release -p euv-performance

JSON_OUT="$(mktemp)"
trap 'rm -f "${JSON_OUT}"' EXIT
cargo run --release -p euv-performance -- --json > "${JSON_OUT}"

python3 - "${JSON_OUT}" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1]))
metrics = {m["name"]: m for m in data["metrics"]}

# Hard performance budgets (maximum average milliseconds per operation in release mode)
budgets_ms = {
    "reactive_signal_memo_diamond": 0.02,
    "reactive_selector_o1_switch": 0.02,
    "layout_clean_subtree_cache": 0.05,
    "render_software_full_redraw": 12.0,
    "stress_long_list_virtual_100k": 12.0,
    "stress_complex_layout_2000_nodes": 5.0,
    "stress_bulk_reactive_1000_nodes": 20.0,
    "stress_wasm_dynamic_load_hotswap": 15.0,
    "stress_hidpi_4x_1440x3168_scroll": 25.0,
    "stress_240hz_scroll_and_hittest": 4.0,
}

failed = []
print("\n=== Performance Budget Verification ===")
for name, limit_ms in budgets_ms.items():
    if name not in metrics:
        failed.append(f"MISSING benchmark: {name}")
        continue
    avg_ms = metrics[name]["ns_per_op"] / 1_000_000.0
    ops = metrics[name]["ops_per_sec"]
    status = "PASS" if avg_ms <= limit_ms else "FAIL"
    print(f"[{status}] {name:36s} avg={avg_ms:7.3f}ms  budget<={limit_ms:5.2f}ms  ({ops:10.1f} ops/s)  {metrics[name]['detail']}")
    if avg_ms > limit_ms:
        failed.append(f"{name}: {avg_ms:.3f}ms > {limit_ms:.2f}ms")

if failed:
    print("\nBUDGET REGRESSIONS:")
    for item in failed:
        print("  -", item)
    sys.exit(1)
print("\nALL STRESS BENCHMARK BUDGETS PASSED!")
PY
