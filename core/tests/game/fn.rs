use super::*;

struct TestBounceScene {
    x: f32,
    vx: f32,
    steps: u32,
}

impl GameScene for TestBounceScene {
    fn fixed_update(&mut self, dt: f32, input: &InputState) {
        let dir: f32 = input.axis(KeyCode::ArrowLeft, KeyCode::ArrowRight);
        self.x += (self.vx + dir * 120.0) * dt;
        self.steps += 1;
    }

    fn render(&mut self, _alpha: f32, ctx: &mut GameRenderContext<'_>) {
        ctx.clear(Color::hex(0x0B1120));
        ctx.fill_rounded_rect(
            Rect::new(self.x, 40.0, 28.0, 28.0),
            Color::hex(0x38BDF8),
            6.0,
        );
        ctx.fill_circle(Point::new(self.x + 14.0, 96.0), 12.0, Color::hex(0x34D399));
        let sprite: ImageBuffer = ImageBuffer::new(2, 2, vec![255; 16]);
        ctx.blit_framebuffer(Rect::new(12.0, 12.0, 24.0, 24.0), sprite);
    }
}

#[test]
fn game_engine_host_fixed_timestep_input_and_viewport_rendering_work() {
    reset_reactive_runtime();
    let host: GameEngineHost = GameEngineHost::new(
        TestBounceScene {
            x: 16.0,
            vx: 60.0,
            steps: 0,
        },
        1.0 / 60.0,
    );
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(320, 200).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win).set_clear_color(COLOR_TRANSPARENT);
    let mut app: App = App::headless(cfg).unwrap();
    let host_mount: GameEngineHost = host.clone();
    let _: NodeId = app.mount(move || {
        host_mount.mount_viewport(
            Style::empty()
                .width(Length::Px(320.0))
                .height(Length::Px(200.0)),
        )
    });
    let frame_a: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    let right_down: WindowEvent = WindowEvent::KeyDown(
        wid,
        KeyEvent::new(KeyCode::ArrowRight, Modifiers::default(), false),
    );
    host.ingest_event(&right_down);
    app.dispatch_window_event(right_down);
    assert!(app.get_input_state().is_key_down(KeyCode::ArrowRight));
    let fixed_steps: u32 = host.step(0.051);
    assert_eq!(fixed_steps, 3);
    let frame_b: FrameStats = app.step_game_frame(1.0 / 60.0).unwrap();
    assert_ne!(frame_a.get_frame_hash(), 0);
    assert_ne!(frame_b.get_frame_hash(), 0);
    assert_ne!(frame_a.get_frame_hash(), frame_b.get_frame_hash());
}

#[test]
fn scroll_container_clamps_offset_and_dispatches_on_scroll_callback() {
    reset_reactive_runtime();
    let scrolled_dy: Rc<Cell<f32>> = Rc::new(Cell::new(0.0));
    let scrolled_c: Rc<Cell<f32>> = scrolled_dy.clone();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(300, 160).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let mut scroll_id: NodeId = NodeId::default();
    let _: NodeId = app.mount(|| {
        let mut ctx: BuildContext = current_build_context();
        scroll_id = ctx.scroll_view(
            Style::empty()
                .column()
                .width(Length::Px(300.0))
                .height(Length::Px(120.0)),
        );
        ctx.on_scroll(scroll_id, move |ev: &UiEvent| {
            if let UiEventKind::Scroll(delta) = ev.get_kind() {
                scrolled_c.set(scrolled_c.get() + delta.get_y());
            }
        });
        for i in 0..20_u32 {
            let row: NodeId = ctx.text(
                format!("Row #{i}"),
                Style::empty()
                    .width(Length::Px(280.0))
                    .height(Length::Px(24.0)),
            );
            ctx.append(scroll_id, row);
        }
        scroll_id
    });
    let frame_top: FrameStats = app.step_frame().unwrap();
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(40.0, 40.0),
    ));
    app.dispatch_window_event(WindowEvent::Scroll(wid, Point::new(0.0, 96.0)));
    let frame_scrolled: FrameStats = app.step_frame().unwrap();
    assert!((scrolled_dy.get() - 96.0).abs() < 0.01);
    assert_ne!(frame_top.get_frame_hash(), frame_scrolled.get_frame_hash());
}

