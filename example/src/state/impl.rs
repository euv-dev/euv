use super::*;

/// Construction and helper methods for `ShowcaseState`.
impl ShowcaseState {
    /// Initializes a new reactive `ShowcaseState` with default values, derived `computed!` memos, and game host.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized reactive showcase state.
    #[must_use]
    pub fn new() -> Self {
        let counter: Signal<i32> = Signal::new(1);
        let doubled: Memo<i32> = computed!((counter) -> i32 => move |n: i32| n * 2);
        let arena: DemoPhysicsScene = DemoPhysicsScene::create_arena();
        let game_host: GameEngineHost = GameEngineHost::new(arena, 1.0 / 60.0);
        Self {
            route: Signal::new(ROUTE_OVERVIEW.to_string()),
            counter,
            doubled,
            render_mode_index: Signal::new(0),
            theme: UiThemeState::new(active_feature_scheme(), ThemeMode::Light),
            username: Signal::new(DEFAULT_USERNAME.to_string()),
            bio: Signal::new("Zero-WebView native Rust UI".to_string()),
            stepper_value: Signal::new(4),
            radio_index: Signal::new(1),
            slider_value: Signal::new(65.0),
            select_index: Signal::new(0),
            select_open: Signal::new(false),
            rating_value: Signal::new(4),
            pin_toggled: Signal::new(false),
            hardware_accel: Signal::new(true),
            agree_terms: Signal::new(true),
            modal_open: Signal::new(false),
            drawer_open: Signal::new(false),
            sheet_open: Signal::new(false),
            popover_open: Signal::new(false),
            dropdown_open: Signal::new(false),
            dropdown_selected: Signal::new(None),
            toast_visible: Signal::new(false),
            notif_visible: Signal::new(false),
            loading_active: Signal::new(false),
            active_tab: Signal::new(0),
            page_index: Signal::new(1),
            step_index: Signal::new(1),
            accordion_open: Signal::new(Some(0)),
            fullscreen: Signal::new(false),
            list_scroll_y: Signal::new(640.0),
            island_active: Signal::new(false),
            island_progress: Signal::new(25),
            camera_perm_status: Signal::new("NotDetermined".to_string()),
            protocol_status: Signal::new("Unregistered".to_string()),
            a11y_announcements: Signal::new(0),
            haptic_pulses: Signal::new(0),
            game_host,
            dynamic_hub: DynamicUpdateHub::new(),
            sibling_counter: Signal::new(0),
            dynamic_action_log: Signal::new("None yet".to_string()),
            alert_dialog_open: Signal::new(false),
            alert_dialog_confirmed: Signal::new(false),
            collapsible_open: Signal::new(true),
            toggle_group_index: Signal::new(0),
            otp_code: Signal::new("202642".to_string()),
            command_query: Signal::new(String::new()),
            command_selected: Signal::new(Some(0)),
            hover_card_open: Signal::new(false),
            calendar_day: Signal::new(25),
            menubar_active: Signal::new(None),
            carousel_index: Signal::new(0),
        }
    }

    /// Binds `hub` from the active `App` and wires interactive dynamic actions to showcase state and routing.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable showcase state.
    /// - `DynamicUpdateHub` - Shared dynamic update hub owned by `App`.
    pub fn attach_dynamic_hub(&mut self, hub: DynamicUpdateHub) {
        let route_sig: Signal<String> = self.get_route();
        let action_log: Signal<String> = self.get_dynamic_action_log();
        hub.on_action(move |ev: &DynamicActionEvent| {
            let summary: String = format!(
                "{} from {} ({})",
                ev.get_action(),
                ev.get_source_id(),
                ev.get_payload()
            );
            let _: bool = action_log.set_if_changed(summary);
            if let Some(target_route) = ev.get_action().strip_prefix("navigate:") {
                let _: bool = route_sig.set_if_changed(target_route.to_string());
            } else if ev.get_action().as_str() == "return_dynamic" {
                let _: bool = route_sig.set_if_changed(ROUTE_DYNAMIC.to_string());
            }
        });
        let Self { dynamic_hub, .. } = self;
        *dynamic_hub = hub;
    }

