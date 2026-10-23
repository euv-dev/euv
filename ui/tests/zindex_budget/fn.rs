use super::*;

const BUDGET: &[(&str, &[&str], &str)] = &[
    (
        "page content",
        &["c_page_content", "c_home_content", "c_euv_watermark_layer"],
        "1",
    ),
    ("loading overlay", &["c_loading_overlay"], "5"),
    ("affix", &["c_euv_affix"], "10"),
    ("app chrome", &["c_mobile_header", "c_euv_navbar"], "100"),
    ("nav dropdown", &["c_euv_dropdown_menu"], "101"),
    (
        "tooltip",
        &[
            "c_euv_tooltip_bubble",
            "c_euv_tooltip_bubble_top",
            "c_euv_tooltip_bubble_bottom",
            "c_euv_tooltip_bubble_left",
            "c_euv_tooltip_bubble_right",
        ],
        "140",
    ),
    (
        "floating panels",
        &[
            "c_euv_select_panel",
            "c_euv_select_panel_open",
            "c_euv_select_panel_closed",
            "c_euv_auto_complete_panel",
            "c_euv_auto_complete_panel_open",
            "c_euv_auto_complete_panel_closed",
            "c_euv_cascader_panel",
            "c_euv_cascader_panel_open",
            "c_euv_cascader_panel_closed",
            "c_euv_date_picker_panel",
            "c_euv_date_picker_panel_open",
            "c_euv_date_picker_panel_closed",
            "c_euv_mentions_panel",
            "c_euv_mentions_panel_open",
            "c_euv_mentions_panel_closed",
            "c_euv_popconfirm_body",
            "c_euv_popconfirm_body_open",
            "c_euv_popconfirm_body_closed",
            "c_euv_popover_body",
            "c_euv_popover_body_open",
            "c_euv_popover_body_closed",
            "c_euv_time_picker_panel",
            "c_euv_time_picker_panel_open",
            "c_euv_time_picker_panel_closed",
            "c_euv_tree_select_panel",
            "c_euv_tree_select_panel_open",
            "c_euv_tree_select_panel_closed",
        ],
        "150",
    ),
    (
        "overlay",
        &["c_euv_drawer_overlay", "c_mobile_overlay"],
        "200",
    ),
    ("drawer", &["c_euv_drawer", "c_mobile_nav_drawer"], "201"),
    (
        "back top",
        &["c_euv_back_top_hidden", "c_euv_back_top_visible"],
        "900",
    ),
    ("modal", &["c_modal_overlay"], "1000"),
    ("message", &["c_euv_message_stack"], "1100"),
    ("notification", &["c_euv_notification_stack"], "1200"),
    ("vconsole fab", &["c_vconsole_fab"], "9999"),
    ("vconsole overlay", &["c_vconsole_overlay"], "10000"),
    ("vconsole panel", &["c_vconsole_panel"], "10001"),
    (
        "fullscreen canvas",
        &[
            "c_canvas_container_fullscreen",
            "c_game_container_fullscreen",
        ],
        "10002",
    ),
];

fn class_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("style")
        .join("class")
}

fn class_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    collect_class_files(&class_directory(), &mut files);
    files.sort();
    files
}

fn collect_class_files(directory: &Path, files: &mut Vec<PathBuf>) {
    let entries: Vec<fs::DirEntry> = match fs::read_dir(directory) {
        Ok(entries) => entries.flatten().collect(),
        Err(_) => return,
    };
    for entry in entries {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            collect_class_files(&path, files);
            continue;
        }
        if path
            .extension()
            .is_some_and(|extension: &OsStr| extension == "rs")
        {
            files.push(path);
        }
    }
}

