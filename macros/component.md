---
title: 组件宏
order: 4
---

`#[component]` 本身**一行代码都不生成**。它存在的唯一目的是让 `html!` 宏在编译期扫描你的工程源码，把带这个标注的函数登记进一张组件表，从而把 `html!` 里以该函数名作为 tag 的写法解析成组件调用，而不是原生元素。

## 属性宏本体

`macros/src/lib.rs:220-223` 的全部内容：

```rust
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
```

原样返回被标注的 item。所以 `#[component] pub fn foo(...) -> VirtualNode { .. }` 编译出来就是那个普通函数本身——不换名字、不加 wrapper、不改可见性。删掉这个属性，函数照样存在，只是 `html!` 不再认得它。

这意味着**属性标注必须在组件函数上**，标在别的位置没有任何作用。

## Props 结构体

组件的第一个参数固定是 `VirtualNode<Props类型>`，运行时从中取出数据：

```rust
use euv::*;

#[derive(Default)]
pub struct MyButtonProps {
    pub label: &'static str,
    pub onclick: Option<Rc<dyn Fn(Event)>>,
}

#[component]
pub fn my_button(node: VirtualNode<MyButtonProps>) -> VirtualNode {
    let MyButtonProps { label, onclick }: MyButtonProps =
        node.try_get_props().unwrap_or_default();
    html! {
        button {
            class: c_button()
            onclick: onclick
            { label }
        }
    }
}
```

在 `html!` 里用：

```rust
html! {
    my_button { label: "确定" }
}
```

两处约定：

- **Props 必须能 `Default`**。`try_get_props()` 返回 `Option<T>`，取不到时 `unwrap_or_default()` 兜底，所以字段得能 `Default`。
- **`VirtualNode<T>` 要能从 `Option` 里解出 `T`**，`try_get_props` 的实现在 `core/src/vdom/node/impl.rs:277`，返回 `Option<T>`。

## 组件是怎么被认出来的

`html!` 展开时调用 `load_component_registry()`（`macros/src/html/fn.rs:274`），它做这几件事：

1. 读环境变量 `CARGO_MANIFEST_DIR`，取不到就直接返回空表（于是 `html!` 里所有 tag 都当原生元素）。
2. 递归收集 `CARGO_MANIFEST_DIR/src/` 下的所有 `.rs` 文件。
3. 再收集**本地 path 依赖**的 `src` 目录，一并扫描。
4. 对这些文件算一个指纹（文件集合 + 修改时间），用它给 `OUT_DIR` 里的缓存做键。源码集合或修改时间一变，指纹就变，缓存失效、重扫。

从每个带 `#[component]` 的函数里提取三类信息，存进 `ComponentInfo`（`macros/src/html/struct.rs:256`）：

| 字段 | 含义 |
| --- | --- |
| 函数名 | 作为匹配用的 tag 标识符 |
| Props 类型名 | 用来构造 `VirtualNode<Props类型>` |
| 字段名 + 字段类型 | 决定 tag 上的属性分别填到哪个字段 |

## 调用点如何展开

`html!` 里每个 tag 都会生成一个 match 分支（`macros/src/html/impl.rs`，`HtmlDynamicTag` 的 `to_tokens`）。对组件名匹配的那个分支：

1. 把 tag 上的属性按 `props_field_types` 归类——**能对应上 Props 字段名的进 Props 字面量，其余的（`class` / `style` / 事件处理）留到后面**。
2. 用收集到的字段名和字段类型拼出 `MyButtonProps { .. }` 字面量。
3. 包成 `VirtualNode<MyButtonProps>` 调组件函数。
4. 把剩下的 `class` / `style` / 事件**接到组件返回的根元素上**。

最后一条是这套设计最实用的地方：

```rust
html! {
    my_button {
        class: c_cta_primary()   // 不在 Props 里 -> 接到组件返回的 <button> 上
        label: "确定"
    }
}
```

所以 Props 只管业务数据，样式和事件照常在调用点写，不必为每个组件都往 Props 里塞 `class`。

## 命名即 tag

match 的分支条件是**函数名**（`component_match_arm_tokens` 里 `Ident::new(fn_name, ..)`），没有前缀、没有命名空间。所以：

- 函数名就是 tag 名，大小写敏感。
- **同名函数会互相覆盖组件表里的条目**。跨 crate 用同一函数名定义两个组件是安全的（各自在自己的 `CARGO_MANIFEST_DIR` 下扫描），但同一个 crate 里不行。

## 限制

- **只在构建期的源码目录里找**。函数如果在 `src/` 之外（比如由 `include!` 引入、或放在生成代码里），扫不到。
- **只扫本地 path 依赖**。从 crates.io 装的依赖源码不会被扫描，所以第三方 crate 里的组件在本工程里认不出来。
- **代价是每次 `html!` 调用都要查表**。缓存（指纹 + `OUT_DIR`）就是为了把这部分压下去；没有 build script 的 crate 会退回到系统临时目录下的 per-manifest 缓存目录。

## 延伸阅读

- [HTML 宏](./html.md) —— 组件在 `html!` 里的调用语法、属性分类规则。
- [原始 HTML](./raw.md) —— `unsafe_no_inline!` 与 `RawHtml`。