    /// Returns the ordered sidebar navigation items for the showcase.
    ///
    /// # Returns
    ///
    /// - `Vec<EuvNavItem>` - Complete list of showcase routes.
    #[must_use]
    pub fn nav_items() -> Vec<EuvNavItem> {
        vec![
            EuvNavItem::new(ROUTE_OVERVIEW.to_string(), NAV_LABEL_OVERVIEW.to_string()),
            EuvNavItem::new(ROUTE_REACTIVE.to_string(), NAV_LABEL_REACTIVE.to_string()),
            EuvNavItem::new(ROUTE_RENDER.to_string(), NAV_LABEL_RENDER.to_string()),
            EuvNavItem::new(ROUTE_WINDOW.to_string(), NAV_LABEL_WINDOW.to_string()),
            EuvNavItem::new(ROUTE_FORMS.to_string(), NAV_LABEL_FORMS.to_string()),
            EuvNavItem::new(ROUTE_THEMES.to_string(), NAV_LABEL_THEMES.to_string()),
            EuvNavItem::new(ROUTE_DATA.to_string(), NAV_LABEL_DATA.to_string()),
            EuvNavItem::new(ROUTE_GAME.to_string(), NAV_LABEL_GAME.to_string()),
            EuvNavItem::new(ROUTE_OVERLAYS.to_string(), NAV_LABEL_OVERLAYS.to_string()),
            EuvNavItem::new(ROUTE_DYNAMIC.to_string(), NAV_LABEL_DYNAMIC.to_string()),
        ]
    }

    /// Resolves the currently selected `RenderMode` from `render_mode_index`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - State reference.
    ///
    /// # Returns
    ///
    /// - `RenderMode` - Active render pipeline mode.
    #[must_use]
    pub fn selected_render_mode(&self) -> RenderMode {
        match self.get_render_mode_index().get_untracked() {
            1 => RenderMode::Software,
            2 => RenderMode::Hardware,
            3 => RenderMode::Hybrid,
            _ => RenderMode::Auto,
        }
    }

    /// Cycles to the next `DesignScheme` in `DesignScheme::all()`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - State reference.
    pub fn next_scheme(&self) {
        let all: [DesignScheme; 17] = DesignScheme::all();
        let current: DesignScheme = self.get_theme().get_scheme().get_untracked();
        let pos: usize = all
            .iter()
            .position(|s: &DesignScheme| *s == current)
            .unwrap_or(0);
        let next: DesignScheme = all[(pos + 1) % all.len()];
        let _: bool = self.get_theme().get_scheme().set_if_changed(next);
    }