#[test]
fn game_world_ecs_physics_raycast_camera_particles_tilemap_and_audio_work_end_to_end() {
    let mut world: GameWorld = GameWorld::with_capacity(64);
    let a: EntityId = world.spawn(EntityTag::Player, Point::new(100.0, 100.0));
    let b: EntityId = world.spawn(EntityTag::Enemy, Point::new(116.0, 100.0));
    let temp: EntityId = world.spawn(EntityTag::Projectile, Point::new(200.0, 100.0));
    if let Some(ent) = world.get_entity_mut(a) {
        ent.set_velocity(Velocity2D::new(60.0, 0.0, 0.0));
        ent.set_body(Some(RigidBody2D::dynamic(1.0, 1.0, 0.0)));
        ent.set_collider(Some(Collider2D::circle(
            12.0,
            COLLISION_LAYER_PLAYER,
            COLLISION_LAYER_ALL,
            false,
        )));
        ent.set_sprite(Some(Sprite2D::new(
            EntityVisualShape::Triangle,
            Size::new(24.0, 24.0),
            Color::hex(0x38BDF8),
            COLOR_WHITE,
            5,
            0.0,
            true,
        )));
    }
    if let Some(ent) = world.get_entity_mut(b) {
        ent.set_velocity(Velocity2D::new(-40.0, 0.0, 0.0));
        ent.set_body(Some(RigidBody2D::dynamic(1.0, 1.0, 0.0)));
        ent.set_collider(Some(Collider2D::aabb(
            20.0,
            20.0,
            COLLISION_LAYER_ENEMY,
            COLLISION_LAYER_ALL,
            false,
        )));
        ent.set_sprite(Some(Sprite2D::new(
            EntityVisualShape::Diamond,
            Size::new(20.0, 20.0),
            Color::hex(0xF43F5E),
            COLOR_WHITE,
            4,
            0.0,
            true,
        )));
    }
    if let Some(ent) = world.get_entity_mut(temp) {
        ent.set_ttl_secs(Some(0.01));
    }
    let mut cfg: PhysicsConfig = PhysicsConfig::default();
    cfg.set_arena_bounds(Some(Rect::new(0.0, 0.0, 400.0, 300.0)));
    let mut physics: PhysicsWorld2D = PhysicsWorld2D::with_config(cfg);
    let contacts: Vec<CollisionEvent> = physics.step(&mut world, 1.0 / 60.0);
    assert!(!contacts.is_empty());
    let hit: Option<RaycastHit> = PhysicsWorld2D::raycast(
        &world,
        Point::new(40.0, 100.0),
        Point::new(1.0, 0.0),
        200.0,
        COLLISION_LAYER_PLAYER | COLLISION_LAYER_ENEMY,
    );
    assert!(hit.is_some());
    assert_eq!(world.tick_lifetimes(0.05), 1);
    assert!(!world.is_alive(temp));
    let recycled: EntityId = world.spawn(EntityTag::Pickup, Point::new(50.0, 50.0));
    assert_eq!(recycled.get_index(), temp.get_index());
    assert!(recycled.get_generation() > temp.get_generation());
    let mut camera: Camera2D = Camera2D::at(Point::new(100.0, 100.0), 1.5);
    camera.add_trauma(0.5);
    camera.follow(
        Point::new(140.0, 120.0),
        8.0,
        1.0 / 60.0,
        Size::new(320.0, 200.0),
    );
    let vp: Size = Size::new(320.0, 200.0);
    let world_pt: Point = Point::new(125.0, 110.0);
    let screen_pt: Point = camera.world_to_screen(world_pt, vp);
    let unproj: Point = camera.screen_to_world(screen_pt, vp);
    assert!((unproj.get_x() - world_pt.get_x()).abs() < 20.0);
    let mut particles: ParticleSystem = ParticleSystem::with_capacity(64);
    particles.emit_burst(
        Point::new(110.0, 100.0),
        24,
        ParticleEmitterConfig::default(),
    );
    assert_eq!(particles.update(1.0 / 60.0), 24);
    let mut tilemap: TileMap = TileMap::empty(8, 6, 32.0);
    let solid_id: u16 = tilemap.add_palette_entry(TilePaletteEntry::new(
        Color::hex(0x1E293B),
        Color::hex(0x38BDF8),
        true,
    ));
    tilemap.set_tile(2, 2, solid_id);
    assert!(tilemap.is_solid_at_world(Point::new(72.0, 72.0)));
    assert!(!tilemap.is_solid_at_world(Point::new(12.0, 12.0)));
    let sheet: SpriteSheet =
        SpriteSheet::from_grid(ImageBuffer::new(16, 8, vec![255; 16 * 8 * 4]), 8, 8);
    let mut animator: SpriteAnimator = SpriteAnimator::new(0, 2, 10.0, true, 0.0);
    assert_eq!(animator.tick(0.15), 1);
    let mut input: InputState = InputState::new();
    input.ingest(&WindowEvent::KeyDown(
        WindowId::new(1),
        KeyEvent::new(KeyCode::Character('d'), Modifiers::default(), false),
    ));
    let actions: ActionMap = ActionMap::default();
    assert!(actions.is_active(GameAction::MoveRight, &input));
    assert!(actions.movement_vector(&input).get_x() > 0.5);
    let mut audio: AudioBus = AudioBus::default();
    audio.play(SoundCue::Explosion, 0.9, 0.0, 1.0);
    assert_eq!(audio.drain().len(), 1);
    assert_eq!(audio.get_total_played(), 1);
    let mut list: DisplayList = DisplayList::default();
    list.reset(vp, 1.0);
    let mut ctx: GameRenderContext<'_> =
        GameRenderContext::new(Rect::new(0.0, 0.0, 320.0, 200.0), &mut list);
    ctx.render_tilemap(&tilemap, &camera);
    let drawn_ent: usize = ctx.render_world(&world, &camera);
    let drawn_part: usize = ctx.render_particles(&particles, &camera);
    ctx.draw_sprite_frame(&sheet, 0, Rect::new(4.0, 4.0, 16.0, 16.0), COLOR_WHITE);
    ctx.draw_debug_colliders(&world, &camera);
    assert!(drawn_ent >= 2);
    assert!(drawn_part >= 1);
    let mut sw: SoftwareRenderer =
        SoftwareRenderer::with_config(320, 200, RenderConfig::software());
    let (_, shaded): (usize, usize) = sw.render(&list, DamageRegion::full());
    assert!(shaded > 0);
}

