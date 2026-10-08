use super::*;

const EPSILON: f64 = 1e-9;
fn point(x: f64, y: f64) -> Vector2D {
    Vector2D::new(x, y)
}

#[test]
fn a_default_camera_sits_at_the_origin_with_no_rotation() {
    let camera: Camera2D = Camera2D::default();
    let rendered: String = format!("{camera:?}");
    assert!(
        rendered.contains("position: Vector2D { x: 0.0, y: 0.0 }"),
        "a fresh camera looks at the world origin, got: {rendered}"
    );
    assert!(
        rendered.contains("rotation: 0.0"),
        "a fresh camera is unrotated, got: {rendered}"
    );
}

#[test]
fn the_world_origin_lands_at_the_centre_of_the_viewport() {
    let camera: Camera2D = Camera2D::create(800.0, 600.0);
    let screen: Vector2D = camera.world_to_screen(Vector2D::zero());
    assert!(
        (screen.get_x() - 400.0).abs() < EPSILON,
        "the world origin is the middle of an 800-wide viewport, got {}",
        screen.get_x()
    );
    assert!(
        (screen.get_y() - 300.0).abs() < EPSILON,
        "and the middle of a 600-tall one, got {}",
        screen.get_y()
    );
}

#[test]
fn the_transform_round_trips_through_both_directions() {
    let camera: Camera2D = Camera2D::create(800.0, 600.0);
    for target in [point(0.0, 0.0), point(10.0, -20.0), point(-300.0, 250.0)] {
        let there: Vector2D = camera.world_to_screen(target);
        let back: Vector2D = camera.screen_to_world(there);
        assert!(
            (back.get_x() - target.get_x()).abs() < EPSILON
                && (back.get_y() - target.get_y()).abs() < EPSILON,
            "{target:?} came back as {back:?}"
        );
    }
}

#[test]
fn a_translated_camera_shifts_what_it_looks_at() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    camera.translate(point(100.0, 50.0));
    let rendered: String = format!("{camera:?}");
    assert!(
        rendered.contains("position: Vector2D { x: 100.0, y: 50.0 }"),
        "translation accumulates on the position, got: {rendered}"
    );
    let screen: Vector2D = camera.world_to_screen(point(100.0, 50.0));
    assert!(
        (screen.get_x() - 400.0).abs() < EPSILON && (screen.get_y() - 300.0).abs() < EPSILON,
        "the point the camera now sits on must land dead centre, got {screen:?}"
    );
}

#[test]
fn translations_accumulate_rather_than_replace() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    camera.translate(point(10.0, 0.0));
    camera.translate(point(5.0, 0.0));
    assert!(
        format!("{camera:?}").contains("position: Vector2D { x: 15.0, y: 0.0 }"),
        "two steps of 10 and 5 must land at 15, not at 5"
    );
}

#[test]
fn zooming_in_magnifies_offsets_from_the_centre() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    let before: Vector2D = camera.world_to_screen(point(100.0, 0.0));
    camera.zoom_by(2.0);
    let after: Vector2D = camera.world_to_screen(point(100.0, 0.0));
    assert!(
        (after.get_x() - before.get_x() - 100.0).abs() < EPSILON,
        "doubling the zoom must double the 100px offset: {} then {}",
        before.get_x(),
        after.get_x()
    );
    assert!(
        (after.get_y() - 300.0).abs() < EPSILON,
        "a point on the x axis must not drift vertically, got {}",
        after.get_y()
    );
}

#[test]
fn zooming_to_zero_is_clamped_so_the_projection_stays_finite() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    camera.zoom_by(0.0);
    let rendered: String = format!("{camera:?}");
    assert!(
        !rendered.contains("zoom: 0.0"),
        "a zero zoom would divide by zero on the way back, got: {rendered}"
    );
    let screen: Vector2D = camera.world_to_screen(point(10.0, 0.0));
    assert!(
        screen.get_x().is_finite(),
        "the clamped zoom must still produce a finite projection, got {}",
        screen.get_x()
    );
}

#[test]
fn every_webgl_init_error_carries_its_own_stable_code() {
    let cases: [(&str, WebGl2InitError); 5] = [
        (
            "WEBGL_CANVAS_NOT_FOUND",
            WebGl2InitError::CanvasNotFound(String::new()),
        ),
        (
            "WEBGL_CANVAS_QUERY",
            WebGl2InitError::CanvasQuery(String::new()),
        ),
        (
            "WEBGL_CONTEXT_UNAVAILABLE",
            WebGl2InitError::ContextUnavailable,
        ),
        (
            "WEBGL_CONTEXT_LOOKUP",
            WebGl2InitError::ContextLookup(String::new()),
        ),
        ("WEBGL_CONTEXT_CAST", WebGl2InitError::ContextCast),
    ];
    let mut seen: Vec<&str> = Vec::new();
    for (expected, error) in cases {
        assert_eq!(
            error.code(),
            expected,
            "the code is written into logs and telemetry, so it is a public contract"
        );
        seen.push(error.code());
    }
    for i in 0..seen.len() {
        for j in (i + 1)..seen.len() {
            assert_ne!(
                seen[i], seen[j],
                "two failures sharing a code cannot be told apart"
            );
        }
    }
}