    /// Toggles between light and dark theme modes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - State reference.
    pub fn toggle_theme_mode(&self) {
        let cur: ThemeMode = self.get_theme().get_mode().get_untracked();
        let next: ThemeMode = match cur {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        let _: bool = self.get_theme().get_mode().set_if_changed(next);
    }
}

/// Default `ShowcaseState` delegating to `ShowcaseState::new`.
impl Default for ShowcaseState {
    /// Creates a default `ShowcaseState`.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized state.
    fn default() -> Self {
        Self::new()
    }
}

/// Arena setup and gameplay helper methods for `DemoPhysicsScene`.
impl DemoPhysicsScene {
    /// Builds a playable 1280x720 tiled neon arena with player ship, obstacles, and initial enemy wave.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized playable arena scene.
    #[must_use]
    pub fn create_arena() -> Self {
        let mut tilemap: TileMap = TileMap::empty(24, 14, 48.0);
        let floor_a: u16 = tilemap.add_palette_entry(TilePaletteEntry::new(
            Color::hex(0x0B1120),
            Color::rgba(30, 41, 59, 90),
            false,
        ));
        let floor_b: u16 = tilemap.add_palette_entry(TilePaletteEntry::new(
            Color::hex(0x0F172A),
            Color::rgba(30, 41, 59, 90),
            false,
        ));
        let wall_tile: u16 = tilemap.add_palette_entry(TilePaletteEntry::new(
            Color::hex(0x1E293B),
            Color::hex(0x38BDF8),
            true,
        ));
        for row in 0..tilemap.get_rows() {
            for col in 0..tilemap.get_columns() {
                let is_border: bool = row == 0
                    || col == 0
                    || row + 1 == tilemap.get_rows()
                    || col + 1 == tilemap.get_columns();
                let tid: u16 = if is_border {
                    wall_tile
                } else if (row + col) % 2 == 0 {
                    floor_a
                } else {
                    floor_b
                };
                tilemap.set_tile(col, row, tid);
            }
        }
        let world_rect: Rect = tilemap.world_bounds();
        let inner_arena: Rect = Rect::new(
            56.0,
            56.0,
            (world_rect.get_width() - 112.0).max(200.0),
            (world_rect.get_height() - 112.0).max(200.0),
        );
        let mut world: GameWorld = GameWorld::with_capacity(256);
        let center: Point = Point::new(world_rect.get_width() * 0.5, world_rect.get_height() * 0.5);
        let player_id: EntityId = world.spawn(EntityTag::Player, center);
        if let Some(player) = world.get_entity_mut(player_id) {
            player.set_body(Some(RigidBody2D::dynamic(1.0, 0.4, 3.2)));
            player.set_collider(Some(Collider2D::circle(
                14.0,
                COLLISION_LAYER_PLAYER,
                COLLISION_MASK_PLAYER_DEFAULT,
                false,
            )));
            player.set_sprite(Some(Sprite2D::new(
                EntityVisualShape::Triangle,
                Size::new(30.0, 26.0),
                Color::hex(0x38BDF8),
                Color::hex(0xE0F2FE),
                10,
                0.0,
                true,
            )));
        }
        let pillars: [(f32, f32); 4] = [
            (280.0, 200.0),
            (860.0, 200.0),
            (280.0, 470.0),
            (860.0, 470.0),
        ];
        for (px, py) in pillars.into_iter() {
            let pid: EntityId = world.spawn(EntityTag::Obstacle, Point::new(px, py));
            if let Some(obs) = world.get_entity_mut(pid) {
                obs.set_body(Some(RigidBody2D::static_body(0.85)));
                obs.set_collider(Some(Collider2D::aabb(
                    52.0,
                    52.0,
                    COLLISION_LAYER_WORLD,
                    COLLISION_LAYER_ALL,
                    false,
                )));
                obs.set_sprite(Some(Sprite2D::new(
                    EntityVisualShape::Rect,
                    Size::new(52.0, 52.0),
                    Color::hex(0x1E293B),
                    Color::hex(0x6366F1),
                    2,
                    8.0,
                    true,
                )));
            }
        }
        let mut phys_cfg: PhysicsConfig = PhysicsConfig::default();
        phys_cfg.set_arena_bounds(Some(inner_arena));
        let mut camera: Camera2D = Camera2D::at(center, 1.0);
        camera.set_world_bounds(Some(world_rect));
        let mut scene: Self = Self {
            world,
            physics: PhysicsWorld2D::with_config(phys_cfg),
            camera,
            particles: ParticleSystem::with_capacity(1024),
            tilemap,
            actions: ActionMap::default(),
            audio: AudioBus::default(),
            player_id,
            fire_cooldown: 0.0,
            dash_cooldown: 0.0,
            score: 0,
            wave: 1,
            show_colliders: false,
            smoothed_fps: 60.0,
            ticks: 0,
        };
        scene.spawn_enemy_wave(8);
        scene
    }

    /// Spawns `count` patrolling enemy drones around the arena perimeter.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable arena scene.
    /// - `usize` - Number of enemy drones to spawn.
    pub fn spawn_enemy_wave(&mut self, count: usize) {
        let bounds: Rect = self.get_tilemap().world_bounds();
        let cx: f32 = bounds.get_width() * 0.5;
        let cy: f32 = bounds.get_height() * 0.5;
        for idx in 0..count {
            let angle: f32 = ((idx as f32) / (count.max(1) as f32)) * 6.283_185_5
                + (self.get_wave() as f32) * 0.45;
            let radius: f32 = 220.0 + ((idx % 3) as f32) * 48.0;
            let sx: f32 = (cx + angle.cos() * radius).clamp(110.0, bounds.get_width() - 110.0);
            let sy: f32 = (cy + angle.sin() * radius).clamp(110.0, bounds.get_height() - 110.0);
            let speed: f32 = 95.0 + ((idx % 4) as f32) * 28.0;
            let eid: EntityId = self
                .get_mut_world()
                .spawn(EntityTag::Enemy, Point::new(sx, sy));
            if let Some(enemy) = self.get_mut_world().get_entity_mut(eid) {
                enemy.set_velocity(Velocity2D::new(
                    -angle.sin() * speed,
                    angle.cos() * speed,
                    1.5,
                ));
                enemy.set_body(Some(RigidBody2D::dynamic(1.2, 0.95, 0.05)));
                enemy.set_collider(Some(Collider2D::circle(
                    13.0,
                    COLLISION_LAYER_ENEMY,
                    COLLISION_LAYER_ALL,
                    false,
                )));
                enemy.set_sprite(Some(Sprite2D::new(
                    if idx % 2 == 0 {
                        EntityVisualShape::Diamond
                    } else {
                        EntityVisualShape::Circle
                    },
                    Size::new(26.0, 26.0),
                    if idx % 2 == 0 {
                        Color::hex(0xF43F5E)
                    } else {
                        Color::hex(0xF59E0B)
                    },
                    Color::hex(0xFEF08A),
                    6,
                    0.0,
                    true,
                )));
            }
        }
    }

