---
title: 响应式宏
order: 3
---

`watch!` 与 `computed!` 是两个过程宏，把「订阅若干 `Signal`、在它们变化时执行一段代码」的样板压成一行。两者共享同一套解析器（`macros/src/watch/` 与 `macros/src/computed/` 各自一份，逻辑对称），差别只在产出的是一个副作用块，还是一个**派生的信号**。

配套的 `vars!` / `var!` 见[类名宏](./class.md)——它们管的是 CSS 自定义属性，不参与响应式，只是同属 `euv-macros`。

## 语法

```
watch! {
    信号表达式, 信号表达式, ...
    |参数名: 参数类型, ...| {
        语句; 语句; ...
    }
}

computed! {
    信号表达式, 信号表达式, ...
    |参数名: 参数类型, ...| -> 返回类型 {
        表达式
    }
}
```

要点：

- **信号个数必须等于闭包参数个数**，一一对应，编译期校验。
- **参数名和参数类型都可以省略**。省略名字只订阅不取值，省略类型则交给推断；两者都省时展开成 `let _ = signal.get();`，即「订阅但不消费」。
- **`computed!` 的返回类型必须显式写出**（`-> T`），因为它要用来定 `Signal<T>` 的参数类型；`watch!` 没有返回值，所以不需要。

`html!` 里 `{表达式}` 的自动重渲染也是这套订阅机制实现的，见[HTML 宏](./html.md)。

## watch!

`watch!` 展开成一个块，只做副作用，没有返回值：

```rust
{
    let __euv_watch_signal_0: ::euv::Signal<_> = 信号0;
    let __euv_watch_signal_1: ::euv::Signal<_> = 信号1;
    let __euv_watch_subscribed: ::euv::Signal<bool> = ::euv::App::use_signal(|| false);
    if !__euv_watch_subscribed.get() {
        let __euv_watch_fire_addr: usize = ::euv::FireHandle::new(move || {
            let a = __euv_watch_signal_0.get();
            let b = __euv_watch_signal_1.get();
            { /* 你的语句 */ }
        }).into();
        ::euv::App::batch(|| {
            __euv_watch_signal_0.subscribe(move || {
                ::euv::App::batch(|| {
                    unsafe { ::euv::FireHandle::fire_at(__euv_watch_fire_addr) }
                });
            });
            __euv_watch_signal_1.subscribe(/* 同上 */);
            /* 首次立即执行一次 */
            let a = __euv_watch_signal_0.get();
            let b = __euv_watch_signal_1.get();
            { /* 你的语句 */ }
            __euv_watch_subscribed.set(true);
        });
    }
}
```

（展开结果经过简化，标识符前缀为 `__euv_watch_`。实现见 `macros/src/watch/impl.rs`。）

三个行为值得注意：

- **首次会立即执行一次**，不只等信号变化。依赖已在首次渲染前就绪的 `watch!` 不需要额外手动调一次。
- **`__euv_watch_subscribed` 守卫保证订阅只注册一次**。`watch!` 通常写在组件体里，组件重渲染会重新执行这整段代码；没有这个守卫就会每次重渲染都叠加一个新订阅，闭包越积越多。
- **变化时经 `FireHandle::fire_at` 回调**，不是直接在 `subscribe` 的闭包里执行用户代码。`FireHandle` 存的是函数地址而非捕获了环境的闭包，因此闭包可以 `'static` 而不必借用任何东西。

## computed!

`computed!` 产出的是一个新 `Signal`，值为闭包的返回值：

```rust
{
    let __euv_computed_signal_0: ::euv::Signal<_> = 信号0;
    let __euv_computed_result: ::euv::Signal<f64> = ::euv::App::use_signal::<f64, _>(|| -> f64 {
        let x = __euv_computed_signal_0.get();
        x * 2.0
    });
    let __euv_computed_subscribed: ::euv::Signal<bool> = ::euv::App::use_signal(|| false);
    if !__euv_computed_subscribed.get() {
        let __euv_computed_fire_addr: usize = ::euv::FireHandle::new(move || {
            let x = __euv_computed_signal_0.get();
            __euv_computed_result.set({ x * 2.0 });
        }).into();
        ::euv::App::batch(|| {
            __euv_computed_signal_0.subscribe(/* fire_at __euv_computed_fire_addr */);
            __euv_computed_subscribed.set(true);
        });
    }
    __euv_computed_result
}
```

（实现见 `macros/src/computed/impl.rs`。）

与 `watch!` 的差别：

- **初值由 `use_signal` 的初始化闭包算出**，且 `use_signal` 带显式类型参数 `::<返回类型, _>`。
- **变化时用 `set()` 写回结果信号**，而不是直接执行副作用。写 `set()` 而不是就地改，是因为只有 `set()` 才会把派生信号的依赖者标脏，从而触发下一轮渲染——这一点是 `computed!` 能参与渲染的前提。
- **初值只算一次**。`use_signal` 的初始化闭包只在首次调用，之后的更新全部来自 `set()`；闭包里若读了别的信号，那些依赖不会自动被 `computed!` 追踪，必须显式写进 `computed!` 的信号列表。

最后一条是实践中最容易踩的坑：

```rust
// 不对：只订阅了 count，doubled 变化时不会重算
let doubled = computed!(count.get() |x: i32| -> i32 { x * 2 });

// 对：把依赖都列进去
let source: Signal<i32> = /* ... */;
let doubled = computed!(source |x: i32| -> i32 { x * 2 });
```

## watch! 还是 computed!

| | `watch!` | `computed!` |
| --- | --- | --- |
| 产出 | 无（副作用块） | 一个 `Signal<T>` |
| 闭包返回 | 任意语句 | 单个表达式 |
| 首次执行 | 是 | 由 `use_signal` 算出初值 |
| 变化时 | 执行语句 | `set()` 写回 |
| 用途 | 命令式副作用：写 DOM、调外部 API、埋点 | 派生状态：显示值、样式、启用与否 |

要「某个值变了就做点什么」用 `watch!`；要「这个值由别的值算出来」用 `computed!`。

## Signal 的相关方法

两个宏都只用到 `Signal` 上的一小组方法，展开后按绝对路径 `::euv::Signal` 调用，所以不需要额外 `use`：

| 方法 | 位置 | 作用 |
| --- | --- | --- |
| `Signal::get()` | `core/src/reactive/signal/impl.rs:92` | 取当前值。宏展开成闭包参数就是调它 |
| `Signal::subscribe(callback)` | `core/src/reactive/signal/impl.rs:198` | 注册变化回调，返回一个 `usize` 句柄 |
| `Signal::set(value)` | `core/src/reactive/signal/impl.rs:438` | 写入新值并标脏依赖者 |
| `App::use_signal(init)` | `core/src/app/impl.rs:23` | 在组件作用域创建一个信号 |
| `App::batch(callback)` | `core/src/app/impl.rs:51` | 把多次写入合并成一次渲染 |

`Signal<T>` 本身是 `Copy` 的——两个宏都按值反复传递同一个信号，所以捕获它进闭包不需要 `clone()`。

## 延伸阅读

- [HTML 宏](./html.md) —— `html!` 里 `{表达式}` 的响应式重渲染来源。
- [类名宏](./class.md) —— `vars!` / `var!` 的 CSS 自定义属性定义。
