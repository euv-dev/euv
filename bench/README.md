# euv runtime performance suite

This is a **non-publishable** member of the root euv workspace. Package metadata, dependencies and build profiles are inherited from the root `Cargo.toml`; the member manifest declares no direct dependency paths or versions. It exercises the checked-out framework, engine and chart crates together.

Local reports under `bench/results/` and the generated `bench/RESULTS.md` summary are ignored by Git and excluded by the root workspace package rules. They remain available on disk but are not source or release artifacts.

## Run

```bash
cargo run --release --manifest-path bench/Cargo.toml --offline -- \
  --label candidate --output /absolute/path/to/results/candidate-allocations.json

cargo run --release --manifest-path bench/Cargo.toml --no-default-features --offline -- \
  --label candidate --output /absolute/path/to/results/candidate-timing.json
```

The default is `--suite all`: **core, engine, chart, and mixed**, in that order. `mixed` invokes all three subsystems within every measured frame; its result is not the sum of three independently measured timings. The runner verifies actual invocation counts and fails when a required subsystem did not run.

The default build measures allocations. The `--no-default-features` build measures CPU timing with the recording allocator completely absent. Run both builds for a complete CPU/heap comparison; each build covers all four suites by default.

A short coverage smoke test:

```bash
cargo run --release --manifest-path bench/Cargo.toml --offline -- \
  --ticks 2 --rounds 1 --warmup 1 --label smoke
```

Options:

- `--suite all|core|engine|chart|mixed` (default: `all`).
- `--ticks N`: measured frames per round (default: 100).
- `--rounds N`: repeated measured rounds (default: 7).
- `--warmup N`: unmeasured warmup frames (default: 20).
- `--scale N`: fixture-size multiplier, 1 through 16 (default: 1).
- `--label TEXT`: source/candidate identifier recorded in the report.
- `--output FILE`: JSON report; without this option JSON goes to stdout.

Human-readable round summaries go to stderr. Unknown options and invalid workloads return a nonzero exit code.

## Workloads

| Suite | Work performed per frame at scale 1 |
| --- | --- |
| Core | One real signal update and subscription notification; class and style snapshots combining 32 reactive string segments |
| Engine | Fixed-step broad-phase physics for 128 rigid bodies; update, count, capacity check, and draw-list recording for 2,048 live particles |
| Chart | Data update, axis/series layout, and display-list generation for three 2,048-point series (scalar, tuple, and object data) |
| Mixed | All of the above in the same frame on the same thread |

The fixtures are deterministic. Payloads, scene sizes, compiler profile, counters, warmup, and iteration counts are identical for baseline and candidate. Every sample records observed work counts and an output checksum so the compared runs can be checked for equivalent work.

## Measurement semantics

Initialization, warmup, and steady state are separate phases. Schema version 2 records the build's `mode` explicitly. Allocation mode records allocation/reallocation calls, allocated/freed requested bytes, signed live-byte change, and peak additional live heap bytes; it takes no frame clocks and leaves timing fields `null`. Timing mode records elapsed time, frames per second and nearest-rank P50/P95/P99 frame latency; the recording allocator and thread-local recorder are compiled out, and heap fields are `null`. A missing metric is never represented as a measured zero. Raw rounds are retained rather than reporting only a best result.

The allocator is a `System` wrapper with const-initialized **per-thread** counters. A successful reallocation counts its full new requested size as allocation traffic and its old size as freed traffic; it does not infer allocator-internal peak residency. A failed allocation is not counted as successful. Sample timing vectors are allocated before fixture measurement, and percentile sorting happens after the allocation snapshot.

`peak_extra_bytes` is the requested heap high-water mark **above the bytes already live at the start of that phase**. It is not the total heap size. `retained_after_drop_bytes` exposes bytes remaining after fixture drop; reactive signal slots intentionally retain their payloads until thread exit to keep stale `Copy` handles valid. This number is not automatically classified as a leak.

