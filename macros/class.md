---
title: 类名宏
order: 2
---

`class!` 用来定义样式类。**每个类定义展开成一个普通 Rust 函数**，返回 `euv::Css`；首次调用时把样式注入 DOM，之后从 `OnceLock`（无参）或 `Mutex<HashMap>`（有参）缓存取回。`vars!` 与 `var!` 是配套的 CSS 自定义属性宏。

## 概览

| 宏 | 定义形态 | 展开 |
| --- | --- | --- |
| `class!` | `可见性 名字 [(形参: 类型, ..)] { .. }` | `fn 名字() -> &'static Css` 或 `fn 名字(..) -> Css` |
| `vars!` | `可见性 名字 [(形参: 类型, ..)] { 变量名: 值; .. }` | 同上，变量名自动加 `--` 前缀 |
| `var!` | `var!(名字)` / `var!("名字")` | 字符串字面量 `"var(--名字)"` |

`vars!` / `var!` 的完整用法见本文末尾。

## 基本语法

真实用例（`example/src/style/class/fn.rs:3-10`）：

```rust
class! {
    pub(crate) c_game_stats_bar {
        display: "flex";
        gap: "16px";
        margin-bottom: "12px";
        font-size: "14px";
        font-weight: "600";
    }
```

最简形态（`macros/tests/class/fn.rs:3-11`）：

```rust
class! {
    c_test_red {
        color: "red";
    }
    c_test_blue {
        color: "blue";
        background: "white";
    }
}
```

`class!` 块内可出现四种条目，可以任意混写、任意顺序：

| 条目 | 语法 |
| --- | --- |
| 普通属性 | `key: 值;` |
| 伪类 / 伪元素块 | `:hover { .. }`、`::before { .. }`、`::-webkit-scrollbar-thumb:hover { .. }` |
| at-rule 块 | `@media (条件) { .. }` |
| 继承（extends） | `父类(实参);` |

展开成（无参、无泛型，全静态路径，`macros/src/class/fn.rs:1146-1155` 原文）：

```rust
pub fn c_a() -> &'static ::euv::Css {
    static C_A: ::std::sync::OnceLock<::euv::Css> = ::std::sync::OnceLock::new();
    C_A.get_or_init(|| {
        let css: ::euv::Css = ::euv::Css::new(
            "c_a".to_string(),
            "color: red; ".to_string(),
            Vec::new(),
            Vec::new(),
        );
        css.inject_style();
        css
    })
}
```

`static` 常量名 = 类名 `to_uppercase()`。类名本身就是注入 CSS 的类选择器，渲染时 `class: c_a()` 会把 `"c_a"` 拼进 `class` 属性。

### 属性名与属性值

- 属性名走 kebab-case 标识符：`font-size`、`-webkit-user-select`、`grid-template-columns` 都可以；`type`、`color` 这类 Rust 关键字也能直接写（宏按 `TokenTree` 解析，不走 `Ident`）。
- 属性值是**完整的 Rust 表达式**（`macros/src/class/impl.rs:164`）：字符串字面量、`format!(..)`、函数调用都可以。
- 重复属性会**全部保留**，不做去重：`c_dup { color: "red"; color: "blue"; }` → `style = "color: red; color: blue; "`。
- 空类体合法：`c_empty {}` → `style = ""`。
- 属性值里带 `if` / `match` 时**必须用花括号包起来**：`color: { if flag { "red" } else { "blue" } }`。裸写 `color: if flag { .. }` 会被误判成元素选择器块并报 `error: expected ':'`。

## 参数化类

真实用例（`example/src/style/class/fn.rs:166-179`）：

```rust
pub(crate) c_binding_color_preview_bg(background: &str) {
    background: {
        background
    };
}

pub(crate) c_slider_value(value_percent: &str) {
    {
        "--value"
    }
    : {
        value_percent
    };
}
```

多参数 + 动态键与值（`ui/src/style/class/page/fn.rs:1012-1016`）：

```rust
pub c_attrs_dynamic_demo(prop_key: &str, prop_value: &str) {
    {
        prop_key
    }
    : prop_value;
```

裸标识符也可以直接当值（`ui/src/style/class/page/fn.rs:1390-1404`）：

```rust
pub c_loading_overlay(background: &str) {
    position: "absolute";
    top: "0";
    background: background;
    z-index: "1";
}
```

