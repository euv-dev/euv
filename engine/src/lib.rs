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
mod raytracing;
mod renderer;
mod scene;
mod scheduler;
mod spatial;
mod sprite;
mod timer;

mod tween;

use wasm_bindgen::JsValue;
pub use {
    asset::*, audio::*, cell::*, collider::*, config::*, easing::*, engine::*, entity::*, input::*,
    lighting::*, math::*, particle::*, physics::*, raytracing::*, renderer::*, scene::*,
    scheduler::*, spatial::*, sprite::*, timer::*, tween::*,
};

pub use std::{
    error::Error,
    f64::consts::{FRAC_PI_2, PI, TAU},
    fmt::{self, Debug, Display, Formatter, Result as FmtResult, Write as _},
    future::{Future, Ready, ready},
    mem::{self, replace},
};

use euv::*;

use std::{
    cell::{RefCell, UnsafeCell},
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};

use {
    js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, wasm_bindgen_futures::JsFuture,
    web_sys::*,
};
