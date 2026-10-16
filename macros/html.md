---
title: HTML 宏
order: 1
---

`html!` 是 euv 声明式 UI 的入口宏。语法接近 Dioxus HTML：元素写作 `div { ... }`，属性写作 `key: value`，子节点直接罗列，**花括号 `{expr}` 表示响应式表达式**。

宏把整棵树展开成 `::euv::VirtualNode` 构造代码，展开结果全部使用绝对路径，因此不需要预先 `use` 任何东西。

## 概览

| 语法 | 展开为 |
| --- | --- |
| `div { .. }` | `VirtualNode::Element { tag: Tag::Element(Cow::Borrowed("div")), .. }` |
| `component_fn { .. }`（`#[component]`） | `component_fn(VirtualNode::Element { tag: Tag::Component(..), props: Some(Box::new(..)), .. })` |
| `"tag" { .. }` | 同 `div`，但强制走 `Tag::Element`（自定义元素） |
| `portal { target: ".." }` | `VirtualNode::Element { tag: Tag::Portal(..), .. }` |
| `"文本"` | `VirtualNode::Text(TextNode::new(Cow::Borrowed("文本"), None))` |
| `{expr}` | `VirtualNode::create_dynamic(move \|_\| (expr).into())` |
| `expr`（裸表达式） | `(expr).into()` |
| `if {s} { .. }` | `VirtualNode::create_dynamic(move \|_\| { if s.get() { .. } else { .. } })` |
| `for x in {s} { .. }` | `VirtualNode::create_dynamic(… Fragment(collect) …)` |
| `match {s} { .. }` | `VirtualNode::create_dynamic(move \|ctx\| { match s.get() { .. } })` |

顶层节点数量决定最终形态（`html/impl.rs:20-24`、`html/fn.rs:1107-1143`）：0 个 → `VirtualNode::Empty`；1 个 → 直接是该节点的 token；N 个 → `VirtualNode::Fragment(vec![..])`。

## 元素与属性

真实用例（`example/src/page/binding/view/fn.rs:230-241`）：

```rust
input {
    id: "color-mixer-red"
    name: "red"
    type: "range"
    autocomplete: "off"
    min: "0"
    max: "255"
    value: red_value.to_string()
    class: c_binding_slider()
    class: c_slider_value(&format!("{}%", red_value * 100 / 255))
    oninput: cross_on_input_i32(red_signal)
}
```

展开为：

```rust
::euv::VirtualNode::Element {
    tag: ::euv::Tag::Element(::std::borrow::Cow::Borrowed("input")),
    attributes: vec![
        ::euv::AttributeEntry::new(
            ::std::borrow::Cow::Borrowed("id"),
            ::euv::AttrValueAdapter::new("color-mixer-red").into(),
        ),
        // .. 其余属性
    ],
    children: ::euv::VirtualNode::Empty,
    key: None,
    props: None,
}
```

规则要点：

- **属性名支持 kebab-case**（`data-observer-container`、`aria-label`），宏内部按 `-` 逐段拼接（`ident/fn.rs:96-117`）。同时也接受 Rust 原始标识符 `r#type`。
- 属性名写 `::`（Rust 路径）时不会被误判为属性键（`html/fn.rs:103-110`）。
- 属性值的花括号是可选的：`class: STATIC_CLASS` 与 `class: { STATIC_CLASS }` 等价（`html/fn.rs:1744-1771`）。歧义消解规则：裸标识符后面紧跟 `{` 时，宏会窥探花括号内是否以 `ident:` 开头——是则当结构体字面量，否则当作"静态值 + 后续子节点"。
- **重复属性**：`class:` 与 `style:` 会合并为 `AttributeValue::merge_class(&[..])` / `merge_style(&[..])`；其他属性只保留最后一次（`html/fn.rs:1774-1834`）。

自定义元素用字符串字面量 tag（`macros/tests/html_static_style/fn.rs:5-13`）：

```rust
let _node: ::euv::VirtualNode = html! {
"div" {
    style: {
        color: "red";
    }
    "x"
}
};
```

