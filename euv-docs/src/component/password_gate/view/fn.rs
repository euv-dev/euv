use super::*;

/// localStorage key prefix used to record that a `route` has been
/// unlocked. The value itself is just a constant `"1"` sentinel — the
/// password digest lives only in [`DocsPage::password_hash`].
///
/// Route is alphanumeric-only into the key so that slashes / locale
/// prefixes don't collide and so the key is plain ASCII (localStorage
/// keys must not contain newlines or other control chars).
const UNLOCK_KEY_PREFIX: &str = "euv-docs:unlocked:";

/// Calls `crypto.subtle.digest("SHA-256", ...)` on the supplied UTF-8
/// string via a tiny inline JS shim and returns the lower-case hex
/// digest (64 chars). Returns `None` on environments without
/// `crypto.subtle` (rare outside of `http://` test fixtures or
/// non-secure-context loads; the form will then reject all attempts
/// so the content stays protected).
///
/// Using `eval` keeps the binding typed-but-loose — we don't
/// need the typed `web_sys::Crypto` / `web_sys::SubtleCrypto` surfaces,
/// which aren't enabled in euv's web-sys feature set. The result is a
/// `Promise<ArrayBuffer>` that we await via `wasm_bindgen_futures`.
async fn async_sha256_hex(input: &str) -> Option<String> {
    // Build the JS call. We escape backslashes, quotes, and newlines
    // in the input so a hostile password cannot break out of the
    // string literal and execute arbitrary code. SHA-256 is defined
    // over byte sequences, so we encode UTF-8 and let the JS side
    // turn it into an ArrayBuffer via `TextEncoder`.
    let escaped: String = input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r");
    let script: String = format!(
        r#"(async () => {{
            const data = new TextEncoder().encode("{escaped}");
            const buf = await crypto.subtle.digest("SHA-256", data);
            const bytes = new Uint8Array(buf);
            let out = "";
            for (let i = 0; i < bytes.length; i++) {{
                out += bytes[i].toString(16).padStart(2, "0");
            }}
            return out;
        }})()"#
    );
    let value: JsValue = eval(&script).ok()?;
    let promise: Promise = value.unchecked_into();
    let result: Result<JsValue, JsValue> =
        JsFuture::from(promise).await.into();
    let digest: String = result.ok()?.as_string()?;
    Some(digest)
}

/// Sanitises a route string for use as a localStorage key. Slashes and
/// non-alphanumeric characters are replaced with `-` so the key is
/// ASCII and reversible for grep.
fn unlock_key_for(route: &str) -> String {
    let mut out: String = String::with_capacity(UNLOCK_KEY_PREFIX.len() + route.len());
    out.push_str(UNLOCK_KEY_PREFIX);
    for c in route.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else {
            out.push('-');
        }
    }
    out
}

/// Renders the password prompt that gates a `private` markdown page.
///
/// # Behaviour
///
/// - On mount, shows an empty password input plus an "Unlock" button.
/// - On submit, hashes the entered password via `crypto.subtle.digest`
///   and compares it byte-for-byte against `expected_hash`.
/// - On match, writes `"euv-docs:unlocked:<route>" = "1"` into
///   `localStorage` so the same browser session re-renders the page
///   body without re-asking on subsequent navigations and refreshes.
///   The route signal then re-resolves and the parent unmounts the
///   gate in favour of the page body.
/// - On mismatch, replaces the input with a red border + an error
///   message; the password field is cleared so the user can retry
///   without leaking what they typed into form history.

/// # Why a custom form instead of `<euv_field>` / `<euv_button>`
///
/// `<euv_field>` auto-routes the `Enter` keypress to the surrounding
/// form's `onsubmit` only when wrapped in `<form>`. The password gate
/// needs explicit submit handling so the password never lingers in
/// the DOM after a wrong attempt — using a bare `<input>` + `<button>`
/// keeps the HTML minimal and lets us clear the value imperatively.
#[component]
pub(crate) fn docs_password_gate(node: VirtualNode<DocsPasswordGateProps>) -> VirtualNode {
    let DocsPasswordGateProps {
        route,
        expected_hash,
        title,
    }: DocsPasswordGateProps = node.try_get_props().unwrap_or_default();
    let input_id: String = format!("pw-gate-{route}");
    let unlock_key: String = unlock_key_for(route);
    let input_signal: Signal<String> = App::use_signal(String::new);
    let error_signal: Signal<String> = App::use_signal(String::new);
    let busy_signal: Signal<bool> = App::use_signal(|| false);
    let submit = submit_handler(
        route,
        expected_hash,
        unlock_key,
        input_signal,
        error_signal,
        busy_signal,
    );
    let oninput = oninput_handler(input_signal);
    let onkeydown = onkeydown_handler(submit.clone());
    html! {
        div {
            class: c_pw_gate_wrapper()
            div {
                class: c_pw_gate_card()
                h2 {
                    class: c_pw_gate_title()
                    {
                        title
                    }
                }
                p {
                    class: c_pw_gate_hint()
                    "This article is password-protected. Enter the passphrase to view its contents."
                }
                input {
                    id: input_id.clone()
                    type: "password"
                    placeholder: "Password"
                    autocomplete: "off"
                    class: if { !error_signal.get().is_empty() } {
                        c_pw_gate_input_error()
                    } else {
                        c_pw_gate_input()
                    }
                    value: input_signal.get()
                    oninput: oninput
                    onkeydown: onkeydown
                }
                if { !error_signal.get().is_empty() } {
                    p {
                        class: c_pw_gate_error()
                        error_signal.get()
                    }
                }
                button {
                    class: c_pw_gate_submit()
                    disabled: busy_signal.get()
                    onclick: submit
                    if { busy_signal.get() } {
                        "Verifying…"
                    } else {
                        "Unlock"
                    }
                }
            }
        }
    }
}

