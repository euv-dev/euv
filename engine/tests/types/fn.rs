use super::*;

fn shape_of<T: Collider>(value: T) -> ColliderShape {
    value.shape()
}

fn center_of<T: Collider>(value: T) -> Vector2D {
    value.center()
}

fn shape3d_of<T: Collider3D>(value: T) -> ColliderShape3D {
    value.shape()
}

#[test]
fn the_three_asset_types_are_mutually_distinct() {
    let all: [AssetType; 3] = [AssetType::Image, AssetType::Audio, AssetType::Data];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
    assert_eq!(AssetType::default(), AssetType::Image);
}

#[test]
fn the_three_asset_states_are_mutually_distinct() {
    let all: [AssetState; 3] = [AssetState::Loading, AssetState::Loaded, AssetState::Error];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
    assert_eq!(
        AssetState::default(),
        AssetState::Loading,
        "an asset starts life as Loading, never as a finished state"
    );
}

#[test]
fn an_asset_type_is_hashable_so_it_can_key_the_cache() {
    let kinds: HashSet<AssetType> = [
        AssetType::Image,
        AssetType::Audio,
        AssetType::Data,
        AssetType::Image,
    ]
    .into_iter()
    .collect();
    assert_eq!(kinds.len(), 3, "the repeated Image must collapse");
}

#[test]
fn the_three_audio_play_states_are_mutually_distinct() {
    let all: [AudioPlayState; 3] = [
        AudioPlayState::Playing,
        AudioPlayState::Paused,
        AudioPlayState::Stopped,
    ];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
    assert_eq!(AudioPlayState::default(), AudioPlayState::Playing);
}

#[test]
fn a_fresh_asset_cache_has_nothing_loaded() {
    let cache: AssetCache = AssetCache::default();
    assert_eq!(
        cache.loaded_count(),
        0,
        "a cache that has loaded nothing must report zero"
    );
}

#[test]
fn a_fresh_asset_cache_reports_an_unknown_key_as_absent() {
    let cache: AssetCache = AssetCache::default();
    assert!(
        cache.get_state("missing.png").is_none(),
        "an unknown key must read as None rather than panic"
    );
    assert!(cache.get_image("missing.png").is_none());
}

#[test]
fn the_closure_store_alias_is_a_shared_cell() {
    let store: AssetClosures = Rc::new(EngineCell::new(AssetClosureStore::default()));
    let cloned: AssetClosures = Rc::clone(&store);
    assert!(
        Rc::ptr_eq(&store, &cloned),
        "the alias is an std::rc::Rc, so a clone must share one allocation"
    );
}

#[test]
fn the_pending_counter_alias_starts_at_zero_and_is_writable() {
    let pending: AssetPending = Rc::new(EngineCell::new(0));
    assert_eq!(*pending.get(), 0);
    *pending.get_mut() = 3;
    assert_eq!(
        *pending.get(),
        3,
        "the counter is a plain u32 behind a cell"
    );
}

#[test]
fn an_aabb_collider_reports_the_aabb_shape() {
    let collider: AabbCollider = AabbCollider::new(Rect::new(0.0, 0.0, 4.0, 2.0));
    assert_eq!(collider.shape(), ColliderShape::Aabb);
    assert_eq!(shape_of(collider), ColliderShape::Aabb);
}

#[test]
fn a_circle_collider_reports_the_circle_shape() {
    let collider: CircleCollider = CircleCollider::from_center(Vector2D::new(1.0, 2.0), 3.0);
    assert_eq!(collider.shape(), ColliderShape::Circle);
    assert_eq!(shape_of(collider), ColliderShape::Circle);
}

#[test]
fn a_circle_collider_centre_is_its_own_circle_centre() {
    let collider: CircleCollider = CircleCollider::from_center(Vector2D::new(1.0, 2.0), 3.0);
    let observed: Vector2D = center_of(collider);
    assert!(
        (observed.get_x() - 1.0).abs() < f64::EPSILON,
        "a circle's collider centre is the circle's centre, got {}",
        observed.get_x()
    );
    assert!((observed.get_y() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn a_2d_collider_contains_a_point_inside_and_rejects_one_outside() {
    let rect: AabbCollider = AabbCollider::new(Rect::new(0.0, 0.0, 4.0, 2.0));
    assert!(rect.contains_point(Vector2D::new(1.0, 1.0)));
    assert!(!rect.contains_point(Vector2D::new(9.0, 1.0)));

    let circle: CircleCollider = CircleCollider::from_center(Vector2D::new(0.0, 0.0), 2.0);
    assert!(circle.contains_point(Vector2D::new(1.0, 0.0)));
    assert!(
        !circle.contains_point(Vector2D::new(5.0, 0.0)),
        "a point well outside the radius must be rejected"
    );
}

#[test]
fn a_3d_box_collider_reports_the_aabb_shape() {
    let collider: AabbCollider3D =
        AabbCollider3D::new(AABB3D::new(Vector3D::zero(), Vector3D::new(1.0, 1.0, 1.0)));
    assert_eq!(collider.shape(), ColliderShape3D::Aabb);
    assert_eq!(shape3d_of(collider), ColliderShape3D::Aabb);
}

#[test]
fn a_3d_sphere_collider_reports_the_sphere_shape() {
    let collider: SphereCollider3D = SphereCollider3D::new(Sphere::new(Vector3D::zero(), 1.0));
    assert_eq!(collider.shape(), ColliderShape3D::Sphere);
    assert_eq!(shape3d_of(collider), ColliderShape3D::Sphere);
}

#[test]
fn the_two_collider_traits_report_different_shape_enums() {
    let two: ColliderShape = shape_of(AabbCollider::default());
    let three: ColliderShape3D = shape3d_of(AabbCollider3D::default());
    assert_eq!(format!("{two:?}"), "Aabb");
    assert_eq!(format!("{three:?}"), "Aabb");
    assert_eq!(ColliderShape::default(), ColliderShape::Aabb);
    assert_eq!(ColliderShape3D::default(), ColliderShape3D::Aabb);
}