> 注意：字符串字面量 tag 必须是合法的 Rust 标识符。`"custom-tag"` 会让宏 panic，错误信息是 `"custom-tag" is not a valid identifier`（`html/impl.rs:299` 用 `Ident::new(&tag_name, ..)` 构造 tag 标识符）。带连字符的自定义元素请走动态标签形式。

## 子节点与文本

三种子节点写法，语义完全不同：

| 写法 | 展开 | 响应式 |
| --- | --- | --- |
| `"literal"` | `VirtualNode::Text(TextNode::new(Cow::Borrowed("literal"), None))` | 否 |
| `{expr}` | `VirtualNode::create_dynamic(move \|_\| (expr).into())` | 是 |
| `expr`（裸） | `(expr).into()` | 否，一次性转换 |

真实用例（`example/src/page/binding/view/fn.rs:211-221`）：

```rust
html! {
    div {
        class: c_binding_color_mixer()
        div {
            class: c_binding_color_preview()
            class: c_binding_color_preview_bg(&hex_value)
            span {
                class: c_binding_color_hex()
                hex_value
            }
        }
```

裸表达式子节点（`example/src/page/list/view/fn.rs:69-72`）：

```rust
span {
    class: c_list_item_text()
    item.clone()
}
```

字面量文本以 `Cow::Borrowed` 发射，字符串在二进制只读段只存一份，每次渲染复用同一块切片，零分配（`html/impl.rs:426-441`）。

## 自动 `.get()`

响应式位置（`{expr}`、`if {..}`、`match {..}`、`for x in {..}`）里，**单段标识符路径**会被自动补上 `.get()`（`html/fn.rs:53-87`）：

```rust
if { on } { .. }              // 展开为 if on.get() { .. }
match { tab } { .. }          // 展开为 match tab.get() { .. }
for item in { items } { .. }   // 展开为 for item in items.get() { .. }
```

链式表达式**不会**被自动解包，必须自己写 `.get()`：`state.field` → `state.field.get()`，`sig.iter()` → `sig.get().iter()`，`sig == X` → `sig.get() == X`。

`example/src/page/attrs/view/fn.rs:65` 就是显式写法：

```rust
if { !dynamic_key.get().is_empty() } {
```

## 条件

真实用例（`example/src/page/websocket/view/fn.rs:43-61`）：

```rust
if { state.get_connecting().get() } {
    euv_button {
        variant: EuvButtonVariant::Primary
        label: WEBSOCKET_WAIT_LABEL
        disabled: state.get_connecting()
    }
} else if { state.get_connected().get() } {
    euv_button {
        variant: EuvButtonVariant::Primary
        label: WEBSOCKET_DISCONNECT_LABEL
        onclick: websocket_on_disconnect(state)
    }
} else {
    euv_button {
        variant: EuvButtonVariant::Primary
        label: WEBSOCKET_CONNECT_LABEL
        onclick: websocket_on_connect(state)
    }
}
```

条件与花括号可以混用：`if a {} else if {b} {}` 与 `if {a} {} else if b {}` 都合法。**只要有一个分支带花括号，整条 if 链就包进 `VirtualNode::create_dynamic`**（`html/struct.rs:67-90`）。省略 `else` 时补 `VirtualNode::Empty`。

响应式展开：

```rust
::euv::VirtualNode::create_dynamic(move |_: &mut ::euv::HookContext| {
    if sig.get() {
        ::euv::VirtualNode::Element { /* .. */ }
    } else {
        ::euv::VirtualNode::Element { /* .. */ }
    }
})
```

内联（非花括号）`if` 也支持，只在渲染时求值一次：

```rust
if flag {
    span { "yes" }
} else {
    span { "no" }
}
```

## 循环

真实用例（`example/src/page/list/view/fn.rs:61-83`）：

```rust
ul {
    class: c_list_ul()
    data_list_container: "true"
    for (index, item) in state.get_items().get().iter().enumerate() {
        li {
            key: index.to_string()
            class: c_list_item()
            data_index: index.to_string()
            span {
                class: c_list_item_text()
                item.clone()
            }
            div {
                class: c_list_item_button()
                euv_button {
                    variant: EuvButtonVariant::Primary
                    label: "Remove"
                    onclick: todo_list_on_remove(state.get_items(), index)
                }
            }
        }
    }
}
```

