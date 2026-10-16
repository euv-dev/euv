---
title: 渲染后端与配置
order: 4
---

`euv_engine` 的渲染模块负责三套后端（Canvas 2D / WebGL 2 / WebGPU）以及它们的配置对象。

## 导入路径

所有类型都通过 `pub use` 从 crate 根**扁平导出**。`renderer` 与 `config` 在 `lib.rs` 里是私有 `mod`，只能整体引用：

```rust
// ✅ 正确
use euv_engine::{CanvasRenderer, EngineConfig, RenderConfig, RenderBackendType};

// ❌ 编译错误：E0603 module `renderer` is private
use euv_engine::renderer::canvas::CanvasRenderer;
use euv_engine::config::RenderConfig;
```

## 派生代码约定

配置与 descriptor 结构体普遍标注 `#[derive(Clone, Data, Debug, New, PartialEq)]`（`Data` / `New` 来自 `lombok-macros`），它按字段生成访问器：

- `Type::new(...)` —— 按**声明顺序**接收字段；标了 `#[new(skip)]` 的字段不参与。
- `Type::get_field() -> T` —— 返回**值拷贝**（标了 `type(copy)`），不是引用。
- `Type::get_mut_field()` / `Type::set_field(v)` —— 标为 `pub(crate)`，**crate 外不可见**。

::: tip
字段一律是 `pub(crate)`。**外部代码无法用结构体字面量构造绝大多数类型**（`error[E0451]: fields ... are private`），只能用 `new(...)` 或专门的具名构造器。若某个类型所有字段都标了 `#[new(skip)]`，`new()` 就是零参数、返回默认值。
:::

## 概览

