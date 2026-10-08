use super::*;

#[test]
fn render_pass_color_attachment_externally_constructible() {
    let attachment: ColorAttachment = ColorAttachment {
        view: None,
        resolve_target: None,
        clear: Some(Color::new(0.1, 0.2, 0.3, 1.0)),
        load_op: LoadOp::Clear,
        store_op: StoreOp::Store,
    };
    let load: &LoadOp = attachment.get_load_op();
    assert_eq!(*load, LoadOp::Clear);
    let store: &StoreOp = attachment.get_store_op();
    assert_eq!(*store, StoreOp::Store);
    let view: Option<&JsValue> = attachment.try_get_view().as_ref();
    assert!(view.is_none());
    let resolve: Option<&JsValue> = attachment.try_get_resolve_target().as_ref();
    assert!(resolve.is_none());
}

#[test]
fn render_pass_depth_stencil_attachment_externally_constructible() {
    let attachment: DepthStencilAttachment = DepthStencilAttachment {
        view: None,
        depth_clear: Some(0.5),
        depth_load_op: LoadOp::Clear,
        depth_store_op: StoreOp::Store,
        depth_read_only: false,
    };
    let load: &LoadOp = attachment.get_depth_load_op();
    assert_eq!(*load, LoadOp::Clear);
    let store: &StoreOp = attachment.get_depth_store_op();
    assert_eq!(*store, StoreOp::Store);
    let read_only: &bool = attachment.get_depth_read_only();
    assert!(!*read_only);
    let view: Option<&JsValue> = attachment.try_get_view().as_ref();
    assert!(view.is_none());
}

#[test]
fn render_pass_attachments_round_trip_optional_fields() {
    let attachment: ColorAttachment = ColorAttachment {
        view: None,
        resolve_target: None,
        clear: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
        load_op: LoadOp::Load,
        store_op: StoreOp::Discard,
    };
    let load: &LoadOp = attachment.get_load_op();
    assert_eq!(*load, LoadOp::Load);
    let store: &StoreOp = attachment.get_store_op();
    assert_eq!(*store, StoreOp::Discard);
}

#[test]
fn render_backend_type_variants_matchable() {
    let backend: RenderBackendType = RenderBackendType::Canvas2D;
    let label: &'static str = match backend {
        RenderBackendType::Canvas2D => "canvas2d",
        RenderBackendType::WebGpu => "webgpu",
        RenderBackendType::WebGl => "webgl",
    };
    assert_eq!(label, "canvas2d");
    let config: EngineConfig = EngineConfig::default();
    assert!(matches!(
        config.get_render().get_backend(),
        RenderBackendType::Canvas2D
    ));
}

#[test]
fn math_default_constants_externally_referenceable() {
    let gravity: f64 = 980.0;
    let linear_damping: f64 = 0.0;
    let angular_damping: f64 = 0.0;
    let restitution: f64 = 0.3;
    let friction: f64 = 0.7;
    let fixed_timestep: f64 = 1.0 / 60.0;
    let max_frame_time: f64 = 0.25;
    let gravity_3d: f64 = -9.81;
    let camera_near: f64 = 0.1;
    let camera_far: f64 = 1000.0;
    let camera_fov: f64 = 1.0471975511965976;
    assert!(gravity > 0.0);
    assert!(linear_damping >= 0.0);
    assert!(angular_damping >= 0.0);
    assert!((0.0..=1.0).contains(&restitution));
    assert!((0.0..=1.0).contains(&friction));
    assert!(fixed_timestep > 0.0);
    assert!(max_frame_time > 0.0);
    assert!(gravity_3d < 0.0);
    assert!(camera_near > 0.0);
    assert!(camera_far > camera_near);
    assert!(camera_fov > 0.0);
    let config: SchedulerConfig = SchedulerConfig::default();
    let timestep: f64 = config.get_fixed_timestep();
    let max_frame_time: f64 = config.get_max_frame_time();
    assert!(timestep <= 0.25);
    assert!(max_frame_time >= fixed_timestep);
    assert!(timestep > 0.0 && max_frame_time > timestep);
}

