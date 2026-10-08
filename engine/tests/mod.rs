mod animator;
mod api_visibility;
mod asset;
mod audio_clip;
mod audio_prop;
mod browser_env;
mod camera3d;
mod canvas_calls;
mod canvas_draw;
mod canvas_gradients;
mod canvas_images;
mod canvas_math;
mod cell;
mod collider;
mod config;
mod descriptor;
mod dom_backstop;
mod dom_fallback;
mod easing;
mod engine_handle;
mod entity;
mod gamepad_poll;
mod geometry;
mod geometry3d;
mod gpu_bits;
mod gpu_desc;
mod gpu_state;
mod r#input;
mod input_fn;
mod input_state;
mod lifecycle;
mod lighting;
mod math;
mod no_dom_fallback;
mod particle;
mod physics;
mod physics_world;
mod pool;
mod quadtree;
mod raytracing;
mod remaining;
mod renderer;
mod rigidbody;
mod scene;
mod scheduler;
mod shader_stages;
mod spatial;
mod spatial_types;
mod sprite;
mod sprite_draw;
mod sprite_slices;
mod timer;
mod transform;
mod tween;
mod types;
mod wasm_fallback;
mod webgl_backend;
mod webgl_buffers;
mod webgl_calls;
mod webgl_draw;
mod webgl_draw_range;
mod webgl_frame;
mod webgl_framebuffer;
mod webgl_programs;
mod webgl_sources;
mod webgl_state;
mod webgl_uniform_blocks;
mod webgl_vao;
mod r#webgpu;
mod webgpu_cache;
mod webgpu_dynamic;
mod webgpu_init;
mod webgpu_lifecycle;
mod webgpu_pass;
mod webgpu_pipelines;
mod webgpu_resources;
mod webgpu_transfer;

use scheduler::CountingHandler;

use std::{
    cell::{Cell, RefCell, RefMut, UnsafeCell},
    collections::HashSet,
    f64::consts::{self, FRAC_PI_2, PI, TAU},
    fmt::{self, Debug, Write},
    future::{Future, Ready, ready},
    rc::Rc,
    sync::OnceLock,
};

use {euv_engine::*, js_sys::*, wasm_bindgen::*, web_sys::*};

const RECORDER_SOURCE: &str = r#"
const seen = [];
function shape(value, depth) {
    if (value === null) { return 'null'; }
    if (value === undefined) { return 'undefined'; }
    const kind = typeof value;
    if (kind === 'number' || kind === 'string' || kind === 'boolean') { return value; }
    if (seen.indexOf(value) >= 0) { return '<gpu>'; }
    if (kind === 'function') { return 'function'; }
    if (depth > 4) { return 'deep'; }
    if (Array.isArray(value)) {
        return Array.from(value).map(function (item) { return shape(item, depth + 1); });
    }
    if (ArrayBuffer.isView(value)) { return { bytes: value.length }; }
    const out = {};
    for (const key of Object.keys(value)) { out[key] = shape(value[key], depth + 1); }
    return out;
}
function node(path) {
    const target = function () {
        log.push([path, Array.from(arguments).map(function (item) { return shape(item, 0); })]);
        return node(path + '()');
    };
    const proxy = new Proxy(target, {
        get: function (fn, key) {
            if (key === 'then') { return undefined; }
            if (typeof key === 'symbol') { return undefined; }
            if (key === 'call' || key === 'apply' || key === 'bind') { return fn[key]; }
            if (key === 'toString' || key === 'valueOf') { return function () { return path; }; }
            return node(path + '.' + String(key));
        },
        set: function (fn, key, value) {
            log.push([path + '=' + String(key), shape(value, 0)]);
            return true;
        },
        apply: function (fn, self, args) {
            log.push([path, Array.from(args).map(function (item) { return shape(item, 0); })]);
            return node(path + '()');
        },
        has: function () { return true; }
    });
    seen.push(proxy);
    return proxy;
}
const handles = {};
handles.open = function (name) { return node(name); };
handles.device = handles.open('device');
handles.queue = handles.open('queue');
handles.context = handles.open('context');
const backing = { width: 800, height: 600, clientWidth: 800, clientHeight: 600 };
handles.canvas = new Proxy(backing, {
    get: function (target, key) {
        if (key === 'getContext') {
            return function () {
                log.push(['canvas.getContext', Array.from(arguments).map(function (item) { return shape(item, 0); })]);
                return handles.context;
            };
        }
        return target[key];
    },
    set: function (target, key, value) {
        log.push(['canvas.' + String(key), shape(value, 0)]);
        target[key] = value;
        return true;
    }
});
return handles;
"#;

