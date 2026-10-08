use super::*;

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

fn vec2_close(left: Vector2D, right: Vector2D) -> bool {
    close(left.get_x(), right.get_x()) && close(left.get_y(), right.get_y())
}

fn world() -> PhysicsWorld2D {
    PhysicsWorld2D::with_config(PhysicsConfig::default())
}

fn body(id: u64, x: f64, y: f64) -> RigidBody2D {
    RigidBody2D::new_dynamic(id, Vector2D::new(x, y))
}

#[test]
fn a_fresh_world_holds_no_bodies() {
    let observed: PhysicsWorld2D = world();
    assert!(
        observed.get_bodies().is_empty(),
        "nothing is registered up front"
    );
    assert_eq!(observed.get_body(1), None, "so no id resolves");
}

#[test]
fn an_added_body_is_found_by_its_id() {
    let mut world: PhysicsWorld2D = world();
    world.add_body(body(7, 1.0, 2.0));
    let observed: Option<&RigidBody2D> = world.get_body(7);
    match observed {
        Some(found) => assert_eq!(found.get_id(), 7, "the id round-trips"),
        None => panic!("an added body must be findable"),
    }
}

#[test]
fn bodies_keep_the_position_they_were_built_with() {
    let mut world: PhysicsWorld2D = world();
    world.add_body(body(1, 30.0, -40.0));
    let found: &RigidBody2D = world.get_body(1).expect("body 1");
    let position: Vector2D = found.get_position();
    assert!(
        close(position.get_x(), 30.0) && close(position.get_y(), -40.0),
        "add_body must not relocate the body, got {position:?}"
    );
}

#[test]
fn bodies_are_appended_in_insertion_order() {
    let mut world: PhysicsWorld2D = world();
    world.add_body(body(1, 0.0, 0.0));
    world.add_body(body(2, 10.0, 0.0));
    world.add_body(body(3, 20.0, 0.0));
    let ids: Vec<u64> = world
        .get_bodies()
        .iter()
        .map(|b: &RigidBody2D| b.get_id())
        .collect();
    assert_eq!(ids, vec![1, 2, 3], "the world keeps insertion order");
}

#[test]
fn removing_a_body_takes_only_the_matching_id() {
    let mut world: PhysicsWorld2D = world();
    world.add_body(body(1, 0.0, 0.0));
    world.add_body(body(2, 10.0, 0.0));
    world.add_body(body(3, 20.0, 0.0));
    world.remove_body(2);
    assert_eq!(world.get_body(2), None, "the named body is gone");
    assert!(world.get_body(1).is_some(), "the earlier body survives");
    assert!(world.get_body(3).is_some(), "and so does the later one");
}

#[test]
fn removing_every_body_leaves_an_empty_world() {
    let mut world: PhysicsWorld2D = world();
    world.add_body(body(1, 0.0, 0.0));
    world.add_body(body(2, 10.0, 0.0));
    world.remove_body(1);
    world.remove_body(2);
    assert!(world.get_bodies().is_empty(), "the registry is drained");
}

#[test]
fn removing_an_unknown_id_is_harmless() {
    let mut world: PhysicsWorld2D = world();
    world.add_body(body(1, 0.0, 0.0));
    world.remove_body(99);
    assert_eq!(world.get_bodies().len(), 1, "the real body is untouched");
}

#[test]
fn removing_an_id_twice_is_harmless() {
    let mut world: PhysicsWorld2D = world();
    world.add_body(body(1, 0.0, 0.0));
    world.add_body(body(2, 10.0, 0.0));
    world.remove_body(1);
    world.remove_body(1);
    assert_eq!(
        world.get_bodies().len(),
        1,
        "a repeated removal changes nothing more"
    );
}

#[test]
fn the_mutable_accessor_hands_back_the_live_body() {
    let mut world: PhysicsWorld2D = world();
    world.add_body(body(5, 0.0, 0.0));
    let mass: f64 = world.get_body(5).expect("body 5").get_mass();
    let found: Option<&mut RigidBody2D> = world.get_body_mut(5);
    match found {
        Some(target) => {
            target.apply_impulse(Vector2D::new(9.0 * mass, 0.0));
        }
        None => panic!("an added body must be reachable mutably"),
    }
    let after: &RigidBody2D = world.get_body(5).expect("body 5");
    assert!(
        close(after.get_velocity().get_x(), 9.0),
        "the write must land on the world's own copy, got {:?}",
        after.get_velocity()
    );
}

#[test]
fn the_mutable_accessor_is_none_for_an_unknown_id() {
    let mut world: PhysicsWorld2D = world();
    let observed: Option<&mut RigidBody2D> = world.get_body_mut(42);
    assert!(observed.is_none(), "there is nothing to hand out");
}

#[test]
fn a_body_without_a_collider_has_no_bounding_box() {
    let body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(5.0, 5.0));
    assert_eq!(
        body.bounding_box(),
        None,
        "a body is just a point until a collider is attached"
    );
}

