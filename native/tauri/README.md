<center>

## euv-app

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/euv.png" alt="" height="160">

[![](https://img.shields.io/github/release/euv-dev/euv/latest.svg)](https://github.com/euv-dev/euv/releases/latest)
[![](https://img.shields.io/crates/l/euv.svg)](./LICENSE)

</center>

[Download APK](https://github.com/euv-dev/euv/releases/latest/download/euv.apk)

> Android client for the **euv** UI framework — bundles the euv web frontend as a standalone APK via Tauri 2.x with a Rust bridge for native operations.

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

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).