`{ expr }` 花括号标出"运行时替换位"：既可以放在键位（动态属性名），也可以放在值位。`"--value"` 这样的字符串字面量也可以放进花括号当键。

在 `html!` 里按**普通函数调用**使用（`example/src/page/binding/view/fn.rs:239`）：

```rust
class: c_slider_value(&format!("{}%", red_value * 100 / 255))
```

展开为（`macros/src/class/fn.rs:1203-1227`，已精简）：

```rust
pub fn c_slider_value(v: &str) -> ::euv::Css {
    static C_SLIDER_VALUE_PARAM_CACHE: ::std::sync::OnceLock<
        ::std::sync::Mutex<::std::collections::HashMap<(String, String), ::euv::Css>>,
    > = ::std::sync::OnceLock::new();
    let cache = C_SLIDER_VALUE_PARAM_CACHE
        .get_or_init(|| ::std::sync::Mutex::new(::std::collections::HashMap::new()));
    let key: (String, String) = ("c_slider_value".to_string(), format!("{:?}", (v)));
    let mut cache_guard = /* cache.lock() */;
    if let Some(cached) = cache_guard.get(&key) {
        return cached.clone();
    }
    let css = ::euv::Css::new(
        format!("{}-{}", "c_slider_value", [::euv::Css::param_class_name(&(v).to_string())].join("-")),
        ["--value: ".to_string() + &({ v }).to_string() + "; "].concat(),
        Vec::new(),
        Vec::new(),
    );
    css.inject_style();
    cache_guard.insert(key, css.clone());
    css
}
```

要点：

- 参数化类返回**所有权 `Css`**（不是 `&'static`），因为每次调用可能有不同的类名。
- 缓存键是 `(类名, format!("{:?}", (参数..)))`，因此**参数类型必须实现 `Debug`**，否则编译失败。
- 缓存无上界。参数连续变化（如百分比滑块）会让缓存无限增长 —— 参数化类只适合少量离散变体。

## 类名生成规则

| 定义 | 展开后的类名 | 实测 |
| --- | --- | --- |
| 无参 `c_static` | `c_static` | `c_static` |
| 有参 + 花括号位 `c_param(v)` + `color: { v }` | `c_` + 类名 + `-` + FNV 哈希 | `c_param("red")` → `c_param-89e9be1960f4c21c` |
| 有参 + 裸标识符 `c_param_typed(v)` + `font-size: v` | 同上 | `c_param_typed("12px")` → `c_param_typed-208877f103d004c2` |
| 无参泛型 `c_generic_no_params<T: Debug>` | 类名原样 | `c_generic_no_params` |

哈希是 `Css::param_class_name`（`core/src/vdom/attribute/impl.rs:749-756`）—— FNV-1a 64 位，十六进制输出。`fnv1a("red") == 89e9be1960f4c21c`、`fnv1a("12px") == 208877f103d004c2`，与上表实测一致。

**注意文档与实现的差异**：`macros/src/lib.rs:75-78` 的 doc comment 声称"不带花括号的参数引用保持默认的按类型命名"，但当前实现只要定义带形参列表，就把**所有形参都登记为动态**（`macros/src/class/fn.rs:203-207`），无条件按值哈希。上表 `c_param_typed` 就是反例：参数没包花括号，类名里仍出现值哈希。类名带值哈希才能让 `c_demo("color","red")` 与 `c_demo("background","red")` 生成不同规则，否则浏览器无法区分。

**参数化 + 泛型目前无法编译**：展开模板在返回类型后追加 `#ty_generics`，生成 `-> ::euv::Css<T>`，报 `E0107`（`Css` 不接受泛型参数）。无参泛型类可以编译，展开为 `fn c_x<T: ..>() -> ::euv::Css`（`macros/src/class/impl.rs:406-412`），**返回所有权值且不缓存** —— 每次调用都新建 `Css` 并 `inject_style()`。

## 伪类与伪元素

真实用例（`ui/src/style/class/page/fn.rs:1343-1345`）：

```rust
    :hover {
        opacity: "1";
    }
```

伪元素与复合选择器（`ui/src/style/class/shell/fn.rs:21-31`）：

