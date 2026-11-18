use super::*;

#[test]
fn chrome_motion_policy_preserves_animations_for_every_theme() {
    let policy: Css = c_example_motion_policy();
    assert_eq!(policy.get_name(), EXAMPLE_MOTION_POLICY_CLASS);
    assert!(policy.get_style().is_empty());
    assert_eq!(
        policy.get_pseudo_rules().len(),
        EXAMPLE_MOTION_POLICY_RULES.len()
    );
    for (rule, (selector, style)) in policy
        .get_pseudo_rules()
        .iter()
        .zip(EXAMPLE_MOTION_POLICY_RULES)
    {
        assert_eq!(
            rule,
            &PseudoRule::new(selector.to_string(), style.to_string())
        );
        assert!(!style.contains("!important"));
        assert!(!selector.contains(":hover"));
        assert!(!selector.contains('*'));
    }
    let source: &str = include_str!("../../src/style/class/motion/const.rs");
    assert!(!source.contains("animation:none"));
    assert!(!source.contains("transition:none"));
}

#[test]
fn chrome_motion_policy_is_attached_to_both_viewport_shells() {
    let source: &str = include_str!("../../src/component/layout/view/fn.rs");
    assert_eq!(
        source.matches("class: c_example_motion_policy()").count(),
        2
    );
}

#[test]
#[cfg(feature = "theme-custom")]
fn custom_chrome_has_state_motion_and_consumer_radius_without_hover_geometry() {
    for selector in [
        " .c_mobile_nav_drawer",
        " .c_mobile_overlay",
        " .c_vconsole_panel",
    ] {
        assert!(
            EXAMPLE_MOTION_POLICY_RULES
                .iter()
                .any(|(target, style): &(&str, &str)| {
                    *target == selector && style.contains("transition:")
                })
        );
    }
    assert!(
        EXAMPLE_MOTION_POLICY_RULES
            .iter()
            .any(|(target, style): &(&str, &str)| {
                *target == " .c_euv_logo_fab"
                    && style.contains("border-radius:var(--radius-md, 0px)")
            })
    );
}
