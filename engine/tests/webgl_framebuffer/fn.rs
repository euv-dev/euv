use super::*;

const GL_NAMES: &str = r#"[
    "createTexture","bindTexture","texParameteri","texImage2D","deleteTexture",
    "createFramebuffer","bindFramebuffer","framebufferTexture2D","deleteFramebuffer",
    "createRenderbuffer","bindRenderbuffer","renderbufferStorage","deleteRenderbuffer",
    "framebufferRenderbuffer","checkFramebufferStatus","getError","clear",
    "clearColor","clearDepth","viewport","scissor","enable","disable","depthFunc",
    "depthMask","blendFunc","blendEquation","cullFace","frontFace","colorMask",
    "setTransform","getParameter"
]"#;

fn reflective(target: &Object, key: &str, value: &JsValue) {
    let installer: js_sys::Function = js_sys::Function::new_with_args(
        "target, key, value",
        "Object.defineProperty(target, key, { value: value, writable: true, configurable: true });",
    );
    let outcome: Result<JsValue, JsValue> = installer.call3(
        &JsValue::NULL,
        target.as_ref(),
        &JsValue::from_str(key),
        value,
    );
    assert!(outcome.is_ok(), "the stand-in must accept {key}");
}

fn recording_context(complete: bool) -> (WebGl2RenderingContext, Object) {
    let context: Object = Object::new();
    reflective(context.as_ref(), "__log", Array::new().as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, names, complete",
        "for (const name of JSON.parse(names)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); if (name === 'checkFramebufferStatus') return complete ? 0x8cd5 : 0x8cd7; if (name.indexOf('create') === 0) return { __gl: name }; return null; }; }",
    );
    let armed: Result<JsValue, JsValue> = arm.call3(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(GL_NAMES),
        &JsValue::from_bool(complete),
    );
    assert!(
        armed.is_ok(),
        "the framebuffer recorder must install cleanly"
    );
    let typed: WebGl2RenderingContext = context.clone().unchecked_into();
    (typed, context)
}

fn ops(context: &Object) -> Vec<String> {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    (0..log.length())
        .map(|index: u32| {
            let entry: Array = log.get(index).unchecked_into();
            entry.get(0).as_string().unwrap_or_default()
        })
        .collect()
}

fn count_of(context: &Object, op: &str) -> usize {
    ops(context)
        .iter()
        .filter(|recorded: &&String| recorded.as_str() == op)
        .count()
}

fn ops_after(context: &Object, from: usize) -> Vec<String> {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    (from..log.length() as usize)
        .map(|index: usize| {
            let entry: Array = log.get(index as u32).unchecked_into();
            entry.get(0).as_string().unwrap_or_default()
        })
        .collect()
}