fn declared_z_indexes() -> BTreeMap<String, String> {
    let mut found: BTreeMap<String, String> = BTreeMap::new();

    for file in class_files() {
        let text: String = fs::read_to_string(&file).expect("a class file of the ui crate");
        let lines: Vec<&str> = text.lines().collect();
        let mut index: usize = 0;
        while index < lines.len() {
            let line: &str = lines[index];
            let trimmed: &str = line.trim();
            let indent: usize = line.len() - line.trim_start().len();
            let is_rule: bool = trimmed.starts_with("pub ") || trimmed.starts_with("pub(crate) ");
            if !is_rule || !trimmed.ends_with('{') {
                index += 1;
                continue;
            }

            let name: String = trimmed
                .trim_start_matches("pub(crate) ")
                .trim_start_matches("pub ")
                .trim_end_matches('{')
                .trim()
                .split('(')
                .next()
                .unwrap_or_default()
                .trim()
                .to_string();

            let mut cursor: usize = index + 1;
            let mut value: Option<String> = None;
            while cursor < lines.len() {
                let current: &str = lines[cursor];
                let current_indent: usize = current.len() - current.trim_start().len();
                if current.trim() == "}" && current_indent == indent {
                    break;
                }
                if let Some(rest) = current.trim().strip_prefix("z-index:") {
                    value = Some(
                        rest.trim()
                            .trim_end_matches(';')
                            .trim()
                            .trim_matches('"')
                            .to_string(),
                    );
                }
                cursor += 1;
            }

            if let Some(value) = value {
                found.insert(name, value);
            }
            index = cursor + 1;
        }
    }
    found
}

#[test]
fn every_z_index_rule_is_registered_in_the_budget() {
    let declared: BTreeMap<String, String> = declared_z_indexes();
    let registered: BTreeSet<&str> = BUDGET
        .iter()
        .flat_map(|entry: &(&str, &[&str], &str)| entry.1.iter().copied())
        .collect();

    let unregistered: Vec<&String> = declared
        .keys()
        .filter(|name: &&String| !registered.contains(name.as_str()))
        .collect();

    assert!(
        unregistered.is_empty(),
        "these rules declare a z-index but are not in the budget: {unregistered:?} — \
         give the layer a slot of its own and register it, or the rule falls back \
         to the document order"
    );
}

#[test]
fn every_layer_keeps_its_own_slot() {
    let declared: BTreeMap<String, String> = declared_z_indexes();
    let mut offenders: Vec<String> = Vec::new();

    for (layer, rules, slot) in BUDGET {
        for rule in *rules {
            match declared.get(*rule) {
                Some(value) if value == slot => {}
                Some(value) => offenders.push(format!(
                    "{rule} declares z-index {value}, the {layer} slot is {slot}"
                )),
                None => offenders.push(format!("{rule} is in the budget but declares no z-index")),
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "the z-index budget and the classes disagree:\n{}",
        offenders.join("\n")
    );
}

#[test]
fn two_layers_never_share_a_slot() {
    let mut seen: BTreeMap<String, &str> = BTreeMap::new();
    let mut offenders: Vec<String> = Vec::new();

    for (layer, _, slot) in BUDGET {
        match seen.get(*slot) {
            Some(other) if *other != *layer => offenders.push(format!(
                "the {other} layer and the {layer} layer both use z-index {slot}"
            )),
            _ => {
                seen.insert((*slot).to_string(), layer);
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "two layers share a z-index, so their order is decided by the document \
         instead of by the design:\n{}",
        offenders.join("\n")
    );
}

#[test]
fn the_layers_are_stacked_in_the_order_they_are_listed() {
    let mut previous: u32 = 0;
    let mut offenders: Vec<String> = Vec::new();

    for (layer, _, slot) in BUDGET {
        let value: u32 = slot.parse().unwrap_or(0);
        if value <= previous {
            offenders.push(format!(
                "the {layer} layer uses z-index {slot}, below the previous layer"
            ));
        }
        previous = value;
    }

    assert!(
        offenders.is_empty(),
        "the budget is not sorted from the bottom of the stacking order up:\n{}",
        offenders.join("\n")
    );
}
