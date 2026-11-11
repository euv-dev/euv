use super::*;

const PUBLISHED_ROLES: [(&str, &str, &str, &str); 26] = [
    (
        "role-background-light",
        "#ffffff",
        "role-background-dark",
        "#0a0a0a",
    ),
    (
        "role-foreground-light",
        "#000000",
        "role-foreground-dark",
        "#fafafa",
    ),
    ("role-card-light", "#ffffff", "role-card-dark", "#171717"),
    (
        "role-card-foreground-light",
        "#000000",
        "role-card-foreground-dark",
        "#fafafa",
    ),
    (
        "role-popover-light",
        "#ffffff",
        "role-popover-dark",
        "#171717",
    ),
    (
        "role-popover-foreground-light",
        "#000000",
        "role-popover-foreground-dark",
        "#fafafa",
    ),
    (
        "role-primary-light",
        "#000000",
        "role-primary-dark",
        "#e5e5e5",
    ),
    (
        "role-primary-foreground-light",
        "#fafafa",
        "role-primary-foreground-dark",
        "#171717",
    ),
    (
        "role-secondary-light",
        "#f5f5f5",
        "role-secondary-dark",
        "#262626",
    ),
    (
        "role-secondary-foreground-light",
        "#171717",
        "role-secondary-foreground-dark",
        "#fafafa",
    ),
    ("role-muted-light", "#f5f5f5", "role-muted-dark", "#262626"),
    (
        "role-muted-foreground-light",
        "#737373",
        "role-muted-foreground-dark",
        "#a1a1a1",
    ),
    (
        "role-accent-light",
        "#f5f5f5",
        "role-accent-dark",
        "#404040",
    ),
    (
        "role-accent-foreground-light",
        "#171717",
        "role-accent-foreground-dark",
        "#fafafa",
    ),
    (
        "role-destructive-light",
        "#e7000b",
        "role-destructive-dark",
        "#ff6467",
    ),
    (
        "role-destructive-foreground-light",
        "#fcf3f3",
        "role-destructive-foreground-dark",
        "#df2225",
    ),
    (
        "role-border-light",
        "#e5e5e5",
        "role-border-dark",
        "rgba(255, 255, 255, 0.10)",
    ),
    (
        "role-input-light",
        "#e5e5e5",
        "role-input-dark",
        "rgba(255, 255, 255, 0.15)",
    ),
    ("role-ring-light", "#a1a1a1", "role-ring-dark", "#737373"),
    (
        "role-success-light",
        "#16a34a",
        "role-success-dark",
        "#22c55e",
    ),
    (
        "role-warning-light",
        "#d97706",
        "role-warning-dark",
        "#f59e0b",
    ),
    (
        "role-error-container-light",
        "#fee2e2",
        "role-error-container-dark",
        "#450a0a",
    ),
    (
        "role-success-container-light",
        "#dcfce7",
        "role-success-container-dark",
        "#052e16",
    ),
    (
        "role-warning-container-light",
        "#fef3c7",
        "role-warning-container-dark",
        "#451a03",
    ),
    (
        "role-success-foreground-light",
        "#ffffff",
        "role-success-foreground-dark",
        "#052e16",
    ),
    (
        "role-warning-foreground-light",
        "#ffffff",
        "role-warning-foreground-dark",
        "#451a03",
    ),
];

const PUBLISHED_RADIUS: [(&str, &str); 4] = [
    ("radius-sm", "6px"),
    ("radius-md", "8px"),
    ("radius-lg", "10px"),
    ("radius-xl", "14px"),
];

const CONDITIONAL_REFERENCE_SLOTS: &[&str] = &["pagination-previous", "pagination-next"];

const REFERENCE_SLOTS: &[&str] = &[
    "badge",
    "checkbox",
    "checkbox-group",
    "checkbox-indicator",
    "dialog-close",
    "empty-icon",
    "label",
    "input",
    "item",
    "item-group",
    "item-separator",
    "item-media",
    "item-content",
    "item-title",
    "item-description",
    "item-actions",
    "menubar",
    "menubar-checkbox-item",
    "menubar-content",
    "menubar-group",
    "menubar-item",
    "menubar-label",
    "menubar-menu",
    "menubar-radio-group",
    "menubar-radio-item",
    "menubar-separator",
    "menubar-shortcut",
    "menubar-sub",
    "menubar-sub-content",
    "menubar-sub-trigger",
    "menubar-trigger",
    "progress-indicator",
    "progress-track",
    "separator",
    "skeleton",
    "pagination",
    "pagination-content",
    "pagination-item",
    "pagination-link",
    "pagination-previous",
    "pagination-next",
    "pagination-ellipsis",
    "slider",
    "slider-range",
    "slider-thumb",
    "slider-track",
    "spinner",
    "switch",
    "switch-thumb",
    "table-caption",
    "table-cell",
    "table-container",
    "table-footer",
    "table-head",
    "table-header",
    "table-row",
    "textarea",
    "toggle-group",
];