fn args_after(context: &Object, op: &str, from: u32) -> Array {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    for index in from..log.length() {
        let entry: Array = log.get(index).unchecked_into();
        if entry.get(0).as_string().as_deref() == Some(op) {
            return entry.get(1).unchecked_into();
        }
    }
    panic!(
        "{op} was never called; recorded ops were {:?}",
        ops(context)
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_colour_only_framebuffer_allocates_one_texture_and_binds_it_as_the_colour_attachment() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true);
    let before: u32 = ops(&log).len() as u32;

    let created: Option<GlFramebuffer> =
        GlFramebuffer::create(&context, 16, 8, GpuTextureFormat::Rgba8Unorm, None);

    assert!(created.is_some(), "the driver handed back a framebuffer");
    let framebuffer: &GlFramebuffer = created.as_ref().expect("created");
    assert_eq!(
        framebuffer.get_width(),
        16,
        "the framebuffer records its width"
    );
    assert_eq!(framebuffer.get_height(), 8, "and its height");
    assert_eq!(
        count_of(&log, "createRenderbuffer"),
        0,
        "no depth buffer was asked for"
    );
    let attached: Array = args_after(&log, "framebufferTexture2D", before);
    assert_eq!(
        attached.get(1).as_f64(),
        Some(0.0),
        "the colour texture is attached at the colour attachment point"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_depth_format_allocates_a_renderbuffer_and_attaches_it_to_the_depth_point() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true);
    let before: u32 = ops(&log).len() as u32;

    let created: Option<GlFramebuffer> = GlFramebuffer::create(
        &context,
        16,
        8,
        GpuTextureFormat::Rgba8Unorm,
        Some(GpuTextureFormat::Depth24Plus),
    );

    assert!(created.is_some(), "the driver handed back a framebuffer");
    assert_eq!(
        count_of(&log, "createRenderbuffer"),
        1,
        "a depth format means one renderbuffer alongside the colour texture"
    );
    let storage: Array = args_after(&log, "renderbufferStorage", before);
    assert_eq!(
        storage.get(2).as_f64(),
        Some(16.0),
        "the depth renderbuffer is allocated at the framebuffer's width"
    );
    assert_eq!(storage.get(3).as_f64(), Some(8.0), "and at its height");
    assert_eq!(
        count_of(&log, "framebufferRenderbuffer"),
        1,
        "the renderbuffer is attached, not merely allocated"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_complete_framebuffer_binds_both_draw_and_read_targets_and_reports_complete() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true);
    let framebuffer: GlFramebuffer =
        GlFramebuffer::create(&context, 16, 8, GpuTextureFormat::Rgba8Unorm, None)
            .expect("created");
    let before: usize = ops(&log).len();

    let status: GlFramebufferStatus = framebuffer.bind(&context);

    assert_eq!(
        status,
        GlFramebufferStatus::Complete,
        "a target with only a colour attachment is complete"
    );
    let fresh: Vec<String> = ops_after(&log, before);
    assert_eq!(
        fresh,
        vec![
            String::from("bindFramebuffer"),
            String::from("bindFramebuffer"),
            String::from("checkFramebufferStatus"),
        ],
        "both draw and read targets are bound, because binding only one makes readPixels \
         silently return the default framebuffer"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn an_incomplete_framebuffer_is_reported_as_incomplete_on_bind() {
    let (context, _log): (WebGl2RenderingContext, Object) = recording_context(false);
    let framebuffer: GlFramebuffer =
        GlFramebuffer::create(&context, 16, 8, GpuTextureFormat::Rgba8Unorm, None)
            .expect("created");

    let status: GlFramebufferStatus = framebuffer.bind(&context);

    assert_eq!(
        status,
        GlFramebufferStatus::Incomplete,
        "the driver said the target is not renderable, and that has to reach the caller instead \
         of being swallowed into a draw that renders nothing"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn an_unchanged_size_matches_so_a_resize_check_can_skip_reallocating() {
    let (context, _log): (WebGl2RenderingContext, Object) = recording_context(true);
    let framebuffer: GlFramebuffer =
        GlFramebuffer::create(&context, 16, 8, GpuTextureFormat::Rgba8Unorm, None)
            .expect("created");

    assert!(
        framebuffer.matches(16, 8),
        "the size it was allocated at is the size it still has"
    );
    assert!(
        !framebuffer.matches(16, 9),
        "a one pixel change must not match, or the attachments would never be rebuilt"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn deleting_a_framebuffer_releases_both_attachments_and_zeroes_its_size() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true);
    let mut framebuffer: GlFramebuffer = GlFramebuffer::create(
        &context,
        16,
        8,
        GpuTextureFormat::Rgba8Unorm,
        Some(GpuTextureFormat::Depth24Plus),
    )
    .expect("created");

    framebuffer.delete(&context);

    assert_eq!(
        framebuffer.get_width(),
        0,
        "a deleted framebuffer keeps no stale size, or a later bind would size the viewport wrong"
    );
    assert_eq!(framebuffer.get_height(), 0, "in both axes");
    assert_eq!(
        count_of(&log, "deleteRenderbuffer"),
        1,
        "the depth renderbuffer goes too"
    );
    assert_eq!(
        count_of(&log, "deleteFramebuffer"),
        1,
        "as does the framebuffer itself"
    );
}
