# euv Scripts

Unified environment, build, and end-to-end verification scripts organized by responsibility.

## Directory Layout

```text
scripts/
├── env/
│   ├── common.sh          # Shared logging, OS/arch detection, and env loader
│   ├── doctor.sh          # 7-platform toolchain & mobile device diagnostics
│   ├── setup-all.sh       # Full cross-platform environment bootstrap
│   ├── setup-rust.sh      # Rust targets (all 7 triples) & WASM tools
│   ├── setup-web.sh       # WebAssembly / zero-JS bundle prerequisites
│   ├── setup-apple.sh     # macOS / iOS / iPadOS Xcode & Simulator check
│   └── setup-android.sh   # Automated Android SDK, adb, NDK, build-tools & AVD setup
├── build/
│   ├── build-web.sh       # Zero-JS Web release bundle (dist/web)
│   ├── build-ios-sim.sh   # Release iOS Simulator .app build & install
│   └── build-android-apk.sh # Release Android native APK build & adb install
└── test/
    └── e2e-all.sh         # Full E2E, standards, release-stripping & 7-target verification
```

## Quick Start

```bash
./scripts/env/setup-android.sh   # Provision Android SDK, NDK, adb, build-tools, AVD
./scripts/env/doctor.sh          # Verify all 7 target toolchains & connected devices
./scripts/test/e2e-all.sh        # Run full E2E & release-stripping verification
./scripts/build/build-android-apk.sh --install
```
