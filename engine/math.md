---
title: 数学库
order: 7
---

`euv_engine` 的数学库，全部类型都通过 `pub use` 从 crate 根直接导出，无需 `use euv_engine::math::...`。所有数值一律使用 `f64`。

## 概览

| 分类 | 内容 |
| --- | --- |
| 命名空间 | [`Numeric`](#numeric) |
| 2D | [`Vector2D`](#vector2d)、[`Rect`](#rect)、[`Circle`](#circle)、[`Transform2D`](#transform2d)、[`Ray2D`](#ray2d) |
| 3D | [`Vector3D`](#vector3d)、[`Quaternion`](#quaternion)、[`Matrix4x4`](#matrix4x4)、[`Transform3D`](#transform3d) |
| 3D 几何 | [`AABB3D`](#aabb3d)、[`Sphere`](#sphere)、[`Plane`](#plane)、[`Ray3D`](#ray3d) |
| 颜色 | [`Color`](#color) |
| Trait | [`Interpolable`](#interpolable)、[`Vector`](#vector) |

## 派生代码约定

所有几何类型都标注了 `#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]`，其中 `Data` / `New` 来自 `lombok-macros`，它按字段生成访问器：

- `Type::new(...)` —— 按字段声明顺序接收全部字段，逐字段构造。
- `Type::get_field() -> T` —— 返回**值拷贝**（标注了 `type(copy)`），不是引用。
- `Type::get_mut_field()` / `Type::set_field(v)` —— 标注为 `pub(crate)`，**crate 外不可见**。

因此外部代码只能通过 `new` 构造、通过 `get_*` 读取；修改要靠返回新值的 API，或用 `Copy` 语义重建整个值。

```rust
use euv_engine::*;

let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
assert_eq!(rect.get_x(), 10.0);
assert_eq!(rect.get_width(), 100.0);

// 字段不可变，修改靠重新构造：
let moved = Rect::new(rect.get_x() + 5.0, rect.get_y(), rect.get_width(), rect.get_height());
```

`Numeric` 是一个零尺寸命名空间 struct，没有字段，全部方法都是关联函数。

## Numeric

自由函数式的数学工具，全部通过 `Numeric::` 调用。

### 基础插值与截断

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `clamp` | `(value: f64, min: f64, max: f64) -> f64` | 把值限制在 `[min, max]`，内部实现为 `value.max(min).min(max)`。 |
| `lerp` | `(start: f64, end: f64, factor: f64) -> f64` | 线性插值 `start + (end - start) * factor`。`factor` 为 `0.0` 返回 `start`，`1.0` 返回 `end`，超出范围会外推。 |
| `smoothstep` | `(edge_min: f64, edge_max: f64, value: f64) -> f64` | 三次 Hermite 平滑因子。先把归一化值截断到 `[0,1]`，再算 `t * t * (3 - 2t)`。两端斜率为零，中点为 `0.5`。 |
| `approach` | `(current: f64, target: f64, max_delta: f64) -> f64` | 向 `target` 最多移动 `max_delta.abs()`。一步就能到达时直接返回 `target`（不抖动、不超越）。 |

```rust
use euv_engine::Numeric;

assert_eq!(Numeric::clamp(5.0, 0.0, 1.0), 1.0);
assert_eq!(Numeric::clamp(-5.0, 0.0, 1.0), 0.0);
assert_eq!(Numeric::lerp(10.0, 20.0, 0.25), 12.5);
assert_eq!(Numeric::smoothstep(0.0, 1.0, 0.5), 0.5);
assert_eq!(Numeric::approach(0.0, 10.0, 3.0), 3.0);
assert_eq!(Numeric::approach(0.0, 1.0, 3.0), 1.0);
```

### 角度

引擎内部角度一律用**弧度**。这四个函数只做弧度↔角度换算和角度归一化。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `deg_to_rad` | `(degrees: f64) -> f64` | 角度转弧度。 |
| `rad_to_deg` | `(radians: f64) -> f64` | 弧度转角度。 |
| `normalize_angle` | `(radians: f64) -> f64` | 把任意角度折叠到 `[-π, π]`。 |
| `angle_delta` | `(from: f64, to: f64) -> f64` | `normalize_angle(to - from)`，即两角之间的**最短有符号差值**，落在 `[-π, π]`。 |
| `lerp_angle` | `(from: f64, to: f64, factor: f64) -> f64` | 沿**最短路径**插值：`from + angle_delta(from, to) * factor`。 |

`normalize_angle` 先取 `% TWO_PI`，再分两支折回，因此整圈恰好归零：

```rust
use euv_engine::Numeric;

let pi: f64 = std::f64::consts::PI;
let tau: f64 = std::f64::consts::TAU;

assert!((Numeric::deg_to_rad(180.0) - pi).abs() < 1e-12);
assert!((Numeric::rad_to_deg(pi) - 180.0).abs() < 1e-12);

// 整圈归零。
assert!(Numeric::normalize_angle(tau).abs() < 1e-12);

// 越过接缝时走短边：tau - 0.1 与 0 只差 -0.1，而不是绕一整圈 +6.18。
assert!((Numeric::angle_delta(0.0, tau - 0.1) + 0.1).abs() < 1e-12);

// lerp_angle 不会因为 6.2 弧度这种表示而选错方向：
// 普通 lerp 会插到 3.1（绕了远路），lerp_angle 走 -0.042 的短路。
assert!(Numeric::lerp(0.0, 6.2, 0.5) > 3.0);
assert!((Numeric::lerp_angle(0.0, 6.2, 0.5) + 0.0416).abs() < 1e-3);
```

关键区别：`Numeric::lerp(0.0, 6.2, 0.5)` 得到 `3.1`，因为它只做算术平均；`Numeric::lerp_angle(0.0, 6.2, 0.5)` 得到约 `-0.042`，因为它绕圆周走了更近的那一侧。做旋转动画插值时务必用 `lerp_angle`。

### 符号与取模

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `sign` | `(value: f64) -> f64` | 返回 `-1.0` / `0.0` / `1.0`。零返回 `0.0`。 |
| `sign_or_positive` | `(value: f64) -> f64` | 返回 `1.0` 或 `-1.0`，**零返回 `1.0`**（无零值）。 |
| `wrap` | `(value: f64, max: f64) -> f64` | 折进半开区间 `[0, max)`：负值会加上 `max` 绕上来。 |

```rust
use euv_engine::Numeric;

assert_eq!(Numeric::sign(-2.0), -1.0);
assert_eq!(Numeric::sign(0.0), 0.0);
assert_eq!(Numeric::sign_or_positive(0.0), 1.0);
assert_eq!(Numeric::wrap(7.0, 5.0), 2.0);
assert_eq!(Numeric::wrap(-1.0, 5.0), 4.0);
```

### 距离

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `distance` | `(a: Vector2D, b: Vector2D) -> f64` | 两点欧氏距离，等价于 `(b - a).magnitude()`。 |
| `distance_squared` | `(a: Vector2D, b: Vector2D) -> f64` | 平方距离，省掉开方。只在**比较距离大小**时用它。 |
| `distance_3d` | `(a: Vector3D, b: Vector3D) -> f64` | 3D 版距离。 |
| `distance_squared_3d` | `(a: Vector3D, b: Vector3D) -> f64` | 3D 版平方距离。 |

```rust
use euv_engine::*;

let a = Vector2D::new(0.0, 0.0);
let b = Vector2D::new(3.0, 4.0);
assert_eq!(Numeric::distance(a, b), 5.0);
assert_eq!(Numeric::distance_squared(a, b), 25.0);
```

## Vector2D

2D 向量，屏幕坐标系（`x` 向右，`y` 向下）。

```rust
use euv_engine::*;

let v = Vector2D::new(3.0, 4.0);
let _: f64 = v.get_x();
let _: f64 = v.get_y();
```

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `zero` | `() -> Vector2D` | `(0.0, 0.0)`。 |
| `right` | `() -> Vector2D` | `(1.0, 0.0)`。 |
| `up` | `() -> Vector2D` | `(0.0, -1.0)`。**y 向下，所以"上"是负 y**。 |
| `from_angle` | `(radians: f64) -> Vector2D` | 由角度求单位向量，`(cos, sin)`。 |
| `magnitude` | `(&self) -> f64` | 向量长度。 |
| `magnitude_squared` | `(&self) -> f64` | 长度平方，比较用免开方。 |
| `normalized` | `(&self) -> Vector2D` | 单位长度副本；模长小于 `1e-6` 时返回零向量。 |
| `normalize` | `(&mut self)` | 原地归一化；同样把近零向量置零。 |
| `dot` | `(&self, other: Vector2D) -> f64` | 点积 `x₁x₂ + y₁y₂`。 |
| `cross` | `(&self, other: Vector2D) -> f64` | 2D 叉积，返回**标量** `x₁y₂ - y₁x₂`。 |
| `perp` | `(&self) -> Vector2D` | 逆时针转 90° 的垂线 `(-y, x)`，长度不变。 |
| `angle` | `(&self) -> f64` | `atan2(y, x)`，自身方位角。 |
| `angle_to` | `(&self, other: Vector2D) -> f64` | `(other - self).angle()`，即指向目标的有符号夹角。 |
| `rotated` | `(&self, radians: f64) -> Vector2D` | 返回旋转后的**新**向量，原值不变。 |
| `rotate` | `(&mut self, radians: f64)` | 原地旋转。 |
| `distance_to` | `(&self, other: Vector2D) -> f64` | 到目标点的距离。 |
| `distance_squared_to` | `(&self, other: Vector2D) -> f64` | 到目标点的平方距离。 |
| `direction_to` | `(&self, other: Vector2D) -> Vector2D` | 指向目标的单位向量。 |
| `lerp` | `(&self, other: Vector2D, factor: f64) -> Vector2D` | 逐分量线性插值。 |
| `scale` | `(&mut self, scalar: f64)` | 原地缩放。 |
| `scaled` | `(&self, scalar: f64) -> Vector2D` | 返回缩放后的副本。 |

```rust
use euv_engine::*;

assert_eq!(Vector2D::up(), Vector2D::new(0.0, -1.0));

let v = Vector2D::new(3.0, 4.0);
assert_eq!(v.magnitude(), 5.0);
assert_eq!(v.magnitude_squared(), v.magnitude() * v.magnitude());
assert_eq!(v.normalized().magnitude(), 1.0);

assert_eq!(v.cross(Vector2D::new(1.0, 0.0)), -4.0);
assert_eq!(v.perp(), Vector2D::new(-4.0, 3.0));
assert_eq!(v.dot(Vector2D::right()), 3.0);

let turned = v.rotated(std::f64::consts::FRAC_PI_2);
assert!((turned.get_x() + 4.0).abs() < 1e-12);
```

`direction_to` 在两点重合时返回零向量（无法归一化），此时判断朝向不要依赖它。

## Vector3D

3D 向量，右手坐标系（`up` 为 `+y`，`forward` 为 `-z`）。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `zero` | `() -> Vector3D` | `(0.0, 0.0, 0.0)`。 |
| `right` | `() -> Vector3D` | `(1.0, 0.0, 0.0)`。 |
| `up` | `() -> Vector3D` | `(0.0, 1.0, 0.0)`。 |
| `forward` | `() -> Vector3D` | `(0.0, 0.0, -1.0)`。 |
| `magnitude` | `(&self) -> f64` | 向量长度。 |
| `magnitude_squared` | `(&self) -> f64` | 长度平方。 |
| `normalized` | `(&self) -> Vector3D` | 单位长度副本；模长小于 `1e-6` 时返回零向量。 |
| `normalize` | `(&mut self)` | 原地归一化。 |
| `dot` | `(&self, other: Vector3D) -> f64` | 点积。 |
| `cross` | `(&self, other: Vector3D) -> Vector3D` | 叉积，返回**向量**（与 2D 的标量叉积不同）。 |
| `distance_to` | `(&self, other: Vector3D) -> f64` | 到目标点的距离。 |
| `distance_squared_to` | `(&self, other: Vector3D) -> f64` | 到目标点的平方距离。 |
| `direction_to` | `(&self, other: Vector3D) -> Vector3D` | 指向目标的单位向量。 |
| `lerp` | `(&self, other: Vector3D, factor: f64) -> Vector3D` | 逐分量线性插值。 |
| `scale` | `(&mut self, scalar: f64)` | 原地缩放。 |
| `scaled` | `(&self, scalar: f64) -> Vector3D` | 返回缩放后的副本。 |
| `rotated_by` | `(&self, quaternion: Quaternion) -> Vector3D` | 用四元数旋转向量，内部算 `q * pure * q⁻¹`。 |

```rust
use euv_engine::*;

assert_eq!(Vector3D::up(), Vector3D::new(0.0, 1.0, 0.0));
assert_eq!(Vector3D::forward(), Vector3D::new(0.0, 0.0, -1.0));

let x = Vector3D::right();
let y = Vector3D::up();
assert_eq!(x.cross(y), Vector3D::forward());

let v = Vector3D::new(1.0, 2.0, 2.0);
assert_eq!(v.magnitude(), 3.0);

let quarter = v.rotated_by(Quaternion::from_axis_angle(Vector3D::up(), std::f64::consts::FRAC_PI_2));
assert!((quarter.magnitude() - v.magnitude()).abs() < 1e-12);
```

## 运算符

`Vector2D` 与 `Vector3D` 都实现了完整的算术运算符，`Quaternion` 实现了四元数乘法。

| 运算符 | 含义 |
| --- | --- |
| `a + b` | 分量相加 |
| `a - b` | 分量相减 |
| `a * s` | 标量缩放（右侧为 `f64`） |
| `-a` | 取负 |
| `a += b` / `a -= b` / `a *= s` | 对应运算的原地版本 |
| `q1 * q2` | 四元数复合（**仅 `Quaternion`**），表示先转 `q2` 再转 `q1` |

```rust
use euv_engine::*;

let a = Vector2D::new(1.0, 2.0);
let b = Vector2D::new(3.0, 4.0);
assert_eq!(a + b, Vector2D::new(4.0, 6.0));
assert_eq!(a * 3.0, Vector2D::new(3.0, 6.0));

let mut acc = a;
acc += b;
acc *= 2.0;
assert_eq!(acc, Vector2D::new(8.0, 12.0));
```

## Rect

轴对齐矩形，以左上角坐标加宽高表示。

`Rect::new(x, y, width, height)`，`get_x` / `get_y` / `get_width` / `get_height`。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `from_center` | `(center: Vector2D, width: f64, height: f64) -> Rect` | 由中心点和尺寸构造，内部转成左上角坐标。 |
| `center` | `(&self) -> Vector2D` | 中心点。 |
| `min` | `(&self) -> Vector2D` | 最小角（左上）。 |
| `max` | `(&self) -> Vector2D` | 最大角（右下）。 |
| `size` | `(&self) -> Vector2D` | 尺寸向量。 |
| `contains` | `(&self, point: Vector2D) -> bool` | 点是否落在矩形内，**边界算作在内**。 |
| `intersects` | `(&self, other: Rect) -> bool` | 两矩形是否相交（严格不等式，边缘相切不算相交）。 |
| `broad_phase_alias` | `(a: Rect, b: Rect) -> bool` | `intersects` 的别名，供物理模块的粗筛阶段调用。 |
| `intersection` | `(&self, other: Rect) -> Option<Rect>` | 求交集矩形，不重叠返回 `None`。 |

```rust
use euv_engine::*;

let r = Rect::from_center(Vector2D::new(5.0, 5.0), 4.0, 6.0);
assert_eq!(r.center(), Vector2D::new(5.0, 5.0));
assert_eq!(r.size(), Vector2D::new(4.0, 6.0));
assert_eq!(r.min(), Vector2D::new(3.0, 2.0));
assert_eq!(r.max(), Vector2D::new(7.0, 8.0));

assert!(r.contains(Vector2D::new(3.0, 2.0)));   // 左上角算在内
assert!(!r.contains(Vector2D::new(0.0, 0.0)));

let other = Rect::from_center(Vector2D::new(6.0, 5.0), 4.0, 6.0);
assert!(r.intersects(other));
assert!(r.intersection(other).is_some());
```

`Rect` 实现了 `Default`（全零矩形），所有字段也可直接用 `new` 构造。

## Circle

圆，由圆心与半径定义。`Circle::new(center: Vector2D, radius: f64)`。

访问器：`get_center() -> Vector2D`、`get_radius() -> f64`。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `contains` | `(&self, point: Vector2D) -> bool` | 点是否在圆内。 |
| `intersects` | `(&self, other: Circle) -> bool` | 两圆是否相交。 |
| `circumference` | `(&self) -> f64` | 周长 `2πr`。 |
| `area` | `(&self) -> f64` | 面积 `πr²`。 |

```rust
use euv_engine::*;

let c = Circle::new(Vector2D::zero(), 2.0);
assert!(c.contains(Vector2D::new(1.0, 0.0)));
assert!(!c.contains(Vector2D::new(3.0, 0.0)));
assert!(c.intersects(Circle::new(Vector2D::new(3.0, 0.0), 2.0)));
```

## Color

RGBA 颜色，四个通道都是 `f64` 取值 `0.0`~`1.0`。`Color::new(red, green, blue, alpha)`。

访问器：`get_red()` / `get_green()` / `get_blue()` / `get_alpha()`。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `from_rgb` | `(red: u8, green: u8, blue: u8) -> Color` | 从 `0`-`255` 的整数字面量构造，alpha 固定为 `1.0`。 |
| `to_css_rgba` | `(&self) -> String` | 生成 CSS `rgba()` 字符串，会**重新分配** `String`。 |
| `write_css_rgba` | `(&self, buffer: &mut String)` | 把 CSS 文本追加进调用方已有的缓冲区，**复用其内存**。 |
| `black` | `() -> Color` | `(0, 0, 0, 1)`。 |
| `white` | `() -> Color` | `(1, 1, 1, 1)`。 |
| `transparent` | `() -> Color` | `(0, 0, 0, 0)`。 |
| `lerp` | `(&self, other: Color, factor: f64) -> Color` | 四个通道各自独立插值。 |

```rust
use euv_engine::*;

let magenta = Color::from_rgb(255, 0, 128);
assert_eq!(magenta.get_red(), 1.0);
assert!(magenta.to_css_rgba().starts_with("rgba("));

// 逐帧渲染的热路径里用 write_css_rgba 复用缓冲区：
let mut buffer = String::with_capacity(32);
magenta.write_css_rgba(&mut buffer);
assert_eq!(buffer, magenta.to_css_rgba());

let fade = Color::black().lerp(Color::white(), 0.5);
assert_eq!(fade.get_red(), 0.5);
```

`write_css_rgba` 把 RGB 三通道四舍五入成整数写入，alpha 保留浮点原样。

`Color::default()` 等价于 `Color::black()`。

## Transform2D

2D 变换，由位置、旋转（弧度）、缩放组成。`Transform2D::new(position, rotation, scale)`。

- `get_position() -> Vector2D` 可公开读取。
- `get_rotation()` 与 `get_scale()` 是 `pub(crate)`，**外部不可见**。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `identity` | `() -> Transform2D` | 原点、无旋转、单位缩放。 |
| `translate` | `(&mut self, offset: Vector2D)` | 位置加上偏移量。 |
| `rotate` | `(&mut self, radians: f64)` | 旋转角**累加**。 |
| `scale_by` | `(&mut self, factors: Vector2D)` | 各轴缩放因子**相乘**。 |
| `apply_to_point` | `(&self, point: Vector2D) -> Vector2D` | 局部坐标转世界坐标，顺序为**先缩放、再旋转、后平移**。 |

```rust
use euv_engine::*;

let mut t = Transform2D::identity();
assert_eq!(t.apply_to_point(Vector2D::new(3.0, 4.0)), Vector2D::new(3.0, 4.0));

t.translate(Vector2D::new(10.0, 0.0));
assert_eq!(t.apply_to_point(Vector2D::zero()), Vector2D::new(10.0, 0.0));

t.rotate(std::f64::consts::FRAC_PI_2);
let turned = t.apply_to_point(Vector2D::zero());
assert!((turned.get_y() - 10.0).abs() < 1e-12);
```

`Transform2D::default()` 等价于 `Transform2D::identity()`。

## Transform3D

3D 变换，由位置、四元数旋转、缩放组成。`Transform3D::new(position, rotation, scale)`。

- `get_position() -> Vector3D` 可公开读取。
- `get_rotation()` 与 `get_scale()` 是 `pub(crate)`，**外部不可见**。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `identity` | `() -> Transform3D` | 原点、无旋转、单位缩放。 |
| `translate` | `(&mut self, offset: Vector3D)` | 位置加上偏移量。 |
| `rotate` | `(&mut self, rotation: Quaternion)` | 后乘一个四元数旋转。 |
| `scale_by` | `(&mut self, factors: Vector3D)` | 各轴缩放因子相乘。 |
| `apply_to_point` | `(&self, point: Vector3D) -> Vector3D` | 局部坐标转世界坐标：先缩放、再旋转、后平移。 |
| `to_matrix` | `(&self) -> Matrix4x4` | 合成为 `T * R * S` 矩阵。 |

```rust
use euv_engine::*;

let mut t = Transform3D::identity();
t.translate(Vector3D::new(1.0, 2.0, 3.0));
assert_eq!(t.get_position(), Vector3D::new(1.0, 2.0, 3.0));
assert_eq!(t.apply_to_point(Vector3D::zero()), Vector3D::new(1.0, 2.0, 3.0));

// to_matrix 与 apply_to_point 结果一致。
let matrix = t.to_matrix();
let via_matrix = matrix.transform_point(Vector3D::new(0.0, 1.0, 0.0));
let direct = t.apply_to_point(Vector3D::new(0.0, 1.0, 0.0));
assert!((via_matrix.get_x() - direct.get_x()).abs() < 1e-12);
```

## Quaternion

3D 旋转四元数，`Quaternion::new(x, y, z, w)`。访问器 `get_x()` / `get_y()` / `get_z()` / `get_w()`。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `identity` | `() -> Quaternion` | `(0, 0, 0, 1)`，无旋转。 |
| `from_axis_angle` | `(axis: Vector3D, angle: f64) -> Vector3D` → `Quaternion` | 绕轴旋转；**内部会先归一化轴**，因此传未归一化的轴也正确。 |
| `from_euler` | `(yaw: f64, pitch: f64, roll: f64) -> Quaternion` | 欧拉角转四元数，三个角均为弧度：yaw 绕 y、pitch 绕 x、roll 绕 z。 |
| `magnitude` | `(&self) -> f64` | 四元数模长。 |
| `normalized` | `(&self) -> Quaternion` | 归一化副本；模长小于 `1e-6` 时**返回单位四元数**（而非零四元数）。 |
| `conjugate` | `(&self) -> Quaternion` | 共轭 `(-x, -y, -z, w)`。 |
| `dot` | `(&self, other: Quaternion) -> f64` | 点积。 |
| `slerp` | `(&self, other: Quaternion, factor: f64) -> Quaternion` | 球面线性插值，沿单位球面匀速过渡。 |
| `Mul` | `q1 * q2 -> Quaternion` | 四元数复合。 |

`slerp` 会自动处理两种退化情况：点积为负时翻转 `other` 取最短路径；两点几乎重合（点积接近 1）时退化为线性插值再归一化，避免除零。

```rust
use euv_engine::*;

let identity = Quaternion::identity();
assert!((identity.magnitude() - 1.0).abs() < 1e-12);

// from_axis_angle 保持单位长度。
let spin = Quaternion::from_axis_angle(Vector3D::up(), std::f64::consts::FRAC_PI_2);
assert!((spin.magnitude() - 1.0).abs() < 1e-12);

// 绕 Y 轴转 90° 把 +X 转到 -Z。
let turned = Vector3D::right().rotated_by(spin);
assert!((turned.get_z() + 1.0).abs() < 1e-12);

let half = identity.slerp(spin, 0.5);
assert!((half.magnitude() - 1.0).abs() < 1e-12);
```

## Matrix4x4

4×4 矩阵，按**列主序**（column-major）存放在 `elements: [f64; 16]` 里。`Matrix4x4::new(elements)`，访问器 `get_elements() -> [f64; 16]`。

列主序意味着 `elements[0..4]` 是第一**列**，`transform_point` 里 `elements[0]`、`elements[4]`、`elements[8]`、`elements[12]` 才共同决定 x 分量。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `identity` | `() -> Matrix4x4` | 单位矩阵。 |
| `translation` | `(translation: Vector3D) -> Matrix4x4` | 平移矩阵。 |
| `scaling` | `(scale: Vector3D) -> Matrix4x4` | 缩放矩阵。 |
| `rotation` | `(quaternion: Quaternion) -> Matrix4x4` | 由四元数生成旋转矩阵。 |
| `perspective` | `(fov: f64, aspect: f64, near: f64, far: f64) -> Matrix4x4` | 透视投影；`fov` 是**垂直**视场角（弧度），`aspect` 是宽高比。 |
| `orthographic` | `(left, right, bottom, top, near, far) -> Matrix4x4` | 正交投影。 |
| `look_at` | `(eye: Vector3D, target: Vector3D, up: Vector3D) -> Matrix4x4` | 构造视图矩阵。 |
| `multiply` | `(&self, other: Matrix4x4) -> Matrix4x4` | 矩阵乘法，返回新矩阵；完全展开算术以提升指令级并行。 |
| `transform_point` | `(&self, point: Vector3D) -> Vector3D` | 变换点并**做透视除法**（除以 `w`）。 |

```rust
use euv_engine::*;

let identity = Matrix4x4::identity();
let p = Vector3D::new(1.0, 2.0, 3.0);
assert_eq!(identity.transform_point(p), p);

// 先缩后转再平移，等价于直接相乘。
let m = Matrix4x4::translation(Vector3D::new(1.0, 0.0, 0.0))
    .multiply(Matrix4x4::scaling(Vector3D::new(2.0, 2.0, 2.0)));
assert_eq!(m.transform_point(Vector3D::new(1.0, 1.0, 0.0)), Vector3D::new(3.0, 2.0, 0.0));

let view = Matrix4x4::look_at(Vector3D::zero(), Vector3D::forward(), Vector3D::up());
let proj = Matrix4x4::perspective(1.0, 16.0 / 9.0, 0.1, 1000.0);
let view_projection = proj.multiply(view);
```

`transform_point` 在 `w` 接近零（`|w| < 1e-6`）时跳过透视除法，直接返回 `(x, y, z)`，避免出现 `NaN`。

`Matrix4x4::default()` 等价于 `Matrix4x4::identity()`。

## AABB3D

3D 轴对齐包围盒，由最小角与最大角定义。`AABB3D::new(min: Vector3D, max: Vector3D)`，访问器 `get_min()` / `get_max()`。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `from_center` | `(center: Vector3D, width: f64, height: f64, depth: f64) -> AABB3D` | 由中心点与三轴尺寸构造。 |
| `center` | `(&self) -> Vector3D` | 中心点。 |
| `size` | `(&self) -> Vector3D` | 尺寸向量 `(width, height, depth)`。 |
| `contains` | `(&self, point: Vector3D) -> bool` | 点是否在盒内。 |
| `intersects` | `(&self, other: AABB3D) -> bool` | 两盒是否相交。 |

```rust
use euv_engine::*;

let b = AABB3D::from_center(Vector3D::zero(), 2.0, 4.0, 6.0);
assert_eq!(b.center(), Vector3D::zero());
assert_eq!(b.size(), Vector3D::new(2.0, 4.0, 6.0));
assert!(b.contains(Vector3D::new(0.5, 0.5, 0.5)));
assert!(!b.contains(Vector3D::new(5.0, 0.0, 0.0)));
```

## Sphere

球体，由球心与半径定义。`Sphere::new(center: Vector3D, radius: f64)`，访问器 `get_center()` / `get_radius()`。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `contains` | `(&self, point: Vector3D) -> bool` | 点是否在球内。 |
| `intersects` | `(&self, other: Sphere) -> bool` | 两球是否相交。 |
| `volume` | `(&self) -> f64` | 体积 `4/3 · πr³`。 |
| `surface_area` | `(&self) -> f64` | 表面积 `4πr²`。 |

```rust
use euv_engine::*;

let s = Sphere::new(Vector3D::zero(), 3.0);
assert!(s.contains(Vector3D::new(1.0, 0.0, 0.0)));
assert!(!s.contains(Vector3D::new(9.0, 0.0, 0.0)));
assert!(s.intersects(Sphere::new(Vector3D::new(5.0, 0.0, 0.0), 3.0)));
```

## Plane

平面，由法向量与到原点的有符号距离定义，满足 `normal · p + distance = 0`。`Plane::new(normal: Vector3D, distance: f64)`，访问器 `get_normal()` / `get_distance()`。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `from_normal_and_point` | `(normal: Vector3D, point: Vector3D) -> Plane` | 由法向量与平面上一点构造；法向量**内部已归一化**。 |
| `distance_to_point` | `(&self, point: Vector3D) -> bool` → `f64` | 点到平面的**有符号**距离，法向量正侧为正。 |
| `normalize` | `(&mut self)` | 归一化法向量并同步调整 `distance`；法向量模长过小时原样保留。 |

```rust
use euv_engine::*;

let ground = Plane::from_normal_and_point(Vector3D::up(), Vector3D::zero());
assert!((ground.distance_to_point(Vector3D::new(0.0, 5.0, 0.0)) - 5.0).abs() < 1e-12);
assert!(ground.distance_to_point(Vector3D::zero()).abs() < 1e-12);

// 平面下方得到负距离。
assert!(ground.distance_to_point(Vector3D::new(0.0, -5.0, 0.0)) < 0.0);
```

## Ray3D

3D 射线。`Ray3D::new(origin: Vector3D, direction: Vector3D)`，访问器 `get_origin()` / `get_direction()`。方向向量应当是单位向量。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `point_at` | `(&self, t: f64) -> Vector3D` | 沿射线前进 `t` 得到的点。 |
| `intersect_sphere` | `(&self, sphere: Sphere) -> Option<f64>` | 与球求交，返回**最近的正向**距离；未命中或全在背后返回 `None`。 |
| `intersect_plane` | `(&self, plane: Plane) -> Option<f64>` | 与平面求交，返回距离；射线与平面平行时返回 `None`。 |
| `intersect_aabb` | `(&self, aabb: AABB3D) -> Option<f64>` | 与包围盒求交，返回进入盒体的距离；未命中返回 `None`。 |

```rust
use euv_engine::*;

let ray = Ray3D::new(Vector3D::new(0.0, 0.0, 5.0), Vector3D::forward());
assert_eq!(ray.point_at(1.0), Vector3D::new(0.0, 0.0, 4.0));

// 命中球体，返回沿射线行进的距离。
let target = Sphere::new(Vector3D::zero(), 1.0);
let hit = ray.intersect_sphere(target).expect("the ray points at the sphere");
assert!((hit - 4.0).abs() < 1e-9);

let ground = Plane::from_normal_and_point(Vector3D::up(), Vector3D::zero());
assert!(ray.intersect_plane(ground).is_some());

// 背离方向的射线不命中。
let away = Ray3D::new(Vector3D::new(0.0, 0.0, 5.0), Vector3D::up());
assert!(away.intersect_plane(ground).is_none());
```

三个求交方法只返回**非负**距离——位于射线起点背后的交点会被当作未命中。

## Ray2D

2D 射线，用于对 2D 图形做射线检测。`Ray2D::new(origin: Vector2D, direction: Vector2D)`，访问器 `get_origin()` / `get_direction()`。

该类型目前只提供构造、读取与 `Copy` / `Default` 等派生能力，**尚未提供任何 `intersect_*` 方法**；需要自行用 `Vector2D` 的 `cross` / `dot` 求解，或等待后续版本补齐。

```rust
use euv_engine::*;

let ray = Ray2D::new(Vector2D::new(1.0, 2.0), Vector2D::new(0.0, 1.0));
assert_eq!(ray.get_origin(), Vector2D::new(1.0, 2.0));
assert_eq!(ray.get_direction(), Vector2D::new(0.0, 1.0));

let copied = ray;
assert_eq!(copied, ray);   // Ray2D 是 Copy

assert_eq!(Ray2D::default().get_origin(), Vector2D::zero());
```

## Interpolable

为支持线性插值的类型提供统一接口。

```rust
pub trait Interpolable {
    fn lerp(&self, other: Self, factor: f64) -> Self;
}
```

已实现：`f64`、`Vector2D`、`Vector3D`、`Color`。`factor` 通常取 `0.0`~`1.0`，超出范围会外推。

```rust
use euv_engine::*;

// 对任意 Interpolable 做插值
fn halfway<T: Interpolable>(a: T, b: T) -> T {
    a.lerp(b, 0.5)
}

assert_eq!(halfway(0.0_f64, 10.0_f64), 5.0);
assert_eq!(halfway(Vector2D::zero(), Vector2D::new(10.0, 20.0)), Vector2D::new(5.0, 10.0));
assert_eq!(halfway(Color::black(), Color::white()).get_red(), 0.5);
```

## Vector

抽象 2D / 3D 向量共有操作的 trait，用于书写与维度无关的泛型算法（例如物理里的 `integrate_linear`）。`Vector2D` 与 `Vector3D` 都实现了它，并把方法转发到各自的固有实现。

```rust
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Vector:
    Copy
    + Clone
    + Default
    + Debug
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<f64, Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign<f64>
{
    fn zero() -> Self;
    fn dot(&self, other: Self) -> f64;
    fn magnitude(&self) -> f64;
    fn magnitude_squared(&self) -> f64;
    fn normalized(&self) -> Self;
    fn scaled(&self, scalar: f64) -> Self;
    fn lerp(&self, other: Self, factor: f64) -> Self;
}
```

超 trait 里带上算术运算符，是为了让泛型代码可以直接写 `a + b`、`v * 2.0`、`-v`、`v += other`、`v *= scalar`，不必再额外导入这些 bound。

| 方法 | 说明 |
| --- | --- |
| `zero` | 返回该维度的零向量。 |
| `dot` | 点积。 |
| `magnitude` | 向量长度。 |
| `magnitude_squared` | 长度平方，比较用免开方。 |
| `normalized` | 单位长度副本。 |
| `scaled` | 标量缩放副本。 |
| `lerp` | 线性插值。 |

维度专属的方法**不在** trait 里（返回值类型不同或只存在于单一维度）：`Vector2D` 独有 `perp`、`cross`（返回 `f64`）、`from_angle`、`angle`、`angle_to`、`rotated`、`rotate`、`distance_to`、`distance_squared_to`、`direction_to`、`scale`、`normalize`；`Vector3D` 独有 `cross`（返回 `Vector3D`）、`rotated_by`、`direction_to`、`distance_to`、`scale`、`normalize`。

```rust
use euv_engine::*;

/// 与维度无关：同一个函数处理 2D 和 3D。
fn blend_towards_zero<V: Vector>(v: V) -> V {
    v.lerp(V::zero(), 0.5)
}

let in_2d = blend_towards_zero(Vector2D::new(10.0, 20.0));
let in_3d = blend_towards_zero(Vector3D::new(10.0, 20.0, 30.0));

assert_eq!(in_2d, Vector2D::new(5.0, 10.0));
assert_eq!(in_3d, Vector3D::new(5.0, 10.0, 15.0));

// 泛型上下文里也能直接用运算符。
fn scaled_sum<V: Vector>(a: V, b: V) -> V {
    (a + b) * 0.5
}
assert_eq!(scaled_sum(Vector2D::new(1.0, 1.0), Vector2D::new(3.0, 3.0)), Vector2D::new(2.0, 2.0));
```

## 内部常量

物理与渲染默认值定义在模块内的 `pub(crate)` 常量中（`PI`、`EPSILON`、`DEFAULT_GRAVITY`、`DEFAULT_CAMERA_NEAR` 等），**不属于公开 API**。外部代码需要常量时自行使用 `std::f64::consts`：

```rust
use std::f64::consts;

let _: f64 = consts::PI;
let _: f64 = consts::TAU;
let _: f64 = consts::FRAC_PI_2;
```