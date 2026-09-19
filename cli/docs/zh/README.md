---
home: true
heroText: euv-docs
tagline: 由 euv + euv-ui 驱动的 VuePress 风格文档站，编译为 WebAssembly 运行。
actions:
  - text: 快速上手 →
    link: /zh/guide/getting-started.html
    type: primary
  - text: 介绍
    link: /zh/guide/
    type: secondary
features:
  - icon: 📝
    title: Markdown 驱动
    details: 在 docs/ 下写 .md 文件即可 —— 页面、侧边栏、锚点目录全部在构建期自动生成。
  - icon: 🧭
    title: VuePress 布局
    details: 首页 Hero、导航栏、多级可折叠侧边栏、右侧锚点目录、上/下一页与页脚 —— 你熟悉的布局。
  - icon: 🦀
    title: Rust + WASM
    details: 整个站点是一个 euv WASM 应用 —— 响应式、可切换明暗主题、运行飞快。
footer: MIT 许可 | 基于 euv + euv-ui 构建
---

## 你好，euv-docs

这是首页正文。分割线以上的内容全部来自本文件的 frontmatter，以下就是普通 markdown。

```rust
fn main() {
    println!("文档即代码，渲染为 WASM。");
}
```