响应式循环 `for item in { items } { .. }` 展开为：

```rust
::euv::VirtualNode::create_dynamic(move |_: &mut ::euv::HookContext| {
    let mut __euv_inner: Vec<::euv::VirtualNode> = Vec::with_capacity(8);
    for item in items.get() {
        __euv_inner.extend(/* 每轮产生的节点 */);
    }
    ::euv::VirtualNode::Fragment(__euv_inner)
})
```

内联循环 `for x in expr { .. }` 不包 `create_dynamic`，直接展开成同形状的 `Fragment` 字面量。

> 响应式循环体里不要直接移动循环变量。`for item in { items } { li { { item } } }` 会报 `cannot move out of value, a captured variable in an FnMut closure`，因为外层是 `move` 闭包且 `create_dynamic` 要求 `FnMut`。写 `{ item.clone() }`。

## match

真实用例（`example/src/page/conditional/view/fn.rs:95-120`）：

```rust
match { user_type } {
    ConditionalUserType::Guest => {
        div {
            p {
                class: c_role_guest_text()
                "Welcome, guest! Please sign in to access more features."
            }
        }
    }
    ConditionalUserType::User => {
        div {
            p {
                class: c_role_user_text()
                "Hello, user! You have standard access."
            }
        }
    }
    _ => {
        div {
            p {
                class: c_role_admin_text()
                "Welcome, administrator! You have full access."
            }
        }
    }
}
```

展开为（`html/impl.rs:493-499`）：

```rust
::euv::VirtualNode::create_dynamic(move |__euv_hook_context: &mut ::euv::HookContext| {
    match user_type.get() {
        ConditionalUserType::Guest => {
            __euv_hook_context.switch_arm(0usize);
            /* 该臂的节点 */
        }
        ConditionalUserType::User => {
            __euv_hook_context.switch_arm(1usize);
            /* .. */
        }
        _ => {
            __euv_hook_context.switch_arm(2usize);
            /* .. */
        }
    }
})
```

`switch_arm` 让运行时在换臂时清理旧臂的 hook。

内联 `match expr { .. }` 只求值一次，不发 `switch_arm`。

### 子节点位置的 match 目前只能用单臂

这是一个**已验证的宏缺陷**。子节点位置的 `match`（无论响应式还是内联）展开时把每个臂直接写进 `match` 的 Rust 语法位置，却没有补逗号：

```rust
match #scrutinee_tokens {
    #pattern => { #body_expr }   // 缺逗号
    #pattern => { #body_expr }
}
```

实测结果：

| 写法 | 位置 | 结果 |
| --- | --- | --- |
| `match { sig } { a => {..} b => {..} }` | 子节点 | ✅ 编译通过（响应式路径走 `create_dynamic`，臂内可多节点） |
| `match { sig } { a => {..}, b => {..}, }` | 子节点 | ✅ 编译通过（手写逗号可绕过） |
| `match flag { true => {..} false => {..} }` | 子节点 | ❌ `error: expected ',' following 'match' arm` |
| `match flag { true => {..}, false => {..}, }` | 子节点 | ❌ 同样报缺逗号（手写逗号无效） |
| `match kind { A => {..} B => {..} }` | 属性 `class:` | ✅ 编译通过 |

内联子节点 match 用 `match expr { .. }`（`html/impl.rs:500-521`），那条分支不补逗号；响应式子节点 match（`html/impl.rs:472-499`）补了逗号——因此只有**内联 + 子节点**这一个组合会炸。属性位置的 match 两条路径都补逗号（`html/fn.rs:1616-1627`），不受影响。

**变通办法**：把内层 match 改成响应式形式 `match { expr } { .. }`，或者把 match 提到组件 props / `class:` 属性位置。

## 组件调用

`html!` 在**编译期扫描工程源码**（`CARGO_MANIFEST_DIR/src` 加上本地 path 依赖的 `src`），找出带 `#[component]` 的函数并登记 `函数名 → Props 类型 + 字段名 + 字段类型`（`html/fn.rs:262-283`）。带 `#[component]` 的标识符 tag 才会被当成组件；其它标识符 tag 一律是原生元素。详见 [组件宏](./component.md)。