These are **native CPU/heap benchmarks**, not browser or GPU benchmarks. Chart text uses `TextMeasurer::approximate()`. No browser DOM patching, GPU upload/submission, GPU memory, process RSS, or end-to-end FPS is measured. CPU timings never come from the allocation-instrumented binary, because instrumentation overhead scales with allocation count and would distort the comparison. Isolated suites run serially to avoid cross-workload interference; mixed measures their shared-frame contention, not multi-core parallelism.

## Reproducible baseline comparison

Keep the baseline framework source unchanged and use this **same** harness and root build configuration in both snapshots. The root-manifest overlay below registers the benchmark workspace member, centralizes its dependencies and keeps compiler profiles and package-version metadata identical; it does not overwrite baseline Rust source. Replace the two absolute paths and the baseline reference below:

```bash
BASE=/absolute/path/to/euv-baseline
OUT=/absolute/path/to/results
mkdir -p "$BASE" "$OUT"
git archive <baseline-ref> | tar -x -C "$BASE"
cp -R bench "$BASE/bench"
cp -R automation "$BASE/automation"
cp Cargo.toml "$BASE/Cargo.toml"
cp Cargo.lock "$BASE/Cargo.lock"

CARGO_TARGET_DIR="$OUT/targets/baseline-allocations" cargo run --release --manifest-path "$BASE/bench/Cargo.toml" --offline -- \
  --label baseline --output "$OUT/baseline-allocations.json"
CARGO_TARGET_DIR="$OUT/targets/candidate-allocations" cargo run --release --manifest-path bench/Cargo.toml --offline -- \
  --label candidate --output "$OUT/candidate-allocations.json"
CARGO_TARGET_DIR="$OUT/targets/baseline-timing" cargo run --release --manifest-path "$BASE/bench/Cargo.toml" --no-default-features --offline -- \
  --label baseline --output "$OUT/baseline-timing.json"
CARGO_TARGET_DIR="$OUT/targets/candidate-timing" cargo run --release --manifest-path bench/Cargo.toml --no-default-features --offline -- \
  --label candidate --output "$OUT/candidate-timing.json"
```

Copying the root lockfile preserves third-party resolutions. Confirm that registry package versions/checksums and dependency edges match before comparing, and record the shared manifest overlay separately from the original source revision. Use the same machine, power/thermal state, scale, warmup, ticks, rounds, and compiler. Do not benchmark while compilation, tests, or other stress workloads are running. Repeat runs in alternating baseline/candidate order before interpreting small timing changes. Check all four suite names, configuration, work counts, and output checksums before comparing round medians. Label the exact source revision and preserve raw JSON locally with the report.

Use a distinct target directory for every source revision and build mode.
A shared target can retain a fresh Cargo fingerprint while another checkout
has overwritten the common executable path. Verify the executable hash, build
features and report's embedded `source_root` against the intended checkout;
a zero build exit alone does not establish which code was measured.

## Verification

The private-fixture tests are included in the library through `tests/mod.rs`.
`autotests = false` prevents Cargo from also compiling that module as an
independent integration-test crate; library unit tests remain enabled and run
in both benchmark build modes.

```bash
cargo test --manifest-path bench/Cargo.toml --all-targets --all-features --offline
cargo test --manifest-path bench/Cargo.toml --all-targets --no-default-features --offline
cargo clippy --manifest-path bench/Cargo.toml --all-targets --all-features --offline -- -D warnings
cargo clippy --manifest-path bench/Cargo.toml --all-targets --no-default-features --offline -- -D warnings
cargo fmt --manifest-path bench/Cargo.toml -- --check
```

Tests execute real mixed frames, verify default coverage and missing-subsystem rejection, check allocator accounting, and assert that warmed particle counting/capacity checks/rendering do not allocate an extra particle vector. A Rust-AST check also rejects direct field access in benchmark business code, including accesses inside formatting macros. Existing CI covers the benchmark through the workspace checks and additionally tests and lints its uninstrumented build.
