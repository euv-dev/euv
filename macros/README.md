---
title: 过程宏
order: 0
---

<center>

## euv-macros

[![](https://img.shields.io/crates/v/euv-macros.svg)](https://crates.io/crates/euv-macros)
[![](https://img.shields.io/crates/d/euv-macros.svg)](https://img.shields.io/crates/d/euv-macros.svg)
[![](https://docs.rs/euv-macros/badge.svg)](https://docs.rs/euv-macros)
[![](https://github.com/euv-dev/euv/workflows/Rust/badge.svg)](https://github.com/euv-dev/euv/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/euv-macros.svg)](../LICENSE)

</center>

[Api Docs](https://docs.rs/euv-macros/latest/)

`euv-macros` 是 euv UI 框架的过程宏 crate，全部入口在 `src/lib.rs`。它提供 **7 个 function-like 过程宏 + 1 个属性宏**。

```shell
cargo add euv-macros
```

宏生成的代码一律使用**绝对路径**（`::euv::`、`::std::`），因此不依赖调用方的 `use` 导入。绝大多数情况下你不需要直接依赖本 crate —— `euv` 已经在根处重导出了全部宏（`src/lib.rs:6`：`pub use {euv_core::*, euv_macros::*};`），写 `use euv::*;` 即可。

## 宏清单

| 宏 | 类别 | 展开结果 | 文档 |
| --- | --- | --- | --- |
| `html!` | function-like | `euv::VirtualNode` 构造 | [HTML 宏](./html.md) |
| `class!` | function-like | `fn() -> &'static Css` / 参数化 `fn(..) -> Css` | [类名宏](./class.md) |
| `watch!` | function-like | 响应式副作用 | [响应式宏](./reactive.md) |
| `computed!` | function-like | 响应式派生信号 | [响应式宏](./reactive.md) |
| `vars!` | function-like | CSS 自定义属性块 | [类名宏](./class.md) |
| `var!` | function-like | `"var(--name)"` 字符串字面量 | [类名宏](./class.md) |
| `unsafe_no_inline!` | function-like | `euv::RawHtml::new(..)` | [原始 HTML](./raw.md) |
| `#[component]` | 属性宏 | **原样返回被标注的 item** | [组件宏](./component.md) |

`#[component]` 自身不生成任何代码（`lib.rs:220-223` 直接 `return item`），它的作用是让 `html!` 在**编译期扫描工程源码**、把该函数登记进组件表，从而把标识符 tag 解析成组件调用而不是原生元素。详见 [组件宏](./component.md)。

## 文档的边界

五个页面各自的分工：

- [HTML 宏](./html.md) —— `html!` 的语法（元素、属性、子节点、条件、循环、组件调用、`class:`/`style:`/事件/原始 html）以及每种写法展开出的 vdom 类型。
- [类名宏](./class.md) —— `class!` / `vars!` / `var!` 的定义语法、展开成的函数形态、样式注入时机与类名生成规则。
- [响应式宏](./reactive.md) —— `watch!` / `computed!` 的语法与展开结果，`html!` 里 `{表达式}` 的重渲染来源。
- [组件宏](./component.md) —— `#[component]` 的作用、Props 结构体约定、组件表的编译期扫描与调用点展开。
- [原始 HTML](./raw.md) —— `unsafe_no_inline!` 与 `RawHtml`，以及 `inner_html` 属性的取舍。

`class!` 与 `html!` 的交界点是 `class:` 属性：`html!` 侧的 `class: c_foo()` 只是普通表达式，`c_foo` 则是 `class!` 生成的函数。

`#[component]` 与 `html!` 的交界点是 tag 名：`html!` 靠组件表认出组件，其登记依赖 `#[component]` 标注。

## 展开出的运行时类型速查

| 类型 | 定义位置 | 说明 |
| --- | --- | --- |
| `VirtualNode<T = ()>` | `core/src/vdom/node/enum.rs:55` | 虚拟节点枚举 |
| `Tag` | `core/src/vdom/node/enum.rs:14` | `Element` / `Component` / `Portal`，载荷都是 `Cow<'static, str>` |
| `TextNode` | `core/src/vdom/node/struct.rs:28` | 文本节点，`content: Cow<'static, str>` |
| `AttributeEntry` | `core/src/vdom/attribute/struct.rs:15` | `name: Cow<'static, str>` + `value: AttributeValue` |
| `AttributeValue` | `core/src/vdom/attribute/enum.rs:8` | `Text` / `StaticText` / `Signal` / `Event` / `Dynamic` / `Css` / `CssRef` / `InnerHtml` / `InnerHtmlSignal` … |
| `Css` | `core/src/vdom/attribute/struct.rs:64` | `name` / `style` / `pseudo_rules` / `media_rules` |
| `RawHtml` | `core/src/vdom/raw/struct.rs:18` | 未转义的 HTML 字符串 |

`VirtualNode` 的完整形状：

```rust
pub enum VirtualNode<T = ()> {
    Element {
        tag: Tag,
        attributes: Vec<AttributeEntry>,
        children: Vec<VirtualNode>,
        key: Option<String>,
        props: Option<Box<T>>,
    },
    Text(TextNode),
    Fragment(Vec<VirtualNode>),
    Dynamic(DynamicNode),
    Empty,
}
```

## License

本项目基于 MIT 许可证，详见 [LICENSE](../LICENSE)。

## Contributing

欢迎贡献：请开 issue 或提交 pull request。

## Contact

问题请联系 [root@ltpp.vip](mailto:root@ltpp.vip)。