真实用例（`example/src/page/binding/view/fn.rs:391-396`）：

```rust
limited_counter {
    disabled: typed_state.get_disabled()
    max_count: typed_state.get_max_count()
    on_increment: typed_props_on_increment(typed_state.get_current_count(), typed_state.get_max_count(), typed_state.get_disabled())
    on_reset: typed_props_on_reset_count(typed_state.get_current_count(), typed_state.get_disabled())
}
```

展开为：

```rust
limited_counter(::euv::VirtualNode::Element {
    tag: ::euv::Tag::Component(::std::borrow::Cow::Borrowed("limited_counter")),
    attributes: Vec::new(),
    children: vec![/* 块内非属性节点 */],
    key: None,
    props: Some(Box::new(LimitedCounterProps {
        disabled: typed_state.get_disabled(),
        max_count: typed_state.get_max_count(),
        on_increment: typed_props_on_increment(/* .. */),
        on_reset: typed_props_on_reset_count(/* .. */),
    })),
})
```

Props 字段全部给齐时不写 `..Default::default()`，只给一部分时补 `..Default::default()`（`html/impl.rs:1179-1185`）。

单字段可以直接写字段名当简写（`example/src/component/layout/view/fn.rs:61-68`）：

```rust
main {
    class: c_app_main()
    page_router {
        route_signal
    }
}
euv_vconsole_panel {
    panel_open
}
```

块内的子节点（元素、表达式）进入 `children`，组件里用 `node.get_children().into()` 取回（`ui/src/component/card/view/fn.rs:15`）：

```rust
let children: VirtualNode = node.get_children().into();
```

## 动态标签

双花括号形式 `{tag_expr} { 属性与子节点 }`。运行时把 `tag_expr` 转成字符串，先匹配组件名，匹配不上则退回原生元素（`html/impl.rs:766-809`）。

真实用例（`example/src/page/dynamic/view/fn.rs:99-110`）：

```rust
div {
    class: c_dynamic_component_panel()
    {
        tag_name_opt.get()
    } {
        title: "Dynamic euv_card"
        onclick: badge_on_click("Dynamic Badge", LogLevel::Log)
        {
            content.get()
        }
    }
}
```

展开为：

```rust
::euv::VirtualNode::create_dynamic(move |_: &mut ::euv::HookContext| {
    let __euv_tag_name: String = (tag).to_string();
    match __euv_tag_name.as_str() {
        "probe_card" => {
            probe_card(::euv::VirtualNode::Element {
                tag: ::euv::Tag::Component(Cow::Borrowed("probe_card")),
                attributes: Vec::new(),
                children: vec![/* .. */],
                key: None,
                props: Some(Box::new(ProbeProps::default())),
            }).extend_attributes([/* class / style / on* 等非 props 属性 */])
        }
        _ => {
            ::euv::VirtualNode::Element {
                tag: ::euv::Tag::Element(Cow::Owned(__euv_tag_name)),
                attributes: vec![/* .. */],
                children: vec![/* .. */],
                key: None,
                props: None,
            }
        }
    }
})
```

要点：

- 花括号里的表达式必须是**能 `.to_string()` 的**（`String` / `&str`）。`Signal<String>` 不能直接用，写 `{ tag.get() }`。
- 命中组件时，匹配 props 字段的属性进 Props，剩下的 `class` / `style` / `on*` 属性用 `.extend_attributes([..])` 拼到组件返回节点上。
- 动态标签没有 `key:` 支持（`key: None` 固定）。

## 动态属性名

花括号键 `{expr}: value`（`html/impl.rs:355-361`）。真实用例（`example/src/page/attrs/view/fn.rs:55-64`）：

```rust
div {
    {
        dynamic_key.get()
    }
    : dynamic_value
    class: c_custom_attrs_demo()
    p {
        class: c_demo_text()
        "This div has a dynamic attribute set by the inputs above."
    }
```

静态键展开为 `AttributeEntry::new(Cow::Borrowed("id"), ..)`，动态键展开为 `AttributeEntry::new(Cow::Owned(Into::<String>::into(expr)), ..)`（`html/fn.rs:2067-2094`）。

## class 绑定

