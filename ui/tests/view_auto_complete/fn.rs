use super::*;

#[test]
fn auto_complete_always_emits_its_same_named_input_factories() {
    let rendered: VirtualNode = auto_complete(city_options(), "Lon");
    assert_eq!(
        attribute_text_for_role(&rendered, "combobox", "class"),
        Some(String::from(
            "c_euv_auto_complete_input c_euv_auto_complete_input_default"
        ))
    );
    assert!(has_classes(&rendered, &["c_euv_auto_complete_input"]));
    assert!(has_classes(
        &rendered,
        &["c_euv_auto_complete_input_default"]
    ));
    if cfg!(any(feature = "theme-custom", feature = "theme-shadcn")) {
        assert!(c_euv_auto_complete_input_default().get_style().is_empty());
    } else {
        assert!(class_property(c_euv_auto_complete_input_default(), "min-height").is_some());
    }
}

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
            id: "",
            name: "",
            form: "",
            autocomplete: "",
            required: false,
            invalid: false,
            disabled: false,
            read_only: false,
            aria_label: "",
            aria_describedby: "",
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
        VirtualNode::Fragment(children) => children
            .iter()
            .map(|child: &VirtualNode| count_tag(child, tag))
            .sum(),
        _ => 0,
    }
}

fn count_role(node: &VirtualNode, role: &str) -> usize {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let own: usize = usize::from(attributes.iter().any(|entry: &AttributeEntry| {
                entry.get_name() == "role" && entry.text_value() == Some(String::from(role))
            }));
            own + children
                .iter()
                .map(|child: &VirtualNode| count_role(child, role))
                .sum::<usize>()
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .map(|child: &VirtualNode| count_role(child, role))
            .sum(),
        _ => 0,
    }
}

fn attribute_text_for_role(node: &VirtualNode, role: &str, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element {
            attributes,
            children,
            ..
        } => {
            let has_role: bool = attributes.iter().any(|entry: &AttributeEntry| {
                entry.get_name() == "role" && entry.text_value() == Some(String::from(role))
            });
            if has_role {
                return attributes
                    .iter()
                    .find(|entry: &&AttributeEntry| entry.get_name() == name)
                    .and_then(|entry: &AttributeEntry| entry.text_value());
            }
            children
                .iter()
                .find_map(|child: &VirtualNode| attribute_text_for_role(child, role, name))
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .find_map(|child: &VirtualNode| attribute_text_for_role(child, role, name)),
        _ => None,
    }
}

fn carries_classes(entry: &AttributeEntry, names: &[&str]) -> bool {
    let classes: Vec<String> = entry.class_names();
    names
        .iter()
        .all(|name: &&str| classes.iter().any(|class: &String| class == name))
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
            own || children
                .iter()
                .any(|child: &VirtualNode| has_classes(child, names))
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .any(|child: &VirtualNode| has_classes(child, names)),
        _ => false,
    }
}

