use super::*;

pub(crate) const EMPTY_ICON_GLYPH: &str = "[ ]";
pub(crate) const TREE_EXPAND_PREFIX: &str = "v ";
pub(crate) const TREE_COLLAPSED_PREFIX: &str = "> ";
pub(crate) const TREE_LEAF_PREFIX: &str = "- ";

/// Precomputed `(cos, sin, alpha)` table for the 8 radial spokes rendered by `EuvSpinner`.
pub(crate) const SPINNER_SPOKES: [(f32, f32, f32); 8] = [
    (1.0, 0.0, 0.22 + 0.78 * (1.0 / 8.0)),
    (FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.22 + 0.78 * (2.0 / 8.0)),
    (0.0, 1.0, 0.22 + 0.78 * (3.0 / 8.0)),
    (-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.22 + 0.78 * (4.0 / 8.0)),
    (-1.0, 0.0, 0.22 + 0.78 * (5.0 / 8.0)),
    (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.22 + 0.78 * (6.0 / 8.0)),
    (0.0, -1.0, 0.22 + 0.78 * (7.0 / 8.0)),
    (FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 1.0),
];
