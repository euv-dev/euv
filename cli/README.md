---
# euv-docs site configuration (consumed by `euv-docs` build.rs).
# This frontmatter is the SINGLE source of truth for site-wide
# settings — replaces the legacy `docs/config.toml`.
site:
  title: euv-docs
  description: A VuePress-style documentation site powered by euv + euv-ui.
  logo: '📘'
locales:
  - prefix: /
    lang: en-US
    label: English
    title: euv-docs
    description: A VuePress-style documentation site powered by euv + euv-ui.
    footer: MIT Licensed | Built with euv + euv-ui
    toc_label: On this page
    prev_label: Previous
    next_label: Next
    navbar:
      - { text: Home, link: / }
      - { text: Guide, link: /guide/ }
      - { text: GitHub, link: https://github.com/euv-dev/euv }
  - prefix: /zh/
    lang: zh-CN
    label: 简体中文
    title: euv-docs
    description: 由 euv + euv-ui 驱动的 VuePress 风格文档站。
    footer: MIT 许可 | 基于 euv + euv-ui 构建
    toc_label: 本页目录
    prev_label: 上一页
    next_label: 下一页
    navbar:
      - { text: 首页, link: /zh/ }
      - { text: 指南, link: /zh/guide/ }
      - { text: GitHub, link: https://github.com/euv-dev/euv }
---

<center>

## euv-cli

<img src="https://docs.ltpp.vip/img/euv-cli.png" alt="" height="160">

[![](https://img.shields.io/crates/v/euv-cli.svg)](https://crates.io/crates/euv-cli)
[![](https://img.shields.io/crates/d/euv-cli.svg)](https://img.shields.io/crates/d/euv-cli.svg)
[![](https://docs.rs/euv-cli/badge.svg)](https://docs.rs/euv-cli)
[![](https://github.com/euv-dev/euv/workflows/Rust/badge.svg)](https://github.com/euv-dev/euv/actions?query=workflow=Rust)
[![](https://img.shields.io/crates/l/euv-cli.svg)](./LICENSE)

</center>

[Api Docs](https://docs.rs/euv-cli/latest/)

> The official CLI tool for the euv UI framework, providing dev/release profiles with run/build modes, hot reload, and wasm-pack integration.

## Installation

To use this crate, you can run cmd:

```shell
cargo install euv-cli
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@devhub.vip](mailto:root@devhub.vip).