struct GpuRecorder {
    handles: Object,
    log: Array,
}

static RECORDER: OnceLock<GpuRecorder> = OnceLock::new();

fn recorder() -> &'static GpuRecorder {
    RECORDER.get_or_init(|| {
        let log: Array = Array::new();
        let build: js_sys::Function = js_sys::Function::new_with_args("log", RECORDER_SOURCE);
        let built: Result<JsValue, JsValue> = build.call1(&JsValue::NULL, log.as_ref());
        assert!(built.is_ok(), "the js_sys recorder must install cleanly");
        let handles: Object = built.unwrap().unchecked_into();
        GpuRecorder { handles, log }
    })
}

fn named(handle: &str) -> Object {
    let value: Result<JsValue, JsValue> =
        Reflect::get(&recorder().handles, &JsValue::from_str(handle));
    assert!(value.is_ok(), "the recorder must expose {handle}");
    value.unwrap().unchecked_into()
}

pub(crate) fn device() -> Object {
    named("device")
}

pub(crate) fn queue() -> Object {
    named("queue")
}

pub(crate) fn context() -> Object {
    named("context")
}

pub(crate) fn node(name: &str) -> Object {
    let opener: Result<JsValue, JsValue> =
        Reflect::get(&recorder().handles, &JsValue::from_str("open"));
    let opened: Result<JsValue, JsValue> = opener
        .expect("the recorder must expose its node factory")
        .dyn_into::<js_sys::Function>()
        .expect("the node factory must be callable")
        .call1(&JsValue::NULL, &JsValue::from_str(name));
    assert!(opened.is_ok(), "the recorder must be able to open {name}");
    opened.unwrap().unchecked_into()
}

pub(crate) fn canvas() -> HtmlCanvasElement {
    named("canvas").unchecked_into()
}

pub(crate) fn renderer(antialias: bool) -> WebGpuRenderer {
    WebGpuRenderer::new(WebGpuRendererInit {
        device: device().into(),
        queue: queue().into(),
        context: context().into(),
        canvas: canvas(),
        format: String::from("bgra8unorm"),
        width: 800,
        height: 600,
        antialias,
    })
}

pub(crate) fn mark() -> u32 {
    recorder().log.length()
}

pub(crate) fn ops_from(from: u32) -> Vec<String> {
    (from..recorder().log.length())
        .map(|index: u32| {
            let entry: Array = recorder().log.get(index).unchecked_into();
            entry.get(0).as_string().unwrap_or_default()
        })
        .collect()
}

pub(crate) fn count_on(receiver: &str, op: &str, from: u32) -> usize {
    ops_from(from)
        .iter()
        .filter(|recorded: &&String| recorded.as_str() == format!("{receiver}.{op}"))
        .count()
}

pub(crate) fn ops_since(from: u32) -> Vec<String> {
    ops_from(from)
        .iter()
        .map(|recorded: &String| {
            recorded
                .rsplit('.')
                .next()
                .unwrap_or(recorded.as_str())
                .to_string()
        })
        .collect()
}

pub(crate) fn count_since(op: &str, from: u32) -> usize {
    ops_since(from)
        .iter()
        .filter(|recorded: &&String| recorded.as_str() == op)
        .count()
}

pub(crate) fn args_since(op: &str, from: u32) -> Array {
    for index in from..recorder().log.length() {
        let entry: Array = recorder().log.get(index).unchecked_into();
        let recorded: String = entry.get(0).as_string().unwrap_or_default();
        if recorded.rsplit('.').next().unwrap_or_default() == op {
            return entry.get(1).unchecked_into();
        }
    }
    panic!(
        "{op} was never called after that point; recorded {:?}",
        ops_from(from)
    );
}

pub(crate) fn args_on(receiver: &str, op: &str, from: u32) -> Array {
    let wanted: String = format!("{receiver}.{op}");
    for index in from..recorder().log.length() {
        let entry: Array = recorder().log.get(index).unchecked_into();
        if entry.get(0).as_string().as_deref() == Some(wanted.as_str()) {
            return entry.get(1).unchecked_into();
        }
    }
    panic!(
        "{wanted} was never called after that point; recorded {:?}",
        ops_from(from)
    );
}

pub(crate) fn field(value: &JsValue, path: &[&str]) -> JsValue {
    let mut current: JsValue = value.clone();
    for key in path {
        current = match Reflect::get(&current, &JsValue::from_str(key)) {
            Ok(next) => next,
            Err(_) => return JsValue::UNDEFINED,
        };
    }
    current
}
