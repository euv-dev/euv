use super::*;

#[test]
fn the_same_seed_reproduces_the_same_sequence() {
    let mut first: ParticleRng = ParticleRng::with_seed(42);
    let mut second: ParticleRng = ParticleRng::with_seed(42);
    for _ in 0..16 {
        let left: u64 = first.next_u64();
        let right: u64 = second.next_u64();
        assert_eq!(left, right, "an identical seed must replay identically");
    }
}

#[test]
fn different_seeds_diverge_immediately() {
    let mut first: ParticleRng = ParticleRng::with_seed(1);
    let mut second: ParticleRng = ParticleRng::with_seed(2);
    let left: u64 = first.next_u64();
    let right: u64 = second.next_u64();
    assert_ne!(left, right, "different seeds must not share a first draw");
}

#[test]
fn a_zero_seed_falls_back_to_the_default_seed() {
    let mut zeroed: ParticleRng = ParticleRng::with_seed(0);
    let mut defaulted: ParticleRng = ParticleRng::default();
    let left: u64 = zeroed.next_u64();
    let right: u64 = defaulted.next_u64();
    assert_eq!(
        left, right,
        "a zero seed must be replaced, since xorshift degenerates at zero"
    );
}

#[test]
fn the_default_generator_replays_from_the_zero_seed_fallback() {
    let mut defaulted: ParticleRng = ParticleRng::default();
    let mut zeroed: ParticleRng = ParticleRng::with_seed(0);
    let left: u64 = defaulted.next_u64();
    let right: u64 = zeroed.next_u64();
    assert_eq!(
        left, right,
        "Default must be the same generator the zero-seed fallback produces"
    );
}

#[test]
fn a_long_run_never_repeats_a_draw() {
    let mut rng: ParticleRng = ParticleRng::with_seed(7);
    let mut seen: Vec<u64> = Vec::new();
    for _ in 0..256 {
        seen.push(rng.next_u64());
    }
    let before: usize = seen.len();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(
        seen.len(),
        before,
        "a 256-draw xorshift run must not repeat a value"
    );
}

#[test]
fn next_f64_stays_inside_the_unit_interval() {
    let mut rng: ParticleRng = ParticleRng::with_seed(99);
    for _ in 0..512 {
        let observed: f64 = rng.next_f64();
        assert!(
            (0.0..1.0).contains(&observed),
            "next_f64 must land in [0.0, 1.0), got {observed}"
        );
    }
}

#[test]
fn next_f64_is_reproducible_for_a_fixed_seed() {
    let mut first: ParticleRng = ParticleRng::with_seed(2024);
    let mut second: ParticleRng = ParticleRng::with_seed(2024);
    for _ in 0..32 {
        let left: f64 = first.next_f64();
        let right: f64 = second.next_f64();
        assert_eq!(left, right, "the float stream must replay identically");
    }
}

#[test]
fn next_f64_covers_both_halves_of_the_interval() {
    let mut rng: ParticleRng = ParticleRng::with_seed(5);
    let mut low: u32 = 0;
    let mut high: u32 = 0;
    for _ in 0..256 {
        let observed: f64 = rng.next_f64();
        if observed < 0.5 {
            low += 1;
        } else {
            high += 1;
        }
    }
    assert!(low > 0, "some draws must land in the lower half");
    assert!(high > 0, "some draws must land in the upper half");
}

#[test]
fn range_returns_the_lower_bound_when_the_span_is_zero() {
    let mut rng: ParticleRng = ParticleRng::with_seed(11);
    let observed: f64 = rng.range(3.0, 3.0);
    assert_eq!(observed, 3.0, "a zero-width range collapses to its bound");
}

#[test]
fn range_stays_inside_the_requested_window() {
    let mut rng: ParticleRng = ParticleRng::with_seed(13);
    for _ in 0..256 {
        let observed: f64 = rng.range(-4.0, 9.0);
        assert!(
            (-4.0..9.0).contains(&observed),
            "range must stay in [-4.0, 9.0), got {observed}"
        );
    }
}

