use super::*;

#[test]
fn valid_w3c_payload_is_returned_without_inventing_defaults() {
    let actual: Value = response_value(json!({"value": {"ready": true}})).unwrap();
    assert_eq!(actual, json!({"ready": true}));
}

#[test]
fn malformed_envelope_and_driver_errors_remain_failures() {
    assert!(response_value(json!({"unexpected": true})).is_err());
    let failure: String = response_value(
        json!({"value": {"error": "no such element", "message": "selector did not match"}}),
    )
    .unwrap_err();
    assert!(failure.contains("no such element"));
    assert!(failure.contains("selector did not match"));
}

#[test]
fn explicit_runner_options_are_not_silently_ignored() {
    let arguments: Vec<String> = vec![
        String::from("--skip-build"),
        String::from("--probe-failure"),
        String::from("--driver"),
        String::from("driver"),
    ];
    let settings: Settings = Settings::parse(arguments).unwrap();
    assert!(settings.get_skip_build());
    assert!(settings.get_probe_failure());
    assert_eq!(settings.get_driver(), &PathBuf::from("driver"));
}

#[test]
fn missing_values_and_unknown_options_are_rejected() {
    assert!(Settings::parse(vec![String::from("--driver")]).is_err());
    assert!(Settings::parse(vec![String::from("--unknown"), String::from("value")]).is_err());
}

#[test]
fn negative_control_is_not_part_of_the_positive_case_set() {
    let names: Vec<&str> = vec![
        Scenario::Counter,
        Scenario::Input,
        Scenario::Keyboard,
        Scenario::Checkbox,
        Scenario::Modal,
        Scenario::Scroll,
        Scenario::Responsive,
        Scenario::CaretGap,
    ]
    .into_iter()
    .map(Scenario::name)
    .collect();
    assert_eq!(names.len(), 8);
    assert!(!names.contains(&Scenario::Negative.name()));
}