#[test]
fn an_init_error_code_ignores_the_message_it_carries() {
    let bare: WebGl2InitError = WebGl2InitError::CanvasNotFound(String::new());
    let detailed: WebGl2InitError =
        WebGl2InitError::CanvasNotFound(String::from("#app canvas missing"));
    assert_eq!(
        bare.code(),
        detailed.code(),
        "the code classifies the failure; the message is free-form detail"
    );
    assert!(
        format!("{detailed}").contains("#app canvas missing"),
        "the Display form must surface the selector the developer passed in"
    );
}

fn red() -> Color {
    Color::new(1.0, 0.0, 0.0, 1.0)
}

fn translucent_blue() -> Color {
    Color::new(0.0, 0.0, 1.0, 0.5)
}

#[test]
fn a_fresh_draw_list_is_empty() {
    let list: DrawList = DrawList::create();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert!(list.commands().is_empty());
}

#[test]
fn recorded_commands_come_back_in_recording_order() {
    let mut list: DrawList = DrawList::create();
    list.fill_rect(Vector2D::new(0.0, 0.0), 1.0, 1.0, red());
    list.fill_circle(Vector2D::new(5.0, 5.0), 2.0, red());
    list.draw_line(Vector2D::zero(), Vector2D::new(9.0, 9.0), red(), 1.0);
    assert_eq!(list.len(), 3, "three recordings, three commands");
    let rendered: Vec<String> = list
        .commands()
        .iter()
        .map(|command: &DrawCommand| {
            format!("{command:?}")
                .split('{')
                .next()
                .unwrap_or("")
                .trim()
                .to_string()
        })
        .collect();
    assert_eq!(
        rendered,
        vec!["FillRect", "FillCircle", "Line"],
        "replay must see the commands in the order the frame drew them"
    );
}

#[test]
fn clearing_a_draw_list_empties_it_but_leaves_it_usable() {
    let mut list: DrawList = DrawList::create();
    list.fill_rect(Vector2D::zero(), 1.0, 1.0, red());
    list.fill_rect(Vector2D::zero(), 1.0, 1.0, red());
    list.clear();
    assert!(
        list.is_empty(),
        "a fresh frame must not replay last frame's commands"
    );
    list.fill_rect(Vector2D::zero(), 2.0, 2.0, red());
    assert_eq!(list.len(), 1, "the list stays usable after a clear");
}

#[test]
fn a_fill_rect_records_its_geometry_verbatim() {
    let mut list: DrawList = DrawList::create();
    list.fill_rect(Vector2D::new(1.0, 2.0), 30.0, 40.0, red());
    let rendered: String = format!("{:?}", list.commands()[0]);
    assert!(rendered.contains("width: 30.0"), "got: {rendered}");
    assert!(rendered.contains("height: 40.0"), "got: {rendered}");
    assert!(
        rendered.contains("x: 1.0"),
        "the position must survive into the command, got: {rendered}"
    );
}

#[test]
fn a_stroke_rect_carries_its_line_width() {
    let mut list: DrawList = DrawList::create();
    list.stroke_rect(Vector2D::zero(), 10.0, 10.0, red(), 2.5);
    let rendered: String = format!("{:?}", list.commands()[0]);
    assert!(rendered.contains("StrokeRect"), "got: {rendered}");
    assert!(
        rendered.contains("line_width: 2.5"),
        "a stroke with the wrong width is a different drawing, got: {rendered}"
    );
}

#[test]
fn a_line_records_both_ends() {
    let mut list: DrawList = DrawList::create();
    list.draw_line(Vector2D::new(0.0, 0.0), Vector2D::new(7.0, 9.0), red(), 1.0);
    let rendered: String = format!("{:?}", list.commands()[0]);
    assert!(
        rendered.contains("x: 7.0"),
        "the end point must survive, got: {rendered}"
    );
    assert!(rendered.contains("line_width: 1.0"), "got: {rendered}");
}

#[test]
fn text_recording_captures_the_string_and_the_font() {
    let mut list: DrawList = DrawList::create();
    list.fill_text(
        String::from("hello"),
        Vector2D::new(4.0, 5.0),
        red(),
        "16px sans",
    );
    let rendered: String = format!("{:?}", list.commands()[0]);
    assert!(
        rendered.contains("hello"),
        "the glyphs must survive, got: {rendered}"
    );
    assert!(
        rendered.contains("16px sans"),
        "the font is part of the command, not global state, got: {rendered}"
    );
}

