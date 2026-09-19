---
title: Markdown 语法
---

# Markdown 语法

euv-docs 在构建期渲染 VuePress 风格的 markdown 子集。本页是该解析器
支持的**全部**块级与行内语法的权威参考,附实际渲染样例。

## 标题

`h2` 与 `h3` 会自动出现在右侧锚点目录。`h1`、`h4`、`h5`、`h6` 渲染为普通标题,不进目录。

### h3 子标题

#### h4 子子标题

##### h5 子子子标题

###### h6 最深一级标题

### Setext 风格标题 {#setext-h1}

下划线风格的文本也会被识别为标题。`=` 表示 h1,`-` 表示 h2。

Setext h1
=========

Setext h2
---------

### 自定义标题 id {#custom-heading-id}

在标题末尾追加 `{#slug}` 可以覆盖自动生成的 slug,slug 同时成为
元素 id 与锚点永久链接(本页这里是
`#/zh/guide/markdown.html#custom-heading-id`)。

## 行内格式

支持 **加粗**、*斜体*、~~删除线~~ 与 `行内代码`。

加粗和斜体可以嵌套:***加粗斜体***、**带 `行内代码` 的加粗**,以及
*混合 `代码` 与强调*。

下划线同样可用作强调符:_斜体_、__加粗__,以及
*与星号混用如 **这样***。

行内格式也可与删除线嵌套:
**~~加粗删除线~~**、*~~斜体删除线~~*,以及
***~~加粗斜体删除线~~***。

行内代码中若要包含反引号,可以用更长的围栏包裹:
``code with `backtick` inside`` 整体作为一个代码段渲染。

脚注引用以 `[^example]` 形式内联,渲染为纯文本 `[^example]`,
并指向同页下方的脚注定义。

## 硬换行 vs 软换行

这段文字末尾有两个尾随空格,所以下一行  
虽然在同一个段落里,但会另起一行(硬换行,`<br>`)。

这段文字末尾没有尾随空格,所以下一行
会与这一段合并为同一段(软换行,只插入一个空格)。

## 列表

- 无序项
- 另一项
  - 嵌套无序项
  - 另一嵌套项
    1. 无序下的嵌套有序
    2. 第二条嵌套有序
  - 回到无序

1. 有序项
2. 第二项
3. 第三项带一个[链接](./getting-started.md)

- [x] 已完成任务
- [ ] 待办任务
- [x] 已完成任务带 `行内代码`

把任务列表嵌进有序列表:

1. 计划
   - [x] 列出大纲
   - [ ] 撰写初稿
2. 评审
   - [x] 自评
   - [ ] 他人评审

## 表格

| 特性 | VuePress | euv-docs | 备注 |
| :--- | :---: | ---: | --- |
| Frontmatter | ✅ | ✅ | 构建期解析 |
| 自定义容器 | ✅ | ✅ | `::: tip / warning / danger / note` |
| 锚点目录 | ✅ | ✅ | 仅 h2 + h3 |
| 表格对齐 | ✅ | ✅ | 左 / 中 / 右 |

单元格内要插入字面量竖线时,用反斜杠转义 `\|`,会渲染为 `|`。

## 代码块

带语言标签的围栏代码块会在外壳上挂语法类:

```rust
fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}
```

```bash
euv build --release -- --target web --out-dir www/pkg
```

```toml
[site]
title = "euv-docs"

[[locales]]
prefix = "/"
lang = "en-US"
label = "English"
```

```text
纯文本围栏,无语法类
```

```markdown
markdown 围栏,无语法类
```

缩进代码块(4 空格缩进)也支持,但建议用围栏形式,因为语言标签会驱动外壳类。

    // 4 空格缩进块渲染为代码
    fn main() {}

未识别的语言标签仍按围栏渲染:

```haskell
main :: IO ()
main = putStrLn "unknown lang, but still fenced"
```

## 自定义容器

支持的完整类型集合为 `tip`、`warning`、`danger`、`info`、`note`、`details`
(在 euv-ui 容器 CSS 中定义)。

::: tip
提示容器使用实线边框。
:::

::: warning
警告容器使用虚线边框。
:::

::: danger
危险容器使用双线边框。
:::

::: info
信息容器使用点状边框。
:::

::: note 自定义标题
容器支持在类型后写自定义标题。
:::

一个容器可以包含多个块:

