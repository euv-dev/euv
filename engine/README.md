<center>

## euv-engine

[![](https://img.shields.io/crates/v/euv-engine.svg)](https://crates.io/crates/euv-engine)
[![](https://img.shields.io/crates/d/euv-engine.svg)](https://img.shields.io/crates/d/euv-engine.svg)
[![](https://docs.rs/euv-engine/badge.svg)](https://docs.rs/euv-engine)
[![](https://github.com/euv-dev/euv/workflows/Rust/badge.svg)](https://github.com/euv-dev/euv/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/euv-engine.svg)](../LICENSE)

</center>

[Api Docs](https://docs.rs/euv-engine/latest/)

> A high-performance 2D game engine built on the euv framework, featuring ECS, fixed-timestep game loop, canvas rendering, physics, collision detection, sprite animation, and audio.

`euv-engine` 是 euv 的游戏引擎 crate（3D 数学与物理同样在此）。模块内所有类型都通过 `pub use` 从 crate 根直接导出，通常不需要 `use euv_engine::module::...`。

## Installation

```shell
cargo add euv-engine
```

## 模块文档

| 模块 | 内容 |
| --- | --- |
| [输入](./input.md) | 键盘、鼠标、触摸、手柄统一成「每帧读一次」的状态 |
| [渲染后端与配置](./renderer.md) | 渲染后端选择、画布尺寸与设备像素比、描述符 |
| [物理与碰撞](./physics.md) | 刚体、求解器、2D/3D 碰撞体与碰撞结果 |
| [数学库](./math.md) | 向量、四元数、包围盒、缓动函数 |

四页均为全中文 API 文档，配可编译示例。

## 文档的边界

[物理与碰撞](./physics.md) 讲刚体与求解器，同时覆盖 `collider` / `spatial` 两个子模块里的碰撞体与宽相；[数学库](./math.md) 讲 `Vector2D` / `Vector3D` / `Quaternion` / `Rect` / `Circle` / `Sphere` / `AABB3D` 与缓动。

两者的交界点就是碰撞体：物理页用到的一切向量与包围盒都定义在数学页，反过来数学页不涉及任何刚体概念。

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
