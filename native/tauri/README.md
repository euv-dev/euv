<center>

## euv-app

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/euv.png" alt="" height="160">

[![](https://img.shields.io/github/release/euv-dev/euv/latest.svg)](https://github.com/euv-dev/euv/releases/latest)
[![](https://img.shields.io/crates/l/euv.svg)](./LICENSE)

</center>

[Download APK](https://github.com/euv-dev/euv/releases/latest/download/euv.apk)

> Cross-platform client for the **euv** UI framework, with one shared Rust cache, startup-readiness bridge, and runtime update implementation.

## Directory layout

```text
native/tauri/
  config/                  # Authored application configuration
  resources/
    runtime/               # Embedded WebView runtime assets
    signing/               # Existing signing inputs; never log their contents
  scripts/
    build/                 # Web snapshot and native package builders
    config/                # Platform configuration and icon generation
    toolchain/             # SDK installation and compiler compatibility helpers
    validation/            # Artifact and real-device/simulator checks
    benchmark/             # Reproducible measurements, not production startup
    cache/                 # Explicit remote-cache maintenance tools
    tests/                 # Script regression tests and fixtures
  src-tauri/               # Independent Rust workspace and native platform glue
  dist/                    # Tauri-required generated bootstrap frontend
  artifacts/               # Packages and validation evidence, by platform
  .cache/sdk/              # Ignored local SDK/JDK downloads
```

Do not add executable scripts to the client root or directly to `scripts/`.
The npm manifest and the project CLI point directly at grouped implementations;
old-path wrapper scripts are not retained. Tauri's required `src-tauri/gen/`
platform projects and `dist/` input remain in their conventional locations.
The directory policy is checked by `scripts/tests/layout.test.js` in `npm test`.

## Installation

To build this app, you can run cmd:

```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
cargo run -p euv-cli -- build --release --crate-path ./example -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
cd native/tauri
npm install
npm run build:android
```

A signed `euv.apk` is produced in `native/tauri/artifacts/android/`.

This client keeps an independent Cargo workspace so Android and desktop dependencies do not enter the web framework workspace.

## One-command web and native builds

After installing wasm-pack, Node dependencies, and the platform SDKs, run this
from the repository root:

```shell
cargo run -p euv-cli -- build --all --release --crate-path ./example
```

An installed CLI accepts the same arguments as `euv build --all`. The web
snapshot is compiled once and passed to every native builder as `EUV_WEB_DIST`.
Builds are sequential, stream their actual compiler output, and stop with a
nonzero exit status when any requested target fails. Native packaging uses
release mode by default; `--dev` and `--profiling` remain web-only profiles.

`--all` means all clients supported by the current **build host**, not
cross-compilation of every desktop OS from one machine:

| Host | Native output selection, in addition to web |
| --- | --- |
| macOS | macOS, iOS device, iOS simulator, Android |
| Linux | Linux, Android |
| Windows | Windows; the current Android helper requires macOS or Linux |

Inspect a plan or select a smaller native subset:

```shell
cargo run -p euv-cli -- build --all --plan --crate-path ./example
cargo run -p euv-cli -- build --platforms macos,android --release --crate-path ./example
```

`--native-path` selects a different native project directory and requires
`--all` or `--platforms`. Explicitly requesting an incompatible platform fails
before compilation instead of silently omitting it. Web output stays in the
selected crate's serving directory; native output paths are listed below.
All targets embed that newly built web output, including when a different
`--crate-path` or `--www-dir` is used. iOS signing requirements do not change.
Windows/Linux packaging remains outside the current runtime acceptance scope.

## Bundled pages and CI

Run the commands above from the repository root. APK builds embed the complete
local `example/www/` output, including its HTML, WebAssembly, and other assets.
No remote page is fetched while packaging. Build the example first; missing or
invalid local output fails the APK build instead of falling back to a website.
`EUV_WEB_DIST` can select another already-built example output directory.

The APK workflow builds the example from the checked-out source, packages it,
and uploads the verified APK as a workflow artifact. Master pushes and manual
runs on master also update the rolling APK release. Pull requests only upload
the artifact and never publish a release.

```shell
npm run bundle:example
npm run verify:apk -- artifacts/android/euv.apk
```

Verification compares the local output with its SHA-256 manifest and checks
the actual native library inside the APK for every bundled file. The remote
URL in `config/app.config.json` remains the runtime update source; it does not supply
the APK's built-in pages.

## Windows, macOS, Linux, and iOS

Build `example/www/` using the repository-root command above, then run `npm ci`
inside `native/tauri/`. All clients embed that same local snapshot and use the
same startup cache readiness, custom-protocol serving, and runtime update code
as Android. Platform-specific URL origins are covered by bootstrap tests.

| Platform | Required build host | Command | Outputs |
| --- | --- | --- | --- |
| Windows | Windows x64 with MSVC | `npm run build:windows` | Native EXE, NSIS installer, MSI |
| macOS | macOS with Xcode | `npm run build:macos` | App archive and DMG |
| Linux | Linux with WebKitGTK 4.1 development packages | `npm run build:linux` | Native binary, DEB, AppImage |
| iOS | macOS with Xcode and the iOS SDK | `npm run build:ios` | Unsigned device IPA and Xcode archive |

Outputs are published to `artifacts/<platform>/` only after required package
types, file hashes, and embedded local pages pass verification. Each platform
includes a pages manifest and a build report. Recheck a completed output with:

```shell
node scripts/validation/verify-output.js macos
```

For iOS, install real `xcodegen`, `libimobiledevice`, and CocoaPods tools first,
and add the `aarch64-apple-ios` Rust target. The build helper explicitly selects
the iPhoneOS SDK and the native SwiftPM backend required by swift-rs 1.0.x,
including on Swift versions whose default backend is SwiftBuild.

The default desktop packages are unsigned and not notarized. The default iOS
IPA is explicitly named `-unsigned.ipa`: it is a real device build but cannot
be installed or submitted to the App Store without valid Apple signing and
provisioning. With genuine signing credentials already installed and a real
`APPLE_DEVELOPMENT_TEAM` configured, `EUV_IOS_SIGN=1 npm run build:ios` requests
a signed development export. No placeholder identities or team IDs are used.

The active client CI builds macOS, iOS device, and iOS simulator packages on
macOS runners, verifies and uploads them on pull requests, and publishes assets
only after these builds succeed on master. Windows and Linux build entries are
retained for future work but are currently excluded from CI and acceptance.
The Android APK workflow remains separate and retains its signed-APK behavior.

`npm run build:ios:simulator` produces a simulator-specific app archive under
`artifacts/ios-simulator/`. It is checked for the iPhoneSimulator platform and
cannot be confused with an unsigned device IPA. iOS requires version 15 or
newer. Its scene manifest enables Tao's scene lifecycle, and a small ARC
adapter repairs the scene-configuration ownership handoff in Tao 0.35.3.
The native WebView publishes measured UIKit safe-area insets to the shared
CSS contract rather than combining automatic and manual top padding.

### Runtime acceptance

```shell
# Use only a disposable, rootable Android emulator; --fresh clears Euv data.
python3 scripts/validation/smoke-android.py --serial emulator-5580 --fresh

# Creates, boots, installs into, exercises, and removes its own iPhone simulator.
python3 scripts/validation/smoke-ios.py
```

The Android test disables emulator network access for startup and restart,
exercises Counter 0 -> 1 -> 0, and compares deployed files with the local
example hashes. The iOS test uses real XCTest touch input, captures startup,
Counter and restart screenshots, and checks the deployed cache hashes. Its
`.xcresult`, screenshots and JSON report are under `artifacts/validation/ios/`.
Use `--udid` only to borrow an existing simulator; borrowed devices are not
deleted by the test. macOS package acceptance includes an actual installation
into the user's Applications folder and native-window interaction.

## Immersive presentation and refresh rate

`ui.immersiveMode` and `ui.maxFrameRateEnabled` are enabled by default in
`config/app.config.json`. Desktop windows are borderless but remain resizable and do
not enter OS fullscreen unless `ui.window.fullscreen` is enabled separately.
Android hides status/navigation bars while retaining transient swipe access.
iOS asks Tao's own view controller to hide the status bar and auto-hide the
home indicator; the native safe-area bridge still protects interactive content.

Android selects the highest supported refresh mode at the current resolution
and reapplies it as the active display changes. Apple clients derive their
preference from the attached screen's `maximumFramesPerSecond`, use a native
display link, and pause it while inactive. Moving a macOS window between screens
updates the request. iPhone builds opt into ProMotion in their Info.plist.
macOS versions before 14 use the native WebKit compositor cadence because a
public display-link preference API is unavailable there. Windows/Linux WebViews
retain compositor-driven vsync, without an application-imposed fixed FPS timer.

These are maximum-refresh **requests**, not a guarantee of page FPS. OS power,
thermal and accessibility policy can reduce the rate. WKWebView can also limit
`requestAnimationFrame` independently of the physical display; no private
WebKit preference or fabricated animation frame is used to bypass that limit.
The injected `window.__EUV_NATIVE_DISPLAY__` reports the native maximum and
request separately and explicitly sets `guaranteed: false`.
Set `EUV_CLIENT_DIAGNOSTICS=1` when launching an Apple client to log a real
120-frame `requestAnimationFrame` sample, separately from the native request.
The simulator smoke test records these measurements and asserts immersive
status-bar state; it does not claim physical ProMotion hardware validation.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).