/// Builds the submit handler. Hashes the input value with WebCrypto
/// SHA-256 and compares against `expected_hash`. On success writes the
/// unlock record into localStorage **and** forces a route re-resolution
/// so the parent renders the page body instead of the gate. On failure
/// sets an error message and clears the input.
fn submit_handler(
    route: &'static str,
    expected_hash: &'static str,
    unlock_key: String,
    input_signal: Signal<String>,
    error_signal: Signal<String>,
    busy_signal: Signal<bool>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        let typed: String = input_signal.get();
        if typed.is_empty() || busy_signal.get() {
            return;
        }
        error_signal.set(String::new());
        busy_signal.set(true);
        let route_static: &'static str = route;
        let unlock_key_owned: String = unlock_key.clone();
        let expected_hash_owned: &'static str = expected_hash;
        spawn_local(async move {
            let digest: Option<String> = async_sha256_hex(&typed).await;
            let matched: bool = digest
                .as_deref()
                .is_some_and(|computed: &str| computed == expected_hash_owned);
            if matched {
                UseEuvBrowser::local_storage_set(&unlock_key_owned, "1");
                // Force a route re-resolution by re-assigning the
                // current hash. Even when the value is identical, the
                // browser still fires `hashchange`, which is the
                // cheapest cross-browser way to nudge the Router hook
                // back to the parent's listener without adding a new
                // public API to euv-ui.
                if let Some(window) = web_sys::window() {
                    let location: Location = window.location();
                    let current: String = location.hash().unwrap_or_default();
                    let next: String = if current.is_empty() {
                        format!("#{route_static}")
                    } else {
                        current
                    };
                    let _: Result<(), JsValue> = location.set_hash(&next);
                }
                input_signal.set(String::new());
                error_signal.set(String::new());
            } else {
                error_signal.set("Incorrect password.".to_string());
                input_signal.set(String::new());
            }
            busy_signal.set(false);
            // `typed` is dropped here so the plaintext password does
            // not survive beyond the digest call.
        });
    }))
}

/// Updates the bound input signal on each keystroke so the value flows
/// back into the controlled `<input>` after a wrong-submit clear.
fn oninput_handler(input_signal: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        // `Event` derefs to `EventTarget` (via `web_sys::Event`'s own
        // `target()` method), so we can dyn-ref the target directly
        // without an explicit `unwrap_or(EventTarget::NULL)` — no
        // event ever reaches us without a target.
        if let Some(input) = event
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        {
            input_signal.set(input.value());
        }
    }))
}

/// Submits when the user presses `Enter` so they don't have to click
/// the button with the mouse.
fn onkeydown_handler(submit: Option<Rc<dyn Fn(Event)>>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(keyboard) = event.dyn_ref::<KeyboardEvent>() {
            if keyboard.key() == "Enter" {
                if let Some(handler) = submit.as_ref() {
                    handler.as_ref()(event);
                }
            }
        }
    }))
}

/// Returns whether the supplied route has already been unlocked in
/// this browser. The parent calls this on every render — including the
/// direct-URL-paste case where the page is mounted into a freshly
/// loaded `<div id="app">` — so the gate only renders when the user
/// hasn't unlocked this route yet in this session.

/// Reads from `localStorage` synchronously; the call is cheap and the
/// result is a single `bool` branch.
pub(crate) fn is_unlocked(route: &str) -> bool {
    let key: String = unlock_key_for(route);
    UseEuvBrowser::local_storage_get(&key).as_deref() == Some("1")
}