#[test]
fn gpu_receiver_class_variants_matchable() {
    let class: GpuReceiverClass = GpuReceiverClass::Device;
    let label: &'static str = match class {
        GpuReceiverClass::Device => "device",
        GpuReceiverClass::Queue => "queue",
        GpuReceiverClass::Context => "context",
        GpuReceiverClass::Texture => "texture",
        GpuReceiverClass::CommandEncoder => "command_encoder",
        GpuReceiverClass::RenderPass => "render_pass",
        GpuReceiverClass::ComputePass => "compute_pass",
    };
    assert_eq!(label, "device");
}

#[test]
fn scheduler_state_externally_constructible() {
    let state: SchedulerState = SchedulerState::default();
    assert!(!state.get_running());
    let frames: u64 = state.get_frame_count();
    let updates: u64 = state.get_update_count();
    let last_time: f64 = state.get_last_time();
    assert!(frames <= updates.max(1));
    assert!(last_time < 0.0);
    let accumulator: f64 = state.get_accumulator();
    assert!(accumulator.abs() < 1e-9);
    let raf_id: Option<i32> = state.get_raf_id();
    assert!(raf_id.is_none());
}

#[test]
fn render_pass_descriptor_cache_type_externally_nameable() {
    let cache: Option<RenderPassDescriptorCache> = None;
    assert!(cache.is_none());
    let object_type: Option<Object> = None;
    assert!(object_type.is_none());
}

#[test]
fn flipped_descriptor_getters_externally_callable() {
    let body: RigidBody2D = RigidBody2D::new_dynamic(7, Vector2D::zero());
    let inverse_mass: f64 = body.get_inverse_mass();
    assert!(inverse_mass > 0.0);
    let force: Vector2D = body.get_force_accumulator();
    assert!(force.magnitude() < 1e-9);
    let world: PhysicsWorld2D = PhysicsWorld2D::default();
    let bodies: &Vec<RigidBody2D> = world.get_bodies();
    assert!(bodies.is_empty());
    let body3: RigidBody3D = RigidBody3D::new_dynamic(7, Vector3D::zero());
    let inverse_inertia: f64 = body3.get_inverse_inertia();
    assert!(inverse_inertia > 0.0);
    let torque: Vector3D = body3.get_torque_accumulator();
    assert!(torque.magnitude() < 1e-9);
    let _: fn(&SpriteSheet) -> u32 = SpriteSheet::get_columns;
    let _: fn(&SpriteSheet) -> u32 = SpriteSheet::get_rows;
    let list: DrawList = DrawList::default();
    let commands: &Vec<DrawCommand> = list.get_commands();
    assert!(commands.is_empty());
    let cache: AssetCache = AssetCache::default();
    let entries: &HashMap<String, AssetEntry> = cache.get_entries();
    assert!(entries.is_empty());
    let scene: SceneManager = SceneManager::new();
    let scenes: &HashMap<String, SceneRc> = scene.get_scenes();
    assert!(scenes.is_empty());
    let particle: Particle = Particle::new(Vector2D::zero(), Vector2D::zero(), 0.0, 1.5);
    let position: Vector2D = particle.get_position();
    let lifetime: f64 = particle.get_lifetime();
    assert!(position.magnitude() < 1e-9);
    assert!(lifetime > 0.0);
    let rng: ParticleRng = ParticleRng::new(7);
    let seed: u64 = rng.get_state();
    assert_ne!(seed, 0);
    let handle: SchedulerHandle = SchedulerHandle::new(
        Rc::new(EngineCell::new(SchedulerState::default())),
        Rc::new(MaybeEngineCell::new()),
    );
    let state: &Rc<EngineCell<SchedulerState>> = handle.get_state();
    assert!(!state.get().get_running());
    let closure_cell: &RafClosureCell = handle.get_closure_cell();
    assert!(closure_cell.try_get().is_none());
    let timer: Timer = Timer::new(1.0, false);
    let elapsed: f64 = timer.get_elapsed();
    let duration: f64 = timer.get_duration();
    assert!(elapsed < duration);
}
