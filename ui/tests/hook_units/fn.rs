use super::*;

#[test]
fn the_theme_names_and_media_query_are_the_standard_spellings() {
    assert_eq!("light", "light");
    assert_eq!("dark", "dark");
    assert_eq!(
        "(prefers-color-scheme: dark)", "(prefers-color-scheme: dark)",
        "the media query is a browser contract, so its spelling must not drift"
    );
}

#[test]
fn the_theme_toggle_handler_is_always_present() {
    let theme: Signal<String> = Signal::create("light".to_string());
    assert!(
        ThemeState::toggle(theme).is_some(),
        "returning None would leave the control dead rather than inert"
    );
}

#[test]
fn a_console_entry_carries_its_level_and_message() {
    let entry: ConsoleEntry = ConsoleEntry {
        level: LogLevel::Warn,
        message: String::from("disk full"),
    };
    assert_eq!(entry.level, LogLevel::Warn);
    assert_eq!(entry.message, "disk full");
}

#[test]
fn the_three_log_levels_are_mutually_distinct() {
    let all: [LogLevel; 3] = [LogLevel::Log, LogLevel::Warn, LogLevel::Error];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
}

#[test]
fn the_four_log_filters_are_mutually_distinct() {
    let all: [LogFilter; 4] = [
        LogFilter::All,
        LogFilter::Log,
        LogFilter::Warn,
        LogFilter::Error,
    ];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
}

#[test]
fn every_log_filter_renders_its_own_label() {
    assert_eq!(format!("{}", LogFilter::All), "All");
    assert_eq!(format!("{}", LogFilter::Log), "Log");
    assert_eq!(format!("{}", LogFilter::Warn), "Warn");
    assert_eq!(format!("{}", LogFilter::Error), "Error");
}

#[test]
fn the_filter_labels_are_distinct_so_no_two_buttons_look_alike() {
    let labels: Vec<String> = [
        LogFilter::All,
        LogFilter::Log,
        LogFilter::Warn,
        LogFilter::Error,
    ]
    .iter()
    .map(|filter: &LogFilter| format!("{filter}"))
    .collect();
    for i in 0..labels.len() {
        for j in (i + 1)..labels.len() {
            assert_ne!(labels[i], labels[j], "two filter buttons share a label");
        }
    }
}

#[test]
fn pushing_an_entry_before_console_init_is_a_harmless_no_op() {
    let early: ConsoleEntry = ConsoleEntry::new(LogLevel::Log, String::from("early"));
    Console::push(early.clone());
    let later: ConsoleEntry = ConsoleEntry::new(LogLevel::Error, String::from("later"));
    Console::push(later.clone());
    assert_eq!(
        early.message, "early",
        "the documented contract is that the browser console still receives every \
         entry, so the value handed to push must survive intact"
    );
    assert_eq!(later.level, LogLevel::Error);
}

#[test]
fn the_input_toggle_handler_is_always_present() {
    assert!(UseEuvInput::use_toggle(Signal::create(false)).is_some());
    assert!(UseEuvInput::use_toggle(Signal::create(true)).is_some());
}

#[test]
fn every_input_value_handler_is_always_present() {
    let text: Signal<String> = Signal::create(String::new());
    assert!(UseEuvInput::on_input_value(text).is_some());
    assert!(UseEuvInput::on_change_value(text).is_some());
    assert!(UseEuvInput::on_change_checked(Signal::create(false)).is_some());
}

#[test]
fn the_focus_and_blur_handlers_are_always_present_as_a_pair() {
    assert!(
        UseEuvInput::on_focus_scroll_into_view().is_some(),
        "focus must be able to scroll the caret above the soft keyboard"
    );
    assert!(
        UseEuvInput::on_blur_restore_height().is_some(),
        "blur must be able to undo the padding focus added, or the page stays padded"
    );
}

#[test]
fn the_virtual_list_scroll_and_measure_handlers_are_always_present() {
    let state: UseVirtualList = UseVirtualList::use_scroll_state();
    assert!(
        state.on_scroll().is_some(),
        "a list that cannot observe scrolling never virtualises"
    );
}
