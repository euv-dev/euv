---
title: 物理与碰撞
order: 6
---

`euv_engine` 的物理与碰撞模块，提供 2D / 3D 两套刚体世界，以及可直接复用的几何碰撞体。全部类型都通过 `pub use` 从 crate 根直接导出，无需 `use euv_engine::physics::...`。

物理模块负责**积分与求解**（重力、阻尼、摩擦、位置修正），碰撞体模块负责**几何求交**。两者互相独立：你可以只用碰撞体做点选、拾取、命中检测，而不引入任何物理世界。

## 概览

| 分类 | 内容 |
| --- | --- |
| 2D 世界 | [`PhysicsConfig`](#physicsconfig)、[`PhysicsWorld2D`](#physicsworld2d)、[`RigidBody2D`](#rigidbody2d) |
| 3D 世界 | [`PhysicsConfig3D`](#physicsconfig3d)、[`PhysicsWorld3D`](#physicsworld3d)、[`RigidBody3D`](#rigidbody3d) |
| 碰撞体 | [`Collider`](#collider)、[`Collider3D`](#collider3d) |
| 2D 碰撞体 | [`AabbCollider`](#aabbcollider)、[`CircleCollider`](#circlecollider)、[`CollisionResult`](#collisionresult) |
| 3D 碰撞体 | [`AabbCollider3D`](#aabbcollider3d)、[`SphereCollider3D`](#spherecollider3d)、[`CollisionResult3D`](#collisionresult3d) |
| 枚举 | [`BodyType`](#bodytype)、[`BodyCollider`](#bodycollider)、[`BodyCollider3D`](#bodycollider3d)、[`ColliderShape`](#collidershape)、[`ColliderShape3D`](#collidershape3d) |

2D 与 3D 的 API **刻意对称**：方法同名、语义一致，只有力矩（`apply_torque`）和角速度的表示不同（2D 是 `f64` 弧度/秒，3D 是 `Vector3D` 轴×速率）。

## 可见性约定

这两个模块的 struct 都标注了 `#[derive(Clone, Data, Debug, New, PartialEq)]`，`Data` / `New` 来自 `lombok-macros`，按字段生成访问器。**可见性由字段上的属性逐个决定**，不是统一的：

- `get_*` —— 未标注可见性的字段（如 `RigidBody2D::get_id`、`get_position`、`get_velocity`、`get_mass`、`get_body_type`、`get_collider`）是 `pub`，可直接读取。
- `get_*` / `get_mut_*` / `set_*` 标注 `pub(crate)` 的字段（如 `get_restitution`、`get_friction`、`get_linear_damping`、`get_angular_damping`、`set_mass`、`set_inverse_mass`、`set_rotation`、`set_angular_velocity`）在 **crate 外不可见**。
- 完全没有 `#[set]` 属性的字段，其 `set_*` 反而是 `pub` —— 例如 `set_friction`、`set_body_type`、`set_id`、`set_position`、`set_velocity` 在外部可调用，并返回 `&mut Self` 以便链式书写。

所以调整质量请用公开方法 [`update_mass`](#rigidbody2d)，而不是 `set_mass`；调整转动惯性请用 `update_inertia`；调整阻尼与恢复系数则没有公开入口，只能在构造时通过 `PhysicsConfig::new` / `RigidBody2D::new` 传入。这不是疏漏，而是当前可见性标注的结果，改动前请先用 `cargo build` 验证。

## PhysicsConfig

物理世界的配置，由 `PhysicsConfig::new(gravity, linear_damping, angular_damping)` 按字段顺序构造。

| 访问器 | 可见性 | 说明 |
| --- | --- | --- |
| `get_gravity() -> Vector2D` | 公开 | 重力加速度，单位 **像素/秒²**。 |
| `get_linear_damping() -> f64` | `pub(crate)` | 线性速度阻尼系数。 |
| `get_angular_damping() -> f64` | `pub(crate)` | 角速度阻尼系数。 |

`PhysicsConfig::default()` 的取值为 `Vector2D::new(0.0, 980.0)`，两个阻尼均为 `0.0` —— 2D 重力沿 **+y 向下**（屏幕坐标系），且默认**无阻尼**。

```rust
use euv_engine::*;

let config = PhysicsConfig::new(Vector2D::new(0.0, 980.0), 0.0, 0.0);
assert_eq!(config.get_gravity().get_y(), 980.0);
```

## RigidBody2D

2D 刚体。`RigidBody2D::new_dynamic(id, position)` 建动态体（质量 `1.0`），`RigidBody2D::new_static(id, position)` 建静态体（质量 `0.0`，逆质量 `0.0`）。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `new_dynamic` | `(id: u64, position: Vector2D) -> RigidBody2D` | 动态刚体，默认质量 `1.0`，恢复系数 `0.3`、摩擦 `0.7`。 |
| `new_static` | `(id: u64, position: Vector2D) -> RigidBody2D` | 静态刚体，质量与逆质量都是 `0.0`，即无限质量。 |
| `apply_force` | `(&mut self, force: Vector2D)` | **累加**到力累加器，作用于**下一次** `step`。 |
| `apply_impulse` | `(&mut self, impulse: Vector2D)` | 瞬时冲量，立刻改速度（`Δv = impulse × 逆质量`）。逆质量为 `0` 时**直接忽略**。 |
| `update_mass` | `(&mut self, mass: f64)` | 设置质量并同步逆质量。`mass <= 0.0` 时逆质量置 `0.0`，即不可推动。 |
| `is_dynamic` | `(&self) -> bool` | 是否为 `BodyType::Dynamic`。 |
| `update_collider` | `(&mut self, collider: BodyCollider)` | 挂载碰撞体。 |
| `bounding_box` | `(&self) -> Option<Rect>` | 碰撞体的**世界坐标**包围盒；没挂碰撞体返回 `None`。 |

`apply_force` 与 `apply_impulse` 的区别是本页最容易踩的坑：**力要等到 `step` 才生效，且会被清空**；冲量立刻生效，但会被质量缩放。

```rust
use euv_engine::*;

// 力：先攒着，step 时被消耗掉。
let mut body = RigidBody2D::new_dynamic(1, Vector2D::zero());
body.apply_force(Vector2D::new(10.0, 0.0));
assert_eq!(body.get_force_accumulator().get_x(), 10.0);

let mut world = PhysicsWorld2D::default();
world.add_body(body);
world.step(1.0);
assert_eq!(world.get_body(1).expect("still registered").get_position().get_x() > 0.0, true);

// 冲量：立刻改变速度。
let mut body = RigidBody2D::new_dynamic(2, Vector2D::zero());
body.apply_impulse(Vector2D::new(5.0, 0.0));
assert!(body.get_velocity().get_x() > 0.0);

// 静态体（逆质量为 0）对冲量免疫。
let mut ground = RigidBody2D::new_static(3, Vector2D::zero());
let before = ground.get_velocity();
ground.apply_impulse(Vector2D::new(100.0, 0.0));
assert_eq!(ground.get_velocity(), before);
```

`bounding_box` 把碰撞体从局部坐标搬到世界坐标：碰撞体的中心加上刚体位置。AABB 保持自身尺寸，圆形则取直径为边长的正方形包围盒。

```rust
use euv_engine::*;

let mut body = RigidBody2D::new_dynamic(1, Vector2D::new(100.0, 200.0));
assert!(body.bounding_box().is_none());   // 没挂碰撞体

body.update_collider(BodyCollider::Circle(CircleCollider::from_center(
    Vector2D::zero(),
    5.0,
)));
let bounds = body.bounding_box().expect("a collider yields a box");
assert_eq!(bounds.center(), Vector2D::new(100.0, 200.0));
assert_eq!(bounds.get_width(), 10.0);     // 直径
assert_eq!(bounds.get_height(), 10.0);
```

::: note 2D 与 3D 的 `bounding_box` 偏移方式不同
`RigidBody2D` 给圆形碰撞体生成的包围盒**直接以刚体位置为中心**（不加碰撞体自身中心），而 `RigidBody3D` 会**加上**碰撞体内部中心（`position + sphere.get_center()`）。若你在两个维度间搬运写法，注意这个差别。
:::

## PhysicsWorld2D

管理全部刚体并驱动仿真步进。`PhysicsWorld2D::with_config(config)` 用给定配置建世界（并初始化空间哈希网格），`PhysicsWorld2D::default()` 等价于 `with_config(PhysicsConfig::default())`。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `with_config` | `(config: PhysicsConfig) -> PhysicsWorld2D` | 建世界并初始化空间哈希网格。 |
| `add_body` | `(&mut self, body: RigidBody2D)` | 追加刚体，按插入顺序存放。 |
| `remove_body` | `(&mut self, id: u64)` | 移除**第一个**匹配 id 的刚体；id 不存在则无操作。 |
| `get_body` | `(&self, id: u64) -> Option<&RigidBody2D>` | 按 id 查找。 |
| `get_body_mut` | `(&mut self, id: u64) -> Option<&mut RigidBody2D>` | 按 id 取可变引用，用于原地改状态。 |
| `get_bodies` | `(&self) -> &Vec<RigidBody2D>` | 全部刚体（只读）。 |
| `step` | `(&mut self, delta_time: f64)` | 推进一帧仿真，`delta_time` 单位为**秒**。 |

```rust
use euv_engine::*;

let mut world = PhysicsWorld2D::default();
world.add_body(RigidBody2D::new_dynamic(1, Vector2D::zero()));
world.add_body(RigidBody2D::new_static(2, Vector2D::new(0.0, 100.0)));

// 按 id 查；查不到返回 None。
assert!(world.get_body(1).is_some());
assert!(world.get_body(99).is_none());

// 需要原地改状态时走 get_body_mut。
world.get_body_mut(1).expect("body 1").set_velocity(Vector2D::new(2.0, 0.0));
assert_eq!(world.get_body(1).expect("body 1").get_velocity().get_x(), 2.0);

// 移除不存在的 id 是无害的。
world.remove_body(999);
assert!(world.get_body(1).is_some());
```

`PhysicsWorld2D` 还实现了 `Updatable` trait，因此可以直接进入引擎的统一更新循环，与实体、动画器一起推进：

```rust
use euv_engine::*;

fn advance(world: &mut dyn Updatable, dt: f64) {
    world.update(dt);
}
```

### step 的执行顺序

`step(delta_time)` 用**半隐式欧拉**积分，只对 `is_dynamic()` 的刚体生效：

1. 把重力累加进力累加器（`gravity × mass`）。
2. 用累加力更新速度：`Δv = force × inverse_mass × dt`。
3. 施加线性阻尼（`1 - damping × dt`，下限 `0.0`）。
4. 用速度更新位置：`position += velocity × dt`。
5. 清空力累加器与力矩累加器。
6. 角速度乘角阻尼，角速度积分进 `rotation`。
7. 调用内部的 `resolve_collisions` 做碰撞检测与求解。

静态体与 `BodyType::Kinematic` **在整个积分阶段被跳过**，只有碰撞阶段仍会参与。阻尼因子在循环外预先算好，避免每帧重复计算。

### 碰撞求解

碰撞求解分两个阶段：

- **粗筛（broad phase）** —— 把每个刚体的包围盒插入空间哈希网格，一次性收集候选对。网格与候选对列表在**每个 `step` 只重建一次**，之后所有求解迭代复用，避免重复查询与重复分配。这一步把复杂度从 O(n²) 降到接近 O(n)。
- **窄相 + 求解（narrow phase）** —— 按形状组合分派到对应的 `collide_with_*`，再用冲量响应、库仑摩擦和位置修正求解。

求解最多迭代 `PHYSICS_MAX_ITERATIONS`（`8`）轮；某一轮完全没有碰撞就提前退出。位置修正每次只推 `PHYSICS_POSITION_PERCENT`（`0.8`）的穿透深度，避免过度修正。

两侧逆质量都是 `0` 的刚体对（两个静态体）会被直接跳过。两个逆质量之和为 `0` 时求解函数直接返回。

**摩擦独立于法向冲量求解。** 这点很关键：一个物体贴着地面滑动时，它沿法线方向通常是在**分离**的，因此法向冲量为 `0`；若摩擦依赖于法向冲量，摩擦就会被整个跳过，物体将永远滑下去。实现改用**穿透深度驱动的修正冲量**作为法向载荷（再加上法向冲量的绝对值）来钳制库仑摩擦锥，从而让静止的物体最终停下来而不是持续抖动。

```rust
use euv_engine*;

// 零重力、零阻尼下，一个圆在地面上滑动，摩擦会把切向速度逐渐吃掉。
let config = PhysicsConfig::new(Vector2D::zero(), 0.0, 0.0);
let mut world = PhysicsWorld2D::with_config(config);

let mut ground = RigidBody2D::new_static(1, Vector2D::new(0.0, -5.0));
ground.update_collider(BodyCollider::Aabb(AabbCollider::new(Rect::new(
    -50.0, 0.0, 100.0, 100.0,
))));
world.add_body(ground);

let mut slider = RigidBody2D::new_dynamic(2, Vector2D::new(0.0, 0.19));
slider.update_collider(BodyCollider::Circle(CircleCollider::from_center(
    Vector2D::zero(),
    0.2,
)));
world.add_body(slider);
world.get_body_mut(2).expect("slider").set_velocity(Vector2D::new(2.0, 0.0));

for _ in 0..900 {
    world.step(1.0 / 60.0);
}
let remaining = world.get_body(2).expect("slider").get_velocity().get_x();
assert!(remaining < 0.5, "friction should bleed off the slide, got {remaining}");

// 摩擦为 0 时同样滑 900 帧，速度几乎不变。
let config = PhysicsConfig::new(Vector2D::zero(), 0.0, 0.0);
let mut world = PhysicsWorld2D::with_config(config);
let mut ground = RigidBody2D::new_static(1, Vector2D::new(0.0, -5.0));
ground.update_collider(BodyCollider::Aabb(AabbCollider::new(Rect::new(
    -50.0, 0.0, 100.0, 100.0,
))));
world.add_body(ground);
let mut slider = RigidBody2D::new_dynamic(2, Vector2D::new(0.0, 0.19));
slider.update_collider(BodyCollider::Circle(CircleCollider::from_center(
    Vector2D::zero(),
    0.2,
)));
slider.set_friction(0.0);          // 无摩擦
world.add_body(slider);
world.get_body_mut(2).expect("slider").set_velocity(Vector2D::new(2.0, 0.0));

for _ in 0..900 {
    world.step(1.0 / 60.0);
}
assert!(world.get_body(2).expect("slider").get_velocity().get_x() > 1.0);
```

::: warning 2D 世界没有力矩接口
`RigidBody2D` 有 `angular_velocity` 与 `rotation`，也受角阻尼影响，但**没有 `apply_torque`**。2D 只能通过 `set_angular_velocity` 直接设角速度。需要累积力矩请用 3D 的 [`RigidBody3D::apply_torque`](#rigidbody3d)。
:::

## RigidBody3D

3D 刚体，构造方式与 2D 完全对称。`RigidBody3D::new_dynamic` 额外会按质量调用 `update_inertia(mass)`，让转动惯性与质量保持一致。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `new_dynamic` | `(id: u64, position: Vector3D) -> RigidBody3D` | 动态刚体，默认质量 `1.0`，并按质量设置惯量。 |
| `new_static` | `(id: u64, position: Vector3D) -> RigidBody3D` | 静态刚体，质量、逆质量、惯量全为 `0.0`。 |
| `apply_force` | `(&mut self, force: Vector3D)` | 累加到力累加器。 |
| `apply_torque` | `(&mut self, torque: Vector3D)` | 累加到力矩累加器，作用于**下一次** `step`。 |
| `apply_impulse` | `(&mut self, impulse: Vector3D)` | 瞬时冲量，立刻改速度；逆质量为 `0` 时忽略。 |
| `update_mass` | `(&mut self, mass: f64)` | 设置质量并同步逆质量。 |
| `update_inertia` | `(&mut self, inertia: f64)` | 设置惯量的**倒数**（`update_inertia` 内部只写 `inverse_inertia`，不写 `inertia`）。`inertia <= 0.0` 时不可转动。 |
| `is_dynamic` | `(&self) -> bool` | 是否为 `BodyType::Dynamic`。 |
| `update_collider` | `(&mut self, collider: BodyCollider3D)` | 挂载 3D 碰撞体。 |
| `bounding_box` | `(&self) -> Option<AABB3D>` | 世界坐标包围盒；无碰撞体返回 `None`。 |

::: note `update_inertia` 只更新倒数
`update_inertia(4.0)` 把 `inverse_inertia` 设为 `0.25`，但**不会**把 `inertia` 字段本身写成 `4.0`。需要"不可转动"时传 `0.0` 即可。
:::

3D 的 `step` 在 2D 流程之外多做一步：角速度积分进**四元数**旋转，用半步四元数增量左乘后归一化。

```rust
use euv_engine::*;

let mut body = RigidBody3D::new_dynamic(1, Vector3D::zero());
let mut ground = RigidBody3D::new_static(2, Vector3D::zero());
assert!(body.is_dynamic());
assert!(!ground.is_dynamic());

// 力矩先累积，角速度等到 step 才变；角速度沿力矩所在的 z 轴增长。
let before = body.get_angular_velocity().get_z();
body.apply_torque(Vector3D::new(0.0, 0.0, 4.0));
assert_eq!(body.get_torque_accumulator().get_z(), 4.0);
assert_eq!(body.get_angular_velocity().get_z(), before);

let mut world = PhysicsWorld3D::default();
world.add_body(body);
world.step(1.0 / 60.0);
let rotated = world.get_body(1).expect("body 1");
assert_ne!(rotated.get_angular_velocity().get_z(), before);

// 零惯量让刚体无法绕任何轴转动。
let mut frozen = RigidBody3D::new_dynamic(3, Vector3D::zero());
frozen.update_inertia(0.0);
assert_eq!(frozen.get_inverse_inertia(), 0.0);
```

## PhysicsConfig3D / PhysicsWorld3D

`PhysicsConfig3D::new(gravity, linear_damping, angular_damping)` 与 2D 同构，区别在单位：重力是**米/秒²**，`Vector3D::new(0.0, -9.81, 0.0)`（沿 **-y 向下**，右手坐标系）。阻尼项的 getter 同样是 `pub(crate)`。

`PhysicsWorld3D` 提供与 2D 同名的一整套方法：`with_config`、`add_body`、`remove_body`、`get_body`、`get_body_mut`、`get_bodies`、`step`，并同样实现了 `Updatable`。

```rust
use euv_engine::*;

let mut world = PhysicsWorld3D::default();
world.add_body(RigidBody3D::new_dynamic(1, Vector3D::zero()));
world.add_body(RigidBody3D::new_dynamic(2, Vector3D::new(5.0, 0.0, 0.0)));

for body in world.get_bodies() {
    let _: Vector3D = body.get_position();
}

// 默认重力沿 -y，x 不变；两个刚体自由落体。
world.step(1.0 / 60.0);
assert_eq!(world.get_body(2).expect("body 2").get_position().get_x(), 5.0);
assert!(world.get_body(2).expect("body 2").get_position().get_y() < 0.0);
```

3D 的碰撞求解同样走"空间哈希粗筛 + 窄相 + 冲量求解"，迭代上限与位置修正比例与 2D 一致。

## BodyType

刚体参与仿真的方式。

| 变体 | 行为 |
| --- | --- |
| `Static` | 默认值。不受力与碰撞影响，相当于墙壁与地面。 |
| `Dynamic` | 完全由力、重力与碰撞驱动。 |
| `Kinematic` | 由代码驱动位移，但**不受力或重力影响**。 |

`Static` 是 `Default`。`Kinematic` 类型存在于枚举中，但当前 `step` 的积分阶段只推进 `Dynamic`，因此 kinematic 刚体不会被自动积分。

```rust
use euv_engine::*;

assert_eq!(BodyType::default(), BodyType::Static);
assert_eq!(RigidBody2D::new_dynamic(1, Vector2D::zero()).get_body_type(), BodyType::Dynamic);
assert_eq!(RigidBody2D::new_static(2, Vector2D::zero()).get_body_type(), BodyType::Static);
```

## Collider

2D 碰撞体可参与碰撞检测的 trait。已由 [`AabbCollider`](#aabbcollider) 与 [`CircleCollider`](#circlecollider) 实现。

```rust
pub trait Collider {
    fn shape(&self) -> ColliderShape;
    fn bounding_box(&self) -> Rect;
    fn contains_point(&self, point: Vector2D) -> bool;
    fn center(&self) -> Vector2D;
}
```

| 方法 | 返回 | 说明 |
| --- | --- | --- |
| `shape` | `ColliderShape` | 形状变体，供 `match` 分派。 |
| `bounding_box` | `Rect` | 包围该碰撞体的轴对齐矩形。 |
| `contains_point` | `bool` | 点是否在内部。 |
| `center` | `Vector2D` | 几何中心。 |

## Collider3D

3D 版本，除类型不同外语义一致，由 [`AabbCollider3D`](#aabbcollider3d) 与 [`SphereCollider3D`](#spherecollider3d) 实现。

```rust
pub trait Collider3D {
    fn shape(&self) -> ColliderShape3D;
    fn bounding_box(&self) -> AABB3D;
    fn contains_point(&self, point: Vector3D) -> bool;
    fn center(&self) -> Vector3D;
}
```

::: tip 自己实现碰撞体
两个 trait 都只有四个只读方法，很容易为自定义形状（多边形、胶囊体）实现 `Collider` / `Collider3D`，从而接入 `shape()` 的 `match` 分派。注意 `contains_point` 的**边界语义要自己定义** —— 内置的 `Rect` / `Circle` / `AABB3D` / `Sphere` 都把边界算作在内。
:::

## CollisionResult / CollisionResult3D

碰撞结果，仅在**重叠**时返回，`Option` 为 `None` 表示没有碰撞。

`CollisionResult` 字段（均通过 `get_*` 读取）：

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `normal` | `Vector2D` | 最小平移向量（MTV），由**第一个**碰撞体指向第二个。 |
| `depth` | `f64` | 沿法线方向的穿透深度。 |
| `contact_point` | `Vector2D` | 接触点。 |

`CollisionResult3D` 结构相同，`Vector2D` 换成 `Vector3D`。两者都可由 `CollisionResult::new(normal, depth, contact_point)` 构造，但 `set_normal` 是 `pub(crate)`，外部只能读。

**法线的方向约定很重要**：它指向"把第二个碰撞体推开"的方向。`AabbCollider::from_center((0,0), 4, 4)` 与 `from_center((2,0), 4, 4)` 相撞时，第一个（靠左）得到 `(-1, 0)` 的法线 —— 即被推向 -x。

```rust
use euv_engine::*;

let left = AabbCollider::from_center(Vector2D::zero(), 4.0, 4.0);
let right = AabbCollider::from_center(Vector2D::new(2.0, 0.0), 4.0, 4.0);

let hit = left.collide_with_aabb(&right).expect("the boxes overlap");
assert!((hit.get_depth() - 2.0).abs() < 1e-9, "two units of overlap");
assert_eq!(hit.get_normal(), Vector2D::new(-1.0, 0.0));
```

## AabbCollider

2D 轴对齐包围盒碰撞体，内部包装一个 `Rect`。`AabbCollider::from_center(center, width, height)` 构造。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `from_center` | `(center: Vector2D, width: f64, height: f64) -> AabbCollider` | 由中心点与尺寸构造。 |
| `collide_with_aabb` | `(&self, other: &AabbCollider) -> Option<CollisionResult>` | 与另一个 AABB 求交。 |
| `collide_with_circle` | `(&self, circle: &CircleCollider) -> Option<CollisionResult>` | 与圆形求交。 |

两个 AABB 相交时，按**重叠较小的轴**选法线方向；接触点是两个矩形重叠区域的那个角。任一轴的重叠量 `<= 1e-4`（`COLLIDER_CONTACT_EPSILON`）就判定为**不相交** —— 也就是说边缘恰好相切**不算碰撞**。

```rust
use euv_engine::*;

let left = AabbCollider::from_center(Vector2D::zero(), 10.0, 10.0);
let right = AabbCollider::from_center(Vector2D::new(8.0, 0.0), 10.0, 10.0);

// 只差 0.1 单位的重叠仍然算碰撞。
assert!(left.collide_with_aabb(&right).is_some());

// 相差 100 则完全分离。
let far = AabbCollider::from_center(Vector2D::new(100.0, 0.0), 10.0, 10.0);
assert!(left.collide_with_aabb(&far).is_none());

// 交换参数顺序不影响是否碰撞与深度。
let a = AabbCollider::from_center(Vector2D::zero(), 10.0, 10.0);
let b = AabbCollider::from_center(Vector2D::new(6.0, 0.0), 10.0, 10.0);
let forward = a.collide_with_aabb(&b).expect("overlapping");
let backward = b.collide_with_aabb(&a).expect("overlapping");
assert!((forward.get_depth() - backward.get_depth()).abs() < 1e-9);
```

AABB 与圆形求交时，把圆心**钳制**到矩形范围内得到最近点，再判断距离；圆心正好在矩形内部（距离为 `0`）会退化为从矩形中心指向圆心的方向，两者完全重合时用 `Vector2D::up()` 兜底。

```rust
use euv_engine::*;

let box_shape = AabbCollider::from_center(Vector2D::zero(), 20.0, 20.0);
let circle = CircleCollider::from_center(Vector2D::new(8.0, 0.0), 2.0);
assert!(box_shape.collide_with_circle(&circle).is_some());

let distant = CircleCollider::from_center(Vector2D::new(40.0, 0.0), 2.0);
assert!(box_shape.collide_with_circle(&distant).is_none());
```

## CircleCollider

2D 圆形碰撞体，内部包装一个 `Circle`。`CircleCollider::from_center(center, radius)` 构造。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `from_center` | `(center: Vector2D, radius: f64) -> CircleCollider` | 由圆心与半径构造。 |
| `collide_with_circle` | `(&self, other: &CircleCollider) -> Option<CollisionResult>` | 与另一个圆求交。 |

两圆求交用距离与半径和比较，`>=` 判定为分离 —— 恰好相切**不算碰撞**。两圆同心时距离为 `0`，法线退化为 `Vector2D::right()`，接触点落在自身半径处。

```rust
use euv_engine::*;

let a = CircleCollider::from_center(Vector2D::zero(), 1.0);
let b = CircleCollider::from_center(Vector2D::new(1.5, 0.0), 1.0);

let hit = a.collide_with_circle(&b).expect("the circles overlap");
assert!((hit.get_depth() - 0.5).abs() < 1e-9);
assert_eq!(hit.get_normal(), Vector2D::right());

// 同心圆：深度是半径和，法线兜底为 +x。
let concentric = a.collide_with_circle(&a).expect("identical circles");
assert_eq!(concentric.get_normal(), Vector2D::right());
assert!((concentric.get_depth() - 2.0).abs() < 1e-9);

// 恰好相切不算碰撞。
let tangent = CircleCollider::from_center(Vector2D::new(2.0, 0.0), 1.0);
assert!(a.collide_with_circle(&tangent).is_none());

// 半径为 0 的退化圆仍可参与查询——只要它落在对方内部。
let point = CircleCollider::from_center(Vector2D::zero(), 0.0);
assert!(point.collide_with_circle(&a).is_some());   // 与同心、r=1 的圆相交
assert!(point.collide_with_circle(&b).is_none());   // 离圆心 1.5，远在半径和之外
```

`bounding_box` 对圆形返回**直径为边长的正方形**，而不是紧致包围盒 —— 这是空间哈希网格的粗筛依据，用紧致盒会漏掉对角方向的接触。

## AabbCollider3D

3D 轴对齐包围盒碰撞体，内部包装一个 `AABB3D`。`AabbCollider3D::from_center(center, width, height, depth)` 构造。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `from_center` | `(center: Vector3D, width, height, depth: f64) -> AabbCollider3D` | 由中心点与三轴尺寸构造。 |
| `collide_with_aabb` | `(&self, other: &AabbCollider3D) -> Option<CollisionResult3D>` | 与另一个 3D AABB 求交。 |
| `collide_with_sphere` | `(&self, sphere: &SphereCollider3D) -> Option<CollisionResult3D>` | 与球体求交。 |

三个轴**任意一个**重叠量 `<= 1e-4` 即判定分离，因此只在 z 轴上分开的两个盒子不会碰撞。重叠最小的轴决定法线方向。

```rust
use euv_engine::*;

let left = AabbCollider3D::from_center(Vector3D::zero(), 10.0, 10.0, 10.0);
let right = AabbCollider3D::from_center(Vector3D::new(5.0, 0.0, 0.0), 10.0, 10.0, 10.0);
let hit = left.collide_with_aabb(&right).expect("overlapping");
assert!(hit.get_depth() > 0.0);

let above = AabbCollider3D::from_center(Vector3D::new(0.0, 0.0, 50.0), 10.0, 10.0, 10.0);
assert!(left.collide_with_aabb(&above).is_none());

let box_shape = AabbCollider3D::from_center(Vector3D::zero(), 20.0, 20.0, 20.0);
let sphere = SphereCollider3D::from_center(Vector3D::new(9.0, 0.0, 0.0), 2.0);
assert!(box_shape.collide_with_sphere(&sphere).is_some());
```

## SphereCollider3D

3D 球体碰撞体，内部包装一个 `Sphere`。`SphereCollider3D::from_center(center, radius)` 构造。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `from_center` | `(center: Vector3D, radius: f64) -> SphereCollider3D` | 由球心与半径构造。 |
| `collide_with_sphere` | `(&self, other: &SphereCollider3D) -> Option<CollisionResult3D>` | 与另一个球体求交。 |

语义与 2D 的 `CircleCollider` 一致：同心时法线退化为 `Vector3D::right()`，恰好相切不算碰撞。

```rust
use euv_engine::*;

let a = SphereCollider3D::from_center(Vector3D::zero(), 1.0);
let b = SphereCollider3D::from_center(Vector3D::new(1.5, 0.0, 0.0), 1.0);
let hit = a.collide_with_sphere(&b).expect("overlapping");
assert!((hit.get_depth() - 0.5).abs() < 1e-9);

let far = SphereCollider3D::from_center(Vector3D::new(50.0, 0.0, 0.0), 3.0);
assert!(a.collide_with_sphere(&far).is_none());
```

## AABB3D::broad_phase

```rust
pub fn broad_phase(a: AABB3D, b: AABB3D) -> bool
```

粗筛用的关联函数，等价于 `a.intersects(b)`。物理世界内部用它先排除明显不重叠的刚体对，再进入昂贵的窄相。直接构造两个 `AABB3D` 即可调用：

```rust
use euv_engine::*;

let a = AABB3D::new(Vector3D::zero(), Vector3D::new(10.0, 10.0, 10.0));
let overlapping = AABB3D::new(
    Vector3D::new(5.0, 0.0, 0.0),
    Vector3D::new(15.0, 10.0, 10.0),
);
let separated = AABB3D::new(
    Vector3D::new(100.0, 0.0, 0.0),
    Vector3D::new(110.0, 10.0, 10.0),
);

assert!(AABB3D::broad_phase(a, overlapping));
assert!(!AABB3D::broad_phase(a, separated));
```

2D 的对应物是数学库里的 `Rect::broad_phase_alias(a, b)`。

## BodyCollider / BodyCollider3D

包住碰撞体数据的枚举，挂在刚体上。

| 变体 | 内容 |
| --- | --- |
| `BodyCollider::Aabb` | `AabbCollider`。 |
| `BodyCollider::Circle` | `CircleCollider`。 |
| `BodyCollider3D::Aabb` | `AabbCollider3D`。 |
| `BodyCollider3D::Sphere` | `SphereCollider3D`。 |

两者的 `Default` 都是 AABB 变体。用 `update_collider` 挂载：

```rust
use euv_engine::*;

let mut body = RigidBody2D::new_dynamic(1, Vector2D::zero());
body.update_collider(BodyCollider::Circle(CircleCollider::from_center(
    Vector2D::zero(),
    8.0,
)));
assert!(matches!(
    body.get_collider(),
    Some(BodyCollider::Circle(_))
));

let mut body3 = RigidBody3D::new_dynamic(2, Vector3D::zero());
body3.update_collider(BodyCollider3D::Sphere(SphereCollider3D::from_center(
    Vector3D::zero(),
    1.0,
)));
assert!(matches!(
    body3.get_collider(),
    Some(BodyCollider3D::Sphere(_))
));
```

## ColliderShape / ColliderShape3D

碰撞体的形状标记，用于 `match` 分派。

| 枚举 | 变体 | 默认 |
| --- | --- | --- |
| `ColliderShape` | `Aabb`、`Circle` | `Aabb` |
| `ColliderShape3D` | `Aabb`、`Sphere` | `Aabb` |

```rust
use euv_engine::*;

fn label(shape: ColliderShape) -> &'static str {
    match shape {
        ColliderShape::Aabb => "aabb",
        ColliderShape::Circle => "circle",
    }
}
assert_eq!(label(AabbCollider::from_center(Vector2D::zero(), 1.0, 1.0).shape()), "aabb");
assert_eq!(label(CircleCollider::from_center(Vector2D::zero(), 1.0).shape()), "circle");

fn label3d(shape: ColliderShape3D) -> &'static str {
    match shape {
        ColliderShape3D::Aabb => "aabb",
        ColliderShape3D::Sphere => "sphere",
    }
}
assert_eq!(
    label3d(SphereCollider3D::from_center(Vector3D::zero(), 1.0).shape()),
    "sphere"
);
```

## 完整示例：2D 落球

一个球掉到地面上并停下：

```rust
use euv_engine::*;

let mut world = PhysicsWorld2D::default();

// 地面：静态 AABB。
let mut ground = RigidBody2D::new_static(1, Vector2D::zero());
ground.update_collider(BodyCollider::Aabb(AabbCollider::from_center(
    Vector2D::zero(),
    400.0,
    20.0,
)));
world.add_body(ground);

// 球：动态圆形，初始位于地面上方的 100 像素。
let mut ball = RigidBody2D::new_dynamic(2, Vector2D::new(0.0, -100.0));
ball.update_collider(BodyCollider::Circle(CircleCollider::from_center(
    Vector2D::zero(),
    10.0,
)));
world.add_body(ball);

// 固定步长跑 120 帧（2 秒）。
for _ in 0..120 {
    world.step(1.0 / 60.0);
}

let resting = world.get_body(2).expect("the ball");
let position = resting.get_position();

// 球停在地面之上，且已经不再继续下落。
assert!(position.get_y() > -60.0, "the ball fell, got y={}", position.get_y());
assert!(resting.get_velocity().get_y() < 100.0, "it has slowed down a lot");
```

## 完整示例：3D 球落地

```rust
use euv_engine::*;

let mut world = PhysicsWorld3D::default();

let mut floor = RigidBody3D::new_static(1, Vector3D::zero());
floor.update_collider(BodyCollider3D::Aabb(AabbCollider3D::from_center(
    Vector3D::zero(),
    100.0,
    1.0,
    100.0,
)));
world.add_body(floor);

let mut ball = RigidBody3D::new_dynamic(2, Vector3D::new(0.0, 5.0, 0.0));
ball.update_collider(BodyCollider3D::Sphere(SphereCollider3D::from_center(
    Vector3D::zero(),
    0.5,
)));
world.add_body(ball);

for _ in 0..180 {
    world.step(1.0 / 60.0);
}

let resting = world.get_body(2).expect("the ball");
// 3D 重力是 -9.81 m/s²，3 秒后应该已经落地并停在地板顶面之上。
assert!(resting.get_position().get_y() < 5.0, "the ball fell, got {:?}", resting.get_position());
```

## 内部常量

物理与碰撞的调参常量定义在模块内的 `pub(crate)` 常量中，**不属于公开 API**：

| 常量 | 值 | 作用 |
| --- | --- | --- |
| `PHYSICS_DEFAULT_MASS` | `1.0` | 动态刚体默认质量（千克）。 |
| `PHYSICS_STATIC_MASS` | `0.0` | 静态刚体的质量，代表无限质量。 |
| `PHYSICS_MAX_ITERATIONS` | `8` | 每步的碰撞求解迭代上限，保证堆叠稳定。 |
| `PHYSICS_POSITION_PERCENT` | `0.8` | 每次迭代施加的位置修正比例。 |
| `COLLIDER_CONTACT_EPSILON` | `1e-4` | 判定"刚好相切"与"真正重叠"的容差。 |

它们与数学库里的 `DEFAULT_GRAVITY`（`980.0`）、`DEFAULT_GRAVITY_3D`（`-9.81`）、`DEFAULT_RESTITUTION`（`0.3`）、`DEFAULT_FRICTION`（`0.7`）、`EPSILON`（`1e-6`）一样，都是 crate 内部值。若你需要复现默认参数，自行写死这些数字：

```rust
use euv_engine::*;

// 复刻 PhysicsConfig::default() 的重力。
let mut config = PhysicsConfig::new(Vector2D::new(0.0, 980.0), 0.0, 0.0);
assert_eq!(config.get_gravity().get_y(), 980.0);
```
