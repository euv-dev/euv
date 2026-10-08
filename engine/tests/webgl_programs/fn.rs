use super::*;

const CTX_METHODS: &str = r#"[
    "createShader","shaderSource","compileShader","getShaderParameter",
    "getShaderInfoLog","deleteShader","createProgram","attachShader",
    "linkProgram","getProgramParameter","getProgramInfoLog","deleteProgram",
    "getUniformLocation","uniform1f","uniform2f","uniform3f","uniform4f",
    "uniformMatrix4fv","uniform4fv","bindBuffer","bufferData","deleteBuffer"
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

fn recording_context(linked: bool, compiled: bool) -> (WebGl2RenderingContext, Object) {
    let context: Object = Object::new();
    reflective(context.as_ref(), "__log", Array::new().as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, methods, linked, compiled",
        "for (const name of JSON.parse(methods)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); if (name === 'createShader') return { __gl: 'shader' }; if (name === 'createProgram') return { __gl: 'program' }; if (name === 'getShaderParameter') return compiled; if (name === 'getProgramParameter') return linked; if (name === 'getUniformLocation') return { __gl: 'location' }; if (name.indexOf('InfoLog') >= 0) return 'driver says no'; return null; }; }",
    );
    let armed: Result<JsValue, JsValue> = arm.call4(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(CTX_METHODS),
        &JsValue::from_bool(linked),
        &JsValue::from_bool(compiled),
    );
    assert!(armed.is_ok(), "the program recorder must install cleanly");
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

fn args_at(context: &Object, op: &str, nth: usize) -> Array {
    nth_of(context, op, nth)
}

fn nth_of(context: &Object, op: &str, nth: usize) -> Array {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    let mut seen: usize = 0;
    for index in 0..log.length() {
        let entry: Array = log.get(index).unchecked_into();
        if entry.get(0).as_string().as_deref() == Some(op) {
            if seen == nth {
                return entry.get(1).unchecked_into();
            }
            seen += 1;
        }
    }
    panic!(
        "{op} call #{nth} was never made; recorded ops were {:?}",
        ops(context)
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_linked_program_compiles_both_stages_attaches_them_and_frees_the_shaders() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true, true);

    let program: Result<GlProgram, WebGlProgramError> =
        GlProgram::create(&context, "vertex body", "fragment body");

    assert!(
        program.is_ok(),
        "a shader that compiles and links produces a program"
    );
    assert_eq!(
        count_of(&log, "createShader"),
        2,
        "a vertex stage and a fragment stage, so two shader objects"
    );
    assert_eq!(
        count_of(&log, "attachShader"),
        2,
        "both stages are attached to the program"
    );
    assert_eq!(
        count_of(&log, "linkProgram"),
        1,
        "linking happens exactly once"
    );
    assert_eq!(
        count_of(&log, "deleteShader"),
        2,
        "the shader objects are intermediate products and are freed once linked"
    );
    assert_eq!(
        count_of(&log, "deleteProgram"),
        0,
        "a successful program must survive for later draws"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn the_shader_source_reaches_the_driver_verbatim_for_each_stage() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true, true);

    GlProgram::create(&context, "vertex body", "fragment body").expect("linked");

    assert_eq!(
        args_at(&log, "shaderSource", 0).get(1).as_string(),
        Some(String::from("vertex body")),
        "the vertex source is forwarded to the shader object unchanged"
    );
    assert_eq!(
        args_at(&log, "shaderSource", 1).get(1).as_string(),
        Some(String::from("fragment body")),
        "the fragment source is forwarded to its own shader object unchanged"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_failed_link_returns_the_driver_log_and_frees_the_program() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(false, true);

    let outcome: Result<GlProgram, WebGlProgramError> =
        GlProgram::create(&context, "vertex body", "fragment body");

    match outcome {
        Err(WebGlProgramError::ProgramLink(log_text)) => assert_eq!(
            log_text,
            String::from("driver says no"),
            "a GLSL diagnostic is only actionable in the driver's own wording"
        ),
        Err(other) => panic!("a failed link must report ProgramLink, got {other:?}"),
        Ok(_) => panic!("a driver that refuses to link must not yield a program"),
    }
    assert_eq!(
        count_of(&log, "deleteProgram"),
        1,
        "a program that failed to link is unusable and has to be released"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_shader_that_fails_to_compile_stops_before_the_program_is_ever_created() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true, false);

    let outcome: Result<GlProgram, WebGlProgramError> =
        GlProgram::create(&context, "broken vertex", "fragment body");

    match outcome {
        Err(WebGlProgramError::ShaderCompile(log_text)) => assert_eq!(
            log_text,
            String::from("driver says no"),
            "the compile log is what the caller needs, not a generic failure"
        ),
        Err(other) => panic!("a failed compile must report ShaderCompile, got {other:?}"),
        Ok(_) => panic!("a shader the driver refused must not yield a program"),
    }
    assert_eq!(
        count_of(&log, "createProgram"),
        0,
        "a failed compile must not reach program creation, or the fragment stage would leak"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_uniform_location_is_resolved_once_and_then_served_from_cache() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true, true);
    let mut program: GlProgram =
        GlProgram::create(&context, "vertex body", "fragment body").expect("linked");
    let after_link: usize = count_of(&log, "getUniformLocation");

    program.set_uniform_1f(&context, "u_alpha", 0.5);
    program.set_uniform_1f(&context, "u_alpha", 0.75);

    assert_eq!(
        count_of(&log, "getUniformLocation"),
        after_link + 1,
        "a repeated name is served from the cache instead of a second lookup"
    );
    assert_eq!(
        count_of(&log, "uniform1f"),
        2,
        "each upload still reaches the driver, cache or not"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn each_uniform_width_forwards_exactly_its_own_component_count() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true, true);
    let mut program: GlProgram =
        GlProgram::create(&context, "vertex body", "fragment body").expect("linked");

    program.set_uniform_1f(&context, "a", 1.0);
    program.set_uniform_2f(&context, "b", 1.0, 2.0);
    program.set_uniform_3f(&context, "c", 1.0, 2.0, 3.0);
    program.set_uniform_4f(&context, "d", 1.0, 2.0, 3.0, 4.0);

    assert_eq!(
        args_at(&log, "uniform1f", 0).length(),
        2,
        "uniform1f passes the location then the single value"
    );
    assert_eq!(
        args_at(&log, "uniform2f", 0).length(),
        3,
        "uniform2f passes the location then x and y, not a third component"
    );
    assert_eq!(
        args_at(&log, "uniform3f", 0).length(),
        4,
        "uniform3f passes the location then x, y and z"
    );
    assert_eq!(
        args_at(&log, "uniform4f", 0).length(),
        5,
        "uniform4f passes the location then the whole vector including w"
    );
    assert_eq!(
        args_at(&log, "uniform2f", 0).get(2).as_f64(),
        Some(2.0),
        "the second component is y, so it has to arrive second after the location"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_matrix_upload_passes_sixteen_lanes_untransposed() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true, true);
    let mut program: GlProgram =
        GlProgram::create(&context, "vertex body", "fragment body").expect("linked");
    let matrix: Matrix4x4 = Matrix4x4::identity();

    program.set_uniform_mat4(&context, "u_model", &matrix);

    let upload: Array = args_at(&log, "uniformMatrix4fv", 0);
    assert_eq!(
        upload.get(1).as_bool(),
        Some(false),
        "the transpose flag is false, so the engine matrix goes across in the order GL expects"
    );
    let lanes: Array = upload.get(2).unchecked_into();
    assert_eq!(
        lanes.length(),
        16,
        "a mat4 is sixteen floats, converted into a fixed stack array rather than a heap Vec"
    );
    let observed: Vec<f64> = (0..lanes.length())
        .map(|index: u32| lanes.get(index).as_f64().unwrap_or(-1.0))
        .collect();
    assert_eq!(
        observed.len(),
        16,
        "the engine's own Matrix4x4 is sixteen elements, so all of them are converted"
    );
    assert!(
        observed.contains(&1.0),
        "an identity matrix carries ones, so the f64 to f32 conversion preserved a value: {observed:?}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_vec4_array_upload_hands_the_callers_slice_straight_to_the_driver() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true, true);
    let mut program: GlProgram =
        GlProgram::create(&context, "vertex body", "fragment body").expect("linked");
    let data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

    program.set_uniform_vec4_array(&context, "u_offsets[0]", &data);

    let upload: Array = args_at(&log, "uniform4fv", 0);
    assert!(
        !upload.get(0).is_undefined(),
        "the resolved location is what makes the upload land on the right uniform"
    );
    let forwarded: Array = upload.get(1).unchecked_into();
    assert_eq!(
        forwarded.length(),
        8,
        "the slice is handed over whole; two packed vec4 elements is eight floats"
    );
    assert_eq!(
        forwarded.get(7).as_f64(),
        Some(8.0),
        "the tail of the caller slice reaches the driver, so nothing is truncated on the way"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn deleting_a_program_releases_the_handle() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(true, true);
    let mut program: GlProgram =
        GlProgram::create(&context, "vertex body", "fragment body").expect("linked");
    let before: usize = count_of(&log, "deleteProgram");

    program.delete(&context);

    assert_eq!(
        count_of(&log, "deleteProgram"),
        before + 1,
        "the driver handle is released exactly once"
    );
}
