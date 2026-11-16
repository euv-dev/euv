use super::*;

#[test]
fn chrome_motion_policy_is_class_scoped_and_keeps_correct_descendant_selectors() {
    let policy: Css = c_example_motion_policy();
    assert_eq!(policy.get_name(), EXAMPLE_MOTION_POLICY_CLASS);
    assert_eq!(
        policy.get_pseudo_rules().len(),
        EXAMPLE_MOTION_POLICY_SELECTORS.len()
    );
    for (rule, selector) in policy
        .get_pseudo_rules()
        .iter()
        .zip(EXAMPLE_MOTION_POLICY_SELECTORS)
    {
        assert_eq!(
            rule,
            &PseudoRule::new(selector.to_string(), policy.get_style().clone())
        );
    }
    let is_default_build: bool = Some(scheme_name())
        == THEME_FEATURE_MONOCHROME.strip_prefix(THEME_FEATURE_PREFIX)
        && !cfg!(feature = "theme-monochrome")
        && !cfg!(feature = "theme-custom");
    assert_eq!(
        policy.get_style(),
        &EXAMPLE_MOTION_POLICY_STYLES[usize::from(is_default_build)]
    );
}

#[test]
fn chrome_motion_policy_is_attached_to_both_viewport_shells() {
    let source: &str = include_str!("../../src/component/layout/view/fn.rs");
    assert_eq!(
        source.matches("class: c_example_motion_policy()").count(),
        2
    );
    assert!(!EXAMPLE_MOTION_POLICY_STYLES[1].contains("display"));
    assert!(!EXAMPLE_MOTION_POLICY_STYLES[1].contains("visibility"));
}
