---
# Site-level configuration for the euv-docs demo home page
# (consumed by build.rs when EUV_DOCS_SRC_DIR is unset, e.g.
# during `cargo clippy` / `cargo test` in the euv-docs crate itself).
site:
  title: euv-docs
  description: A documentation site generator powered by euv + euv-ui.
  logo: '📘'
locales:
  - prefix: /
    # Content directory under docs/docs/. Required: the build refuses to
    # guess a locale's content location, and fails loudly when it is empty
    # instead of emitting a one-page site.
    dir: en
    lang: en-US
    label: English
    title: euv-docs
    description: A documentation site generator powered by euv + euv-ui.
    footer: MIT Licensed | Built with euv + euv-ui
    toc_label: On this page
    prev_label: Previous
    next_label: Next
    navbar:
      - { text: Home, link: / }
      - { text: Guide, link: /guide/ }
      - { text: GitHub, link: https://github.com/euv-dev/euv }
---

<center>

## euv-docs

<img src="https://docs.ltpp.vip/img/euv-docs.png" alt="" height="160">

[![](https://img.shields.io/crates/v/euv-docs.svg)](https://crates.io/crates/euv-docs)
[![](https://img.shields.io/crates/d/euv-docs.svg)](https://img.shields.io/crates/d/euv-docs.svg)
[![](https://docs.rs/euv-docs/badge.svg)](https://docs.rs/euv-docs)
[![](https://github.com/euv-dev/euv/workflows/Rust/badge.svg)](https://github.com/euv-dev/euv/actions?query=workflow=Rust)
[![](https://img.shields.io/crates/l/euv-docs.svg)](./LICENSE)

</center>

[Api Docs](https://docs.rs/euv-docs/latest/)

> A documentation site generator powered by euv + euv-ui (Rust/WASM).

## Installation

To use this crate, you can run cmd:

```shell
cargo install euv-docs
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
