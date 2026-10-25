use super::*;

#[test]
fn internal_anchor_decoder_handles_utf8_and_preserves_plus() {
    assert_eq!(decode_anchor("%E2%9C%93+section"), "✓+section");
    assert_eq!(decode_anchor("already-readable"), "already-readable");
}

#[test]
fn internal_anchor_decoder_preserves_invalid_or_truncated_input() {
    for source in ["%", "%2", "%GG", "%FF", "valid%20then%GG"] {
        assert_eq!(decode_anchor(source), source);
    }
}