    /// Fires a fast plasma projectile from the player ship along `aim_angle`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable arena scene.
    /// - `Point` - Launch origin in world coordinates.
    /// - `f32` - Launch angle in radians.
    fn fire_projectile(&mut self, origin: Point, aim_angle: f32) {
        let dir_x: f32 = aim_angle.cos();
        let dir_y: f32 = aim_angle.sin();
        let launch: Point =
            Point::new(origin.get_x() + dir_x * 20.0, origin.get_y() + dir_y * 20.0);
        let pid: EntityId = self.get_mut_world().spawn(EntityTag::Projectile, launch);
        if let Some(proj) = self.get_mut_world().get_entity_mut(pid) {
            proj.get_mut_transform().set_rotation_rad(aim_angle);
            proj.set_velocity(Velocity2D::new(dir_x * 520.0, dir_y * 520.0, 0.0));
            proj.set_body(Some(RigidBody2D::dynamic(0.2, 0.9, 0.0)));
            proj.set_collider(Some(Collider2D::circle(
                5.0,
                COLLISION_LAYER_PROJECTILE,
                COLLISION_LAYER_ENEMY | COLLISION_LAYER_WORLD,
                true,
            )));
            proj.set_sprite(Some(Sprite2D::new(
                EntityVisualShape::Circle,
                Size::new(10.0, 10.0),
                Color::hex(0x34D399),
                Color::hex(0xFFFFFF),
                8,
                0.0,
                true,
            )));
            proj.set_ttl_secs(Some(1.4));
        }
        let mut muzzle: ParticleEmitterConfig = ParticleEmitterConfig::default();
        muzzle
            .set_base_angle(aim_angle)
            .set_spread_rad(0.45)
            .set_min_speed(80.0)
            .set_max_speed(180.0)
            .set_min_life(0.12)
            .set_max_life(0.26)
            .set_start_size(3.5)
            .set_start_color(Color::hex(0x34D399))
            .set_end_color(Color::rgba(56, 189, 248, 0));
        self.get_mut_particles().emit_burst(launch, 6, muzzle);
        self.get_mut_audio().play(SoundCue::Laser, 0.65, 0.0, 1.05);
    }
}

/// Deterministic fixed-timestep 2D game engine simulation and camera-driven HUD rendering for `DemoPhysicsScene`.
impl GameScene for DemoPhysicsScene {
    /// Steps player controls, ECS physics, collision combat, particles, and camera follow at 60Hz fixed cadence.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable arena scene.
    /// - `f32` - Fixed simulation delta time in seconds.
    /// - `&InputState` - Real-time polled keyboard and pointer state.
    fn fixed_update(&mut self, dt: f32, input: &InputState) {
        let move_vec: Point = self.get_actions().movement_vector(input);
        let fire_pressed: bool = self.get_actions().is_active(GameAction::Fire, input);
        let dash_pressed: bool = self.get_actions().is_just_pressed(GameAction::Dash, input);
        let wave_pressed: bool = self
            .get_actions()
            .is_just_pressed(GameAction::ActionAlt, input);
        if wave_pressed {
            let next_wave: u32 = self.get_wave().saturating_add(1);
            self.set_wave(next_wave);
            self.spawn_enemy_wave(6);
        }
        let fire_cd: f32 = (self.get_fire_cooldown() - dt).max(0.0);
        let dash_cd: f32 = (self.get_dash_cooldown() - dt).max(0.0);
        self.set_fire_cooldown(fire_cd);
        self.set_dash_cooldown(dash_cd);
        let player_id: EntityId = self.get_player_id();
        let mut player_pos: Point = Point::new(576.0, 336.0);
        let mut aim_angle: f32 = 0.0;
        let mut dashed: bool = false;
        if let Some(player) = self.get_mut_world().get_entity_mut(player_id) {
            player_pos = player.get_transform().get_position();
            let accel: f32 = 920.0;
            let mut vel: Velocity2D = player.get_velocity();
            vel.set_vx(vel.get_vx() + move_vec.get_x() * accel * dt);
            vel.set_vy(vel.get_vy() + move_vec.get_y() * accel * dt);
            let mut rot: f32 = player.get_transform().get_rotation_rad();
            if move_vec.get_x().abs() > 0.05 || move_vec.get_y().abs() > 0.05 {
                rot = move_vec.get_y().atan2(move_vec.get_x());
            } else {
                rot += dt * 0.8;
            }
            if dash_pressed && dash_cd <= 0.0 {
                vel.set_vx(vel.get_vx() + rot.cos() * 440.0);
                vel.set_vy(vel.get_vy() + rot.sin() * 440.0);
                dashed = true;
            }
            player.set_velocity(vel);
            player.get_mut_transform().set_rotation_rad(rot);
            aim_angle = rot;
        }
        if dashed {
            self.set_dash_cooldown(0.45);
            self.get_mut_camera().add_trauma(0.25);
            let mut burst: ParticleEmitterConfig = ParticleEmitterConfig::default();
            burst
                .set_base_angle(aim_angle + PI)
                .set_spread_rad(0.9)
                .set_start_color(Color::hex(0x38BDF8))
                .set_end_color(Color::rgba(56, 189, 248, 0));
            self.get_mut_particles().emit_burst(player_pos, 18, burst);
            self.get_mut_audio().play(SoundCue::Dash, 0.8, 0.0, 1.0);
        }
        if (fire_pressed || self.get_ticks().is_multiple_of(22)) && fire_cd <= 0.0 {
            self.fire_projectile(player_pos, aim_angle);
            self.set_fire_cooldown(0.14);
        }
        let collisions: Vec<CollisionEvent> = {
            let Self { world, physics, .. } = self;
            physics.step(world, dt)
        };
        let mut to_despawn: Vec<(EntityId, Point, bool)> = Vec::new();
        for ev in collisions.into_iter() {
            let tag_a: Option<EntityTag> = self
                .get_world()
                .get_entity(ev.get_entity_a())
                .map(EntityRecord::get_tag);
            let tag_b: Option<EntityTag> = self
                .get_world()
                .get_entity(ev.get_entity_b())
                .map(EntityRecord::get_tag);
            match (tag_a, tag_b) {
                (Some(EntityTag::Projectile), Some(EntityTag::Enemy)) => {
                    if let Some(e) = self.get_world().get_entity(ev.get_entity_b()) {
                        to_despawn.push((
                            ev.get_entity_b(),
                            e.get_transform().get_position(),
                            true,
                        ));
                    }
                    to_despawn.push((ev.get_entity_a(), Point::default(), false));
                }
                (Some(EntityTag::Enemy), Some(EntityTag::Projectile)) => {
                    if let Some(e) = self.get_world().get_entity(ev.get_entity_a()) {
                        to_despawn.push((
                            ev.get_entity_a(),
                            e.get_transform().get_position(),
                            true,
                        ));
                    }
                    to_despawn.push((ev.get_entity_b(), Point::default(), false));
                }
                (Some(EntityTag::Projectile), Some(EntityTag::Obstacle)) => {
                    to_despawn.push((ev.get_entity_a(), Point::default(), false));
                }
                (Some(EntityTag::Obstacle), Some(EntityTag::Projectile)) => {
                    to_despawn.push((ev.get_entity_b(), Point::default(), false));
                }
                (Some(EntityTag::Player), Some(EntityTag::Enemy))
                | (Some(EntityTag::Enemy), Some(EntityTag::Player)) => {
                    self.get_mut_camera().add_trauma(0.35);
                    self.get_mut_audio().play(SoundCue::Bounce, 0.7, 0.0, 0.85);
                }
                _ => {}
            }
        }
        for (eid, pos, is_enemy) in to_despawn.into_iter() {
            if self.get_mut_world().despawn(eid) && is_enemy {
                let next_score: u32 = self.get_score().saturating_add(100);
                self.set_score(next_score);
                self.get_mut_camera().add_trauma(0.32);
                let mut exp: ParticleEmitterConfig = ParticleEmitterConfig::default();
                exp.set_min_speed(70.0)
                    .set_max_speed(260.0)
                    .set_start_size(5.5)
                    .set_start_color(Color::hex(0xFB7185))
                    .set_end_color(Color::rgba(250, 204, 21, 0));
                self.get_mut_particles().emit_burst(pos, 22, exp);
                self.get_mut_audio()
                    .play(SoundCue::Explosion, 0.9, 0.0, 0.95);
            }
        }
        if self.get_world().count_by_tag(EntityTag::Enemy) < 3 {
            let next_wave: u32 = self.get_wave().saturating_add(1);
            self.set_wave(next_wave);
            self.spawn_enemy_wave(6);
        }
        let _: usize = self.get_mut_world().tick_lifetimes(dt);
        let _: usize = self.get_mut_particles().update(dt);
        let _: Vec<AudioCommand> = self.get_mut_audio().drain();
        if let Some(player) = self.get_world().get_entity(player_id) {
            let p: Point = player.get_transform().get_position();
            self.get_mut_camera()
                .follow(p, 7.5, dt, Size::new(820.0, 320.0));
        }
        self.set_ticks(self.get_ticks().saturating_add(1));
    }

