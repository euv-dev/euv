use super::*;

#[test]
fn native_color_parser_accepts_prefixed_unprefixed_and_uppercase_channels() {
    assert_eq!(game_3d_hex_to_rgb("#ff0000"), (1.0, 0.0, 0.0));
    assert_eq!(game_3d_hex_to_rgb("00FF00"), (0.0, 1.0, 0.0));
    assert_eq!(game_3d_hex_to_rgb("#0000ff"), (0.0, 0.0, 1.0));
}

#[test]
fn native_color_parser_falls_back_for_missing_or_invalid_channels() {
    assert_eq!(game_3d_hex_to_rgb(""), (1.0, 1.0, 1.0));
    assert_eq!(game_3d_hex_to_rgb("#gggggg"), (1.0, 1.0, 1.0));
}