#[test]
fn a_circle_collider_bounds_a_square_twice_the_radius() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(100.0, 200.0));
    body.update_collider(BodyCollider::Circle(CircleCollider::from_center(
        Vector2D::zero(),
        5.0,
    )));
    let observed: Option<Rect> = body.bounding_box();
    match observed {
        Some(rect) => {
            assert!(
                close(rect.get_width(), 10.0),
                "the box is twice the radius, got {rect:?}"
            );
            assert!(close(rect.get_height(), 10.0), "and square");
        }
        None => panic!("a circle collider always bounds a box"),
    }
}

#[test]
fn a_circle_collider_is_centred_on_the_body_position() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(100.0, 200.0));
    body.update_collider(BodyCollider::Circle(CircleCollider::from_center(
        Vector2D::zero(),
        5.0,
    )));
    let observed: Option<Rect> = body.bounding_box();
    match observed {
        Some(rect) => assert!(
            vec2_close(rect.center(), Vector2D::new(100.0, 200.0)),
            "the box follows the body, got {:?}",
            rect.center()
        ),
        None => panic!("a circle collider always bounds a box"),
    }
}

#[test]
fn an_aabb_collider_is_offset_by_the_body_position() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::zero());
    body.update_collider(BodyCollider::Aabb(AabbCollider::from_center(
        Vector2D::new(50.0, 50.0),
        10.0,
        10.0,
    )));
    let observed: Option<Rect> = body.bounding_box();
    match observed {
        Some(rect) => assert!(
            vec2_close(rect.center(), Vector2D::new(50.0, 50.0)),
            "a collider authored around (50, 50) with a body at the origin is \
             unmoved — subtracting half the size a second time would displace it \
             to (45, 45), got {:?}",
            rect.center()
        ),
        None => panic!("an aabb collider always bounds a box"),
    }
}

#[test]
fn an_aabb_collider_keeps_its_size() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::zero());
    body.update_collider(BodyCollider::Aabb(AabbCollider::from_center(
        Vector2D::new(50.0, 50.0),
        10.0,
        10.0,
    )));
    let observed: Option<Rect> = body.bounding_box();
    match observed {
        Some(rect) => {
            assert!(
                close(rect.get_width(), 10.0),
                "the width is the collider's, got {rect:?}"
            );
            assert!(close(rect.get_height(), 10.0), "and so is the height");
        }
        None => panic!("an aabb collider always bounds a box"),
    }
}

#[test]
fn an_aabb_collider_follows_the_body_it_is_attached_to() {
    let mut near: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(0.0, 0.0));
    let mut far: RigidBody2D = RigidBody2D::new_dynamic(2, Vector2D::new(100.0, 0.0));
    let collider: BodyCollider =
        BodyCollider::Aabb(AabbCollider::from_center(Vector2D::zero(), 4.0, 4.0));
    near.update_collider(collider);
    far.update_collider(BodyCollider::Aabb(AabbCollider::from_center(
        Vector2D::zero(),
        4.0,
        4.0,
    )));
    let near_box: Option<Rect> = near.bounding_box();
    let far_box: Option<Rect> = far.bounding_box();
    match (near_box, far_box) {
        (Some(a), Some(b)) => assert!(
            vec2_close(a.center(), Vector2D::zero())
                && vec2_close(b.center(), Vector2D::new(100.0, 0.0)),
            "identical colliders on different bodies must separate, got {:?} and {:?}",
            a.center(),
            b.center()
        ),
        _ => panic!("both bodies have colliders attached"),
    }
}

#[test]
fn a_body_built_at_an_offset_bounds_its_own_way_round() {
    let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::new(50.0, 0.0));
    body.update_collider(BodyCollider::Circle(CircleCollider::from_center(
        Vector2D::zero(),
        2.0,
    )));
    let observed: Option<Rect> = body.bounding_box();
    match observed {
        Some(rect) => assert!(
            vec2_close(rect.center(), Vector2D::new(50.0, 0.0)),
            "the box is built around wherever the body already is, got {:?}",
            rect.center()
        ),
        None => panic!("the collider is still attached"),
    }
}

#[test]
fn a_bigger_circle_collider_makes_a_bigger_box() {
    let small: Rect = {
        let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::zero());
        body.update_collider(BodyCollider::Circle(CircleCollider::from_center(
            Vector2D::zero(),
            1.0,
        )));
        body.bounding_box().expect("box")
    };
    let large: Rect = {
        let mut body: RigidBody2D = RigidBody2D::new_dynamic(1, Vector2D::zero());
        body.update_collider(BodyCollider::Circle(CircleCollider::from_center(
            Vector2D::zero(),
            8.0,
        )));
        body.bounding_box().expect("box")
    };
    assert!(
        large.get_width() > small.get_width(),
        "eight times the radius is eight times the box, got {} vs {}",
        large.get_width(),
        small.get_width()
    );
}