#[test]
fn range_handles_a_descending_window() {
    let mut rng: ParticleRng = ParticleRng::with_seed(17);
    for _ in 0..64 {
        let observed: f64 = rng.range(5.0, 1.0);
        assert!(
            observed <= 5.0 && observed > 1.0,
            "a descending range runs from max down to min, got {observed}"
        );
    }
}

#[test]
fn range_is_reproducible_for_a_fixed_seed() {
    let mut first: ParticleRng = ParticleRng::with_seed(31);
    let mut second: ParticleRng = ParticleRng::with_seed(31);
    for _ in 0..32 {
        let left: f64 = first.range(0.0, 100.0);
        let right: f64 = second.range(0.0, 100.0);
        assert_eq!(left, right, "the range stream must replay identically");
    }
}

#[test]
fn range_reaches_both_ends_of_its_window_over_a_long_run() {
    let mut rng: ParticleRng = ParticleRng::with_seed(23);
    let mut low: f64 = f64::MAX;
    let mut high: f64 = f64::MIN;
    for _ in 0..1024 {
        let observed: f64 = rng.range(0.0, 1.0);
        if observed < low {
            low = observed;
        }
        if observed > high {
            high = observed;
        }
    }
    assert!(
        low < 0.05,
        "a long run must reach near the lower bound, got {low}"
    );
    assert!(
        high > 0.95,
        "a long run must reach near the upper bound, got {high}"
    );
}

fn emitter_with(max_particles: usize) -> ParticleEmitter {
    let config: ParticleConfig = ParticleConfig::new(1.0, max_particles, 5.0, 5.0, 1.0, 1.0);
    ParticleEmitter::create(Vector2D::zero(), config)
}

#[test]
fn a_fresh_emitter_has_no_live_particles() {
    let emitter: ParticleEmitter = ParticleEmitter::with_defaults(Vector2D::zero());
    assert_eq!(
        emitter.alive_count(),
        0,
        "nothing spawns before time passes"
    );
}

#[test]
fn a_burst_spawns_exactly_the_requested_particles() {
    let mut emitter: ParticleEmitter = emitter_with(100);
    emitter.burst(7);
    assert_eq!(emitter.alive_count(), 7, "a burst of seven spawns seven");
}

#[test]
fn a_burst_cannot_exceed_the_configured_capacity() {
    let mut emitter: ParticleEmitter = emitter_with(4);
    emitter.burst(50);
    assert_eq!(
        emitter.alive_count(),
        4,
        "the pool caps a burst at the configured maximum"
    );
}

#[test]
fn a_second_burst_stacks_on_top_of_the_first() {
    let mut emitter: ParticleEmitter = emitter_with(100);
    emitter.burst(3);
    emitter.burst(2);
    assert_eq!(
        emitter.alive_count(),
        5,
        "bursts accumulate while there is room"
    );
}

#[test]
fn a_burst_into_a_full_pool_spawns_nothing() {
    let mut emitter: ParticleEmitter = emitter_with(4);
    emitter.burst(4);
    emitter.burst(4);
    assert_eq!(
        emitter.alive_count(),
        4,
        "a full pool absorbs a further burst"
    );
}

#[test]
fn a_zero_burst_does_nothing() {
    let mut emitter: ParticleEmitter = emitter_with(10);
    emitter.burst(0);
    assert_eq!(emitter.alive_count(), 0, "a burst of zero is a no-op");
}

#[test]
fn a_repeating_update_accumulates_towards_the_rate() {
    let mut emitter: ParticleEmitter = emitter_with(100);
    emitter.update(0.6);
    emitter.update(0.6);
    assert_eq!(
        emitter.alive_count(),
        1,
        "neither update alone reaches a whole particle, but together they do"
    );
}

