use super::*;

fn fixed_lifetime_config(lifetime: f64, max_particles: usize) -> ParticleConfig {
    let mut config: ParticleConfig = ParticleConfig::default();
    config.set_emission_rate(0.0);
    config.set_lifetime_min(lifetime);
    config.set_lifetime_max(lifetime);
    config.set_max_particles(max_particles);
    config.set_gravity(Vector2D::zero());
    config
}

fn uniform_life_config(lifetime: f64, max_particles: usize) -> ParticleConfig {
    fixed_lifetime_config(lifetime, max_particles)
}

fn positions(emitter: &ParticleEmitter) -> Vec<Vector2D> {
    let mut out: Vec<Vector2D> = Vec::new();
    for particle in emitter.get_particles().iter() {
        out.push(particle.get_position());
    }
    out
}

#[test]
fn burst_spawns_exactly_the_requested_number_of_particles() {
    let config: ParticleConfig = uniform_life_config(10.0, 64);
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.burst(7);
    assert_eq!(
        emitter.alive_count(),
        7,
        "burst must spawn exactly the requested particle count"
    );
}

#[test]
fn burst_clamps_to_the_configured_max_particle_budget() {
    let config: ParticleConfig = uniform_life_config(10.0, 4);
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.burst(9);
    assert_eq!(
        emitter.alive_count(),
        4,
        "burst must not exceed max_particles"
    );
}

#[test]
fn update_retires_particles_once_their_age_reaches_their_lifetime() {
    let config: ParticleConfig = uniform_life_config(1.0, 64);
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.burst(3);
    assert_eq!(emitter.alive_count(), 3, "burst must populate the emitter");
    emitter.update(0.5);
    assert_eq!(
        emitter.alive_count(),
        3,
        "particles below their lifetime must survive the update"
    );
    emitter.update(0.5);
    assert_eq!(
        emitter.alive_count(),
        0,
        "particles reaching their lifetime must be retired by the same update"
    );
}

#[test]
fn inactive_emitter_still_retires_the_particles_it_already_spawned() {
    let config: ParticleConfig = uniform_life_config(1.0, 64);
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.burst(2);
    emitter.set_active(false);
    emitter.update(2.0);
    assert_eq!(
        emitter.alive_count(),
        0,
        "ageing out is independent of the emission flag"
    );
}

#[test]
fn update_integrates_velocity_into_position() {
    let config: ParticleConfig = uniform_life_config(10.0, 64);
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.burst(1);
    let before: Vector2D = positions(&emitter)[0];
    emitter.update(0.25);
    let after: Vector2D = positions(&emitter)[0];
    assert!(
        (after.get_x() - before.get_x()).abs() > 0.0
            || (after.get_y() - before.get_y()).abs() > 0.0,
        "a particle with non-zero velocity must move once integrated"
    );
}

#[test]
fn update_applies_gravity_to_particle_velocity() {
    let mut config: ParticleConfig = ParticleConfig::default();
    config.set_emission_rate(0.0);
    config.set_lifetime_min(10.0);
    config.set_lifetime_max(10.0);
    config.set_max_particles(64);
    config.set_speed_min(0.0);
    config.set_speed_max(0.0);
    config.set_gravity(Vector2D::new(0.0, 100.0));
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.burst(1);
    emitter.update(0.5);
    let velocity: Vector2D = emitter.get_particles()[0].get_velocity();
    assert!(
        velocity.get_y() > 0.0,
        "downward gravity must add downward velocity, got {velocity:?}"
    );
}

#[test]
fn surviving_particles_keep_their_relative_order_after_compaction() {
    let mut config: ParticleConfig = ParticleConfig::default();
    config.set_emission_rate(0.0);
    config.set_lifetime_min(0.2);
    config.set_lifetime_max(1.0);
    config.set_max_particles(64);
    config.set_speed_min(1.0);
    config.set_speed_max(4.0);
    config.set_gravity(Vector2D::zero());
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.burst(24);
    let mut before: Vec<f64> = Vec::new();
    for particle in emitter.get_particles().iter() {
        before.push(particle.get_velocity().get_x());
    }
    assert_eq!(before.len(), 24, "twenty four particles were spawned");
    emitter.update(0.35);
    let mut after: Vec<f64> = Vec::new();
    for particle in emitter.get_particles().iter() {
        after.push(particle.get_velocity().get_x());
    }
    assert!(
        after.len() < before.len(),
        "the varying lifetimes must retire at least one particle, {} left of {}",
        after.len(),
        before.len()
    );
    let mut cursor: usize = 0;
    for identity in after.iter() {
        while cursor < before.len() && before[cursor] != *identity {
            cursor += 1;
        }
        assert!(
            cursor < before.len(),
            "every survivor must come from the original particle set"
        );
        cursor += 1;
    }
}