`class:` 的值就是普通表达式，可以写 `class!` 生成的类函数、字符串、`Signal<String>`、`&'static Css`。多条 `class:` 会合并：

```rust
div {
    class: root_class_signal
    class: c_example_theme()
```

真实位置：`example/src/component/layout/view/fn.rs:20-23`。

展开为 `AttributeValue::merge_class(&[..])`，运行时把各段类名用空格拼起来；其中只要有一段是 `Signal`，整条 `class` 就变成响应式的 `AttributeValue::Signal`（`core/src/vdom/attribute/impl.rs:96-108`）。

条件类（`example/src/page/list/view/fn.rs:39-43`）：

```rust
class: if { state.get_add_error().get().is_empty() } {
    c_list_input()
} else {
    c_list_input_error()
}
```

展开为 `AttributeValue::reactive(move || { if .. { String::from(..) } else { (..).to_string() } })`。无 `else` 时隐式补空字符串。

参数化类（`example/src/page/binding/view/fn.rs:239`）：

```rust
class: c_slider_value(&format!("{}%", red_value * 100 / 255))
```

类名宏的细节见 [类名宏](./class.md)。

## style 绑定

两种写法都能编译，展开不同：

**样式对象** `style: { key: value; .. }`（`html/fn.rs:1699-1739`）。全部字面量时在编译期拼成静态字符串：

```rust
style: { color: "red"; }
```

```rust
::euv::AttributeValue::StaticText("color: red;")
```

多个字面量属性一起拼成一条静态字符串；含表达式时走 `Css::style_string(&[(k, v), ..])`；含 `if` / `match` 时走 `AttributeValue::reactive(move || Css::style_string_owned(..))`。

**整串样式** `style: SOME_CSS_STRING`，真实用例（`example/src/page/gesture/view/fn.rs:90`）：

```rust
div {
    id: GESTURE_READOUT_CONTAINER_ID
    class: c_event_touch_zone()
    class: c_event_drag_zone()
    style: GESTURE_PAD_STYLE
```

值定义在同文件 `const.rs:20-21`：

```rust
pub(crate) const GESTURE_PAD_STYLE: &str =
    "min-height: 340px; touch-action: none; user-select: none;";
```

多条 `style:` 同样会合并为 `AttributeValue::merge_style(&[..])`（`ui/src/component/virtual_list/view/fn.rs:127-130` 有真实用例）。

## 事件

事件属性**不带 `on:` 前缀**，直接写 `on` + 事件名：`onclick`、`oninput`、`ontouchstart`、`onkeydown`、`onscroll`……宏识别 `on` 前缀的键（`html/const.rs:44`）。

原生元素走 `EventNamedAdapter::new(expr, "click")`（`html/fn.rs:2125-2128`）；组件 props 上的回调走 `CallbackNamedAdapter::new(expr, "on_increment")`（`html/fn.rs:1857-1866`）。

真实用例（`example/src/page/binding/view/fn.rs:240`）：

```rust
oninput: cross_on_input_i32(red_signal)
```

命名 handler 变量（`example/src/page/gesture/view/fn.rs:91-94`）：

```rust
div {
    style: GESTURE_PAD_STYLE
    ontouchstart: state.on_start
    ontouchmove: state.on_move
    ontouchend: state.on_end
    ontouchcancel: state.on_cancel
```

组件上的 `on_*` props（`example/src/page/badge/view/fn.rs:37-42`）：

```rust
euv_tag {
    color: EuvTagColor::Black
    variant: EuvTagVariant::Solid
    text: BADGE_BLACK_TAG_TEXT
    on_click: badge_on_click(BADGE_BLACK_TAG_TEXT, LogLevel::Log)
}
```

处理器通常在 `html!` 外面构造好再传入（`example/src/page/event/view/fn.rs:83-87`）：

```rust
let on_double_click: Box<dyn FnMut(Event)> = Box::new(move |_: Event| {
    let current: i32 = mouse.get_double_click_count().get();
    mouse.get_double_click_count().set(current + 1);
    Console::log(format!("DblClick: #{}", current + 1));
});
```

## 特殊属性

