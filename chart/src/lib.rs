//! euv-chart
//!
//! A high-performance charting library for the euv framework.
//!
//! The crate offers an ECharts-compatible option model, renders through
//! WebGPU, WebGL2, or Canvas2D — whichever the host browser supports —
//! and exposes one API for all three, so a developer never has to know
//! which backend is in use.

mod animation;
mod backend;
mod chart;
mod display;
mod event;
mod geo;
mod geom;
mod model;
mod option;
mod paint;
mod series;
mod text;

pub use {
    animation::*, backend::*, chart::*, display::*, event::*, geo::*, geom::*, model::*, option::*,
    paint::*, series::*, text::*,
};

// The easing of an animation belongs to the engine, and so does the
// easing a chart's animator takes. It is re-exported here so that an
// application can name it without reaching for the engine.
pub use euv_engine::Easing;

// The DOM types the chart crate talks to. They are imported one by one
// rather than through a blanket `web_sys::*` glob, because the crate
// defines its own `Transform2D` and `Position`, which would collide with
// the DOM types of the same name.
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    error::Error,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    iter::{Peekable, once},
    mem::take,
    rc::Rc,
    slice::from_ref,
    str::{Chars, SplitWhitespace},
};

use {
    euv_core::Signal,
    euv_engine::{Tween, TweenState},
    js_sys::{Array, Float32Array, Function, Object, Uint8Array, Uint32Array},
    lombok_macros::Data,
    wasm_bindgen::{JsCast, JsValue},
    wasm_bindgen_futures::JsFuture,
    web_sys::{
        CanvasGradient, CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement,
        Navigator, WebGl2RenderingContext, WebGlBuffer, WebGlContextAttributes, WebGlProgram,
        WebGlShader, WebGlTexture, WebGlUniformLocation, WebGlVertexArrayObject, Window,
    },
};