```rust
        ::-webkit-scrollbar-thumb {
            background: var!(scrollbar-thumb);
            border: "none";
            border-radius: "0px";
        }
        ::-webkit-scrollbar-thumb:hover {
            background: var!(scrollbar-thumb-hover);
        }
```

伪元素（`ui/src/style/class/shell/fn.rs:232-240`）：

```rust
        ::after {
            content: "''";
            position: "absolute";
            bottom: "0px";
            background: format!("linear-gradient(90deg, transparent, {}, transparent)", var!(border));
        }
```

**不支持 `&`。** 裸写 `&:hover { .. }` 报 `error: expected ','`；全仓零实例。伪类一律写裸 `:hover`。函数式伪类 `:nth-child(2n)` / `:not(.)` / `:is(.)` 解析层支持（`macros/src/class/fn.rs:430-440`），但真实代码里没有用例。

### 嵌套选择器会被扁平化

```rust
pub c_nested_dyn(v: &str) {
    color: "red";
    ::placeholder { :hover { color: { v }; } }
}
```

展开结果：

```text
pseudo_rules: [ PseudoRule { selector: "::placeholder:hover", style: "color: red; " } ]
```

嵌套块不是生成后代选择器，而是把选择器字符串**直接拼起来**（`macros/src/class/fn.rs:892-913`）。

### 元素选择器块

```rust
pub c_attr_sel {
    input[type="text"] { font-size: "16px"; }
}
```

展开为 `PseudoRule { selector: "input[type=text]", style: "font-size: 16px; " }`。元素选择器与普通伪类共用 `pseudo_rules` 通道。`h1 { .. }`、`code, pre, kbd, samp { .. }`、`*, *::before, *::after { .. }` 都支持（`macros/src/class/shell/fn.rs:37,147`、`page/fn.rs:480`）。

注意属性选择器里的引号在重建时会被丢掉（`input[type="text"]` → `input[type=text]`），多数浏览器都能接受，但与源码不一致。

## @media

真实用例（`ui/src/style/class/page/fn.rs:1058-1062`）：

```rust
        @media ((max-width: 767px)) {
            margin-bottom: var!(space-lg);
            flex: "0 0 auto";
            justify-content: "flex-start";
        }
```

`@media` 的条件原样透传到 `MediaRule.query`，实测：

| 写法 | `query` 字段 | 生成的 CSS |
| --- | --- | --- |
| `@media (max-width: 767px)` | `(max-width: 767px)` | `@media (max-width: 767px) { … }` |
| `@media ((max-width: 767px))` | `((max-width: 767px))` | `@media ((max-width: 767px)) { … }` |

解析器见到 `(` 会自己补一层外括号（`macros/src/class/fn.rs:798-803`），所以单括号写法得到的才是标准 CSS 媒体查询字符串；本仓库 `ui/src/style/class/*` 全部写双括号，双括号也会原样保留。

`@media` 内可以再写伪类：

```rust
// ui/src/style/class/shell/fn.rs:606-612
@media ((max-width: 767px)) {
    padding: format!("{} {} 0 {}", var!(padding-main-top-mobile), var!(edge-gutter-mobile), var!(edge-gutter-mobile));
    ::-webkit-scrollbar {
        width: "0px";
    }
}
```

覆盖同级的伪元素（`ui/src/style/class/shell/fn.rs:241-246`）：

```rust
@media ((max-width: 767px)) {
    ::after {
        left: var!(edge-gutter-mobile);
        right: var!(edge-gutter-mobile);
    }
}
```

### 已知限制

- `@media` 里写**元素选择器**（`p { .. }`、`input, select { .. }`）会被错误地拼进媒体查询的 style 文本。实测 `@media ((max-width: 767px)) { :hover { color: red; } p { color: green; } }` 得到 `MediaRule { style: "p color: green;", pseudo_rules: [":hover"] }` —— `p color: green;` 不是合法 CSS，浏览器会丢弃。
- `@media` 套 `@media` 同样会拼出非法文本：`style: "@media ((min-width: 100px)) color: red;"`。
- 解析层认识 23 种 at-rule（`macros/src/class/fn.rs:735-761`：`@media` / `@keyframes` / `@supports` / `@layer` / `@container` / `@property` / `@scope` / `@font-face` / `@charset` / `@import` / …），但**只有 `@media` 会转成 `MediaRule`**（`macros/src/class/fn.rs:930-933`），其余静默丢弃。实测 `@supports (display: grid) { display: "grid"; }` 展开后 `media_rules` 为空。
- `@keyframes fade { from { opacity: "0"; } }` 直接报 `error: unexpected end of input, expected curly braces` —— `from` / `to` 会被当成元素选择器块，解析不下去。