fn scheme_file() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("style")
        .join("var")
        .join("scheme")
        .join("shadcn")
        .join("fn.rs")
}

fn scheme_values() -> BTreeMap<String, String> {
    let path: PathBuf = scheme_file();
    let text: String = fs::read_to_string(&path).expect("the shadcn scheme file");
    let mut values: BTreeMap<String, String> = BTreeMap::new();
    for line in text.lines() {
        let trimmed: &str = line.trim();
        if !trimmed.ends_with(';') {
            continue;
        }
        let Some(separator) = trimmed.find(':') else {
            continue;
        };
        let name: &str = trimmed[..separator].trim();
        if name.is_empty() || name.contains(' ') || name.contains('(') {
            continue;
        }
        let value: &str = trimmed[separator + 1..].trim().trim_end_matches(';').trim();
        let value: String = value.trim_matches('"').to_string();
        values.insert(name.to_string(), value);
    }
    values
}

#[test]
fn the_scheme_publishes_the_reference_colour_roles() {
    let values: BTreeMap<String, String> = scheme_values();
    let mut drifted: Vec<String> = Vec::new();
    for (light, expected_light, dark, expected_dark) in PUBLISHED_ROLES {
        let actual_light: Option<&String> = values.get(light);
        if actual_light.map(String::as_str) != Some(expected_light) {
            drifted.push(format!(
                "{light} is {:?}, the reference publishes {expected_light}",
                actual_light.unwrap_or(&String::new())
            ));
        }
        let actual_dark: Option<&String> = values.get(dark);
        if actual_dark.map(String::as_str) != Some(expected_dark) {
            drifted.push(format!(
                "{dark} is {:?}, the reference publishes {expected_dark}",
                actual_dark.unwrap_or(&String::new())
            ));
        }
    }
    assert!(
        drifted.is_empty(),
        "the shadcn scheme drifted from the reference: {drifted:?} — a scheme that is not the \
         reference's colour language is a different design system wearing its name"
    );
}

fn emitted_slots() -> BTreeSet<String> {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("component");
    let mut slots: BTreeSet<String> = BTreeSet::new();
    let mut stack: Vec<PathBuf> = vec![root];
    while let Some(directory) = stack.pop() {
        let entries: Vec<PathBuf> = fs::read_dir(&directory)
            .expect("the component directory")
            .filter_map(|entry: Result<fs::DirEntry, Error>| entry.ok())
            .map(|entry: fs::DirEntry| entry.path())
            .collect();
        for path in entries {
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(OsStr::to_str) != Some("rs") {
                continue;
            }
            let text: String = fs::read_to_string(&path).expect("a component source file");
            let mut rest: &str = &text;
            while let Some(found) = rest.find("\"data-slot\"") {
                rest = &rest[found + "\"data-slot\"".len()..];
                let Some(colon) = rest.find(':') else {
                    break;
                };
                rest = &rest[colon + 1..];
                let Some(start) = rest.find('"').map(|index: usize| index + 1) else {
                    break;
                };
                let Some(end) = rest[start..].find('"') else {
                    break;
                };
                slots.insert(rest[start..start + end].to_string());
                rest = &rest[start + end..];
            }
        }
    }
    for slot in CONDITIONAL_REFERENCE_SLOTS {
        slots.insert((*slot).to_string());
    }
    slots
}

#[test]
fn every_reference_slot_is_emitted_by_a_component() {
    let slots: BTreeSet<String> = emitted_slots();
    let missing: Vec<&str> = REFERENCE_SLOTS
        .iter()
        .copied()
        .filter(|slot: &&str| !slots.contains(*slot))
        .collect();
    assert!(
        missing.is_empty(),
        "the reference publishes slots euv-ui does not emit: {missing:?} — a part without its \
         marker does not match the reference's markup, so reference-styled content cannot \
         attach to it"
    );
}

#[test]
fn the_radius_ladder_matches_the_reference_scale() {
    let values: BTreeMap<String, String> = scheme_values();
    let mut drifted: Vec<String> = Vec::new();
    for (name, expected) in PUBLISHED_RADIUS {
        let actual: Option<&String> = values.get(name);
        if actual.map(String::as_str) != Some(expected) {
            drifted.push(format!(
                "{name} is {:?}, the reference publishes {expected}",
                actual.unwrap_or(&String::new())
            ));
        }
    }
    assert!(
        drifted.is_empty(),
        "the shadcn radius ladder drifted from the reference: {drifted:?} — the ladder multiplies \
         the base radius by a fixed factor per step, so a changed corner breaks every step above it"
    );
}