#[test]
fn an_active_emitter_spawns_according_to_its_rate() {
    let mut emitter: ParticleEmitter = emitter_with(100);
    emitter.update(1.5);
    assert!(
        emitter.alive_count() > 0,
        "one and a half seconds at one particle per second crosses the budget"
    );
    assert!(
        emitter.alive_count() <= 100,
        "and never more than the configured maximum"
    );
}

#[test]
fn an_update_with_no_elapsed_time_emits_nothing() {
    let mut emitter: ParticleEmitter = emitter_with(100);
    emitter.update(0.0);
    assert_eq!(
        emitter.alive_count(),
        0,
        "emission is driven by accumulated time, not by the call itself"
    );
}

fn short_lived_emitter() -> ParticleEmitter {
    let config: ParticleConfig = ParticleConfig::new(1.0, 100, 0.5, 0.5, 1.0, 1.0);
    ParticleEmitter::create(Vector2D::zero(), config)
}

#[test]
fn a_particle_spawned_and_aged_in_the_same_update_survives_when_it_fits() {
    let mut emitter: ParticleEmitter = emitter_with(100);
    emitter.update(1.5);
    assert_eq!(
        emitter.alive_count(),
        1,
        "a particle born this update is aged by the same delta, so it lives \
         as long as its own lifetime is at least that long"
    );
}

#[test]
fn a_particle_spawned_and_aged_in_the_same_update_dies_when_the_delta_outlives_it() {
    let mut emitter: ParticleEmitter = short_lived_emitter();
    emitter.update(2.0);
    assert_eq!(
        emitter.alive_count(),
        0,
        "emission happens before ageing in the same update, so a long frame \
         spawns a particle and then immediately outlives it"
    );
}

#[test]
fn the_same_particle_survives_a_shorter_frame() {
    let mut emitter: ParticleEmitter = short_lived_emitter();
    emitter.update(0.2);
    assert_eq!(
        emitter.alive_count(),
        0,
        "a fifth of a second has not yet reached the one-particle budget"
    );
    emitter.update(0.2);
    emitter.update(0.2);
    emitter.update(0.2);
    emitter.update(0.2);
    assert_eq!(
        emitter.alive_count(),
        1,
        "frames short enough for the particle's own half-second keep it alive"
    );
}

fn circles(list: &DrawList) -> usize {
    list.get_commands()
        .iter()
        .filter(|command: &&DrawCommand| matches!(command, DrawCommand::FillCircle { .. }))
        .count()
}

#[test]
fn rendering_a_burst_draws_one_circle_per_live_particle() {
    let mut emitter: ParticleEmitter = ParticleEmitter::with_defaults(Vector2D::new(0.0, 0.0));
    emitter.burst(5);
    let mut list: DrawList = DrawList::create();

    emitter.render(&mut list);

    assert_eq!(
        circles(&list),
        5,
        "every live particle contributes exactly one filled circle, so the rendered count has \
         to match the live count or the emitter is drawing ghost particles"
    );
}

#[test]
fn a_particle_that_has_run_out_of_life_is_dropped_from_the_frame() {
    let mut emitter: ParticleEmitter = ParticleEmitter::with_defaults(Vector2D::new(0.0, 0.0));
    emitter.burst(1);
    let mut list: DrawList = DrawList::create();

    emitter.update(1_000.0);
    emitter.render(&mut list);

    assert_eq!(
        circles(&list),
        0,
        "a dead particle has to stop being drawn. Drawing it with a zero radius would be \
         harmless on its own, but the renderer would still walk it every frame forever"
    );
}

#[test]
fn rendering_an_emitter_with_nothing_alive_touches_nothing() {
    let emitter: ParticleEmitter = ParticleEmitter::with_defaults(Vector2D::new(0.0, 0.0));
    let mut list: DrawList = DrawList::create();

    emitter.render(&mut list);

    assert_eq!(
        list.len(),
        0,
        "an emitter that never emitted must not disturb commands another system recorded"
    );
}
