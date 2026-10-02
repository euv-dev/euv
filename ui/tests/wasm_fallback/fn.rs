#[cfg(target_arch = "wasm32")]
use super::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn local_storage_get_returns_none_when_there_is_no_window() {
    assert_eq!(
        UseEuvBrowser::local_storage_get("euv.never.set"),
        None,
        "a host without localStorage must read as absent, not panic"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn local_storage_set_is_a_no_op_when_there_is_no_window() {
    UseEuvBrowser::local_storage_set("euv.probe", "value");
    assert_eq!(
        UseEuvBrowser::local_storage_get("euv.probe"),
        None,
        "the write must not have created storage that the read can then see"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn local_storage_get_of_an_absent_key_and_of_any_key_agree_under_node() {
    assert_eq!(UseEuvBrowser::local_storage_get("a"), None);
    assert_eq!(UseEuvBrowser::local_storage_get(""), None);
    assert_eq!(UseEuvBrowser::local_storage_get(String::from("b")), None);
}