#[test]
fn global_alpha_and_blend_mode_are_recorded_as_state_commands() {
    let mut list: DrawList = DrawList::create();
    list.set_global_alpha(0.25);
    list.set_blend_mode(BlendMode::Multiply);
    assert_eq!(list.len(), 2, "state changes are part of the replay stream");
    let first: String = format!("{:?}", list.commands()[0]);
    assert!(first.contains("0.25"), "got: {first}");
    let second: String = format!("{:?}", list.commands()[1]);
    assert!(second.contains("Multiply"), "got: {second}");
}

#[test]
fn a_colour_converts_to_a_css_rgba_string() {
    let opaque: String = Color::to_css(&red());
    assert!(
        opaque.starts_with("rgba("),
        "the canvas API takes rgba(), got: {opaque}"
    );
    assert!(opaque.ends_with(')'), "got: {opaque}");
    assert!(
        opaque.contains("1"),
        "the channels must appear in the string, got: {opaque}"
    );
}

#[test]
fn a_translucent_colour_keeps_its_alpha_in_the_css_form() {
    let translucent: String = Color::to_css(&translucent_blue());
    assert!(
        translucent.starts_with("rgba("),
        "a half-transparent colour must not silently become opaque, got: {translucent}"
    );
    assert_ne!(
        translucent,
        Color::to_css(&red()),
        "two different colours must not collapse to the same string"
    );
}

#[test]
fn a_linear_gradient_records_its_endpoints_in_order() {
    let gradient: LinearGradient = LinearGradient::new(
        Vector2D::new(0.0, 0.0),
        Vector2D::new(100.0, 0.0),
        vec![
            (0.0, String::from("rgba(255,0,0,1)")),
            (1.0, String::from("rgba(0,0,255,1)")),
        ],
    );
    let rendered: String = format!("{gradient:?}");
    assert!(
        rendered.find("x: 0.0").unwrap() < rendered.find("x: 100.0").unwrap(),
        "a gradient's start and end must not be swapped, got: {rendered}"
    );
    assert!(
        rendered.contains("rgba(255,0,0,1)"),
        "the colour stops must survive verbatim for the canvas API, got: {rendered}"
    );
}

#[test]
fn a_radial_gradient_records_its_two_radii() {
    let gradient: RadialGradient = RadialGradient::new(
        Vector2D::new(10.0, 10.0),
        2.0,
        Vector2D::new(10.0, 10.0),
        8.0,
        vec![(0.0, String::from("rgba(0,0,0,1)"))],
    );
    let rendered: String = format!("{gradient:?}");
    assert!(rendered.contains("inner_radius: 2.0"), "got: {rendered}");
    assert!(
        rendered.contains("outer_radius: 8.0"),
        "an outer radius smaller than the inner one inverts the gradient, got: {rendered}"
    );
}

#[test]
fn a_shadow_config_carries_its_colour_blur_and_offset() {
    let shadow: ShadowConfig = ShadowConfig::new(String::from("rgba(0,0,0,0.5)"), 6.0, 2.0, 4.0);
    let rendered: String = format!("{shadow:?}");
    assert!(rendered.contains("rgba(0,0,0,0.5)"), "got: {rendered}");
    assert!(rendered.contains("blur: 6.0"), "got: {rendered}");
    assert!(rendered.contains("offset_x: 2.0"), "got: {rendered}");
    assert!(rendered.contains("offset_y: 4.0"), "got: {rendered}");
}

#[test]
fn the_default_render_layer_starts_hidden_at_depth_zero() {
    let base: RenderLayer = RenderLayer::default();
    let other: RenderLayer = RenderLayer::default();
    assert_eq!(
        base, other,
        "the default layer is a fixed starting point, so two of them must match"
    );
    assert!(
        format!("{base:?}").contains("visible: false"),
        "a layer is not drawn until something opts in, got: {base:?}"
    );
}

#[test]
fn a_camera_looks_from_its_position_towards_its_target() {
    let camera: Camera3D =
        Camera3D::create(Vector3D::new(0.0, 0.0, 5.0), Vector3D::zero(), 800.0, 600.0);
    let forward: Vector3D = camera.forward();
    assert!(
        (forward.get_x() - 0.0).abs() < EPSILON,
        "the camera sits on the z axis, so forward has no x, got {}",
        forward.get_x()
    );
    assert!(
        (forward.get_y() - 0.0).abs() < EPSILON,
        "got {}",
        forward.get_y()
    );
    assert!(
        (forward.get_z() + 1.0).abs() < EPSILON,
        "looking from z=5 at the origin means looking down -z, got {}",
        forward.get_z()
    );
    assert!(
        (forward.magnitude() - 1.0).abs() < EPSILON,
        "forward must be normalised, got magnitude {}",
        forward.magnitude()
    );
}

