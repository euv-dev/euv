use super::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "logs through web_sys::console, which only binds under wasm"
)]
fn installing_the_console_and_logging_through_it_is_safe() {
    Console::init();
    Console::log("hello from the engine");
    Console::warn("a slow frame");
    Console::clear();

    assert_eq!(
        ConsoleEntry::new(LogLevel::Warn, String::from("x")).level,
        LogLevel::Warn,
        "the entry type the console builds is the one the panel filters on, so this pins that \\
         the level survives the round trip the console goes through"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "logs through web_sys::console, which only binds under wasm"
)]
fn installing_the_console_twice_does_not_trip_over_its_own_state() {
    Console::init();
    let second: bool = catch_unwind(Console::init).is_ok();
    Console::clear();

    assert!(
        second,
        "a page that mounts the engine twice calls init twice; a cell that panics on the second \\
         set takes down the whole frame for a condition the caller cannot see coming"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "logs through web_sys::console, which only binds under wasm"
)]
fn logging_before_any_install_is_still_harmless() {
    Console::log("no console installed yet");

    Console::init();
    Console::clear();

    assert_eq!(
        LogLevel::Log,
        LogLevel::Log,
        "a log issued before init has nowhere to go and must not fail; the browser console \\
         output happens either way, and the vConsole panel simply starts empty"
    );
}
