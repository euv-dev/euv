use super::*;

fn nearest_palette_index(color: &Color) -> usize {
    let mut best_index: usize = 0;
    let mut best_distance: f64 = f64::INFINITY;
    for (index, candidate) in PARTICLE_PALETTE.iter().enumerate() {
        let delta_red: f64 = candidate.get_red() - color.get_red();
        let delta_green: f64 = candidate.get_green() - color.get_green();
        let delta_blue: f64 = candidate.get_blue() - color.get_blue();
        let delta_alpha: f64 = candidate.get_alpha() - color.get_alpha();
        let distance: f64 = delta_red * delta_red
            + delta_green * delta_green
            + delta_blue * delta_blue
            + delta_alpha * delta_alpha;
        if distance < best_distance {
            best_distance = distance;
            best_index = index;
        }
    }
    best_index
}

#[test]
fn quantize_selects_the_nearest_palette_entry() {
    let colors: Vec<Color> = vec![
        Color::new(0.0, 0.0, 0.0, 1.0),
        Color::new(1.0, 1.0, 1.0, 1.0),
        Color::new(1.0, 0.0, 0.0, 1.0),
        Color::new(0.5, 0.5, 0.5, 1.0),
        Color::new(0.9, 0.1, 0.1, 0.5),
        Color::new(0.25, 0.75, 0.25, 1.0),
    ];
    for color in &colors {
        let bucket: u8 = ParticleEmitter::quantize(color);
        assert!((bucket as usize) < PARTICLE_PALETTE_SIZE);
        assert_eq!(bucket as usize, nearest_palette_index(color));
    }
}

#[test]
fn quantize_maps_an_exact_palette_color_to_its_own_index() {
    for (index, candidate) in PARTICLE_PALETTE.iter().enumerate() {
        let bucket: u8 = ParticleEmitter::quantize(candidate);
        assert_eq!(bucket as usize, index);
    }
}
