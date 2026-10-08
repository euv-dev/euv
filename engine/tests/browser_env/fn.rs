use super::*;
fn listener_count() -> u32 {
    let read: js_sys::Function = js_sys::Function::new_no_args(
        "return typeof globalThis.__euvListenerCount === 'number' ? globalThis.__euvListenerCount : -1;",
    );
    read.call0(&JsValue::NULL)
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .map(|count: f64| count as u32)
        .unwrap_or(0)
}

fn pump_animation_frame() -> bool {
    let run: js_sys::Function = js_sys::Function::new_no_args(
        "if (typeof globalThis.__euvRaf !== \"function\") { return false; } globalThis.__euvRaf(); return true;",
    );
    let fired: bool = run
        .call0(&JsValue::NULL)
        .ok()
        .and_then(|value: JsValue| value.as_bool())
        .unwrap_or(false);
    fired
}

fn dom_teardown() {
    let strip: js_sys::Function = js_sys::Function::new_no_args(
        r#"
        for (const key of ['document','navigator','performance','requestAnimationFrame',
                           'cancelAnimationFrame','addEventListener','removeEventListener',
                           'localStorage','window','Window','Document','Element','Node',
                           'Navigator','Performance','HTMLCanvasElement','HTMLImageElement',
                           'CanvasRenderingContext2D','WebGL2RenderingContext','Gamepad',
                           'GamepadList','Storage','__euvDomInstalled','__euvDomLog',
                           '__euvContext','__euvCanvas2d','__euvWebgl2','__euvCanvasElement','__euvNode',
                           '__euvRaf','__euvListenerCount']) {
            try { delete globalThis[key]; } catch (e) {}
        }
        Object.setPrototypeOf(globalThis, Object.prototype);
        "#,
    );
    let _: Result<JsValue, JsValue> = strip.call0(&JsValue::NULL);
}
fn dom_source() -> js_sys::Function {
    js_sys::Function::new_no_args(
        r#"
        if (globalThis.__euvDomInstalled) { return; }
        globalThis.__euvDomInstalled = true;

        const seen = [];
        function node(path, proto) {
            const target = function () {
                globalThis.__euvDomLog.push([path, Array.from(arguments)]);
                return node(path + '()');
            };
            const proxy = new Proxy(target, {
                getPrototypeOf() { return proto || target; },
                get(fn, key) {
                    if (key === 'then') { return undefined; }
                    if (typeof key === 'symbol') { return undefined; }
                    if (key === 'call' || key === 'apply' || key === 'bind') { return fn[key]; }
                    if (key === 'constructor') { return undefined; }
                    if (Object.prototype.hasOwnProperty.call(fn, key)) { return fn[key]; }
                    return node(path + '.' + String(key));
                },
                set(fn, key, value) {
                    globalThis.__euvDomLog.push([path + '=' + String(key), [value]]);
                    fn[key] = value;
                    return true;
                },
                apply(fn, self, args) {
                    globalThis.__euvDomLog.push([path, Array.from(args)]);
                    return node(path + '()');
                },
                has() { return true; }
            });
            seen.push(proxy);
            return proxy;
        }
        globalThis.__euvDomLog = [];

        function Ctor() {}
        for (const name of ['Window', 'Document', 'Element', 'Node', 'Navigator',
                            'Performance', 'HTMLCanvasElement', 'HTMLImageElement',
                            'CanvasRenderingContext2D', 'WebGL2RenderingContext',
                            'Gamepad', 'GamepadList', 'Storage']) {
            globalThis[name] = Ctor;
        }

        const canvasProto = Object.create(globalThis.HTMLCanvasElement.prototype);
        const canvas = Object.create(canvasProto);
        canvas.__tag = 'canvas';
        canvas.width = 800;
        canvas.height = 600;
        canvas.getContext = function (kind) {
            globalThis.__euvDomLog.push(['canvas.getContext', [kind]]);
            if (kind === 'webgpu') { return globalThis.__euvContext; }
            if (kind === '2d') { return globalThis.__euvCanvas2d; }
            return globalThis.__euvWebgl2;
        };
        globalThis.__euvCanvasElement = canvas;

        globalThis.document = node('document', globalThis.Document.prototype);
        globalThis.document.createElement = function (tag) {
            globalThis.__euvDomLog.push(['document.createElement', [tag]]);
            return node('element(' + tag + ')');
        };
        globalThis.document.querySelector = function (sel) {
            globalThis.__euvDomLog.push(['document.querySelector', [sel]]);
            if (String(sel).indexOf('canvas') >= 0) { return canvas; }
            return null;
        };
        globalThis.document.getElementById = function (id) {
            globalThis.__euvDomLog.push(['document.getElementById', [id]]);
            return canvas;
        };
        globalThis.document.body = node('document.body');
        globalThis.document.head = node('document.head');
        globalThis.document.documentElement = node('document.documentElement');

        globalThis.performance = node('performance', globalThis.Performance.prototype);
        globalThis.performance.now = function () { return 16.0; };

        const pads = [];
        globalThis.navigator = node('navigator', globalThis.Navigator.prototype);
        globalThis.navigator.getGamepads = function () { return pads; };
        globalThis.navigator.userAgent = 'euv-test';

        globalThis.requestAnimationFrame = function (cb) {
            globalThis.__euvDomLog.push(['requestAnimationFrame', []]);
            globalThis.__euvRaf = cb;
            return 1;
        };
        globalThis.cancelAnimationFrame = function () {};
        globalThis.__euvListenerCount = 0;
        globalThis.addEventListener = function (name) {
            globalThis.__euvListenerCount += 1;
            globalThis.__euvDomLog.push(['addEventListener', [name]]);
        };
        globalThis.removeEventListener = function (name) {
            globalThis.__euvDomLog.push(['removeEventListener', [name]]);
        };

        globalThis.localStorage = node('localStorage');

        Object.setPrototypeOf(globalThis, globalThis.Window.prototype);

        const win = globalThis;
        win.document = globalThis.document;
        win.navigator = globalThis.navigator;
        win.performance = globalThis.performance;
        win.requestAnimationFrame = globalThis.requestAnimationFrame;
        win.cancelAnimationFrame = globalThis.cancelAnimationFrame;
        win.addEventListener = globalThis.addEventListener;
        win.removeEventListener = globalThis.removeEventListener;
        win.localStorage = globalThis.localStorage;
        win.devicePixelRatio = 1;
        win.innerWidth = 800;
        win.innerHeight = 600;
        win.getComputedStyle = function () { return {}; };
        globalThis.window = win;

        globalThis.__euvContext = node('context');
        globalThis.__euvCanvas2d = node(
            'canvas2d', globalThis.CanvasRenderingContext2D.prototype);
        globalThis.__euvWebgl2 = node(
            'webgl2', globalThis.WebGL2RenderingContext.prototype);
        globalThis.__euvNode = node;
        "#,
    )
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "installs browser globals into the JS realm, which only exists under wasm"
)]
fn the_browser_shaped_globals_can_be_installed_into_the_node_realm() {
    let install: js_sys::Function = dom_source();
    let _: Result<JsValue, JsValue> = install.call0(&JsValue::NULL);

    assert!(
        window().is_some(),
        "window() is an instanceof check against the global Window constructor; \
         supplying both the constructor and an object carrying its prototype is what makes it \
         resolve"
    );
    let handle: Window = window().expect("just asserted");
    assert!(
        handle.document().is_some(),
        "and a window that answers with a document"
    );
    assert_eq!(
        handle.device_pixel_ratio(),
        1.0,
        "with a device pixel ratio the size helpers can read"
    );

    dom_teardown();
    assert!(
        window().is_none(),
        "and taking the environment back must leave the realm as it was found, because the          globals are process-wide: a fake document left behind would silently satisfy every          later test that asserts the engine degrades without one"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "installs browser globals into the JS realm, which only exists under wasm"
)]
fn a_webgl_backend_can_be_built_once_the_document_exists() {
    let install: js_sys::Function = dom_source();
    let _: Result<JsValue, JsValue> = install.call0(&JsValue::NULL);

    let backend: Result<WebGl2Backend, WebGl2InitError> =
        WebGl2Backend::init(&EngineConfig::default().get_render());

    let shown: String = match &backend {
        Ok(_) => String::from("Ok"),
        Err(error) => format!("{:?}", error),
    };
    dom_teardown();
    assert!(
        backend.is_ok(),
        "initialization only ever needed a document that reports a canvas and a context;          without one it short-circuits to CanvasNotFound, which is why this entry point looked          untestable rather than merely untested. Observed: {shown}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "installs browser globals into the JS realm, which only exists under wasm"
)]
fn the_scheduler_loop_runs_when_its_animation_frame_callback_fires() {
    let install: js_sys::Function = dom_source();
    let _: Result<JsValue, JsValue> = install.call0(&JsValue::NULL);
    let (handler, updates, renders): (TickHandlerRc, Rc<Cell<u32>>, Rc<Cell<u32>>) =
        CountingHandler::spawn();

    let handle: SchedulerHandle =
        SchedulerHandle::start(SchedulerConfig::default(), handler, None, None);
    let updates_at_start: u64 = handle.update_count();
    let frames_at_start: u64 = handle.frame_count();

    let fired: bool = pump_animation_frame();
    handle.stop();
    dom_teardown();

    assert!(
        fired,
        "starting the scheduler has to register an animation-frame callback, or the loop has \
         nothing to drive it and the game never advances"
    );
    assert!(
        handle.frame_count() > frames_at_start,
        "one fired frame increments the frame counter, which is what a profiler panel reads to \
         compute fps"
    );
    assert!(
        handle.update_count() >= updates_at_start,
        "and the fixed-update counter only ever moves forward"
    );
    let _: u32 = updates.get();
    let _: u32 = renders.get();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "installs browser globals into the JS realm, which only exists under wasm"
)]
fn a_stopped_scheduler_does_not_advance_even_if_its_callback_is_fired() {
    let install: js_sys::Function = dom_source();
    let _: Result<JsValue, JsValue> = install.call0(&JsValue::NULL);
    let (handler, _updates, _renders): (TickHandlerRc, Rc<Cell<u32>>, Rc<Cell<u32>>) =
        CountingHandler::spawn();

    let handle: SchedulerHandle =
        SchedulerHandle::start(SchedulerConfig::default(), handler, None, None);
    handle.stop();
    let after_stop: u64 = handle.frame_count();

    let _: bool = pump_animation_frame();
    dom_teardown();

    assert!(
        !handle.is_running(),
        "stopping has to flip the running flag, or the next animation frame keeps ticking"
    );
    assert_eq!(
        handle.frame_count(),
        after_stop,
        "and a callback that fires after the stop must not count another frame"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "installs browser globals into the JS realm, which only exists under wasm"
)]
fn attaching_input_registers_listeners_and_hands_back_the_same_state_cell() {
    let install: js_sys::Function = dom_source();
    let _: Result<JsValue, JsValue> = install.call0(&JsValue::NULL);
    let window: Window = window().expect("the environment is installed");
    let cell: InputStateCell = Rc::new(EngineCell::new(InputState::new()));

    let returned: InputStateCell = Input::attach(
        Rc::clone(&cell),
        &window,
        &window.clone().unchecked_into::<EventTarget>(),
    );

    let registered: u32 = listener_count();
    dom_teardown();

    assert!(
        Rc::ptr_eq(&cell, &returned),
        "attach is documented to hand the cell back, and returning a different one would leave \
         the caller mutating a state the event handlers never write to"
    );
    assert!(
        registered > 0,
        "and it has to actually register listeners, not just return the cell — otherwise a \
         caller believes input is wired up while no event ever reaches the state"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "installs browser globals into the JS realm, which only exists under wasm"
)]
fn a_supersampled_canvas_builds_its_offscreen_buffer_and_blits_on_present() {
    let install: js_sys::Function = dom_source();
    let _: Result<JsValue, JsValue> = install.call0(&JsValue::NULL);

    let canvas: Option<SsaaCanvas> = SsaaCanvas::from_selector("canvas", 800.0, 600.0);
    let presentable: bool = canvas.is_some();
    if presentable {
        let subject: SsaaCanvas = canvas.expect("just checked");
        subject.present();
    }
    dom_teardown();

    assert!(
        presentable,
        "with a document that reports a canvas and a 2D context, the supersampled canvas has \
         to build; before the environment existed this entry point could not be reached at all"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn the_capability_probe_answers_false_where_there_is_no_navigator_at_all() {
    assert!(
        !WebGpuRenderer::probe().await,
        "a host with no window exposes no navigator and therefore no `gpu`; the probe is the \
         cheap pre-flight a caller runs before promising a GPU-backed canvas, and answering true \
         here would send it down a path that ends in a blank canvas"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn running_the_engine_on_the_canvas_backend_brings_up_a_renderer_and_a_running_loop() {
    let install: js_sys::Function = dom_source();
    let _: Result<JsValue, JsValue> = install.call0(&JsValue::NULL);
    let (handler, updates, _renders): (TickHandlerRc, Rc<Cell<u32>>, Rc<Cell<u32>>) =
        CountingHandler::spawn();

    let handle: EngineHandle = Engine::run(EngineConfig::default(), handler).await;
    let canvas_up: bool = handle.get_canvas_renderer().is_some();
    let pumping: bool = pump_animation_frame();
    handle.stop();
    dom_teardown();

    assert!(
        canvas_up,
        "run has to bring up the renderer the configuration asks for; the default config is the \
         Canvas 2D backend, so an empty handle after run means the frame has nothing to draw on"
    );
    assert!(
        pumping,
        "and it has to register an animation-frame callback, or the loop never advances"
    );
    assert_eq!(
        updates.get(),
        1,
        "one pumped frame drives exactly one fixed update"
    );
}