#[test]
fn idle_frames_skip_rasterization_and_hover_uses_partial_dirty_rect() {
    reset_reactive_runtime();
    let mut win: WindowConfig = WindowConfig::default();
    win.set_size(400, 300).set_render(RenderConfig::software());
    let mut cfg: AppConfig = AppConfig::default();
    cfg.set_window(win);
    let mut app: App = App::headless(cfg).unwrap();
    let _: NodeId = app.mount(|| {
        let mut ctx: BuildContext = current_build_context();
        let root: NodeId = ctx.view(
            Style::empty()
                .column()
                .width(Length::Px(400.0))
                .height(Length::Px(300.0))
                .padding(Edges::all(16.0))
                .background(COLOR_WHITE),
        );
        let btn: NodeId = ctx.view(
            Style::empty()
                .width(Length::Px(80.0))
                .height(Length::Px(32.0))
                .background(Color::hex(0xE4E4E7))
                .cursor(CursorIcon::Pointer),
        );
        ctx.append(root, btn);
        root
    });
    let first: FrameStats = app.step_frame().unwrap();
    assert!(first.get_pixels_shaded() > 0);
    let idle: FrameStats = app.step_frame().unwrap();
    assert_eq!(idle.get_pixels_shaded(), 0);
    assert_eq!(idle.get_frame_hash(), first.get_frame_hash());
    let wid: WindowId = app.get_window_state().get_id();
    app.dispatch_window_event(WindowEvent::PointerMove(
        wid,
        PointerEvent::mouse(40.0, 30.0),
    ));
    let hovered: FrameStats = app.step_frame().unwrap();
    assert!(hovered.get_pixels_shaded() > 0);
    assert!(hovered.get_pixels_shaded() < first.get_pixels_shaded());
    assert_ne!(hovered.get_frame_hash(), first.get_frame_hash());
}

