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
cd native/tauri
npm install
npm run build:android
```

A signed `euv.apk` is produced in `native/tauri/`.

This client keeps an independent Cargo workspace so Android and desktop dependencies do not enter the web framework workspace.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).