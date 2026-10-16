---
title: 输入
order: 5
---

`euv_engine` 的输入模块，负责把键盘、鼠标、触摸与游戏手柄的原始事件折叠成"每个游戏帧读一次"的状态。模块内所有类型都通过 `pub use` 从 crate 根直接导出，无需 `use euv_engine::input::...`：

```rust
use euv_engine::*;
```

> **运行环境**：模块里的 DOM 事件监听（`Input::attach`、`GamepadManager::attach`、`GamepadManager::poll`）只有在**浏览器 wasm** 下才有意义 —— 它们注册的是 `window` / canvas 上的 DOM 监听器。非浏览器环境（如 native 单元测试）没有 `window`，`register_input()` 会返回 `None`。但状态查询本身（`InputState` 的 `is_*` 方法、`GamepadState` / `GamepadManager` 的全部查询、全部自由函数）都是纯逻辑，不触碰 DOM，可以在任何环境测试。

## 概览

| 分类 | 内容 |
| --- | --- |
| 命名空间 | [`Input`](#input) |
| 每帧状态 | [`InputState`](#inputstate)、[`InputStateCell`](#inputstatecell) |
| 手柄 | [`GamepadState`](#gampadstate)、[`GamepadManager`](#gamepadminager)、[`GamepadButtonSet`](#gampadbuttonset) |
| 枚举 | [`MouseButton`](#mousebutton)、[`InputAction`](#inputaction) |
| 自由函数 | [`apply_axis_deadzone`](#apply_axis_deadzone)、[`is_button_down`](#is_button_down)、[`compute_pressed_buttons`](#compute_pressed_buttons)、[`compute_released_buttons`](#compute_released_buttons)、[`compute_held_buttons`](#compute_held_buttons)、[`read_raw_axis`](#read_raw_axis) |

## 派生代码约定

`InputState` 标注了 `#[derive(Clone, Data, Debug, New, PartialEq)]`，`GamepadState` / `GamepadManager` 标注了 `#[derive(Clone, Data, Debug, Default, PartialEq)]`，其中 `Data` / `New` 来自 `lombok-macros`，按字段生成访问器：

- `Type::new(...)` —— 按字段声明顺序构造。
- `Type::get_field() -> T` —— 返回**值拷贝**（标注 `type(copy)`）或引用（未标注时）。
- `Type::get_mut_field()` / `Type::set_field(v)` —— 标注 `pub(crate)`，**crate 外不可见**。

> `InputState` 的全部字段都是 `#[new(skip)]`，因此 `InputState::new()` **不接收任何参数**，等价于 `InputState::default()`。`GamepadState` / `GamepadManager` 的字段没有 `skip`，但都由 `Default` 派生提供无参构造。

字段本身是 `pub(crate)`，但 `InputState` 上的 `get_*` 全部是公开的，可以直接读取三套集合：

```rust
use euv_engine::*;

let mut state: InputState = InputState::new();
state.press_key(String::from("KeyW"));

let pressed: &std::collections::HashSet<String> = state.get_keys_pressed();
let held: &std::collections::HashSet<String> = state.get_keys_held();
assert!(pressed.contains("KeyW") && held.contains("KeyW"));
```

`GamepadState` / `GamepadManager` 的可见性则更窄，只有 `get_id()`、`get_connected()`、`get_states()` 可从 crate 外调用，`get_axes()`、`get_button_values()`、`get_buttons_pressed()`、`get_buttons_held()`、`get_buttons_released()`、`get_disconnected()` 均为私有 —— 手柄的原始数据一律通过下面的**查询方法**读取，不要试图直接拿集合。

## 帧生命周期：每帧末的收尾

这是理解整个模块最重要的一条：**边沿（edge）只报告发生的那一帧**。

`EngineHandle::start` 的文档注释给出了权威表述：若 `register_input` 已挂上 DOM 监听，共享输入状态会在**每一帧结束时被收尾（close out）**，于是边沿触发的查询（`keys_pressed`、`keys_released`、`mouse_buttons_pressed`、`touch_started`）只报告它们真正发生的那一帧。

收尾动作由调度器在**渲染回调之后**执行（`scheduler/impl.rs` 的 `tick` 末尾）：

```
handler.get_mut().on_render(interpolation);
*self.get_mut_frame_count() += 1;
if let Some(cell) = input_cell {
    cell.get_mut().end_frame();
}
```

`InputState::end_frame` 清空 `keys_pressed`、`keys_released`、`mouse_buttons_pressed`、`mouse_buttons_released`、`touch_started`、`touch_ended`，并把 `mouse_delta` 归零、`mouse_moved` 置回 `false`；**`keys_held` 与 `mouse_buttons_held` 不清空**，仍然按住的东西会继续报"按住"。

`GamepadState::end_frame` 与 `GamepadManager::end_frame` 遵循同一套约定：清空 pressed / released，保留 held 与 `connected` 标志。

```rust
use euv_engine::*;

let mut state: InputState = InputState::new();
state.press_key(String::from("KeyW"));

// 边沿在这一帧有效
assert!(state.is_key_pressed("KeyW"));

state.end_frame();

// 边沿随帧消失，但键还在手上
assert!(state.get_keys_pressed().is_empty());
assert!(state.get_keys_held().contains("KeyW"));
```

这条约定有三条推论，全部有测试覆盖：

1. **长按不会每帧重发边沿**。按住不放时浏览器会持续派发 `keydown`（自动重复），但 `press_key` 只在该键**当前不在 held 集合里**时才写入 `keys_pressed`，所以重复的 `keydown` 不会堆积成"下一帧的第二条边沿"：

   ```rust
   use euv_engine::*;

   let mut input: InputState = InputState::default();
   input.press_key(String::from("KeyA"));
   for _ in 0..5 {
       input.end_frame();
       input.press_key(String::from("KeyA"));
   }
   assert!(!input.is_key_pressed("KeyA"));
   ```

2. **陈旧的边沿不会在下一帧重放**。连按两次调度器，第二次 `keys_pressed` 仍然是空的，而 `keys_held` 保留。
3. **鼠标位移是每帧量**。`mouse_delta` 由两次 `update_mouse_position` 的差值得出，`end_frame` 后归零 —— 想在整帧上累加位移，必须在自己的逻辑里把每帧的 delta 累加起来。

## InputState

一帧之内的全部键鼠触摸状态。事件处理器负责写入，游戏代码每帧读一次。

| `get_*` | 类型 | 说明 |
| --- | --- | --- |
| `get_keys_pressed` | `&HashSet<String>` | 本帧按下的键码 |
| `get_keys_held` | `&HashSet<String>` | 当前按住不松的键码 |
| `get_keys_released` | `&HashSet<String>` | 本帧松开的键码 |
| `get_mouse_buttons_pressed` | `&HashSet<MouseButton>` | 本帧按下的鼠标键 |
| `get_mouse_buttons_held` | `&HashSet<MouseButton>` | 当前按住的鼠标键 |
| `get_mouse_buttons_released` | `&HashSet<MouseButton>` | 本帧松开的鼠标键 |
| `get_mouse_position` | `Vector2D` | 鼠标当前位置（视口坐标） |
| `get_mouse_delta` | `Vector2D` | 相对上一帧的位移，每帧末归零 |
| `get_mouse_moved` | `bool` | 本帧是否发生过移动 |
| `get_touch_points` | `&HashMap<i32, Vector2D>` | 当前活跃触点，按标识索引 |
| `get_touch_started` | `&HashSet<i32>` | 本帧落下的触点 |
| `get_touch_ended` | `&HashSet<i32>` | 本帧离开的触点 |

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `press_key` | `(&mut self, key_code: String)` | 记录一次按下；键不在 held 中才追加到 `keys_pressed`。 |
| `release_key` | `(&mut self, key_code: String)` | 记录一次松开；移出 `keys_held` 并追加到 `keys_released`。 |
| `is_key_pressed` | `(&self, key_code: K) -> bool` | 本帧是否按下。`K: AsRef<str>`。 |
| `is_key_held` | `(&self, key_code: K) -> bool` | 当前是否按住。`K: AsRef<str>`。 |
| `is_key_released` | `(&self, key_code: K) -> bool` | 本帧是否松开。`K: AsRef<str>`。 |
| `press_mouse_button` | `(&mut self, button: MouseButton, position: Vector2D)` | 记录一次按下，并把鼠标位置记到该位置。 |
| `release_mouse_button` | `(&mut self, button: MouseButton)` | 记录一次松开。 |
| `update_mouse_position` | `(&mut self, position: Vector2D)` | 更新位置，同时计算 `delta` 并置 `moved`。 |
| `is_mouse_button_pressed` | `(&self, button: MouseButton) -> bool` | 本帧是否按下。 |
| `is_mouse_button_held` | `(&self, button: MouseButton) -> bool` | 当前是否按住。 |
| `update_touch` | `(&mut self, identifier: i32, position: Vector2D)` | 新增或移动一个触点。 |
| `start_touch` | `(&mut self, identifier: i32, position: Vector2D)` | 记录一个本帧落下的触点。 |
| `end_touch` | `(&mut self, identifier: i32)` | 移除触点并记入 `touch_ended`。 |
| `primary_touch_position` | `(&self) -> Option<Vector2D>` | 标识最小的活跃触点位置；无触点时 `None`。 |
| `end_frame` | `(&mut self)` | 清空全部每帧数据，保留 held。 |

`is_key_pressed` / `is_key_held` / `is_key_released` 的参数是泛型 `K: AsRef<str>`，`&str` 与 `String` 都能直接传：

```rust
use euv_engine::*;

let state: InputState = InputState::default();
let by_str: bool = state.is_key_held("KeyW");
let by_string: bool = state.is_key_held(String::from("KeyW"));
let _: bool = by_str && by_string;
```

### 边沿与按住

`press_key` 写入 `keys_pressed` 的同时总是写入 `keys_held`，`release_key` 则把键从 `keys_held` 移到 `keys_released`。由此得到几条值得记住的契约：

- **未按过的键松开，也照样报松开**。`release_key` 不检查该键此前是否被按下：

  ```rust
  use euv_engine::*;

  let mut input: InputState = InputState::default();
  input.release_key(String::from("KeyZ"));
  assert!(input.is_key_released("KeyZ"));
  assert!(!input.is_key_held("KeyZ"));
  ```

- **一帧内按下又松开，两个边沿都报**。`release_key` 不会清掉本帧的按下边沿，所以"在两帧之间点一下又松开"的按键会同时命中 `is_key_pressed` 与 `is_key_released`：

  ```rust
  use euv_engine::*;

  let mut input: InputState = InputState::default();
  input.press_key(String::from("KeyA"));
  input.release_key(String::from("KeyA"));
  assert!(input.is_key_pressed("KeyA") && input.is_key_released("KeyA"));
  ```

  因此**按下与松开不互斥**，判断"本帧是否有交互"要写 `pressed || released`，而不是 `pressed != released`。

- **`end_frame` 之后仍按住的键，下一帧 `pressed` 为假、`held` 为真**。

### 鼠标位置与位移

```rust
use euv_engine::*;

let mut input: InputState = InputState::default();
input.update_mouse_position(Vector2D::new(10.0, 10.0));
input.update_mouse_position(Vector2D::new(20.0, 10.0));
assert_eq!(input.get_mouse_delta(), Vector2D::new(10.0, 0.0));
assert!(input.get_mouse_moved());

input.end_frame();
assert_eq!(input.get_mouse_delta(), Vector2D::zero());
assert!(!input.get_mouse_moved());
```

`press_mouse_button` 会顺带把鼠标位置记成按下时的坐标，因此在 `mousedown` 里读 `get_mouse_position()` 拿到的就是点击点。

### 触摸

触点按 `i32` 标识独立跟踪。`primary_touch_position` 返回**标识最小**的那个触点 —— 由标识顺序而非落下的先后决定：

```rust
use euv_engine::*;

let mut input: InputState = InputState::default();
input.start_touch(2, Vector2D::new(0.0, 0.0));
input.start_touch(0, Vector2D::new(7.0, 8.0));
input.start_touch(1, Vector2D::new(3.0, 4.0));
assert_eq!(
    input.primary_touch_position(),
    Some(Vector2D::new(7.0, 8.0))
);
```

触摸事件不会更新 `mouse_position`，所以把触点当鼠标光标用的代码**只能**通过 `primary_touch_position()` 读取。主触点离开后会自动提升为下一个最小标识的触点；移动不会改变标识，因此主触点身份稳定。对从未开始过的标识调用 `end_touch` 是无害的。

## MouseButton

鼠标按键枚举，按 DOM `MouseEvent.button` 编号映射，`Left` 是默认值。

| 变体 | 说明 |
| --- | --- |
| `Left` | 左键（button 0，默认） |
| `Middle` | 中键 / 滚轮按下（button 1） |
| `Right` | 右键（button 2） |
| `Button4` | 按钮 3（浏览器后退） |
| `Button5` | 按钮 4（浏览器前进） |

## InputAction

一个按键在某一帧的状态，四种变体互斥。`button_action` 的判定优先级为 **Pressed > Released > Held**，因此连接与松开落在同一帧时报 `Pressed`。

| 变体 | 说明 |
| --- | --- |
| `Pressed` | 本帧按下（`Default`） |
| `Held` | 当前按住 |
| `Released` | 本帧松开 |
| `Idle` | 当前空闲，既没按也没按过 |

## GamepadState

单个手柄索引的一帧状态。原始数据（轴、按压力）由 `GamepadManager::poll` 写入，并通过下面的查询方法读取。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `axis` | `(&self, axis: u32) -> f64` | 读取第 `axis` 个模拟轴，**已套用死区**；越界返回 `0.0`。 |
| `button_value` | `(&self, button: u32) -> f64` | 读取第 `button` 个按键的原始压力，**不套死区**；越界返回 `0.0`。 |
| `is_button_pressed` | `(&self, button: u32) -> bool` | 本帧是否按下。 |
| `is_button_held` | `(&self, button: u32) -> bool` | 当前是否按住。 |
| `is_button_released` | `(&self, button: u32) -> bool` | 本帧是否松开。 |
| `button_action` | `(&self, button: u32) -> InputAction` | 折成枚举形式。 |
| `axis_pressed` | `(&self, axis: u32, threshold: f64) -> bool` | 修正后的读数是否达到 `threshold`。 |
| `end_frame` | `(&mut self)` | 清空 pressed / released，保留 held。 |

边沿集合遵循与 `InputState` 相同的约定：`buttons_pressed` 只报按下那一帧并被 `end_frame` 清空，`buttons_held` 跨帧存活，`buttons_released` 报"曾经按住、本帧松开"。

刚构造出来的 `GamepadState` 什么都不会报 —— 这让"还没轮询到手柄"的路径天然安全：

```rust
use euv_engine::*;

let pad: GamepadState = GamepadState::default();
assert_eq!(pad.button_action(0), InputAction::Idle);
assert_eq!(pad.axis(0), 0.0);
assert_eq!(pad.button_value(0), 0.0);
assert!(!pad.is_button_pressed(0));
assert!(!pad.axis_pressed(0, 0.1));
```

模拟扳机会给出连续的压力值，数字键则是 `1.0` / `0.0`；两者都以**压力达到 `0.5`** 为按下，所以 `button_value` 与 `is_button_held` 对数字键和模拟扳机是一致的。

## GamepadManager

持有页面上出现过的全部手柄状态，并驱动每帧轮询。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `poll` | `(&mut self, window: &Window)` | 重新读取 `navigator.get_gamepads()` 并重算本帧边沿。 |
| `attach` | `(manager_cell: GamepadManagerCell, window: &Window) -> GamepadManagerCell` | 挂上连接 / 断开监听，返回传入的同一单元。 |
| `state` | `(&self, index: u32) -> Option<&GamepadState>` | 按索引取手柄状态，未知索引返回 `None`。 |
| `state_mut` | `(&mut self, index: u32) -> Option<&mut GamepadState>` | 同上，可变。 |
| `is_connected` | `(&self, index: u32) -> bool` | 该索引当前是否连接。 |
| `axis` | `(&self, index: u32, axis: u32) -> Option<f64>` | 已套死区的轴读数；索引未知为 `None`。 |
| `button_value` | `(&self, index: u32, button: u32) -> Option<f64>` | 原始按键压力；索引未知为 `None`。 |
| `is_button_pressed` | `(&self, index: u32, button: u32) -> bool` | 本帧是否按下；索引未知为 `false`。 |
| `is_button_held` | `(&self, index: u32, button: u32) -> bool` | 当前是否按住；索引未知为 `false`。 |
| `is_button_released` | `(&self, index: u32, button: u32) -> bool` | 本帧是否松开；索引未知为 `false`。 |
| `button_action` | `(&self, index: u32, button: u32) -> InputAction` | 折成枚举；索引未知为 `Idle`。 |
| `connected_count` | `(&self) -> usize` | 当前连接的手柄数量。 |
| `end_frame` | `(&mut self)` | 清空每个手柄的每帧数据以及管理器自己的边沿集合。 |

`GamepadManager::attach` 的参数类型 `GamepadManagerCell` 是 `pub(crate)` 别名（等价于 `Rc<EngineCell<GamepadManager>>`），因此**类型可以直接拼写**，但别名本身不在公开 API 里。

### 轮询是权威来源

Gamepad API 是**轮询式**的：浏览器只更新 `navigator.get_gamepads()` 的快照，不会为单个按键派发事件，所以按键边沿必须靠"本帧快照 vs 上一帧"做差分得出。

`poll` 分两个阶段：先把每个在线手柄折进存储状态，再用"浏览器不再列出"的手柄索引执行释放。`gamepadconnected` / `gamepaddisconnected` 事件同样被挂上，但它们是**降低延迟**的优化 —— 两次轮询之间出现的设备在事件到达时就已经记录在案，而单靠轮询同样能发现连接与断开。

被浏览器移出列表的手柄会被标记为断开，并把当时的 held 集合整体作为一次"松开"上报 —— 这正是手柄被拔掉后角色不会一直往前跑的原因。状态条目本身会**保留**，这样同一个手柄重新插入时仍被识别为同一台设备。

```rust
use euv_engine::*;

let mut manager: GamepadManager = GamepadManager::default();
assert_eq!(manager.connected_count(), 0);

// 还没轮询时，所有查询都安全地报告"无"
assert!(manager.state(0).is_none());
assert!(!manager.is_connected(0));
assert_eq!(manager.axis(0, 0), None);
assert_eq!(manager.button_value(0, 0), None);
assert_eq!(manager.button_action(0, 0), InputAction::Idle);
```

轮询本身也有明确契约：没有任何手柄时轮询是空操作，列表中的空槽位（`null` / `undefined`）被跳过而不会被当成手柄解包。

## 自由函数

手柄数值处理的核心逻辑被拆成自由函数，既能被 `GamepadState` / `GamepadManager` 内部使用，也能在没有浏览器的情况下直接测试。

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `apply_axis_deadzone` | `(value: f64) -> f64` | 对单个原始轴读数套用模拟死区。 |
| `is_button_down` | `(value: f64) -> bool` | 判断一个原始按键压力是否算"按下"。 |
| `compute_pressed_buttons` | `(previous_held: &GamepadButtonSet, values: &[f64]) -> GamepadButtonSet` | 本帧**新按下**的按键。 |
| `compute_released_buttons` | `(previous_held: &GamepadButtonSet, values: &[f64]) -> GamepadButtonSet` | 本帧**新松开**的按键。 |
| `compute_held_buttons` | `(values: &[f64]) -> GamepadButtonSet` | 本帧所有按住的按键。 |
| `read_raw_axis` | `(axes: &[f64], axis: u32) -> f64` | 按下标读取原始读数，越界返回 `0.0`。 |

按键下标就是原始切片里的**位置**，集合类型是 `GamepadButtonSet`。

### apply_axis_deadzone

摇杆静止时总会偏几个百分点，若直接使用就会有一份恒定漂移渗进玩法。所以幅度落在死区带内（`≤ 0.15`，**含边界**）时一律报 `0.0`；超出后把剩余跨度重新拉伸回满量程，并在满偏时正好得到 `1.0`，超出范围的值被钳制。符号被保留，向左推仍然是负值。

```rust
use euv_engine::*;

// 死区内（含边界）→ 0.0
assert_eq!(apply_axis_deadzone(0.05), 0.0);
assert_eq!(apply_axis_deadzone(0.1), 0.0);
assert_eq!(apply_axis_deadzone(0.15), 0.0);
assert_eq!(apply_axis_deadzone(-0.05), 0.0);

// 越过死区后拉伸回满量程
assert!(apply_axis_deadzone(0.16) > 0.0);
assert_eq!(apply_axis_deadzone(1.0), 1.0);
assert_eq!(apply_axis_deadzone(-1.0), -1.0);

// 越界钳制，符号保留
assert_eq!(apply_axis_deadzone(1.5), 1.0);
assert_eq!(apply_axis_deadzone(-1.5), -1.0);
```

### is_button_down

阈值是 `0.5` 且**含边界**，让上报 `1.0` 的数字键与越过中点的模拟扳机在同一个阈值上达成一致。

```rust
use euv_engine::*;

assert!(is_button_down(0.5));
assert!(!is_button_down(0.49));
assert!(is_button_down(1.0));
assert!(!is_button_down(0.0));
```

### 三个按键集合函数

`compute_pressed_buttons` 与 `compute_released_buttons` 都拿**上一帧的 held 集合**做差分，`compute_held_buttons` 只看本帧读数。三者的组合保证：一帧之内不可能有按键同时落在 pressed 与 released 上，没有任何变化的一帧不产生任何边沿。

```rust
use euv_engine::*;

// 上一帧什么都没有
let previous: GamepadButtonSet = GamepadButtonSet::new();

// 本帧按下 0 和 2
let held: GamepadButtonSet = compute_held_buttons(&[1.0, 0.0, 1.0]);
let pressed: GamepadButtonSet = compute_pressed_buttons(&previous, &[1.0, 0.0, 1.0]);
assert!(held.contains(&0) && held.contains(&2));
assert_eq!(pressed.len(), 2);

// 持续按住不会重复产生按下边沿
assert!(compute_pressed_buttons(&held, &[1.0, 0.0, 1.0]).is_empty());

// 全部松开
let released: GamepadButtonSet = compute_released_buttons(&held, &[0.0, 0.0, 0.0]);
assert_eq!(released.len(), 2);
```

**消失的按键槽位算作松开**：上一帧按住、这一帧不再上报（切片变短）的按键会进入 released，而不是卡在 held 上。

### read_raw_axis

```rust
use euv_engine::*;

assert_eq!(read_raw_axis(&[0.1, 0.2, 0.3], 1), 0.2);
assert_eq!(read_raw_axis(&[0.75, 0.0], 0), 0.75);

// 越界返回 0.0 而不是 panic，空切片同理
assert_eq!(read_raw_axis(&[0.1], 9), 0.0);
assert_eq!(read_raw_axis(&[], 0), 0.0);
```

## Input

零尺寸命名空间 `struct Input`，与 `App` 同构 —— 所有操作都是关联函数，本身不持有状态。

| 方法 | 签名 | 说明 |
| --- | --- | --- |
| `attach` | `(state_cell: InputStateCell, window: &Window, pointer_target: &EventTarget) -> InputStateCell` | 挂上全部键鼠触摸监听，返回传入的同一单元。 |
| `extract_key_code` | `(event: &Event) -> String` | 从键盘事件抽取键码字符串（`"KeyA"`、`"Space"`、`"ArrowLeft"`）。 |
| `extract_mouse_button` | `(event: &Event) -> MouseButton` | 从鼠标事件抽取按键枚举。 |
| `extract_mouse_position` | `(event: &Event) -> Vector2D` | 从鼠标事件抽取视口坐标。 |

`attach` 的绑定目标是有讲究的：

- **键盘事件绑到 `window`**，而不是 canvas —— `<canvas>` 默认不可聚焦，除非设置了 `tabindex` 且用户点击过它，否则绑在 canvas 上的键盘监听永远不会触发。
- **鼠标与触摸事件绑到指针目标**（通常是 canvas 元素），这样命中测试坐标保持在 canvas 局部空间。
- `touchstart` / `touchmove` 会调用 `prevent_default()`，避免浏览器在引擎看到之前把触摸解释成滚动或缩放手势；`contextmenu` 被屏蔽，使右键作为 `MouseButton::Right` 到达引擎，而不是弹出浏览器菜单。
- 单个触摸事件可能携带多个变化的触点，因此会遍历整个 `changedTouches`，而不是只读第 0 个。

注册后的闭包被 `.forget()` 掉，随文档生命周期常驻（引擎的"只挂载不卸载"约定），因此没有配对的注销 API；即使调用方丢弃自己的 `Rc`，监听器持有的单元克隆仍会保持状态可达。

## InputStateCell

```text
pub type InputStateCell = Rc<EngineCell<InputState>>;
```

DOM 监听器克隆这个 `Rc` 让状态活到文档结束，游戏代码则通过 `EngineHandle::register_input` / `try_get_input_cell` 读同一个单元。`EngineCell` 是单线程可变单元，`get()` 给出共享引用、`get_mut()` 给出可变引用：

```rust
use euv_engine::*;

let cell: InputStateCell = std::rc::Rc::new(EngineCell::new(InputState::default()));
cell.get_mut().press_key(String::from("Space"));
assert!(cell.get().is_key_held("Space"));
```

在引擎里，注册输入后从句柄取回状态：

```rust
use euv_engine::*;

let mut handle: EngineHandle = Engine::new_handle(EngineConfig::default());

// 重复调用是幂等的，返回同一个单元
if let Some(cell) = handle.register_input() {
    let state: &InputState = cell.get();
    if state.is_key_held("Space") {
        // 跳跃
    }
}
```

## 内部常量

死区 `0.15` 与按键阈值 `0.5` 定义为模块内的 `pub(crate)` 常量（`INPUT_GAMEPAD_AXIS_DEADZONE`、`GAMEPAD_BUTTON_PRESS_THRESHOLD`），**不属于公开 API**，外部代码无法按名引用。它们的数值直接决定上面的行为；要改死区只能改引擎源码。

## 设计要点

- **边沿与按住分离**，是整个模块的骨架：按住的东西跨帧持续可读，而"刚刚发生"的事件只活一帧，两者不会互相污染。
- **收尾由调度器统一执行**，且在渲染回调之后，因此游戏逻辑在本帧内读到的永远是本帧的事实，不会读到下一帧的边沿。
- **游戏手柄是轮询 API，边沿靠差分得出**；`poll` 是权威来源，连接 / 断开事件只是把延迟降低一帧。
- **手柄被拔掉时 held 集合被整体当作松开上报**，堵住"角色一直朝一个方向跑"这个真实故障。
- **松开的按键不要求此前按下过**，按下与松开边沿也不互斥 —— 想避免重复响应，需要调用方自己在逻辑里去重。