| 键 | 行为 | 展开 |
| --- | --- | --- |
| `key: expr` | 元素身份，驱动 diff | 不进 attributes，写进 `key: Some((expr).into())` |
| `inner_html: expr` | 用未转义 HTML 替换子树，语义等同 React 的 `dangerouslySetInnerHTML` | `AttributeValue::InnerHtml` / `InnerHtmlSignal` |
| `ref: node_ref` | DOM 节点引用，类型 `NodeRef<T>` | `AttributeValue` 的 `From<NodeRef<T>>` |
| `children: expr` | 显式传入 children | `AttributeValue::Dynamic(Box::new(expr))` |

`key:` 真实用例见 `example/src/page/list/view/fn.rs:66`；`ref:` 与 `inner_html:` 的真实用例在 ui crate 里（`ui/src/component/virtual_list/view/fn.rs:120-125`、`ui/src/component/markdown/view/fn.rs:177-181`）：

```rust
html! {
    div {
        class: c_virtual_list_container()
        id: container_id
        ref: container_ref
        onscroll: scroll_handler
        div {
            style: scroll_spacer_style_ref
            div {
                style: row_wrapper_style_ref
                children
            }
        }
    }
}
```

`inner_html` 接受 `String` / `&str`（静态）或 `Signal<String>`（响应式），由 `InnerHtmlAdapter` 的 `From` 实现按类型推断分派（`html/fn.rs:2131-2138`）。

## portal

`portal { target: "<CSS 选择器>" .. }` 是伪元素，展开为 `Tag::Portal(Cow::Owned(String::from(target)))`。渲染器在挂载时用 `document.query_selector` 解析目标，匹配不到时退回 `document.body`；原位留一个 HTML 注释标记，保证父节点按下标遍历的结构完整。

```rust
::euv::VirtualNode::Element {
    tag: ::euv::Tag::Portal(Cow::Owned(String::from("#root"))),
    attributes: vec![/* target 被消费，不出现在这里 */],
    children: vec![/* .. */],
    key: None,
    props: None,
}
```

`target:` 必填，缺失时展开为 `compile_error!("portal element requires a `target:` attribute")`（`html/const.rs:127`）。`target:` 的值走 `String::from(expr)`，所以 `Signal<String>` 要写 `target: sig.get()`。`example/` 里目前没有 portal 用例。

## raw html

`html!` 里的原始 HTML 走 `inner_html:` 属性（见上表），而不是子节点位置。`unsafe_no_inline!` 产出的是 `RawHtml` 值，用于在 `html!` **之外**构造并传递；`RawHtml` 没有 `Into<VirtualNode>` 实现，`{ unsafe_no_inline!("..") }` 直接当子节点会编译失败。详见 [原始 HTML](./raw.md)。

## 错误信息

| 错误 | 触发条件 | 定义 |
| --- | --- | --- |
| `expected an element, string literal, if, match, for, or expression` | 顶层遇到无法识别的 token | `html/const.rs:62` |
| `unexpected token in HTML element` | 元素体内出现无法识别的 token | `html/const.rs:66` |
| `unexpected token in HTML` | 子节点位置出现无法识别的 token | `html/const.rs:69` |
| `unexpected token in dynamic component` | 动态标签体内出现无法识别的 token | `html/const.rs:72` |
| `portal element requires a \`target:\` attribute` | `portal` 缺 `target:` | `html/const.rs:127` |
| `expected identifier` | 标识符片段缺失（属性名 / 选择器解析共用） | `ident/const.rs:1` |
| `proc macro panicked ... is not a valid identifier` | 字符串字面量 tag 不是合法标识符（如 `"custom-tag"`） | `html/impl.rs:299`（`Ident::new(&tag_name, ..)`） |

## 交叉引用

- [响应式宏](./reactive.md) —— `watch!` / `computed!` / `vars!` / `var!`，以及 `html!` 中 `{expr}` 的响应式语义来源。
- [组件宏](./component.md) —— `#[component]` 属性、Props 结构体、组件注册表的扫描规则。
- [类名宏](./class.md) —— `class!` / `vars!` / `var!` 定义语法与展开结果。
- [原始 HTML](./raw.md) —— `unsafe_no_inline!` 与 `RawHtml`。