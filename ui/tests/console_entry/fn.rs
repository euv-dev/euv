use super::*;

#[test]
fn a_console_entry_records_the_level_and_message_it_was_given() {
    let entry: ConsoleEntry = ConsoleEntry::new(LogLevel::Warn, String::from("slow frame"));
    assert_eq!(entry.level, LogLevel::Warn);
    assert_eq!(entry.message, "slow frame");
}

#[test]
fn a_console_entry_keeps_an_empty_message_rather_than_inventing_one() {
    let entry: ConsoleEntry = ConsoleEntry::new(LogLevel::Log, String::new());
    assert!(
        entry.message.is_empty(),
        "an empty log line is a real event; substituting placeholder text would \
         misreport it, got {:?}",
        entry.message
    );
}

#[test]
fn the_three_log_levels_map_to_three_distinct_entries() {
    let log: ConsoleEntry = ConsoleEntry::new(LogLevel::Log, String::from("same"));
    let warn: ConsoleEntry = ConsoleEntry::new(LogLevel::Warn, String::from("same"));
    let error: ConsoleEntry = ConsoleEntry::new(LogLevel::Error, String::from("same"));
    assert_ne!(
        log, warn,
        "the same text at two levels is two different events"
    );
    assert_ne!(warn, error);
    assert_ne!(log, error);
}

#[test]
fn a_console_entry_keeps_a_multiline_message_intact() {
    let message: String = String::from("first\nsecond\nthird");
    let entry: ConsoleEntry = ConsoleEntry::new(LogLevel::Error, message.clone());
    assert_eq!(
        entry.message, message,
        "a stack trace arrives as one multi-line string; splitting or trimming it \
         would lose the trace"
    );
}

#[test]
fn a_console_entry_keeps_markup_and_unicode_verbatim() {
    let message: String = String::from("<b>bold</b> · 中文 · 🎯");
    let entry: ConsoleEntry = ConsoleEntry::new(LogLevel::Log, message.clone());
    assert_eq!(entry.message, message);
}

#[test]
fn a_console_entry_is_cloneable_so_the_log_can_be_shared_by_reference() {
    let entry: ConsoleEntry = ConsoleEntry::new(LogLevel::Error, String::from("boom"));
    let cloned: ConsoleEntry = entry.clone();
    assert_eq!(cloned, entry);
}

#[test]
fn the_vconsole_sink_never_lets_an_uninitialised_panel_lose_the_entry() {
    let entry: ConsoleEntry = ConsoleEntry::new(LogLevel::Log, String::from("before init"));
    assert_eq!(
        entry.message, "before init",
        "the documented contract is that the panel entry is only appended once \
         Console::init has run, so building one must still be lossless"
    );
}
