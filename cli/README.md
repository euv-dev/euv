---
# euv-docs site configuration (consumed by `euv-docs` build.rs).
# This frontmatter is the SINGLE source of truth for site-wide
# settings — replaces the legacy `docs/config.toml`.
site:
  title: euv-docs
  description: A VuePress-style documentation site powered by euv + euv-ui.
  logo: "📘"
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

[![](https://img.shields.io/crates/v/euv-cli.svg)](https://crates.io/crates/euv-cli)
[![](https://img.shields.io/crates/d/euv-cli.svg)](https://img.shields.io/crates/d/euv-cli.svg)
[![](https://docs.rs/euv-cli/badge.svg)](https://docs.rs/euv-cli)
[![](https://github.com/euv-dev/euv/workflows/Rust/badge.svg)](https://github.com/euv-dev/euv/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/euv-cli.svg)](../LICENSE)

</center>

[Api Docs](https://docs.rs/euv-cli/latest/)

> The official CLI tool for the euv UI framework, providing run/build/fmt modes with hot reload and wasm-pack integration.

## Installation

```shell
cargo install euv-cli
```

> After installation, the CLI binary is named `euv`.

## Usage

The CLI uses the pattern `euv <action>` where:

- **Action**: `run` (build + start server), `build` (build only), or `fmt` (format euv macros)

All `wasm-pack build` arguments are transparently forwarded after `--`.

### Run (build + dev server)

```shell
# Dev build with dev server
euv run --dev --crate-path ./example --port 80 --www-dir www --index-html ./template.html -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
# Release build with dev server
euv run --release --crate-path ./example --port 80 --www-dir www --index-html ./template.html -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
# Dev build for local development
euv run --dev --crate-path ./example --port 80 --www-dir www -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
# Release build for local development
euv run --release --crate-path ./example --port 80 --www-dir www -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

### Fmt (format euv macros)

Format euv macro invocations (`html!`, `class!`, `vars!`, `watch!`) in Rust source files:

```shell
# Format all .rs files in the current directory (recursive)
euv fmt

# Format all .rs files in a specific directory
euv fmt --path ./src

# Format a single file
euv fmt --path src/main.rs

# Check if formatting is needed without modifying files
euv fmt --check

# Check a specific path
euv fmt --path ./src --check
```

#### Formatting Rules

- Tag name and `{` separated by exactly one space
- Attribute name immediately followed by `:`, then one space before the value
- `if { expr }` / `match { expr }` / `for pattern in { expr }` with proper spacing
- `=>` in `watch!` macros with proper spacing
- Content inside expression braces `{ expr }` is preserved verbatim; only template-level whitespace is normalized

#### Fmt Options

| Option    | Short | Default | Description                                           |
| --------- | ----- | ------- | ----------------------------------------------------- |
| `--path`  | `-p`  | `.`     | Path to the directory or file to format               |
| `--check` |       | `false` | Check if formatting is needed without modifying files |

### Build (build only)

```shell
# Dev build
euv build --dev --crate-path ./example --www-dir www --index-html ./template.html -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
# Release build
euv build --release --crate-path ./example --www-dir www --index-html ./template.html -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
# Dev build for local development
euv build --dev --crate-path ./example --www-dir www -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
# Release build for local development
euv build --release --crate-path ./example --www-dir www -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

### Via Cargo

#### Run (build + dev server)

```shell
cargo run -p euv-cli -- run --dev --crate-path ./example --port 80 --www-dir www --index-html ./template.html -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
cargo run -p euv-cli -- run --release --crate-path ./example --port 80 --www-dir www --index-html ./template.html -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
cargo run -p euv-cli -- run --dev --crate-path ./example --port 80 --www-dir www -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
cargo run -p euv-cli -- run --release --crate-path ./example --port 80 --www-dir www -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

#### Build (build only)

```shell
cargo run -p euv-cli -- build --dev --crate-path ./example --www-dir www --index-html ./template.html -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
cargo run -p euv-cli -- build --release --crate-path ./example --www-dir www --index-html ./template.html -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
cargo run -p euv-cli -- build --dev --crate-path ./example --www-dir www -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

```shell
cargo run -p euv-cli -- build --release --crate-path ./example --www-dir www -- --target web --out-dir www/pkg --out-name euv --no-typescript --no-pack --no-gitignore
```

#### Fmt (format euv macros)

```shell
# Format euv macros
cargo run -p euv-cli -- fmt --path ./example
```

```shell
# Check formatting
cargo run -p euv-cli -- fmt --check --path ./example
```

### Options

| Option         | Short | Default | Description                                                                  |
| -------------- | ----- | ------- | ---------------------------------------------------------------------------- |
| `--crate-path` | `-c`  | `.`     | Path to the Rust crate containing the WASM application                       |
| `--port`       | `-p`  | `80`    | Port for the development server                                              |
| `--www-dir`    |       | `www`   | Directory name for static assets and generated HTML (relative to crate-path) |
| `--index-html` |       |         | Path to a custom index.html template file                                    |
| `--dev`        |       |         | Create a development build (default if no mode specified)                    |
| `--release`    |       |         | Create a release build with optimizations                                    |
| `--profiling`  |       |         | Create a profiling build with optimizations and debug info                   |

### How `--www-dir` Works

The `--www-dir` option controls where the CLI looks for and generates static assets:

- **`index.html`** is generated inside `{crate-path}/{www-dir}/`
- **WASM artifacts** are placed in `{crate-path}/{www-dir}/pkg/` by default (unless overridden by `--out-dir`)
- **`.gitignore`** generated by `wasm-pack` in the output directory is automatically removed after build
- **Dev server** serves files under the `/{www-dir}/` route prefix
- **JS import path** in `index.html` is automatically computed relative to the www directory

For example, with `--www-dir www`:

```
example/
├── www/           ← static assets directory
│   ├── index.html    ← generated HTML
│   └── pkg/          ← WASM build output
│       ├── euv.js
│       └── euv_bg.wasm
```

### wasm-pack Options (transparent passthrough after `--`)

All `wasm-pack build` flags are forwarded as-is. Common options:

| Option              | Description                                                |
| ------------------- | ---------------------------------------------------------- |
| `--target`          | Target environment: bundler, nodejs, web, no-modules, deno |
| `--out-dir`         | Output directory with a relative path                      |
| `--out-name`        | Output file names, defaults to package name                |
| `--scope`           | The npm scope to use in package.json                       |
| `--mode`            | Steps to be run: no-install, normal, force                 |
| `--no-typescript`   | Disable TypeScript declaration file generation             |
| `--weak-refs`       | Enable usage of the JS weak references proposal            |
| `--reference-types` | Enable usage of WebAssembly reference types                |
| `--no-pack`         | Do not generate a package.json                             |
| `--no-opt`          | Skip optimization with wasm-opt                            |

See `wasm-pack build --help` for the full list of options.

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
