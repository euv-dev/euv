---
title: 介绍
order: -1
---

# 介绍

**euv-docs** 把一个 markdown 目录变成完整的文档站 —— 就像 VuePress，
但由 Rust/WASM 的 [euv](https://github.com/euv-dev/euv) 应用渲染。

## 工作原理

1. 在 `docs/` 下编写 markdown 文件。
2. `build.rs` 解析它们（frontmatter + VuePress 风格 markdown），生成 Rust 站点模型。
3. `euv build` 把全部内容编译成单个 WASM 包。

## 目录映射

| 源文件 | 路由 |
| --- | --- |
| `docs/README.md` | `/` |
| `docs/guide/README.md` | `/guide/` |
| `docs/guide/getting-started.md` | `/guide/getting-started.html` |
| `docs/zh/README.md` | `/zh/` |

## 下一步

前往 [快速上手](./getting-started.md) 构建你自己的站点。
