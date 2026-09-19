---
title: 快速上手
---

# 快速上手

## 环境要求

- Rust 1.97+，安装 `wasm32-unknown-unknown` target
- 已安装 `euv-cli`（`cargo install euv-cli`）

## 使用本模板

在 GitHub 上点击 **Use this template**，或直接克隆仓库：

```bash
git clone https://github.com/euv-dev/euv-docs.git my-docs
cd my-docs
```

## 写第一页

新建 `docs/guide/hello.md`：

```markdown
# 你好

我的第一页。
```

该文件自动出现在 `#/guide/hello.html`，并自动进入侧边栏。

## 构建

```bash
euv build -- --target web --out-dir www/pkg --out-name euv_docs --no-typescript --no-pack
```

静态站点产物输出到 `www/`。

## 开发服务器

```bash
euv run --dev --port 8080 -- --target web --out-dir www/pkg --out-name euv_docs --no-typescript --no-pack
```

然后打开 <http://localhost:8080>。

::: tip
开发服务器监听 `src/` 与 `docs/` —— 修改 markdown 会自动重新构建。
:::
