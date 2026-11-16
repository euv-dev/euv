use super::*;

/// Resolves the app-chrome motion policy independently from functional demonstrations.
///
/// # Returns
///
/// - `Css` - A class-scoped policy, empty for explicitly selected theme features.
pub(crate) fn c_example_motion_policy() -> Css {
    let style: &str = EXAMPLE_MOTION_POLICY_STYLE;
    let rules: Vec<PseudoRule> = EXAMPLE_MOTION_POLICY_SELECTORS
        .iter()
        .map(|selector: &&str| PseudoRule::new((*selector).to_string(), style.to_string()))
        .collect();
    let class: Css = Css::new(
        EXAMPLE_MOTION_POLICY_CLASS.to_string(),
        style.to_string(),
        rules,
        Vec::new(),
    );
    class.inject_style();
    class
}
