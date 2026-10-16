---
title: 原始 HTML
order: 5
---

`unsafe_no_inline!` 产出一个 `RawHtml` 值，内容**不转义**地插进 DOM。这是整个框架里唯一能绕过 HTML 转义的口子，所以宏名故意带了 `unsafe_no_inline` 这个前缀——编译器不会警告，但调用点在 code review 里一眼就能看见。

内容永远是可信的常量或你自己拼的字符串。**任何来自用户输入的内容都不要走这条路**，那等于直接开 XSS。

## 语法

```
unsafe_no_inline!("<svg viewBox=\"0 0 24 24\"></svg>")
```

接受一个字面量，展开成 `RawHtml::new(...)`：

```rust
::euv::RawHtml::new("<svg viewBox=\"0 0 24 24\"></svg>")
```

`RawHtml` 定义在 `core/src/vdom/raw/struct.rs:18`，`content` 字段是 `pub(crate)`，由 Lombok 的 `New` 派生 `RawHtml::new`。字段带公开 getter `get_content()`，供渲染器读取未转义内容；setter 和 mut-getter 都是 crate 内私有，所以内容**构造之后不能改**。

直接写 `RawHtml::new("...")` 也能编译，但拿不到那个响亮的安全提示，所以优先用宏。

## 两种插法，别搞混

### 在 html! 外面传递

`RawHtml` 的用途是**在 `html!` 之外**构造、然后传值：

```rust
let icon: RawHtml = unsafe_no_inline!("<svg viewBox=\"0 0 24 24\"><circle r=\"10\"/></svg>");
render_root(icon);
```

注意 **`RawHtml` 没有 `Into<VirtualNode>` 实现**，所以这样写**编译不过**：

```rust
// 错：RawHtml 不是 VirtualNode
html! {
    div {
        { unsafe_no_inline!("<b>hi</b>") }
    }
}
```

要让一个元素直接吃原始 HTML，用 `inner_html` 属性（见下）。

### inner_html 属性

`html!` 里想在某个元素上插未转义内容，走 `inner_html:`（`macros/src/html/const.rs:38`，常量名 `ATTR_KEY_INNER_HTML`）：

```rust
html! {
    div {
        inner_html: "<svg viewBox=\"0 0 24 24\"><circle r=\"10\"/></svg>"
    }
}
```

`inner_html` 接受 `String` / `&str`，渲染器走 `Element::set_inner_html` 而不是逐字设置子节点（`macros/src/html/fn.rs:1870`）。

两种写法的定位差别：

| | `inner_html:` | `unsafe_no_inline!` |
| --- | --- | --- |
| 用在哪 | `html!` 里元素上 | `html!` 之外 |
| 展开成 | `Element::set_inner_html(..)` | `RawHtml` 值 |
| 后续能否复用 | 不能，字符串内联在标签上 | 能，当普通值传来传去 |

所以规则很简单：**元素上直接插用 `inner_html`；需要跨函数传递、存进结构体字段、或条件性地构造两套不同内容，用 `unsafe_no_inline!` 产出一个 `RawHtml` 值。**

## 为什么不做成 Into<VirtualNode>

因为 `RawHtml` 的内容是**一整段字符串**，而 `VirtualNode` 是结构化节点树，两者没有一一对应关系——真要做转换得引入 HTML 解析器，那已经超出 vdom 的职责了。`RawHtml` 的定位就是渲染器写 `innerHTML` 时的一个「这段别转义」标记，而不是节点树的另一种表示。

## 安全性

这个宏是纯粹的手动挡：编译器不拦、运行时无保护，责任全在调用方。

- 只写常量或可信来源。
- 需要拼接时，先自己转义好再传进来，别指望这里处理。
- code review 看到 `unsafe_no_inline!` 就当成一个待确认项——名字起这么长就是为了这个。

## 延伸阅读

- [HTML 宏](./html.md) —— `inner_html` 属性的完整属性表。
- [组件宏](./component.md) —— `#[component]` 与 Props。