#[test]
fn engine3d_model_import_bvh_raytracing_frustum_culling_lod_and_runtime_quality_work() {
    let mut world: World3D = World3D::create_open_world_city();
    assert!(world.get_objects().len() >= 30);
    assert!(world.get_bvh().get_nodes().len() >= 3);
    let obj_src: &str = "o crystal_arch\nv -1 0 -1\nv 1 0 -1\nv 1 0 1\nv -1 0 1\nv 0 3 0\nf 1 2 5\nf 2 3 5\nf 3 4 5\nf 4 1 5\n";
    let imported_obj: ImportedModel3D = world
        .import_model(
            ModelFormat::Obj,
            obj_src,
            Transform3D::new(Vec3::new(0.0, 0.0, -8.0), 0.0, Vec3::splat(2.0)),
            PbrMaterial::pbr(Color::hex(0x38BDF8), 0.85, 0.15, 0.7),
            0.5,
        )
        .unwrap();
    assert_eq!(imported_obj.get_name(), "crystal_arch");
    assert_eq!(imported_obj.get_total_triangles(), 4);
    let gltf_src: &str = r#"{"asset":{"version":"2.0"},"meshes":[{"name":"gltf_beacon","primitives":[{"positions":[0,0,0, 2,0,0, 1,4,0, 1,0,2],"indices":[0,1,2, 1,3,2, 3,0,2]}]}]}"#;
    let imported_gltf: ImportedModel3D = world
        .import_model(
            ModelFormat::GltfJson,
            gltf_src,
            Transform3D::new(Vec3::new(4.0, 0.0, -10.0), 0.2, Vec3::splat(1.5)),
            PbrMaterial::neon(Color::hex(0xA855F7), 1.6),
            0.3,
        )
        .unwrap();
    assert_eq!(imported_gltf.get_format(), ModelFormat::GltfJson);
    assert_eq!(imported_gltf.get_total_triangles(), 3);
    assert_eq!(world.get_imported_model_count(), 2);
    let mut renderer: Renderer3D = Renderer3D::default();
    let cfg_off: RenderQualityConfig = RenderQualityConfig::from_preset(QualityPreset::Low);
    let img_off: ImageBuffer = renderer.render_world(&mut world, 360, 200, &cfg_off);
    let stats_off: RenderStats3D = renderer.get_last_stats();
    assert_eq!(stats_off.get_rays_traced(), 0);
    assert!(stats_off.get_triangles_drawn() > 0);
    let mut cfg_rt: RenderQualityConfig = RenderQualityConfig::from_preset(QualityPreset::Ultra);
    cfg_rt.set_ray_tracing(RayTracingMode::FullPathTracedGi);
    let img_rt: ImageBuffer = renderer.render_world(&mut world, 360, 200, &cfg_rt);
    let stats_rt: RenderStats3D = renderer.get_last_stats();
    assert!(stats_rt.get_rays_traced() > 0);
    assert!(stats_rt.get_bvh_nodes() > 0);
    assert!(stats_rt.get_instanced_batches() <= stats_rt.get_visible_objects());
    assert_ne!(img_off.get_pixels(), img_rt.get_pixels());
    let mut drs: RenderQualityConfig = RenderQualityConfig::from_preset(QualityPreset::High);
    let before_scale: f32 = drs.get_resolution_scale();
    drs.adjust_dynamic_resolution(28.0);
    assert!(drs.get_resolution_scale() < before_scale);
    let mut input: InputState = InputState::new();
    input.ingest(&WindowEvent::KeyDown(
        WindowId::new(1),
        KeyEvent::new(KeyCode::Character('w'), Modifiers::default(), false),
    ));
    let before_z: f32 = world.get_objects()[world.vehicle_index().unwrap()]
        .get_transform()
        .get_position()
        .get_z();
    world.step_simulation(0.1, &input, &ActionMap::default());
    let after_z: f32 = world.get_objects()[world.vehicle_index().unwrap()]
        .get_transform()
        .get_position()
        .get_z();
    assert!(after_z < before_z);
}