## 继承（extends）

在类体里写 `父类(实参);` 即继承父类的全部规则。

真实用例（`ui/src/style/class/page/fn.rs:472-479`）：

```rust
    pub c_event_info_value {
        c_text_ellipsis();
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
        flex: "1";
    }
```

三段链式继承（`ui/src/style/class/shell/fn.rs:1204-1216`）：

```rust
    pub c_euv_toc_link_nested {
        c_euv_toc_link();
        padding-left: var!(space-lg);
    }

    pub c_euv_toc_link_nested_active {
        c_euv_toc_link_nested();
        color: var!(accent);
        font-weight: "700";
        :hover {
            font-weight: "700";
        }
    }
```

继承带参数的父类同样可用（实测）：

```rust
class! {
    pub c_parent(c: &str) { color: { c }; }
    pub c_kid { c_parent("red"); font-size: "12px"; }
}
```

`c_kid()` → `style = "color: red;  font-size: 12px; "`（父类 style 以空格分隔前置拼接）。

展开成（`macros/src/class/impl.rs:294-298, 231-259`，已精简）：

```rust
let css = ::euv::Css::new(
    "c_kid".to_string(),
    [c_static().get_style().to_string() + " ", "margin: 0px; ".to_string()].concat(),
    {
        let mut all_pseudo: Vec<::euv::PseudoRule> = Vec::new();
        all_pseudo.extend(c_static().get_pseudo_rules().iter().cloned());
        all_pseudo.extend(Vec::new());
        all_pseudo
    },
    {
        let mut all_media: Vec<::euv::MediaRule> = Vec::new();
        all_media.extend(c_static().get_media_rules().iter().cloned());
        all_media.extend(Vec::new());
        all_media
    },
);
```

继承会合并父类的 **style、pseudo 规则、media 规则**三项。子类自己的同名属性追加在父类之后，CSS 层叠自然覆盖。

实测继承 + `@media`：父类 `c_static` 有 `:hover` 与 `@media`，子类 `c_ext { c_static(); margin: "0px"; }` 展开后 `pseudo_rules` 与 `media_rules` 都完整继承了父类。

## 运行时行为

`Css` 的四个字段（`core/src/vdom/attribute/struct.rs:64-90`）：

```rust
pub struct Css {
    name: String,
    style: String,
    pseudo_rules: Vec<PseudoRule>,
    media_rules: Vec<MediaRule>,
}
```

字段访问器是 `pub(crate)`，外部只能读 `get_name()` / `get_style()` 等公开 getter（`get_pseudo_rules` / `get_media_rules` 是 crate 内可见）。

- **注入**：`inject_style()`（`core/src/vdom/attribute/impl.rs:580-635`）把 `.类名 { style }`、`.类名:伪类 { … }`、`@media 条件 { … }` 拼成 CSS 文本，**append** 到共享的 `<style id="euv-css-injected">`。类名里非 `[a-zA-Z0-9_-]` 的字符会加反斜杠转义。
- **去重**：全局 `INJECTED_CLASSES: LazyLock<RwLock<HashSet<String>>>`（`core/src/vdom/attribute/static.rs:18`）按类名判重，同一个类不会重复注入。
- **非 wasm 目标**：所有 CSS 操作被 `cfg!(target_arch = "wasm32")` 提前返回，所以类函数在原生 `cargo test` 里也能安全调用（`macros/tests/class/fn.rs:25-38` 就是这么断言的）。
- 无参类的 `OnceLock` 保证**两次调用返回同一指针**（`macros/tests/class/fn.rs:35-38` 断言 `ptr::eq`）。

`html!` 侧把 `&'static Css` 装进 `AttributeValue::CssRef`，所有权 `Css` 装进 `AttributeValue::Css`（`core/src/vdom/attribute/enum.rs:28,37`）；多条 `class:` 在渲染前由 `AttributeValue::merge_class` 用空格拼接。

## vars! 与 var!

