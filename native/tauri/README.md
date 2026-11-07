<center>

## euv-app

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/euv.png" alt="" height="160">

[![](https://img.shields.io/github/release/euv-dev/euv/latest.svg)](https://github.com/euv-dev/euv/releases/latest)
[![](https://img.shields.io/crates/l/euv.svg)](./LICENSE)

</center>

[Download APK](https://github.com/euv-dev/euv/releases/latest/download/euv.apk)

> Cross-platform client for the **euv** UI framework, with one shared Rust cache, startup-readiness bridge, and runtime update implementation.

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

A signed `euv.apk` is produced in `native/tauri/`.

This client keeps an independent Cargo workspace so Android and desktop dependencies do not enter the web framework workspace.

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
npm run verify:apk -- euv.apk
```

Verification compares the local output with its SHA-256 manifest and checks
the actual native library inside the APK for every bundled file. The remote
URL in `app.config.json` remains the runtime update source; it does not supply
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
node scripts/verify-output.js macos
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
python3 scripts/smoke-android.py --serial emulator-5580 --fresh

# Creates, boots, installs into, exercises, and removes its own iPhone simulator.
python3 scripts/smoke-ios.py
```

The Android test disables emulator network access for startup and restart,
exercises Counter 0 -> 1 -> 0, and compares deployed files with the local
example hashes. The iOS test uses real XCTest touch input, captures startup,
Counter and restart screenshots, and checks the deployed cache hashes. Its
`.xcresult`, screenshots and JSON report are under `artifacts/validation/ios/`.
Use `--udid` only to borrow an existing simulator; borrowed devices are not
deleted by the test. macOS package acceptance includes an actual installation
into the user's Applications folder and native-window interaction.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).