| 分类 | 内容 |
| --- | --- |
| 配置 | [`RenderConfig`](#renderconfig)、[`EngineConfig`](#engineconfig)、[`RenderBackendType`](#renderbackendtype)、[`GpuPowerPreference`](#gpupowerpreference) |
| Canvas 2D | [`CanvasRenderer`](#canvasrenderer)、[`SsaaCanvas`](#ssacanvas)、[`DrawList`](#drawlist)、[`Camera2D`](#camera2d)、[`Camera3D`](#camera3d) |
| 渐变与效果 | [`LinearGradient`](#lineargradient)、[`RadialGradient`](#radialgradient)、[`ShadowConfig`](#shadowconfig)、[`RenderLayer`](#renderlayer) |
| 通用枚举 | [`RenderQuality`](#renderquality)、[`BlendMode`](#blendmode) |
| WebGPU | [`WebGpuRenderer`](#webgpurenderer)、[`WebGpuInitError`](#webgpuiniterror) |
| WebGL | [`WebGl2Backend`](#webgl2backend)、[`WebGl2InitError`](#webgl2initerror) |
| 未展开 | 见 [未逐个展开的 pub 项](#未逐个展开的-pub-项) |

## 三个后端的入口

后端实例化分两层：`RenderConfig` 描述「用哪个后端、画到哪个 canvas」，具体实例化由 `Engine` 的关联函数完成。

| 后端 | 构造函数 | 是否 async |
| --- | --- | --- |
| Canvas 2D | `RenderConfig::canvas2d(...)` + `CanvasRenderer::from_selector(...)` | 否 |
| WebGPU | `RenderConfig::webgpu(...)` + `Engine::webgpu_renderer(&config).await` | **是** |
| WebGL 2 | `RenderConfig::webgl(...)` + `Engine::webgl_renderer(&config)` | 否 |

::: warning
`Engine::webgpu_renderer` 是 `async fn`，`Engine::webgl_renderer` 是普通 `fn` —— 两者签名不对称，后者**不需要** `.await`。
:::

## RenderBackendType

选择渲染后端。`Default` 为 `Canvas2D`。

| 变体 | 对应上下文 | 适用场景 |
| --- | --- | --- |
| `Canvas2D` | `CanvasRenderingContext2d` | 全浏览器支持，即时模式绘制，适合 2D 游戏与软件渲染 3D |
| `WebGpu` | `GpuDevice` + `GpuCanvasContext` | GPU 加速，需要 WebGPU 浏览器 |
| `WebGl` | `WebGl2RenderingContext` | GLSL ES 3.00，WebGPU 不可用时的 GPU 回退方案 |

## GpuPowerPreference

传给 `navigator.gpu.requestAdapter` 的偏好提示。`Default` 为 `LowPower`。

| 变体 | WebGPU 字符串 |
| --- | --- |
| `LowPower` | `"low-power"` |
| `HighPerformance` | `"high-performance"` |

```rust
use euv_engine::GpuPowerPreference;

assert_eq!(GpuPowerPreference::LowPower.to_web_sys_string(), "low-power");
assert_eq!(
    GpuPowerPreference::HighPerformance.to_web_sys_string(),
    "high-performance"
);
```

## RenderConfig

渲染配置。三个后端各有一个具名构造器，签名一致：

| 方法 | 设置 `backend` 为 |
| --- | --- |
| `RenderConfig::canvas2d(selector, width, height)` | `Canvas2D` |
| `RenderConfig::webgpu(selector, width, height)` | `WebGpu` |
| `RenderConfig::webgl(selector, width, height)` | `WebGl` |

参数 `S: AsRef<str>`，`&str` 与 `String` 都接受；`width` / `height` 是 **CSS 像素**单位的逻辑视口。

### 只读访问器

| 访问器 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| `get_backend()` | `RenderBackendType` | `Canvas2D` | 后端类型 |
| `get_canvas_selector()` | `String` | `"#engine-canvas"` | canvas 的 CSS 选择器 |
| `get_width()` / `get_height()` | `f64` | `800.0` / `600.0` | 逻辑视口尺寸 |
| `get_quality()` | `RenderQuality` | `High` | 平滑与降采样质量 |
| `get_antialias()` | `bool` | `true` | WebGPU 的 MSAA 开关 |
| `get_power_preference()` | `GpuPowerPreference` | `LowPower` | WebGPU 适配器偏好 |
| `get_ssaa_scale_factor()` | `f64` | `2.0` | Canvas 2D 的 SSAA 倍率 |

`get_canvas_selector()` 返回 `String` 的**克隆**（标了 `type(clone)`），因此与 `&str` 比较时直接 `assert_eq!` 即可。

```rust
use euv_engine::{GpuPowerPreference, RenderBackendType, RenderConfig, RenderQuality};

let config: RenderConfig = RenderConfig::webgpu("#app", 800.0, 600.0);

assert_eq!(config.get_backend(), RenderBackendType::WebGpu);
assert_eq!(config.get_canvas_selector(), "#app");
assert_eq!(config.get_width(), 800.0);
assert_eq!(config.get_height(), 600.0);
assert_eq!(config.get_quality(), RenderQuality::High);
assert!(config.get_antialias());
assert_eq!(config.get_power_preference(), GpuPowerPreference::LowPower);
assert_eq!(config.get_ssaa_scale_factor(), 2.0);
```

::: warning
`quality` / `antialias` / `power_preference` / `ssaa_scale_factor` / `backend` / `canvas_selector` 的 **setter 与可变访问器都是 `pub(crate)`**，crate 外调用 `set_quality(...)` 会得到 `error[E0624]: method 'set_quality' is private`。

也就是说：**下游只能通过三个具名构造器创建 `RenderConfig`，之后只读。** 要换后端就换一个构造器，而不是改字段。
:::

## EngineConfig

顶层引擎配置，包裹 `RenderConfig` 与 `SchedulerConfig`。

| 方法 | 说明 |
| --- | --- |
| `EngineConfig::create(render)` | 用给定 `RenderConfig` 创建，`scheduler` 取 `SchedulerConfig::default()` |
| `EngineConfig::with_scheduler(self, scheduler)` | 消费 `self`、替换 scheduler 并返回新的 `EngineConfig` |
| `get_render() -> RenderConfig` | 取渲染配置（克隆） |
| `get_scheduler() -> SchedulerConfig` | 取调度器配置（值拷贝） |

```rust
use euv_engine::{EngineConfig, RenderConfig, SchedulerConfig};

let render: RenderConfig = RenderConfig::canvas2d("#root", 320.0, 240.0);
let base: EngineConfig = EngineConfig::create(render);
assert_eq!(base.get_render().get_canvas_selector(), "#root");

// 60Hz 之外的自定义步长
let tuned: EngineConfig = base.with_scheduler(SchedulerConfig::new(0.125, 0.25));
assert_eq!(tuned.get_scheduler().get_fixed_timestep(), 0.125);
```

`Engine::default_config()` 是 `EngineConfig::default()` 的便捷包装，便于不具名类型直接取默认配置。

## Canvas 2D

### CanvasRenderer

对 `CanvasRenderingContext2d` 的即时模式封装。

| 方法 | 说明 |
| --- | --- |
| `CanvasRenderer::from_selector(selector, width, height) -> Option<Self>` | 按 CSS 选择器查找 canvas 并初始化；找不到返回 `None` |
| `clear()` / `clear_color(c)` | 清空画布 / 用指定颜色铺满 |
| `save()` / `restore()` | 上下文状态栈 |
| `apply_camera()` | 施加 `Camera2D` 变换 |
| `replay(&DrawList)` | 把录制好的绘制列表回放到上下文 |
| `set_fill_color(c)` / `set_stroke_color(c)` / `set_line_width(w)` / `set_font(f)` | 画笔状态 |
| `set_global_alpha(a)` | 全局透明度，内部钳制到 `[0, 1]` |
| `set_blend_mode(BlendMode)` | 合成模式 |
| `set_shadow(&ShadowConfig)` / `clear_shadow()` | 阴影 |
| `set_linear_gradient_fill(&g)` / `set_radial_gradient_fill(&g)` | 渐变填充 |
| `set_linear_gradient_stroke(&g)` / `set_radial_gradient_stroke(&g)` | 渐变描边 |
| `fill_rect` / `stroke_rect` / `fill_circle` / `stroke_circle` / `draw_line` / `fill_text` / `draw_image` / `draw_image_rect` | 直接绘制，颜色取当前画笔状态 |

颜色类参数用泛型 `C: AsRef<str>`，所以 `&str`、`String` 都能直接传。

关联函数（不需要 `self`）：

| 方法 | 说明 |
| --- | --- |
| `CanvasRenderer::font(size, family) -> String` | 拼出 CSS font 简写，如 `16px sans-serif` |
| `CanvasRenderer::default_font() -> String` | `16px sans-serif` |
| `CanvasRenderer::enable_smoothing_on(&ctx)` | 打开平滑（等价 `RenderQuality::High`） |
| `CanvasRenderer::detect_dpr() -> f64` | 读 `window.devicePixelRatio`，无 window 时回退 `1.0` |
| `CanvasRenderer::to_css(&Color) -> String` | 颜色转 CSS `rgba(...)` 字符串 |

`apply_camera()` 施加的变换顺序固定为 **居中 → 缩放 → 旋转 → 撤销居中**，这正是「相机在世界原点上」的实现方式。

### SsaaCanvas

SSAA 超采样画布：一张离屏画布按 `scale_factor` 放大绘制，`present()` 时缩小呈现。

| 方法 | 说明 |
| --- | --- |
| `SsaaCanvas::from_selector(selector, width, height) -> Option<Self>` | 用 `RenderConfig` 的默认倍率 `2.0` |
| `SsaaCanvas::from_selector_with_scale(selector, width, height, scale) -> Option<Self>` | 显式指定倍率；`2.0` 表示 4 倍超采样 |
| `present()` | 把离屏缓冲缩小绘制到可见画布 |
| `clear()` / `clear_color(c)` | 清离屏缓冲 |
| `get_offscreen_context()` | 取离屏 2D 上下文，直接绘制用 |

::: tip
真实示例 `example/src/page/game_2d/` 的 2D 路径走的就是 `SsaaCanvas::from_selector_with_scale` + 拿原生 `CanvasRenderingContext2d` 自行绘制，而不是 `CanvasRenderer` / `DrawList`。按机型决定倍率是常见做法：

```rust
use euv_engine::SsaaCanvas;

let scale_factor: f64 = if is_mobile { 1.0 } else { 2.0 };
let ssaa_canvas: SsaaCanvas =
    SsaaCanvas::from_selector_with_scale("#stage", 800.0, 600.0, scale_factor)
        .expect("画布不在 DOM 中");

// 画在离屏缓冲上，再 present() 缩小呈现
ssaa_canvas.get_offscreen_context().clear_rect(0.0, 0.0, 800.0, 600.0);
ssaa_canvas.present();
```
:::

### DrawList

把绘制录制为命令列表，而不是立刻画 —— 引擎每帧统一 `replay` 一次，从而合并同风格的连续图形、跳过冗余的画布状态切换。

| 方法 | 签名要点 |
| --- | --- |
| `create()` | 新建空列表 |
| `is_empty()` / `len()` / `commands()` | 查询；`commands()` 返回 `&[DrawCommand]` |
| `clear()` | 清空但保留可复用性 |
| `fill_rect(position, width, height, color)` | |
| `stroke_rect(position, width, height, color, line_width)` | |
| `fill_circle(center, radius, color)` | |
| `stroke_circle(center, radius, color, line_width)` | |
| `draw_line(start, end, color, line_width)` | |
| `fill_text(text, position, color, font)` | `T: AsRef<str>`, `F: AsRef<str>` |
| `draw_sprite(image, source: Rect, transform: Transform2D)` | |
| `draw_image_rect(image, source, dest_position, dest_width, dest_height)` | |
| `set_global_alpha(alpha)` / `set_blend_mode(mode)` | 状态变更也进录制流 |

```rust
use euv_engine::{BlendMode, Color, DrawList, Vector2D};

let mut list: DrawList = DrawList::create();
list.fill_rect(Vector2D::new(10.0, 20.0), 32.0, 48.0, Color::new(1.0, 0.0, 0.0, 1.0));
list.fill_circle(Vector2D::zero(), 8.0, Color::new(0.0, 1.0, 0.0, 1.0));
list.set_blend_mode(BlendMode::Multiply);
list.set_global_alpha(0.5);

assert_eq!(list.len(), 4);
assert_eq!(list.commands().len(), 4);
```

::: warning
`set_blend_mode` / `set_global_alpha` 是**状态命令**，会各自占一条记录计入 `len()`。上面的列表是 2 条绘制 + 2 条状态 = 4 条。
:::

### Camera2D

| 方法 | 说明 |
| --- | --- |
| `Camera2D::create(width, height)` | 按视口尺寸创建 |
| `world_to_screen(world)` / `screen_to_world(screen)` | 双向变换，可往返 |
| `translate(offset)` | **累加**平移 |
| `zoom_by(factor)` | 按倍率缩放；缩到 0 会被钳制，保证投影有限 |
| `set_zoom(z)` / `set_position(p)` | 直接设定（这两个是公开 setter） |

世界原点落在视口正中。

```rust
use euv_engine::{Camera2D, Vector2D};

let camera: Camera2D = Camera2D::create(800.0, 600.0);
let screen: Vector2D = camera.world_to_screen(Vector2D::zero());
assert_eq!((screen.get_x() - 400.0).abs() < 1e-9, true);
```

### Camera3D

| 方法 | 说明 |
| --- | --- |
| `Camera3D::create(position, target, width, height)` | |
| `forward()` / `right()` | 基向量 |
| `view_matrix()` / `view_proj_matrix()` | 视图矩阵与视图投影矩阵 |
| `world_to_screen(world)` | 世界坐标投影到屏幕 |
| `in_frustum(world)` | 视锥剔除判定 |
| `translate(offset)` / `zoom(distance)` / `orbit(yaw_delta, pitch_delta)` | 平移 / 推拉 / 环绕 |
| `aspect()` | 宽高比；视口高度为 0 时返回 `1.0` |

## 渐变与效果

### LinearGradient

| 方法 | 说明 |
| --- | --- |
| `LinearGradient::create(start, end, stops)` | `create` 是 `new` 的具名包装 |
| `LinearGradient::new(start, end, stops)` | 按字段顺序构造 |
| `to_gradient(&context) -> Option<CanvasGradient>` | 交给原生 2D 上下文构建 |

`stops` 是 `Vec<(f64, String)>`，每项为 `(位置 0.0..=1.0, CSS 颜色字符串)`。**空 stops 列表也是合法渐变**。

### RadialGradient

`RadialGradient::create(inner_center, inner_radius, outer_center, outer_radius, stops)` / `::new(...)` / `to_gradient(&context)`。内圈与外圈都会透传。

### ShadowConfig

`ShadowConfig::create()` 返回默认值（`rgba(0, 0, 0, 0.5)`、模糊 `4.0`、偏移 `0.0`），也可 `new(color, blur, offset_x, offset_y)`。配合 `CanvasRenderer::set_shadow` / `clear_shadow` 使用。

### RenderLayer

渲染层的 z 序与可见性。

| 方法 | z-index | 可见 |
| --- | --- | --- |
| `background()` | `0` | 是 |
| `foreground()` | `100` | 是 |
| `ui()` | `1000` | 是 |
| `create(z_index, visible)` | 自定义 | 自定义 |

`RenderLayer::default()` 是**隐藏**且 z-index 为 0 的起点 —— 层的可见性是主动开启的。

## 通用枚举

### RenderQuality

平滑策略预设。`Default` 为 `High`。

| 变体 | 效果 |
| --- | --- |
| `Low` | 关闭 `imageSmoothingEnabled`，`imageSmoothingQuality = "low"`；配合 CSS `image-rendering: pixelated` 得到锐利像素画缩放 |
| `Medium` | `imageSmoothingQuality = "medium"` |
| `High` | `imageSmoothingQuality = "high"`；HiDPI 上的矢量内容效果最好 |

::: tip
引擎在「保画质」与「保帧率」之间默认偏向前者 —— 用户通常先注意到锯齿，再注意到多花的几毫秒 GPU 时间，所以默认是 `High`。
:::

### BlendMode

对应 CSS `globalCompositeOperation`，共 17 个变体：`Normal`（默认）、`Multiply`、`Screen`、`Lighter`、`Overlay`、`Darken`、`Lighten`、`ColorDodge`、`ColorBurn`、`HardLight`、`SoftLight`、`Difference`、`Exclusion`、`Hue`、`Saturation`、`Color`、`Luminosity`。

`BlendMode::to_css() -> &str` 返回对应的 CSS 关键字。

## WebGPU

### WebGpuRenderer

对 `GpuDevice` + `GpuQueue` + `GpuCanvasContext` 的封装。GPU 句柄一律存为 `JsValue`，方法调用走 `Reflect` / `JsCast` —— 这样可以绕开 `web_sys` 的 feature gate。

| 分类 | 主要方法 |
| --- | --- |
| 可用性 | `is_available() -> bool`、`probe() -> bool`（async，实际请求适配器） |
| 初始化 | `init(&RenderConfig)`（async） |
| 管线 | `create_render_pipeline(shader)`、`create_render_pipeline_full(&RenderPipelineDescriptor)`、`create_compute_pipeline(shader, entry_point)` |
| 渲染通道 | `begin_render_pass_full(...)`、`begin_render_pass_to_texture(...)` |
| 绘制 | `draw(&pass, &DrawArgs)`、`draw_indexed(&pass, &DrawIndexedArgs)`、`draw_indexed_offset(...)`、`dispatch(&pass, &DispatchArgs)` |
| 缓冲 | `create_vertex_buffer(&[u8])`、`create_index_buffer(&[u8])`、`create_uniform_buffer(&[f32])`、`update_uniform_buffer(...)`、`write_buffer(...)` |
| 纹理 | `create_texture(&TextureDescriptor)`、`create_texture_2d(&Texture2DDescriptor)`、`create_texture_view(&JsValue)` |
| 提交 | `submit(&[JsValue])` |
| 尺寸 | `resize(w, h)`、`sync_to_current_canvas()` |
| 诊断 | `push_error_scope(GpuErrorFilter)`、`take_last_error()`、`on_device_lost(callback)` |

`WebGpuRendererInit` 的字段全是 `pub`，可在测试里用 `Reflect` 替身对象拼装渲染器。

### WebGpuInitError

每个变体对应初始化管线中的一个具体失败点，原始 JS 错误以 `JsValue` 携带，便于调用方原样打印而不丢诊断信息。

引擎不在内部打印诊断，而是返回 `Result` 让调用方决定 —— 通常是 `Console::error`，或回退到 Canvas 2D。

| 变体 | 含义 |
| --- | --- |
| `NavigatorGpuMissing` | 浏览器未暴露 `navigator.gpu`（最常见：非 HTTPS / 非 localhost，或浏览器不支持） |
| `AdapterUnavailable` | `requestAdapter()` 解析为 `null`，没有兼容适配器 |
| `DeviceUnavailable` | 分配不到设备，通常适配器已处于 device-lost 状态 |
| `CanvasNotFound(String)` | 选择器没匹配到 canvas（携带选择器字符串） |
| `CanvasContextUnavailable` | `canvas.get_context("webgpu")` 返回 `None` |
| `*Lookup(JsValue)` / `*Call(JsValue)` / `*Promise(JsValue)` | 反射查找、同步调用、Promise 各自的失败点 |

`code() -> &'static str` 返回稳定的错误码字符串（如 `"WEBGL_CANVAS_NOT_FOUND"` 风格），适合上报给 UI 或日志。

## WebGL 2

### WebGl2Backend

| 方法 | 说明 |
| --- | --- |
| `init(&RenderConfig) -> Result<Self, WebGl2InitError>` | **同步**，WebGL 上下文获取只是普通 DOM 调用 |
| `is_available() -> bool` | 环境探测 |
| `resize(width, height)` | 调整视口 |
| `is_context_lost()` | 上下文丢失检查 |
| `apply_state(&GlRenderState)` | 施加整套固定管线状态，**只在真正变化时**才发 GL 调用 |
| `invalidate_state(width, height)` | 状态缓存失效后重置 |
| `render_frame(&context, &GlProgram, color, vertex_count)` | 帧入口 |
| `draw_elements(...)` / `draw_elements_instanced(...)` | 索引绘制 |
| `draw_arrays_instanced(&context, topology, &DrawArgs)` | 数组实例化绘制 |
| `draw_element_range(...)` | 索引区间绘制 |
| `bind_texture_unit(&context, unit, &GlTexture)` | 绑定纹理单元，重复绑定会被跳过 |
| `read_pixels(&context, x, y, width, height) -> Vec<u8>` | 回读像素 |

固定管线状态由 `GlRenderState` 描述，内含 `GlBlendState`、`GlDepthState`、`GlCullState`、`GlColorMask`、`GlScissor`、`GlViewport`。`GlRenderState::context_defaults(width, height)` 是目前构造它的入口。

### WebGl2InitError

| 变体 | 含义 |
| --- | --- |
| `CanvasNotFound(String)` | 选择器没匹配到 canvas |
| `CanvasQuery(String)` | 查询选择器时抛异常 |
| `ContextUnavailable` | `get_context("webgl2")` 返回 `None` |
| `ContextLookup(String)` | 反射查找失败 |
| `ContextCast` | 句柄无法转成 `WebGl2RenderingContext` |

每个变体都有独立的 `code() -> &'static str`。

### 着色器与帧缓冲

- `GlShaderKind` —— `Vertex` / `Fragment`，避免直接传裸 `u32`。
- `GlFramebufferStatus` —— `Complete`（全部就绪）/ `Incomplete`（缺附件）/ `Unsupported`（附件尺寸或采样数不一致）。

### 真实用法

```rust
use euv_engine::{Engine, GlProgram, RenderConfig, WebGl2Backend};

// 同步，不需要 await
fn setup() -> Result<(), String> {
    let config: RenderConfig = RenderConfig::webgl("#stage", 800.0, 600.0);
    let renderer: WebGl2Backend = Engine::webgl_renderer(&config).map_err(|e| e.to_string())?;

    let context = renderer.get_context().clone();
    let program: GlProgram = GlProgram::create(
        &context,
        GAME_2D_WEBGL_VERTEX_SHADER,
        GAME_2D_WEBGL_FRAGMENT_SHADER,
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
```

WebGPU 路径则需要包在 async 块里：

```rust
use euv::{wasm_bindgen::JsValue, wasm_bindgen_futures::spawn_local, web_sys::console};
use euv_engine::{Engine, RenderConfig, WebGpuInitError, WebGpuRenderer};

spawn_local(async move {
    let config: RenderConfig = RenderConfig::webgpu("#stage", 800.0, 600.0);
    let renderer: Result<WebGpuRenderer, WebGpuInitError> =
        Engine::webgpu_renderer(&config).await;

    let renderer: WebGpuRenderer = match renderer {
        Ok(value) => value,
        Err(error) => {
            console::error_1(&JsValue::from_str(&format!("webgpu init failed: {error}")));
            return;
        }
    };

    let pipeline = renderer.create_render_pipeline(GAME_2D_WEBGPU_SHADER);
    let uniform_buffer = renderer.create_uniform_buffer(&vec![0.0; 4 + MAX_BALLS * 8]);
    let bind_group = renderer.create_uniform_bind_group(&pipeline, &uniform_buffer);
});
```

::: tip
`console` 是 `web_sys` 提供的浏览器全局函数模块（`euv` crate 有 `pub use web_sys`），不是 `euv_engine` 的导出 —— `use euv::web_sys::console;` 后调用 `console::error_1(...)`（`_1` 表示单个参数的重载）。`create_render_pipeline` 这类方法需要真实的 GPU 环境，完整示例见 `example/src/page/game_2d/hook/fn.rs`。
:::

## 未逐个展开的 pub 项

`renderer` 是全 crate 最大的模块，约 341 个 `pub` 项，其中绝大多数是 WebGPU 的 GPU 常量表、位标志与 descriptor 字段 —— 它们成组出现、命名规整，逐个列出反而淹没重点。下列分类给出数量与源码位置，供按需查阅。

| 类别 | 数量 | 源码位置 |
| --- | --- | --- |
| WebGPU 枚举（`BufferUsage`、`TextureUsage`、`LoadOp`、`StoreOp`、`CompareFunction`、`BlendFactor`、`BlendOperation`、`PrimitiveTopology`、`CullMode`、`FrontFace`、`IndexFormat`、`VertexAttributeFormat`、`GpuTextureFormat`、`GpuErrorFilter`、`FilterMode`、`MipmapFilter`、`AddressMode`、`VertexStepMode`、`ShaderStage`） | 19 | `renderer/status/enum.rs` |
| WebGPU descriptor 结构体（`RenderPipelineDescriptor`、`ColorAttachment`、`DepthStencilAttachment`、`Texture2DDescriptor`、`SamplerDescriptor` 等） | 19 | `renderer/descriptor/struct.rs` |
| WebGPU descriptor 构造器与 `to_js` 转换 | 约 25 | `renderer/descriptor/impl.rs` |
| WebGPU 字符串常量表（格式串、方法名、usage 位值等 `pub(crate)`） | 大量 | `renderer/webgpu/const.rs`、`renderer/descriptor/const.rs` |
| WebGL 资源句柄（`GlBuffer`、`GlTexture`、`GlVertexArray`、`GlFramebuffer`、`GlProgram`、`GlUniformBlock`） | 6 | `renderer/webgl/struct.rs` |
| WebGL 状态结构体（`GlRenderState`、`GlBlendState`、`GlDepthState`、`GlCullState`、`GlColorMask`、`GlScissor`、`GlViewport` 及各自的 Init） | 10 | `renderer/webgl/struct.rs` |
| WebGL 内部常量与 `Reflect` 辅助 | 大量 | `renderer/webgl/const.rs`、`renderer/webgl/fn.rs` |
| WebGPU 渲染器内部方法（纹理写入、拷贝、离线渲染目标等） | 约 60 | `renderer/webgpu/impl.rs` |
| Canvas 2D 常量与绘制辅助 | 约 21 | `renderer/canvas/const.rs`、`renderer/canvas/fn.rs` |
| 位标志辅助函数（`buffer_usage_bit`、`shader_stage_bit` 等） | 少量 | `renderer/status/fn.rs` |
| 内部默认值常量（`CONFIG_DEFAULT_*`、`GPU_POWER_PREFERENCE_*`） | 7 | `config/const.rs`、`renderer/canvas/const.rs` |

::: tip
这些项基本都通过同样的扁平路径导入：`use euv_engine::{GpuTextureFormat, PrimitiveTopology, ...};`。需要某个具体项时，直接在上述文件里搜类型名即可。
:::

## 常见陷阱

1. **不要写 `euv_engine::renderer::canvas::X` 或 `euv_engine::config::X`** —— 模块私有，是编译错误。正确写法是 `euv_engine::X`。
2. **不要写 `config.set_quality(...)`** —— setter 是 `pub(crate)`，外部调用编译失败。`get_quality()` 可用。
3. **不要用结构体字面量构造 descriptor** —— 字段是 `pub(crate)`。用 `new(...)` 或具名构造器。
4. **`Engine::webgpu_renderer` 是 async，`Engine::webgl_renderer` 是同步** —— 别给后者加 `.await`。
5. **`DrawList::len()` 会计入状态命令** —— `set_blend_mode` / `set_global_alpha` 也占一条记录。
6. **Canvas 2D 有两条使用路径** —— 示例页走 `SsaaCanvas` + 原生上下文，`CanvasRenderer` / `DrawList` 是另一套。别混用。