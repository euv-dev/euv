use super::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_canvas_without_a_window_falls_back_to_a_device_pixel_ratio_of_one() {
    let observed: f64 = CanvasRenderer::detect_dpr();
    assert_eq!(
        observed, 1.0,
        "with no window there is no device pixel ratio to read, so 1.0 is the neutral default"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_default_font_stays_parseable_without_a_browser() {
    let observed: String = CanvasRenderer::default_font();
    assert_eq!(
        observed, "16px sans-serif",
        "font construction is pure string building and must not depend on a canvas"
    );
}

fn mouse_event_with_button(value: f64) -> Event {
    let event: Event = Event::new("mousedown").expect("an event");
    let set: Result<bool, JsValue> = js_sys::Reflect::set(
        &event,
        &JsValue::from_str("button"),
        &JsValue::from_f64(value),
    );
    assert!(set.expect("reflect set"), "the button property must be set");
    event
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn every_mouse_button_number_maps_to_its_own_variant() {
    let pairs: [(f64, MouseButton); 5] = [
        (0.0, MouseButton::Left),
        (1.0, MouseButton::Middle),
        (2.0, MouseButton::Right),
        (3.0, MouseButton::Button4),
        (4.0, MouseButton::Button5),
    ];
    for (value, expected) in pairs {
        let observed: MouseButton = Input::extract_mouse_button(&mouse_event_with_button(value));
        assert_eq!(
            observed, expected,
            "button {value} mapped to the wrong variant"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_button_number_outside_the_table_falls_back_to_left() {
    let observed: MouseButton = Input::extract_mouse_button(&mouse_event_with_button(9.0));
    assert_eq!(
        observed,
        MouseButton::Left,
        "an unknown button must not invent a variant, and left is the documented default"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn mouse_coordinates_come_back_as_the_pair_that_was_set() {
    let event: Event = Event::new("mousemove").expect("an event");
    for (name, value) in [("clientX", 128.0), ("clientY", -47.0)] {
        let set: Result<bool, JsValue> =
            js_sys::Reflect::set(&event, &JsValue::from_str(name), &JsValue::from_f64(value));
        assert!(set.expect("reflect set"), "the {name} property must be set");
    }
    let observed: Vector2D = Input::extract_mouse_position(&event);
    assert_eq!(
        observed.get_x(),
        128.0,
        "x must not be swapped with y, got {observed:?}"
    );
    assert_eq!(
        observed.get_y(),
        -47.0,
        "a negative y is a real position above the viewport, not something to clamp"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_key_code_is_read_verbatim_from_the_event() {
    let event: Event = Event::new("keydown").expect("an event");
    let set: Result<bool, JsValue> = js_sys::Reflect::set(
        &event,
        &JsValue::from_str("code"),
        &JsValue::from_str("ArrowLeft"),
    );
    assert!(set.expect("reflect set"), "the code property must be set");
    let observed: String = Input::extract_key_code(&event);
    assert_eq!(
        observed, "ArrowLeft",
        "the key code is passed through untouched, not normalised or defaulted"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_2d_texture_write_keeps_the_caller_values_and_defaults_the_rest() {
    let payload: Vec<u8> = vec![1, 2, 3, 4];
    let target: JsValue = JsValue::from_str("tex");
    let built: TextureWriteDescriptor =
        TextureWriteDescriptor::for_2d(payload.clone(), 256, target.clone());
    assert_eq!(
        built.get_data(),
        payload,
        "the pixel bytes must come through untouched"
    );
    assert_eq!(
        built.get_bytes_per_row(),
        256,
        "the row stride is the caller choice"
    );
    assert_eq!(
        built.get_texture(),
        target,
        "the destination handle is the caller choice"
    );
    assert_eq!(
        built.get_mip_level(),
        0,
        "a 2d convenience upload targets the base mip level"
    );
    assert_eq!(
        built.get_rows_per_image(),
        0,
        "0 rows-per-image is the spec value for a 2d texture without a mip chain"
    );
    assert!(
        !built.get_flip_y(),
        "packed bytes are already in texture order, so flipping would invert them"
    );
    assert!(
        built.get_origin().is_none(),
        "no origin means the whole texture, which is what None defaults to"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_2d_texture_write_accepts_an_empty_payload() {
    let built: TextureWriteDescriptor =
        TextureWriteDescriptor::for_2d(Vec::new(), 0, JsValue::NULL);
    assert!(
        built.get_data().is_empty(),
        "an empty upload is legal, so the constructor must not require bytes"
    );
    assert_eq!(
        built.get_bytes_per_row(),
        0,
        "the stride is passed through even when there is no data to stride over"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_default_blend_state_leaves_blending_off() {
    let blend: GlBlendState = GlBlendState::default();
    assert!(
        !blend.get_enabled(),
        "a fresh context draws opaque, so BLEND must start disabled"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_default_cull_state_keeps_both_faces() {
    let cull: GlCullState = GlCullState::default();
    assert_eq!(
        *cull.get_mode(),
        CullMode::None,
        "2d sprites and double-sided materials must not have a face discarded"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_default_cull_state_counts_counter_clockwise_as_front() {
    let cull: GlCullState = GlCullState::default();
    assert_eq!(
        *cull.get_front_face(),
        FrontFace::CounterClockwise,
        "the winding convention is what front and back mean, so its default must be pinned"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_default_render_state_has_no_scissor_and_no_blend() {
    let state: GlRenderState = GlRenderState::context_defaults(320, 240);
    assert!(
        state.get_scissor().is_none(),
        "a zero-sized scissor clips everything, so off has to be the absence of one"
    );
    assert!(
        !state.get_blend().get_enabled(),
        "the blend half of a fresh state must agree with the standalone default"
    );
    assert_eq!(
        *state.get_cull().get_mode(),
        CullMode::None,
        "the cull default reached through context_defaults must match the standalone one"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reaches the no-DOM fallback only under wasm"
)]
fn a_canvas_renderer_without_a_document_reports_no_backend() {
    let built: Option<CanvasRenderer> = Engine::canvas_renderer(&RenderConfig::default());
    assert!(
        built.is_none(),
        "there is no canvas element to attach to, so the Option must stay empty"
    );
}

#[test]
fn nine_slice_insets_split_a_square_into_nine_tiles() {
    let insets: NineSliceInsets = NineSliceInsets::new(1.0, 1.0, 1.0, 1.0);
    let rects: NineSliceRects = insets.source_rects(Rect::new(0.0, 0.0, 10.0, 10.0));
    let grid: [[Rect; 3]; 3] = rects.get_grid();
    assert_eq!(
        (
            grid[1][1].get_x(),
            grid[1][1].get_y(),
            grid[1][1].get_width(),
            grid[1][1].get_height()
        ),
        (1.0, 1.0, 8.0, 8.0),
        "the centre tile starts after both insets and keeps what remains"
    );
    assert_eq!(
        (grid[0][0].get_width(), grid[0][0].get_height()),
        (1.0, 1.0),
        "the top-left tile is exactly the left/top inset"
    );
}

#[test]
fn nine_slice_insets_clamp_to_the_source_width() {
    let insets: NineSliceInsets = NineSliceInsets::new(20.0, 20.0, 0.0, 0.0);
    let rects: NineSliceRects = insets.source_rects(Rect::new(0.0, 0.0, 10.0, 10.0));
    let grid: [[Rect; 3]; 3] = rects.get_grid();
    let centre_width: f64 = grid[1][1].get_width();
    assert!(
        centre_width >= 0.0,
        "insets larger than the source must be clamped, not produce a negative centre"
    );
}

#[test]
fn a_point_light_has_no_direction_and_unit_falloff() {
    let light: Light = Light::new_point(
        Vector3D::new(1.0, 2.0, 3.0),
        Vector3D::new(0.5, 0.5, 0.5),
        2.0,
    );
    assert!(
        light.get_kind() == LightType::Point,
        "a point light must report itself as one"
    );
    assert_eq!(
        light.get_position(),
        Vector3D::new(1.0, 2.0, 3.0),
        "the position is kept verbatim"
    );
    assert_eq!(
        light.get_direction(),
        Vector3D::zero(),
        "a point light radiates in every direction, so it carries none"
    );
    assert_eq!(light.get_intensity(), 2.0, "the scalar passes through");
}

#[test]
fn a_spot_light_normalises_its_direction() {
    let light: Light = Light::new_spot(
        Vector3D::zero(),
        Vector3D::new(0.0, 5.0, 0.0),
        Vector3D::new(1.0, 1.0, 1.0),
        1.0,
        consts::FRAC_PI_4,
    );
    let dir: Vector3D = light.get_direction();
    assert!(
        (dir.magnitude() - 1.0).abs() < 1e-9,
        "the stored direction must be a unit vector, got magnitude {}",
        dir.magnitude()
    );
    assert!(
        dir.get_y() > 0.99,
        "a (0,5,0) direction must still point along +y after normalising"
    );
    assert!(
        (light.get_spot_cos() - consts::FRAC_1_SQRT_2).abs() < 1e-9,
        "spot_cos must cache cos(half_angle) = cos(pi/4), got {}",
        light.get_spot_cos()
    );
}

#[test]
fn a_default_gamepad_reports_nothing_connected_or_pressed() {
    let state: GamepadState = GamepadState::default();
    assert!(
        state.button_value(0) == 0.0,
        "a fresh pad reads no pressure on any button"
    );
    assert!(!state.is_button_pressed(0), "nothing went down yet");
    assert!(!state.is_button_held(0), "nothing is down yet");
    assert!(!state.is_button_released(0), "nothing came up yet");
    assert!(
        !state.axis_pressed(0, 0.5),
        "a zeroed axis never crosses a positive threshold"
    );
}

#[test]
fn a_button_beyond_the_pad_array_reads_as_zero_rather_than_panicking() {
    let state: GamepadState = GamepadState::default();
    assert_eq!(
        state.button_value(99),
        0.0,
        "asking about a button the pad does not have must read as released"
    );
    assert!(
        !state.axis_pressed(99, 0.5),
        "and the same for an axis that does not exist"
    );
}

#[test]
fn an_empty_scene_lets_every_ray_reach_the_ambient_light() {
    let scene: RayTraceScene = RayTraceScene::new(Vec::new());
    let lights: LightingUniforms =
        LightingUniforms::new(Vec::new(), Vector3D::new(0.2, 0.2, 0.2), Vector3D::zero());
    let hit: Vector3D = scene.trace_with_bounces(
        Ray::new(Vector3D::zero(), Vector3D::new(0.0, 1.0, 0.0)),
        &lights,
        2,
    );
    assert_eq!(
        hit,
        Vector3D::new(0.2, 0.2, 0.2),
        "nothing blocks the ray, so it must come back as the ambient term unchanged"
    );
}

#[test]
fn a_gamepad_manager_with_no_pads_counts_nothing_as_connected() {
    let manager: GamepadManager = GamepadManager::default();
    assert_eq!(
        manager.connected_count(),
        0,
        "a fresh manager has seen no pads"
    );
    assert!(
        !manager.is_connected(0),
        "and index 0 is certainly not among them"
    );
}

#[test]
fn a_button_with_no_edge_at_all_reads_as_idle() {
    let state: GamepadState = GamepadState::default();
    assert_eq!(
        state.button_action(0),
        InputAction::Idle,
        "a pad that reports nothing must not invent an action"
    );
}

#[test]
fn a_manager_with_no_pad_at_that_index_reads_as_idle() {
    let manager: GamepadManager = GamepadManager::default();
    assert_eq!(
        manager.button_action(0, 0),
        InputAction::Idle,
        "asking about a pad index the manager never saw must be idle, not a button press"
    );
}

#[test]
fn a_uniform_slice_of_zero_size_means_the_rest_of_the_range() {
    let open_ended: UniformSlice = UniformSlice::new(256, 0);
    assert_eq!(
        open_ended.get_offset(),
        256,
        "the offset positions the record inside the bound range"
    );
    assert_eq!(
        open_ended.get_size(),
        0,
        "size zero is the sentinel for 'to the end', not an empty record"
    );
    let bounded: UniformSlice = UniformSlice::new(0, 64);
    assert_eq!(
        bounded.get_size(),
        64,
        "an explicit size must survive construction untouched"
    );
}

#[test]
fn a_uniform_slice_keeps_the_two_numbers_from_being_transposed() {
    let a: UniformSlice = UniformSlice::new(16, 32);
    let b: UniformSlice = UniformSlice::new(32, 16);
    assert_eq!(
        (a.get_offset(), a.get_size()),
        (16, 32),
        "offset and size must land in their own slots, not be transposed"
    );
    assert_eq!(
        (b.get_offset(), b.get_size()),
        (32, 16),
        "and the transposed constructor must actually transpose them"
    );
}

#[test]
fn a_texture_descriptor_leaves_the_skipped_fields_at_their_defaults() {
    let descriptor: TextureDescriptor =
        TextureDescriptor::new(64, 32, "2d", GpuTextureFormat::Rgba8Unorm, 0x0f);
    assert_eq!(descriptor.get_width(), 64, "width comes from the caller");
    assert_eq!(descriptor.get_height(), 32, "height comes from the caller");
    assert_eq!(
        descriptor.get_usage(),
        0x0f,
        "the usage bitmask passes through"
    );
    assert_eq!(
        descriptor.get_mip_level_count(),
        0,
        "zero is the one-mip-level sentinel; a real count would be written afterwards"
    );
    assert_eq!(
        descriptor.get_sample_count(),
        0,
        "an ordinary texture is not multisampled until the caller says so"
    );
    assert_eq!(
        descriptor.get_depth_or_layers(),
        0,
        "a plain 2d texture has a single layer"
    );
    assert!(
        descriptor.get_label().is_none(),
        "an unnamed texture must not carry a label"
    );
}

#[test]
fn a_default_framebuffer_status_is_complete() {
    let status: GlFramebufferStatus = GlFramebufferStatus::default();
    assert_eq!(
        status,
        GlFramebufferStatus::Complete,
        "a zero-initialised framebuffer has no attachment problem to report"
    );
    assert_ne!(
        status,
        GlFramebufferStatus::Incomplete,
        "defaulting to Incomplete would make every fresh framebuffer look broken"
    );
}

#[test]
fn program_errors_carry_the_driver_message_verbatim() {
    let compile: WebGlProgramError =
        WebGlProgramError::ShaderCompile(String::from("undeclared id"));
    let link: WebGlProgramError = WebGlProgramError::ProgramLink(String::from("varying mismatch"));
    assert_eq!(
        format!("{compile:?}"),
        r#"ShaderCompile("undeclared id")"#,
        "the GLSL diagnostic must survive verbatim, with no wrapping or truncation"
    );
    assert_eq!(
        format!("{link:?}"),
        r#"ProgramLink("varying mismatch")"#,
        "a link failure is a different stage and must read differently from a compile one"
    );
}

struct CountingShape {
    position: Vector2D,
    size: f64,
}

impl Renderable for CountingShape {
    fn draw(&self, draw_list: &mut DrawList, transform: &Transform2D) {
        let placed: Vector2D = transform.apply_to_point(self.position);
        draw_list.fill_rect(placed, self.size, self.size, Color::from_rgb(255, 255, 255));
    }
}

#[test]
fn a_fresh_draw_list_records_nothing() {
    let list: DrawList = DrawList::create();
    assert_eq!(list.len(), 0, "a new frame starts with no commands");
    assert!(list.is_empty(), "and must report itself as empty");
}

#[test]
fn drawing_through_the_renderable_trait_appends_exactly_one_command() {
    let mut list: DrawList = DrawList::create();
    let shape: CountingShape = CountingShape {
        position: Vector2D::new(3.0, 4.0),
        size: 5.0,
    };
    shape.draw(&mut list, &Transform2D::identity());
    assert_eq!(
        list.len(),
        1,
        "one draw call must append exactly one command"
    );
    assert!(
        matches!(list.commands().first(), Some(DrawCommand::FillRect { .. })),
        "and it must be the fill the implementation asked for, got {:?}",
        list.commands().first()
    );
}

#[test]
fn clearing_a_draw_list_drops_the_previous_frame() {
    let mut list: DrawList = DrawList::create();
    let shape: CountingShape = CountingShape {
        position: Vector2D::zero(),
        size: 1.0,
    };
    shape.draw(&mut list, &Transform2D::identity());
    shape.draw(&mut list, &Transform2D::identity());
    assert_eq!(list.len(), 2, "two draws accumulate within a frame");
    list.clear();
    assert_eq!(
        list.len(),
        0,
        "clear must empty the list so the next frame does not replay old commands"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "needs a wasm realm: JsValue is wasm32-only"
)]
fn a_compute_pipeline_descriptor_keeps_its_module_and_entry_point_apart() {
    let module: JsValue = JsValue::from_str("wgsl-module");
    let descriptor: ComputePipelineDescriptor =
        ComputePipelineDescriptor::new(module.clone(), String::from("main"));
    assert_eq!(
        descriptor.get_module(),
        module,
        "the compiled module handle must survive construction untouched"
    );
    assert_eq!(
        descriptor.get_entry_point(),
        "main",
        "the entry point is a name inside the module, not the module itself"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "needs a wasm realm: JsValue is wasm32-only"
)]
fn a_buffer_binding_with_no_explicit_size_runs_to_the_end_of_the_buffer() {
    let open_ended: BindGroupEntry = BindGroupEntry::Buffer {
        binding: 3,
        buffer: JsValue::from_str("buf"),
        offset: 64,
        size: None,
    };
    match open_ended {
        BindGroupEntry::Buffer {
            binding,
            offset,
            size,
            ..
        } => {
            assert_eq!(
                binding, 3,
                "the binding slot matches what the shader declared"
            );
            assert_eq!(offset, 64, "the byte offset is preserved");
            assert_eq!(size, None, "a None size means from the offset to the end");
        }
        _ => panic!("a buffer binding must stay a buffer binding"),
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "needs a wasm realm: JsValue is wasm32-only"
)]
fn a_read_only_storage_binding_is_not_the_same_as_a_read_write_one() {
    let read_only: BindGroupEntry = BindGroupEntry::StorageTexture {
        binding: 0,
        view: JsValue::NULL,
        read_only: true,
    };
    let read_write: BindGroupEntry = BindGroupEntry::StorageTexture {
        binding: 0,
        view: JsValue::NULL,
        read_only: false,
    };
    assert_ne!(
        format!("{read_only:?}"),
        format!("{read_write:?}"),
        "read-only and read-write storage textures share a slot but grant different \
         shader permissions; collapsing them would silently widen access"
    );
}

#[test]
fn a_pad_with_no_axes_reports_zero_for_every_index_including_beyond_its_list() {
    let state: GamepadState = GamepadState::default();

    assert_eq!(
        state.axis(0),
        0.0,
        "a fresh pad reports no deflection on axis zero"
    );
    assert_eq!(
        state.axis(99),
        0.0,
        "and an axis index past the end reads as zero rather than panicking; the browser reports \
         however many axes the device actually has, and asking about one that is not there is a \
         programming error at worst, not a crash"
    );
}
