use super::*;

fn compact_style(css: &Css) -> String {
    css.get_style().replace(' ', "")
}

fn motion_rules(css: &Css) -> String {
    format!("{:?}", css.get_media_rules()).replace(' ', "")
}

fn elements(node: &VirtualNode) -> Vec<&VirtualNode> {
    let mut result: Vec<&VirtualNode> = vec![node];
    for child in node.get_children() {
        result.extend(elements(child));
    }
    result
}

fn has_class(node: &VirtualNode, name: &str) -> bool {
    match node {
        VirtualNode::Element { attributes, .. } => {
            attributes.iter().any(|entry: &AttributeEntry| {
                entry.get_name() == "class"
                    && entry
                        .class_names()
                        .iter()
                        .any(|value: &String| value == name)
            })
        }
        _ => false,
    }
}

#[test]
fn functional_motion_has_local_timings_and_reduced_motion_overrides() {
    for (css, name, timing) in [
        (c_animation_spin(), "example-animation-spin", "1500ms"),
        (c_animation_pulse(), "example-animation-pulse", "1500ms"),
        (
            c_animation_progress_running(),
            "example-animation-progress",
            "1600ms",
        ),
    ] {
        let rules: String = motion_rules(css);
        assert!(rules.contains("prefers-reduced-motion:no-preference"));
        assert!(rules.contains(name));
        assert!(rules.contains(timing));
        assert!(rules.contains("prefers-reduced-motion:reduce"));
        assert!(rules.contains("animation:none!important"));
        assert!(!rules.contains("var(--duration"));
        assert!(ANIMATION_DEMO_KEYFRAMES.contains(&format!("@keyframes {name}")));
    }
    for css in [c_animation_fade(), c_animation_scale()] {
        let rules: String = motion_rules(css);
        assert!(rules.contains("500ms"));
        assert!(rules.contains("prefers-reduced-motion:reduce"));
        assert!(rules.contains("transition:none!important"));
        assert!(compact_style(css).contains("transition:none!important"));
    }
}

#[test]
fn progress_has_a_visible_track_full_size_fill_and_accessible_reduced_end_state() {
    let track: String = compact_style(c_animation_progress_track());
    let fill: String = compact_style(c_animation_progress_fill());
    assert!(track.contains("height:var(--space-lg)"));
    assert!(track.contains("width:100%"));
    assert!(track.contains("overflow:hidden"));
    assert!(track.contains("border-radius:var(--radius-sm)"));
    assert!(!track.contains("radius-full"));
    assert!(fill.contains("height:100%"));
    assert!(fill.contains("width:100%"));
    assert!(fill.contains("background:currentColor"));
    assert!(fill.contains("transform:scaleX(0)"));
    assert!(fill.contains("animation:none!important"));
    assert!(compact_style(c_animation_progress_running()).contains("transform:scaleX(1)"));
    assert!(motion_rules(c_animation_progress_running()).contains("transform:scaleX(1)"));
}

#[test]
fn stopped_symbols_and_reversible_fade_and_scale_have_explicit_states() {
    let symbol: String = compact_style(c_animation_symbol());
    assert!(symbol.contains("animation:none!important"));
    assert!(symbol.contains("font-size:var(--font-5xl)"));
    assert!(
        compact_style(c_animation_stage())
            .contains("min-height:calc(var(--space-3xl)*2+var(--space-lg))")
    );
    let hidden: String = compact_style(c_animation_fade_hidden());
    assert!(hidden.contains("opacity:0"));
    assert!(hidden.contains("visibility:hidden"));
    assert!(hidden.contains("pointer-events:none"));
    let visible: String = compact_style(c_animation_fade());
    assert!(visible.contains("opacity:1"));
    assert!(visible.contains("visibility:visible"));
    let scale: String = compact_style(c_animation_scale());
    assert!(scale.contains("height:calc(var(--space-3xl)*2+var(--space-lg))"));
    assert!(scale.contains("transform:scale(1)"));
    assert!(compact_style(c_animation_scale_shrink()).contains("transform:scale(0.65)"));
}

#[test]
fn every_progress_start_changes_generation_and_reset_empties_the_fill() {
    let state: UseProgress = UseProgress::new(Signal::create(false), Signal::create(0));
    progress_start(state);
    assert!(state.get_running().get());
    assert_eq!(state.get_generation().get(), 1);
    progress_start(state);
    assert_eq!(state.get_generation().get(), 2);
    progress_reset(state);
    assert!(!state.get_running().get());
    progress_start(state);
    assert!(state.get_running().get());
    assert_eq!(state.get_generation().get(), 3);
}

#[test]
fn page_keeps_the_hidden_fade_mounted_and_uses_dedicated_demo_slots() {
    let node: VirtualNode = page_animation(VirtualNode::default());
    let all: Vec<&VirtualNode> = elements(&node);
    for name in [
        "c_animation_fade",
        "c_animation_symbol",
        "c_animation_progress_track",
        "c_animation_progress_fill",
        "c_animation_scale",
    ] {
        assert!(
            all.iter()
                .any(|entry: &&VirtualNode| has_class(entry, name)),
            "{name}"
        );
    }
    let fade: &VirtualNode = all
        .iter()
        .copied()
        .find(|entry: &&VirtualNode| has_class(entry, "c_animation_fade"))
        .expect("mounted fade");
    if let VirtualNode::Element { attributes, .. } = fade {
        assert!(
            attributes
                .iter()
                .any(|entry: &AttributeEntry| entry.get_name() == "aria-hidden"
                    && entry.text_value().as_deref() == Some("true")),
            "{attributes:?}"
        );
    }
    let source: &str = include_str!("../../src/page/animation/view/fn.rs");
    assert!(!source.contains("if { box_visible } {\n                    div"));
    assert!(source.contains("progress.get_generation().get().is_multiple_of(2)"));
    assert!(source.contains("c_animation_progress_replay()"));
    assert!(!source.contains("c_anim_"));
    assert!(!source.contains("c_progress_container()"));
}
