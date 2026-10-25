# euv UI automation

A non-publishable root-workspace package that builds an actual euv/euv-ui WASM fixture and operates it through an isolated headless ChromeDriver session. Its normal cases use real element clicks, editable-control key input, Enter/Tab, checkbox interaction, modal controls and wheel actions. Browser-side JavaScript is used to observe state, collect diagnostics and reveal a wheel target, not to fake component events or assertions.

## Run

Prerequisites: a working Rust toolchain with the `wasm32-unknown-unknown` target, a `wasm-bindgen` CLI matching the workspace dependency, compatible Chrome/ChromeDriver, curl and Python 3. Tools are checked for actual, nonempty version output before the run; empty or broken executables cannot report a successful build.

```bash
cargo run -p euv-automation -- \
  --driver /path/to/chromedriver \
  --bindgen /path/to/wasm-bindgen
```

The runner builds its own fixture, starts a loopback-only static server, launches an owned headless browser, waits for the real WASM mount marker and executes every case. It does not use the user's browser profile or desktop mouse/keyboard.

Options: `--cargo PATH`, `--chrome PATH`, `--output DIR`, `--url URL`, `--skip-build`, `--probe-failure`. `--url` requires a served application implementing the fixture's stable selector contract; generic `Browser` operations can also be used directly against other pages. `--skip-build` is explicit and still validates the required nonempty fixture assets.

## Verification and evidence

The positive suite checks trusted click events and reactive counts, input-to-signal updates, Enter handling and Tab focus, checkbox DOM and signal state, modal visibility/open/close, wheel scrolling and reactive scroll events, and actual narrow-viewport geometry. Assertions poll observable browser state under a timeout rather than assuming a fixed sleep is enough.

Narrow viewports use ChromeDriver's device-metrics command after setting the window rectangle, so Chrome's physical minimum window width cannot turn a mobile-layout check into an unmeasured desktop check. The fixture mounts through the real component lifecycle and initializes the framework theme tokens; modal visibility stays signal-bound while its subtree remains mounted.

Every case writes a real PNG screenshot and a DOM/URL/focus/visibility JSON snapshot. `results/report.json` contains each actual outcome, elapsed time, capture errors and owned process/session identities. Any case, artifact, startup or cleanup failure produces a nonzero process exit. Outputs remain local: the root Git and package exclusions cover `automation/results/`.

```bash
cargo test -p euv-automation --all-targets
cargo clippy -p euv-automation --all-targets -- -D warnings
cargo run -p euv-automation -- --driver /path/to/chromedriver --bindgen /path/to/wasm-bindgen --skip-build --probe-failure
```

The last command intentionally returns a nonzero exit and captures a failed `999` counter expectation. It is a negative control, never part of the normal green suite. Library unit tests enter through `#[cfg(test)] #[path = "../tests/mod.rs"] mod tests;`; browser execution is a separate, explicit acceptance command.