#[test]
fn the_right_vector_is_perpendicular_to_both_forward_and_up() {
    let camera: Camera3D =
        Camera3D::create(Vector3D::new(0.0, 0.0, 5.0), Vector3D::zero(), 800.0, 600.0);
    let right: Vector3D = camera.right();
    let forward: Vector3D = camera.forward();
    assert!(
        (right.dot(forward)).abs() < EPSILON,
        "a non-perpendicular basis skews every projected axis, got dot {}",
        right.dot(forward)
    );
    assert!(
        (right.magnitude() - 1.0).abs() < EPSILON,
        "right must be normalised, got {}",
        right.magnitude()
    );
    assert!(
        right.get_x() > 0.0,
        "with the default up vector, right points along +x, got {}",
        right.get_x()
    );
}

#[test]
fn the_aspect_ratio_is_width_over_height() {
    let wide: Camera3D = Camera3D::create(
        Vector3D::zero(),
        Vector3D::new(0.0, 0.0, -1.0),
        1600.0,
        900.0,
    );
    assert!(
        (wide.aspect() - 1600.0 / 900.0).abs() < EPSILON,
        "got {}",
        wide.aspect()
    );
    let square: Camera3D = Camera3D::create(
        Vector3D::zero(),
        Vector3D::new(0.0, 0.0, -1.0),
        100.0,
        100.0,
    );
    assert!(
        (square.aspect() - 1.0).abs() < EPSILON,
        "got {}",
        square.aspect()
    );
}

#[test]
fn a_degenerate_viewport_height_reports_an_aspect_of_one() {
    let flat: Camera3D =
        Camera3D::create(Vector3D::zero(), Vector3D::new(0.0, 0.0, -1.0), 800.0, 0.0);
    assert!(
        (flat.aspect() - 1.0).abs() < EPSILON,
        "a zero-height viewport must not divide by zero, got {}",
        flat.aspect()
    );
}

#[test]
fn a_default_camera_looks_down_the_negative_z_axis() {
    let camera: Camera3D = Camera3D::default();
    let forward: Vector3D = camera.forward();
    assert!(
        (forward.get_z() + 1.0).abs() < EPSILON,
        "the standard 3D camera convention looks down -z, got {forward:?}"
    );
    assert!(
        (camera.get_near() - camera.get_far()).abs() > 0.0,
        "near must be strictly closer than far, got {} and {}",
        camera.get_near(),
        camera.get_far()
    );
    assert!(
        camera.get_fov() > 0.0,
        "a zero field of view collapses the projection, got {}",
        camera.get_fov()
    );
}

#[test]
fn the_named_layers_are_all_visible_and_ordered_back_to_front() {
    let background: RenderLayer = RenderLayer::background();
    let foreground: RenderLayer = RenderLayer::foreground();
    let ui: RenderLayer = RenderLayer::ui();
    assert!(
        background.get_visible() && foreground.get_visible() && ui.get_visible(),
        "a named layer nobody has to opt into must draw itself"
    );
    assert!(
        background.get_z_index() < foreground.get_z_index(),
        "the background must be behind the foreground: {} vs {}",
        background.get_z_index(),
        foreground.get_z_index()
    );
    assert!(
        foreground.get_z_index() < ui.get_z_index(),
        "the UI overlay must sit above world content: {} vs {}",
        foreground.get_z_index(),
        ui.get_z_index()
    );
}

#[test]
fn the_named_layers_are_pairwise_distinct() {
    let all: [RenderLayer; 3] = [
        RenderLayer::background(),
        RenderLayer::foreground(),
        RenderLayer::ui(),
    ];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j], "two named layers collide");
        }
    }
}

#[test]
fn a_layer_can_be_created_hidden_at_an_explicit_depth() {
    let layer: RenderLayer = RenderLayer::create(5, false);
    assert_eq!(layer.get_z_index(), 5);
    assert!(!layer.get_visible(), "the caller asked for a hidden layer");
    assert_ne!(
        layer,
        RenderLayer::create(5, true),
        "visibility is part of a layer's identity"
    );
}

#[test]
fn a_css_font_string_puts_the_size_first_and_the_family_second() {
    let built: String = CanvasRenderer::font(16.0, "sans-serif");

    assert_eq!(
        built,
        String::from("16px sans-serif"),
        "the canvas font shorthand needs the unit attached and the family after it; a bare \
         number or a reversed pair is silently ignored by the 2D context"
    );
}

#[test]
fn a_fractional_font_size_keeps_its_fraction() {
    let built: String = CanvasRenderer::font(10.5, "Inter, system-ui");

    assert_eq!(
        built,
        String::from("10.5px Inter, system-ui"),
        "a fractional size is legal CSS and gets rounded if truncated here, which changes the \
         line height the text metrics were measured against"
    );
}
