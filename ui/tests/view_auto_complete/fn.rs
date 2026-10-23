use super::*;

fn props<T>(value: T) -> VirtualNode<T> {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: Some(Box::new(value)),
    }
}

fn option(value: &'static str, label: &'static str) -> EuvAutoCompleteOption {
    EuvAutoCompleteOption { value, label }
}

fn city_options() -> Vec<EuvAutoCompleteOption> {
    vec![
        option("lisbon", "Lisbon"),
        option("london", "London"),
        option("madrid", "Madrid"),
    ]
}

fn auto_complete(options: Vec<EuvAutoCompleteOption>, text: &str) -> VirtualNode {
    HookContext::with(HookContext::default(), || {
        euv_auto_complete(props(EuvAutoCompleteProps {
            options,
            value: Signal::create(String::from(text)),
            placeholder: "Search a city",
            label: "City",
            on_change: None,
            on_select: None,
        }))
    })
}

fn count_tag(node: &VirtualNode, tag: &str) -> usize {
    match node {
        VirtualNode::Element {
            tag: own_tag,
            children,
            ..
        } => {
            let own: usize = usize::from(format!("{own_tag:?}") == format!("Element(\"{tag}\")"));
            own + children
                .iter()
                .map(|child: &VirtualNode| count_tag(child, tag))
                .sum::<usize>()
        }
        VirtualNode::Fragment(children) => children.iter().map(|child: &VirtualNode| count_tag(child, tag)).sum(),
        _ => 0,
    }
}

fn carries_classes(entry: &AttributeEntry, names: &[&str]) -> bool {
    if names.len() == 1 {
        let single: AttributeEntry = AttributeEntry::new(
            Cow::Borrowed("class"),
            AttributeValue::Css(Css::new(
                String::from(names[0]),
                String::new(),
                Vec::new(),
                Vec::new(),
            )),
        );
        if entry == &single {
            return true;
        }
    }
    let merged: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("class"),
        AttributeValue::Text(names.join(" ")),
    );
    entry == &merged
}

fn has_classes(node: &VirtualNode, names: &[&str]) -> bool {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let own: bool = attributes
                .iter()
                .filter(|entry: &&AttributeEntry| entry.get_name() == "class")
                .any(|entry: &AttributeEntry| carries_classes(entry, names));
            own || children.iter().any(|child: &VirtualNode| has_classes(child, names))
        }
        VirtualNode::Fragment(children) => children.iter().any(|child: &VirtualNode| has_classes(child, names)),
        _ => false,
    }
}

#[test]
fn a_suggestion_matches_when_its_label_contains_the_query() {
    let matched: bool = auto_complete_matches(&option("lisbon", "Lisbon"), "lis");
    assert!(matched, "the label holds the query");
}

#[test]
fn a_suggestion_matches_when_its_value_contains_the_query() {
    let matched: bool = auto_complete_matches(&option("lisbon", "Lisboa"), "lisbon");
    assert!(
        matched,
        "the value holds the query even when the label does not"
    );
}

#[test]
fn the_match_ignores_the_case_of_the_query() {
    let matched: bool = auto_complete_matches(&option("lisbon", "Lisbon"), "LIS");
    assert!(matched, "the query is lower-cased before it is compared");
}

#[test]
fn an_empty_query_matches_every_suggestion() {
    let matched: bool = auto_complete_matches(&option("lisbon", "Lisbon"), "");
    assert!(matched, "an empty query does not rule anything out");
}

#[test]
fn the_exact_match_is_the_active_suggestion() {
    let filtered: Vec<EuvAutoCompleteOption> = city_options()
        .iter()
        .filter(|option: &EuvAutoCompleteOption| auto_complete_matches(option, "lond"))
        .copied()
        .collect();
    let active: Option<&EuvAutoCompleteOption> = auto_complete_active(&filtered, "London");
    assert_eq!(
        active.map(|option: &EuvAutoCompleteOption| option.value),
        Some("london"),
        "the suggestion whose label equals the text carries the marker"
    );
}

#[test]
fn the_first_suggestion_is_active_when_nothing_matches_exactly() {
    let filtered: Vec<EuvAutoCompleteOption> = city_options()
        .iter()
        .filter(|option: &EuvAutoCompleteOption| auto_complete_matches(option, "mad"))
        .copied()
        .collect();
    let active: Option<&EuvAutoCompleteOption> = auto_complete_active(&filtered, "made");
    assert_eq!(
        active.map(|option: &EuvAutoCompleteOption| option.value),
        Some("madrid"),
        "a partial query falls back to the first suggestion on show"
    );
}

#[test]
fn an_auto_complete_renders_a_text_field_and_a_kept_mounted_panel() {
    let rendered: VirtualNode = auto_complete(city_options(), "lis");
    assert_eq!(
        count_tag(&rendered, "input"),
        1,
        "the field is a real text input"
    );
    assert!(
        has_classes(
            &rendered,
            &[
                "c_euv_auto_complete_panel",
                "c_euv_auto_complete_panel_closed"
            ]
        ),
        "the panel is rendered even while it is closed"
    );
    assert!(
        has_classes(&rendered, &["c_euv_auto_complete_input"]),
        "the field carries the auto complete input class"
    );
}

#[test]
fn an_auto_complete_without_options_renders_no_suggestion_rows() {
    let rendered: VirtualNode = auto_complete(Vec::new(), "lis");
    assert_eq!(
        count_tag(&rendered, "button"),
        0,
        "there is nothing to suggest, so there is no row to click"
    );
    assert!(
        !has_classes(&rendered, &["c_euv_auto_complete_empty"]),
        "the empty row only shows when there are options to miss"
    );
}
