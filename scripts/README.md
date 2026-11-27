# euv Scripts

Unified environment provisioning, cross-platform build, theme packaging, benchmarking, and end-to-end verification scripts organized by responsibility.

## Directory Layout

```text
scripts/
├── bench/
│   └── stress-bench.sh            # High-load layout, list, reactivity & 3D stress benchmarks
├── build/
│   ├── build-web.sh               # Zero-JS WebAssembly release bundle (dist/web)
│   ├── build-ios-sim.sh           # Release iOS Simulator .app build & launch
│   ├── build-android-apk.sh       # Release Android native APK build & adb install
│   ├── build-theme-variants.sh    # Builds all 17 theme-feature WebAssembly showcase variants
│   └── write-release-manifest.py  # Generates deterministic SHA-256 release inventory manifests
├── env/
│   ├── common.sh                  # Shared logging, OS/arch detection, and env loader
│   ├── doctor.sh                  # 7-platform toolchain & mobile device diagnostics
│   ├── env.sh                     # Local environment exports (SDK/NDK/WASM paths)
│   ├── generate-icon.py           # Cross-platform app icon generator
│   ├── setup-all.sh               # Full cross-platform environment bootstrap
│   ├── setup-rust.sh              # Rust targets (all 7 triples) & WASM tools
│   ├── setup-web.sh               # WebAssembly / zero-JS bundle prerequisites
│   ├── setup-apple.sh             # macOS / iOS / iPadOS Xcode & Simulator check
│   └── setup-android.sh           # Automated Android SDK, adb, NDK, build-tools & AVD setup
└── test/
    └── e2e-all.sh                 # Full E2E, standards, release-stripping & 7-target verification
```

## Quick Start

```bash
./scripts/env/setup-android.sh        # Provision Android SDK, NDK, adb, build-tools, AVD
./scripts/env/doctor.sh               # Verify all 7 target toolchains & connected devices
./scripts/build/build-web.sh          # Build zero-JS WebAssembly bundle
./scripts/build/build-theme-variants.sh # Build all 17 theme-feature WebAssembly variants
./scripts/build/build-android-apk.sh --install
./scripts/build/build-ios-sim.sh
./scripts/test/e2e-all.sh             # Run full E2E & release-stripping verification
```
