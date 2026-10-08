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

pub use wasm_bindgen::JsValue;
pub use {
    asset::*, audio::*, cell::*, collider::*, config::*, easing::*, engine::*, entity::*, input::*,
    lighting::*, math::*, particle::*, physics::*, pool::*, raytracing::*, renderer::*, scene::*,
    scheduler::*, spatial::*, sprite::*, timer::*, tween::*,
};

pub use wasm_bindgen_futures::JsFuture;

pub use std::{
    error::Error,
    // Both forms on purpose: the items are the crate's public numeric
    // vocabulary, and the module itself is what math/const.rs refers to,
    // because that file declares its own PI / TWO_PI / HALF_PI and importing
    // the same names there would shadow the very constants it defines.
    f64::consts,
    f64::consts::{FRAC_PI_2, PI, TAU},
    fmt::{self, Debug, Display, Formatter, Write},
    future::{Future, Ready, ready},
    mem::{self, replace},
    sync::LazyLock,
};

use euv::*;

use std::{
    cell::{RefCell, RefMut, UnsafeCell},
    collections::{HashMap, HashSet},
    mem::take,
    ops::{Add, AddAssign, BitOr, Mul, MulAssign, Neg, Sub, SubAssign},
    rc::Rc,
    rc::Weak,
    slice::from_ref,
    sync::atomic::{AtomicU64, Ordering},
};