#[test]
fn update_retires_particles_that_passed_their_lifetime_in_an_earlier_step() {
    let config: ParticleConfig = uniform_life_config(1.0, 64);
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.burst(4);
    emitter.update(0.6);
    assert_eq!(
        emitter.alive_count(),
        4,
        "the first step keeps every particle"
    );
    emitter.update(0.6);
    assert_eq!(
        emitter.alive_count(),
        0,
        "the second step pushes every particle past its lifetime"
    );
}

#[test]
fn clear_drops_every_particle_and_rewinds_the_emit_budget() {
    let config: ParticleConfig = uniform_life_config(100.0, 64);
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.burst(5);
    emitter.update(0.1);
    emitter.clear();
    assert_eq!(emitter.alive_count(), 0, "clear must drop every particle");
    assert_eq!(
        emitter.get_emit_accumulator(),
        0.0,
        "clear must rewind the fractional spawn budget"
    );
}

#[test]
fn active_emitter_accrues_particles_from_its_emission_rate() {
    let mut config: ParticleConfig = ParticleConfig::default();
    config.set_lifetime_min(100.0);
    config.set_lifetime_max(100.0);
    config.set_max_particles(64);
    config.set_emission_rate(10.0);
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.update(0.5);
    assert_eq!(
        emitter.alive_count(),
        5,
        "an emission rate of ten per second must yield five particles per half second"
    );
}
#[test]
fn seeded_random_generator_is_reproducible() {
    let mut first: ParticleRng = ParticleRng::with_seed(12345);
    let mut second: ParticleRng = ParticleRng::with_seed(12345);
    for _ in 0..8 {
        let left: u64 = first.next_u64();
        let right: u64 = second.next_u64();
        assert_eq!(left, right, "the same seed must produce the same stream");
    }
}

#[test]
fn zero_seed_falls_back_to_the_default_seed() {
    let mut from_zero: ParticleRng = ParticleRng::with_seed(0);
    let mut from_default: ParticleRng = ParticleRng::default();
    let zeroed: u64 = from_zero.next_u64();
    let defaulted: u64 = from_default.next_u64();
    assert_eq!(
        zeroed, defaulted,
        "a zero seed must be replaced by the default seed"
    );
}

#[test]
fn next_f64_stays_inside_the_unit_interval() {
    let mut rng: ParticleRng = ParticleRng::with_seed(99);
    for _ in 0..64 {
        let value: f64 = rng.next_f64();
        assert!(
            (0.0..1.0).contains(&value),
            "next_f64 must stay in [0, 1), got {value}"
        );
    }
}

#[test]
fn range_stays_within_the_requested_bounds() {
    let mut rng: ParticleRng = ParticleRng::with_seed(7);
    for _ in 0..64 {
        let value: f64 = rng.range(-2.0, 5.0);
        assert!(
            (-2.0..5.0).contains(&value),
            "range must stay within its bounds, got {value}"
        );
    }
}

#[test]
fn render_records_one_circle_per_visible_particle() {
    let mut config: ParticleConfig = ParticleConfig::default();
    config.set_emission_rate(0.0);
    config.set_lifetime_min(100.0);
    config.set_lifetime_max(100.0);
    config.set_max_particles(64);
    config.set_size_start(4.0);
    config.set_size_end(4.0);
    let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
    emitter.burst(3);
    let mut list: DrawList = DrawList::default();
    emitter.render(&mut list);
    assert_eq!(
        list.get_commands().len(),
        3,
        "every visible particle must contribute one draw command"
    );
}

#[test]
fn an_emitter_created_with_defaults_keeps_its_position_and_the_default_config() {
    let position: Vector2D = Vector2D::new(12.0, -3.0);
    let emitter: ParticleEmitter = ParticleEmitter::with_defaults(position);
    assert_eq!(emitter.get_position().get_x(), 12.0);
    assert_eq!(emitter.get_position().get_y(), -3.0);
    let config: ParticleConfig = emitter.get_config();
    assert_eq!(
        config.get_lifetime_min(),
        ParticleConfig::default().get_lifetime_min(),
        "with_defaults must not invent a config of its own"
    );
    assert_eq!(emitter.alive_count(), 0, "a fresh emitter holds no particles");
}

#[test]
fn two_emitters_created_with_defaults_share_one_config_shape() {
    let first: ParticleEmitter = ParticleEmitter::with_defaults(Vector2D::zero());
    let second: ParticleEmitter = ParticleEmitter::with_defaults(Vector2D::new(5.0, 5.0));
    assert_eq!(
        first.get_config().get_lifetime_max(),
        second.get_config().get_lifetime_max()
    );
    assert_eq!(first.get_position().get_x(), 0.0);
    assert_eq!(second.get_position().get_x(), 5.0);
}
