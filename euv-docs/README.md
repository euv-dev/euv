---
# Site-level configuration for the euv-docs demo home page
# (consumed by build.rs when EUV_DOCS_SRC_DIR is unset, e.g.
# during `cargo clippy` / `cargo test` in the euv-docs crate itself).
site:
  title: euv-docs
  description: A VuePress-style documentation site generator powered by euv + euv-ui.
  logo: "📘"
locales:
  - prefix: /
    lang: en-US
    label: English
    title: euv-docs
    description: A VuePress-style documentation site generator powered by euv + euv-ui.
    footer: MIT Licensed | Built with euv + euv-ui
    toc_label: On this page
    prev_label: Previous
    next_label: Next
    navbar:
      - { text: Home, link: / }
      - { text: Guide, link: /guide/ }
      - { text: GitHub, link: https://github.com/euv-dev/euv }
---

# euv-docs

A **VuePress-style documentation site generator** built with
[euv](https://github.com/euv-dev/euv) + `euv-ui`, compiled to WebAssembly.

Write markdown in any directory containing a `README.md` — get a full docs
site with a home hero, navbar, multi-level collapsible sidebar, right anchor
TOC, prev/next links, footer, dark mode, and i18n.

## Quick start

```bash
cargo install euv-cli

# dev server with hot reload (run from this directory)
euv run --dev --port 8080 --index-html euv-docs/template.html -- --target web --out-dir www/pkg --out-name euv_docs --no-typescript --no-pack

# production build → www/
euv build --release --index-html euv-docs/template.html -- --target web --out-dir www/pkg --out-name euv_docs --no-typescript --no-pack
```

Open <http://localhost:8080> after `euv run`.

## Writing docs

| Source file                                   | Route                             |
| --------------------------------------------- | --------------------------------- |
| `cli/docs/README.md`                          | `/` (home, with frontmatter hero) |
| `cli/docs/guide/README.md`                    | `/guide/` (sidebar group index)   |
| `cli/docs/guide/getting-started.md`           | `/guide/getting-started.html`     |
| `cli/docs/zh/README.md`                       | `/zh/` (locale home)              |

- **Site config (title, locales, navbar, footer, labels)** —
  `cli/README.md` frontmatter (single source of truth, replaces the
  legacy `docs/config.toml`).
- **Sidebar** — auto-generated from the file tree; order with
  frontmatter `order: <int>`.
- **Home page** — frontmatter `home: true` + `heroText` / `tagline` /
  `actions` / `features` / `footer`.
- **Static assets** — put them in `cli/docs/public/`, reference as
  `/logo.png`.

## Building a site from any markdown directory

The `euv-docs` binary builds any directory containing a top-level
`README.md` (with site config in its frontmatter) plus `**/*.md`
content files into a static site. Use this to ship documentation for a
project without modifying euv-docs itself.

```bash
cargo install --path euv-docs --bin euv-docs

euv-docs <SRC_DIR> [--out <OUT_DIR>] [--name <NAME>] [--index-html <FILE>] [--debug]
```

| Argument              | Default     | Meaning                                                                                                  |
| --------------------- | ----------- | -------------------------------------------------------------------------------------------------------- |
| `<SRC_DIR>`           | (required)  | Directory containing the markdown tree.                                                                 |
| `--out`               | `www`       | Output directory.                                                                                       |
| `--name`              | `euv_docs`  | Output package name (`<name>.js` / `<name>_bg.wasm`).                                                  |
| `--index-html`        | (none)      | Path to a custom `index.html` template (defaults to `template.html` shipped in the binary).              |
| `--debug`             | (off)       | Enable debug logging.                                                                                   |

## Crate structure

```
euv-docs/
├── src/
│   ├── bin/euv-docs/      CLI entry: arg parsing, build orchestration
│   ├── component/         VuePress-style layout components
│   │   ├── doc_page/      Router-level home / doc / 404 switch
│   │   ├── home_page/     Hero + feature grid + footer
│   │   ├── layout/        Navbar + sidebar shell
│   │   ├── not_found/     404 page
│   │   └── password_gate/ Private page protection
│   ├── data/              Generated `DocsSite` types (consumed by `build.rs`)
│   ├── router/            Hash-route parser / link handler
│   └── lib.rs             WASM entry: inject global styles, mount app
├── build.rs               Codegen `docs_gen.rs` from `**/*.md` + README frontmatter
├── template.html          Default `<base href="./">` shell
├── Cargo.toml             `[package] euv-docs`, `euv = "0.18"` pin (CSS safe-area)
└── docs/                  (this crate ships demo content only — real docs
                           live at `cli/docs/`)
```

## CSS safe-area pin

The `euv = "0.18"` pin in `Cargo.toml` is **intentional** (PR #13) and
must not be floated to `*`. The euv-docs layout depends on the
`safe-area` CSS class shipped by `euv-ui 0.18.x` to make the
mobile navigation header sit beneath the system status bar in
Tauri / WebView hosts. Floating to `euv 0.24` would silently break
this and leave the header pinned under the status bar.