fn attribute_text(node: &VirtualNode, tag: &str, name: &str) -> Option<String> {
    match node {
        VirtualNode::Element {
            tag: own_tag,
            attributes,
            children,
            ..
        } => {
            if format!("{own_tag:?}") == format!("Element(\"{tag}\")")
                && let Some(entry) = attributes
                    .iter()
                    .find(|entry: &&AttributeEntry| entry.get_name() == name)
            {
                return entry.text_value();
            }
            children
                .iter()
                .find_map(|child: &VirtualNode| attribute_text(child, tag, name))
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .find_map(|child: &VirtualNode| attribute_text(child, tag, name)),
        _ => None,
    }
}

fn has_attribute(node: &VirtualNode, tag: &str, name: &str) -> bool {
    match node {
        VirtualNode::Element {
            tag: own_tag,
            attributes,
            children,
            ..
        } => {
            let own: bool = format!("{own_tag:?}") == format!("Element(\"{tag}\")")
                && attributes
                    .iter()
                    .any(|entry: &AttributeEntry| entry.get_name() == name);
            own || children
                .iter()
                .any(|child: &VirtualNode| has_attribute(child, tag, name))
        }
        VirtualNode::Fragment(children) => children
            .iter()
            .any(|child: &VirtualNode| has_attribute(child, tag, name)),
        _ => false,
    }
}

#[test]
fn an_auto_complete_generates_an_id_for_its_label() {
    let rendered: VirtualNode = auto_complete(city_options(), "");
    let input_id: String = attribute_text(&rendered, "input", "id").unwrap_or_default();
    let label_for: String = attribute_text(&rendered, "label", "for").unwrap_or_default();
    assert!(!input_id.is_empty(), "the input receives a generated id");
    assert_eq!(label_for, input_id, "the label references the input id");
    assert_eq!(
        attribute_text(&rendered, "input", "aria-label"),
        Some(String::from("City"))
    );
}

#[test]
fn an_auto_complete_exposes_combobox_listbox_and_option_roles() {
    let rendered: VirtualNode = auto_complete(city_options(), "lis");
    assert_eq!(
        attribute_text(&rendered, "input", "role"),
        Some(String::from("combobox"))
    );
    assert_eq!(
        attribute_text(&rendered, "input", "aria-autocomplete"),
        Some(String::from("list"))
    );
    assert_eq!(
        attribute_text(&rendered, "div", "role"),
        Some(String::from("listbox"))
    );
    assert_eq!(count_role(&rendered, "listbox"), 1);
    assert_eq!(count_role(&rendered, "option"), 1);
    assert_eq!(count_role(&rendered, "status"), 1);
    assert_eq!(count_tag(&rendered, "button"), 0);
    assert_eq!(
        attribute_text(&rendered, "input", "aria-controls"),
        attribute_text(&rendered, "div", "id")
    );
}

#[test]
fn an_auto_complete_does_not_highlight_a_suggestion_until_navigation() {
    let rendered: VirtualNode = auto_complete(city_options(), "lis");
    assert!(!has_attribute(&rendered, "input", "aria-activedescendant"));
    assert_eq!(
        attribute_text_for_role(&rendered, "option", "aria-selected"),
        Some(String::from("false"))
    );
}

#[test]
fn an_auto_complete_marks_an_exact_current_value_as_selected() {
    let rendered: VirtualNode = auto_complete(city_options(), "lisbon");
    assert_eq!(
        attribute_text_for_role(&rendered, "option", "aria-selected"),
        Some(String::from("true"))
    );
}

#[test]
fn an_auto_complete_keeps_a_status_region_for_empty_results() {
    let rendered: VirtualNode = auto_complete(city_options(), "unknown");
    assert_eq!(count_role(&rendered, "status"), 1);
    assert_eq!(
        attribute_text_for_role(&rendered, "status", "aria-live"),
        Some(String::from("polite"))
    );
    assert!(has_classes(&rendered, &["c_euv_auto_complete_empty"]));
}

#[test]
fn an_auto_complete_forwards_form_and_control_state_attributes() {
    let rendered: VirtualNode = HookContext::with(HookContext::default(), || {
        euv_auto_complete(props(EuvAutoCompleteProps {
            options: city_options(),
            value: Signal::create(String::new()),
            placeholder: "Search",
            label: "",
            id: "external-query",
            name: "query",
            form: "search-form",
            autocomplete: "off",
            required: true,
            invalid: true,
            disabled: true,
            read_only: true,
            aria_label: "City query",
            aria_describedby: "query-help",
            on_change: None,
            on_select: None,
        }))
    });
    assert_eq!(
        attribute_text(&rendered, "input", "name"),
        Some(String::from("query"))
    );
    assert_eq!(
        attribute_text(&rendered, "input", "form"),
        Some(String::from("search-form"))
    );
    assert_eq!(
        attribute_text(&rendered, "input", "autocomplete"),
        Some(String::from("off"))
    );
    assert_eq!(
        attribute_text(&rendered, "input", "aria-label"),
        Some(String::from("City query"))
    );
    assert_eq!(
        attribute_text(&rendered, "input", "aria-describedby"),
        Some(String::from("query-help"))
    );
    assert_eq!(
        attribute_text(&rendered, "input", "aria-invalid"),
        Some(String::from("true"))
    );
    assert_eq!(
        attribute_text(&rendered, "input", "aria-disabled"),
        Some(String::from("true"))
    );
    assert_eq!(
        attribute_text(&rendered, "input", "aria-readonly"),
        Some(String::from("true"))
    );
    assert!(has_attribute(&rendered, "input", "required"));
    assert!(has_attribute(&rendered, "input", "disabled"));
    assert!(has_attribute(&rendered, "input", "readonly"));
    assert!(has_classes(
        &rendered,
        &["c_euv_auto_complete_input_invalid"]
    ));
    assert!(has_classes(
        &rendered,
        &["c_euv_auto_complete_input_disabled"]
    ));
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
        .filter(|option: &&EuvAutoCompleteOption| auto_complete_matches(option, "lond"))
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
        .filter(|option: &&EuvAutoCompleteOption| auto_complete_matches(option, "mad"))
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
        count_role(&rendered, "option"),
        0,
        "there is nothing to suggest, so there is no row to click"
    );
    assert_eq!(count_role(&rendered, "status"), 1);
    assert!(
        !has_classes(&rendered, &["c_euv_auto_complete_empty"]),
        "the empty row only shows when there are options to miss"
    );
}
