use super::*;

#[test]
fn interpolate_replaces_a_known_placeholder() {
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("name", "euv");
    let rendered: String = interpolate("hello {name}", &vars);
    assert_eq!(rendered, "hello euv");
}

#[test]
fn interpolate_keeps_an_unknown_placeholder_verbatim() {
    let mut vars: HashMap<&'static str, &'static str> = HashMap::new();
    vars.insert("name", "euv");
    let rendered: String = interpolate("hello {name}, {other}", &vars);
    assert_eq!(rendered, "hello euv, {other}");
}

#[test]
fn interpolate_round_trips_a_template_without_placeholders() {
    let vars: HashMap<&'static str, &'static str> = HashMap::new();
    let rendered: String = interpolate("plain text", &vars);
    assert_eq!(rendered, "plain text");
}