::: tip 多个块
本容器内含 **两段** 与一个列表。

- 第一个内嵌项
- 第二个内嵌项
:::

容器不支持嵌套——容器里再写 `:::` 会被当字面文本。

::: details 点击展开
可展开块中渲染的隐藏内容。该块可以容纳**任何块级内容**:
代码、列表,甚至嵌套引用块。

```rust
fn main() {
    println!("inside details");
}
```
:::

## 引用块

> 文档是写给未来自己的情书。

嵌套引用块:

> 第一层。
>
> > 第二层。
> >
> > 里面也能用 **加粗** 与 `代码`。
>
> 回到第一层。

引用块也可以包含其他块:

> - 引用块里的列表
> - 第二项
>
> ```
> 引用块里的代码
> ```

## 分割线

三个或更多 `-`、`_`、`*` 各成一行:

---

***

___

## 链接

- [指南首页的内部链接](./README.md)
- [euv 仓库的外链](https://github.com/euv-dev/euv)
- [跳转到表格](#表格)
- [跳转到 Setext 标题](#setext-h1)
- [跳转到自定义标题 id](#custom-heading-id)
- [邮件链接](mailto:nobody@example.com)
- [带标题的链接](https://github.com/euv-dev/euv "GitHub 上的 euv 仓库")

裸 URL 与邮箱地址会被自动识别为链接:
<https://github.com/euv-dev/euv> 与 <nobody@example.com> 不用任何
markdown 标记就直接渲染为可点击的链接。

引用式链接也支持。一次定义 id,多处复用:

[euv 仓库][euv-repo] 与 [它的 issue 列表][euv-issues] 都在 GitHub 上。
同一个 id 可以重复使用:[euv 仓库][euv-repo] 再次出现。

[euv-repo]: https://github.com/euv-dev/euv "GitHub 上的 euv 仓库"
[euv-issues]: https://github.com/euv-dev/euv/issues "issue 列表"

## 脚注

脚注引用指向同页的脚注定义。定义语法是 `[^name]: 正文`,引用
语法是行内的 `[^name]`。[^syntax] 渲染器不会自动编号或生成脚注列表
——定义会被渲染为以 id 为前缀的引用块,引用则显示字面文本 `[^name]`。

[^syntax]: 这是脚注正文。里面可以用 **强调**、`行内代码`,也支持多段。

    同一脚注的第二段也是支持的。

    > 引用块也能写在脚注里。

## 图片

图片保留 alt 文本与源 URL 渲染:

![euv logo 占位](https://dummyimage.com/120x40/eee/aaa.png)

需要图片完全离线可用时,用 base64 编码的 `data:` URL:

![inline base64](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNDAiIGhlaWdodD0iNTAiPjxyZWN0IHdpZHRoPSIyNDAiIGhlaWdodD0iNTAiIGZpbGw9IiM0NDQiLz48dGV4dCB4PSIxMjAiIHk9IjMyIiB0ZXh0LWFuY2hvcj0ibWlkZGxlIiBmaWxsPSJ3aGl0ZSIgZm9udC1mYW1pbHk9InNhbnMtc2VyaWYiIGZvbnQtc2l6ZT0iMTYiPmlubGluZSBzdmc8L3RleHQ+PC9zdmc+)

## 转义字符

反斜杠转义下一个字符: \*字面星号\*、\[不是链接\]、\<不是标签\>、
\`不是代码\`。反斜杠被消耗,字符以纯文本渲染。

## 内联与块级 HTML

段落内允许嵌入一段内联 HTML:

这段文字里有 <sup>上标</sup> 与 <sub>下标</sub>
通过原生 HTML 标签嵌入。

原生 HTML 属性也会保留:<kbd>Ctrl</kbd>+<kbd>S</kbd>
渲染为键盘快捷键,<mark>高亮文本</mark> 用原生 `<mark>` 元素。

一段原生 HTML 块按字面渲染:

<div style="border: 1px dashed currentColor; padding: 8px 12px;">
  此块作为原生 HTML 渲染,不走 markdown 解析。
</div>

<details>
<summary>原生 HTML <code>&lt;details&gt;</code></summary>

`<details>` 块也会原样穿透。这与 `::: details` 自定义容器不同——
后者是同款外观的 markdown 写法,这里是直接用原生 HTML。

</details>
