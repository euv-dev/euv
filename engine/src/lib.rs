//! euv-engine
//!
//! A high-performance 2D and 3D game engine built on the euv framework for WebAssembly,
//! featuring an ECS-style entity system, fixed-timestep game loop, canvas rendering,
//! WebGPU rendering, physics simulation, collision detection, sprite animation,
//! scene management, asset loading, and Web Audio integration.

mod asset;
mod audio;
mod cell;
mod collider;
mod config;
mod easing;
mod engine;
mod entity;
mod input;
mod lighting;
mod math;
mod particle;
mod physics;
mod pool;
mod raytracing;
mod renderer;
mod scene;
mod scheduler;
mod spatial;
mod sprite;
mod timer;
mod tween;

pub use {
    asset::*, audio::*, cell::*, collider::*, config::*, easing::*, engine::*, entity::*, input::*,
    lighting::*, math::*, particle::*, physics::*, pool::*, raytracing::*, renderer::*, scene::*,
    scheduler::*, spatial::*, sprite::*, timer::*, tween::*,
};

pub use {wasm_bindgen::JsValue, wasm_bindgen_futures::JsFuture};

use euv::*;

use std::{
    cell::{RefCell, RefMut, UnsafeCell},
    collections::{HashMap, HashSet},
    error::Error,
    f64::consts::{self, PI},
    fmt::{self, Debug, Display, Formatter, Write},
    mem::take,
    mem::{self, replace},
    ops::{Add, AddAssign, BitOr, Mul, MulAssign, Neg, Sub, SubAssign},
    rc::Rc,
    rc::Weak,
    slice::from_ref,
    sync::LazyLock,
    sync::atomic::{AtomicU64, Ordering},
};
