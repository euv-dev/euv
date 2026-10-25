use super::*;

/// Mounts the actual euv component fixture used by the browser suite.
#[wasm_bindgen]
pub fn mount_fixture() {
    inject_app_global_css();
    App::mount(APP_SELECTOR, || {
        html! {
        fixture_page {}
        }
    });
}