`vars!` 定义一组 CSS 自定义属性（设计令牌）。真实用例（`example/src/style/var/fn.rs`）：

```rust
vars! {
    pub(crate) c_example_theme {}
}
```

完整形态（`ui/src/style/var/fn.rs:3-50` 节选）：

```rust
vars! {
    pub(crate) c_theme_light {
        // ─── Surface ───
        background: "#ffffff";
        foreground: "#000000";

        // ─── Accent ───
        accent: "#000000";

        // ─── Spacing Scale ───
        space-2xs: "2px";
        space-md: "12px";
    }
}
```

变量名自动加 `--` 前缀，展开成与 `class!` 相同的函数形态（`OnceLock` + `Css::new` + `inject_style`）：

```text
c_theme_light() → Css { name: "c_theme_light", style: "--background: #ffffff; --accent: #000000; --space-md: 12px; " }
```

- 变量名可以是 kebab-case 标识符，也可以是字符串字面量：`"bg-primary": "#fff"` 与 `bg-primary: "#fff"` 等价。
- `vars!` 支持参数化，参数按 `format!("{:?}", param)` 进类名：`vars! { pub c_dyn(flag: bool) { accent: { if flag { "#fff" } else { "#000" } }; } }` → `c_dyn(true)` 的类名是 `c_dyn-true`。
- 拿到 vars 块后，在 `html!` 里当普通类函数用（`example/src/component/layout/view/fn.rs:23`）：

```rust
html! {
    div {
        class: root_class_signal
        class: c_example_theme()
```

`var!` 引用单个变量名，展开为 CSS `var()` 字符串：

```rust
class! {
    pub(crate) c_game_description {
        line-height: "1.5";
        color: "inherit";
        margin-bottom: var!(gap-component);
    }

    pub(crate) c_game_3d_canvas {
        background: var!(accent);
        object-fit: "contain";
    }
}
```

展开为 `"var(--gap-component)"` / `"var(--accent)"`。`var!("bg-primary")`（带引号）与 `var!(bg-primary)` 结果相同。

`var!` 可以嵌在 `format!` 里，宏会递归扫描 token 流展开（`macros/src/class/fn.rs:116-163`）：

```rust
// ui/src/style/class/shell/fn.rs:1021-1023
margin-left: format!("calc(-1 * {})", var!(side-indent));
width: format!("calc(100% + {})", var!(side-indent));
```

展开结果：`"margin-left: calc(-1 * var(--side-indent)); "` 一类 —— 变量名全部替换成 `var(--..)`，**不是**替换成变量的值；值由浏览器在计算样式时解析。

> ⚠️ **单位不能直接拼在 `var()` 后面。** `--side-indent-num` 这类不带单位的 token 只能当数字用（`calc(-1 * var(--side-indent-num) * 1px)`）；写成 `var(--side-indent-num)px` 时，替换后是一个数字 token 紧跟一个 `px` ident，**不是合法长度，整条声明会被解析器丢弃**，属性静默保持初始值 —— fmt、clippy、测试全都发现不了。要带单位就用带单位的 token（`--side-indent`），或让单位参与 `calc()` 运算。

## 错误信息

| 错误 | 触发条件 | 定义 |
| --- | --- | --- |
| `unknown at-rule: @<keyword>` | at-rule 关键字不在 23 种白名单里 | `macros/src/class/fn.rs:779`（内联 `format!`，无常量） |
| `expected identifier` | 属性名 / 选择器片段缺失 | `macros/src/ident/const.rs:1-2` |
| `expected ','` | 写了 `&` 伪类等不支持的语法 | 解析器自然报错 |
| `expected ':'` | 属性值裸写 `if` / `match`（被当成元素选择器块） | 解析器自然报错 |
| `T doesn't implement Debug` | 参数化类的形参没有 `Debug` | 由生成的 `format!("{:?}", ..)` 触发 |

## 交叉引用

- [HTML 宏](./html.md) —— `class:` / `style:` 属性在 `html!` 里的写法与展开结果。
- [响应式宏](./reactive.md) —— `watch!` / `computed!`，以及 `vars!` / `var!` 的完整参考。
- [组件宏](./component.md) —— `#[component]` 属性与 Props 结构体。
- [原始 HTML](./raw.md) —— `unsafe_no_inline!` 与 `RawHtml`。