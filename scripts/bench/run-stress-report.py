#!/usr/bin/env python3
"""Canonical release benchmark, stress-matrix, and automation verification runner for euv.

Builds `euv-performance` once in release mode, runs the 27-benchmark suite across
N warmed passes (computing per-benchmark medians), executes the 4-scenario opt-in
`--stress-matrix` verification suite (including interleaved A/B partial-layout damage
bit-exact parity), verifies `euv-automation` release tests (async DevAutomationBridge
and AI Schema v2), enforces hard performance budgets, and optionally writes the single
canonical report at `performance/BENCHMARK_REPORT.md`.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import math
import os
import platform
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Any

REPO_ROOT: Path = Path(__file__).resolve().parents[2]
CANONICAL_REPORT_PATH: Path = REPO_ROOT / "performance" / "BENCHMARK_REPORT.md"

# Hard performance budgets (maximum median milliseconds per operation in release mode).
BENCH_BUDGETS_MS: dict[str, float] = {
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
    "app_smooth_scroll_fractional": 4.0,
    "ui_router_freeze_switch": 5.0,
    "ui_datatable_filter_select": 5.0,
    "font_text_wrap_measure_cache": 0.1,
}


def run_cmd(cmd: list[str], cwd: Path = REPO_ROOT) -> subprocess.CompletedProcess[str]:
    """Execute a subprocess and exit with diagnostic output on failure."""
    proc = subprocess.run(
        cmd,
        cwd=str(cwd),
        capture_output=True,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        sys.stderr.write(f"Command failed ({proc.returncode}): {' '.join(cmd)}\n")
        if proc.stdout:
            sys.stderr.write(f"--- STDOUT ---\n{proc.stdout}\n")
        if proc.stderr:
            sys.stderr.write(f"--- STDERR ---\n{proc.stderr}\n")
        raise SystemExit(proc.returncode)
    return proc


def resolve_release_binary(repo_root: Path, binary_name: str) -> Path:
    """Resolve the compiled release binary path honoring CARGO_TARGET_DIR."""
    proc = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        cwd=str(repo_root),
        capture_output=True,
        text=True,
        check=False,
    )
    target_dir = repo_root / "target"
    if proc.returncode == 0:
        try:
            meta = json.loads(proc.stdout)
            raw_target = meta.get("target_directory")
            if isinstance(raw_target, str) and raw_target:
                target_dir = Path(raw_target)
        except json.JSONDecodeError:
            pass
    exe_suffix = ".exe" if os.name == "nt" else ""
    return target_dir / "release" / f"{binary_name}{exe_suffix}"


def extract_json_object(stdout: str) -> dict[str, Any]:
    """Extract the top-level JSON object from command stdout."""
    start = stdout.find("{")
    end = stdout.rfind("}")
    if start == -1 or end == -1 or end < start:
        raise ValueError("Output did not contain a valid JSON object")
    payload = json.loads(stdout[start : end + 1])
    if not isinstance(payload, dict):
        raise ValueError("Parsed JSON payload is not an object")
    return payload


def aggregate_bench_runs(run_metrics: list[list[dict[str, Any]]]) -> list[dict[str, Any]]:
    """Compute per-benchmark element-wise medians across N release runs."""
    if len(run_metrics) == 1:
        return run_metrics[0]
    order = [m["name"] for m in run_metrics[0]]
    by_run = [{m["name"]: m for m in run} for run in run_metrics]
    aggregated: list[dict[str, Any]] = []
    for name in order:
        samples = [rm[name] for rm in by_run]
        med_ns = float(statistics.median(float(s["ns_per_op"]) for s in samples))
        med_ops = float(statistics.median(float(s["ops_per_sec"]) for s in samples))
        med_micros = int(round(statistics.median(int(s["total_micros"]) for s in samples)))
        rep = min(samples, key=lambda s: abs(float(s["ns_per_op"]) - med_ns))
        aggregated.append(
            {
                "name": name,
                "iterations": int(rep["iterations"]),
                "total_micros": med_micros,
                "ns_per_op": med_ns,
                "ops_per_sec": med_ops,
                "detail": str(rep.get("detail", "")),
            }
        )
    return aggregated


def compute_geomean_speedup(
    baseline_map: dict[str, dict[str, Any]],
    current_map: dict[str, dict[str, Any]],
    order: list[str],
) -> float:
    """Compute geometric-mean speedup across shared benchmarks."""
    logs: list[float] = []
    for name in order:
        base = baseline_map.get(name)
        cur = current_map.get(name)
        if not base or not cur:
            continue
        base_ns = float(base["ns_per_op"])
        cur_ns = float(cur["ns_per_op"])
        if base_ns > 0.0 and cur_ns > 0.0:
            logs.append(math.log(base_ns / cur_ns))
    if not logs:
        return 1.0
    return math.exp(sum(logs) / len(logs))


def fmt_ns(ns: float) -> str:
    """Format nanoseconds using human-readable units."""
    if ns >= 1_000_000.0:
        return f"{ns / 1_000_000.0:,.2f} ms"
    if ns >= 1_000.0:
        return f"{ns / 1_000.0:,.2f} us"
    return f"{ns:,.1f} ns"


def fmt_ops(ops: float) -> str:
    """Format operations-per-second compactly."""
    if ops >= 1_000_000.0:
        return f"{ops / 1_000_000.0:,.2f}M"
    if ops >= 1_000.0:
        return f"{ops / 1_000.0:,.1f}k"
    return f"{ops:,.1f}"


def fmt_us(us: float) -> str:
    """Format microseconds using human-readable units."""
    if us >= 1_000.0:
        return f"{us / 1_000.0:,.2f} ms"
    return f"{us:,.1f} us"


def format_gfm_table(
    headers: list[str],
    alignments: list[str],
    rows: list[list[str]],
) -> list[str]:
    """Format a Prettier-aligned GitHub Flavored Markdown table."""
    col_count = len(headers)
    widths: list[int] = [len(h) for h in headers]
    for row in rows:
        for idx in range(col_count):
            widths[idx] = max(widths[idx], len(row[idx]))
    # Ensure separator cells have at least 3 dashes plus alignment colons.
    for idx, align in enumerate(alignments):
        min_w = 5 if align == "center" else 4
        widths[idx] = max(widths[idx], min_w)

    def pad_row(cells: list[str]) -> str:
        padded = [f" {cells[idx].ljust(widths[idx])} " for idx in range(col_count)]
        return "|" + "|".join(padded) + "|"

    sep_cells: list[str] = []
    for idx, align in enumerate(alignments):
        width = widths[idx]
        if align == "right":
            sep = "-" * (width - 1) + ":"
        elif align == "center":
            sep = ":" + "-" * (width - 2) + ":"
        else:
            sep = ":" + "-" * (width - 1)
        sep_cells.append(f" {sep} ")
    lines = [pad_row(headers), "|" + "|".join(sep_cells) + "|"]
    for row in rows:
        lines.append(pad_row(row))
    return lines


def get_git_info(repo_root: Path) -> tuple[str, str]:
    """Return current (branch, short_sha) for metadata reporting."""
    branch_proc = subprocess.run(
        ["git", "rev-parse", "--abbrev-ref", "HEAD"],
        cwd=str(repo_root),
        capture_output=True,
        text=True,
        check=False,
    )
    sha_proc = subprocess.run(
        ["git", "rev-parse", "--short=10", "HEAD"],
        cwd=str(repo_root),
        capture_output=True,
        text=True,
        check=False,
    )
    branch = branch_proc.stdout.strip() if branch_proc.returncode == 0 else "unknown"
    sha = sha_proc.stdout.strip() if sha_proc.returncode == 0 else "unknown"
    return branch, sha


def verify_all_gates(
    metrics: list[dict[str, Any]],
    stress_payload: dict[str, Any],
    automation_passed: bool | None,
) -> list[str]:
    """Enforce all hard performance budgets and stress-matrix invariants."""
    failures: list[str] = []
    metrics_map = {m["name"]: m for m in metrics}

    print("\n=== Performance Budget Verification ===")
    for name, limit_ms in BENCH_BUDGETS_MS.items():
        if name not in metrics_map:
            failures.append(f"MISSING benchmark: {name}")
            continue
        item = metrics_map[name]
        avg_ms = float(item["ns_per_op"]) / 1_000_000.0
        ops = float(item["ops_per_sec"])
        status = "PASS" if avg_ms <= limit_ms else "FAIL"
        print(
            f"[{status}] {name:36s} avg={avg_ms:7.3f}ms  budget<={limit_ms:5.2f}ms  "
            f"({ops:10.1f} ops/s)  {item['detail']}"
        )
        if avg_ms > limit_ms:
            failures.append(f"{name}: {avg_ms:.3f}ms > {limit_ms:.2f}ms")

    print("\n=== Stress Verification Matrix Invariants ===")
    entries = {e["name"]: e for e in stress_payload.get("entries", [])}
    required_scenarios = [
        "stress_partial_single_row_update_ab",
        "stress_virtual_list_scroll_tail",
        "stress_window_resize_reflow",
        "stress_terminal_tui_diff",
    ]
    for scenario in required_scenarios:
        if scenario not in entries:
            failures.append(f"MISSING stress scenario: {scenario}")
            continue
        entry = entries[scenario]
        lat = entry["latency"]
        over_144 = int(lat["over_6_94ms_count"])
        over_60 = int(lat["over_16_67ms_count"])
        status = "PASS" if over_144 == 0 and over_60 == 0 else "FAIL"
        print(
            f"[{status}] {scenario:36s} p50={float(lat['p50_us']):7.1f}us "
            f"p95={float(lat['p95_us']):7.1f}us p99={float(lat['p99_us']):7.1f}us "
            f">6.94ms={over_144} >16.67ms={over_60}  {entry['detail']}"
        )
        if over_144 > 0:
            failures.append(f"{scenario}: over_6_94ms_count={over_144} (> 0)")
        if over_60 > 0:
            failures.append(f"{scenario}: over_16_67ms_count={over_60} (> 0)")

    ab = stress_payload.get("partial_damage_ab")
    if not isinstance(ab, dict):
        failures.append("MISSING partial_damage_ab summary in stress matrix output")
    else:
        if not bool(ab.get("bit_exact_parity")):
            failures.append("stress_partial_single_row_update_ab: bit_exact_parity is false")
        cand = ab.get("candidate_partial_damage", {})
        if int(cand.get("full_redraw_count", 1)) != 0:
            failures.append(
                f"stress_partial_single_row_update_ab: full_redraw_count={cand.get('full_redraw_count')} (expected 0)"
            )
        dirty_drop = float(ab.get("dirty_area_ratio_drop_pct", 0.0))
        if dirty_drop <= 50.0:
            failures.append(
                f"stress_partial_single_row_update_ab: dirty_area_ratio_drop_pct={dirty_drop:.1f}% (<= 50%)"
            )

    resize_entry = entries.get("stress_window_resize_reflow")
    if resize_entry and "stale_bounds=0" not in str(resize_entry.get("detail", "")):
        failures.append(
            f"stress_window_resize_reflow: non-zero stale bounds ({resize_entry.get('detail')})"
        )

    tui_entry = entries.get("stress_terminal_tui_diff")
    if tui_entry and "idle_bytes=0" not in str(tui_entry.get("detail", "")):
        failures.append(
            f"stress_terminal_tui_diff: non-zero idle flush bytes ({tui_entry.get('detail')})"
        )

    if automation_passed is False:
        failures.append("euv-automation release tests failed")

    return failures


def extract_round_wall_secs(entry: dict[str, Any]) -> float:
    """Extract scalar median wall seconds from a ledger round entry."""
    if "median_wall_secs" in entry and isinstance(entry["median_wall_secs"], (int, float)):
        return float(entry["median_wall_secs"])
    raw = entry.get("wall_secs", 0.0)
    if isinstance(raw, list):
        nums = [float(x) for x in raw if isinstance(x, (int, float))]
        return float(statistics.median(nums)) if nums else 0.0
    if isinstance(raw, (int, float)):
        return float(raw)
    return 0.0


def build_canonical_markdown_report(
    metrics: list[dict[str, Any]],
    runs: int,
    median_wall_s: float,
    stress_payload: dict[str, Any],
    automation_summary: str,
    ledger_path: Path | None,
) -> str:
    """Render the single canonical English GFM report for `performance/BENCHMARK_REPORT.md`."""
    branch, sha = get_git_info(REPO_ROOT)
    timestamp_utc = dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    machine = f"{platform.system()} {platform.machine()}"

    ledger_data: dict[str, Any] | None = None
    baseline_round: dict[str, Any] | None = None
    if ledger_path is not None and ledger_path.exists():
        loaded = json.loads(ledger_path.read_text(encoding="utf-8"))
        rounds = loaded.get("rounds", [])
        if isinstance(rounds, list) and rounds:
            ledger_data = loaded
            baseline_round = rounds[0]

    lines: list[str] = [
        "# Euv Release Benchmark & Stress Verification Report",
        "",
        "Canonical latest performance and stress-verification snapshot generated by `scripts/bench/run-stress-report.py --write-report`.",
        "",
        "## Run Metadata",
        "",
    ]

    meta_rows: list[list[str]] = [
        ["Branch / Commit", f"`{branch}` (`{sha}`)"],
        ["Generated (UTC)", f"`{timestamp_utc}`"],
        ["Host", f"`{machine}`"],
        ["Benchmark Suite", f"{len(metrics)} release benchmarks (median of {runs} warmed runs)"],
        ["Full-Suite Median Wall Time", f"{median_wall_s:.3f}s"],
        ["Automation Release Verification", automation_summary],
    ]

    baseline_map: dict[str, dict[str, Any]] = {}
    current_map: dict[str, dict[str, Any]] = {m["name"]: m for m in metrics}
    order: list[str] = [m["name"] for m in metrics]
    if baseline_round is not None:
        baseline_map = {m["name"]: m for m in baseline_round.get("metrics", [])}
        geomean = compute_geomean_speedup(baseline_map, current_map, order)
        base_wall = extract_round_wall_secs(baseline_round)
        wall_factor = (base_wall / median_wall_s) if base_wall > 0.0 and median_wall_s > 0.0 else 0.0
        base_id = str(baseline_round.get("id", "R000"))
        base_sha = str(baseline_round.get("commit", ""))[:10]
        round_count = len(ledger_data.get("rounds", [])) if ledger_data else 0
        meta_rows.append(
            [
                f"Campaign Baseline (`{base_id}`)",
                f"`{base_sha}` ({round_count} recorded rounds, baseline wall {base_wall:.2f}s)",
            ]
        )
        meta_rows.append(
            [
                f"Overall Geometric-Mean Speedup vs `{base_id}`",
                f"**{geomean:.3f}x** ({(geomean - 1.0) * 100.0:+.1f}%, wall **{wall_factor:.2f}x** faster)",
            ]
        )

    lines.extend(format_gfm_table(["Field", "Value"], ["left", "left"], meta_rows))
    lines.extend(["", "## Stress Verification Matrix (Tail Latency & Invariants)", ""])

    stress_rows: list[list[str]] = []
    for entry in stress_payload.get("entries", []):
        lat = entry["latency"]
        stress_rows.append(
            [
                f"`{entry['name']}`",
                str(lat["iterations"]),
                fmt_us(float(lat["p50_us"])),
                fmt_us(float(lat["p95_us"])),
                fmt_us(float(lat["p99_us"])),
                fmt_us(float(lat["max_us"])),
                str(lat["over_6_94ms_count"]),
                str(lat["over_16_67ms_count"]),
                f"{float(lat['mean_dirty_area_ratio']):.4f}",
                str(lat["full_redraw_count"]),
            ]
        )
    lines.extend(
        format_gfm_table(
            [
                "Scenario",
                "Iters",
                "p50",
                "p95",
                "p99",
                "Max",
                ">6.94ms (144Hz)",
                ">16.67ms (60Hz)",
                "Dirty Ratio",
                "Full Redraws",
            ],
            [
                "left",
                "right",
                "right",
                "right",
                "right",
                "right",
                "right",
                "right",
                "right",
                "right",
            ],
            stress_rows,
        )
    )

    ab = stress_payload.get("partial_damage_ab")
    if isinstance(ab, dict):
        base = ab["baseline_full_redraw"]
        cand = ab["candidate_partial_damage"]
        lines.extend(
            [
                "",
                "### Interleaved A/B Partial-Layout Damage Proof (`stress_partial_single_row_update_ab`)",
                "",
            ]
        )
        ab_rows: list[list[str]] = [
            [
                "Full Redraw (`partial_layout_damage = false`)",
                fmt_us(float(base["p50_us"])),
                fmt_us(float(base["p95_us"])),
                fmt_us(float(base["p99_us"])),
                f"{float(base['mean_dirty_area_ratio']):.4f}",
                str(base["full_redraw_count"]),
            ],
            [
                "Surgical Partial Damage (`partial_layout_damage = true`)",
                fmt_us(float(cand["p50_us"])),
                fmt_us(float(cand["p95_us"])),
                fmt_us(float(cand["p99_us"])),
                f"{float(cand['mean_dirty_area_ratio']):.4f}",
                str(cand["full_redraw_count"]),
            ],
            [
                "**Delta / Verification**",
                f"**{float(ab['p50_speedup_pct']):+.1f}%**",
                f"**{float(ab['p95_speedup_pct']):+.1f}%**",
                f"**{float(ab['p99_speedup_pct']):+.1f}%**",
                f"**-{float(ab['dirty_area_ratio_drop_pct']):.1f}%**",
                f"bit-exact parity = `{str(bool(ab['bit_exact_parity'])).lower()}` ({ab['verified_steps']} steps)",
            ],
        ]
        lines.extend(
            format_gfm_table(
                ["Arm", "p50", "p95", "p99", "Dirty Ratio", "Full Redraws / Parity"],
                ["left", "right", "right", "right", "right", "left"],
                ab_rows,
            )
        )

    lines.extend(["", "## Standard 27-Benchmark Release Suite", ""])
    if baseline_map:
        bench_rows: list[list[str]] = []
        for idx, item in enumerate(metrics, start=1):
            name = item["name"]
            cur_ns = float(item["ns_per_op"])
            cur_ops = float(item["ops_per_sec"])
            base_item = baseline_map.get(name)
            if base_item:
                base_ns = float(base_item["ns_per_op"])
                speedup = (base_ns / cur_ns) if cur_ns > 0.0 else 1.0
                delta_pct = ((cur_ns - base_ns) / base_ns * 100.0) if base_ns > 0.0 else 0.0
                bench_rows.append(
                    [
                        str(idx),
                        f"`{name}`",
                        str(item["iterations"]),
                        fmt_ns(base_ns),
                        fmt_ns(cur_ns),
                        f"**{speedup:.2f}x**",
                        fmt_ops(cur_ops),
                        f"{delta_pct:+.1f}%",
                    ]
                )
            else:
                bench_rows.append(
                    [
                        str(idx),
                        f"`{name}`",
                        str(item["iterations"]),
                        "-",
                        fmt_ns(cur_ns),
                        "-",
                        fmt_ops(cur_ops),
                        "-",
                    ]
                )
        lines.extend(
            format_gfm_table(
                ["#", "Benchmark ID", "Iters", "Baseline", "Current", "Speedup", "Ops/s", "Latency Delta"],
                ["center", "left", "right", "right", "right", "right", "right", "right"],
                bench_rows,
            )
        )
    else:
        bench_rows = []
        for idx, item in enumerate(metrics, start=1):
            bench_rows.append(
                [
                    str(idx),
                    f"`{item['name']}`",
                    str(item["iterations"]),
                    fmt_ns(float(item["ns_per_op"])),
                    fmt_ops(float(item["ops_per_sec"])),
                    str(item["detail"]),
                ]
            )
        lines.extend(
            format_gfm_table(
                ["#", "Benchmark ID", "Iters", "Median Latency", "Throughput (ops/s)", "Verification Detail"],
                ["center", "left", "right", "right", "right", "left"],
                bench_rows,
            )
        )

    if ledger_data is not None:
        rounds = ledger_data.get("rounds", [])
        if len(rounds) > 1 and baseline_map:
            lines.extend(["", "## Optimization Campaign Wave Milestones", ""])
            milestone_indices: list[int] = [0]
            for round_target in range(10, len(rounds), 10):
                milestone_indices.append(round_target)
            if (len(rounds) - 1) not in milestone_indices:
                milestone_indices.append(len(rounds) - 1)
            milestone_rows: list[list[str]] = []
            for r_idx in milestone_indices:
                entry = rounds[r_idx]
                r_map = {m["name"]: m for m in entry.get("metrics", [])}
                r_geo = compute_geomean_speedup(baseline_map, r_map, order)
                milestone_rows.append(
                    [
                        str(entry.get("round", r_idx)),
                        f"`{entry.get('id', '')}`",
                        f"`{entry.get('track', '')}`",
                        f"`{str(entry.get('commit', ''))[:10]}`",
                        f"{extract_round_wall_secs(entry):.2f}s",
                        f"**{r_geo:.3f}x**",
                        str(entry.get("title", "")),
                    ]
                )
            lines.extend(
                format_gfm_table(
                    ["Round", "ID", "Track", "Commit", "Wall", "GeoMean vs R000", "Summary"],
                    ["right", "left", "left", "left", "right", "right", "left"],
                    milestone_rows,
                )
            )

    lines.extend(
        [
            "",
            "## Reproducing This Report",
            "",
            "```bash",
            "./scripts/bench/stress-bench.sh --runs 3 --write-report",
            "```",
            "",
        ]
    )
    return "\n".join(lines)


def write_file_atomic(path: Path, content: str) -> None:
    """Atomically replace target file without leaving partial writes or duplicate files."""
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp_name = tempfile.mkstemp(prefix=".bench-report-", suffix=".tmp", dir=str(path.parent))
    try:
        with os.fdopen(fd, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(content)
        os.replace(tmp_name, path)
    except Exception:
        if os.path.exists(tmp_name):
            os.unlink(tmp_name)
        raise


def main() -> int:
    """CLI entrypoint for canonical euv benchmark and stress verification."""
    parser = argparse.ArgumentParser(
        description="Run euv release benchmarks, stress verification matrix, and automation release tests."
    )
    parser.add_argument(
        "--runs",
        type=int,
        default=3,
        help="Number of warmed release benchmark runs to aggregate by median (default: 3).",
    )
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="Reuse existing compiled release binary instead of rebuilding euv-performance.",
    )
    parser.add_argument(
        "--skip-automation",
        action="store_true",
        help="Skip `cargo test --release -p euv-automation --all-features`.",
    )
    parser.add_argument(
        "--write-report",
        action="store_true",
        help="Atomically overwrite the single canonical report at performance/BENCHMARK_REPORT.md.",
    )
    parser.add_argument(
        "--ledger",
        type=Path,
        default=None,
        help="Optional path to a JSON round ledger to include baseline-vs-final deltas and wave milestones.",
    )
    parser.add_argument(
        "--json-out",
        type=Path,
        default=None,
        help="Optional path to write the combined JSON summary.",
    )
    args = parser.parse_args()
    runs = max(1, int(args.runs))

    if not args.skip_build:
        print("=== Building euv-performance (release) ===", flush=True)
        run_cmd(["cargo", "build", "--release", "-p", "euv-performance"])

    bench_bin = resolve_release_binary(REPO_ROOT, "euv-performance")
    if not bench_bin.exists():
        sys.stderr.write(f"Compiled release binary not found at {bench_bin}\n")
        return 1

    print("=== Warming release benchmark binary ===", flush=True)
    run_cmd([str(bench_bin), "--json"])

    run_metrics: list[list[dict[str, Any]]] = []
    wall_times: list[float] = []
    for idx in range(1, runs + 1):
        started = time.perf_counter()
        proc = run_cmd([str(bench_bin), "--json"])
        elapsed = time.perf_counter() - started
        payload = extract_json_object(proc.stdout)
        metrics = payload.get("metrics")
        if not isinstance(metrics, list) or not metrics:
            raise ValueError("euv-performance --json returned empty metrics list")
        run_metrics.append(metrics)
        wall_times.append(elapsed)
        print(f"[bench run {idx}/{runs}] {len(metrics)} benchmarks in {elapsed:.3f}s", flush=True)

    median_metrics = aggregate_bench_runs(run_metrics)
    median_wall_s = float(statistics.median(wall_times))

    print("\n=== Running Stress Verification Matrix (--stress-matrix --json) ===", flush=True)
    stress_started = time.perf_counter()
    stress_proc = run_cmd([str(bench_bin), "--stress-matrix", "--json"])
    stress_elapsed = time.perf_counter() - stress_started
    stress_payload = extract_json_object(stress_proc.stdout)
    print(
        f"[stress-matrix] {len(stress_payload.get('entries', []))} scenarios completed in {stress_elapsed:.3f}s",
        flush=True,
    )

    automation_passed: bool | None = None
    automation_summary = "Skipped (`--skip-automation`)"
    if not args.skip_automation:
        print("\n=== Verifying euv-automation (release) ===", flush=True)
        auto_started = time.perf_counter()
        auto_proc = subprocess.run(
            ["cargo", "test", "--release", "-p", "euv-automation", "--all-features"],
            cwd=str(REPO_ROOT),
            capture_output=True,
            text=True,
            check=False,
        )
        auto_elapsed = time.perf_counter() - auto_started
        automation_passed = auto_proc.returncode == 0
        if automation_passed:
            passed_line = next(
                (line.strip() for line in auto_proc.stdout.splitlines() if "test result: ok." in line),
                "PASS",
            )
            automation_summary = f"PASS (`{passed_line}`, {auto_elapsed:.2f}s)"
            print(f"[PASS] euv-automation release tests: {passed_line}", flush=True)
        else:
            automation_summary = f"FAIL (exit {auto_proc.returncode})"
            sys.stderr.write(auto_proc.stdout)
            sys.stderr.write(auto_proc.stderr)

    failures = verify_all_gates(median_metrics, stress_payload, automation_passed)

    if args.json_out is not None:
        combined = {
            "runs": runs,
            "median_wall_s": median_wall_s,
            "metrics": median_metrics,
            "stress_matrix": stress_payload,
            "automation_passed": automation_passed,
            "failures": failures,
        }
        write_file_atomic(args.json_out, json.dumps(combined, indent=2) + "\n")

    if args.write_report:
        report_md = build_canonical_markdown_report(
            metrics=median_metrics,
            runs=runs,
            median_wall_s=median_wall_s,
            stress_payload=stress_payload,
            automation_summary=automation_summary,
            ledger_path=args.ledger,
        )
        write_file_atomic(CANONICAL_REPORT_PATH, report_md)
        print(f"\nWrote canonical report: {CANONICAL_REPORT_PATH}", flush=True)

    if failures:
        print("\nVERIFICATION FAILURES:")
        for item in failures:
            print(f"  - {item}")
        return 1

    print("\nALL BENCHMARK BUDGETS, STRESS MATRIX INVARIANTS, AND AUTOMATION GATES PASSED!")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