    /// Updates smoothed frame-rate telemetry once per visual frame.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable arena scene.
    /// - `f32` - Real frame delta time in seconds.
    /// - `&InputState` - Current polled input snapshot.
    fn update(&mut self, dt: f32, _input: &InputState) {
        if dt > 0.0001 {
            let inst_fps: f32 = (1.0 / dt).clamp(1.0, 240.0);
            let next: f32 = self.get_smoothed_fps() * 0.9 + inst_fps * 0.1;
            self.set_smoothed_fps(next);
        }
    }

    /// Renders the camera-projected tilemap, ECS entities, raycast targeting laser, particles, and engine HUD.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable arena scene.
    /// - `f32` - Interpolation alpha in `[0.0, 1.0]`.
    /// - `&mut GameRenderContext<'_>` - Immediate-mode game draw surface.
    fn render(&mut self, _alpha: f32, ctx: &mut GameRenderContext<'_>) {
        let size: Size = ctx.viewport_size();
        ctx.clear(Color::hex(0x020617));
        let cam: Camera2D = self.get_camera();
        ctx.render_tilemap(self.get_tilemap(), &cam);
        if let Some(player) = self.get_world().get_entity(self.get_player_id()) {
            let origin: Point = player.get_transform().get_position();
            let rot: f32 = player.get_transform().get_rotation_rad();
            let dir: Point = Point::new(rot.cos(), rot.sin());
            let end_world: Point = match PhysicsWorld2D::raycast(
                self.get_world(),
                origin,
                dir,
                340.0,
                COLLISION_LAYER_ENEMY | COLLISION_LAYER_WORLD,
            ) {
                Some(hit) => hit.get_point(),
                None => Point::new(
                    origin.get_x() + dir.get_x() * 340.0,
                    origin.get_y() + dir.get_y() * 340.0,
                ),
            };
            let s0: Point = cam.world_to_screen(origin, size);
            let s1: Point = cam.world_to_screen(end_world, size);
            ctx.draw_line(s0, s1, 1.5, Color::rgba(56, 189, 248, 120));
            ctx.fill_circle(s1, 3.5, Color::hex(0x38BDF8));
        }
        let drawn_entities: usize = ctx.render_world(self.get_world(), &cam);
        let drawn_particles: usize = ctx.render_particles(self.get_particles(), &cam);
        if self.get_show_colliders() {
            ctx.draw_debug_colliders(self.get_world(), &cam);
        }
        ctx.fill_rounded_rect(
            Rect::new(
                10.0,
                10.0,
                (size.get_width() - 20.0).clamp(180.0, 560.0),
                28.0,
            ),
            Color::rgba(15, 23, 42, 210),
            6.0,
        );
        ctx.draw_text(
            Point::new(18.0, 16.0),
            format!(
                "FPS {:.0} | Wave {} | Score {} | Ent {} | Part {} | SFX {}",
                self.get_smoothed_fps(),
                self.get_wave(),
                self.get_score(),
                drawn_entities,
                drawn_particles,
                self.get_audio().get_total_played()
            ),
            12.0,
            Color::hex(0xE2E8F0),
        );
        ctx.draw_text(
            Point::new(14.0, (size.get_height() - 22.0).max(40.0)),
            GAME_HUD_CONTROLS_HINT.to_string(),
            11.0,
            Color::rgba(148, 163, 184, 220),
        );
    }
}
